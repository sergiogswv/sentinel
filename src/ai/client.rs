//! Cliente para comunicación con APIs de IA
//!
//! Soporta múltiples proveedores:
//! - Anthropic (Claude)
//! - Google Gemini (Content API e Interactions API)
//!
//! Incluye sistema de fallback automático entre modelos.

use crate::ai::cache::{guardar_en_cache, intentar_leer_cache};
use crate::config::{ModelConfig, SentinelConfig};
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
        let tokens = (res.len() as u64 / 4) + (prompt_len as u64 / 4);
        let mut s = stats.lock().unwrap();
        s.total_tokens_used += tokens;

        // Estimación de costo: 0.01$ por cada 1K tokens (promedio)
        s.total_cost_usd += (tokens as f64 / 1000.0) * 0.01;
    }

    resultado
}

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
