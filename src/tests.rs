//! Módulo de ejecución de tests
//!
//! Se encarga de correr los tests con Jest y reportar resultados.

use crate::ai;
use colored::*;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};

use crate::config::SentinelConfig;
use crate::stats::SentinelStats;

/// Ejecuta los tests de un archivo específico usando Jest.
pub fn ejecutar_tests(test_path: &str, project_path: &Path) -> Result<(), String> {
    println!("🧪 Ejecutando tests: {}", test_path.cyan());

    let output = Command::new("npx")
        .args(["jest", test_path, "--passWithNoTests"])
        .current_dir(project_path)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Pide ayuda a la IA cuando un test falla.
pub fn pedir_ayuda_test(
    codigo: &str,
    error_jest: &str,
    config: &SentinelConfig,
    stats: Arc<Mutex<SentinelStats>>,
    project_path: &Path,
) -> anyhow::Result<()> {
    println!(
        "{}",
        "🔍 Pidiendo ayuda a la IA para resolver el fallo...".magenta()
    );

    let prompt = format!(
        "El siguiente test de NestJS falló con este error:\n\n{}\n\nCódigo del archivo modificado:\n{}\n\nAnaliza el error y sugiere una solución técnica concisa.",
        error_jest, codigo
    );

    let respuesta =
        ai::consultar_ia_dinamico(prompt, ai::TaskType::Deep, config, stats, project_path)?;

    println!("\n💡 SOLUCIÓN SUGERIDA:\n{}", respuesta.yellow());
    Ok(())
}
