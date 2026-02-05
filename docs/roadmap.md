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

## Fase 4: Multi-Model AI & Intelligent Features (Completada ✅)

**Enfoque:** Flexibilidad en modelos de IA y detección avanzada

**🎉 LANZAMIENTO v4.0.0 - Cambios Mayores (Breaking Changes)**

### Gestión de API Keys y Modelos

- [x] **Soporte multi-proveedor de IA**:
  - [x] Anthropic Claude (Sonnet, Opus, Haiku)
  - [x] Google Gemini (2.0 Flash, Pro, Flash, etc.)
  - [x] Estructura extensible para agregar más proveedores
- [x] **Sistema de fallback automático** entre modelos
- [x] **Caché inteligente** de respuestas (reduce costos hasta 70%)
- [x] **Estimación y tracking** de costos por proveedor
- [x] **Dashboard de métricas** en tiempo real (comando 'm')
- [x] **Asistente interactivo** de configuración inicial
- [x] **Configuración flexible** por archivo `.sentinelrc.toml`

### Actualizaciones de Seguridad y UX (v4.1.0 - v4.1.1)

- [x] **Protección automática de API Keys** - Auto-gitignore para archivos sensibles
- [x] **Gestión de caché** - Comando 'l' para limpiar caché
- [x] **Ayuda interactiva** - Comando 'h' o 'help' siempre disponible
- [x] **Mejoras en seguridad** - Protección de credenciales

### Detección de Archivos Padres (v4.2.0) ✨

- [x] **Detección automática de módulos padres**:
  - Detecta archivos hijos (ej: `call-inbound.ts` → `call.service.ts`)
  - Ejecuta tests completos del módulo padre
  - Soporta patrones: `.service.ts`, `.controller.ts`, `.repository.ts`, `.module.ts`, `.gateway.ts`, `.guard.ts`, `.interceptor.ts`, `.pipe.ts`, `.filter.ts`
  - Sistema de prioridad inteligente (service > controller > repository > ...)
- [x] **Mejor cobertura de tests**: Los archivos hijos ejecutan tests del módulo completo
- [x] **Módulo `files.rs`**: Utilidades especializadas para detección de padres

**Current Version:** v4.2.0

**Key Achievements:**
- Multi-provider AI support with intelligent fallback
- Smart caching system (70% cost reduction)
- Real-time metrics dashboard
- Automatic failover system
- Secure credential management
- Parent file detection for comprehensive testing
- Interactive configuration wizard

---

## 🌐 Conexión a Nuevos Modelos de IA (En Progreso 🚧)

**Enfoque:** Expansión del ecosistema de IA soportado

### Próximos Proveedores (Prioridad Alta)

- [ ] **OpenAI** 🟢
  - GPT-4 Turbo / GPT-4 Vision
  - GPT-3.5 Turbo (opción económica)
  - o1 / o1-mini (razonamiento avanzado)
  - Integración con OpenAI API

- [ ] **Mistral AI** 🟡
  - Mistral 7B / Mistral Large
  - Mixtral 8x7B (mixture of experts)
  - Codestral (especializado en código)
  - Opción de modelos locales y cloud

- [ ] **Meta Llama** 🟡
  - Llama 3.1 (8B, 70B, 405B)
  - Llama 3.2 (multimodal)
  - A través de proveedores (Groq, Anyscale, Together)

- [ ] **Cohere** 🔵
  - Command R / R+
  - Especializados en RAG y herramientas
  - Soporte para citas y referencias

### Modelos Locales y Self-Hosted

- [ ] **Ollama** 💻
  - Integración con API local de Ollama
  - Soporte para Llama, Mistral, Gemma, phi
  - Sin costos de API
  - Privacidad total de código

- [ ] **LM Studio** 💻
  - Conexión a servidor local
  - Modelos GGUF variados
  - Interfaz gráfica de gestión

- [ ] **LocalAI** 💻
  - OpenAI-compatible API local
  - Soporte para múltiples modelos
  - Sin dependencias de servicios externos

- [ ] **vLLM** 🚀
  - Inferencia de alta velocidad
  - Batch processing optimizado
  - Ideal para despliegues on-premise

### Modelos Especializados en Código

- [ ] **CodeLlama** 💻
  - CodeLlama 13B/34B
  - Especializado en Python, JS, etc.
  - Completado de código

- [ ] **DeepSeek Coder** 🇨🇳
  - Modelo open-source competitivo
  - Soporte para múltiples lenguajes
  - Opción económica y potente

- [ ] **StarCoder** 🌟
  - StarCoder 2 (3B, 7B, 15B)
  - Entrenado en código abierto
  - Licencia permissiva

### Plataformas de Inferencia

- [ ] **Groq** ⚡
  - LPUs (Language Processing Units)
  - Inferencia ultra-rápida
  - Soporte para Llama, Mixtral

- [ ] **Together AI** 🤝
  - API unificada para múltiples modelos
  - Fine-tuning customizado
  - Optimización de costos

- [ ] **Anyscale** ☁️
  - Plataforma para Ray y Llama
  - Escalabilidad automática
  - Enterprise-grade

- [ ] **Fireworks AI** 🎆
  - Inferencia rápida y económica
  - Modelos optimizados
  - Sin lock-in

### Características Avanzadas

- [ ] **Selección dinámica de modelo según tarea**:
  - Archivo pequeño → Modelo rápido (Haiku, Flash)
  - Archivo grande → Modelo potente (Sonnet, GPT-4)
  - Test diagnosis → Modelo especializado
  - Commit messages → Modelo económico

- [ ] **A/B testing de modelos**:
  - Comparar calidad de respuestas
  - Métricas de satisfacción del usuario
  - Optimización automática de costos

- [ ] **Modelos multimodales**:
  - Análisis de imágenes (diagramas, screenshots)
  - Procesamiento de PDFs técnicos
  - Generación de diagramas desde código

**Target Release:** v4.3.0 - v4.5.0 (Rolling updates)

**Benefits:**
- Mayor disponibilidad (menos dependencias)
- Redundancia en caso de caídas
- Opciones económicas y premium
- Soporte para modelos locales (privacidad)
- Inferencia ultra-rápida (Groq LPUs)

---

## Fase 5: Expansión Multiplataforma (Planificada 🚧)

**Enfoque:** Soporte para más frameworks y lenguajes de programación

### Soporte para Frameworks JavaScript/TypeScript

#### Frontend Frameworks

- [ ] **React** ⚛️
  - Hooks patterns y custom hooks
  - Context API para estado global
  - React Testing Library
  - Next.js App Router
  - Patterns: Higher-Order Components, Render Props

- [ ] **Angular** 🅰️
  - Standalone components
  - Signals API (v16+)
  - Dependency Injection system
  - Angular Testing Library
  - RxJS patterns y observables

- [ ] **Vue 3** 💚
  - Composition API
  - Script setup syntax
  - Vue Testing Library
  - Pinia para state management
  - Vitest integration

- [ ] **SolidJS** 💎
  - Reactive primitives
  - Fine-grained reactivity
  - Signals-based state
  - Solid Testing Library

- [ ] **Svelte** 🧡
  - Compiler-based approach
  - Svelte 5 runes
  - Svelte Testing Library
  - Stores y derivaciones

#### Backend Frameworks

- [ ] **Express.js** 🚀
  - Middleware patterns
  - Route handlers
  - Error handling middleware
  - Testing con Supertest

- [ ] **Fastify** ⚡
  - Plugin architecture
  - Schema validation
  - High-performance patterns
  - Testing utilities

- [ ] **Koa** 🥥
  - Middleware cascading
  - Async/await patterns
  - Context-based architecture

- [ ] **Next.js API Routes** 🔄
  - Route handlers
  - Middleware en edge
  - Server actions
  - Edge runtime patterns

### Integración con Test Runners

- [ ] **Vitest** 🧪
  - Vite-native testing
  - Watch mode instantáneo
  - Component testing
  - Coverage integrado

- [ ] **Mocha + Chai** 📝
  - BDD/TDD styles
  - Assertion libraries
  - Custom reporters
  - Hook system

- [ ] **Jest** (expanding support)
  - Projects monorepo
  - Worker threads
  - Performance optimization
  - ESM support

- [ ] **Playwright** 🎭
  - E2E testing
  - Cross-browser testing
  - Visual regression
  - API testing

- [ ] **Cypress** 🌲
  - E2E testing
  - Component testing
  - Visual testing
  - Network stubbing

- [ ] **Node:test** ✅
  - Built-in Node.js test runner
  - Mock functions
  - Coverage (c8)
  - No dependencies needed

### Soporte para Otros Lenguajes

- [ ] **Python** 🐍
  - pytest integration
  - Django best practices
  - FastAPI patterns
  - Flask blueprints
  - Pydantic models
  - Type hints validation

- [ ] **Go** 🔵
  - testing package
  - Table-driven tests
  - Go idioms y patterns
  - Interfaces y composition
  - Goroutines y channels

- [ ] **Java** ☕
  - JUnit 5 integration
  - Spring Boot patterns
  - Maven/Gradle support
  - Mockito testing
  - Dependency Injection

- [ ] **C#** 🟣
  - xUnit/NUnit integration
  - ASP.NET Core patterns
  - Entity Framework
  - Dependency Injection
  - Async/await patterns

- [ ] **Ruby** ❤️
  - RSpec testing
  - Rails conventions
  - Sinatra patterns
  - Rake tasks

- [ ] **PHP** 🐘
  - PHPUnit integration
  - Laravel patterns
  - Symfony architecture
  - PSR standards

**Target Release:** v5.0.0

**Benefits:**
- Universal development assistant
- Language-agnostic architecture
- Support for modern frameworks
- Polyglot development teams

---

## Fase 6: El Guardián de Seguridad (SecOps) 🔒

**Enfoque:** Prevención de riesgos y blindaje de código

### Módulo de Escaneo de Secretos

- [ ] **Motor de detección basado en Regex y entropía**:
  - Llaves de API (AWS, Google Cloud, Azure, Stripe)
  - Tokens JWT en texto plano
  - Contraseñas hardcodeadas
  - Credenciales de bases de datos
  - Certificados y claves privadas
  - OAuth tokens

- [ ] **Bloqueo automático**:
  - Previene commits con secretos detectados
  - Sugerencias de variables de entorno (.env)
  - Redacción automática de secretos en diffs
  - Notificaciones de seguridad

- [ ] **Base de datos de patrones**:
  - Actualización continua de nuevos patrones
  - Custom regex para casos específicos
  - False positive management

### Auditoría de Dependencias

- [ ] **Análisis de vulnerabilidades**:
  - Lectura de `package-lock.json`, `Cargo.lock`, `requirements.txt`
  - Consulta de bases de datos CVEs
  - Integración con GitHub Security Advisories
  - npm audit, cargo audit, pip-audit integration

- [ ] **Alertas y recomendaciones**:
  - Dependencias obsoletas o inseguras
  - Versiones con vulnerabilidades conocidas
  - Dependencias abandonadas (no maintenance)
  - Sugerencias de alternativas seguras

- [ ] **Score de seguridad**:
  - Calificación del proyecto (A-F)
  - Métricas de deuda técnica de seguridad
  - Reportes de cumplimiento (compliance)

### Sanitización Automática

- [ ] **Análisis de seguridad en NestJS**:
  - DTOs sin decoradores de validación (`class-validator`)
  - Validación de `ValidationPipe` en uso
  - Prevención de inyección SQL en TypeORM
  - Sanitización de inputs en endpoints
  - Uso correcto de guards y throttling

- [ ] **OWASP Top 10 Coverage**:
  - Inyección (SQL, NoSQL, OS command)
  - Autenticación rota
  - Datos encriptados expuestos
  - XML External Entities (XXE)
  - Broken Access Control
  - Security misconfiguration
  - XSS (Cross-Site Scripting)
  - Insecure deserialization
  - Using components with known vulnerabilities
  - Insufficient logging & monitoring

- [ ] **Sugerencias automáticas**:
  - Validadores faltantes en DTOs
  - Headers de seguridad faltantes (Helmet, CORS)
  - Rate limiting en endpoints públicos
  - Proper error handling (sin info sensible)

### Integración DevSecOps

- [ ] **CI/CD Integration**:
  - GitHub Actions workflows
  - GitLab CI templates
  - Pre-commit hooks
  - Pre-push hooks

- [ ] **Reportes y compliance**:
  - Reportes de seguridad en PDF/JSON
  - Integración con herramientas de auditoría
  - SARIF output format
  - Métricas de seguridad en dashboard

**Target Release:** v6.0.0

**Benefits:**
- Prevent security breaches before they happen
- Automated vulnerability scanning
- Compliance with security standards (OWASP, SOC2)
- Proactive threat detection
- Reduced security audit time

---

## Fase 7: El Revisor de Élite (PR Mode) 🔍

**Enfoque:** Colaboración y calidad colectiva en Pull Requests

### Integración con GitHub/GitLab API

- [ ] **Autenticación**:
  - Personal Access Tokens
  - GitHub Apps integration
  - GitLab Personal Access Tokens
  - OAuth2 flow

- [ ] **Gestión de Pull Requests**:
  - Descarga automática de archivos del PR
  - Lectura de comentarios y conversaciones
  - Detección de cambios relacionados
  - Análisis de diffs línea por línea

- [ ] **Publicación de revisiones**:
  - Comentarios inline en código específico
  - Review general (approve, request changes, comment)
  - Threads de discusión automáticos
  - Revisión de múltiples commits

### Análisis de Diffs y Cambios

- [ ] **Parser inteligente de Git diffs**:
  - Extracción de solo líneas modificadas
  - Contexto del código cambiado
  - Detección de archivos movidos/renombrados
  - Análisis de conflictos de merge

- [ ] **Análisis contextual**:
  - Evaluación del cambio en relación al código existente
  - Detección de breaking changes
  - Validación de APIs modificadas
  - Análisis de firma de funciones
  - Impacto en otras partes del código

- [ ] **Detección de regresiones**:
  - Tests que dejan de pasar
  - Cobertura de tests reducida
  - Dead code introducido
  - Performance degradation

### Reporte de Revisión Inteligente

- [ ] **Resumen ejecutivo estructurado**:
  - ✅ **Aprobación**: "Este PR es seguro para mergear"
  - ⚠️ **Advertencias**: "El servicio de facturación perdió cobertura de tests"
  - ❌ **Bloqueos**: "Detectada vulnerabilidad de inyección SQL"
  - 📊 **Métricas**: +150 líneas, -30 líneas, 3 files changed

- [ ] **Checklist automático de calidad**:
  - Tests actualizados/presentes
  - Documentación actualizada
  - Sin secretos/credenciales
  - Sin dependencias vulnerables
  - Code coverage > threshold
  - Linting rules passed

- [ ] **Sugerencias clasificadas**:
  - 🔴 **Críticas**: Debe corregirse antes de merge
  - 🟡 **Opcionales**: Mejoras sugeridas pero no bloqueantes
  - 🟢 **Informativas**: Buenas prácticas o optimizaciones

- [ ] **Integración con CI/CD**:
  - Bloqueo automático de merges inseguros
  - Status checks en GitHub/GitLab
  - Required checks para merge
  - Protección de branches

### Colaboración en Equipo

- [ ] **Asignación de revisores**:
  - Detección de expertos por área
  - Load balancing de revisiones
  - Escalado automático a maintainers

- [ ] **Templates y estandarización**:
  - Plantillas de review customizables
  - Reglas de equipo configurables
  - Checklists por tipo de cambio

- [ ] **Métricas de calidad**:
  - Tiempo de revisión promedio
  - PRs revisados vs merged
  - Detección de bugs en producción
  - Technical debt tracking

**Target Release:** v7.0.0

**Benefits:**
- Automated code review (24/7)
- Consistent review quality
- 50-80% faster PR turnaround
- Reduced reviewer workload
- Improved code quality standards
- Knowledge sharing and onboarding
- Detection of human errors

---

## Fase 8: Enterprise y Escalabilidad de Élite 🚀

**Enfoque:** Herramientas para equipos grandes y organizaciones

### Core Features

- [ ] **Modo Daemon/Servicio**:
  - Ejecución en segundo plano
  - Auto-start en boot/systemd
  - Process management (PM2, systemd)
  - Health checks y auto-restart

- [ ] **Multi-project Monitoring**:
  - Soporte para 10+ proyectos simultáneos
  - Resource isolation per project
  - Prioritización de proyectos
  - Load balancing de recursos

- [ ] **Dashboard Web**:
  - Interfaz web para métricas del equipo
  - Grafana/Prometheus integration
  - Real-time monitoring
  - Customizable dashboards
  - Mobile-responsive

- [ ] **Integración con Webhooks**:
  - Slack notifications
  - Discord bot integration
  - Microsoft Teams webhooks
  - Google Chat integration
  - Mattermost

- [ ] **Reportes Avanzados**:
  - Métricas semanales/mensuales
  - Export a PDF/Excel/CSV
  - Executive summaries
  - Trend analysis

### Team Collaboration

- [ ] **Sistema de Permisos y Roles**:
  - Líder técnico (configuración completa)
  - Desarrollador (configuración limitada)
  - Revisor (solo lectura y sugerencias)
  - Viewer (solo lectura)
  - Admin (gestión de usuarios)

- [ ] **Integración con Project Management**:
  - Jira API integration
  - Linear API
  - GitHub Projects
  - Trello
  - Asana
  - Monday.com
  - Auto-creation de tickets

- [ ] **Notificaciones en Tiempo Real**:
  - Team-wide alerts
  - Incident notifications
  - Deployment notifications
  - Batch digests (hourly/daily)

- [ ] **Configuración Compartida**:
  - Team configuration templates
  - Global standards enforcement
  - Remote config synchronization
  - Override policies per project

- [ ] **Code Quality Standards**:
  - Team-wide linting rules
  - Style guides enforcement
  - Architecture patterns validation
  - Best practices library

### Enterprise Features

- [ ] **REST API**:
  - Endpoints para todas las operaciones
  - Webhook management
  - Metrics retrieval
  - Configuration management
  - Authentication (API Keys, JWT)

- [ ] **SSO Integration**:
  - SAML 2.0
  - OAuth 2.0 / OpenID Connect
  - LDAP / Active Directory
  - Okta
  - Auth0

- [ ] **Audit Logs & Compliance**:
  - Activity logging
  - Change history
  - User actions tracking
  - Compliance reports (SOC2, ISO27001)
  - Data retention policies

- [ ] **Custom AI Deployment**:
  - On-premise LLM deployment
  - Private cloud integration (AWS, GCP, Azure)
  - Custom model fine-tuning
  - Enterprise AI providers (Azure OpenAI, AWS Bedrock)

- [ ] **SLA Monitoring**:
  - Uptime tracking
  - Performance metrics
  - Alert thresholds
  - Incident management
  - Escalation policies

- [ ] **Multi-tenant Architecture**:
  - Team isolation
  - Resource quotas per team
  - Billing per department
  - Centralized administration

### Deployment Options

- [ ] **Cloud-hosted Service**:
  - SaaS offering
  - Managed infrastructure
  - Automatic updates
  - 99.9% SLA guarantee

- [ ] **Self-hosted Option**:
  - Docker images
  - Kubernetes Helm charts
  - On-premise deployment
  - Air-gapped environments

- [ ] **Hybrid Deployment**:
  - Local agent + Cloud dashboard
  - Edge computing
  - Distributed architecture

**Target Release:** v8.0.0

**Benefits:**
- Scalable for 100+ developers
- Centralized monitoring and governance
- Enterprise-grade security and compliance
- Custom integrations with existing toolchain
- Advanced analytics and business intelligence
- Reduced operational overhead
- Standardized development practices

---

## Consideraciones Futuras

### Community Requests

Features bajo consideración basadas en feedback de la comunidad:

- [ ] Plugin system para custom analyzers
- [ ] Custom rule definitions (YAML/JSON)
- [ ] Language Server Protocol (LSP) support
- [ ] IDE extensions (VS Code, IntelliJ, NeoVim)
- [ ] Docker container support
- [ ] Cloud-hosted service option
- [ ] Mobile app (iOS/Android)

### Investigación e Innovación

Explorando tecnologías de vanguardia:

- [ ] AI-powered code generation (más allá de sugerencias)
- [ ] Predictive bug detection (antes de escribir código)
- [ ] Automated refactoring suggestions
- [ ] Code smell detection con Machine Learning
- [ ] Performance optimization recommendations
- [ ] Architecture pattern recognition
- [ ] Natural language to code translation
- [ ] Automated test generation

---

## Cronograma de Releases (Tentativo)

| Versión | Fecha Estimada | Enfoque |
|---------|----------------|---------|
| v4.3.0 | Q1 2025 | Nuevos modelos IA (OpenAI, Mistral) |
| v4.4.0 | Q1 2025 | Modelos locales (Ollama, LM Studio) |
| v4.5.0 | Q2 2025 | Plataformas inferencia (Groq, Together AI) |
| v5.0.0 | Q2 2025 | Expansión multiplataforma (frameworks y lenguajes) |
| v6.0.0 | Q3 2025 | Características SecOps |
| v7.0.0 | Q4 2025 | PR review automation |
| v8.0.0 | Q1 2026 | Enterprise features |

> **Nota:** Las fechas son estimaciones y pueden cambiar según el progreso del desarrollo y feedback de la comunidad.

---

## Cómo Contribuir

Aceptamos contribuciones en cualquier fase del roadmap:

1. **Feature Requests**: Abre un issue con el label `enhancement`
2. **Bug Reports**: Ayúdanos a mejorar la estabilidad
3. **Code Contributions**: Envía PRs para características que te gustaría ver
4. **Documentation**: Mejora guías y ejemplos
5. **Testing**: Prueba features beta y da feedback

Ver [Contributing Guide](../CONTRIBUTING.md) para más detalles.

---

## Versión Actual

**📦 Current Release:** v4.2.0 (Parent File Detection)

**🚀 Next Release:** v4.3.0 (New AI Models - OpenAI, Mistral)

---

**Navigation:**
- [← Previous: Examples](examples.md)
- [← Back to README](../README.md)
