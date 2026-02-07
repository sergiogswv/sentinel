# Estructura del Proyecto Sentinel

## Organización de Módulos

El proyecto ha sido refactorizado en módulos especializados para mejorar la mantenibilidad y claridad del código.

### Módulos

```
src/
├── main.rs        # Punto de entrada y loop principal del watcher
├── ai/            # Módulo de integración con IA (modularizado v4.4.3)
│   ├── mod.rs           # Definición del módulo y re-exports
│   ├── cache.rs         # Sistema de caché de respuestas
│   ├── client.rs        # Comunicación con APIs de IA
│   ├── framework.rs     # Detección de frameworks con IA
│   ├── analysis.rs      # Análisis de arquitectura
│   └── utils.rs         # Utilidades (extraer/eliminar código)
├── config.rs      # Gestión de configuración (.sentinelrc.toml)
├── docs.rs        # Generación de documentación
├── files.rs       # Utilidades de detección de archivos padres
├── git.rs         # Operaciones de Git
├── stats.rs       # Estadísticas y métricas de productividad
├── tests.rs       # Ejecución y diagn��stico de tests
└── ui.rs          # Interfaz de usuario y validación de proyectos
```

## Descripción de Módulos

### `main.rs`
**Responsabilidad**: Punto de entrada y orquestación principal

- Configuración del file watcher (notify)
- **Validación de rutas y estructura del proyecto** (v3.3.1):
  - Valida existencia del proyecto seleccionado
  - Valida existencia del directorio `src/`
  - Manejo de errores descriptivos con `eprintln!`
- Loop principal de detección de cambios
- Coordinación entre módulos
- Gestión de hilos (pausa/reporte/estadísticas/config)
- Manejo de estado compartido (Arc/Mutex)
- Lectura centralizada de stdin mediante canal compartido con el hilo de teclado
- Debounce de eventos del watcher para evitar procesamiento duplicado
- Drenado de eventos pendientes después de cada procesamiento

**Funciones**:
- `main()` - Punto de entrada principal con validaciones robustas
- `inicializar_sentinel(project_path: &Path) -> SentinelConfig` - Inicializa o carga configuración

---

### `ai/` (v4.4.3 - Estructura Modularizada)
**Responsabilidad**: Integración completa con proveedores de IA

El módulo AI ha sido refactorizado en submódulos especializados para mejor mantenibilidad:

#### `ai/mod.rs`
- Define el módulo y sus re-exports públicos
- API pública: `analizar_arquitectura`, `limpiar_cache`, `consultar_ia_dinamico`, `TaskType`, `detectar_framework_con_ia`

#### `ai/cache.rs`
**Responsabilidad**: Sistema de caché de respuestas de IA

**Funciones públicas**:
- `limpiar_cache(project_path: &Path)` - Elimina todo el caché
- `obtener_cache_path()`, `intentar_leer_cache()`, `guardar_en_cache()` - Gestión de caché basada en hash

**Implementación**:
- Hash-based storage en `.sentinel/cache/`
- Clave: hash SHA del prompt
- Reduce costos de API hasta 70%

#### `ai/client.rs`
**Responsabilidad**: Comunicación con APIs de proveedores de IA

**Funciones públicas**:
- `consultar_ia_dinamico(prompt, task_type, config, stats, project_path)` - Punto de entrada con caché y fallback secuencial a través de todos los proveedores configurados
- `consultar_ia(prompt, config, stats)` - Cliente genérico independiente del proveedor
- `TaskType` enum - Light (commits, docs) vs Deep (arquitectura, debug)

**Implementaciones de proveedores**:
- `consultar_anthropic()` - Anthropic Claude (Opus, Sonnet, Haiku)
- `consultar_gemini_content()` - Google Gemini Content API
- `consultar_gemini_interactions()` - Google Gemini Interactions API

**Sistema de fallback**:
- Fallback secuencial: Itera a través de `config.ai_configs`, intentando cada proveedor hasta que uno tenga éxito.
- Tracking de tokens y costos por consulta

**Dependencias**:
- `reqwest` - Cliente HTTP
- `serde_json` - Serialización JSON
- `colored` - Output con colores

#### `ai/framework.rs`
**Responsabilidad**: Detección automática de frameworks con IA

**Funciones públicas**:
- `detectar_framework_con_ia(project_path, config)` - Analiza proyecto y deteta framework

**Funciones privadas**:
- `parsear_deteccion_framework(respuesta)` - Parser JSON con fallback a configuración genérica

**Proceso de detección**:
1. Lee archivos raíz del proyecto (package.json, requirements.txt, composer.json)
2. Envía contexto a IA con prompt especializado
3. IA puede solicitar leer archivos específicos para más contexto
4. Retorna `FrameworkDetection` con: framework, code_language, rules, extensions, parent_patterns, test_patterns

**Dependencias**:
- `colored` - Output con colores
- `reqwest` - Cliente HTTP para Gemini API

#### `ai/analysis.rs`
**Responsabilidad**: Análisis de arquitectura de código

**Funciones públicas**:
- `analizar_arquitectura(codigo, file_name, stats, config, project_path, file_path)` - Análisis completo de código

**Proceso**:
1. Construye prompt con reglas de arquitectura específicas del framework detectado
2. Usa `code_language` dinámico para bloques de código
3. Consulta IA y evalúa respuesta (CRITICO vs SEGURO)
4. Genera archivo `.suggested` con código mejorado
5. Actualiza estadísticas (bugs evitados, sugerencias, tiempo ahorrado)
6. Muestra consejo sin bloques de código en consola

**Dependencias**:
- `crate::ai::client` - Para consultas
- `crate::ai::utils` - Para procesamiento de respuestas

#### `ai/utils.rs`
**Responsabilidad**: Utilidades para procesamiento de respuestas de IA

**Funciones públicas**:
- `extraer_codigo(texto)` - Extrae bloques ```lenguaje``` de respuestas markdown
- `eliminar_bloques_codigo(texto)` - Remueve código, mantiene solo explicaciones

**Lenguajes soportados**:
- typescript, javascript, python, php, go, rust, java, jsx, tsx, code

**Tests unitarios incluidos**:
- `test_extraer_codigo_typescript()`
- `test_extraer_codigo_sin_lenguaje()`
- `test_extraer_codigo_sin_bloque()`
- `test_eliminar_bloques_codigo()`
- `test_eliminar_multiples_bloques()`

#### `ai/testing.rs`
**Responsabilidad**: Detección y validación de frameworks de testing

**Funciones públicas**:
- `detectar_testing_framework(project_path, config)` - Analiza y detecta frameworks de testing instalados

**Estructuras de datos**:
- `TestingFrameworkInfo` - Información completa del análisis de testing
  - `testing_framework: Option<String>` - Framework principal detectado
  - `additional_frameworks: Vec<String>` - Frameworks adicionales
  - `config_files: Vec<String>` - Archivos de configuración encontrados
  - `status: TestingStatus` - Estado del testing (Valid, Incomplete, Missing)
  - `suggestions: Vec<TestingSuggestion>` - Sugerencias de instalación
- `TestingStatus` - Enum: Valid, Incomplete, Missing
- `TestingSuggestion` - Sugerencia con framework, reason, install_command, priority

**Proceso de detección**:
1. **Análisis estático**: Busca archivos de configuración (jest.config.js, pytest.ini, cypress.json, etc.)
2. **Análisis de dependencias**:
   - JavaScript/TypeScript: package.json (dependencies/devDependencies)
   - Python: requirements.txt
   - PHP: composer.json
   - Rust: Cargo.toml (testing nativo)
   - Go: go.mod (testing nativo)
3. **Determinación de estado**: Valid (completo), Incomplete (sin config), Missing (ninguno)
4. **Generación de sugerencias**: Basadas en el framework principal detectado
5. **Validación con IA**: Consulta al modelo para confirmar y mejorar recomendaciones

**Frameworks soportados por ecosistema**:
- **JavaScript/TypeScript**: Jest, Vitest, Cypress, Playwright, Mocha, Jasmine, Karma
- **Python**: Pytest, Unittest, Coverage.py
- **PHP**: PHPUnit, Pest, Laravel Dusk
- **Rust**: Built-in testing, cargo-tarpaulin
- **Go**: Go Testing, testify, httptest
- **Java**: JUnit 5, Spring Test, Mockito

**Recomendaciones contextuales**:
- `obtener_frameworks_recomendados(framework)` - Retorna frameworks apropiados por tecnología
  - React/Next.js → Jest, Vitest, Cypress
  - NestJS → Jest (integrado), Supertest, Cypress
  - Django/FastAPI → Pytest, Coverage.py
  - Laravel → PHPUnit, Pest, Laravel Dusk
  - Rust frameworks → Built-in testing con framework-specific helpers

**Funciones auxiliares**:
- `generar_comando_instalacion(framework, project_framework, manager)` - Genera comandos de instalación específicos
- `mostrar_resumen_testing(info)` - Muestra resumen visual colorido con indicadores de prioridad
- `consultar_ia_para_testing()` - Validación y mejora de recomendaciones con IA
- `parsear_testing_info()` - Parser JSON con fallback a datos básicos

**Dependencias**:
- `crate::ai::client` - Para consultas a IA
- `crate::config` - Para configuración del proyecto
- `serde` - Serialización/deserialización JSON
- `colored` - Output colorido en consola

---

### `git.rs`
**Responsabilidad**: Operaciones de Git

**Funciones públicas**:
- `obtener_resumen_git(project_path: &Path) -> String`
  - Obtiene commits del día (desde 00:00:00)
  - Ejecuta `git log --since=00:00:00`

- `generar_mensaje_commit(codigo: &str, file_name: &str) -> String`
  - Genera mensajes siguiendo Conventional Commits
  - Usa Claude AI para crear mensajes descriptivos

- `generar_reporte_diario(project_path: &Path)`
  - Analiza commits del día con Claude AI
  - Genera reporte dividido en: Logros, Aspectos Técnicos, Próximos Pasos
  - Guarda en `docs/DAILY_REPORT.md`

- `preguntar_commit(project_path: &Path, mensaje: &str, respuesta: &str)`
  - Ejecuta `git add .` y `git commit -m` si la respuesta es "s"
  - La lectura de stdin se centraliza en `main.rs` para evitar conflictos entre hilos

**Dependencias**:
- `crate::ai` - Para análisis con IA

---

### `tests.rs`
**Responsabilidad**: Ejecución y diagnóstico de tests

**Funciones públicas**:
- `ejecutar_tests(test_path: &str, project_path: &Path) -> Result<(), String>`
  - Ejecuta Jest con `npm run test -- --findRelatedTests`
  - Muestra la salida de Jest en tiempo real en la consola
  - Retorna Ok si tests pasan, Err con código de salida si fallan

- `pedir_ayuda_test(codigo: &str, error_jest: &str) -> Result<()>`
  - Solicita diagnóstico a Claude AI cuando tests fallan
  - Muestra solución sugerida al usuario

**Dependencias**:
- `crate::ai` - Para diagnóstico con IA

---

### `docs.rs`
**Responsabilidad**: Generación de documentación

**Funciones públicas**:
- `actualizar_documentacion(codigo: &str, file_path: &Path) -> Result<()>`
  - Genera "manuales de bolsillo" en formato Markdown
  - Resúmenes ultra-concisos (máximo 150 palabras)
  - Crea archivos .md junto a cada .ts modificado
  - Ejemplo: `src/users/users.service.ts` → `src/users/users.service.md`

**Dependencias**:
- `crate::ai` - Para generar resúmenes técnicos

---

### `files.rs`
**Responsabilidad**: Detección de archivos padres en módulos NestJS

**Funciones públicas**:
- `es_archivo_padre(file_name: &str) -> bool`
  - Verifica si un archivo es de tipo padre (.service.ts, .controller.ts, etc.)
  - Patrones soportados: service, controller, repository, module, gateway, guard, interceptor, pipe, filter

- `detectar_archivo_padre(changed_path: &Path, project_path: &Path) -> Option<String>`
  - Detecta si un archivo modificado es un "hijo" de un módulo padre
  - Busca archivos padres en el mismo directorio
  - Si hay múltiples padres, retorna el de mayor prioridad (service > controller > repository > ...)
  - Retorna `Some(nombre_base)` o `None` si no hay padre

**Prioridad de padres** (de mayor a menor):
1. `.service.ts` - Lógica de negocio
2. `.controller.ts` - Endpoints HTTP
3. `.repository.ts` - Acceso a datos
4. `.gateway.ts` - WebSockets
5. `.module.ts` - Módulos NestJS
6. `.guard.ts`, `.interceptor.ts`, `.pipe.ts`, `.filter.ts` - Otros

**Casos de uso**:
- Archivo modificado: `src/calls/call-inbound.ts`
- Padre detectado: `src/calls/call.service.ts`
- Test a ejecutar: `test/calls/calls.spec.ts` (del módulo padre, no del hijo)

**Dependencias**:
- `std::fs` - Lectura de directorios
- `std::path` - Manipulación de rutas

**Tests unitarios**:
- Incluye tests completos para verificación de patrones de archivo
- Pruebas de archivos con puntos en el nombre
- Validación de prioridades

---

### `ui.rs`
**Responsabilidad**: Interfaz de usuario en terminal

**Funciones públicas**:
- `seleccionar_proyecto() -> PathBuf`
  - Muestra menú interactivo de proyectos disponibles
  - Escanea directorio padre (`../`)
  - Valida que la selección del usuario sea válida
  - Valida que haya proyectos disponibles
  - Retorna PathBuf del proyecto seleccionado
  - Maneja errores con mensajes descriptivos (v3.3.1)

---

### `config.rs`
**Responsabilidad**: Gestión de configuración del proyecto

**Funciones públicas**:
- `SentinelConfig::load(project_path: &Path) -> Option<SentinelConfig>`
  - Carga configuración desde `.sentinelrc.toml`
  - Retorna None si el archivo no existe

- `SentinelConfig::save(&self, project_path: &Path) -> Result<()>`
  - Guarda la configuración actual en `.sentinelrc.toml`

- `SentinelConfig::default(nombre: String, gestor: String) -> Self`
  - Crea configuración por defecto para un nuevo proyecto

- `SentinelConfig::detectar_gestor(project_path: &Path) -> String`
  - Detecta el gestor de paquetes (npm, yarn, pnpm, bun)

- `SentinelConfig::debe_ignorar(&self, path: &Path) -> bool`
  - Verifica si un archivo debe ser ignorado según la configuración

- `SentinelConfig::abrir_en_editor(project_path: &Path)`
  - Abre el archivo de configuración en el editor del sistema

- `SentinelConfig::eliminar(project_path: &Path) -> Result<()>`
  - Elimina el archivo de configuración

**Estructura de datos**:
```rust
pub struct SentinelConfig {
    pub version: String,
    pub project_name: String,
    pub manager: String,
    pub framework: String,
    pub architecture_rules: Vec<String>,
    pub file_extensions: Vec<String>,
    pub code_language: String,
    pub parent_patterns: Vec<String>,
    pub test_patterns: Vec<String>,
    pub ignore_patterns: Vec<String>,
    pub ai_configs: Vec<AIConfig>,
    pub use_cache: bool,
}
```

**Dependencias**:
- `toml` - Serialización/deserialización TOML
- `serde` - Framework de serialización

---

### `stats.rs`
**Responsabilidad**: Estadísticas y métricas de productividad

**Funciones públicas**:
- `SentinelStats::cargar(project_path: &Path) -> Self`
  - Carga estadísticas desde `.sentinel-stats.json`
  - Crea estadísticas vacías si no existen

- `SentinelStats::guardar(&self, project_path: &Path)`
  - Guarda las estadísticas actuales en `.sentinel-stats.json`

- `SentinelStats::incrementar_bugs_evitados(&mut self)`
  - Incrementa contador de bugs críticos evitados

- `SentinelStats::incrementar_sugerencias(&mut self)`
  - Incrementa contador de sugerencias aplicadas

- `SentinelStats::incrementar_tests_corregidos(&mut self)`
  - Incrementa contador de tests fallidos corregidos con IA

- `SentinelStats::agregar_tiempo_ahorrado(&mut self, minutos: u32)`
  - Agrega tiempo estimado ahorrado en minutos

**Estructura de datos**:
```rust
pub struct SentinelStats {
    pub bugs_criticos_evitados: u32,
    pub sugerencias_aplicadas: u32,
    pub tests_fallidos_corregidos: u32,
    pub tiempo_estimado_ahorrado_mins: u32,
}
```

**Dependencias**:
- `serde` - Serialización
- `serde_json` - Formato JSON

---

## Flujo de Datos

```
┌──────────────────────────────────────────────────────────────┐
│                         main.rs                              │
│                     (inicialización)                         │
└──────┬───────────────────────────────────────────────────────┘
       │
       ├──▶ ui::seleccionar_proyecto()
       │         └──▶ Validación de ruta válida (v3.3.1)
       │         └──▶ Validación de proyectos disponibles (v3.3.1)
       │
       ├──▶ Validación de existencia del proyecto (v3.3.1)
       │
       ├──▶ config::SentinelConfig::load() / inicializar_sentinel()
       │         └──▶ Carga .sentinelrc.toml o crea configuración por defecto
       │         └──▶ Detecta gestor de paquetes (npm/yarn/pnpm/bun)
       │
       ├──▶ Validación de existencia de directorio src/ (v3.3.1)
       │         └──▶ Error descriptivo si no existe
       │
       ├──▶ stats::SentinelStats::cargar()
       │         └──▶ Carga .sentinel-stats.json
       │
       └──▶ Configuración del watcher con validación de errores (v3.3.1)

┌──────────────────────────────────────────────────────────────┐
│                    main.rs (loop principal)                  │
│                    (monitoreo de archivos)                   │
└──────┬───────────────────────────────────────────────────────┘
       │
       ├──▶ config::debe_ignorar()  (filtrado de archivos según config)
       │
       ├──▶ files::detectar_archivo_padre()  (detección de módulo padre - v4.2.0)
       │         └──▶ Si es hijo, usa nombre del padre para tests
       │         └──▶ Si no es hijo, usa nombre del archivo actual
       │
       ├──▶ ai::analizar_arquitectura()  (consejo en consola, código en .suggested)
       │         └──▶ ai::client::consultar_ia_dinamico()  (v4.6.0 multi-proveedor)
       │               ├──▶ ai::cache::intentar_leer_cache() [si use_cache=true]
       │               ├──▶ ai::client::consultar_ia() [si no hay caché, itera proveedores]
       │               │     ├──▶ Intenta Config #1
       │               │     ├──▶ Intenta Config #2 [si #1 falla]
       │               │     └──▶ ... y así sucesivamente
       │               └──▶ ai::cache::guardar_en_cache() [si éxito]
       │         └──▶ ai::utils::extraer_codigo() [extrae código sugerido]
       │         └──▶ ai::utils::eliminar_bloques_codigo() [muestra consejo]
       │         └──▶ stats::incrementar_bugs_evitados() [si crítico]
       │         └──▶ stats::incrementar_sugerencias() [si genera .suggested]
       │
       ├──▶ tests::ejecutar_tests()      (salida de Jest visible en consola)
       │         └──▶ tests::pedir_ayuda_test() [si falla, con timeout 30s]
       │                   └──▶ stats::incrementar_tests_corregidos()
       │
       ├──▶ docs::actualizar_documentacion()
       │
       ├──▶ git::generar_mensaje_commit()
       │         └──▶ git::preguntar_commit() [con timeout 30s]
       │
       └──▶ stats::guardar()  (persiste métricas)

┌──────────────────────────────────────────────────────────────┐
│           Hilo de teclado (stdin centralizado)               │
└──────┬───────────────────────────────────────────────────────┘
       │
       ├──▶ 'p'       ──▶ Pausa/Reanuda monitoreo
       │
       ├──▶ 'r'       ──▶ git::generar_reporte_diario()
       │
       ├──▶ 'm'       ──▶ Muestra dashboard de stats en consola
       │
       ├──▶ 'c'       ──▶ config::abrir_en_editor()
       │
       ├──▶ 'x'       ──▶ config::eliminar() (con confirmación)
       │
       └──▶ 's'/'n'   ──▶ Reenvía respuesta al loop principal (cuando espera input)

Mecanismos de optimización:
  • Debounce: ignora eventos duplicados del mismo archivo (15s)
  • Drenado: descarta eventos acumulados después de cada procesamiento
  • Validación temprana: errores descriptivos antes de operaciones costosas
```

## Ventajas de esta Arquitectura

### Separación de Responsabilidades
Cada módulo tiene una responsabilidad clara y bien definida.

### Reusabilidad
Las funciones pueden ser reutilizadas en otros contextos o proyectos.

### Mantenibilidad
Es fácil localizar y modificar funcionalidades específicas.

### Testabilidad
Cada módulo puede ser testeado independientemente.

### Escalabilidad
Fácil agregar nuevas funcionalidades sin afectar código existente.

## Convenciones

### Visibilidad
- Funciones públicas: `pub fn` - Expuestas al resto del proyecto
- Funciones privadas: `fn` - Uso interno del módulo

### Documentación
- Comentarios de módulo: `//!` al inicio del archivo
- Comentarios de función: `///` antes de cada función pública
- Incluye: descripción, argumentos, retornos, efectos secundarios, ejemplos

### Imports
- Módulos internos: `use crate::nombre_modulo`
- Crates externos: `use nombre_crate`
- Agrupación por tipo (std, externos, internos)

## Próximos Pasos

Posibles mejoras a la arquitectura:

- [x] Agregar módulo `config.rs` para configuración centralizada (v3.3)
- [x] Agregar módulo `stats.rs` para métricas de productividad (v3.3)
- [x] Validación robusta de rutas y directorios (v3.3.1)
- [ ] Crear módulo `errors.rs` con tipos de error personalizados
- [ ] Implementar traits para abstraer funcionalidades comunes
- [ ] Agregar tests unitarios para cada módulo
- [ ] Documentación con `cargo doc`
- [ ] Módulo `security.rs` para escaneo de secretos (Fase 4)
- [ ] Módulo `pr.rs` para integración con GitHub API (Fase 5)
