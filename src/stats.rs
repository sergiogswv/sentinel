use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
pub struct SentinelStats {
    pub bugs_criticos_evitados: u32,
    pub sugerencias_aplicadas: u32,
    pub tests_fallidos_corregidos: u32,
    pub total_analisis: u32,
    pub tiempo_estimado_ahorrado_mins: u32,
    pub total_cost_usd: f64,
    pub total_tokens_used: u64,
}

impl SentinelStats {
    pub fn cargar(path: &Path) -> Self {
        let stats_path = path.join(".sentinel_stats.json");
        if let Ok(content) = fs::read_to_string(stats_path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn guardar(&self, path: &Path) {
        let stats_path = path.join(".sentinel_stats.json");
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = fs::write(stats_path, content);
        }
    }
}
