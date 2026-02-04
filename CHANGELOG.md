# Changelog

All notable changes to Sentinel will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [4.0.0] - 2025-02-03

### 🚨 Breaking Changes

- **Configuración renovada**: Las variables de entorno `ANTHROPIC_AUTH_TOKEN` y `ANTHROPIC_BASE_URL` han sido reemplazadas por un archivo de configuración `.sentinelrc.toml` más flexible y portable
- **Arquitectura multi-proveedor**: El sistema ahora soporta múltiples proveedores de IA, no solo Anthropic Claude

### ✨ Added

- **Soporte multi-proveedor de IA**:
  - Anthropic Claude (Opus, Sonnet, Haiku)
  - Google Gemini (2.0 Flash, 1.5 Pro, etc.)
  - Estructura extensible para agregar más proveedores
- **Sistema de fallback automático**: Configura un modelo de respaldo que se activa si el principal falla
- **Caché inteligente de respuestas**: Reduce costos de API hasta 70% evitando consultas repetidas
- **Dashboard de métricas en tiempo real** (comando `m`):
  - Bugs críticos evitados
  - Costo acumulado de APIs
  - Tokens consumidos
  - Tiempo estimado ahorrado
- **Nuevos comandos interactivos**:
  - `m` - Ver dashboard de métricas
  - `c` - Abrir configuración en el editor
  - `x` - Reiniciar configuración
- **Asistente de configuración interactivo**: Guía paso a paso en el primer uso
- **Listado automático de modelos**: Para Gemini, muestra modelos disponibles durante configuración
- **Tracking de costos y tokens**: Estadísticas persistentes en `.sentinel_stats.json`

### 🔧 Changed

- Archivos `.suggested` ahora se guardan en el mismo directorio que el archivo original (antes se guardaban en el directorio de Sentinel)
- Documentación completamente renovada con guías de proveedores de IA
- Mejores mensajes de error y validación de configuración

### 📁 New Files

- `.sentinelrc.toml` - Archivo de configuración del proyecto
- `.sentinel_stats.json` - Métricas persistentes de productividad
- `.sentinel/cache/` - Directorio de caché de respuestas de IA

### 🔄 Migration Guide

Para migrar desde v3.x:

1. Actualiza el código a v4.0.0
2. Ejecuta Sentinel - se iniciará el asistente de configuración
3. Ingresa tu API Key cuando se te solicite
4. Opcionalmente configura un modelo de fallback

No se requiere migración manual de datos.

---

## [3.5.0] - 2025-01-XX

### Added

- Métricas básicas de productividad
- Sistema de estadísticas
- Configuración personalizable

### Fixed

- Corrección de archivos `.suggested`
- Mejoras en el manejo de errores

---

## [3.3.0] - 2025-01-XX

### Added

- Stdin centralizado sin conflictos entre hilos
- Tests de Jest visibles en consola en tiempo real
- Debounce y drenado de eventos duplicados del watcher
- Comando 'p' para pausar/reanudar
- Comando 'r' para reportes diarios

### Changed

- Arquitectura de módulos separados
- Mejora en la estructura del código

---

## [3.2.0] - 2025-01-XX

### Added

- Reportes diarios de productividad generados con IA
- Análisis de commits del día

---

## [3.1.0] - 2025-01-XX

### Added

- Auto-documentación técnica (archivos .md generados automáticamente)
- "Manual de bolsillo" junto a cada archivo .ts

---

## [3.0.0] - 2024-12-XX

### Added

- Diagnóstico automático de fallos en tests
- Sugerencias de código en archivos `.suggested`
- Mensajes de commit inteligentes siguiendo Conventional Commits

---

## [2.0.0] - 2024-11-XX

### Added

- Integración con Claude AI para análisis de código
- Evaluación de principios SOLID y Clean Code
- Detección y ejecución automática de tests con Jest

---

## [1.0.0] - 2024-10-XX

### Added

- Monitoreo en tiempo real del sistema de archivos
- Flujo interactivo de commits con Git
- Soporte básico para proyectos NestJS/TypeScript
