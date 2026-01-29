//! Módulo de inteligencia artificial
//!
//! Funciones relacionadas con la comunicación y análisis usando Claude AI.

use reqwest::blocking::Client;
use serde_json::json;
use std::fs;

/// Realiza una consulta al API de Claude AI (Anthropic).
///
/// # Argumentos
///
/// * `prompt` - El prompt a enviar a Claude, generalmente código + instrucciones
///
/// # Variables de entorno requeridas
///
/// * `ANTHROPIC_AUTH_TOKEN` - API key de Anthropic
/// * `ANTHROPIC_BASE_URL` - URL base de la API (ej: https://api.anthropic.com)
///
/// # Retorna
///
/// * `Ok(String)` - Respuesta de texto generada por Claude
/// * `Err` - Error de red, autenticación o respuesta malformada
///
/// # Ejemplo
///
/// ```no_run
/// let respuesta = consultar_claude("Analiza este código: fn main() {}".to_string())?;
/// println!("Claude dice: {}", respuesta);
/// ```

pub fn consultar_claude(prompt: String) -> anyhow::Result<String> {
    let api_key = std::env::var("ANTHROPIC_AUTH_TOKEN").expect("❌ Falta ANTHROPIC_AUTH_TOKEN");
    let base_url = std::env::var("ANTHROPIC_BASE_URL").expect("❌ Falta ANTHROPIC_BASE_URL");
    let url = format!("{}/v1/messages", base_url.trim_end_matches('/'));

    let client = Client::new();
    let response = client.post(&url)
        .header("x-api-key", api_key)
        .header("content-type", "application/json")
        .json(&json!({
            "model": "claude-opus-4-5-20251101",
            "max_tokens": 1500,
            "messages": [{"role": "user", "content": prompt}]
        }))
        .send()?;

    let body: serde_json::Value = response.json()?;
    let texto = body["content"][0]["text"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Respuesta vacía de la IA"))?;
    
    Ok(texto.to_string())
}

/// Analiza código TypeScript/NestJS con Claude AI enfocándose en arquitectura y buenas prácticas.
///
/// Evalúa principios SOLID, Clean Code y patrones de NestJS. Si encuentra problemas críticos,
/// Claude responderá comenzando con "CRITICO", de lo contrario con "SEGURO".
///
/// # Argumentos
///
/// * `codigo` - Código fuente a analizar
/// * `file_name` - Nombre del archivo (para contexto en el prompt)
///
/// # Retorna
///
/// * `Ok(true)` - Código aprobado (sin problemas críticos)
/// * `Ok(false)` - Código rechazado (problemas críticos detectados)
/// * `Err` - Error de comunicación con la IA
///
/// # Efectos secundarios
///
/// Crea un archivo `{file_name}.suggested` con la versión mejorada del código.

pub fn analizar_arquitectura(codigo: &str, file_name: &str) -> anyhow::Result<bool> {
    let prompt = format!(
        "Actúa como un Arquitecto de Software experto en NestJS. Analiza {}.\n\
        REGLAS: Inicia con 'CRITICO' si hay fallos graves, o 'SEGURO' si está bien.\n\
        Incluye el código mejorado en un bloque ```typescript.\n\nCódigo:\n{}", 
        file_name, codigo
    );

    let respuesta = consultar_claude(prompt)?;

    let sugerencia = extraer_codigo(&respuesta);
    fs::write(format!("{}.suggested", file_name), &sugerencia)?;

    let consejo = eliminar_bloques_codigo(&respuesta);
    println!("\n✨ CONSEJO DE CLAUDE:\n{}", consejo);

    Ok(!respuesta.trim().to_uppercase().starts_with("CRITICO"))
}

/// Extrae bloques de código TypeScript de una respuesta de Claude.
///
/// Busca y extrae el contenido entre delimitadores \`\`\`typescript...\`\`\`.
/// Si no encuentra un bloque delimitado, devuelve el texto completo.
///
/// # Argumentos
///
/// * `texto` - Respuesta completa de Claude AI
///
/// # Retorna
///
/// Código TypeScript extraído (sin delimitadores) o el texto original.

fn eliminar_bloques_codigo(texto: &str) -> String {
    let mut resultado = String::new();
    let mut dentro_bloque = false;
    for linea in texto.lines() {
        if linea.trim_start().starts_with("```") {
            dentro_bloque = !dentro_bloque;
            continue;
        }
        if !dentro_bloque {
            resultado.push_str(linea);
            resultado.push('\n');
        }
    }
    resultado.trim().to_string()
}

pub fn extraer_codigo(texto: &str) -> String {
    if let Some(start) = texto.find("```typescript") {
        let resto = &texto[start + 13..];
        if let Some(end) = resto.find("```") { return resto[..end].trim().to_string(); }
    }
    texto.to_string()
}