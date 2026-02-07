//! Detección y validación de frameworks de testing
//!
//! Analiza el proyecto para identificar frameworks de testing instalados,
//! valida sus configuraciones y sugiere alternativas apropiadas basadas
//! en el framework principal detectado.

use crate::ai::client::consultar_ia;
use crate::config::SentinelConfig;
use crate::stats::SentinelStats;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Información sobre un framework de testing detectado
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestingFrameworkInfo {
    /// Framework de testing principal detectado (ej: "Jest", "Pytest", "Vitest")
    pub testing_framework: Option<String>,
    /// Frameworks adicionales detectados
    pub additional_frameworks: Vec<String>,
    /// Archivos de configuración encontrados
    pub config_files: Vec<String>,
    /// Estado de la configuración (valid, incomplete, missing)
    pub status: TestingStatus,
    /// Sugerencias de frameworks a instalar si no hay ninguno
    pub suggestions: Vec<TestingSuggestion>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TestingStatus {
    Valid,      // Framework de testing configurado correctamente
    Incomplete, // Framework detectado pero configuración incompleta
    Missing,    // No se detectó ningún framework de testing
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestingSuggestion {
    pub framework: String,
    pub reason: String,
    pub install_command: String,
    pub priority: u8, // 1 = alta prioridad, 3 = baja prioridad
}

// Función eliminada - ahora todo es dinámico con IA

/// Detecta frameworks de testing usando análisis básico + IA dinámica
pub fn detectar_testing_framework(
    project_path: &Path,
    config: &SentinelConfig,
) -> anyhow::Result<TestingFrameworkInfo> {
    println!("{}", "🧪 Detectando frameworks de testing...".cyan());

    // 1. Análisis estático rápido: detectar qué archivos de config/deps existen
    let analisis_estatico = analizar_archivos_proyecto(project_path);

    // 2. La IA hace todo el análisis dinámico
    let testing_info = consultar_ia_para_testing_dinamico(
        project_path,
        config,
        &analisis_estatico,
    )?;

    // Mostrar resultados
    mostrar_resumen_testing(&testing_info);

    Ok(testing_info)
}

/// Análisis estático rápido de archivos del proyecto
fn analizar_archivos_proyecto(project_path: &Path) -> AnalisisEstatico {
    let mut archivos_config = Vec::new();
    let mut contenido_deps = String::new();

    // Buscar archivos de configuración de testing comunes
    let archivos_comunes = vec![
        "jest.config.js", "jest.config.ts", "jest.config.json",
        "vitest.config.js", "vitest.config.ts",
        "cypress.json", "cypress.config.js", "cypress.config.ts",
        "playwright.config.js", "playwright.config.ts",
        "karma.conf.js", "pytest.ini", "pyproject.toml",
        "phpunit.xml", "phpunit.xml.dist", "pest.php",
        ".rspec", "spec_helper.rb",
    ];

    for archivo in archivos_comunes {
        if project_path.join(archivo).exists() {
            archivos_config.push(archivo.to_string());
        }
    }

    // Leer archivos de dependencias si existen
    if let Ok(content) = fs::read_to_string(project_path.join("package.json")) {
        contenido_deps.push_str("\n=== package.json ===\n");
        // Solo primeras 100 líneas para no saturar
        contenido_deps.push_str(&content.lines().take(100).collect::<Vec<_>>().join("\n"));
    }

    if let Ok(content) = fs::read_to_string(project_path.join("requirements.txt")) {
        contenido_deps.push_str("\n=== requirements.txt ===\n");
        contenido_deps.push_str(&content.lines().take(50).collect::<Vec<_>>().join("\n"));
    }

    if let Ok(content) = fs::read_to_string(project_path.join("composer.json")) {
        contenido_deps.push_str("\n=== composer.json ===\n");
        contenido_deps.push_str(&content.lines().take(50).collect::<Vec<_>>().join("\n"));
    }

    if let Ok(content) = fs::read_to_string(project_path.join("Cargo.toml")) {
        contenido_deps.push_str("\n=== Cargo.toml ===\n");
        contenido_deps.push_str(&content.lines().take(50).collect::<Vec<_>>().join("\n"));
    }

    if let Ok(content) = fs::read_to_string(project_path.join("go.mod")) {
        contenido_deps.push_str("\n=== go.mod ===\n");
        contenido_deps.push_str(&content.lines().take(50).collect::<Vec<_>>().join("\n"));
    }

    if let Ok(content) = fs::read_to_string(project_path.join("pom.xml")) {
        contenido_deps.push_str("\n=== pom.xml ===\n");
        contenido_deps.push_str(&content.lines().take(50).collect::<Vec<_>>().join("\n"));
    }

    AnalisisEstatico {
        archivos_config,
        contenido_deps,
    }
}

struct AnalisisEstatico {
    archivos_config: Vec<String>,
    contenido_deps: String,
}

/// Consulta a la IA para análisis completo y dinámico de testing
fn consultar_ia_para_testing_dinamico(
    project_path: &Path,
    config: &SentinelConfig,
    analisis: &AnalisisEstatico,
) -> anyhow::Result<TestingFrameworkInfo> {
    let archivos_raiz = SentinelConfig::listar_archivos_raiz(project_path);

    let prompt = format!(
        "Eres un experto en Testing de Software y arquitectura. Analiza este proyecto y determina su estrategia de testing.\n\n\
        CONTEXTO DEL PROYECTO:\n\
        - Framework principal: {}\n\
        - Lenguaje: {}\n\
        - Gestor de paquetes: {}\n\
        - Archivos raíz: {}\n\n\
        ARCHIVOS DE CONFIGURACIÓN DE TESTING ENCONTRADOS:\n\
        {}\n\n\
        CONTENIDO DE ARCHIVOS DE DEPENDENCIAS:\n\
        {}\n\n\
        TAREAS:\n\
        1. DETECTAR frameworks de testing instalados/configurados (analiza las dependencias)\n\
        2. DETERMINAR el estado:\n\
           - \"valid\": Configuración completa y funcional\n\
           - \"incomplete\": Framework detectado pero falta configuración\n\
           - \"missing\": No hay frameworks de testing\n\
        3. Si falta testing o está incompleto, SUGERIR frameworks apropiados:\n\
           - Considera el framework principal ({})\n\
           - Prioriza estándares de la industria actuales (2024-2025)\n\
           - Incluye frameworks para unit testing, integration testing y E2E\n\
           - Genera comandos de instalación correctos para el gestor: {}\n\
        4. PRIORIZA sugerencias: 1 (alta/recomendado), 2 (alternativa), 3 (adicional)\n\n\
        IMPORTANTE SOBRE COMANDOS:\n\
        - Para npm: 'npm install --save-dev <package>'\n\
        - Para yarn: 'yarn add --dev <package>'\n\
        - Para pnpm: 'pnpm add -D <package>'\n\
        - Para pip: 'pip install <package>'\n\
        - Para composer: 'composer require --dev <package>'\n\
        - Para cargo: 'cargo add --dev <package>' o manual en Cargo.toml\n\
        - Para go: 'go get <package>' o manual en go.mod\n\n\
        RESPONDE SOLO CON JSON VÁLIDO:\n\
        {{\n\
          \"testing_framework\": \"Framework principal o null\",\n\
          \"additional_frameworks\": [\"otros frameworks\"],\n\
          \"config_files\": [\"archivos de config encontrados\"],\n\
          \"status\": \"valid\"|\"incomplete\"|\"missing\",\n\
          \"suggestions\": [\n\
            {{\n\
              \"framework\": \"nombre del framework\",\n\
              \"reason\": \"por qué es apropiado para este proyecto\",\n\
              \"install_command\": \"comando completo de instalación\",\n\
              \"priority\": 1-3\n\
            }}\n\
          ]\n\
        }}\n\n\
        Responde ÚNICAMENTE con el JSON, sin explicaciones adicionales.",
        config.framework,
        config.code_language,
        config.manager,
        archivos_raiz.join(", "),
        if analisis.archivos_config.is_empty() {
            "Ninguno".to_string()
        } else {
            analisis.archivos_config.join(", ")
        },
        if analisis.contenido_deps.is_empty() {
            "No se encontraron archivos de dependencias".to_string()
        } else {
            analisis.contenido_deps.clone()
        },
        config.framework,
        config.manager
    );

    let respuesta = consultar_ia(
        prompt,
        &config.primary_model.api_key,
        &config.primary_model.url,
        &config.primary_model.name,
        Arc::new(Mutex::new(SentinelStats::default())),
    )?;

    parsear_testing_info(&respuesta)
}

/// Parsea la respuesta JSON de la IA
fn parsear_testing_info(respuesta: &str) -> anyhow::Result<TestingFrameworkInfo> {
    let json_str = if let Some(inicio) = respuesta.find('{') {
        if let Some(fin) = respuesta.rfind('}') {
            &respuesta[inicio..=fin]
        } else {
            respuesta
        }
    } else {
        respuesta
    };

    match serde_json::from_str::<TestingFrameworkInfo>(json_str) {
        Ok(info) => {
            println!("   ✅ Análisis de testing completado");
            Ok(info)
        }
        Err(e) => {
            println!("   ⚠️  Error al parsear respuesta: {}", e.to_string().yellow());
            println!("   Respuesta recibida: {}", json_str.chars().take(200).collect::<String>());
            // Fallback básico
            Ok(TestingFrameworkInfo {
                testing_framework: None,
                additional_frameworks: vec![],
                config_files: vec![],
                status: TestingStatus::Missing,
                suggestions: vec![],
            })
        }
    }
}

// Función eliminada - Los comandos de instalación ahora son generados dinámicamente por la IA

/// Obtiene sugerencias complementarias para un proyecto que ya tiene testing configurado
pub fn obtener_sugerencias_complementarias(
    project_path: &Path,
    config: &SentinelConfig,
    testing_actual: &str,
) -> anyhow::Result<Vec<TestingSuggestion>> {
    println!("\n{}", "🔍 Analizando frameworks complementarios...".cyan());

    let analisis = analizar_archivos_proyecto(project_path);

    let prompt = format!(
        "Eres un experto en Testing de Software. El proyecto YA TIENE testing configurado.\n\n\
        CONTEXTO DEL PROYECTO:\n\
        - Framework principal: {}\n\
        - Lenguaje: {}\n\
        - Gestor de paquetes: {}\n\
        - Testing actual: {}\n\n\
        DEPENDENCIAS ACTUALES:\n\
        {}\n\n\
        TAREA:\n\
        Sugiere frameworks de testing COMPLEMENTARIOS que añadan valor al stack actual:\n\
        1. Si solo tiene unit testing (ej: Jest), sugiere E2E (Cypress, Playwright)\n\
        2. Si falta coverage, sugiere herramientas de cobertura\n\
        3. Si es backend, sugiere testing de integración o carga\n\
        4. NO repitas frameworks que ya están instalados\n\
        5. Prioriza por utilidad: 1 (muy recomendado), 2 (útil), 3 (opcional)\n\n\
        RESPONDE SOLO CON JSON:\n\
        {{\n\
          \"suggestions\": [\n\
            {{\n\
              \"framework\": \"nombre\",\n\
              \"reason\": \"qué añade al stack actual de testing\",\n\
              \"install_command\": \"comando completo con gestor {}\",\n\
              \"priority\": 1-3\n\
            }}\n\
          ]\n\
        }}\n\n\
        Si no hay sugerencias útiles, retorna array vacío. Responde SOLO JSON.",
        config.framework,
        config.code_language,
        config.manager,
        testing_actual,
        if analisis.contenido_deps.is_empty() {
            "No disponible".to_string()
        } else {
            analisis.contenido_deps
        },
        config.manager
    );

    let respuesta = consultar_ia(
        prompt,
        &config.primary_model.api_key,
        &config.primary_model.url,
        &config.primary_model.name,
        Arc::new(Mutex::new(SentinelStats::default())),
    )?;

    // Extraer JSON
    let json_str = if let Some(inicio) = respuesta.find('{') {
        if let Some(fin) = respuesta.rfind('}') {
            &respuesta[inicio..=fin]
        } else {
            &respuesta
        }
    } else {
        &respuesta
    };

    #[derive(Deserialize)]
    struct SugerenciasComplementarias {
        suggestions: Vec<TestingSuggestion>,
    }

    match serde_json::from_str::<SugerenciasComplementarias>(json_str) {
        Ok(result) => {
            println!("   ✅ Análisis completado");
            Ok(result.suggestions)
        }
        Err(_) => {
            println!("   ⚠️  No se encontraron sugerencias complementarias");
            Ok(vec![])
        }
    }
}

/// Muestra un resumen colorido del análisis de testing
fn mostrar_resumen_testing(info: &TestingFrameworkInfo) {
    println!("\n{}", "═══ ANÁLISIS DE TESTING ═══".bold().cyan());

    match info.status {
        TestingStatus::Valid => {
            println!("   {} Testing configurado correctamente", "✅".green());
            if let Some(main) = &info.testing_framework {
                println!("   📦 Framework principal: {}", main.green().bold());
            }
            if !info.additional_frameworks.is_empty() {
                println!("   🔧 Frameworks adicionales: {}",
                    info.additional_frameworks.join(", ").cyan());
            }
            if !info.config_files.is_empty() {
                println!("   📄 Configuración encontrada:");
                for file in &info.config_files {
                    println!("      • {}", file.yellow());
                }
            }
        }
        TestingStatus::Incomplete => {
            println!("   {} Configuración de testing incompleta", "⚠️".yellow());
            if let Some(main) = &info.testing_framework {
                println!("   📦 Framework detectado: {}", main.yellow());
            }
            println!("   💡 Recomendación: Completar configuración o instalar herramientas");
        }
        TestingStatus::Missing => {
            println!("   {} No se detectaron frameworks de testing", "❌".red());
            println!("   💡 Se recomienda configurar testing para el proyecto");
        }
    }

    if !info.suggestions.is_empty() {
        println!("\n   {}", "SUGERENCIAS DE INSTALACIÓN:".bold().yellow());
        for (i, suggestion) in info.suggestions.iter().enumerate() {
            let priority_icon = match suggestion.priority {
                1 => "🔥",
                2 => "⭐",
                _ => "💡",
            };
            println!("\n   {} {}. {}", priority_icon, i + 1, suggestion.framework.bold());
            println!("      📝 {}", suggestion.reason);
            println!("      💻 {}", suggestion.install_command.cyan());
        }
    }

    println!("{}", "═══════════════════════════".cyan());
}
