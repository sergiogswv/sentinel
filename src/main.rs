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
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

// Módulos
mod ai;
mod git;
mod tests;
mod docs;
mod ui;

// --- MAIN ---

/// Punto de entrada principal de Sentinel v3.2.
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

    // Hilo de Teclado (Pausa 'P' y Reporte 'R')
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                let cmd = input.trim().to_lowercase();
                if cmd == "p" {
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
    println!("\n{} {}", "🛡️  Sentinel v3.2 activo en:".green(), project_path.display());

    // 6. EL LOOP PRINCIPAL (Ahora 'rx' sí existe aquí)
    for changed_path in rx {
        // Verificamos pausa (Archivo físico o Tecla P)
        if pause_file_hilo.exists() || *pausa_loop.lock().unwrap() {
            continue;
        }

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
                            git::preguntar_commit(&project_path, &mensaje_ia);
                        },
                        Err(err_msg) => {
                            println!("{}", "   ❌ Tests fallaron".red().bold());
                            print!("\n🔍 ¿Analizar error con IA? (s/n): ");
                            io::stdout().flush().unwrap();
                            let mut res = String::new();
                            io::stdin().read_line(&mut res).ok();
                            if res.trim().to_lowercase() == "s" {
                                let _ = tests::pedir_ayuda_test(&codigo, &err_msg);
                            }
                        }
                    }
                },
                Ok(false) => println!("{}", "   ❌ CRITICO: Corrige SOLID/Bugs".red().bold()),
                Err(e) => println!("   ⚠️  Error de IA: {}", e),
            }
        }
    }
}