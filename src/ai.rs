use crate::config::{ModelConfig, SentinelConfig};
use crate::stats::SentinelStats;
use colored::*;
use reqwest::blocking::Client;
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
pub enum TaskType {
    Light, // Commits, docs
    Deep,  // Arquitectura, debug tests
}

/// Punto de entrada inteligente con Fallback y Caché
pub fn consultar_ia_dinamico(
    prompt: String,
    _task: TaskType, // Mantenemos el enum por compatibilidad pero lo ignoramos
    config: &SentinelConfig,
    stats: Arc<Mutex<SentinelStats>>,
    project_path: &Path,
) -> anyhow::Result<String> {
    // 1. Intentar Caché
    if config.use_cache {
        if let Some(res) = intentar_leer_cache(&prompt, project_path) {
            println!("{}", "   ♻️  Usando respuesta de caché...".dimmed());
            return Ok(res);
        }
    }

    // 2. Usar modelo primario
    let modelo_principal = &config.primary_model;

    // 3. Intentar ejecución con Fallback
    let resultado = ejecutar_con_fallback(
        prompt.clone(),
        modelo_principal,
        config.fallback_model.as_ref(),
        Arc::clone(&stats),
    );

    // 4. Guardar en Caché si tuvo éxito
    if let Ok(ref res) = resultado {
        if config.use_cache {
            let _ = guardar_en_cache(&prompt, res, project_path);
        }
    }

    resultado
}

fn ejecutar_con_fallback(
    prompt: String,
    principal: &ModelConfig,
    fallback: Option<&ModelConfig>,
    stats: Arc<Mutex<SentinelStats>>,
) -> anyhow::Result<String> {
    match consultar_ia(
        prompt.clone(),
        &principal.api_key,
        &principal.url,
        &principal.name,
        Arc::clone(&stats),
    ) {
        Ok(res) => Ok(res),
        Err(e) => {
            if let Some(fb) = fallback {
                println!(
                    "{}",
                    format!(
                        "   ⚠️  Modelo principal falló: {}. Intentando fallback con {}...",
                        e, fb.name
                    )
                    .yellow()
                );
                consultar_ia(prompt, &fb.api_key, &fb.url, &fb.name, stats)
            } else {
                Err(e)
            }
        }
    }
}
pub fn consultar_ia(
    prompt: String,
    api_key: &str,
    base_url: &str,
    model_name: &str,
    stats: Arc<Mutex<SentinelStats>>,
) -> anyhow::Result<String> {
    let client = Client::new();

    let prompt_len = prompt.len();
    let resultado = if base_url.contains("interactions") {
        consultar_gemini_interactions(&client, prompt, api_key, base_url, model_name)
    } else if base_url.contains("googleapis.com") {
        consultar_gemini_content(&client, prompt, api_key, base_url, model_name)
    } else {
        // Por defecto asumimos estructura Anthropic (Claude)
        consultar_anthropic(&client, prompt, api_key, base_url, model_name)
    };

    if let Ok(ref res) = resultado {
        // Track stats (Estimación simple: 1 token ≈ 4 caracteres)
        let tokens = (res.len() as u64 / 4) + (prompt_len as u64 / 4); // + prompt tokens
        let mut s = stats.lock().unwrap();
        s.total_tokens_used += tokens;

        // Estimación de costo: 0.01$ por cada 1K tokens (promedio)
        s.total_cost_usd += (tokens as f64 / 1000.0) * 0.01;
    }

    resultado
}

// --- CACHE ---

fn obtener_cache_path(prompt: &str, project_path: &Path) -> PathBuf {
    let mut s = DefaultHasher::new();
    prompt.hash(&mut s);
    let hash = s.finish();
    project_path
        .join(".sentinel/cache")
        .join(format!("{:x}.cache", hash))
}

fn intentar_leer_cache(prompt: &str, project_path: &Path) -> Option<String> {
    let path = obtener_cache_path(prompt, project_path);
    fs::read_to_string(path).ok()
}

fn guardar_en_cache(prompt: &str, respuesta: &str, project_path: &Path) -> anyhow::Result<()> {
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
        println!("{}", "   💡 El caché se regenerará automáticamente en las próximas consultas.".dimmed());
    } else {
        println!("{}", "   ℹ️  No hay caché para limpiar.".yellow());
    }

    Ok(())
}

// --- IMPLEMENTACIONES ESPECÍFICAS ---

fn consultar_gemini_interactions(
    client: &Client,
    prompt: String,
    api_key: &str,
    base_url: &str,
    model_name: &str,
) -> anyhow::Result<String> {
    let response = client
        .post(base_url)
        .header("x-goog-api-key", api_key)
        .header("content-type", "application/json")
        .json(&json!({
            "model": model_name,
            "input": prompt
        }))
        .send()?;

    let status = response.status();
    let body_text = response.text()?;

    if !status.is_success() {
        return Err(anyhow::anyhow!(
            "Error de API Gemini Interactions (Status {}): {}",
            status,
            body_text
        ));
    }

    let body: serde_json::Value = serde_json::from_str(&body_text)?;

    body["output"]
        .as_str()
        .or_else(|| {
            body["outputs"].as_array().and_then(|outputs| {
                outputs
                    .iter()
                    .find(|o| o["type"] == "text")
                    .and_then(|o| o["text"].as_str())
            })
        })
        .or_else(|| body["candidates"][0]["content"]["parts"][0]["text"].as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No se pudo encontrar el texto en la respuesta de Gemini Interactions. Body: {}",
                body_text
            )
        })
}

fn consultar_gemini_content(
    client: &Client,
    prompt: String,
    api_key: &str,
    base_url: &str,
    model_name: &str,
) -> anyhow::Result<String> {
    let url = if base_url.contains("generateContent") {
        base_url.to_string()
    } else {
        format!(
            "{}/v1beta/models/{}:generateContent",
            base_url.trim_end_matches('/'),
            model_name
        )
    };

    let response = client
        .post(&url)
        .header("x-goog-api-key", api_key)
        .header("content-type", "application/json")
        .json(&json!({
            "contents": [{
                "parts": [{ "text": prompt }]
            }]
        }))
        .send()?;

    let status = response.status();
    let body_text = response.text()?;

    if !status.is_success() {
        return Err(anyhow::anyhow!(
            "Error de API Gemini (Status {}): {}",
            status,
            body_text
        ));
    }

    let body: serde_json::Value = serde_json::from_str(&body_text)?;
    body["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("Estructura de Gemini inesperada. Body: {}", body_text))
}

fn consultar_anthropic(
    client: &Client,
    prompt: String,
    api_key: &str,
    base_url: &str,
    model_name: &str,
) -> anyhow::Result<String> {
    let url = format!("{}/v1/messages", base_url.trim_end_matches('/'));

    let response = client
        .post(&url)
        .header("x-api-key", api_key)
        .header("content-type", "application/json")
        .json(&json!({
            "model": model_name,
            "max_tokens": 1500,
            "messages": [{"role": "user", "content": prompt}]
        }))
        .send()?;

    let status = response.status();
    let body_text = response.text()?;

    if !status.is_success() {
        return Err(anyhow::anyhow!(
            "Error de API Anthropic (Status {}): {}",
            status,
            body_text
        ));
    }

    let body: serde_json::Value = serde_json::from_str(&body_text)?;
    body["content"][0]["text"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow::anyhow!("Estructura de Anthropic inesperada. Body: {}", body_text))
}

/// Obtiene el listado de modelos disponibles en Gemini.
pub fn listar_modelos_gemini(api_key: &str) -> anyhow::Result<Vec<String>> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models?key={}",
        api_key
    );
    let client = Client::new();
    let response = client.get(&url).send()?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Error al obtener modelos: {}",
            response.status()
        ));
    }

    let body: serde_json::Value = response.json()?;
    let models = body["models"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("No se encontraron modelos en la respuesta"))?
        .iter()
        .filter_map(|m| m["name"].as_str())
        .map(|name| name.replace("models/", ""))
        .filter(|name| name.starts_with("gemini"))
        .collect();

    Ok(models)
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

pub fn analizar_arquitectura(
    codigo: &str,
    file_name: &str,
    stats: Arc<Mutex<SentinelStats>>,
    config: &SentinelConfig,
    project_path: &Path,
    file_path: &Path, // <-- Ruta completa del archivo modificado
) -> anyhow::Result<bool> {
    // Convertimos el Vec<String> de reglas en una lista numerada para el prompt
    let reglas_str = config
        .architecture_rules
        .iter()
        .enumerate()
        .map(|(i, r)| format!("{}. {}", i + 1, r))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        "Actúa como un Arquitecto de Software experto en {}. \n\
        Analiza el archivo '{}' basándote estrictamente en estas reglas:\n\
        {}\n\n\
        REGLAS DE SALIDA: Inicia con 'CRITICO' si hay fallos graves, o 'SEGURO' si está bien.\n\
        Incluye el código mejorado en un bloque ```typescript.\n\n\
        Código:\n{}",
        config.framework, file_name, reglas_str, codigo
    );

    let respuesta = consultar_ia_dinamico(
        prompt,
        TaskType::Deep,
        config,
        Arc::clone(&stats),
        project_path,
    )?;
    let es_critico = respuesta.trim().to_uppercase().starts_with("CRITICO");

    // Actualizamos estadísticas en memoria
    {
        let mut s = stats.lock().unwrap();
        s.total_analisis += 1;
        if es_critico {
            s.bugs_criticos_evitados += 1;
            s.sugerencias_aplicadas += 1;
            s.tiempo_estimado_ahorrado_mins += 20;
        }
        s.guardar(project_path); // Guardamos en disco de inmediato
    }

    // Guardamos sugerencia en el proyecto original (mismo path que el archivo)
    let sugerencia = extraer_codigo(&respuesta);
    let suggested_path = file_path.with_extension(format!(
        "{}.suggested",
        file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("ts")
    ));
    fs::write(&suggested_path, &sugerencia)?;

    let consejo = eliminar_bloques_codigo(&respuesta);
    println!("\n✨ CONSEJO DE CLAUDE:\n{}", consejo);

    Ok(!es_critico)
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

pub fn eliminar_bloques_codigo(texto: &str) -> String {
    let mut resultado = String::new();
    let mut en_bloque = false;

    for linea in texto.lines() {
        if linea.trim().starts_with("```") {
            en_bloque = !en_bloque;
            if !en_bloque {
                resultado.push_str("\n[... Código guardado en .suggested ...]\n");
            }
            continue;
        }
        if !en_bloque {
            resultado.push_str(linea);
            resultado.push('\n');
        }
    }
    resultado.trim().to_string()
}

pub fn extraer_codigo(texto: &str) -> String {
    if let Some(start) = texto.find("```typescript") {
        let resto = &texto[start + 13..];
        if let Some(end) = resto.find("```") {
            return resto[..end].trim().to_string();
        }
    }
    texto.to_string()
}
