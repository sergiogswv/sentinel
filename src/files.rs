//! # Utilidades para detección de archivos padres
//!
//! Este módulo proporciona funciones para detectar cuando un archivo modificado
//! es un "hijo" de un módulo padre más grande, permitiendo ejecutar los tests
//! del módulo completo en lugar de buscar tests para el archivo individual.
//!
//! Los patrones de archivos padre son ahora dinámicos y detectados por IA según
//! el framework del proyecto (NestJS, Django, Laravel, etc.)

use std::fs;
use std::path::Path;

/// Verifica si un archivo es de tipo "padre" según los patrones del framework
///
/// # Argumentos
/// * `file_name` - Nombre del archivo a verificar
/// * `parent_patterns` - Patrones de archivos padre del framework (detectados por IA)
///
/// # Retorna
/// * `true` si el archivo coincide con algún patrón de padre
/// * `false` en caso contrario
///
/// # Ejemplos
/// ```
/// let nestjs_patterns = vec![".service.ts".to_string(), ".controller.ts".to_string()];
/// assert!(es_archivo_padre("user.service.ts", &nestjs_patterns));
/// assert!(es_archivo_padre("auth.controller.ts", &nestjs_patterns));
/// assert!(!es_archivo_padre("user.dto.ts", &nestjs_patterns));
/// ```
pub fn es_archivo_padre(file_name: &str, parent_patterns: &[String]) -> bool {
    parent_patterns
        .iter()
        .any(|pattern| file_name.ends_with(pattern))
}

/// Detecta si un archivo es un "hijo" y retorna el nombre del módulo padre
///
/// Esta función busca en el mismo directorio del archivo modificado si existe
/// un archivo padre según los patrones del framework y retorna el nombre
/// base del módulo. Si hay múltiples padres, usa el de mayor prioridad (primero en la lista).
///
/// # Argumentos
/// * `changed_path` - Path del archivo modificado
/// * `project_path` - Path raíz del proyecto (no usado directamente, pero útil para validaciones futuras)
/// * `parent_patterns` - Patrones de archivos padre del framework (detectados por IA)
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
pub fn detectar_archivo_padre(
    changed_path: &Path,
    _project_path: &Path,
    parent_patterns: &[String],
) -> Option<String> {
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
        if es_archivo_padre(file_name, parent_patterns) {
            // Encontrar la prioridad de este tipo de archivo (según el orden en parent_patterns)
            let priority = parent_patterns
                .iter()
                .position(|pattern| file_name.ends_with(pattern))
                .unwrap_or(parent_patterns.len());

            // Extraer el nombre base (ej: "call.service.ts" -> "call")
            let base_name = file_name.split('.').next().unwrap_or("").to_string();

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

/// Busca archivos de test para un módulo usando los patrones del framework
///
/// Esta función intenta encontrar archivos de test siguiendo los patrones específicos
/// del framework detectado por IA. Reemplaza {name} con el nombre del módulo (lowercase)
/// y {Name} con el nombre capitalizado.
///
/// # Argumentos
/// * `base_name` - Nombre base del módulo (ej: "user", "call")
/// * `project_path` - Path raíz del proyecto
/// * `test_patterns` - Patrones de ubicación de tests del framework
///
/// # Retorna
/// * `Some(PathBuf)` - Path relativo al archivo de test si se encuentra
/// * `None` - Si no se encuentra ningún archivo de test
///
/// # Ejemplos
/// ```
/// let patterns = vec!["test/{name}/{name}.spec.ts".to_string()];
/// // Para base_name = "user"
/// // Buscará: test/user/user.spec.ts
/// ```
pub fn buscar_archivo_test(
    base_name: &str,
    project_path: &Path,
    test_patterns: &[String],
) -> Option<String> {
    // Capitalizar primera letra para {Name}
    let capitalized = {
        let mut chars = base_name.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    };

    // Intentar cada patrón
    for pattern in test_patterns {
        let path_str = pattern
            .replace("{name}", base_name)
            .replace("{Name}", &capitalized);

        let test_path = project_path.join(&path_str);

        if test_path.exists() {
            return Some(path_str);
        }
    }

    None
}

#[cfg(test)]
mod test_buscar {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_buscar_archivo_test_nestjs() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        // Crear estructura de directorios para NestJS
        let test_dir = project_path.join("test").join("user");
        fs::create_dir_all(&test_dir).unwrap();
        fs::write(test_dir.join("user.spec.ts"), "// test content").unwrap();

        let patterns = vec!["test/{name}/{name}.spec.ts".to_string()];
        let result = buscar_archivo_test("user", project_path, &patterns);

        assert_eq!(result, Some("test/user/user.spec.ts".to_string()));
    }

    #[test]
    fn test_buscar_archivo_test_django() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        // Crear estructura para Django
        let tests_dir = project_path.join("tests");
        fs::create_dir_all(&tests_dir).unwrap();
        fs::write(tests_dir.join("test_user.py"), "# test content").unwrap();

        let patterns = vec![
            "tests/test_{name}.py".to_string(),
            "{name}/tests.py".to_string(),
        ];
        let result = buscar_archivo_test("user", project_path, &patterns);

        assert_eq!(result, Some("tests/test_user.py".to_string()));
    }

    #[test]
    fn test_buscar_archivo_test_laravel_capitalizado() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        // Crear estructura para Laravel (con capitalización)
        let tests_dir = project_path.join("tests").join("Unit");
        fs::create_dir_all(&tests_dir).unwrap();
        fs::write(tests_dir.join("UserTest.php"), "<?php").unwrap();

        let patterns = vec!["tests/Unit/{Name}Test.php".to_string()];
        let result = buscar_archivo_test("user", project_path, &patterns);

        assert_eq!(result, Some("tests/Unit/UserTest.php".to_string()));
    }

    #[test]
    fn test_buscar_archivo_test_no_encontrado() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        let patterns = vec!["test/{name}/{name}.spec.ts".to_string()];
        let result = buscar_archivo_test("user", project_path, &patterns);

        assert_eq!(result, None);
    }

    #[test]
    fn test_buscar_archivo_test_multiples_patrones() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        // Crear solo el segundo patrón
        let src_dir = project_path.join("src").join("user");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("user.spec.ts"), "// test").unwrap();

        let patterns = vec![
            "test/{name}/{name}.spec.ts".to_string(),
            "src/{name}/{name}.spec.ts".to_string(),
        ];
        let result = buscar_archivo_test("user", project_path, &patterns);

        // Debe encontrar el segundo patrón
        assert_eq!(result, Some("src/user/user.spec.ts".to_string()));
    }

    #[test]
    fn test_buscar_archivo_test_go() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        // En Go, los tests están al lado del código
        fs::write(project_path.join("user_test.go"), "package main").unwrap();

        let patterns = vec!["{name}_test.go".to_string()];
        let result = buscar_archivo_test("user", project_path, &patterns);

        assert_eq!(result, Some("user_test.go".to_string()));
    }
}
