//! Módulo de gestión de Git
//!
//! Funciones relacionadas con operaciones de Git: commits, reportes y gestión de historial.

use crate::ai;
use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};

use crate::config::SentinelConfig;
use crate::stats::SentinelStats;

/// Obtiene un resumen de los commits realizados hoy.
pub fn obtener_resumen_git(project_path: &Path) -> String {
    let output = Command::new("git")
        .args(["log", "--since=00:00:00", "--oneline", "--pretty=format:%s"])
        .current_dir(project_path)
        .output()
        .expect("Fallo al leer git logs");

    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Genera un mensaje de commit automático siguiendo Conventional Commits.
pub fn generar_mensaje_commit(
    codigo: &str,
    file_name: &str,
    config: &SentinelConfig,
    stats: Arc<Mutex<SentinelStats>>,
    project_path: &Path,
) -> String {
    println!(
        "{}",
        "📝 Generando mensaje de commit inteligente...".magenta()
    );
    let prompt = format!(
        "Genera un mensaje de commit corto (máximo 50 caracteres) siguiendo 'Conventional Commits' para los cambios en {}. Solo devuelve el texto del mensaje.\n\nCódigo:\n{}",
        file_name, codigo
    );

    match ai::consultar_ia_dinamico(prompt, ai::TaskType::Light, config, stats, project_path) {
        Ok(msg) => msg.trim().replace('"', ""),
        Err(_) => format!("feat: update {}", file_name),
    }
}

/// Genera un reporte de productividad diario usando Claude AI.
pub fn generar_reporte_diario(
    project_path: &Path,
    config: &SentinelConfig,
    stats: Arc<Mutex<SentinelStats>>,
) {
    println!(
        "\n📊 {}...",
        "Generando reporte de productividad diaria".magenta().bold()
    );

    let logs = obtener_resumen_git(project_path);
    if logs.is_empty() {
        println!(
            "{}",
            "⚠️ No hay commits registrados el día de hoy.".yellow()
        );
        return;
    }

    let prompt = format!(
        "Actúa como un Lead Developer. Basado en estos mensajes de commit de hoy, \
        genera un reporte de progreso diario para el equipo. \
        Divide en: ✨ Logros Principales, 🛠️ Aspectos Técnicos (NestJS/Rust) y 🚀 Próximos Pasos. \
        Sé profesional y directo.\n\nCommits del día:\n{}",
        logs
    );

    match ai::consultar_ia_dinamico(prompt, ai::TaskType::Deep, config, stats, project_path) {
        Ok(reporte) => {
            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("{}", "📝 REPORTE DIARIO DE SENTINEL".cyan().bold());
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            println!("{}", reporte);
            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            let _ = fs::write(project_path.join("docs/DAILY_REPORT.md"), reporte);
        }
        Err(e) => println!("❌ Error al generar reporte: {}", e),
    }
}

/// Pregunta interactivamente al usuario si desea crear un commit.
pub fn preguntar_commit(project_path: &Path, mensaje: &str, respuesta: &str) {
    if respuesta == "s" {
        Command::new("git")
            .args(["add", "."])
            .current_dir(project_path)
            .status()
            .ok();
        match Command::new("git")
            .args(["commit", "-m", mensaje])
            .current_dir(project_path)
            .status()
        {
            Ok(_) => println!("   ✅ Commit exitoso!"),
            Err(e) => println!("   ❌ Error: {}", e),
        }
    } else {
        println!("   ⏭️  Commit omitido.");
    }
}
