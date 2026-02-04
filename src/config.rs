use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

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
    pub ignore_patterns: Vec<String>,
    pub primary_model: ModelConfig,
    pub fallback_model: Option<ModelConfig>,
    pub use_cache: bool,
}

impl SentinelConfig {
    pub fn default(name: String, manager: String) -> Self {
        let default_model = ModelConfig {
            name: "claude-opus-4-5-20251101".to_string(),
            url: "https://api.anthropic.com".to_string(),
            api_key: "".to_string(),
        };

        Self {
            project_name: name,
            framework: "NestJS".to_string(), // Framework por defecto
            manager: manager.clone(),
            test_command: format!("{} run test", manager),
            architecture_rules: vec![
                "SOLID Principles".to_string(),
                "Clean Code".to_string(),
                "NestJS Best Practices".to_string(),
            ],
            ignore_patterns: vec![
                "node_modules".to_string(),
                "dist".to_string(),
                ".git".to_string(),
            ],
            primary_model: default_model,
            fallback_model: None,
            use_cache: true,
        }
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let toml = toml::to_string_pretty(self)?;
        fs::write(path.join(".sentinelrc.toml"), toml)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Option<Self> {
        let content = fs::read_to_string(path.join(".sentinelrc.toml")).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn debe_ignorar(&self, path: &Path) -> bool {
        let path_str = path.to_str().unwrap_or("");

        // 1. Filtros básicos de extensión
        if !path_str.ends_with(".ts")
            || path_str.contains(".spec.ts")
            || path_str.contains(".suggested")
        {
            return true;
        }

        // 2. Filtros personalizados del config (.sentinelrc)
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

    pub fn abrir_en_editor(path: &Path) {
        let config_path = path.join(".sentinelrc.toml");
        println!("📝 Abriendo configuración en el editor...");

        // Intentamos abrir con 'code' (VS Code), si falla, usamos el comando genérico del sistema
        if Command::new("code").arg(&config_path).status().is_err() {
            #[cfg(target_os = "linux")]
            let _ = Command::new("xdg-open").arg(&config_path).status();
            #[cfg(target_os = "macos")]
            let _ = Command::new("open").arg(&config_path).status();
        }
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
