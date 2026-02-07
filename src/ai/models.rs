use crate::config::AIProvider;
use reqwest::blocking::Client;

/// Obtiene la lista de modelos disponibles para el proveedor configurado
pub fn obtener_modelos_disponibles(
    provider: &AIProvider,
    api_url: &str,
    api_key: &str,
) -> anyhow::Result<Vec<String>> {
    let client = Client::new();
    let url = api_url.trim_end_matches('/');

    match provider {
        AIProvider::Claude => {
            let response = client
                .get(&format!("{}/v1/models", url))
                .header("x-api-key", api_key)
                .header("anthropic-version", "2023-06-01")
                .send()?;

            let json: serde_json::Value = response.json()?;
            let models = json["data"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Respuesta de Claude inválida"))?
                .iter()
                .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
                .collect();
            Ok(models)
        }
        AIProvider::Gemini => {
            let response = client
                .get(&format!("{}/v1beta/models?key={}", url, api_key))
                .send()?;

            let json: serde_json::Value = response.json()?;
            let models = json["models"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Respuesta de Gemini inválida"))?
                .iter()
                .filter_map(|m| {
                    m["name"]
                        .as_str()
                        .map(|s| s.trim_start_matches("models/").to_string())
                })
                .collect();
            Ok(models)
        }
        AIProvider::OpenAI
        | AIProvider::Groq
        | AIProvider::Ollama
        | AIProvider::Kimi
        | AIProvider::DeepSeek => {
            let mut request = client.get(&format!("{}/models", url));
            if !api_key.is_empty() {
                request = request.header("authorization", format!("Bearer {}", api_key));
            }

            let response = request.send()?;
            let json: serde_json::Value = response.json()?;
            let models = json["data"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Respuesta de API compatible inválida"))?
                .iter()
                .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
                .collect();
            Ok(models)
        }
    }
}
