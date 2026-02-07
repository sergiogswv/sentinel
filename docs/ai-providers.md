# AI Providers

Sentinel can work with multiple AI providers. Choose the one that best suits your needs.

## Supported AI Providers

### Anthropic Claude (Recommended)

**Available models:**
- `claude-3-5-sonnet-20241022` - Most powerful, deep analysis
- `claude-3-opus-20240229` - Balanced, good quality/cost ratio
- `claude-3-haiku-20240307` - Fast and economical

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

### OpenAI & Compatible (Groq, Ollama, Kimi, DeepSeek)

Sentinel now supports any OpenAI-compatible API. This includes:
- **Groq**: Extremely fast inference (Llama 3, Mixtral)
- **Ollama**: Local models for privacy and 0 cost
- **Kimi / DeepSeek**: Efficient models with specialized capabilities

**Note:** Sentinel automatically fetches available models from these providers during configuration.

---

## Cascading Fallback System

Unlike previous versions, Sentinel now supports **cascading fallback**. You can configure a list of $N$ providers, and Sentinel will try them sequentially:

```
1. Claude 3.5 Sonnet (Primary)
      ↓ (if fails)
2. Groq Llama 3 (Fast Fallback)
      ↓ (if fails)
3. Gemini Flash (Reliable Fallback)
      ↓ (if fails)
4. Ollama Local (Last Resort)
```

This ensures maximum availability and resilience.

### Configuring Providers

**During initial setup (`sentinel init`):**
Sentinel will guide you through adding each provider interactively.
1. Choose provider type
2. Enter API URL (defaults are provided)
3. Enter API Key
4. Select model (Sentinel will try to fetch the list for you)
5. Choose if you want to add more providers

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

Sentinel automatically lists available models for almost all supported providers:

```
🤖 CONFIGURACIÓN DE LA IA (#1)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
? Selecciona el proveedor: Groq
? URL de la API: https://api.groq.com/openai
? API Key: gsk_...

🔍 Obteniendo modelos disponibles de Groq...

? Selecciona el modelo:
  1. llama-3.1-70b-versatile
  2. llama-3.1-8b-instant
> 3. mixtape-8x7b-32768
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
