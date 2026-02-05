use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Resultado de la detección de framework por IA
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrameworkDetection {
    pub framework: String,
    pub rules: Vec<String>,
    pub extensions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelConfig {
    pub name: String,
    pub url: String,
    pub api_key: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            name: "claude-opus-4-5-20251101".to_string(),
            url: "https://api.anthropic.com".to_string(),
            api_key: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SentinelConfig {
    pub project_name: String,
    pub framework: String,
    pub manager: String,
    pub test_command: String,
    pub architecture_rules: Vec<String>,
    pub file_extensions: Vec<String>, // Extensiones de archivo a monitorear
    pub ignore_patterns: Vec<String>,
    pub primary_model: ModelConfig,
    pub fallback_model: Option<ModelConfig>,
    pub use_cache: bool,
}

impl SentinelConfig {
    pub fn default(
        name: String,
        manager: String,
        framework: String,
        rules: Vec<String>,
        extensions: Vec<String>,
    ) -> Self {
        let default_model = ModelConfig {
            name: "claude-opus-4-5-20251101".to_string(),
            url: "https://api.anthropic.com".to_string(),
            api_key: "".to_string(),
        };

        Self {
            project_name: name,
            framework,
            manager: manager.clone(),
            test_command: format!("{} run test", manager),
            architecture_rules: rules,
            file_extensions: extensions,
            ignore_patterns: vec![
                "node_modules".to_string(),
                "dist".to_string(),
                ".git".to_string(),
                "build".to_string(),
                ".next".to_string(),
                "target".to_string(),
                "vendor".to_string(),
                "__pycache__".to_string(),
            ],
            primary_model: default_model,
            fallback_model: None,
            use_cache: true,
        }
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let toml = toml::to_string_pretty(self)?;
        fs::write(path.join(".sentinelrc.toml"), toml)?;

        // Agregar archivos sensibles al .gitignore automáticamente
        Self::actualizar_gitignore(path)?;

        Ok(())
    }

    /// Agrega archivos sensibles de Sentinel al .gitignore para proteger API keys
    pub fn actualizar_gitignore(path: &Path) -> anyhow::Result<()> {
        let gitignore_path = path.join(".gitignore");

        // Entradas que queremos agregar
        let sentinel_entries = vec![
            "# Sentinel - Archivos de configuración y caché (contienen API keys)",
            ".sentinelrc.toml",
            ".sentinel_stats.json",
            ".sentinel/",
        ];

        // Leer .gitignore existente o crear uno nuevo
        let mut content = if gitignore_path.exists() {
            fs::read_to_string(&gitignore_path)?
        } else {
            String::new()
        };

        // Verificar si ya existe la sección de Sentinel
        if !content.contains(".sentinelrc.toml") {
            // Agregar sección de Sentinel
            if !content.is_empty() && !content.ends_with('\n') {
                content.push('\n');
            }
            content.push('\n');
            for entry in sentinel_entries {
                content.push_str(entry);
                content.push('\n');
            }

            fs::write(&gitignore_path, content)?;
            println!(
                "{}",
                "   ✅ Archivos sensibles agregados a .gitignore".green()
            );
        }

        Ok(())
    }

    pub fn load(path: &Path) -> Option<Self> {
        let content = fs::read_to_string(path.join(".sentinelrc.toml")).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn debe_ignorar(&self, path: &Path) -> bool {
        let path_str = path.to_str().unwrap_or("");

        // 1. Ignorar archivos de tests y sugerencias
        if path_str.contains(".spec.")
            || path_str.contains(".test.")
            || path_str.contains("_test.")
            || path_str.contains(".suggested")
        {
            return true;
        }

        // 2. Validar que tenga una extensión permitida
        let tiene_extension_valida = self.file_extensions.iter().any(|ext| {
            path_str.ends_with(&format!(".{}", ext))
        });

        if !tiene_extension_valida {
            return true;
        }

        // 3. Filtros personalizados del config (.sentinelrc)
        self.ignore_patterns
            .iter()
            .any(|pattern| path_str.contains(pattern))
    }

    pub fn detectar_gestor(path: &Path) -> String {
        if path.join("pnpm-lock.yaml").exists() {
            "pnpm".to_string()
        } else if path.join("yarn.lock").exists() {
            "yarn".to_string()
        } else {
            "npm".to_string()
        }
    }

    /// Lista los archivos en la raíz del proyecto (excluyendo node_modules, .git, etc.)
    pub fn listar_archivos_raiz(path: &Path) -> Vec<String> {
        let mut archivos = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    // Ignorar directorios comunes y archivos ocultos
                    if !file_name.starts_with('.')
                        && file_name != "node_modules"
                        && file_name != "dist"
                        && file_name != "build"
                        && file_name != "target"
                        && file_name != "vendor"
                    {
                        archivos.push(file_name);
                    }
                }
            }
        }

        archivos.sort();
        archivos
    }

    pub fn eliminar(path: &Path) -> anyhow::Result<()> {
        let config_path = path.join(".sentinelrc.toml");
        if config_path.exists() {
            fs::remove_file(config_path)?;
            println!("{}", "🗑️  Configuración eliminada correctamente.".yellow());
        }
        Ok(())
    }
}
