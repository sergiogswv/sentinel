//! Módulo de integración con IA
//!
//! Proporciona funcionalidades para:
//! - Consultas a diferentes proveedores de IA (Anthropic, Gemini)
//! - Detección automática de frameworks
//! - Análisis de arquitectura de código
//! - Sistema de caché para optimizar consultas

pub mod analysis;
pub mod cache;
pub mod client;
pub mod framework;
pub mod utils;

// Re-exports públicos
pub use analysis::analizar_arquitectura;
pub use cache::limpiar_cache;
pub use client::{consultar_ia_dinamico, TaskType};
pub use framework::{detectar_framework_con_ia, listar_modelos_gemini};
