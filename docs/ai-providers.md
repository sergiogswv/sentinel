# AI Providers

Sentinel can work with multiple AI providers. Choose the one that best suits your needs.

## Supported AI Providers

### Anthropic Claude (Recommended)

**Available models:**
- `claude-opus-4-5-20251101` - Most powerful, deep analysis
- `claude-sonnet-4-20250514` - Balanced, good quality/cost ratio
- `claude-haiku-3-5-20241022` - Fast and economical

**Configuration:**
- URL: `https://api.anthropic.com`
- Get your API Key at: https://console.anthropic.com

**Example configuration:**
```toml
[primary_model]
name = "claude-opus-4-5-20251101"
url = "https://api.anthropic.com"
api_key = "sk-ant-api03-..."
```

**Best for:**
- Deep code analysis
- Complex architectural reviews
- Detailed debugging assistance
- High-quality documentation generation

---

### Google Gemini

**Available models:**
- `gemini-2.0-flash` - Fast and efficient
- `gemini-1.5-pro` - Deep analysis
- `gemini-1.5-flash` - Economical

**Configuration:**
- URL: `https://generativelanguage.googleapis.com`
- Get your API Key at: https://makersuite.google.com/app/apikey

**Example configuration:**
```toml
[primary_model]
name = "gemini-2.0-flash"
url = "https://generativelanguage.googleapis.com"
api_key = "AIza..."
```

**Best for:**
- Fast responses
- Cost-effective analysis
- Quick code reviews
- Lightweight tasks

**Note:** Sentinel can automatically list available Gemini models during configuration.

---

## Fallback System

You can configure a backup model that will activate automatically if the primary model fails:

```
Primary Model: Claude Opus (deep analysis)
      ↓ (if fails)
Fallback Model: Gemini Flash (fast response)
```

This ensures high availability and reduces workflow interruptions.

### Configuring Fallback

**During initial setup:**
```
👉 ¿Configurar un modelo de respaldo por si falla el principal? (s/n): s
👉 API Key: [tu-api-key]
👉 URL del modelo: [url-del-proveedor]
👉 Nombre del modelo: [nombre-del-modelo]
```

**In .sentinelrc.toml:**
```toml
[primary_model]
name = "claude-opus-4-5-20251101"
url = "https://api.anthropic.com"
api_key = "sk-ant-api03-..."

[fallback_model]
name = "gemini-2.0-flash"
url = "https://generativelanguage.googleapis.com"
api_key = "AIza..."
```

### Fallback in Action

```
🔔 CAMBIO EN: auth.service.ts

   ⚠️  Modelo principal falló: Connection timeout. Intentando fallback con gemini-2.0-flash...

✨ CONSEJO DE CLAUDE:
SEGURO - La implementación de autenticación JWT es correcta.
[... Código guardado en .suggested ...]

   ✅ Arquitectura aprobada.
```

---

## Model Selection Guidelines

### For Production/Critical Projects
- **Primary**: Claude Opus (highest quality)
- **Fallback**: Claude Sonnet or Gemini Pro

### For Development/Testing
- **Primary**: Claude Sonnet (balanced)
- **Fallback**: Gemini Flash (fast and cheap)

### For Cost Optimization
- **Primary**: Gemini Flash (economical)
- **Fallback**: Gemini Flash (same model, different region)

---

## Automatic Model Listing

Sentinel can automatically list available models during configuration (currently supported for Gemini):

```
Fetching available models from Gemini...

Available models:
  1. gemini-2.0-flash
  2. gemini-1.5-pro
  3. gemini-1.5-flash

Select model number: 1
```

---

## API Key Management

### Getting API Keys

**Anthropic Claude:**
1. Visit https://console.anthropic.com
2. Sign up or log in
3. Navigate to API Keys section
4. Create a new API key
5. Copy the key (starts with `sk-ant-api03-`)

**Google Gemini:**
1. Visit https://makersuite.google.com/app/apikey
2. Sign in with Google account
3. Create API key
4. Copy the key (starts with `AIza`)

### Security Best Practices

See the [Security Guide](security.md) for detailed information on:
- API key protection
- .gitignore configuration
- Safe project sharing
- Cache security

---

## Future Providers (Roadmap)

Planned support for additional providers:
- OpenAI (GPT-4, GPT-3.5)
- Mistral AI
- Local models (Ollama, LM Studio)
- Dynamic model selection based on task type

---

**Navigation:**
- [← Previous: Commands](commands.md)
- [Next: Security →](security.md)
