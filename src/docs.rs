//! Módulo de documentación
//!
//! Funciones para generar documentación automática de archivos modificados.

use crate::ai;
use colored::*;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::config::SentinelConfig;
use crate::stats::SentinelStats;

/// Genera un "manual de bolsillo" automático para cada archivo modificado.
pub fn actualizar_documentacion(
    codigo: &str,
    file_path: &Path,
    config: &SentinelConfig,
    stats: Arc<Mutex<SentinelStats>>,
    project_path: &Path,
) -> anyhow::Result<()> {
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    println!(
        "📚 Actualizando manual de bolsillo para: {}",
        file_name.magenta()
    );

    let prompt = format!(
        "Como documentador técnico de NestJS, analiza este código: {}. \
        Genera un resumen técnico ultra-conciso (máximo 150 palabras) en Markdown. \
        Enfócate en: ¿Qué hace este servicio? y ¿Cuáles son sus métodos principales? \
        Usa emojis para las secciones. No uses introducciones innecesarias.\n\n{}",
        file_name, codigo
    );

    let resumen =
        ai::consultar_ia_dinamico(prompt, ai::TaskType::Light, config, stats, project_path)?;

    let mut docs_path = file_path.to_path_buf();
    docs_path.set_extension("md");

    let nueva_doc = format!(
        "# 📖 Documentación: {}\n\n> ✨ Actualizado automáticamente por Sentinel v{}\n\n{}\n\n---\n*Último refactor: {:?}*",
        file_name,
        crate::config::SENTINEL_VERSION,
        resumen,
        std::time::SystemTime::now()
    );

    fs::write(&docs_path, nueva_doc)?;
    println!("   ✅ Documento generado: {}", docs_path.display());
    Ok(())
}
