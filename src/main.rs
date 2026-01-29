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
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Instant;

// Módulos
mod ai;
mod git;
mod tests;
mod docs;
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
fn main() {
    // 1. Selección y rutas (PathBuf es nuestro mejor amigo)
    let project_path = ui::seleccionar_proyecto();
    let path_to_watch = project_path.join("src");
    // Usamos Arc para que el hilo y el loop compartan la ruta del archivo de pausa
    let pause_file = Arc::new(project_path.join(".sentinel-pause"));

    // 2. Control de Pausa Compartida
    let esta_pausado = Arc::new(Mutex::new(false));
    let pausa_hilo = Arc::clone(&esta_pausado);
    let pausa_loop = Arc::clone(&esta_pausado);

    // 3. Clones para los hilos (Rust requiere copias explícitas)
    let project_path_hilo = project_path.clone();
    let pause_file_hilo = Arc::clone(&pause_file);

    // 4. EL CANAL (Debe estar aquí afuera para que 'rx' sea visible en el loop)
    let (tx, rx) = std::sync::mpsc::channel::<PathBuf>();

    // Canal para reenviar input de stdin al loop principal cuando se espera respuesta
    let (stdin_tx, stdin_rx) = mpsc::channel::<String>();
    let stdin_rx = Arc::new(Mutex::new(stdin_rx));
    let esperando_input = Arc::new(Mutex::new(false));
    let esperando_input_hilo = Arc::clone(&esperando_input);

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
                    println!(" ⌨️  SENTINEL: {}", if *p { "PAUSADO".yellow() } else { "ACTIVO".green() });
                } else if cmd == "r" {
                    git::generar_reporte_diario(&project_path_hilo);
                }
            }
        }
    });

    // 5. El Watcher
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            if let EventKind::Modify(_) = event.kind {
                for path in event.paths {
                    if path.extension().map_or(false, |ext| ext == "ts") && 
                       !path.to_str().unwrap().contains(".spec.ts") &&
                       !path.to_str().unwrap().contains(".suggested") {
                        let _ = tx.send(path); // Enviamos PathBuf por el canal
                    }
                }
            }
        }
    }).unwrap();

    watcher.watch(&path_to_watch, RecursiveMode::Recursive).unwrap();
    println!("\n{} {}", "🛡️  Sentinel v3.3 activo en:".green(), project_path.display());

    // Helper: pedir input al usuario a través del hilo de teclado (timeout 30s)
    let esperando_ref = Arc::clone(&esperando_input);
    let stdin_rx_ref = Arc::clone(&stdin_rx);
    let leer_respuesta = move || -> Option<String> {
        *esperando_ref.lock().unwrap() = true;
        let resultado = stdin_rx_ref.lock().unwrap()
            .recv_timeout(std::time::Duration::from_secs(30))
            .ok();
        *esperando_ref.lock().unwrap() = false;
        resultado
    };

    // 6. EL LOOP PRINCIPAL (Ahora 'rx' sí existe aquí)
    let mut ultimo_cambio: HashMap<PathBuf, Instant> = HashMap::new();
    let debounce = std::time::Duration::from_secs(15);

    while let Ok(changed_path) = rx.recv() {
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
        let file_name = changed_path.file_name().unwrap().to_str().unwrap().to_string();
        let base_name = file_name.split('.').next().unwrap();
        let test_rel_path = format!("test/{}/{}.spec.ts", base_name, base_name);
        
        if !project_path.join(&test_rel_path).exists() {
            println!("\n⏭️  IGNORADO (sin test): {}", file_name);
            continue;
        }

        println!("\n🔔 CAMBIO EN: {}", file_name.cyan().bold());
        thread::sleep(std::time::Duration::from_millis(250));

        if let Ok(codigo) = std::fs::read_to_string(&changed_path) {
            if codigo.trim().is_empty() { continue; }

            match ai::analizar_arquitectura(&codigo, &file_name) {
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
                                Some(resp) => git::preguntar_commit(&project_path, &mensaje_ia, &resp),
                                None => println!("   ⏭️  Timeout, commit omitido."),
                            }
                        },
                        Err(err_msg) => {
                            println!("{}", "   ❌ Tests fallaron".red().bold());
                            print!("\n🔍 ¿Analizar error con IA? (s/n, timeout 30s): ");
                            io::stdout().flush().unwrap();
                            if leer_respuesta().as_deref() == Some("s") {
                                let _ = tests::pedir_ayuda_test(&codigo, &err_msg);
                            }
                        }
                    }
                },
                Ok(false) => println!("{}", "   ❌ CRITICO: Corrige SOLID/Bugs".red().bold()),
                Err(e) => println!("   ⚠️  Error de IA: {}", e),
            }
        }

        // Drenar eventos pendientes que se acumularon durante el procesamiento
        while rx.try_recv().is_ok() {}
        ultimo_cambio.insert(changed_path, Instant::now());
    }
}