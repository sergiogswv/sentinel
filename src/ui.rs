//! Módulo de interfaz de usuario
//!
//! Funciones relacionadas con la interacción con el usuario en la terminal.

use crate::ai;
use crate::config::{AIConfig, AIProvider, SentinelConfig};
use colored::*;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Muestra el banner ASCII art de Sentinel al inicio del programa
pub fn mostrar_banner() {
    println!();
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        r"
   ███████╗███████╗███╗   ██╗████████╗██╗███╗   ██╗███████╗██╗     
   ██╔════╝██╔════╝████╗  ██║╚══██╔══╝██║████╗  ██║██╔════╝██║     
   ███████╗█████╗  ██╔██╗ ██║   ██║   ██║██╔██╗ ██║█████╗  ██║     
   ╚════██║██╔══╝  ██║╚██╗██║   ██║   ██║██║╚██╗██║██╔══╝  ██║     
   ███████║███████╗██║ ╚████║   ██║   ██║██║ ╚████║███████╗███████╗
   ╚══════╝╚══════╝╚═╝  ╚═══╝   ╚═╝   ╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝
"
        .bright_cyan()
        .bold()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════╝".bright_cyan()
    );
    println!();
    println!(
        "{}",
        "              🛡️  AI-Powered Code Monitor  🛡️"
            .bright_white()
            .bold()
    );
    println!(
        "{}",
        "              ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan()
    );
    println!(
        "{}",
        "                 Vigilando tu código 24/7 ⚡".bright_yellow()
    );
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
pub fn mostrar_ayuda(config: Option<&SentinelConfig>) {
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

    // Mostrar comando T solo si hay testing configurado
    if let Some(cfg) = config {
        if cfg.testing_framework.is_some()
            && cfg.testing_status.as_ref().map_or(false, |s| s == "valid")
        {
            println!(
                "{}",
                "  t       Ver sugerencias de testing complementarias".dimmed()
            );
        }
    }

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
    let gestor = SentinelConfig::detectar_gestor(project_path);
    let nombre = project_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let mut existia_config = false;
    let mut config = if let Some(cfg) = SentinelConfig::load(project_path) {
        println!("{}", "🔄 Configuración existente encontrada".yellow());
        println!("   💾 Preservando API keys y configuraciones personalizadas...");
        existia_config = true;
        cfg
    } else {
        // Nueva configuración - pedir API keys
        println!(
            "{}",
            "🚀 Configurando nuevo proyecto en Sentinel...".bright_cyan()
        );

        let mut config = SentinelConfig::default(
            nombre.clone(),
            gestor.clone(),
            "Detectando...".to_string(),
            vec!["Analizando proyecto...".to_string()],
            vec!["js".to_string(), "ts".to_string()],
            "typescript".to_string(),
            vec![],
            vec![],
        );

        println!(
            "\n{}",
            "🤖 Configuración de Modelos AI".bright_magenta().bold()
        );

        config.ai_configs = ask_ai_configs().unwrap_or_else(|e| {
            eprintln!("⚠️  Error al configurar modelos: {}. Usando default.", e);
            vec![AIConfig {
                name: "Claude Default".to_string(),
                provider: AIProvider::Claude,
                api_url: "https://api.anthropic.com".to_string(),
                api_key: "".to_string(),
                model: "claude-3-5-sonnet-20241022".to_string(),
            }]
        });

        let _ = config.save(project_path);
        config
    };

    // Guardar framework actual para comparar
    let framework_actual = config.framework.clone();
    let tiene_config_existente = existia_config;

    // Detectar framework con IA (silenciosamente)
    let deteccion = match ai::detectar_framework_con_ia(project_path, &config) {
        Ok(d) => d,
        Err(e) => {
            println!(
                "   ⚠️  Error al detectar framework: {}",
                e.to_string().yellow()
            );
            if tiene_config_existente {
                println!("   ℹ️  Manteniendo configuración actual");
                return config;
            }
            crate::config::FrameworkDetection {
                framework: "Generic".to_string(),
                rules: vec![
                    "Clean Code principles".to_string(),
                    "SOLID design patterns".to_string(),
                    "Code maintainability".to_string(),
                    "Comprehensive testing".to_string(),
                ],
                extensions: vec!["js".to_string(), "ts".to_string()],
                code_language: "typescript".to_string(),
                parent_patterns: vec![],
                test_patterns: vec!["{name}.test.ts".to_string(), "{name}.spec.ts".to_string()],
            }
        }
    };

    // Comparar con framework actual
    if tiene_config_existente && deteccion.framework == framework_actual {
        println!(
            "   ✓ Framework: {} (sin cambios)",
            deteccion.framework.green()
        );

        // Detectar frameworks de testing si no está ya configurado
        if config.testing_framework.is_none() || config.testing_status.is_none() {
            match ai::detectar_testing_framework(project_path, &config) {
                Ok(testing_info) => {
                    config.testing_framework = testing_info.testing_framework;
                    config.testing_status = Some(match testing_info.status {
                        ai::TestingStatus::Valid => "valid".to_string(),
                        ai::TestingStatus::Incomplete => "incomplete".to_string(),
                        ai::TestingStatus::Missing => "missing".to_string(),
                    });
                    let _ = config.save(project_path);
                }
                Err(e) => {
                    println!(
                        "   ⚠️  Error al detectar testing framework: {}",
                        e.to_string().yellow()
                    );
                    println!("   ℹ️  Continuando sin detección de testing");
                }
            }
        } else {
            let default_fw = "N/A".to_string();
            let default_status = "unknown".to_string();
            let testing_fw = config.testing_framework.as_ref().unwrap_or(&default_fw);
            let testing_status = config.testing_status.as_ref().unwrap_or(&default_status);

            println!(
                "   ✓ Testing: {} ({})",
                testing_fw.green(),
                testing_status.cyan()
            );
        }

        return config;
    }

    // Hay cambios o es primera vez - mostrar y confirmar
    println!("\n{}", "📋 Framework Detectado:".bright_yellow().bold());
    println!("   Framework: {}", deteccion.framework.bright_green());
    println!("   Lenguaje: {}", deteccion.code_language.bright_green());
    println!(
        "   Extensiones: {}",
        deteccion.extensions.join(", ").bright_green()
    );

    if tiene_config_existente {
        println!(
            "\n   ⚠️  Cambio detectado: {} → {}",
            framework_actual.yellow(),
            deteccion.framework.green()
        );
    }

    print!("\n👉 ¿Es correcto? (s/n): ");
    io::stdout().flush().unwrap();
    let mut confirmacion = String::new();
    io::stdin().read_line(&mut confirmacion).unwrap();

    if confirmacion.trim().to_lowercase() != "s" {
        println!("   ℹ️  Manteniendo configuración actual");
        return config;
    }

    // Actualizar config con framework, reglas, extensiones, lenguaje y patrones detectados
    config.framework = deteccion.framework;
    config.architecture_rules = deteccion.rules;
    config.file_extensions = deteccion.extensions;
    config.code_language = deteccion.code_language;
    config.parent_patterns = deteccion.parent_patterns;
    config.test_patterns = deteccion.test_patterns;

    // Detectar frameworks de testing
    match ai::detectar_testing_framework(project_path, &config) {
        Ok(testing_info) => {
            config.testing_framework = testing_info.testing_framework.clone();
            config.testing_status = Some(match testing_info.status {
                ai::TestingStatus::Valid => "valid".to_string(),
                ai::TestingStatus::Incomplete => "incomplete".to_string(),
                ai::TestingStatus::Missing => "missing".to_string(),
            });
        }
        Err(e) => {
            println!(
                "   ⚠️  Error al detectar testing framework: {}",
                e.to_string().yellow()
            );
            println!("   ℹ️  Continuando sin detección de testing");
        }
    }

    match config.save(project_path) {
        Ok(_) => println!(
            "   💾 Configuración guardada en: {}",
            project_path
                .join(".sentinelrc.toml")
                .display()
                .to_string()
                .cyan()
        ),
        Err(e) => eprintln!("   ❌ Error al guardar la configuración: {}", e),
    }
    println!("{}", "✅ Configuración actualizada.".green());
    config
}

pub fn ask_ai_configs() -> anyhow::Result<Vec<AIConfig>> {
    let mut configs = Vec::new();

    loop {
        println!("\n🤖 CONFIGURACIÓN DE LA IA (#{})", configs.len() + 1);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Pedir un nombre para esta configuración
        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Nombre para esta configuración (ej: Claude Pro, Ollama Local)")
            .interact_text()?;

        let providers = vec![
            "Claude (Anthropic)",
            "Gemini (Google)",
            "OpenAI",
            "Groq",
            "Ollama (Local)",
            "Kimi (Moonshot)",
            "DeepSeek",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Selecciona un proveedor de IA")
            .items(&providers)
            .default(0)
            .interact()?;

        let provider = match selection {
            0 => crate::config::AIProvider::Claude,
            1 => crate::config::AIProvider::Gemini,
            2 => crate::config::AIProvider::OpenAI,
            3 => crate::config::AIProvider::Groq,
            4 => crate::config::AIProvider::Ollama,
            5 => crate::config::AIProvider::Kimi,
            6 => crate::config::AIProvider::DeepSeek,
            _ => unreachable!(),
        };

        // URLs base según el proveedor (Hardcoded)
        let default_url = match provider {
            crate::config::AIProvider::Claude => "https://api.anthropic.com".to_string(),
            crate::config::AIProvider::Gemini => {
                "https://generativelanguage.googleapis.com".to_string()
            }
            crate::config::AIProvider::OpenAI => "https://api.openai.com/v1".to_string(),
            crate::config::AIProvider::Groq => "https://api.groq.com/openai/v1".to_string(),
            crate::config::AIProvider::Ollama => "http://localhost:11434/v1".to_string(),
            crate::config::AIProvider::Kimi => "https://api.moonshot.ai/v1".to_string(),
            crate::config::AIProvider::DeepSeek => "https://api.deepseek.com".to_string(),
        };

        // Verificar si existen variables de entorno
        let env_url = env::var(format!("{}_BASE_URL", provider.as_str().to_uppercase())).ok();
        let env_key = env::var(format!("{}_API_KEY", provider.as_str().to_uppercase())).ok();

        // Solo pedimos la URL si es Ollama o si el usuario quiere cambiarla
        let api_url: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("URL de la API para {}", provider.as_str()))
            .default(env_url.unwrap_or(default_url))
            .interact_text()?;

        let api_key: String = if provider == crate::config::AIProvider::Ollama {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("API Key (opcional para Ollama)")
                .allow_empty(true)
                .default(env_key.unwrap_or_else(|| String::new()))
                .interact_text()?
        } else {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("API Key para {}", provider.as_str()))
                .default(env_key.unwrap_or_else(|| String::new()))
                .interact_text()?
        };

        // Obtener modelos dinámicamente
        println!(
            "🔍 Conectando con {} para obtener modelos...",
            provider.as_str()
        );
        let model: String =
            match crate::ai::obtener_modelos_disponibles(&provider, &api_url, &api_key) {
                Ok(mut models) if !models.is_empty() => {
                    models.sort();
                    let selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Selecciona el modelo")
                        .items(&models)
                        .default(0)
                        .interact()?;
                    models[selection].clone()
                }
                Err(e) => {
                    println!(
                        "⚠️  No se pudieron obtener los modelos automáticamente: {}",
                        e
                    );
                    Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        "Ingresa el nombre del modelo manualmente (ej: claude-3-5-sonnet-20241022)",
                    )
                    .interact_text()?
                }
                _ => {
                    println!("⚠️  La lista de modelos está vacía.");
                    Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Ingresa el nombre del modelo manualmente")
                        .interact_text()?
                }
            };

        configs.push(crate::config::AIConfig {
            name,
            provider,
            api_url,
            api_key,
            model,
        });

        println!("✅ Configuración añadida.");

        let add_another = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("¿Deseas agregar otro modelo de IA?")
            .default(false)
            .interact()?;

        if !add_another {
            break;
        }
    }

    Ok(configs)
}
