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
use reqwest::blocking::Client;
use serde_json::json;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// --- NÚCLEO DE COMUNICACIÓN (DRY) ---

/// Realiza una consulta al API de Claude AI (Anthropic).
///
/// # Argumentos
///
/// * `prompt` - El prompt a enviar a Claude, generalmente código + instrucciones
///
/// # Variables de entorno requeridas
///
/// * `ANTHROPIC_AUTH_TOKEN` - API key de Anthropic
/// * `ANTHROPIC_BASE_URL` - URL base de la API (ej: https://api.anthropic.com)
///
/// # Retorna
///
/// * `Ok(String)` - Respuesta de texto generada por Claude
/// * `Err` - Error de red, autenticación o respuesta malformada
///
/// # Ejemplo
///
/// ```no_run
/// let respuesta = consultar_claude("Analiza este código: fn main() {}".to_string())?;
/// println!("Claude dice: {}", respuesta);
/// ```
fn consultar_claude(prompt: String) -> anyhow::Result<String> {
    let api_key = std::env::var("ANTHROPIC_AUTH_TOKEN").expect("❌ Falta ANTHROPIC_AUTH_TOKEN");
    let base_url = std::env::var("ANTHROPIC_BASE_URL").expect("❌ Falta ANTHROPIC_BASE_URL");
    let url = format!("{}/v1/messages", base_url.trim_end_matches('/'));

    let client = Client::new();
    let response = client.post(&url)
        .header("x-api-key", api_key)
        .header("content-type", "application/json")
        .json(&json!({
            "model": "claude-opus-4-5-20251101",
            "max_tokens": 1500,
            "messages": [{"role": "user", "content": prompt}]
        }))
        .send()?;

    let body: serde_json::Value = response.json()?;
    let texto = body["content"][0]["text"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Respuesta vacía de la IA"))?;
    
    Ok(texto.to_string())
}

// --- FUNCIONES DE ANÁLISIS ---

/// Analiza código TypeScript/NestJS con Claude AI enfocándose en arquitectura y buenas prácticas.
///
/// Evalúa principios SOLID, Clean Code y patrones de NestJS. Si encuentra problemas críticos,
/// Claude responderá comenzando con "CRITICO", de lo contrario con "SEGURO".
///
/// # Argumentos
///
/// * `codigo` - Código fuente a analizar
/// * `file_name` - Nombre del archivo (para contexto en el prompt)
///
/// # Retorna
///
/// * `Ok(true)` - Código aprobado (sin problemas críticos)
/// * `Ok(false)` - Código rechazado (problemas críticos detectados)
/// * `Err` - Error de comunicación con la IA
///
/// # Efectos secundarios
///
/// Crea un archivo `{file_name}.suggested` con la versión mejorada del código.
fn analizar_arquitectura(codigo: &str, file_name: &str) -> anyhow::Result<bool> {
    let prompt = format!(
        "Actúa como un Arquitecto de Software experto en NestJS y Clean Code. Analiza {}.\n\
        Enfócate en SOLID, Clean Code y NestJS Best Practices.\n\
        REGLAS: Inicia con 'CRITICO' si hay fallos graves, o 'SEGURO' si está bien.\n\
        Incluye una versión mejorada en bloque ```typescript.\n\nCódigo:\n{}", 
        file_name, codigo
    );

    let respuesta = consultar_claude(prompt)?;
    println!("\n✨ CONSEJO DE CLAUDE:\n{}", respuesta);

    let sugerencia = extraer_codigo(&respuesta);
    fs::write(format!("{}.suggested", file_name), sugerencia)?;

    Ok(!respuesta.trim().to_uppercase().starts_with("CRITICO"))
}

/// Solicita ayuda a Claude AI cuando fallan los tests de Jest.
///
/// Envía el código y el error de Jest a Claude para obtener un diagnóstico
/// y sugerencia de solución.
///
/// # Argumentos
///
/// * `codigo` - Código fuente que causó el fallo
/// * `error_jest` - Salida de error completa de Jest (stdout + stderr)
fn pedir_ayuda_test(codigo: &str, error_jest: &str) -> anyhow::Result<()> {
    println!("{}", "🔍 Analizando fallo en tests...".magenta());
    let prompt = format!(
        "Los tests de NestJS fallaron.\nERROR DE JEST:\n{}\n\nCÓDIGO:\n{}\nExplica el fallo y dame el fix breve.",
        error_jest, codigo
    );

    let respuesta = consultar_claude(prompt)?;
    println!("\n💡 SOLUCIÓN SUGERIDA:\n{}", respuesta.yellow());
    Ok(())
}

/// Genera un mensaje de commit automático siguiendo Conventional Commits.
///
/// Analiza los cambios en el código y genera un mensaje descriptivo y conciso
/// (máximo 50 caracteres) siguiendo el formato: `tipo: descripción`.
///
/// # Argumentos
///
/// * `codigo` - Código fuente modificado
/// * `file_name` - Nombre del archivo modificado
///
/// # Retorna
///
/// Mensaje de commit generado, o un fallback genérico si Claude falla.
///
/// # Ejemplo de salida
///
/// ```text
/// feat: add user authentication service
/// fix: resolve null pointer in validator
/// refactor: simplify error handling logic
/// ```
fn generar_mensaje_commit(codigo: &str, file_name: &str) -> String {
    println!("{}", "📝 Generando mensaje de commit inteligente...".magenta());
    let prompt = format!(
        "Genera un mensaje de commit corto (máximo 50 caracteres) siguiendo 'Conventional Commits' para los cambios en {}. Solo devuelve el texto del mensaje.\n\nCódigo:\n{}", 
        file_name, codigo
    );

    match consultar_claude(prompt) {
        Ok(msg) => msg.trim().replace('"', ""),
        Err(_) => format!("feat: update {}", file_name)
    }
}

// --- UTILIDADES ---

/// Ejecuta los tests de Jest relacionados con un archivo específico.
///
/// Utiliza `npm run test -- --findRelatedTests` para ejecutar solo los tests
/// que están vinculados al archivo modificado.
///
/// # Argumentos
///
/// * `test_path` - Ruta relativa del archivo de test (ej: "test/users/users.spec.ts")
/// * `project_path` - Ruta absoluta del directorio del proyecto NestJS
///
/// # Retorna
///
/// * `Ok(())` - Tests pasaron exitosamente
/// * `Err(String)` - Tests fallaron, con salida de error completa
fn ejecutar_tests(test_path: &str, project_path: &Path) -> Result<(), String> {
    println!("\n🧪 Ejecutando Jest para: {}", test_path.cyan());
    let output = Command::new("npm")
        .args(["run", "test", "--", "--findRelatedTests", test_path])
        .current_dir(project_path)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!("{}\n{}", 
            String::from_utf8_lossy(&output.stdout), 
            String::from_utf8_lossy(&output.stderr)))
    }
}

/// Pregunta interactivamente al usuario si desea crear un commit.
///
/// Muestra el mensaje generado y espera 30 segundos por confirmación.
/// Si el usuario responde 's', ejecuta `git add .` seguido de `git commit`.
///
/// # Argumentos
///
/// * `project_path` - Ruta del proyecto donde ejecutar los comandos git
/// * `mensaje` - Mensaje de commit propuesto
///
/// # Comportamiento
///
/// - Timeout de 30 segundos
/// - Requiere respuesta 's' para confirmar (cualquier otra input se ignora)
/// - Ejecuta git add y git commit de forma secuencial si se confirma
fn preguntar_commit(project_path: &Path, mensaje: &str) {
    println!("\n🚀 Mensaje sugerido: {}", mensaje.bright_cyan().bold());
    print!("📝 ¿Quieres hacer commit? (s/n, timeout 30s): ");
    io::stdout().flush().unwrap();

    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let mut respuesta = String::new();
        if io::stdin().read_line(&mut respuesta).is_ok() {
            let _ = tx.send(respuesta.trim().to_lowercase());
        }
    });

    if let Ok(r) = rx.recv_timeout(Duration::from_secs(30)) {
        if r == "s" {
            Command::new("git").args(["add", "."]).current_dir(project_path).status().ok();
            match Command::new("git").args(["commit", "-m", mensaje]).current_dir(project_path).status() {
                Ok(_) => println!("   ✅ Commit exitoso!"),
                Err(e) => println!("   ❌ Error: {}", e),
            }
        }
    }
}

/// Obtiene un resumen de los commits realizados hoy.
///
/// Ejecuta `git log --since=00:00:00` para obtener todos los mensajes de commit
/// del día actual (desde las 00:00:00 hasta el momento presente).
///
/// # Argumentos
///
/// * `project_path` - Ruta del proyecto donde ejecutar el comando git
///
/// # Retorna
///
/// String con los mensajes de commit, uno por línea. String vacío si no hay commits.
///
/// # Panics
///
/// Si el comando git falla (repositorio no inicializado, git no instalado, etc.)
fn obtener_resumen_git(project_path: &Path) -> String {
    let output = Command::new("git")
        .args([
            "log",
            "--since=00:00:00",
            "--oneline",
            "--pretty=format:%s"
        ])
        .current_dir(project_path)
        .output()
        .expect("Fallo al leer git logs");

    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Genera un reporte de productividad diario usando Claude AI.
///
/// Analiza todos los commits del día actual y genera un reporte profesional
/// dividido en tres secciones:
/// - ✨ Logros Principales
/// - 🛠️ Aspectos Técnicos (NestJS/Rust)
/// - 🚀 Próximos Pasos
///
/// # Argumentos
///
/// * `project_path` - Ruta del proyecto donde obtener commits y guardar el reporte
///
/// # Efectos secundarios
///
/// - Imprime el reporte en la consola
/// - Guarda el reporte en `docs/DAILY_REPORT.md`
///
/// # Comportamiento
///
/// Si no hay commits del día, muestra advertencia y sale sin generar reporte.
///
/// # Ejemplo de uso
///
/// Presiona 'r' en la consola de Sentinel para generar el reporte.
///
/// # Formato de salida
///
/// ```markdown
/// ✨ Logros Principales
/// - Implementación de autenticación JWT
/// - Migración de base de datos completada
///
/// 🛠️ Aspectos Técnicos
/// - Integración con NestJS Guards
/// - Refactorización de servicios
///
/// 🚀 Próximos Pasos
/// - Testing de endpoints
/// - Documentación de API
/// ```
fn generar_reporte_diario(project_path: &Path) {
    println!("\n📊 {}...", "Generando reporte de productividad diaria".magenta().bold());

    let logs = obtener_resumen_git(project_path);
    if logs.is_empty() {
        println!("{}", "⚠️ No hay commits registrados el día de hoy.".yellow());
        return;
    }

    let prompt = format!(
        "Actúa como un Lead Developer. Basado en estos mensajes de commit de hoy, \
        genera un reporte de progreso diario para el equipo. \
        Divide en: ✨ Logros Principales, 🛠️ Aspectos Técnicos (NestJS/Rust) y 🚀 Próximos Pasos. \
        Sé profesional y directo.\n\nCommits del día:\n{}",
        logs
    );

    match consultar_claude(prompt) {
        Ok(reporte) => {
            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("{}", "📝 REPORTE DIARIO DE SENTINEL".cyan().bold());
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            println!("{}", reporte);
            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            // Opcional: Guardarlo en un archivo
            let _ = fs::write(project_path.join("docs/DAILY_REPORT.md"), reporte);
        },
        Err(e) => println!("❌ Error al generar reporte: {}", e),
    }
}

/// Genera un "manual de bolsillo" automático para cada archivo modificado.
///
/// Crea documentación técnica ultra-concisa (máximo 150 palabras) en formato Markdown
/// usando Claude AI. El archivo .md se genera en el mismo directorio que el archivo .ts
/// con el mismo nombre pero extensión .md.
///
/// # Argumentos
///
/// * `codigo` - Código fuente del archivo modificado
/// * `file_path` - Ruta completa al archivo .ts modificado
///
/// # Retorna
///
/// * `Ok(())` - Documentación generada exitosamente
/// * `Err` - Error al comunicarse con la IA o al escribir el archivo
///
/// # Efectos secundarios
///
/// Crea/sobrescribe un archivo .md en la misma ubicación que el .ts original.
/// Por ejemplo: `src/users/users.service.ts` → `src/users/users.service.md`
///
/// # Formato de salida
///
/// ```markdown
/// # 📖 Documentación: users.service.ts
///
/// > ✨ Actualizado automáticamente por Sentinel v3.1
///
/// 🎯 **Funcionalidad**: Gestiona operaciones CRUD de usuarios...
/// 🔧 **Métodos principales**: findAll(), create(), update()...
///
/// ---
/// *Último refactor: SystemTime { ... }*
/// ```
fn actualizar_documentacion(codigo: &str, file_path: &Path) -> anyhow::Result<()> {
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    println!("📚 Actualizando manual de bolsillo para: {}", file_name.magenta());

    let prompt = format!(
        "Como documentador técnico de NestJS, analiza este código: {}. \
        Genera un resumen técnico ultra-conciso (máximo 150 palabras) en Markdown. \
        Enfócate en: ¿Qué hace este servicio? y ¿Cuáles son sus métodos principales? \
        Usa emojis para las secciones. No uses introducciones innecesarias.\n\n{}",
        file_name, codigo
    );

    let resumen = consultar_claude(prompt)?;

    // Cambiamos la extensión de .ts a .md en la misma carpeta
    let mut docs_path = file_path.to_path_buf();
    docs_path.set_extension("md");

    let nueva_doc = format!(
        "# 📖 Documentación: {}\n\n> ✨ Actualizado automáticamente por Sentinel v3.1\n\n{}\n\n---\n*Último refactor: {:?}*",
        file_name,
        resumen,
        std::time::SystemTime::now()
    );

    fs::write(&docs_path, nueva_doc)?;
    println!("   ✅ Documento generado: {}", docs_path.display());
    Ok(())
}

/// Extrae bloques de código TypeScript de una respuesta de Claude.
///
/// Busca y extrae el contenido entre delimitadores \`\`\`typescript...\`\`\`.
/// Si no encuentra un bloque delimitado, devuelve el texto completo.
///
/// # Argumentos
///
/// * `texto` - Respuesta completa de Claude AI
///
/// # Retorna
///
/// Código TypeScript extraído (sin delimitadores) o el texto original.
fn extraer_codigo(texto: &str) -> String {
    if let Some(start) = texto.find("```typescript") {
        let resto = &texto[start + 13..];
        if let Some(end) = resto.find("```") {
            return resto[..end].trim().to_string();
        }
    }
    texto.to_string()
}

/// Presenta un menú interactivo para seleccionar un proyecto del directorio padre.
///
/// Escanea el directorio padre (`../`) y muestra todos los subdirectorios como
/// opciones de proyectos. El usuario selecciona mediante un número.
///
/// # Retorna
///
/// PathBuf del proyecto seleccionado.
///
/// # Nota
///
/// Si el usuario ingresa un número inválido, por defecto selecciona el proyecto 1.
fn seleccionar_proyecto() -> PathBuf {
    println!("{}", "\n📂 Proyectos detectados:".bright_cyan().bold());
    let entries = fs::read_dir("../").unwrap();
    let proyectos: Vec<PathBuf> = entries.flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();

    for (i, p) in proyectos.iter().enumerate() {
        println!("{}. {}", i + 1, p.file_name().unwrap().to_str().unwrap());
    }

    print!("\n👉 Selecciona número: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let idx = input.trim().parse::<usize>().unwrap_or(1) - 1;
    proyectos[idx].clone()
}

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
    let project_path = seleccionar_proyecto();
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
                    generar_reporte_diario(&project_path_hilo);
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
        thread::sleep(Duration::from_millis(250));

        if let Ok(codigo) = fs::read_to_string(&changed_path) {
            if codigo.trim().is_empty() { continue; }

            match analizar_arquitectura(&codigo, &file_name) {
                Ok(true) => {
                    println!("{}", "   ✅ Arquitectura aprobada.".green());
                    
                    match ejecutar_tests(&test_rel_path, &project_path) {
                        Ok(_) => {
                            println!("{}", "   ✅ Tests pasados con éxito".green().bold());
                            let _ = actualizar_documentacion(&codigo, &changed_path);
                            let mensaje_ia = generar_mensaje_commit(&codigo, &file_name);
                            preguntar_commit(&project_path, &mensaje_ia);
                        },
                        Err(err_msg) => {
                            println!("{}", "   ❌ Tests fallaron".red().bold());
                            print!("\n🔍 ¿Analizar error con IA? (s/n): ");
                            io::stdout().flush().unwrap();
                            let mut res = String::new();
                            io::stdin().read_line(&mut res).ok();
                            if res.trim().to_lowercase() == "s" {
                                let _ = pedir_ayuda_test(&codigo, &err_msg);
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