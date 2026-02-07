# Installation Guide

This guide covers the requirements, installation steps, initial configuration, and expected project structure for Sentinel.

## Requirements

- **Git** 2.0+ (required for auto-updates)
- **Rust** 1.70+ (automatically installed if missing)
- API Key from at least one AI provider (Claude or Gemini recommended)
- **Internet connection** (for initial installation and updates)

## Installation Methods

### 🚀 Method 1: Automatic Installation (Recommended)

The automatic installers will:
- ✅ Check and install dependencies (Git, Rust)
- ✅ Clone or update the repository
- ✅ Compile and install the binary
- ✅ Add Sentinel to your PATH
- ✅ Create auto-update scripts

#### Linux/macOS

```bash
# Quick install (one-liner)
curl -sSL https://raw.githubusercontent.com/sergiogswv/sentinel-rust/master/install.sh | bash

# Or download and run
wget https://raw.githubusercontent.com/sergiogswv/sentinel-rust/master/install.sh
chmod +x install.sh
./install.sh
```

#### Windows

**PowerShell (Recommended):**
```powershell
# One-liner (run as Administrator or normal user)
irm https://raw.githubusercontent.com/sergiogswv/sentinel-rust/master/install.ps1 | iex

# Or download and run
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/sergiogswv/sentinel-rust/master/install.ps1" -OutFile "install.ps1"
Set-ExecutionPolicy Bypass -Scope Process -Force
.\install.ps1
```

**CMD/Batch:**
```cmd
# Download install.bat from the repository
# Then double-click it or run:
install.bat
```

After installation:
- **Linux/macOS**: Binary installed at `/usr/local/bin/sentinel`
- **Windows**: Binary installed at `%USERPROFILE%\.local\bin\sentinel.exe`

### 📦 Method 2: Manual Installation

If you prefer manual control:

#### 1. Clone the repository

```bash
git clone https://github.com/sergiogswv/sentinel-rust.git
cd sentinel-rust
```

#### 2. Use the installation script

```bash
# Linux/macOS
chmod +x install.sh
./install.sh

# Windows PowerShell
.\install.ps1

# Windows CMD
install.bat
```

#### 3. Or compile manually

```bash
cargo build --release
```

The compiled binary will be at:
- **Linux/macOS**: `target/release/sentinel-rust`
- **Windows**: `target\release\sentinel-rust.exe`

To install manually:

**Linux/macOS:**
```bash
sudo cp target/release/sentinel-rust /usr/local/bin/sentinel
sudo chmod +x /usr/local/bin/sentinel
```

**Windows:**
```powershell
# Create directory
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.local\bin"

# Copy binary
Copy-Item target\release\sentinel-rust.exe "$env:USERPROFILE\.local\bin\sentinel.exe"

# Add to PATH (add this directory to your system PATH)
```

## Updating Sentinel

### 🔄 Auto-Update

The installers create convenient update commands:

```bash
# All platforms (after using auto-installer)
sentinel-update
```

Or run the update scripts directly:
- **Linux/macOS**: `~/.sentinel/update.sh`
- **Windows**: `%USERPROFILE%\.sentinel\update.ps1` or `update.bat`

### 🔄 Manual Update

```bash
cd ~/.sentinel  # Linux/macOS
# cd %USERPROFILE%\.sentinel  # Windows

git pull origin master
cargo build --release

# Then copy the binary as shown in manual installation
```

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
