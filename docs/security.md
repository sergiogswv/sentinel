# Security and Best Practices

This guide covers security measures to protect your API keys and sensitive project data.

## API Key Protection

Sentinel takes automatic measures to protect your credentials:

### 1. Auto-gitignore (v4.1.0+)

When creating the configuration, Sentinel automatically adds these files to `.gitignore`:

```gitignore
# Sentinel - Configuration and cache files (contain API keys)
.sentinelrc.toml
.sentinel_stats.json
.sentinel/
```

This prevents accidental exposure of credentials in public repositories.

### 2. Manual Verification

Always verify that `.gitignore` includes these files before pushing:

```bash
git status  # You should NOT see .sentinelrc.toml in the list
```

### 3. If You Already Committed Credentials by Mistake

If you accidentally pushed your API keys:

```bash
# 1. Immediately rotate your API Key in the provider's dashboard
# 2. Update .sentinelrc.toml with the new key
# 3. Add to .gitignore if not already there
# 4. Remove from git history (optional, advanced)
```

**Rotating API Keys:**

**Anthropic:**
1. Visit https://console.anthropic.com
2. Go to API Keys section
3. Delete the compromised key
4. Create a new key
5. Update `.sentinelrc.toml` with new key

**Gemini:**
1. Visit https://makersuite.google.com/app/apikey
2. Delete the old key
3. Create a new key
4. Update `.sentinelrc.toml` with new key

---

## Sharing Projects

If you want to share your Sentinel configuration without exposing your API Key:

### Create an Example Configuration

```bash
# Create an example file
cp .sentinelrc.toml .sentinelrc.example.toml

# Edit the example and replace the API Key
# api_key = "YOUR_API_KEY_HERE"

# Add the example to version control
git add .sentinelrc.example.toml
git commit -m "docs: add Sentinel configuration example"
```

### Example Configuration Template

```toml
[project]
project_name = "my-project"
framework = "NestJS"
manager = "npm"
test_command = "npm run test"
use_cache = true

[primary_model]
name = "claude-opus-4-5-20251101"
url = "https://api.anthropic.com"
api_key = "YOUR_ANTHROPIC_API_KEY_HERE"

# Optional: Configure fallback model
# [fallback_model]
# name = "gemini-2.0-flash"
# url = "https://generativelanguage.googleapis.com"
# api_key = "YOUR_GEMINI_API_KEY_HERE"

[[architecture_rules]]
"SOLID Principles"
"Clean Code"
"NestJS Best Practices"
```

---

## Cache Security

The cache can contain code fragments, so consider cleaning it before sharing the project:

### Using Sentinel Command (Recommended)

```bash
# Press 'l' in Sentinel
l → s
```

### Manual Cleanup

```bash
# Remove cache directory
rm -rf .sentinel/

# The cache will regenerate automatically when needed
```

### View Cache Size

```bash
# On Unix/Linux/macOS
du -sh .sentinel/cache

# On Windows
dir .sentinel\cache
```

---

## .gitignore Best Practices

### Verify Protected Files

Ensure your `.gitignore` includes:

```gitignore
# Sentinel configuration (contains API keys)
.sentinelrc.toml
.sentinel_stats.json
.sentinel/

# Environment variables (if used)
.env
.env.local
.env.*.local

# IDE specific files
.vscode/
.idea/
*.swp
*.swo
```

### Check Before Committing

```bash
# Always check what will be committed
git status

# If you see sensitive files, add them to .gitignore
echo ".sentinelrc.toml" >> .gitignore
git add .gitignore
git commit -m "chore: protect sensitive files"
```

---

## Sharing Projects with Teams

### For Team Members

**Option 1: Each member uses their own API key**
1. Share `.sentinelrc.example.toml`
2. Each team member copies it to `.sentinelrc.toml`
3. Each member adds their own API key

**Option 2: Use a shared team API key**
1. Create a dedicated API key for the team
2. Share it securely (using a password manager or secrets management tool)
3. Monitor usage in the provider's dashboard

### For Open Source Projects

If your project is open source:
1. **Never commit** `.sentinelrc.toml`
2. Provide `.sentinelrc.example.toml` with placeholders
3. Document the configuration process in README
4. Recommend free-tier API keys for contributors

---

## Environment-Specific Configuration

If you work with multiple environments:

```bash
# Development
.sentinelrc.toml          # Local development (gitignored)

# Production/CI
.sentinelrc.prod.toml     # Production config (use secrets manager)

# Team shared
.sentinelrc.example.toml  # Template for team (committed)
```

---

## Security Checklist

Before pushing to a repository:

- [ ] `.sentinelrc.toml` is in `.gitignore`
- [ ] `.sentinel_stats.json` is in `.gitignore`
- [ ] `.sentinel/` directory is in `.gitignore`
- [ ] Run `git status` to verify no sensitive files are staged
- [ ] Consider clearing cache if project contains sensitive code
- [ ] API keys are not hardcoded anywhere in the codebase
- [ ] Example configuration file uses placeholder values

---

## Incident Response

If you accidentally expose credentials:

### Immediate Actions (First 5 minutes)
1. **Rotate the API key immediately** in provider dashboard
2. Update local `.sentinelrc.toml` with new key
3. Add to `.gitignore` if missing

### Follow-up Actions (Next hour)
4. Check provider dashboard for unauthorized usage
5. Review recent API calls for suspicious activity
6. If repository is public, consider making it private temporarily
7. Document the incident for team awareness

### Long-term Actions
8. Implement secrets scanning in CI/CD
9. Use pre-commit hooks to prevent future leaks
10. Consider using a secrets management service

---

**Navigation:**
- [← Previous: AI Providers](ai-providers.md)
- [Next: Troubleshooting →](troubleshooting.md)
