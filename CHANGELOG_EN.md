# Changelog

All notable changes to Sentinel will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [4.6.0] - 2026-02-07

### 🚀 New Features

- **Multi-AI Provider Configuration**: Support for multiple simultaneous AI providers.
  - Supported: Claude, Gemini, OpenAI, Groq, Ollama, Kimi, DeepSeek.
  - Incremental configuration system: Add as many providers as you need.
  - Cascading fallback: If one provider fails, Sentinel automatically tries the next one in the list.
- **Dynamic Model Discovery**: Sentinel now queries provider APIs to list all your available models automatically.
- **Enhanced Configuration Assistant**: Completely revamped `sentinel init` process using `dialoguer` for an elite interactive experience.
- **Smart Migration**: Your old configurations are automatically migrated to the new provider list format while preserving your API keys.

### 🔧 Improvements & Refactors

- **IA-Agnostic Modules**: Framework detection, testing, and code analysis now work with any configured provider.
- **Robustness**: Fixed configuration saving issues during initialization.
- **UI UX**: Generic "AI ADVICE" labels instead of provider-specific ones.

---

## [4.5.0] - 2025-02-05

### 🚀 New Features

- **Intelligent Testing Framework Detection**: New automatic testing framework analysis system
  - Detects installed frameworks (Jest, Pytest, Vitest, Cypress, PHPUnit, etc.)
  - Validates existing configurations (config files, dependencies)
  - Suggests appropriate frameworks based on the project's main framework
  - Multi-language support: JavaScript/TypeScript, Python, PHP, Rust, Go, Java
  - Testing status: `valid`, `incomplete`, or `missing`

### ✨ Enhanced

- **Contextual Recommendations**: Testing suggestions adapt to the detected framework:
  - **React/Next.js**: Prioritizes Jest, Vitest, Cypress
  - **NestJS**: Recommends Jest (integrated by default) + Supertest
  - **Django/FastAPI**: Suggests Pytest as standard
  - **Laravel**: PHPUnit or Pest with Laravel Dusk for E2E
  - **Rust/Go**: Native language testing frameworks

### 🧪 Testing Intelligence

- **Static Analysis**: Detects configuration files (jest.config.js, pytest.ini, etc.)
- **Dependency Analysis**: Checks package.json, requirements.txt, composer.json, Cargo.toml
- **AI Validation**: Confirms and improves recommendations using configured model
- **Installation Commands**: Generates specific commands based on package manager (npm/yarn/pnpm/pip/composer)

### 📊 New Configuration Fields

```toml
[config]
testing_framework = "Jest"           # Detected testing framework
testing_status = "valid"             # Status: valid|incomplete|missing
```

### 🎨 UI Improvements

- Colorful visual summary of testing analysis
- Priority indicators for suggestions (🔥 high, ⭐ medium, 💡 low)
- Detailed information about detected frameworks and configuration files

### 🏗️ Architecture

- New module `src/ai/testing.rs` (450+ lines)
  - `TestingFrameworkInfo`: Testing information structure
  - `TestingStatus`: Enum for states (Valid, Incomplete, Missing)
  - `TestingSuggestion`: Suggestions with priority and installation commands
  - `detectar_testing_framework()`: Main detection function
  - Support for 20+ popular testing frameworks

### 🔧 Technical Details

- Integration with initialization process (`inicializar_sentinel`)
- Automatic detection during `sentinel init`
- Backwards compatible: optional configuration fields
- Compiles without warnings

---

## [4.4.3] - 2025-02-05

### 🏗️ Refactored

- **AI System Modularization**: Refactored `ai.rs` (678 lines) into organized modular structure:
  - `src/ai/mod.rs` - Module definition and public re-exports
  - `src/ai/cache.rs` - Caching system with hash-based storage
  - `src/ai/client.rs` - Communication with AI APIs (Anthropic, Gemini)
  - `src/ai/framework.rs` - Automatic framework detection with AI
  - `src/ai/analysis.rs` - Code architecture analysis
  - `src/ai/utils.rs` - Response processing utilities (extract/remove code blocks)

### ✨ Improved

- **Better maintainability**: Code organized by single responsibility
- **Enhanced navigation**: Easy to locate specific functionalities
- **Isolated testing**: Unit tests included in `utils.rs`
- **Clear documentation**: Each module documents its purpose with `//!` comments
- **Scalability**: Structure prepared to add new AI providers

### 🔧 Internal Changes

- Optimized public re-exports: Only functions used outside the AI module are exported
- Internal functions (`consultar_ia`, `eliminar_bloques_codigo`, `extraer_codigo`) are now module-private
- Updated internal imports to use submodule paths (`crate::ai::client::consultar_ia`)
- Clean compilation without warnings

### 📝 Documentation

- **ESTRUCTURA.md** / **STRUCTURE.md**: Updated with new modular structure of `src/ai/`
- **docs/architecture.md**: Updated component diagram and file structure
- Complete inline documentation in each submodule

### 💡 Benefits

- **Readability**: 6 specialized files vs 1 monolithic file
- **Separation of concerns**: Cache, client, framework, analysis, utils clearly divided
- **Facilitates contributions**: Developers can work on independent modules
- **Future-proof**: Extensible structure for new AI providers

---

## [4.4.2] - 2025-02-05

### 🐛 Fixed

- **Critical configuration bug**: Resolved issue where configuration was not read correctly when making project changes
  - Before: When modifying the project, Sentinel asked to reconfigure from scratch
  - Now: Configuration persists correctly between sessions

### ✨ Added

- **Configuration versioning system**: Added `version` field in `.sentinelrc.toml`
  - Allows tracking the configuration format version
  - Facilitates automatic migrations in future versions
- **Automatic configuration migration**:
  - Detects old configurations (without `version` field) and migrates them automatically
  - Preserves API keys and custom configurations
  - Validates and completes missing fields with appropriate defaults
- **Dynamic version**: Sentinel version is now read from `Cargo.toml` using `env!("CARGO_PKG_VERSION")`
  - Single source of truth for version
  - No more hardcoded versions in code
  - `SENTINEL_VERSION` constant used throughout the project

### 🔧 Changed

- **Robust configuration loading**: The `load()` function now:
  - Attempts to deserialize with current format
  - If it fails, tries old format (backward compatibility)
  - Automatically migrates and saves updated configuration
  - Shows clear messages during migration process
- **Configuration validation**: Missing fields are completed automatically:
  - `test_command`: If empty, uses `{manager} run test`
  - `ignore_patterns`: If empty, uses default patterns
  - `file_extensions`: If empty, uses `["js", "ts"]`
  - `architecture_rules`: If empty, uses default rules

### 📝 Documentation

- **MIGRATION.md**: New comprehensive migration guide
  - Detailed explanation of resolved problem
  - Migration process flow diagram
  - Before/after configuration examples
  - Migration system testing guide
- **CHANGELOG.md**: Updated with all v4.4.2 changes
- **README.md**: Version badge updated to 4.4.2

### 🏗️ Internal Changes

- New public constant `config::SENTINEL_VERSION` for version access from any module
- Private function `migrar_config()` to handle version updates
- Helper structure `SentinelConfigV1` for old config deserialization

### 💡 Use Cases

**Before (v4.4.1):**
```
User modifies project
→ Sentinel cannot read .sentinelrc.toml
→ Asks to reconfigure API keys and everything from scratch
→ 😞 Frustration, time wasted
```

**Now (v4.4.2):**
```
User modifies project
→ Detects config version
→ If old, migrates automatically
→ If fields missing, completes with defaults
→ Preserves API keys and custom configuration
→ 😊 Configuration ready without intervention
```

### 🔄 Migration

- **No user action required**: Migration is completely automatic
- **Data preservation**: API keys and custom configurations are maintained
- **Transparent update**: `.sentinelrc.toml` file updates automatically
- **Informative messages**: User sees when migration is performed

---

## [4.2.0] - 2025-02-04

### ✨ Added

- **Automatic parent file detection**: Sentinel now detects when a modified file is part of a larger module
  - Example: When modifying `src/calls/call-inbound.ts`, detects that `src/calls/call.service.ts` is the parent module
  - Runs parent module tests: `test/calls/calls.spec.ts` instead of looking for tests for the child file
  - Supports multiple parent file patterns: `.service.ts`, `.controller.ts`, `.repository.ts`, `.module.ts`, `.gateway.ts`, `.guard.ts`, `.interceptor.ts`, `.pipe.ts`, `.filter.ts`

### 🔧 Changed

- **Test detection logic**: Now searches for parent module before determining which tests to run
- **User notification**: Shows informative message when detecting a child file and using parent module tests

### 🎯 Improved

- **Better test coverage**: Child files now run complete module tests, detecting regressions
- **Smart priority**: When multiple parent files exist, uses the following priority order:
  1. `.service.ts` (business logic - highest priority)
  2. `.controller.ts` (HTTP endpoints)
  3. `.repository.ts` (data access)
  4. `.gateway.ts` (WebSockets)
  5. `.module.ts` (NestJS modules)
  6. Others (*.guard.ts, *.interceptor.ts, etc.)

### 📁 New Files

- `src/files.rs` - Module with utilities for parent file detection
  - `es_archivo_padre()` - Checks if a file matches parent patterns
  - `detectar_archivo_padre()` - Searches for parents in the same directory with priority

### 📝 Documentation

- **ESTRUCTURA.md**: Added documentation for `files.rs` module
- **docs/architecture.md**: Updated data flow with parent detection

### 🧪 Testing

- **Complete unit tests**: The `files.rs` module includes tests for:
  - Verification of all parent file patterns
  - Files with dots in the name (e.g., `user-v2.dto.ts`)
  - Edge cases: multiple parents, no parents, empty folders

### 💡 Use Cases

**Before (v4.1.1):**
```
Modified file: src/calls/call-inbound.ts
Test searched: test/call-inbound/call-inbound.spec.ts (doesn't exist)
Result: ❌ No tests run
```

**Now (v4.2.0):**
```
Modified file: src/calls/call-inbound.ts
Parent detected: src/calls/call.service.ts ℹ️
Test executed: test/calls/calls.spec.ts ✅
Result: ✅ Complete module tests executed
```

---

## [4.1.1] - 2025-02-03

### ✨ Added

- **Startup command help**: Sentinel now automatically shows the list of available commands at startup
- **Help command** (keys `h` or `help`): Shows command list at any time during execution
- Better user experience with clear description of each command

### 🔧 Changed

- Improved startup message with visible version number
- Help panel with clear and readable format
- **Command `c` removed**: Configuration is now edited manually according to user preference

### 🐛 Fixed

- **Real-time test output**: Jest tests now display correctly in console while running
- Improved error capture for AI diagnosis
- Tests now show Jest colors (`--colors`) for better readability
- When tests fail and help is requested, complete error is captured for AI analysis

### 🎯 Improved

- **More concise AI responses**: Solutions to test errors are now ultra-direct
  - Problem identified in one line
  - Solution in maximum 3 steps
  - Only shows code that needs to change (doesn't repeat entire file)
  - Maximum 150 words to maintain clarity

---

## [4.1.0] - 2025-02-03

### 🔒 Security

- **Automatic API Key protection**: Sentinel now automatically adds sensitive files to `.gitignore` when creating configuration:
  - `.sentinelrc.toml` (contains API keys)
  - `.sentinel_stats.json` (personal statistics)
  - `.sentinel/` (complete cache directory)
- Prevents accidental exposure of credentials in public repositories

### ✨ Added

- **Command to clear cache** (key `l`):
  - Removes all AI response cache with confirmation
  - Useful to free space or force fresh responses
  - Includes informative messages about cache status

### 🔧 Changed

- `.gitignore` file updates automatically when creating configuration
- Improvements in confirmation messages for destructive actions

### 📝 Documentation

- Updated documentation with new `l` command
- Improved cache management guide
- Security and API Key protection section added

---

## [4.0.0] - 2025-02-03

### 🚨 Breaking Changes

- **Renewed configuration**: Environment variables `ANTHROPIC_AUTH_TOKEN` and `ANTHROPIC_BASE_URL` have been replaced by a more flexible and portable `.sentinelrc.toml` configuration file
- **Multi-provider architecture**: The system now supports multiple AI providers, not just Anthropic Claude

### ✨ Added

- **Multi-provider AI support**:
  - Anthropic Claude (Opus, Sonnet, Haiku)
  - Google Gemini (2.0 Flash, 1.5 Pro, etc.)
  - Extensible structure to add more providers
- **Automatic fallback system**: Configure a backup model that activates if the primary fails
- **Smart response caching**: Reduces API costs up to 70% avoiding repeated queries
- **Real-time metrics dashboard** (command `m`):
  - Critical bugs avoided
  - Accumulated API cost
  - Tokens consumed
  - Estimated time saved
- **New interactive commands**:
  - `m` - View metrics dashboard
  - `c` - Open configuration in editor
  - `x` - Reset configuration
- **Interactive configuration assistant**: Step-by-step guide on first use
- **Automatic model listing**: For Gemini, shows available models during configuration
- **Cost and token tracking**: Persistent statistics in `.sentinel_stats.json`

### 🔧 Changed

- `.suggested` files are now saved in the same directory as the original file (previously saved in Sentinel directory)
- Completely renewed documentation with AI provider guides
- Better error messages and configuration validation

### 📁 New Files

- `.sentinelrc.toml` - Project configuration file
- `.sentinel_stats.json` - Persistent productivity metrics
- `.sentinel/cache/` - AI response cache directory

### 🔄 Migration Guide

To migrate from v3.x:

1. Update code to v4.0.0
2. Run Sentinel - configuration assistant will start
3. Enter your API Key when prompted
4. Optionally configure a fallback model

No manual data migration required.

---

## [3.5.0] - 2025-01-XX

### Added

- Basic productivity metrics
- Statistics system
- Customizable configuration

### Fixed

- `.suggested` file fixes
- Error handling improvements

---

## [3.3.0] - 2025-01-XX

### Added

- Centralized stdin without thread conflicts
- Jest tests visible in console in real-time
- Debounce and draining of duplicate watcher events
- Command 'p' to pause/resume
- Command 'r' for daily reports

### Changed

- Separate module architecture
- Code structure improvement

---

## [3.2.0] - 2025-01-XX

### Added

- AI-generated daily productivity reports
- Analysis of commits from the day

---

## [3.1.0] - 2025-01-XX

### Added

- Automatic technical documentation (.md files generated automatically)
- "Pocket manual" next to each .ts file

---

## [3.0.0] - 2024-12-XX

### Added

- Automatic diagnosis of test failures
- Code suggestions in `.suggested` files
- Smart commit messages following Conventional Commits

---

## [2.0.0] - 2024-11-XX

### Added

- Claude AI integration for code analysis
- Evaluation of SOLID principles and Clean Code
- Automatic Jest test detection and execution

---

## [1.0.0] - 2024-10-XX

### Added

- Real-time file system monitoring
- Interactive commit flow with Git
- Basic support for NestJS/TypeScript projects
