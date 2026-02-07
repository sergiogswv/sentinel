//! Sistema de caché para optimizar consultas a IA
//!
//! Guarda respuestas de IA en disco para evitar consultas repetidas.
//! Usa hash del prompt como identificador del caché.

use colored::*;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

fn obtener_cache_path(prompt: &str, project_path: &Path) -> PathBuf {
    let mut s = DefaultHasher::new();
    prompt.hash(&mut s);
    let hash = s.finish();
    project_path
        .join(".sentinel/cache")
        .join(format!("{:x}.cache", hash))
}

pub fn intentar_leer_cache(prompt: &str, project_path: &Path) -> Option<String> {
    let path = obtener_cache_path(prompt, project_path);
    fs::read_to_string(path).ok()
}

pub fn guardar_en_cache(prompt: &str, respuesta: &str, project_path: &Path) -> anyhow::Result<()> {
    let cache_dir = project_path.join(".sentinel/cache");
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir)?;
    }
    let path = obtener_cache_path(prompt, project_path);
    fs::write(path, respuesta)?;
    Ok(())
}

/// Limpia completamente el caché de Sentinel
pub fn limpiar_cache(project_path: &Path) -> anyhow::Result<()> {
    let cache_dir = project_path.join(".sentinel/cache");

    if cache_dir.exists() {
        fs::remove_dir_all(&cache_dir)?;
        println!("{}", "   🗑️  Caché limpiado exitosamente.".green());
        println!(
            "{}",
            "   💡 El caché se regenerará automáticamente en las próximas consultas.".dimmed()
        );
    } else {
        println!("{}", "   ℹ️  No hay caché para limpiar.".yellow());
    }

    Ok(())
}
