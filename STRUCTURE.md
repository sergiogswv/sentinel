# Sentinel Project Structure

## Module Organization

The project has been refactored into specialized modules to improve maintainability and code clarity.

### Modules

```
src/
├── main.rs        # Entry point and main watcher loop
├── ai/            # AI integration module (modularized v4.4.3)
│   ├── mod.rs           # Module definition and re-exports
│   ├── cache.rs         # Response caching system
│   ├── client.rs        # Communication with AI APIs
│   ├── framework.rs     # Framework detection with AI
│   ├── analysis.rs      # Architecture analysis
│   └── utils.rs         # Utilities (extract/remove code)
├── config.rs      # Configuration management (.sentinelrc.toml)
├── docs.rs        # Documentation generation
├── files.rs       # Parent file detection utilities
├── git.rs         # Git operations
├── stats.rs       # Statistics and productivity metrics
├── tests.rs       # Test execution and diagnostics
└── ui.rs          # User interface and project validation
```

## Module Descriptions

### `main.rs`
**Responsibility**: Entry point and main orchestration

- File watcher configuration (notify)
- **Path and project structure validation** (v3.3.1):
  - Validates selected project existence
  - Validates `src/` directory existence
  - Descriptive error handling with `eprintln!`
- Main change detection loop
- Cross-module coordination
- Thread management (pause/report/stats/config)
- Shared state handling (Arc/Mutex)
- Centralized stdin reading via shared channel with keyboard thread
- Watcher event debounce to avoid duplicate processing
- Pending event draining after each processing

**Functions**:
- `main()` - Main entry point with robust validations
- `inicializar_sentinel(project_path: &Path) -> SentinelConfig` - Initializes or loads configuration

---

### `ai/` (v4.4.3 - Modularized Structure)
**Responsibility**: Complete integration with AI providers

The AI module has been refactored into specialized submodules for better maintainability:

#### `ai/mod.rs`
- Defines the module and its public re-exports
- Public API: `analizar_arquitectura`, `limpiar_cache`, `consultar_ia_dinamico`, `TaskType`, `detectar_framework_con_ia`, `listar_modelos_gemini`

#### `ai/cache.rs`
**Responsibility**: AI response caching system

**Public functions**:
- `limpiar_cache(project_path: &Path)` - Clears all cache
- `obtener_cache_path()`, `intentar_leer_cache()`, `guardar_en_cache()` - Hash-based cache management

**Implementation**:
- Hash-based storage in `.sentinel/cache/`
- Key: SHA hash of prompt
- Reduces API costs up to 70%

#### `ai/client.rs`
**Responsibility**: Communication with AI provider APIs

**Public functions**:
- `consultar_ia_dinamico(prompt, task_type, config, stats, project_path)` - Entry point with cache and fallback
- `consultar_ia(prompt, api_key, base_url, model_name, stats)` - Multi-provider base client
- `TaskType` enum - Light (commits, docs) vs Deep (architecture, debug)

**Provider implementations**:
- `consultar_anthropic()` - Anthropic Claude (Opus, Sonnet, Haiku)
- `consultar_gemini_content()` - Google Gemini Content API
- `consultar_gemini_interactions()` - Google Gemini Interactions API

**Fallback system**:
- `ejecutar_con_fallback()` - Tries primary model, automatic fallback if fails
- Token and cost tracking per query

**Dependencies**:
- `reqwest` - HTTP client
- `serde_json` - JSON serialization
- `colored` - Colored output

#### `ai/framework.rs`
**Responsibility**: Automatic framework detection with AI

**Public functions**:
- `detectar_framework_con_ia(project_path, config)` - Analyzes project and detects framework
- `listar_modelos_gemini(api_key)` - Gets available Gemini models

**Private functions**:
- `parsear_deteccion_framework(respuesta)` - JSON parser with fallback to generic configuration

**Detection process**:
1. Reads project root files (package.json, requirements.txt, composer.json)
2. Sends context to AI with specialized prompt
3. AI can request to read specific files for more context
4. Returns `FrameworkDetection` with: framework, code_language, rules, extensions, parent_patterns, test_patterns

**Dependencies**:
- `colored` - Colored output
- `reqwest` - HTTP client for Gemini API

#### `ai/analysis.rs`
**Responsibility**: Code architecture analysis

**Public functions**:
- `analizar_arquitectura(codigo, file_name, stats, config, project_path, file_path)` - Complete code analysis

**Process**:
1. Builds prompt with detected framework-specific architecture rules
2. Uses dynamic `code_language` for code blocks
3. Queries AI and evaluates response (CRITICAL vs SAFE)
4. Generates `.suggested` file with improved code
5. Updates statistics (bugs avoided, suggestions, time saved)
6. Shows advice without code blocks in console

**Dependencies**:
- `crate::ai::client` - For queries
- `crate::ai::utils` - For response processing

#### `ai/utils.rs`
**Responsibility**: AI response processing utilities

**Public functions**:
- `extraer_codigo(texto)` - Extracts ```language``` blocks from markdown responses
- `eliminar_bloques_codigo(texto)` - Removes code, keeps only explanations

**Supported languages**:
- typescript, javascript, python, php, go, rust, java, jsx, tsx, code

**Included unit tests**:
- `test_extraer_codigo_typescript()`
- `test_extraer_codigo_sin_lenguaje()`
- `test_extraer_codigo_sin_bloque()`
- `test_eliminar_bloques_codigo()`
- `test_eliminar_multiples_bloques()`

#### `ai/testing.rs`
**Responsibility**: Testing framework detection and validation

**Public functions**:
- `detectar_testing_framework(project_path, config)` - Analyzes and detects installed testing frameworks

**Data structures**:
- `TestingFrameworkInfo` - Complete testing analysis information
  - `testing_framework: Option<String>` - Main detected framework
  - `additional_frameworks: Vec<String>` - Additional frameworks
  - `config_files: Vec<String>` - Found configuration files
  - `status: TestingStatus` - Testing status (Valid, Incomplete, Missing)
  - `suggestions: Vec<TestingSuggestion>` - Installation suggestions
- `TestingStatus` - Enum: Valid, Incomplete, Missing
- `TestingSuggestion` - Suggestion with framework, reason, install_command, priority

**Detection process**:
1. **Static analysis**: Searches for config files (jest.config.js, pytest.ini, cypress.json, etc.)
2. **Dependency analysis**:
   - JavaScript/TypeScript: package.json (dependencies/devDependencies)
   - Python: requirements.txt
   - PHP: composer.json
   - Rust: Cargo.toml (native testing)
   - Go: go.mod (native testing)
3. **Status determination**: Valid (complete), Incomplete (no config), Missing (none)
4. **Suggestion generation**: Based on detected main framework
5. **AI validation**: Queries model to confirm and improve recommendations

**Supported frameworks by ecosystem**:
- **JavaScript/TypeScript**: Jest, Vitest, Cypress, Playwright, Mocha, Jasmine, Karma
- **Python**: Pytest, Unittest, Coverage.py
- **PHP**: PHPUnit, Pest, Laravel Dusk
- **Rust**: Built-in testing, cargo-tarpaulin
- **Go**: Go Testing, testify, httptest
- **Java**: JUnit 5, Spring Test, Mockito

**Contextual recommendations**:
- `obtener_frameworks_recomendados(framework)` - Returns appropriate frameworks by technology
  - React/Next.js → Jest, Vitest, Cypress
  - NestJS → Jest (integrated), Supertest, Cypress
  - Django/FastAPI → Pytest, Coverage.py
  - Laravel → PHPUnit, Pest, Laravel Dusk
  - Rust frameworks → Built-in testing with framework-specific helpers

**Helper functions**:
- `generar_comando_instalacion(framework, project_framework, manager)` - Generates specific installation commands
- `mostrar_resumen_testing(info)` - Shows colorful visual summary with priority indicators
- `consultar_ia_para_testing()` - Validation and improvement of recommendations with AI
- `parsear_testing_info()` - JSON parser with fallback to basic data

**Dependencies**:
- `crate::ai::client` - For AI queries
- `crate::config` - For project configuration
- `serde` - JSON serialization/deserialization
- `colored` - Colorful console output

---

### `git.rs`
**Responsibility**: Git operations

**Public functions**:
- `obtener_resumen_git(project_path: &Path) -> String`
  - Gets commits from the day (since 00:00:00)
  - Runs `git log --since=00:00:00`

- `generar_mensaje_commit(codigo: &str, file_name: &str) -> String`
  - Generates messages following Conventional Commits
  - Uses AI to create descriptive messages

- `generar_reporte_diario(project_path: &Path)`
  - Analyzes day's commits with AI
  - Generates report divided into: Achievements, Technical Aspects, Next Steps
  - Saves to `docs/DAILY_REPORT.md`

- `preguntar_commit(project_path: &Path, mensaje: &str, respuesta: &str)`
  - Executes `git add .` and `git commit -m` if response is "s"
  - stdin reading is centralized in `main.rs` to avoid thread conflicts

**Dependencies**:
- `crate::ai` - For AI analysis

---

### `tests.rs`
**Responsibility**: Test execution and diagnostics

**Public functions**:
- `ejecutar_tests(test_path: &str, project_path: &Path) -> Result<(), String>`
  - Runs Jest with `npm run test -- --findRelatedTests`
  - Shows Jest output in real-time in console
  - Returns Ok if tests pass, Err with exit code if they fail

- `pedir_ayuda_test(codigo: &str, error_jest: &str) -> Result<()>`
  - Requests AI diagnosis when tests fail
  - Shows suggested solution to user

**Dependencies**:
- `crate::ai` - For AI diagnostics

---

### `docs.rs`
**Responsibility**: Documentation generation

**Public functions**:
- `actualizar_documentacion(codigo: &str, file_path: &Path) -> Result<()>`
  - Generates "pocket manuals" in Markdown format
  - Ultra-concise summaries (maximum 150 words)
  - Creates .md files next to each modified file
  - Example: `src/users/users.service.ts` → `src/users/users.service.md`

**Dependencies**:
- `crate::ai` - To generate technical summaries

---

### `files.rs`
**Responsibility**: Parent file detection in NestJS modules

**Public functions**:
- `es_archivo_padre(file_name: &str) -> bool`
  - Checks if a file is a parent type (.service.ts, .controller.ts, etc.)
  - Supported patterns: service, controller, repository, module, gateway, guard, interceptor, pipe, filter

- `detectar_archivo_padre(changed_path: &Path, project_path: &Path) -> Option<String>`
  - Detects if a modified file is a "child" of a parent module
  - Searches for parent files in the same directory
  - If multiple parents exist, returns the highest priority (service > controller > repository > ...)
  - Returns `Some(base_name)` or `None` if no parent

**Parent priority** (highest to lowest):
1. `.service.ts` - Business logic
2. `.controller.ts` - HTTP endpoints
3. `.repository.ts` - Data access
4. `.gateway.ts` - WebSockets
5. `.module.ts` - NestJS modules
6. `.guard.ts`, `.interceptor.ts`, `.pipe.ts`, `.filter.ts` - Others

**Use cases**:
- Modified file: `src/calls/call-inbound.ts`
- Detected parent: `src/calls/call.service.ts`
- Test to run: `test/calls/calls.spec.ts` (parent module test, not child)

**Dependencies**:
- `std::fs` - Directory reading
- `std::path` - Path manipulation

**Unit tests**:
- Includes complete tests for file pattern verification
- Tests for files with dots in the name
- Priority validation

---

### `ui.rs`
**Responsibility**: Terminal user interface

**Public functions**:
- `seleccionar_proyecto() -> PathBuf`
  - Shows interactive menu of available projects
  - Scans parent directory (`../`)
  - Validates user selection is valid
  - Validates projects are available
  - Returns PathBuf of selected project
  - Handles errors with descriptive messages (v3.3.1)

---

### `config.rs`
**Responsibility**: Project configuration management

**Public functions**:
- `SentinelConfig::load(project_path: &Path) -> Option<SentinelConfig>`
  - Loads configuration from `.sentinelrc.toml`
  - Returns None if file doesn't exist

- `SentinelConfig::save(&self, project_path: &Path) -> Result<()>`
  - Saves current configuration to `.sentinelrc.toml`

- `SentinelConfig::default(nombre: String, gestor: String) -> Self`
  - Creates default configuration for a new project

- `SentinelConfig::detectar_gestor(project_path: &Path) -> String`
  - Detects package manager (npm, yarn, pnpm, bun)

- `SentinelConfig::debe_ignorar(&self, path: &Path) -> bool`
  - Checks if a file should be ignored according to configuration

- `SentinelConfig::abrir_en_editor(project_path: &Path)`
  - Opens configuration file in system editor

- `SentinelConfig::eliminar(project_path: &Path) -> Result<()>`
  - Removes configuration file

**Data structure**:
```rust
pub struct SentinelConfig {
    pub nombre_proyecto: String,
    pub gestor_paquetes: String,
    pub ignorar_patrones: Vec<String>,
}
```

**Dependencies**:
- `toml` - TOML serialization/deserialization
- `serde` - Serialization framework

---

### `stats.rs`
**Responsibility**: Statistics and productivity metrics

**Public functions**:
- `SentinelStats::cargar(project_path: &Path) -> Self`
  - Loads statistics from `.sentinel-stats.json`
  - Creates empty statistics if they don't exist

- `SentinelStats::guardar(&self, project_path: &Path)`
  - Saves current statistics to `.sentinel-stats.json`

- `SentinelStats::incrementar_bugs_evitados(&mut self)`
  - Increments counter of critical bugs avoided

- `SentinelStats::incrementar_sugerencias(&mut self)`
  - Increments counter of applied suggestions

- `SentinelStats::incrementar_tests_corregidos(&mut self)`
  - Increments counter of failed tests fixed with AI

- `SentinelStats::agregar_tiempo_ahorrado(&mut self, minutos: u32)`
  - Adds estimated time saved in minutes

**Data structure**:
```rust
pub struct SentinelStats {
    pub bugs_criticos_evitados: u32,
    pub sugerencias_aplicadas: u32,
    pub tests_fallidos_corregidos: u32,
    pub tiempo_estimado_ahorrado_mins: u32,
}
```

**Dependencies**:
- `serde` - Serialization
- `serde_json` - JSON format

---

## Data Flow

```
┌──────────────────────────────────────────────────────────────┐
│                         main.rs                              │
│                     (initialization)                         │
└──────┬───────────────────────────────────────────────────────┘
       │
       ├──▶ ui::seleccionar_proyecto()
       │         └──▶ Valid path validation (v3.3.1)
       │         └──▶ Available projects validation (v3.3.1)
       │
       ├──▶ Project existence validation (v3.3.1)
       │
       ├──▶ config::SentinelConfig::load() / inicializar_sentinel()
       │         └──▶ Loads .sentinelrc.toml or creates default config
       │         └──▶ Detects package manager (npm/yarn/pnpm/bun)
       │
       ├──▶ src/ directory existence validation (v3.3.1)
       │         └──▶ Descriptive error if doesn't exist
       │
       ├──▶ stats::SentinelStats::cargar()
       │         └──▶ Loads .sentinel-stats.json
       │
       └──▶ Watcher configuration with error validation (v3.3.1)

┌──────────────────────────────────────────────────────────────┐
│                    main.rs (main loop)                       │
│                    (file monitoring)                         │
└──────┬───────────────────────────────────────────────────────┘
       │
       ├──▶ config::debe_ignorar()  (file filtering per config)
       │
       ├──▶ files::detectar_archivo_padre()  (parent module detection - v4.2.0)
       │         └──▶ If child, uses parent name for tests
       │         └──▶ If not child, uses current file name
       │
       ├──▶ ai::analizar_arquitectura()  (advice in console, code in .suggested)
       │         └──▶ ai::client::consultar_ia_dinamico()  (v4.4.3 modularized)
       │               ├──▶ ai::cache::intentar_leer_cache() [if use_cache=true]
       │               ├──▶ ai::client::ejecutar_con_fallback() [if no cache]
       │               │     ├──▶ Tries primary_model
       │               │     └──▶ Tries fallback_model [if primary fails]
       │               └──▶ ai::cache::guardar_en_cache() [if success]
       │         └──▶ ai::utils::extraer_codigo() [extracts suggested code]
       │         └──▶ ai::utils::eliminar_bloques_codigo() [shows advice]
       │         └──▶ stats::incrementar_bugs_evitados() [if critical]
       │         └──▶ stats::incrementar_sugerencias() [if generates .suggested]
       │
       ├──▶ tests::ejecutar_tests()      (Jest output visible in console)
       │         └──▶ tests::pedir_ayuda_test() [if fails, with 30s timeout]
       │                   └──▶ stats::incrementar_tests_corregidos()
       │
       ├──▶ docs::actualizar_documentacion()
       │
       ├──▶ git::generar_mensaje_commit()
       │         └──▶ git::preguntar_commit() [with 30s timeout]
       │
       └──▶ stats::guardar()  (persists metrics)

┌──────────────────────────────────────────────────────────────┐
│           Keyboard thread (centralized stdin)                │
└──────┬───────────────────────────────────────────────────────┘
       │
       ├──▶ 'p'       ──▶ Pause/Resume monitoring
       │
       ├──▶ 'r'       ──▶ git::generar_reporte_diario()
       │
       ├──▶ 'm'       ──▶ Shows stats dashboard in console
       │
       ├──▶ 'c'       ──▶ config::abrir_en_editor()
       │
       ├──▶ 'x'       ──▶ config::eliminar() (with confirmation)
       │
       └──▶ 's'/'n'   ──▶ Forwards response to main loop (when awaiting input)

Optimization mechanisms:
  • Debounce: ignores duplicate events from same file (15s)
  • Draining: discards accumulated events after each processing
  • Early validation: descriptive errors before expensive operations
```

## Architecture Advantages

### Separation of Concerns
Each module has a clear and well-defined responsibility.

### Reusability
Functions can be reused in other contexts or projects.

### Maintainability
Easy to locate and modify specific functionalities.

### Testability
Each module can be tested independently.

### Scalability
Easy to add new functionalities without affecting existing code.

## Conventions

### Visibility
- Public functions: `pub fn` - Exposed to the rest of the project
- Private functions: `fn` - Internal module use

### Documentation
- Module comments: `//!` at the beginning of the file
- Function comments: `///` before each public function
- Includes: description, arguments, returns, side effects, examples

### Imports
- Internal modules: `use crate::module_name`
- External crates: `use crate_name`
- Grouping by type (std, external, internal)

## Next Steps

Possible architecture improvements:

- [x] Add `config.rs` module for centralized configuration (v3.3)
- [x] Add `stats.rs` module for productivity metrics (v3.3)
- [x] Robust path and directory validation (v3.3.1)
- [x] Modularize `ai.rs` into specialized submodules (v4.4.3)
- [ ] Create `errors.rs` module with custom error types
- [ ] Implement traits to abstract common functionalities
- [ ] Add unit tests for each module
- [ ] Documentation with `cargo doc`
- [ ] `security.rs` module for secret scanning (Phase 6)
- [ ] `pr.rs` module for GitHub API integration (Phase 7)
