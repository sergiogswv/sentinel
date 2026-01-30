//! # Sentinel - AI-Powered Code Monitor
//!
//! Herramienta de monitoreo en tiempo real que vigila cambios en archivos TypeScript,
//! analiza el código con Claude AI, ejecuta tests y gestiona commits automáticamente.
//!
//! ## Arquitectura
//!
//! ```text
//! ┌─────────────────┐
//! │  File Watcher   │ (notify crate)
//! └────────┬────────┘
//!          │ Detecta cambio en .ts
//!          ▼
//! ┌─────────────────┐
//! │ Análisis Claude │ (consultar_claude)
//! └────────┬────────┘
//!          │ Código aprobado
//!          ▼
//! ┌─────────────────┐
//! │  Jest Tests     │ (ejecutar_tests)
//! └────────┬────────┘
//!          │ Tests pasan
//!          ▼
//! ┌─────────────────┐
//! │  Git Commit     │ (preguntar_commit)
//! └─────────────────┘
//! ```

use colored::*;
use config::SentinelConfig;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use stats::SentinelStats;
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Instant;

// Módulos
mod ai;
mod config;
mod docs;
mod git;
mod stats;
mod tests;
mod ui;

// --- MAIN ---

/// Punto de entrada principal de Sentinel v3.3.
///
/// # Flujo de ejecución
///
/// 1. Solicita al usuario seleccionar un proyecto
/// 2. Configura el watcher en el directorio `src/` del proyecto
/// 3. Inicia un hilo para detectar comandos de teclado:
///    - 'p' → Pausa/Reanuda el monitoreo
///    - 'r' → Genera reporte diario de productividad
/// 4. Monitorea cambios en archivos .ts (excepto .spec.ts y .suggested)
/// 5. Para cada cambio detectado:
///    - Analiza arquitectura con Claude
///    - Si pasa, ejecuta tests con Jest
///    - Si tests pasan:
///      * Genera documentación automática (.md)
///      * Genera mensaje de commit inteligente
///      * Pregunta si hacer commit
///    - Si tests fallan, ofrece diagnóstico de Claude
///
/// # Comandos interactivos
///
/// - **'p'** → Pausar/reanudar el monitoreo de archivos
/// - **'r'** → Generar reporte diario basado en commits del día
///
/// # Mecanismos de pausa
///
/// - Archivo `.sentinel-pause` en el directorio del proyecto
/// - Comando 'p' en stdin (pausa/reanuda)
///
/// # Arquitectura interna
///
/// Utiliza Arc<Mutex<T>> para compartir estado entre hilos:
/// - `esta_pausado`: Bandera de pausa compartida entre hilo de teclado y loop principal
/// - `pause_file`: Ruta del archivo de pausa compartida entre hilos
/// - Channel (tx/rx): Comunicación entre watcher y loop principal
///
/// # Panics
///
/// - Si faltan variables de entorno `ANTHROPIC_AUTH_TOKEN` o `ANTHROPIC_BASE_URL`
/// - Si el directorio `src/` no existe en el proyecto seleccionado
/// - Si git no está instalado o el proyecto no es un repositorio git válido

fn inicializar_sentinel(project_path: &Path) -> SentinelConfig {
    // 1. Intentar cargar configuración existente
    if let Some(config) = SentinelConfig::load(project_path) {
        println!(
            "{}",
            "🔄 Configuración cargada desde .sentinelrc.toml".green()
        );
        return config;
    }

    // 2. Si no existe, configurar por primera vez
    println!(
        "{}",
        "🚀 Configurando nuevo proyecto en Sentinel...".bright_cyan()
    );

    let gestor = SentinelConfig::detectar_gestor(project_path);
    let nombre = project_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let config = SentinelConfig::default(nombre, gestor);

    // 3. Guardar para la próxima vez
    if let Err(e) = config.save(project_path) {
        println!("⚠️ No se pudo guardar la config: {}", e);
    }

    println!("{}", "✅ Proyecto inicializado y guardado.".green());
    config
}

fn main() {
    // 1. Selección y rutas (PathBuf es nuestro mejor amigo)
    let project_path = ui::seleccionar_proyecto();

    // Validar que el proyecto existe
    if !project_path.exists() {
        eprintln!("{}", "❌ Error: La ruta del proyecto no existe.".red().bold());
        eprintln!("   Ruta: {}", project_path.display());
        std::process::exit(1);
    }

    // 2. Inicializar configuración (carga o crea .sentinelrc.toml)
    let config = inicializar_sentinel(&project_path);
    let config = Arc::new(config); // Compartir entre hilos

    let path_to_watch = project_path.join("src");

    // Validar que el directorio src/ existe
    if !path_to_watch.exists() {
        eprintln!("{}", "❌ Error: El directorio 'src/' no existe en el proyecto.".red().bold());
        eprintln!("   Proyecto: {}", project_path.display());
        eprintln!("   Se esperaba: {}", path_to_watch.display());
        eprintln!("\n💡 Asegúrate de seleccionar un proyecto que tenga una carpeta 'src/'");
        std::process::exit(1);
    }
    // Usamos Arc para que el hilo y el loop compartan la ruta del archivo de pausa
    let pause_file = Arc::new(project_path.join(".sentinel-pause"));

    // 3. Control de Pausa Compartida
    let esta_pausado = Arc::new(Mutex::new(false));
    let pausa_hilo = Arc::clone(&esta_pausado);
    let pausa_loop = Arc::clone(&esta_pausado);

    // 4. Clones para los hilos (Rust requiere copias explícitas)
    let config_watcher = Arc::clone(&config);
    let config_loop = Arc::clone(&config);
    let project_path_hilo = project_path.clone();
    let pause_file_hilo = Arc::clone(&pause_file);

    // 5. EL CANAL (Debe estar aquí afuera para que 'rx' sea visible en el loop)
    let (tx, rx) = std::sync::mpsc::channel::<PathBuf>();

    // Canal para reenviar input de stdin al loop principal cuando se espera respuesta
    let (stdin_tx, stdin_rx) = mpsc::channel::<String>();
    let stdin_rx = Arc::new(Mutex::new(stdin_rx));
    let esperando_input = Arc::new(Mutex::new(false));
    let esperando_input_hilo = Arc::clone(&esperando_input);

    let stats = Arc::new(Mutex::new(SentinelStats::cargar(&project_path)));
    let stats_hilo = Arc::clone(&stats); // Para el comando 'm'
    let stats_loop = Arc::clone(&stats); // Para el análisis de archivos

    // Hilo de Teclado (Pausa 'P', Reporte 'R', y reenvío de respuestas)
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                let cmd = input.trim().to_lowercase();
                // Si el loop principal espera una respuesta, reenviar el input
                if *esperando_input_hilo.lock().unwrap() {
                    let _ = stdin_tx.send(cmd);
                } else if cmd == "p" {
                    let mut p = pausa_hilo.lock().unwrap();
                    *p = !*p;
                    println!(
                        " ⌨️  SENTINEL: {}",
                        if *p {
                            "PAUSADO".yellow()
                        } else {
                            "ACTIVO".green()
                        }
                    );
                } else if cmd == "r" {
                    git::generar_reporte_diario(&project_path_hilo);
                } else if cmd == "m" {
                    let stats = stats_hilo.lock().unwrap();
                    println!(
                        "\n{}",
                        "📊 DASHBOARD DE RENDIMIENTO SENTINEL".bright_green().bold()
                    );
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!(
                        "🚫 Bugs Críticos Evitados:  {}",
                        stats.bugs_criticos_evitados.to_string().red()
                    );
                    println!(
                        "✅ Sugerencias Generadas:   {}",
                        stats.sugerencias_aplicadas.to_string().cyan()
                    );
                    println!(
                        "🧪 Tests Corregidos con IA: {}",
                        stats.tests_fallidos_corregidos.to_string().yellow()
                    );
                    println!(
                        "⏳ Tiempo Ahorrado:         {} horas",
                        (stats.tiempo_estimado_ahorrado_mins as f32 / 60.0)
                    );
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                } else if cmd == "c" {
                    // Editar configuración
                    SentinelConfig::abrir_en_editor(&project_path_hilo);
                } else if cmd == "x" {
                    // Reiniciar configuración
                    print!(
                        "{}",
                        "⚠️  ¿Estás seguro de que quieres reiniciar la config? (s/n): "
                            .red()
                            .bold()
                    );
                    io::stdout().flush().unwrap();

                    // Leer la siguiente línea de stdin directamente
                    let mut confirmacion = String::new();
                    if io::stdin().read_line(&mut confirmacion).is_ok() {
                        if confirmacion.trim().to_lowercase() == "s" {
                            let _ = SentinelConfig::eliminar(&project_path_hilo);
                            println!(
                                "{}",
                                "🔄 Por favor, reinicia Sentinel para aplicar los cambios."
                                    .bright_cyan()
                            );
                            std::process::exit(0); // Salimos para que el usuario lo vuelva a lanzar
                        } else {
                            println!("{}", "   ⏭️  Operación cancelada.".yellow());
                        }
                    }
                }
            }
        }
    });

    // 6. El Watcher (usa config para filtrar archivos)
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            if let EventKind::Modify(_) = event.kind {
                for path in event.paths {
                    // Usar la configuración para decidir si ignorar el archivo
                    if !config_watcher.debe_ignorar(&path) {
                        let _ = tx.send(path); // Enviamos PathBuf por el canal
                    }
                }
            }
        }
    })
    .unwrap();

    if let Err(e) = watcher.watch(&path_to_watch, RecursiveMode::Recursive) {
        eprintln!("{}", "❌ Error al configurar el watcher.".red().bold());
        eprintln!("   Ruta: {}", path_to_watch.display());
        eprintln!("   Error: {}", e);
        std::process::exit(1);
    }

    println!(
        "\n{} {}",
        "🛡️  Sentinel v3.3 activo en:".green(),
        project_path.display()
    );

    // Helper: pedir input al usuario a través del hilo de teclado (timeout 30s)
    let esperando_ref = Arc::clone(&esperando_input);
    let stdin_rx_ref = Arc::clone(&stdin_rx);
    let leer_respuesta = move || -> Option<String> {
        *esperando_ref.lock().unwrap() = true;
        let resultado = stdin_rx_ref
            .lock()
            .unwrap()
            .recv_timeout(std::time::Duration::from_secs(30))
            .ok();
        *esperando_ref.lock().unwrap() = false;
        resultado
    };

    // 7. EL LOOP PRINCIPAL (Ahora 'rx' sí existe aquí)
    let mut ultimo_cambio: HashMap<PathBuf, Instant> = HashMap::new();
    let debounce = std::time::Duration::from_secs(15);

    while let Ok(changed_path) = rx.recv() {
        // --- DEBOUNCE REAL (Drenado) ---
        // Esperamos 500ms para que se acumulen los eventos duplicados y los limpiamos
        thread::sleep(std::time::Duration::from_millis(500));
        while rx.try_recv().is_ok() {}

        if pause_file_hilo.exists() || *pausa_loop.lock().unwrap() {
            continue;
        }

        // Verificamos pausa (Archivo físico o Tecla P)
        if pause_file_hilo.exists() || *pausa_loop.lock().unwrap() {
            continue;
        }

        // Debounce: ignorar si el mismo archivo se procesó hace menos de 2 segundos
        let ahora = Instant::now();
        if let Some(ultimo) = ultimo_cambio.get(&changed_path) {
            if ahora.duration_since(*ultimo) < debounce {
                continue;
            }
        }
        ultimo_cambio.insert(changed_path.clone(), ahora);

        // Rust ahora sabe que changed_path es un PathBuf
        let file_name = changed_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let base_name = file_name.split('.').next().unwrap();
        let test_rel_path = format!("test/{}/{}.spec.ts", base_name, base_name);

        if !project_path.join(&test_rel_path).exists() {
            println!("\n⏭️  IGNORADO (sin test): {}", file_name);
            continue;
        }

        println!("\n🔔 CAMBIO EN: {}", file_name.cyan().bold());
        thread::sleep(std::time::Duration::from_millis(250));

        if let Ok(codigo) = std::fs::read_to_string(&changed_path) {
            if codigo.trim().is_empty() {
                continue;
            }

            match ai::analizar_arquitectura(
                &codigo,
                &file_name,
                Arc::clone(&stats_loop),
                &config_loop, // Pasamos la config completa
                &project_path,
            ) {
                Ok(true) => {
                    println!("{}", "   ✅ Arquitectura aprobada.".green());

                    match tests::ejecutar_tests(&test_rel_path, &project_path) {
                        Ok(_) => {
                            println!("{}", "   ✅ Tests pasados con éxito".green().bold());
                            let _ = docs::actualizar_documentacion(&codigo, &changed_path);
                            let mensaje_ia = git::generar_mensaje_commit(&codigo, &file_name);
                            println!("\n🚀 Mensaje sugerido: {}", mensaje_ia.bright_cyan().bold());
                            print!("📝 ¿Quieres hacer commit? (s/n, timeout 30s): ");
                            io::stdout().flush().unwrap();
                            match leer_respuesta() {
                                Some(resp) => {
                                    git::preguntar_commit(&project_path, &mensaje_ia, &resp)
                                }
                                None => println!("   ⏭️  Timeout, commit omitido."),
                            }
                        }
                        Err(err_msg) => {
                            println!("{}", "   ❌ Tests fallaron".red().bold());
                            print!("\n🔍 ¿Analizar error con IA? (s/n, timeout 30s): ");
                            io::stdout().flush().unwrap();
                            if leer_respuesta().as_deref() == Some("s") {
                                let _ = tests::pedir_ayuda_test(&codigo, &err_msg);
                            }
                        }
                    }
                }
                Ok(false) => println!("{}", "   ❌ CRITICO: Corrige SOLID/Bugs".red().bold()),
                Err(e) => println!("   ⚠️  Error de IA: {}", e),
            }
        }

        // Drenar eventos pendientes que se acumularon durante el procesamiento
        while rx.try_recv().is_ok() {}
        ultimo_cambio.insert(changed_path, Instant::now());
    }
}
