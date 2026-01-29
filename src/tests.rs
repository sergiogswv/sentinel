//! Módulo de testing
//!
//! Funciones relacionadas con la ejecución y diagnóstico de tests de Jest.

use std::process::Command;
use std::path::Path;
use colored::*;
use crate::ai;

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
pub fn ejecutar_tests(test_path: &str, project_path: &Path) -> Result<(), String> {
    println!("\n🧪 Ejecutando Jest para: {}", test_path.cyan());
    let status = Command::new("npm")
        .args(["run", "test", "--", "--findRelatedTests", test_path])
        .current_dir(project_path)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Tests fallaron con código de salida: {}", status))
    }
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
pub fn pedir_ayuda_test(codigo: &str, error_jest: &str) -> anyhow::Result<()> {
    println!("{}", "🔍 Analizando fallo en tests...".magenta());
    let prompt = format!(
        "Los tests de NestJS fallaron.\nERROR DE JEST:\n{}\n\nCÓDIGO:\n{}\nExplica el fallo y dame el fix breve.",
        error_jest, codigo
    );

    let respuesta = ai::consultar_claude(prompt)?;
    println!("\n💡 SOLUCIÓN SUGERIDA:\n{}", respuesta.yellow());
    Ok(())
}
