//! Módulo de interfaz de usuario
//!
//! Funciones relacionadas con la interacción con el usuario en la terminal.

use crate::ai;
use crate::config::{ModelConfig, SentinelConfig};
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Muestra el banner ASCII art de Sentinel al inicio del programa
pub fn mostrar_banner() {
    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", r"
   ███████╗███████╗███╗   ██╗████████╗██╗███╗   ██╗███████╗██╗     
   ██╔════╝██╔════╝████╗  ██║╚══██╔══╝██║████╗  ██║██╔════╝██║     
   ███████╗█████╗  ██╔██╗ ██║   ██║   ██║██╔██╗ ██║█████╗  ██║     
   ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║██║╚██╗██║██╔══╝  ██║     
   ███████║███████╗██║ ╚████║   ██║   ██║██║ ╚████║███████╗███████╗
   ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝
".bright_cyan().bold());
    println!("{}", "╚═══════════════════════════════════════════════════════════╝".bright_cyan());
    println!();
    println!("{}", "              🛡️  AI-Powered Code Monitor  🛡️".bright_white().bold());
    println!("{}", "              ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "                 Vigilando tu código 24/7 ⚡".bright_yellow());
    println!();
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

    let proyectos: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();

    if proyectos.is_empty() {
        eprintln!(
            "{}",
            "❌ No se encontraron proyectos en el directorio padre."
                .red()
                .bold()
        );
        std::process::exit(1);
    }

    for (i, p) in proyectos.iter().enumerate() {
        let nombre = p
            .file_name()
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
            eprintln!(
                "❌ Selección inválida. Usa un número entre 1 y {}",
                proyectos.len()
            );
            std::process::exit(1);
        }
    };

    proyectos[idx].clone()
}

/// Muestra la ayuda de comandos disponibles
pub fn mostrar_ayuda() {
    println!(
        "\n{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan()
    );
    println!("{}", "⌨️  COMANDOS DISPONIBLES".bright_cyan().bold());
    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan()
    );
    println!("{}", "  p       Pausar/Reanudar monitoreo".dimmed());
    println!(
        "{}",
        "  r       Generar reporte diario de productividad".dimmed()
    );
    println!(
        "{}",
        "  m       Ver dashboard de métricas (bugs, costos, tokens)".dimmed()
    );
    println!("{}", "  l       Limpiar caché de respuestas de IA".dimmed());
    println!(
        "{}",
        "  x       Reiniciar configuración desde cero".dimmed()
    );
    println!("{}", "  h/help  Mostrar esta ayuda".dimmed());
    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n".bright_cyan()
    );
}

pub fn inicializar_sentinel(project_path: &Path) -> SentinelConfig {
    if let Some(config) = SentinelConfig::load(project_path) {
        println!(
            "{}",
            "🔄 Configuración cargada desde .sentinelrc.toml".green()
        );
        return config;
    }

    println!(
        "{}",
        "🚀 Configurando nuevo proyecto en Sentinel...".bright_cyan()
    );

    let gestor = SentinelConfig::detectar_gestor(project_path);
    let nombre = project_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // Crear config temporal para poder detectar framework con IA
    let mut config = SentinelConfig::default(
        nombre.clone(),
        gestor.clone(),
        "Detectando...".to_string(),
        vec!["Analizando proyecto...".to_string()],
        vec!["js".to_string(), "ts".to_string()], // Extensiones temporales
    );

    println!(
        "\n{}",
        "🤖 Configuración de Modelos AI".bright_magenta().bold()
    );

    // 1. Configurar Modelo Principal
    println!("\n--- MODELO PRINCIPAL ---");
    print!("👉 API Key: ");
    io::stdout().flush().unwrap();
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key).unwrap();
    config.primary_model.api_key = api_key.trim().to_string();

    print!("👉 URL [Enter para Anthropic]: ");
    io::stdout().flush().unwrap();
    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap();
    if !url.trim().is_empty() {
        config.primary_model.url = url.trim().to_string();
    }

    // Listar modelos si es Gemini
    if config.primary_model.url.contains("googleapis") {
        if let Ok(modelos) = ai::listar_modelos_gemini(&config.primary_model.api_key) {
            println!("{}", "📂 Modelos disponibles:".cyan());
            for (i, m) in modelos.iter().enumerate() {
                println!("{}. {}", i + 1, m);
            }
            print!("👉 Selecciona número: ");
            io::stdout().flush().unwrap();
            let mut sel = String::new();
            io::stdin().read_line(&mut sel).unwrap();
            if let Ok(idx) = sel.trim().parse::<usize>() {
                if idx > 0 && idx <= modelos.len() {
                    config.primary_model.name = modelos[idx - 1].clone();
                }
            }
        }
    }

    // 2. Configurar Modelo de Fallback (Opcional)
    println!("\n--- MODELO DE FALLBACK (Opcional) ---");
    print!("👉 ¿Configurar un modelo de respaldo por si falla el principal? (s/n): ");
    io::stdout().flush().unwrap();
    let mut use_fallback = String::new();
    io::stdin().read_line(&mut use_fallback).unwrap();

    if use_fallback.trim().to_lowercase() == "s" {
        let mut fb = ModelConfig::default();
        print!("👉 API Key: ");
        io::stdout().flush().unwrap();
        let mut ak = String::new();
        io::stdin().read_line(&mut ak).unwrap();
        fb.api_key = ak.trim().to_string();

        print!("👉 URL del modelo: ");
        io::stdout().flush().unwrap();
        let mut u = String::new();
        io::stdin().read_line(&mut u).unwrap();
        fb.url = u.trim().to_string();

        print!("👉 Nombre del modelo: ");
        io::stdout().flush().unwrap();
        let mut nm = String::new();
        io::stdin().read_line(&mut nm).unwrap();
        fb.name = nm.trim().to_string();

        config.fallback_model = Some(fb);
    }

    // 3. Detectar framework con IA
    println!(
        "\n{}",
        "🔍 Detectando framework del proyecto...".bright_cyan().bold()
    );

    let deteccion = match ai::detectar_framework_con_ia(project_path, &config) {
        Ok(d) => d,
        Err(e) => {
            println!(
                "   ⚠️  Error al detectar framework: {}. Usando valores por defecto.",
                e.to_string().yellow()
            );
            crate::config::FrameworkDetection {
                framework: "JavaScript/TypeScript".to_string(),
                rules: vec![
                    "Clean Code".to_string(),
                    "SOLID Principles".to_string(),
                    "Best Practices".to_string(),
                    "Code Maintainability".to_string(),
                ],
                extensions: vec!["js".to_string(), "ts".to_string()],
            }
        }
    };

    // Actualizar config con framework, reglas y extensiones detectadas
    config.framework = deteccion.framework;
    config.architecture_rules = deteccion.rules;
    config.file_extensions = deteccion.extensions;

    let _ = config.save(project_path);
    println!("{}", "✅ Configuración guardada.".green());
    config
}
