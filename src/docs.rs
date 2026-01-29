//! Módulo de documentación
//!
//! Funciones para generar documentación automática de archivos modificados.

use std::path::Path;
use std::fs;
use colored::*;
use crate::ai;

/// Genera un "manual de bolsillo" automático para cada archivo modificado.
///
/// Crea documentación técnica ultra-concisa (máximo 150 palabras) en formato Markdown
/// usando Claude AI. El archivo .md se genera en el mismo directorio que el archivo .ts
/// con el mismo nombre pero extensión .md.
///
/// # Argumentos
///
/// * `codigo` - Código fuente del archivo modificado
/// * `file_path` - Ruta completa al archivo .ts modificado
///
/// # Retorna
///
/// * `Ok(())` - Documentación generada exitosamente
/// * `Err` - Error al comunicarse con la IA o al escribir el archivo
///
/// # Efectos secundarios
///
/// Crea/sobrescribe un archivo .md en la misma ubicación que el .ts original.
/// Por ejemplo: `src/users/users.service.ts` → `src/users/users.service.md`
///
/// # Formato de salida
///
/// ```markdown
/// # 📖 Documentación: users.service.ts
///
/// > ✨ Actualizado automáticamente por Sentinel v3.1
///
/// 🎯 **Funcionalidad**: Gestiona operaciones CRUD de usuarios...
/// 🔧 **Métodos principales**: findAll(), create(), update()...
///
/// ---
/// *Último refactor: SystemTime { ... }*
/// ```
pub fn actualizar_documentacion(codigo: &str, file_path: &Path) -> anyhow::Result<()> {
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    println!("📚 Actualizando manual de bolsillo para: {}", file_name.magenta());

    let prompt = format!(
        "Como documentador técnico de NestJS, analiza este código: {}. \
        Genera un resumen técnico ultra-conciso (máximo 150 palabras) en Markdown. \
        Enfócate en: ¿Qué hace este servicio? y ¿Cuáles son sus métodos principales? \
        Usa emojis para las secciones. No uses introducciones innecesarias.\n\n{}",
        file_name, codigo
    );

    let resumen = ai::consultar_claude(prompt)?;

    // Cambiamos la extensión de .ts a .md en la misma carpeta
    let mut docs_path = file_path.to_path_buf();
    docs_path.set_extension("md");

    let nueva_doc = format!(
        "# 📖 Documentación: {}\n\n> ✨ Actualizado automáticamente por Sentinel v3.2\n\n{}\n\n---\n*Último refactor: {:?}*",
        file_name,
        resumen,
        std::time::SystemTime::now()
    );

    fs::write(&docs_path, nueva_doc)?;
    println!("   ✅ Documento generado: {}", docs_path.display());
    Ok(())
}
