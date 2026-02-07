//! Módulo de integración con IA
//!
//! Proporciona funcionalidades para:
//! - Consultas a diferentes proveedores de IA (Anthropic, Gemini)
//! - Detección automática de frameworks
//! - Análisis de arquitectura de código
//! - Detección y validación de frameworks de testing
//! - Sistema de caché para optimizar consultas

pub mod analysis;
pub mod cache;
pub mod client;
pub mod framework;
pub mod models;
pub mod testing;
pub mod utils;

// Re-exports públicos
pub use analysis::analizar_arquitectura;
pub use cache::limpiar_cache;
pub use client::{TaskType, consultar_ia_dinamico};
pub use framework::detectar_framework_con_ia;
pub use models::obtener_modelos_disponibles;
pub use testing::{TestingStatus, detectar_testing_framework, obtener_sugerencias_complementarias};
