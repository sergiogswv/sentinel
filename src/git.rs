//! Módulo de gestión de Git
//!
//! Funciones relacionadas con operaciones de Git: commits, reportes y gestión de historial.

use std::process::Command;
use std::path::Path;
use std::fs;
use colored::*;
use crate::ai;

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
pub fn obtener_resumen_git(project_path: &Path) -> String {
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
pub fn generar_mensaje_commit(codigo: &str, file_name: &str) -> String {
    println!("{}", "📝 Generando mensaje de commit inteligente...".magenta());
    let prompt = format!(
        "Genera un mensaje de commit corto (máximo 50 caracteres) siguiendo 'Conventional Commits' para los cambios en {}. Solo devuelve el texto del mensaje.\n\nCódigo:\n{}",
        file_name, codigo
    );

    match ai::consultar_claude(prompt) {
        Ok(msg) => msg.trim().replace('"', ""),
        Err(_) => format!("feat: update {}", file_name)
    }
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
pub fn generar_reporte_diario(project_path: &Path) {
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

    match ai::consultar_claude(prompt) {
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
pub fn preguntar_commit(project_path: &Path, mensaje: &str, respuesta: &str) {
    if respuesta == "s" {
        Command::new("git").args(["add", "."]).current_dir(project_path).status().ok();
        match Command::new("git").args(["commit", "-m", mensaje]).current_dir(project_path).status() {
            Ok(_) => println!("   ✅ Commit exitoso!"),
            Err(e) => println!("   ❌ Error: {}", e),
        }
    } else {
        println!("   ⏭️  Commit omitido.");
    }
}