# Changelog

All notable changes to Sentinel will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [4.5.0] - 2025-02-05

### 🚀 New Features

- **Detección Inteligente de Testing Frameworks**: Nuevo sistema de análisis automático de frameworks de testing
  - Detecta frameworks instalados (Jest, Pytest, Vitest, Cypress, PHPUnit, etc.)
  - Valida configuraciones existentes (archivos de configuración, dependencias)
  - Sugiere frameworks apropiados basados en el framework principal del proyecto
  - Soporte multi-lenguaje: JavaScript/TypeScript, Python, PHP, Rust, Go, Java
  - Estado de testing: `valid`, `incomplete`, o `missing`

### ✨ Enhanced

- **Recomendaciones Contextuales**: Las sugerencias de testing se adaptan al framework detectado:
  - **React/Next.js**: Prioriza Jest, Vitest, Cypress
  - **NestJS**: Recomienda Jest (integrado por defecto) + Supertest
  - **Django/FastAPI**: Sugiere Pytest como estándar
  - **Laravel**: PHPUnit o Pest con Laravel Dusk para E2E
  - **Rust/Go**: Frameworks de testing nativos del lenguaje

### 🧪 Testing Intelligence

- **Análisis Estático**: Detecta archivos de configuración (jest.config.js, pytest.ini, etc.)
- **Análisis de Dependencias**: Verifica package.json, requirements.txt, composer.json, Cargo.toml
- **Validación con IA**: Confirma y mejora recomendaciones usando el modelo configurado
- **Comandos de Instalación**: Genera comandos específicos según el gestor de paquetes (npm/yarn/pnpm/pip/composer)

### 📊 New Configuration Fields

```toml
[config]
testing_framework = "Jest"           # Framework de testing detectado
testing_status = "valid"             # Estado: valid|incomplete|missing
```

### 🎨 UI Improvements

- Resumen visual colorido del análisis de testing
- Indicadores de prioridad para sugerencias (🔥 alta, ⭐ media, 💡 baja)
- Información detallada sobre frameworks detectados y archivos de configuración

### 🏗️ Architecture

- Nuevo módulo `src/ai/testing.rs` (450+ líneas)
  - `TestingFrameworkInfo`: Estructura de información de testing
  - `TestingStatus`: Enum para estados (Valid, Incomplete, Missing)
  - `TestingSuggestion`: Sugerencias con prioridad y comandos de instalación
  - `detectar_testing_framework()`: Función principal de detección
  - Soporte para 20+ frameworks de testing populares

### 🔧 Technical Details

- Integración con proceso de inicialización (`inicializar_sentinel`)
- Detección automática durante `sentinel init`
- Backwards compatible: campos opcionales en configuración
- Sin warnings de compilación

---

## [4.4.3] - 2025-02-05

### 🏗️ Refactored

- **Modularización del sistema AI**: Refactorizado `ai.rs` (678 líneas) en estructura modular organizada:
  - `src/ai/mod.rs` - Definición del módulo y re-exports públicos
  - `src/ai/cache.rs` - Sistema de caché con almacenamiento basado en hash
  - `src/ai/client.rs` - Comunicación con APIs de IA (Anthropic, Gemini)
  - `src/ai/framework.rs` - Detección automática de frameworks con IA
  - `src/ai/analysis.rs` - Análisis de arquitectura de código
  - `src/ai/utils.rs` - Utilidades para procesamiento de respuestas (extraer/eliminar bloques de código)

### ✨ Improved

- **Mejor mantenibilidad**: Código organizado por responsabilidad única
- **Navegación mejorada**: Fácil localizar funcionalidades específicas
- **Testing aislado**: Tests unitarios incluidos en `utils.rs`
- **Documentación clara**: Cada módulo documenta su propósito con comentarios `//!`
- **Escalabilidad**: Estructura preparada para agregar nuevos proveedores de IA

### 🔧 Internal Changes

- Optimización de re-exports públicos: Solo se exportan funciones usadas fuera del módulo AI
- Funciones internas (`consultar_ia`, `eliminar_bloques_codigo`, `extraer_codigo`) ahora son privadas al módulo
- Imports internos actualizados para usar rutas del submódulo (`crate::ai::client::consultar_ia`)
- Compilación limpia sin warnings

### 📝 Documentation

- **ESTRUCTURA.md**: Actualizado con nueva estructura modular de `src/ai/`
- **docs/architecture.md**: Actualizado diagrama de componentes y estructura de archivos
- Documentación inline completa en cada submódulo

### 💡 Benefits

- **Legibilidad**: 6 archivos especializados vs 1 archivo monolítico
- **Separación de responsabilidades**: Cache, client, framework, analysis, utils claramente divididos
- **Facilita contribuciones**: Desarrolladores pueden trabajar en módulos independientes
- **Preparado para el futuro**: Estructura extensible para nuevos proveedores de IA

---

## [4.4.2] - 2025-02-05

### 🐛 Fixed

- **Bug crítico de configuración**: Resuelto el problema donde la configuración no se leía correctamente al hacer cambios en el proyecto
  - Antes: Al modificar el proyecto, Sentinel pedía reconfigurar desde cero
  - Ahora: La configuración persiste correctamente entre sesiones

### ✨ Added

- **Sistema de versiones de configuración**: Agregado campo `version` en `.sentinelrc.toml`
  - Permite rastrear la versión de formato de configuración
  - Facilita migraciones automáticas en futuras versiones
- **Migración automática de configuraciones**:
  - Detecta configuraciones antiguas (sin campo `version`) y las migra automáticamente
  - Preserva API keys y configuraciones personalizadas
  - Valida y completa campos faltantes con valores por defecto apropiados
- **Versión dinámica**: La versión de Sentinel ahora se lee desde `Cargo.toml` usando `env!("CARGO_PKG_VERSION")`
  - Single source of truth para la versión
  - No más versiones harcodeadas en el código
  - La constante `SENTINEL_VERSION` se usa en todo el proyecto

### 🔧 Changed

- **Carga robusta de configuración**: La función `load()` ahora:
  - Intenta deserializar con el formato actual
  - Si falla, intenta con formato antiguo (compatibilidad backward)
  - Migra automáticamente y guarda la configuración actualizada
  - Muestra mensajes claros durante el proceso de migración
- **Validación de configuración**: Campos faltantes se completan automáticamente:
  - `test_command`: Si está vacío, usa `{manager} run test`
  - `ignore_patterns`: Si está vacío, usa patrones por defecto
  - `file_extensions`: Si está vacío, usa `["js", "ts"]`
  - `architecture_rules`: Si está vacío, usa reglas por defecto

### 📝 Documentation

- **MIGRATION.md**: Nueva guía completa de migración de configuraciones
  - Explicación detallada del problema resuelto
  - Diagrama de flujo del proceso de migración
  - Ejemplos de configuraciones antes/después
  - Guía de testing del sistema de migración
- **CHANGELOG.md**: Actualizado con todos los cambios de v4.4.2
- **README.md**: Badge de versión actualizado a 4.4.2

### 🏗️ Internal Changes

- Nueva constante pública `config::SENTINEL_VERSION` para acceso a la versión desde cualquier módulo
- Función privada `migrar_config()` para manejar actualizaciones de versión
- Estructura auxiliar `SentinelConfigV1` para deserialización de configs antiguas

### 💡 Use Cases

**Antes (v4.4.1):**
```
Usuario modifica proyecto
→ Sentinel no puede leer .sentinelrc.toml
→ Pide reconfigurar API keys y todo desde cero
→ 😞 Frustración, pérdida de tiempo
```

**Ahora (v4.4.2):**
```
Usuario modifica proyecto
→ Detecta versión de config
→ Si es antigua, migra automáticamente
→ Si faltan campos, los completa con defaults
→ Preserva API keys y configuración personalizada
→ 😊 Configuración lista sin intervención
```

### 🔄 Migration

- **No requiere acción del usuario**: La migración es completamente automática
- **Preservación de datos**: API keys y configuraciones personalizadas se mantienen
- **Actualización transparente**: El archivo `.sentinelrc.toml` se actualiza automáticamente
- **Mensajes informativos**: Usuario ve cuando se realiza una migración

---

## [4.2.0] - 2025-02-04

### ✨ Added

- **Detección automática de archivos padres**: Sentinel ahora detecta cuando un archivo modificado es parte de un módulo más grande
  - Ejemplo: Al modificar `src/calls/call-inbound.ts`, detecta que `src/calls/call.service.ts` es el módulo padre
  - Ejecuta los tests del módulo padre: `test/calls/calls.spec.ts` en lugar de buscar tests para el archivo hijo
  - Soporta múltiples patrones de archivos padres: `.service.ts`, `.controller.ts`, `.repository.ts`, `.module.ts`, `.gateway.ts`, `.guard.ts`, `.interceptor.ts`, `.pipe.ts`, `.filter.ts`

### 🔧 Changed

- **Lógica de detección de tests**: Ahora busca el módulo padre antes de determinar qué tests ejecutar
- **Notificación al usuario**: Muestra un mensaje informativo cuando detecta un archivo hijo y usa los tests del módulo padre

### 🎯 Improved

- **Mejor cobertura de tests**: Los archivos hijos ahora ejecutan los tests completos del módulo, detectando regresiones
- **Prioridad inteligente**: Cuando existen múltiples archivos padres, usa el siguiente orden de prioridad:
  1. `.service.ts` (lógica de negocio - máxima prioridad)
  2. `.controller.ts` (endpoints HTTP)
  3. `.repository.ts` (acceso a datos)
  4. `.gateway.ts` (WebSockets)
  5. `.module.ts` (módulos NestJS)
  6. Otros (*.guard.ts, *.interceptor.ts, etc.)

### 📁 New Files

- `src/files.rs` - Módulo con utilidades para detección de archivos padres
  - `es_archivo_padre()` - Verifica si un archivo coincide con patrones de padre
  - `detectar_archivo_padre()` - Busca padres en el mismo directorio con prioridad

### 📝 Documentation

- **ESTRUCTURA.md**: Agregada documentación del módulo `files.rs`
- **docs/architecture.md**: Actualizado el flujo de datos con detección de padres

### 🧪 Testing

- **Tests unitarios completos**: El módulo `files.rs` incluye tests para:
  - Verificación de todos los patrones de archivos padres
  - Archivos con puntos en el nombre (ej: `user-v2.dto.ts`)
  - Casos edge: múltiples padres, sin padres, carpetas vacías

### 💡 Use Cases

**Antes (v4.1.1):**
```
Archivo modificado: src/calls/call-inbound.ts
Test buscado: test/call-inbound/call-inbound.spec.ts (no existe)
Resultado: ❌ No se ejecutan tests
```

**Ahora (v4.2.0):**
```
Archivo modificado: src/calls/call-inbound.ts
Padre detectado: src/calls/call.service.ts ℹ️
Test ejecutado: test/calls/calls.spec.ts ✅
Resultado: ✅ Tests del módulo completo ejecutados
```

---

## [4.1.1] - 2025-02-03

### ✨ Added

- **Ayuda de comandos al inicio**: Sentinel ahora muestra automáticamente la lista de comandos disponibles al iniciar
- **Comando de ayuda** (teclas `h` o `help`): Muestra la lista de comandos en cualquier momento durante la ejecución
- Mejor experiencia de usuario con descripción clara de cada comando

### 🔧 Changed

- Mensaje de inicio mejorado con número de versión visible
- Panel de ayuda con formato claro y legible
- **Comando `c` eliminado**: La configuración ahora se edita manualmente según preferencia del usuario

### 🐛 Fixed

- **Salida de tests en tiempo real**: Los tests de Jest ahora se muestran correctamente en la consola mientras se ejecutan
- Mejora en la captura de errores para diagnóstico con IA
- Los tests ahora muestran colores de Jest (`--colors`) para mejor legibilidad
- Cuando los tests fallan y se solicita ayuda, se captura el error completo para análisis de IA

### 🎯 Improved

- **Respuestas de IA más concisas**: Las soluciones a errores de tests ahora son ultra-directas
  - Problema identificado en una línea
  - Solución en máximo 3 pasos
  - Solo muestra el código que debe cambiar (no repite todo el archivo)
  - Máximo 150 palabras para mantener claridad

---

## [4.1.0] - 2025-02-03

### 🔒 Security

- **Protección automática de API Keys**: Sentinel ahora agrega automáticamente archivos sensibles al `.gitignore` al crear la configuración:
  - `.sentinelrc.toml` (contiene API keys)
  - `.sentinel_stats.json` (estadísticas personales)
  - `.sentinel/` (directorio completo de caché)
- Previene la exposición accidental de credenciales en repositorios públicos

### ✨ Added

- **Comando para limpiar caché** (tecla `l`):
  - Elimina todo el caché de respuestas de IA con confirmación
  - Útil para liberar espacio o forzar respuestas frescas
  - Incluye mensajes informativos sobre el estado del caché

### 🔧 Changed

- El archivo `.gitignore` se actualiza automáticamente al crear la configuración
- Mejoras en los mensajes de confirmación para acciones destructivas

### 📝 Documentation

- Documentación actualizada con el nuevo comando `l`
- Guía de gestión de caché mejorada
- Sección de seguridad y protección de API Keys agregada

---

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
