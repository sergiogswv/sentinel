use crate::config::{FrameworkDetection, ModelConfig, SentinelConfig};
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

/// Detecta el framework y sus reglas usando IA analizando los archivos del proyecto
pub fn detectar_framework_con_ia(
    project_path: &Path,
    config: &SentinelConfig,
) -> anyhow::Result<FrameworkDetection> {
    println!("{}", "🤖 Detectando framework con IA...".magenta());

    let archivos = SentinelConfig::listar_archivos_raiz(project_path);
    let archivos_str = archivos.join("\n");

    let prompt_inicial = format!(
        "Eres un experto en detectar frameworks y tecnologías de desarrollo.\n\n\
        ARCHIVOS EN LA RAÍZ DEL PROYECTO:\n{}\n\n\
        INSTRUCCIONES:\n\
        1. Analiza los archivos listados para identificar el framework/tecnología principal\n\
        2. Si necesitas ver el contenido de UN archivo específico para confirmar, responde SOLO: LEER:nombre_archivo\n\
        3. Si ya puedes determinar el framework, responde INMEDIATAMENTE en formato JSON:\n\
        {{\n  \
          \"framework\": \"nombre del framework\",\n  \
          \"rules\": [\"regla específica 1\", \"regla específica 2\", \"regla específica 3\", \"regla específica 4\"],\n  \
          \"extensions\": [\"ext1\", \"ext2\", \"ext3\"]\n\
        }}\n\n\
        - framework: Nombre del framework/tecnología principal\n\
        - rules: Principios de arquitectura y buenas prácticas específicas\n\
        - extensions: Extensiones de archivo principales a monitorear (sin el punto, ej: \"ts\", \"js\", \"py\", \"php\", \"go\")\n\n\
        IMPORTANTE: Responde SOLO con JSON o LEER:archivo, nada más.",
        archivos_str
    );

    // Primera consulta
    let respuesta = consultar_ia(
        prompt_inicial,
        &config.primary_model.api_key,
        &config.primary_model.url,
        &config.primary_model.name,
        Arc::new(Mutex::new(SentinelStats::default())),
    )?;

    // Si la IA pide leer un archivo
    if respuesta.trim().starts_with("LEER:") {
        let archivo = respuesta.trim().replace("LEER:", "").trim().to_string();
        let archivo_path = project_path.join(&archivo);

        println!("   📄 IA solicita leer: {}", archivo.cyan());

        if let Ok(contenido) = fs::read_to_string(&archivo_path) {
            // Limitar contenido a primeras 100 líneas para no saturar
            let contenido_limitado: String = contenido
                .lines()
                .take(100)
                .collect::<Vec<_>>()
                .join("\n");

            let prompt_con_contenido = format!(
                "ARCHIVOS EN LA RAÍZ:\n{}\n\n\
                CONTENIDO DE '{}':\n{}\n\n\
                Ahora determina el framework y responde en formato JSON:\n\
                {{\n  \
                  \"framework\": \"nombre del framework\",\n  \
                  \"rules\": [\"regla específica 1\", \"regla específica 2\", \"regla específica 3\", \"regla específica 4\"],\n  \
                  \"extensions\": [\"ext1\", \"ext2\", \"ext3\"]\n\
                }}\n\n\
                - framework: Nombre del framework/tecnología principal\n\
                - rules: Principios de arquitectura y buenas prácticas específicas\n\
                - extensions: Extensiones de archivo principales a monitorear (sin el punto)\n\n\
                IMPORTANTE: Responde SOLO con JSON válido, nada más.",
                archivos_str, archivo, contenido_limitado
            );

            let respuesta_final = consultar_ia(
                prompt_con_contenido,
                &config.primary_model.api_key,
                &config.primary_model.url,
                &config.primary_model.name,
                Arc::new(Mutex::new(SentinelStats::default())),
            )?;

            return parsear_deteccion_framework(&respuesta_final);
        }
    }

    // Parsear respuesta JSON
    parsear_deteccion_framework(&respuesta)
}

/// Parsea la respuesta JSON de la IA con la detección del framework
fn parsear_deteccion_framework(respuesta: &str) -> anyhow::Result<FrameworkDetection> {
    // Extraer JSON si está envuelto en texto
    let json_str = if let Some(inicio) = respuesta.find('{') {
        if let Some(fin) = respuesta.rfind('}') {
            &respuesta[inicio..=fin]
        } else {
            respuesta
        }
    } else {
        respuesta
    };

    match serde_json::from_str::<FrameworkDetection>(json_str) {
        Ok(deteccion) => {
            println!("   ✅ Framework detectado: {}", deteccion.framework.green());
            Ok(deteccion)
        }
        Err(e) => {
            // Fallback si falla el parsing
            println!(
                "   ⚠️  Error al parsear respuesta de IA: {}",
                e.to_string().yellow()
            );
            Ok(FrameworkDetection {
                framework: "JavaScript/TypeScript".to_string(),
                rules: vec![
                    "Clean Code".to_string(),
                    "SOLID Principles".to_string(),
                    "Best Practices".to_string(),
                    "Code Maintainability".to_string(),
                ],
                extensions: vec!["js".to_string(), "ts".to_string()],
            })
        }
    }
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

    // Determinar el lenguaje para el bloque de código según las extensiones
    let lenguaje_bloque = if config.file_extensions.contains(&"ts".to_string())
        || config.file_extensions.contains(&"tsx".to_string())
    {
        "typescript"
    } else if config.file_extensions.contains(&"js".to_string())
        || config.file_extensions.contains(&"jsx".to_string())
    {
        "javascript"
    } else if config.file_extensions.contains(&"py".to_string()) {
        "python"
    } else if config.file_extensions.contains(&"php".to_string()) {
        "php"
    } else if config.file_extensions.contains(&"go".to_string()) {
        "go"
    } else if config.file_extensions.contains(&"rs".to_string()) {
        "rust"
    } else if config.file_extensions.contains(&"java".to_string()) {
        "java"
    } else {
        "code"
    };

    let prompt = format!(
        "Actúa como un Arquitecto de Software experto en {}.\n\n\
        CONTEXTO DEL PROYECTO:\n\
        - Framework/Tecnología: {}\n\
        - Archivo a analizar: {}\n\n\
        REGLAS DE ARQUITECTURA ESPECÍFICAS:\n\
        {}\n\n\
        ANÁLISIS REQUERIDO:\n\
        Analiza el código siguiente basándote ESTRICTAMENTE en las reglas de arquitectura listadas arriba.\n\
        Considera las mejores prácticas específicas de {}.\n\n\
        FORMATO DE RESPUESTA:\n\
        1. Inicia con 'CRITICO' si hay fallos graves de arquitectura/seguridad, o 'SEGURO' si está bien\n\
        2. Explica brevemente los problemas encontrados o aspectos positivos\n\
        3. Incluye el código mejorado en un bloque ```{}\n\n\
        CÓDIGO A ANALIZAR:\n{}",
        config.framework,
        config.framework,
        file_name,
        reglas_str,
        config.framework,
        lenguaje_bloque,
        codigo
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
    // Lista de posibles etiquetas de lenguaje
    let lenguajes = [
        "typescript",
        "javascript",
        "python",
        "php",
        "go",
        "rust",
        "java",
        "jsx",
        "tsx",
        "code",
    ];

    for lenguaje in &lenguajes {
        let tag = format!("```{}", lenguaje);
        if let Some(start) = texto.find(&tag) {
            let resto = &texto[start + tag.len()..];
            if let Some(end) = resto.find("```") {
                return resto[..end].trim().to_string();
            }
        }
    }

    // Si no encuentra ningún bloque de código específico, buscar cualquier ```
    if let Some(start) = texto.find("```") {
        let resto = &texto[start + 3..];
        if let Some(end) = resto.find("```") {
            return resto[..end].trim().to_string();
        }
    }

    texto.to_string()
}
