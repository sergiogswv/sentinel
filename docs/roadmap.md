# Roadmap

Sentinel's development roadmap with completed features and future plans.

## Fase 1: Fundamentos (Completada ✅)

**Enfoque:** Monitoreo básico y análisis de código

- [x] Monitoreo en tiempo real con file watcher (notify)
- [x] Análisis de arquitectura con Claude AI (SOLID, Clean Code)
- [x] Ejecución automática de tests con Jest
- [x] Generación de mensajes de commit inteligentes
- [x] Flujo interactivo de commits con Git

**Release:** v1.0.0 - Initial Release

**Key Achievements:**
- Core file monitoring functionality
- Basic AI integration
- Automated testing workflow
- Git commit automation

---

## Fase 2: Productividad y Documentación (Completada ✅)

**Enfoque:** Automatización de tareas repetitivas

- [x] Auto-documentación de archivos (.md generados automáticamente) - v3.1
- [x] Reportes diarios de productividad - v3.2
- [x] Sugerencias de código en archivos `.suggested` - v3.3
- [x] Diagnóstico automático de fallos en tests - v3.3

**Releases:**
- v3.1 - Auto-documentation
- v3.2 - Daily reports
- v3.3 - Enhanced suggestions

**Key Achievements:**
- Automated documentation generation
- Productivity tracking and reporting
- Improved code suggestions
- Intelligent test failure diagnosis

---

## Fase 3: Optimización y Estabilidad (Completada ✅)

**Enfoque:** Mejoras de rendimiento y UX

- [x] Stdin centralizado sin conflictos entre hilos - v3.3
- [x] Tests de Jest visibles en consola en tiempo real - v3.3
- [x] Debounce y drenado de eventos duplicados del watcher - v3.3
- [x] Validación de estructura de proyecto (directorio `src/`) - v3.3.1
- [x] Manejo robusto de errores con mensajes descriptivos - v3.3.1
- [x] Configuración personalizable mediante archivo `.sentinelrc.toml` - v3.3
- [x] Sistema de estadísticas y métricas de productividad - v3.3

**Releases:**
- v3.3 - Performance improvements
- v3.3.1 - Stability enhancements

**Key Achievements:**
- Thread-safe stdin handling
- Real-time test output
- Duplicate event filtering
- Project structure validation
- Better error handling
- Flexible configuration system

---

## Fase 4: API Keys y Modelos de IA + Expansión Multiplataforma 🌐🤖 (Completada ✅)

**Enfoque:** Flexibilidad en modelos de IA y compatibilidad con más tecnologías

**🎉 LANZAMIENTO v4.0.0 - Cambios Mayores (Breaking Changes)**

### Gestión de API Keys y Modelos - v4.0.0

- [x] Soporte para múltiples proveedores de IA:
  - [x] Anthropic Claude (Sonnet, Opus, Haiku)
  - [x] Google Gemini (2.0 Flash, Pro, etc.)
  - [x] Estructura extensible para agregar más proveedores
- [x] Configuración flexible por archivo `.sentinelrc.toml` (reemplaza variables de entorno)
- [x] Sistema de fallback automático entre modelos
- [x] Caché de respuestas para reducir costos de API
- [x] Estimación y tracking de costos por proveedor
- [x] Dashboard de métricas en tiempo real (comando 'm')
- [x] Listado automático de modelos disponibles (Gemini)
- [x] Asistente interactivo de configuración inicial

**Additional Updates:**
- v4.1.0 - Security & Cache Management
  - [x] Auto-gitignore for sensitive files
  - [x] Cache clearing command ('l')
  - [x] Enhanced security features

- v4.1.1 - Interactive Help
  - [x] Automatic help display on startup
  - [x] Help command ('h' or 'help')
  - [x] Version display in startup message

**Key Achievements:**
- Multi-provider AI support
- Intelligent caching (70% cost reduction)
- Real-time metrics dashboard
- Automatic failover system
- Secure credential management
- Interactive configuration wizard

### Future Providers (Planned)

- [ ] OpenAI (GPT-4, GPT-3.5) - Próxima iteración
- [ ] Mistral AI - Próxima iteración
- [ ] Modelos locales (Ollama, LM Studio) - Próxima iteración
- [ ] Selección dinámica de modelo según tarea - Próxima iteración

---

## Fase 5: Expansión Multiplataforma (Planificada 🚧)

**Enfoque:** Soporte para más frameworks y lenguajes

### Soporte para Otros Frameworks JavaScript

- [ ] **Angular**
  - Standalone components
  - Signals API
  - Angular Testing Library
- [ ] **React**
  - Hooks patterns
  - Context API
  - React Testing Library
- [ ] **Vue 3**
  - Composition API
  - Script setup syntax
  - Vitest integration
- [ ] **SolidJS**
  - Reactive primitives
  - Fine-grained reactivity
- [ ] **Svelte**
  - Compiler-based approach
  - Svelte Testing Library

### Integración con Otros Test Runners

- [ ] **Vitest**
  - Vite-native testing
  - Fast test execution
  - Component testing
- [ ] **Mocha + Chai**
  - BDD/TDD styles
  - Assertion libraries
- [ ] **Playwright**
  - E2E testing
  - Cross-browser support
- [ ] **Cypress**
  - E2E testing
  - Visual testing

### Soporte para Otros Lenguajes

- [ ] **Python**
  - pytest integration
  - Django best practices
  - FastAPI patterns
- [ ] **Go**
  - testing package
  - Go idioms
  - Table-driven tests
- [ ] **Java**
  - JUnit integration
  - Spring Boot patterns
  - Maven/Gradle support

**Target Release:** v5.0.0

---

## Fase 6: El Guardián de Seguridad (SecOps) 🔒

**Enfoque:** Prevención de riesgos y blindaje de código

### Módulo de Escaneo de Secretos

- [ ] Motor basado en Regex y entropía para detectar:
  - Llaves de API (AWS, Google Cloud, Azure)
  - Tokens JWT en texto plano
  - Contraseñas hardcodeadas
  - Credenciales de bases de datos
- [ ] Bloqueo automático de commits con secretos detectados
- [ ] Sugerencias de variables de entorno (.env)

### Auditoría de Dependencias

- [ ] Lectura y análisis de `package-lock.json` / `Cargo.lock`
- [ ] Consulta de bases de datos de vulnerabilidades (CVEs)
- [ ] Integración con GitHub Security Advisories
- [ ] Alertas de dependencias obsoletas o inseguras
- [ ] Reporte de score de seguridad del proyecto

### Sanitización Automática

- [ ] Prompt especializado para NestJS:
  - Detección de DTOs sin decoradores de validación (`class-validator`)
  - Validación de uso correcto de `ValidationPipe`
  - Prevención de inyección SQL en queries de TypeORM
  - Validación de sanitización de inputs en endpoints
- [ ] Sugerencias automáticas de validadores faltantes
- [ ] Análisis de vectores de ataque comunes (OWASP Top 10)

**Target Release:** v6.0.0

**Benefits:**
- Prevent security breaches before they happen
- Automated vulnerability scanning
- Compliance with security standards
- Proactive threat detection

---

## Fase 7: El Revisor de Élite (PR Mode) 🔍

**Enfoque:** Colaboración y calidad colectiva

### Integración con GitHub API

- [ ] Autenticación con tokens personales o GitHub Apps
- [ ] Descarga automática de archivos de Pull Requests
- [ ] Lectura de comentarios y conversaciones existentes
- [ ] Capacidad de publicar revisiones directamente en GitHub

### Análisis de Diff/Cambios

- [ ] Parser de diffs de Git para extraer solo líneas modificadas
- [ ] Análisis contextual: la IA evalúa el cambio en relación al código existente
- [ ] Detección de breaking changes (APIs modificadas, firmas de funciones)
- [ ] Validación de que los cambios no rompen la lógica existente
- [ ] Análisis de impacto en otras partes del código

### Reporte de Revisión

- [ ] Generación de resumen ejecutivo estructurado:
  - ✅ **Aprobación:** "Este PR es seguro para mergear"
  - ⚠️ **Advertencias:** "El servicio de facturación perdió cobertura de tests"
  - ❌ **Bloqueos:** "Detectada vulnerabilidad de inyección SQL"
- [ ] Comentarios en línea sobre código específico
- [ ] Checklist automático de calidad (tests, docs, seguridad)
- [ ] Sugerencias de mejoras opcionales vs. cambios obligatorios
- [ ] Integración con sistemas de CI/CD para bloquear merges inseguros

**Target Release:** v7.0.0

**Benefits:**
- Automated code review
- Consistent review quality
- Faster PR turnaround
- Reduced reviewer workload
- Improved code quality standards

---

## Fase 8: Empresa y Escalabilidad 🚀

**Enfoque:** Herramientas para equipos y organizaciones

### Core Features

- [ ] Modo daemon/servicio en segundo plano
- [ ] Soporte para múltiples proyectos simultáneos
- [ ] Dashboard web para visualización de métricas del equipo
- [ ] Integración con webhooks (Slack, Discord, Microsoft Teams)
- [ ] Métricas y reportes semanales/mensuales

### Team Collaboration

- [ ] Sistema de permisos y roles (líder técnico, desarrollador, revisor)
- [ ] Integración con Jira/Linear para tracking de tareas
- [ ] Notificaciones de equipo en tiempo real
- [ ] Shared configuration templates
- [ ] Team-wide code quality standards

### Enterprise Features

- [ ] API REST para integración con herramientas externas
- [ ] SSO (Single Sign-On) integration
- [ ] Audit logs and compliance reporting
- [ ] Custom AI model deployment (on-premise)
- [ ] SLA monitoring and alerting

**Target Release:** v8.0.0

**Benefits:**
- Scalable for large teams
- Centralized monitoring
- Enterprise-grade security
- Custom integrations
- Advanced analytics

---

## Future Considerations

### Community Requests

Features under consideration based on community feedback:

- Plugin system for custom analyzers
- Custom rule definitions (YAML/JSON)
- Language Server Protocol (LSP) support
- IDE extensions (VS Code, IntelliJ)
- Docker container support
- Cloud-hosted service option

### Research and Innovation

Exploring cutting-edge technologies:

- AI-powered code generation (beyond suggestions)
- Predictive bug detection (before code is written)
- Automated refactoring suggestions
- Code smell detection with ML
- Performance optimization recommendations
- Architecture pattern recognition

---

## Release Schedule (Tentative)

| Version | Target Date | Focus Area |
|---------|-------------|------------|
| v5.0.0 | Q2 2025 | Multi-platform support |
| v6.0.0 | Q3 2025 | Security features |
| v7.0.0 | Q4 2025 | PR review automation |
| v8.0.0 | Q1 2026 | Enterprise features |

> Note: Dates are estimates and subject to change based on development progress and community feedback.

---

## How to Contribute

We welcome contributions to any phase of the roadmap:

1. **Feature Requests**: Open an issue with the `enhancement` label
2. **Bug Reports**: Help us improve stability
3. **Code Contributions**: Submit PRs for features you'd like to see
4. **Documentation**: Improve guides and examples
5. **Testing**: Try beta features and provide feedback

See [Contributing Guide](../CONTRIBUTING.md) for details.

---

**Navigation:**
- [← Previous: Examples](examples.md)
- [← Back to README](../README.md)
