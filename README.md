# Sentinel 🛡️

<p align="center">
  <img src="./public/sentinel.jpg" alt="Sentinel Logo" width="100%"/>
</p>

<p align="center">
  <strong>Elite Productivity Assistant: Multi-Model AI Orchestrator for Architecture Auditing, Autonomous Testing, and Development Observability.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-4.4.2-blue.svg" alt="Version">
  <img src="https://img.shields.io/badge/rust-2024-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License">
</p>

---

## 🚀 What is Sentinel?

Real-time monitoring tool written in **Rust** that analyzes code changes using **multiple AI providers** (Claude, Gemini, etc.) and manages workflow with Git. Designed for **NestJS/TypeScript** projects as an intelligent development assistant.

### ✨ Key Features

- 🤖 **Automatic AI Analysis** - Multi-model support (Claude, Gemini) with fallback
- 💾 **Smart Caching** - Reduces API costs up to 70%
- 📊 **Real-time Metrics** - Tracking bugs, costs, tokens, and productivity
- 🧪 **Automatic Tests** - Runs Jest with real-time output
- 🎯 **Parent File Detection** - Automatically finds parent modules for comprehensive testing
- 📚 **Auto-documentation** - Generates technical manuals automatically
- 📈 **Daily Reports** - Intelligent commit summaries
- 🔒 **Security** - Automatic API key protection in `.gitignore`
- ⚙️ **Flexible Configuration** - Per-project, no environment variables

---

## 📦 Quick Start

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- API Key from [Claude](https://console.anthropic.com) or [Gemini](https://makersuite.google.com/app/apikey)
- NestJS project with Jest configured

### Installation

```bash
# 1. Clone the repository
git clone https://github.com/your-username/sentinel-rust.git
cd sentinel-rust

# 2. Build
cargo build --release

# 3. Run
./target/release/sentinel-rust
```

On first use, Sentinel will start an **interactive configuration wizard**.

---

## 🎮 Available Commands

Once started, Sentinel responds to these commands:

| Command | Action |
|---------|--------|
| `p` | Pause/Resume monitoring |
| `r` | Generate daily report |
| `m` | View metrics (bugs, costs, tokens) |
| `l` | Clear cache |
| `h` | Show help |
| `x` | Reset configuration |

💡 **Tip:** On startup, Sentinel automatically displays the command list.

---

## 📖 Complete Documentation

### 📚 User Guides

- **[Installation and Setup](docs/installation.md)** - Complete installation guide
- **[Advanced Configuration](docs/configuration.md)** - `.sentinelrc.toml` in detail
- **[Commands and Usage](docs/commands.md)** - Complete guide to all commands
- **[AI Providers](docs/ai-providers.md)** - Claude, Gemini, and more
- **[Usage Examples](docs/examples.md)** - Real-world use cases

### 🔧 Technical References

- **[Architecture](docs/architecture.md)** - System components and flow
- **[Security](docs/security.md)** - API key protection and best practices
- **[Troubleshooting](docs/troubleshooting.md)** - Common problems and solutions

### 📋 Project

- **[Roadmap](docs/roadmap.md)** - Planned features
- **[Changelog](CHANGELOG.md)** - Change history

---

## 🎯 Quick Example

```bash
# Sentinel detects a change in a child file
🔔 CAMBIO EN: call-inbound.ts
   ℹ️  Archivo hijo detectado, usando tests del módulo: call

✨ CONSEJO DE CLAUDE:
SEGURO - El código sigue correctamente el patrón Repository.

   ✅ Arquitectura aprobada.
🧪 Ejecutando tests: test/calls/calls.spec.ts

 PASS  test/calls/calls.spec.ts
  ✓ should create user (12 ms)
  ✓ should find user by id (8 ms)

   ✅ Tests pasados con éxito

📚 Actualizando manual de bolsillo...
   ✅ Documento generado: src/calls/call.service.md

🚀 Mensaje: feat: add user validation in create method
📝 ¿Commit? (s/n): s
   ✅ Commit exitoso!
```

---

## 🔒 Security

Sentinel automatically protects your API keys:
- ✅ Adds sensitive files to `.gitignore`
- ✅ Per-project configuration (no global variables)
- ✅ Local cache without sharing credentials

[Read more about security →](docs/security.md)

---

## 🌟 Highlighted Features

### Parent File Detection
When you modify a child file (e.g., `call-inbound.ts`), Sentinel automatically detects the parent module (`call.service.ts`) and runs the complete module tests for better coverage.

### Multi-Model System
Use Claude for deep analysis and Gemini as fast fallback. Switch providers without restarting.

### Smart Cache
Reduce costs up to 70% by reusing responses for similar code.

### Productivity Metrics
Automatic tracking of bugs prevented, time saved, and API costs.

[See all features →](docs/configuration.md)

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the project
2. Create a branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -am 'feat: add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Open a Pull Request

---

## 📊 Project Status

```
Phase 1: ✅ Completed - Monitoring and basic analysis
Phase 2: ✅ Completed - Productivity and documentation
Phase 3: ✅ Completed - Optimization and stability
Phase 4: ✅ Completed - Multi-model AI & Parent file detection (v4.2.0)
Phase 5: 🚧 Planned - Multi-platform support (frameworks & languages)
  → Sub-phase: 🌐 New AI Models (OpenAI, Mistral, Local models)
Phase 6: 📅 Planned - Security (SecOps)
Phase 7: 📅 Planned - PR Review Automation (Elite)
Phase 8: 📅 Planned - Enterprise & scalability
```

[View complete roadmap →](docs/roadmap.md)

---

## 📝 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## 👤 Author

**Sergio Guadarrama**

---

<p align="center">
  Made with ❤️ using Rust and Claude AI
</p>

<p align="center">
  <a href="docs/installation.md">Installation</a> •
  <a href="docs/configuration.md">Configuration</a> •
  <a href="docs/commands.md">Commands</a> •
  <a href="docs/troubleshooting.md">Troubleshooting</a>
</p>
