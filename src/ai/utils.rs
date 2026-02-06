//! Utilidades para procesamiento de respuestas de IA
//!
//! Funciones auxiliares para extraer y procesar bloques de código
//! desde respuestas de IA formateadas en markdown.

/// Elimina bloques de código de una respuesta de IA.
///
/// Busca y elimina el contenido entre delimitadores \`\`\`...\`\`\`,
/// dejando solo el texto explicativo. Útil para mostrar consejos
/// al usuario sin el código sugerido.
///
/// # Argumentos
///
/// * `texto` - Respuesta completa de IA con posibles bloques de código
///
/// # Retorna
///
/// Texto sin bloques de código, con marcadores indicando dónde estaban.
///
/// # Ejemplo
///
/// ```
/// let respuesta = "Aquí hay un problema:\n```rust\nfn main() {}\n```\nCorrige esto.";
/// let sin_codigo = eliminar_bloques_codigo(respuesta);
/// // Retorna: "Aquí hay un problema:\n[... Código guardado en .suggested ...]\nCorrige esto."
/// ```
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

/// Extrae bloques de código de una respuesta de IA.
///
/// Busca y extrae el contenido entre delimitadores de markdown \`\`\`lenguaje...\`\`\`.
/// Soporta múltiples lenguajes comunes. Si no encuentra un bloque delimitado,
/// devuelve el texto completo.
///
/// # Argumentos
///
/// * `texto` - Respuesta completa de IA
///
/// # Retorna
///
/// Código extraído (sin delimitadores) o el texto original si no hay bloques.
///
/// # Lenguajes soportados
///
/// typescript, javascript, python, php, go, rust, java, jsx, tsx, code
///
/// # Ejemplo
///
/// ```
/// let respuesta = "Aquí está el código:\n```rust\nfn main() {\n    println!(\"Hola\");\n}\n```";
/// let codigo = extraer_codigo(respuesta);
/// // Retorna: "fn main() {\n    println!(\"Hola\");\n}"
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraer_codigo_typescript() {
        let texto = "Aquí está:\n```typescript\nconst x = 1;\n```\nEso es todo.";
        assert_eq!(extraer_codigo(texto), "const x = 1;");
    }

    #[test]
    fn test_extraer_codigo_sin_lenguaje() {
        let texto = "Código:\n```\nconst x = 1;\n```";
        assert_eq!(extraer_codigo(texto), "const x = 1;");
    }

    #[test]
    fn test_extraer_codigo_sin_bloque() {
        let texto = "Solo texto sin código";
        assert_eq!(extraer_codigo(texto), "Solo texto sin código");
    }

    #[test]
    fn test_eliminar_bloques_codigo() {
        let texto = "Problema:\n```rust\nfn test() {}\n```\nSolución.";
        let resultado = eliminar_bloques_codigo(texto);
        assert!(resultado.contains("Problema:"));
        assert!(resultado.contains("Solución."));
        assert!(resultado.contains("[... Código guardado en .suggested ...]"));
        assert!(!resultado.contains("fn test()"));
    }

    #[test]
    fn test_eliminar_multiples_bloques() {
        let texto = "Antes\n```\ncodigo1\n```\nMedio\n```\ncodigo2\n```\nDespués";
        let resultado = eliminar_bloques_codigo(texto);
        assert_eq!(
            resultado.matches("[... Código guardado en .suggested ...]").count(),
            2
        );
    }
}
