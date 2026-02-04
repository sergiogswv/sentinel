# Troubleshooting Guide

Common issues and their solutions when using Sentinel.

## Installation and Setup Issues

### Error: "Input watch path is neither a file nor a directory"

**Cause:**
- The selected project **does not have** a `src/` directory
- The project path doesn't exist or is invalid

**Solution:**

1. Ensure the project has a `src/` folder:
   ```bash
   mkdir src
   ```

2. Or select a different project that already has this structure

Sentinel now automatically validates the existence of the `src/` directory and shows descriptive error messages.

---

## Configuration Issues

### Error: Configuration or Invalid API Key

If Sentinel cannot connect to the API:

**Step 1: Verify Configuration**

```bash
# Open .sentinelrc.toml manually
code .sentinelrc.toml
# or
vim .sentinelrc.toml
```

**Step 2: Verify API Key**

- Ensure the API Key is valid and has permissions
- For Anthropic: should start with `sk-ant-api03-`
- For Gemini: should be a valid Google Cloud key

**Step 3: Verify Provider URL**

- Anthropic: `https://api.anthropic.com`
- Gemini: `https://generativelanguage.googleapis.com`

**Step 4: Reset Configuration (command 'x')**

```
Press 'x' in Sentinel and confirm to reconfigure from scratch
```

---

### Error: "Cannot connect to API"

**Verify internet connection and correct base URL:**

```bash
# For Anthropic
curl -I https://api.anthropic.com

# For Gemini
curl -I https://generativelanguage.googleapis.com
```

**Possible causes:**
- No internet connection
- Firewall blocking requests
- VPN interfering with connections
- Invalid provider URL in configuration

**Solutions:**
- Check your internet connection
- Disable VPN temporarily to test
- Verify firewall settings
- Ensure URL in `.sentinelrc.toml` is correct

---

### Fallback Model Not Activating

**Verify fallback configuration:**

```bash
# Check if [fallback_model] section exists in .sentinelrc.toml
cat .sentinelrc.toml | grep -A 5 "fallback_model"
```

**Important notes:**
- The fallback only activates if the primary model **completely fails**
- If primary model returns an error response, fallback may not trigger
- Verify that the fallback API key is valid

**Test fallback manually:**
- Temporarily use an invalid API key for primary model
- Sentinel should automatically switch to fallback

---

## Monitoring Issues

### Sentinel Doesn't Detect Changes

**Possible causes and solutions:**

1. **Wrong directory**: Verify you're modifying `.ts` files in the `src/` directory
   - `.spec.ts` and `.suggested` files are intentionally ignored

2. **Monitoring paused**: Check if watcher is active
   ```bash
   # Check for pause file
   ls -la .sentinel-pause

   # If exists, remove it
   rm .sentinel-pause
   ```
   Or press `p` in Sentinel to resume

3. **Debounce window**: The debounce ignores events from the same file within 15 seconds
   - Wait before saving again
   - This is intentional to avoid duplicate processing

4. **File system issues**:
   - Editor not triggering file system events properly
   - Try saving with a different editor
   - Verify file permissions

---

## Test Execution Issues

### Tests Don't Run

**Verify test file exists:**

```bash
# For each src/module/file.ts, there should be test/module/file.spec.ts
# Example:
ls test/users/users.spec.ts
```

**Verify Jest is configured:**

```bash
# Test that npm run test works
npm run test

# Check package.json for test script
cat package.json | grep "test"
```

**Common issues:**
- Test file doesn't exist in expected location
- Jest not installed: `npm install --save-dev jest @types/jest`
- Test command incorrect in `.sentinelrc.toml`

**Solution:**
```bash
# Install Jest if missing
npm install --save-dev jest @types/jest ts-jest

# Verify test script in package.json
{
  "scripts": {
    "test": "jest"
  }
}
```

---

## Git and Commit Issues

### Commits Not Created

**Verify git is initialized:**

```bash
# Check if git repository exists
git status

# If not, initialize git
git init
```

**Check permissions:**

```bash
# Verify write permissions
ls -la .git/
```

**Check for git hooks:**

```bash
# List hooks that might be blocking commits
ls -la .git/hooks/

# Temporarily disable hooks to test
mv .git/hooks/pre-commit .git/hooks/pre-commit.disabled
```

**Common issues:**
- Not in a git repository
- No write permissions
- Git hooks failing
- Git user not configured

**Solutions:**

```bash
# Configure git user if needed
git config user.name "Your Name"
git config user.email "your.email@example.com"

# Or configure globally
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

---

## Report Generation Issues

### Daily Report Not Generated (command 'r')

**Verify commits exist today:**

```bash
# Check commits since midnight
git log --since="00:00:00" --oneline

# If no output, you haven't made commits today
```

**Verify git is installed:**

```bash
git --version
```

**Verify in git repository:**

```bash
git status
```

**Common issues:**
- No commits made today (since 00:00:00)
- Git not installed
- Not in a git repository
- Invalid API key for AI provider

**Solution:**
- Make at least one commit during the day
- Ensure you're in a git repository
- Verify API configuration is correct

---

## Performance Issues

### Slow Analysis

**Possible causes:**
1. **No cache**: Verify cache is enabled
   ```toml
   [project]
   use_cache = true
   ```

2. **Large files**: Very large files take longer to analyze
   - Consider splitting large files into smaller modules

3. **Network latency**: Slow connection to AI provider
   - Check internet speed
   - Try switching to a faster model (e.g., Gemini Flash)

**Solutions:**
- Enable cache in `.sentinelrc.toml`
- Use a faster model for non-critical analysis
- Ensure stable internet connection

---

### High API Costs

**Cost reduction strategies:**

1. **Enable cache** (if not already):
   ```toml
   use_cache = true
   ```

2. **Use economical models**:
   - Primary: Gemini Flash or Claude Haiku
   - Reserve expensive models (Opus, Pro) for critical reviews

3. **Monitor usage**:
   ```bash
   # Check metrics
   # Press 'm' in Sentinel
   ```

4. **Pause during breaks**:
   ```bash
   # Press 'p' when not actively developing
   ```

---

## Cache Issues

### Cache Not Working

**Verify cache is enabled:**

```bash
# Check .sentinelrc.toml
cat .sentinelrc.toml | grep use_cache
# Should show: use_cache = true
```

**Verify cache directory exists:**

```bash
# Check cache directory
ls -la .sentinel/cache/

# If missing, Sentinel will create it automatically
```

**Clear and regenerate cache:**

```bash
# Press 'l' in Sentinel to clear cache
# It will regenerate automatically
```

---

### Cache Taking Too Much Space

**Check cache size:**

```bash
# Unix/Linux/macOS
du -sh .sentinel/cache/

# Windows
dir .sentinel\cache
```

**Clear cache:**

```bash
# Using Sentinel (recommended)
# Press 'l' and confirm

# Or manually
rm -rf .sentinel/cache
```

---

## Editor-Specific Issues

### VS Code Not Triggering Changes

**Issue:** VS Code's auto-save might not trigger file system events properly.

**Solution:**
```json
// In VS Code settings.json
{
  "files.autoSave": "afterDelay",
  "files.autoSaveDelay": 1000
}
```

---

### IntelliJ/WebStorm Not Triggering Changes

**Issue:** JetBrains IDEs use safe write by default.

**Solution:**
1. Go to Settings > Appearance & Behavior > System Settings
2. Uncheck "Use safe write"
3. Restart IDE

---

## Getting Help

If you encounter an issue not covered here:

1. **Check logs**: Look at Sentinel's console output for error messages
2. **Verify configuration**: Ensure `.sentinelrc.toml` is valid
3. **Test components individually**:
   - Test API connection manually with `curl`
   - Test Jest separately with `npm run test`
   - Test git separately with `git status`
4. **Reset configuration**: Use command 'x' to start fresh
5. **Report issue**: Open an issue on GitHub with:
   - Sentinel version
   - Operating system
   - Error messages
   - Steps to reproduce

---

**Navigation:**
- [← Previous: Security](security.md)
- [Next: Architecture →](architecture.md)
