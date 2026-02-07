//! Análisis de código con IA
//!
//! Evalúa código fuente contra reglas de arquitectura específicas del framework,
//! principios SOLID, Clean Code y mejores prácticas.

use crate::ai::client::{consultar_ia_dinamico, TaskType};
use crate::ai::utils::{eliminar_bloques_codigo, extraer_codigo};
use crate::config::SentinelConfig;
use crate::stats::SentinelStats;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Analiza código con IA enfocándose en arquitectura y buenas prácticas.
///
/// Evalúa principios SOLID, Clean Code y patrones específicos del framework.
/// Si encuentra problemas críticos, la IA responderá comenzando con "CRITICO",
/// de lo contrario con "SEGURO".
///
/// # Argumentos
///
/// * `codigo` - Código fuente a analizar
/// * `file_name` - Nombre del archivo (para contexto en el prompt)
/// * `stats` - Estadísticas compartidas del proyecto
/// * `config` - Configuración de Sentinel con reglas y framework
/// * `project_path` - Ruta del proyecto monitoreado
/// * `file_path` - Ruta completa del archivo modificado
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
    file_path: &Path,
) -> anyhow::Result<bool> {
    // Convertimos el Vec<String> de reglas en una lista numerada para el prompt
    let reglas_str = config
        .architecture_rules
        .iter()
        .enumerate()
        .map(|(i, r)| format!("{}. {}", i + 1, r))
        .collect::<Vec<_>>()
        .join("\n");

    // Obtener el lenguaje dinámicamente desde la configuración
    // (detectado por IA durante la inicialización)
    let lenguaje_bloque = &config.code_language;

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
