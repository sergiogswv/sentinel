//! Módulo de interfaz de usuario
//!
//! Funciones relacionadas con la interacción con el usuario en la terminal.

use std::path::PathBuf;
use std::fs;
use std::io::{self, Write};
use colored::*;

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
pub fn seleccionar_proyecto() -> PathBuf {
    println!("{}", "\n📂 Proyectos detectados:".bright_cyan().bold());

    let entries = match fs::read_dir("../") {
        Ok(e) => e,
        Err(e) => {
            eprintln!("{}", "❌ Error al leer el directorio padre.".red().bold());
            eprintln!("   Error: {}", e);
            std::process::exit(1);
        }
    };

    let proyectos: Vec<PathBuf> = entries.flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();

    if proyectos.is_empty() {
        eprintln!("{}", "❌ No se encontraron proyectos en el directorio padre.".red().bold());
        std::process::exit(1);
    }

    for (i, p) in proyectos.iter().enumerate() {
        let nombre = p.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<nombre inválido>");
        println!("{}. {}", i + 1, nombre);
    }

    print!("\n👉 Selecciona número: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let idx = match input.trim().parse::<usize>() {
        Ok(n) if n > 0 && n <= proyectos.len() => n - 1,
        _ => {
            eprintln!("❌ Selección inválida. Usa un número entre 1 y {}", proyectos.len());
            std::process::exit(1);
        }
    };

    proyectos[idx].clone()
}
