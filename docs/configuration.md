# Configuration Guide

Sentinel uses a `.sentinelrc.toml` file per project that is automatically created on first use.

## Configuration File (.sentinelrc.toml)

The configuration includes:

- **Version**: Configuration format version (for migration)
- **AI Models**: Primary model and optional fallback model
- **Supported Providers**: Claude (Anthropic), Gemini (Google), and others
- **Cache**: Enabled by default to reduce costs
- **Architecture Rules**: Customizable (SOLID, Clean Code, etc.)
- **Framework**: NestJS by default, configurable for other frameworks

> **Note**: Environment variables are no longer needed. Everything is managed from `.sentinelrc.toml`

## Configuration Structure

```toml
version = "4.4.3"  # Configuration format version
project_name = "mi-proyecto"
framework = "NestJS"
manager = "npm"
test_command = "npm run test"
architecture_rules = ["SOLID Principles", "Clean Code", "NestJS Best Practices"]
file_extensions = ["js", "ts", "jsx", "tsx"]
ignore_patterns = ["node_modules", "dist", ".git", "build"]
use_cache = true

[primary_model]
name = "claude-opus-4-5-20251101"
url = "https://api.anthropic.com"
api_key = "sk-ant-api03-..."

# Optional fallback model
fallback_model = { name = "gemini-2.0-flash", url = "https://generativelanguage.googleapis.com", api_key = "AIza..." }
```

## Model Configuration

### Primary Model

The primary model is used for all AI analysis by default. Configure:
- `name`: Model identifier (e.g., `claude-opus-4-5-20251101`)
- `url`: Provider API endpoint
- `api_key`: Your API key for the provider

### Fallback Model (Optional)

The fallback model activates automatically if the primary model fails:

```
Primary Model: Claude Opus (deep analysis)
      ↓ (if fails)
Fallback Model: Gemini Flash (fast response)
```

This ensures high availability and reduces workflow interruptions.

## Architecture Rules

Customize the rules that Sentinel uses to analyze your code:

```toml
[[architecture_rules]]
"SOLID Principles"
"Clean Code"
"NestJS Best Practices"
"Domain-Driven Design"
"Hexagonal Architecture"
```

These rules are sent to the AI model as context for code analysis.

## Cache Settings

The cache system stores AI responses to reduce costs and improve response times:

```toml
[project]
use_cache = true  # Enable/disable cache
```

**Benefits:**
- **Cost reduction**: Avoids repeated API queries
- **Instant response**: Cached queries are immediate
- **No quality loss**: The response is identical to the original

**Cache location:** `.sentinel/cache/`

To disable cache, change to `false` and restart Sentinel.

## Configuration Migration (v4.4.2+)

Starting with v4.4.2, Sentinel includes an **automatic migration system** for configuration files.

### Features

- ✅ **Automatic detection**: Detects old configuration formats automatically
- ✅ **Data preservation**: API keys and custom settings are preserved
- ✅ **Field validation**: Missing fields are completed with appropriate defaults
- ✅ **Version tracking**: Each config has a `version` field for compatibility

### How It Works

1. **Old config detected**: If your `.sentinelrc.toml` doesn't have a `version` field, it's considered old
2. **Automatic migration**: Sentinel migrates it to the latest format
3. **Safe update**: Your API keys and custom settings are preserved
4. **Transparent**: You'll see migration messages in the console

### Example Migration

**Before (v4.4.1 or earlier):**
```toml
project_name = "mi-app"
framework = "React"
manager = "npm"

[primary_model]
name = "claude-opus-4-5-20251101"
url = "https://api.anthropic.com"
api_key = "sk-ant-..."
```

**After (v4.4.2 automatic migration):**
```toml
version = "4.4.2"  # ← Automatically added
project_name = "mi-app"
framework = "React"
manager = "npm"
test_command = "npm run test"  # ← Auto-completed if missing
architecture_rules = ["Clean Code", "SOLID Principles"]  # ← Auto-completed
file_extensions = ["js", "ts"]  # ← Auto-completed
ignore_patterns = ["node_modules", "dist", ".git"]  # ← Auto-completed

[primary_model]
name = "claude-opus-4-5-20251101"
url = "https://api.anthropic.com"
api_key = "sk-ant-..."  # ← Preserved
```

### Migration Messages

When Sentinel migrates your configuration, you'll see:

```
   🔄 Detectada configuración antigua, migrando...
   ✅ Configuración migrada exitosamente
```

Or for version updates:

```
   🔄 Migrando configuración de versión 4.4.1 a 4.4.2...
   ✅ Configuración migrada exitosamente
```

[Read more about migration →](../MIGRATION.md)

## Editing Configuration

**Option 1: Manual editing**
Open `.sentinelrc.toml` with your preferred editor:
```bash
code .sentinelrc.toml
# or
vim .sentinelrc.toml
```

**Option 2: Reset configuration (command 'x')**
Press `x` in Sentinel to delete the current configuration and start over. The interactive assistant will run again on next startup.

## Migration from v3.x to v4.0.0

If you're updating from a previous version of Sentinel, note these **breaking changes**:

### Breaking Changes

1. **Configuration via `.sentinelrc.toml` file**
   - ❌ **Deprecated**: Environment variables `ANTHROPIC_AUTH_TOKEN` and `ANTHROPIC_BASE_URL`
   - ✅ **New**: Configuration in `.sentinelrc.toml` with interactive assistant

   **Required action**: When running v4.0.0 for the first time, the configuration assistant will start.

2. **Multi-model system**
   - Now you can choose between Claude, Gemini, and other providers
   - Optional fallback model configuration

3. **New generated files**
   - `.sentinelrc.toml` - Project configuration
   - `.sentinel_stats.json` - Persistent metrics
   - `.sentinel/cache/` - AI response cache

### Automatic Migration

There is no automatic migration from environment variables. Simply:

```bash
# 1. Update to v4.0.0
git pull origin master
cargo build --release

# 2. Run Sentinel (the assistant will start)
./target/release/sentinel-rust

# 3. Configure your API Key when prompted
# 4. Done! Sentinel will work as before
```

### Benefits of v4.0.0

- 🔄 Multi-provider support (not just Claude)
- 💾 Smart cache (reduces costs up to 70%)
- 📊 Real-time metrics
- ⚡ Automatic fallback system
- ⚙️ More flexible and portable configuration

---

**Navigation:**
- [← Previous: Installation](installation.md)
- [Next: Commands →](commands.md)
