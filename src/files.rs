//! # Utilidades para detección de archivos padres
//!
//! Este módulo proporciona funciones para detectar cuando un archivo modificado
//! es un "hijo" de un módulo padre más grande, permitiendo ejecutar los tests
//! del módulo completo en lugar de buscar tests para el archivo individual.

use std::fs;
use std::path::Path;

/// Tipos de archivos que son considerados "padres" en una estructura NestJS
const PARENT_PATTERNS: &[&str] = &[
    ".service.ts",
    ".controller.ts",
    ".repository.ts",
    ".module.ts",
    ".gateway.ts",
    ".guard.ts",
    ".interceptor.ts",
    ".pipe.ts",
    ".filter.ts",
];

/// Prioridad de los tipos de archivos padres (menor índice = mayor prioridad)
/// El orden es: service > controller > repository > gateway > module > otros
const PARENT_PRIORITY: &[&str] = &[
    ".service.ts",
    ".controller.ts",
    ".repository.ts",
    ".gateway.ts",
    ".module.ts",
    ".guard.ts",
    ".interceptor.ts",
    ".pipe.ts",
    ".filter.ts",
];

/// Verifica si un archivo es de tipo "padre" (.service.ts, .controller.ts, etc.)
///
/// # Argumentos
/// * `file_name` - Nombre del archivo a verificar
///
/// # Retorna
/// * `true` si el archivo coincide con algún patrón de padre
/// * `false` en caso contrario
///
/// # Ejemplos
/// ```
/// assert!(es_archivo_padre("user.service.ts"));
/// assert!(es_archivo_padre("auth.controller.ts"));
/// assert!(!es_archivo_padre("user.dto.ts"));
/// assert!(!es_archivo_padre("helper.ts"));
/// ```
pub fn es_archivo_padre(file_name: &str) -> bool {
    PARENT_PATTERNS
        .iter()
        .any(|pattern| file_name.ends_with(pattern))
}

/// Detecta si un archivo es un "hijo" y retorna el nombre del módulo padre
///
/// Esta función busca en el mismo directorio del archivo modificado si existe
/// un archivo padre (.service.ts, .controller.ts, etc.) y retorna el nombre
/// base del módulo. Si hay múltiples padres, usa el de mayor prioridad.
///
/// # Argumentos
/// * `changed_path` - Path del archivo modificado
/// * `project_path` - Path raíz del proyecto (no usado directamente, pero útil para validaciones futuras)
///
/// # Retorna
/// * `Some(nombre_base)` - Si se detecta un padre, retorna el nombre base (ej: "call" para "call.service.ts")
/// * `None` - Si no se detecta ningún padre
///
/// # Ejemplos
/// ```
/// // Archivo: src/calls/call-inbound.ts
/// // Existe: src/calls/call.service.ts
/// // Retorna: Some("call")
///
/// // Archivo: src/users/users.service.ts
/// // No existe ningún padre (es el padre)
/// // Retorna: None
/// ```
pub fn detectar_archivo_padre(changed_path: &Path, _project_path: &Path) -> Option<String> {
    // Obtener el directorio del archivo modificado
    let dir = changed_path.parent()?;

    // Leer todos los archivos en el directorio
    let entries = fs::read_dir(dir).ok()?;

    // Recopilar todos los archivos padres encontrados
    let mut padres: Vec<(String, usize)> = Vec::new();

    for entry in entries {
        let entry = entry.ok()?;
        let path = entry.path();

        // Solo procesar archivos, no directorios
        if !path.is_file() {
            continue;
        }

        // Solo procesar archivos .ts que no sean .spec.ts
        let file_name = path.file_name()?.to_str()?;

        if !file_name.ends_with(".ts") || file_name.contains(".spec.") {
            continue;
        }

        // Verificar si es un archivo padre
        if es_archivo_padre(file_name) {
            // Encontrar la prioridad de este tipo de archivo
            let priority = PARENT_PRIORITY
                .iter()
                .position(|pattern| file_name.ends_with(pattern))
                .unwrap_or(PARENT_PRIORITY.len());

            // Extraer el nombre base (ej: "call.service.ts" -> "call")
            let base_name = file_name
                .split('.')
                .next()
                .unwrap_or("")
                .to_string();

            if !base_name.is_empty() {
                padres.push((base_name, priority));
            }
        }
    }

    // Retornar el padre con mayor prioridad (menor índice)
    if padres.is_empty() {
        None
    } else {
        // Ordenar por prioridad y tomar el primero
        padres.sort_by_key(|(_, priority)| *priority);
        Some(padres[0].0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_archivo_padre_servicio() {
        assert!(es_archivo_padre("user.service.ts"));
        assert!(es_archivo_padre("auth.service.ts"));
    }

    #[test]
    fn test_es_archivo_padre_controlador() {
        assert!(es_archivo_padre("user.controller.ts"));
        assert!(es_archivo_padre("auth.controller.ts"));
    }

    #[test]
    fn test_es_archivo_padre_repositorio() {
        assert!(es_archivo_padre("user.repository.ts"));
    }

    #[test]
    fn test_es_archivo_padre_modulo() {
        assert!(es_archivo_padre("user.module.ts"));
    }

    #[test]
    fn test_es_archivo_padre_gateway() {
        assert!(es_archivo_padre("events.gateway.ts"));
    }

    #[test]
    fn test_es_archivo_padre_otros() {
        assert!(es_archivo_padre("auth.guard.ts"));
        assert!(es_archivo_padre("logging.interceptor.ts"));
        assert!(es_archivo_padre("validation.pipe.ts"));
        assert!(es_archivo_padre("http.filter.ts"));
    }

    #[test]
    fn test_no_es_archivo_padre() {
        assert!(!es_archivo_padre("user.dto.ts"));
        assert!(!es_archivo_padre("call-inbound.ts"));
        assert!(!es_archivo_padre("helper.ts"));
        assert!(!es_archivo_padre("constants.ts"));
        assert!(!es_archivo_padre("user.spec.ts"));
        assert!(!es_archivo_padre("user.service.spec.ts"));
    }

    #[test]
    fn test_archivo_con_puntos() {
        // Archivos con puntos en el nombre no deben ser padres
        // a menos que terminen con el patrón correcto
        assert!(!es_archivo_padre("user-v2.dto.ts"));
        assert!(es_archivo_padre("user-v2.service.ts"));
    }
}
