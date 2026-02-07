//! Cliente para comunicación con APIs de IA
//!
//! Soporta múltiples proveedores:
//! - Anthropic (Claude)
//! - Google Gemini
//! - OpenAI
//! - Groq
//! - Ollama
//! - Kimi
//! - DeepSeek
//!
//! Incluye sistema de fallback automático entre modelos.

use crate::ai::cache::{guardar_en_cache, intentar_leer_cache};
use crate::config::{AIConfig, AIProvider, SentinelConfig};
use crate::stats::SentinelStats;
use colored::*;
use reqwest::blocking::Client;
use serde_json::json;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
pub enum TaskType {
    Light, // Commits, docs
    Deep,  // Arquitectura, debug tests
}

/// Punto de entrada inteligente con Fallback y Caché
pub fn consultar_ia_dinamico(
    prompt: String,
    _task: TaskType,
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

    // 2. Intentar ejecución con Fallback
    let resultado =
        consultar_ia_con_fallback(prompt.clone(), &config.ai_configs, Arc::clone(&stats));

    // 3. Guardar en Caché si tuvo éxito
    if let Ok(ref res) = resultado {
        if config.use_cache {
            let _ = guardar_en_cache(&prompt, res, project_path);
        }
    }

    resultado
}

pub fn consultar_ia_con_fallback(
    prompt: String,
    configs: &[AIConfig],
    stats: Arc<Mutex<SentinelStats>>,
) -> anyhow::Result<String> {
    if configs.is_empty() {
        return Err(anyhow::anyhow!(
            "No hay configuraciones de IA disponibles. Reinicia Sentinel para configurar una."
        ));
    }

    let mut last_error = anyhow::anyhow!("Error desconocido");

    for (i, config) in configs.iter().enumerate() {
        if i > 0 {
            println!(
                "\n⚠️  El modelo '{}' falló. Intentando con el siguiente configurado: '{}'...",
                configs[i - 1].name,
                config.name
            );
        }

        match consultar_ia(prompt.clone(), config.clone(), Arc::clone(&stats)) {
            Ok(res) => {
                if i > 0 {
                    println!(
                        "   ✅ El modelo '{}' respondió correctamente.\n",
                        config.name
                    );
                }
                return Ok(res);
            }
            Err(e) => {
                println!("   ❌ Error en '{}': {}", config.name, e);
                last_error = e;
            }
        }
    }

    Err(anyhow::anyhow!(
        "❌ Todos los modelos configurados fallaron. Último error: {}",
        last_error
    ))
}

pub fn consultar_ia(
    prompt: String,
    config: AIConfig,
    stats: Arc<Mutex<SentinelStats>>,
) -> anyhow::Result<String> {
    let client = Client::new();
    let prompt_len = prompt.len();

    let resultado = match config.provider {
        AIProvider::Claude => consultar_claude(&client, prompt, &config),
        AIProvider::Gemini => consultar_gemini(&client, prompt, &config),
        AIProvider::OpenAI
        | AIProvider::Groq
        | AIProvider::Ollama
        | AIProvider::Kimi
        | AIProvider::DeepSeek => consultar_openai_compatible(&client, prompt, &config),
    };

    if let Ok(ref res) = resultado {
        // Track stats (Estimación simple: 1 token ≈ 4 caracteres)
        let tokens = (res.len() as u64 / 4) + (prompt_len as u64 / 4);
        let mut s = stats.lock().unwrap();
        s.total_tokens_used += tokens;

        // Estimación de costo: 0.01$ por cada 1K tokens (promedio)
        s.total_cost_usd += (tokens as f64 / 1000.0) * 0.01;
    }

    resultado
}

fn consultar_claude(client: &Client, prompt: String, config: &AIConfig) -> anyhow::Result<String> {
    let url = format!("{}/v1/messages", config.api_url.trim_end_matches('/'));
    let response = client
        .post(&url)
        .header("x-api-key", &config.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&json!({
            "model": config.model,
            "max_tokens": 1500,
            "messages": [{"role": "user", "content": prompt}]
        }))
        .send()?;

    procesar_respuesta_json(response, "Claude", |json| {
        json["content"][0]["text"].as_str().map(|s| s.to_string())
    })
}

fn consultar_gemini(client: &Client, prompt: String, config: &AIConfig) -> anyhow::Result<String> {
    let url = format!(
        "{}/v1beta/models/{}:generateContent?key={}",
        config.api_url.trim_end_matches('/'),
        config.model,
        config.api_key
    );

    let response = client
        .post(&url)
        .header("content-type", "application/json")
        .json(&json!({
            "contents": [{
                "parts": [{ "text": prompt }]
            }]
        }))
        .send()?;

    procesar_respuesta_json(response, "Gemini", |json| {
        json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
    })
}

fn consultar_openai_compatible(
    client: &Client,
    prompt: String,
    config: &AIConfig,
) -> anyhow::Result<String> {
    let url = format!("{}/chat/completions", config.api_url.trim_end_matches('/'));

    let mut request = client.post(&url).header("content-type", "application/json");

    if !config.api_key.is_empty() {
        request = request.header("authorization", format!("Bearer {}", config.api_key));
    }

    let response = request
        .json(&json!({
            "model": config.model,
            "messages": [
                {"role": "system", "content": "Eres un Arquitecto de Software Senior."},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.1
        }))
        .send()?;

    procesar_respuesta_json(response, "API Compatible", |json| {
        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
    })
}

fn procesar_respuesta_json<F>(
    response: reqwest::blocking::Response,
    provider_name: &str,
    extractor: F,
) -> anyhow::Result<String>
where
    F: FnOnce(serde_json::Value) -> Option<String>,
{
    let status = response.status();
    let body_text = response.text()?;

    if !status.is_success() {
        return Err(anyhow::anyhow!(
            "Error de API {} (Status {}): {}",
            provider_name,
            status,
            body_text
        ));
    }

    let json: serde_json::Value = serde_json::from_str(&body_text)?;
    extractor(json).ok_or_else(|| {
        anyhow::anyhow!(
            "Estructura de respuesta de {} inesperada. Body: {}",
            provider_name,
            body_text
        )
    })
}
