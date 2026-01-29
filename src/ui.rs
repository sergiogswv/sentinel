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
