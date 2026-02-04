# Installation Guide

This guide covers the requirements, installation steps, initial configuration, and expected project structure for Sentinel.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- API Key from at least one AI provider (Claude or Gemini recommended)

## Installation Steps

### 1. Clone the repository

```bash
git clone https://github.com/<tu-usuario>/sentinel-rust.git
cd sentinel-rust
```

### 2. Compile in release mode

```bash
cargo build --release
```

The compiled binary will be located at `target/release/sentinel-rust` (or `sentinel-rust.exe` on Windows).

## Initial Configuration

When you run Sentinel for the first time in a project, an interactive assistant will guide you through the configuration process:

### 1. Configure the primary model

```
👉 API Key: sk-ant-api03-...
👉 URL [Press Enter for Anthropic]: https://api.anthropic.com
```

**Supported providers:**
- **Anthropic Claude**: `https://api.anthropic.com` (default)
- **Google Gemini**: `https://generativelanguage.googleapis.com`
- Other endpoints compatible with Anthropic format

### 2. Configure fallback model (optional)

```
👉 Configure a backup model in case the primary fails? (s/n): s
👉 API Key: [your-api-key]
👉 Model URL: [provider-url]
👉 Model name: [model-name]
```

The system will try to use the primary model first, and in case of failure, it will automatically use the fallback model.

### 3. Generated configuration file

The configuration is saved in `.sentinelrc.toml` in the project root directory:

```toml
[project]
project_name = "mi-proyecto"
framework = "NestJS"
manager = "npm"
test_command = "npm run test"
use_cache = true

[primary_model]
name = "claude-opus-4-5-20251101"
url = "https://api.anthropic.com"
api_key = "sk-ant-api03-..."

[fallback_model]  # Optional
name = "gemini-2.0-flash"
url = "https://generativelanguage.googleapis.com"
api_key = "AIza..."

[[architecture_rules]]
"SOLID Principles"
"Clean Code"
"NestJS Best Practices"
```

## Expected Project Structure

Sentinel expects your NestJS project to have the following structure:

```
mi-proyecto/
├── src/              ← REQUIRED: Sentinel watches this directory
│   └── users/
│       └── users.service.ts
└── test/
    └── users/
        └── users.spec.ts
```

**Important requirements:**
- The project **MUST** have a `src/` directory (Sentinel will validate this on startup)
- For each file `src/module/file.ts`, there must exist `test/module/file.spec.ts`
- If the project doesn't have `src/`, Sentinel will show a descriptive error and stop

## Starting Sentinel

```bash
# From the project directory
cargo run

# Or using the compiled binary
./target/release/sentinel-rust
```

When starting, you will see:
1. Project selection menu
2. Configuration loading (or interactive assistant if it's the first time)
3. **Command help** automatically displayed
4. Monitoring starts

---

**Navigation:**
- [← Back to README](../README.md)
- [Next: Configuration →](configuration.md)
