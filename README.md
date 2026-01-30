# Sentinel

<p align="center">
  <strong>🛡️ Asistente de Productividad de Élite: Orquestador de IA para la Auditoría de Arquitectura, Testing Autónomo y Observabilidad de Desarrollo.</strong>
</p>

Herramienta de monitoreo de archivos escrita en Rust que analiza cambios de código usando Claude AI y gestiona el flujo de trabajo con Git. Diseñada para integrarse con proyectos NestJS/TypeScript como asistente de desarrollo en tiempo real.

## Características principales

- 🔍 **Monitoreo en tiempo real** del sistema de archivos (directorio `src/`) con debounce para evitar procesamiento duplicado
- 🤖 **Análisis automático de código con Claude AI**
  - Principios SOLID
  - Clean Code
  - Buenas prácticas NestJS
  - Consejo textual visible en consola, código sugerido solo en archivo `.suggested`
- 🧪 **Detección y ejecución de tests con Jest** con salida visible en tiempo real en la consola
- 📝 **Flujo interactivo de commits en Git** con timeout de 30 segundos
- 💡 **Generación de sugerencias de código** guardadas en archivos `.suggested`
- ⏸️ **Mecanismo de pausa** mediante archivo `.sentinel-pause` o comando 'p'
- ✨ **Mensajes de commit inteligentes** siguiendo Conventional Commits
- 🔧 **Diagnóstico automático de fallos en tests** con timeout de 30 segundos
- 📚 **Auto-documentación técnica** - genera archivos .md junto a cada .ts con "manual de bolsillo" generado por IA
- 📊 **Reportes diarios de productividad** - genera resúmenes inteligentes de los commits del día (comando 'r')
- 🔄 **Stdin centralizado** - lectura de input sin conflictos entre hilos

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- Credenciales de la API de Anthropic

## Variables de entorno

| Variable | Descripcion |
|----------|-------------|
| `ANTHROPIC_AUTH_TOKEN` | API key de Anthropic |
| `ANTHROPIC_BASE_URL` | URL base de la API (ej. `https://api.anthropic.com`) |

## Instalación

### Clonar el repositorio

```bash
git clone https://github.com/<tu-usuario>/sentinel-rust.git
cd sentinel-rust
```

### Compilar en modo release

```bash
cargo build --release
```

El binario compilado estará en `target/release/sentinel-rust` (o `sentinel-rust.exe` en Windows).

## Configuración

### Variables de entorno

Configura las credenciales de la API de Anthropic:

```bash
# Linux/macOS
export ANTHROPIC_AUTH_TOKEN="sk-ant-api03-..."
export ANTHROPIC_BASE_URL="https://api.anthropic.com"

# Windows (PowerShell)
$env:ANTHROPIC_AUTH_TOKEN="sk-ant-api03-..."
$env:ANTHROPIC_BASE_URL="https://api.anthropic.com"

# Windows (CMD)
set ANTHROPIC_AUTH_TOKEN=sk-ant-api03-...
set ANTHROPIC_BASE_URL=https://api.anthropic.com
```

Para hacerlas permanentes, agrégalas a tu archivo de perfil (`~/.bashrc`, `~/.zshrc`, etc.).

## Uso

### Iniciar Sentinel

```bash
# Desde el directorio del proyecto
cargo run

# O usando el binario compilado
./target/release/sentinel-rust
```

### Flujo de trabajo

1. **Seleccionar proyecto**: Al iniciar, Sentinel muestra un menú con proyectos disponibles en el directorio padre
2. **Monitoreo activo**: Sentinel vigila cambios en archivos `.ts` del directorio `src/`
3. **Al detectar un cambio**:
   - ✨ Envía el código a Claude AI para análisis
   - ✅ Si no hay problemas críticos, ejecuta los tests relacionados con Jest
   - 🚀 Si los tests pasan, genera un mensaje de commit y pregunta si quieres hacer commit
   - 🔍 Si los tests fallan, ofrece diagnóstico de Claude

### Estructura esperada del proyecto

Sentinel espera que tu proyecto NestJS tenga la siguiente estructura:

```
mi-proyecto/
├── src/              ← OBLIGATORIO: Sentinel vigila este directorio
│   └── users/
│       └── users.service.ts
└── test/
    └── users/
        └── users.spec.ts
```

**Requisitos importantes:**
- El proyecto **DEBE** tener un directorio `src/` (Sentinel lo validará al iniciar)
- Para cada archivo `src/module/file.ts`, debe existir `test/module/file.spec.ts`
- Si el proyecto no tiene `src/`, Sentinel mostrará un error descriptivo y se detendrá

### Controles interactivos

Sentinel v3.3 incluye comandos de teclado para control en tiempo real:

#### Pausar/Reanudar (comando 'p')

Método 1: Presiona `p` en la terminal:
```
⌨️  SENTINEL: PAUSADO
⌨️  SENTINEL: ACTIVO
```

Método 2: Crea el archivo `.sentinel-pause` en el directorio del proyecto:
```bash
touch .sentinel-pause  # Pausar
rm .sentinel-pause     # Reanudar
```

#### Generar reporte diario (comando 'r')

Presiona `r` en la terminal para generar un reporte de productividad del día:

```
📊 Generando reporte de productividad diaria...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📝 REPORTE DIARIO DE SENTINEL
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✨ Logros Principales
- Implementación completa de autenticación JWT
- Migración de base de datos a PostgreSQL 15

🛠️ Aspectos Técnicos
- Integración con NestJS Guards para protección de rutas
- Refactorización de servicios aplicando patrón Repository

🚀 Próximos Pasos
- Testing de endpoints de autenticación
- Documentación de API con Swagger

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

   ✅ Documento generado: docs/DAILY_REPORT.md
```

**Notas:**
- El reporte analiza todos los commits realizados desde las 00:00:00 del día actual
- Se guarda automáticamente en `docs/DAILY_REPORT.md`
- Si no hay commits del día, muestra advertencia y no genera reporte

#### Hacer commit

Cuando los tests pasan:
```
🚀 Mensaje sugerido: feat: add user authentication service
📝 ¿Quieres hacer commit? (s/n, timeout 30s): s
   ✅ Commit exitoso!
```

#### Analizar errores de tests

Cuando los tests fallan:
```
   ❌ Tests fallaron
🔍 ¿Quieres que Claude analice el error? (s/n, timeout 15s): s
💡 SOLUCIÓN SUGERIDA:
[Diagnóstico detallado de Claude]
```

## Ejemplos

### Ejemplo 1: Cambio aprobado

```
🔔 CAMBIO EN: users.service.ts

✨ CONSEJO DE CLAUDE:
SEGURO - El código sigue correctamente el patrón Repository.
Se recomienda agregar validación en el método create().

   ✅ Arquitectura aprobada.
🧪 Ejecutando Jest para: test/users/users.spec.ts

  [Jest output visible en tiempo real...]
  PASS  test/users/users.spec.ts

   ✅ Tests pasados con éxito

📝 Generando mensaje de commit inteligente...
🚀 Mensaje sugerido: feat: add findAll method to users service
📝 ¿Quieres hacer commit? (s/n, timeout 30s): n
   ⏭️  Commit omitido.
```

> **Nota:** El consejo de Claude muestra solo el texto explicativo. El código sugerido se guarda en `users.service.ts.suggested`.

### Ejemplo 2: Problemas detectados

```
🔔 CAMBIO EN: products.controller.ts

✨ CONSEJO DE CLAUDE:
CRITICO - Violación del principio de responsabilidad única (SRP).
El controlador está accediendo directamente a la base de datos.

   ❌ CRITICO: Corrige SOLID/Bugs
```

### Ejemplo 3: Tests fallidos

```
🔔 CAMBIO EN: auth.service.ts
   ✅ Arquitectura aprobada.
🧪 Ejecutando Jest para: test/auth/auth.spec.ts

  [Jest output visible en tiempo real...]
  FAIL  test/auth/auth.spec.ts

   ❌ Tests fallaron

🔍 ¿Analizar error con IA? (s/n, timeout 30s): s

🔍 Analizando fallo en tests...
💡 SOLUCIÓN SUGERIDA:
El problema está en que el método `validateUser` no está manejando...
```

### Ejemplo 4: Timeout sin respuesta

```
🚀 Mensaje sugerido: feat: add user validation
📝 ¿Quieres hacer commit? (s/n, timeout 30s):
   ⏭️  Timeout, commit omitido.
```

### Ejemplo 5: Reporte diario de productividad

```
🛡️  Sentinel v3.3 activo en: C:\projects\mi-api-nestjs

[... trabajas durante el día, haciendo varios commits ...]

r  ← [Usuario presiona 'r']

📊 Generando reporte de productividad diaria...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📝 REPORTE DIARIO DE SENTINEL
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✨ Logros Principales
- Sistema de autenticación JWT completamente implementado
- Integración de base de datos PostgreSQL finalizada
- Módulo de usuarios con operaciones CRUD operativo

🛠️ Aspectos Técnicos
- Implementación de Guards de NestJS para protección de rutas
- Configuración de TypeORM con migraciones automáticas
- Aplicación de patrón Repository en servicios
- Validación de DTOs con class-validator

🚀 Próximos Pasos
- Implementar tests E2E para flujo de autenticación
- Añadir documentación Swagger a los endpoints
- Configurar rate limiting para prevenir abusos
- Implementar sistema de refresh tokens

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Arquitectura

### Flujo principal (monitoreo de archivos)

```
┌─────────────────┐
│  File Watcher   │ (notify crate)
└────────┬────────┘
         │ Detecta cambio en .ts
         ▼
┌─────────────────┐
│ Análisis Claude │ (consultar_claude)
└────────┬────────┘
         │ Código aprobado
         ▼
┌─────────────────┐
│  Jest Tests     │ (ejecutar_tests)
└────────┬────────┘
         │ Tests pasan
         ▼
┌──────────────────────┐
│ Auto-Documentación   │ (actualizar_documentacion)
│ Genera archivo .md   │
└────────┬─────────────┘
         ▼
┌─────────────────┐
│  Git Commit     │ (preguntar_commit)
└─────────────────┘
```

### Hilo de teclado (stdin centralizado)

```
┌─────────────────┐
│  Usuario (stdin)│  ← Único punto de lectura de stdin
└────────┬────────┘
         │
         ├─ [esperando input] ──▶ Reenvía respuesta al loop principal (s/n)
         │
         ├─ 'p' ──▶ Pausar/Reanudar
         │
         └─ 'r' ──▶ ┌────────────────────┐
                    │ Reporte Diario     │
                    │ (generar_reporte_  │
                    │  diario)           │
                    └────────┬───────────┘
                             │
                             ├─▶ git log --since=00:00:00
                             │
                             ├─▶ Claude AI (análisis)
                             │
                             └─▶ docs/DAILY_REPORT.md
```

### Debounce y drenado de eventos

- Eventos duplicados del mismo archivo se ignoran dentro de una ventana de 15 segundos
- Al terminar de procesar un archivo, se drenan todos los eventos pendientes del canal
- Esto evita reprocesar el mismo archivo cuando el editor genera múltiples eventos de escritura

### Componentes principales

| Componente | Descripción |
|------------|-------------|
| `consultar_claude()` | Comunicación con API de Claude AI (Anthropic) |
| `analizar_arquitectura()` | Evaluación de código basada en SOLID y Clean Code |
| `ejecutar_tests()` | Ejecución de tests de Jest con salida visible en consola |
| `pedir_ayuda_test()` | Diagnóstico de fallos con IA |
| `actualizar_documentacion()` | Genera "manual de bolsillo" .md junto a cada archivo .ts |
| `generar_mensaje_commit()` | Generación de mensajes siguiendo Conventional Commits |
| `preguntar_commit()` | Ejecuta commit si el usuario acepta (recibe respuesta del loop principal) |
| `obtener_resumen_git()` | Obtiene commits del día usando git log |
| `generar_reporte_diario()` | Crea reporte de productividad con IA basado en commits |

## Archivos generados

### `.suggested` files

Cuando Claude analiza un archivo, genera una versión mejorada guardada como:

```
users.service.ts.suggested
```

Este archivo contiene el código refactorizado siguiendo las recomendaciones de Claude.

### Archivos `.md` (Manuales de bolsillo)

Cuando los tests pasan exitosamente, Sentinel genera automáticamente un "manual de bolsillo" en formato Markdown para cada archivo modificado. El archivo .md se crea en el mismo directorio que el .ts original.

**Ubicación:** `src/users/users.service.ts` → `src/users/users.service.md`

**Contenido:**
- Resumen ultra-conciso (máximo 150 palabras)
- Descripción de funcionalidad principal
- Lista de métodos importantes
- Timestamp de última actualización

**Ejemplo:**

```markdown
# 📖 Documentación: users.service.ts

> ✨ Actualizado automáticamente por Sentinel v3.1

🎯 **Funcionalidad**: Gestiona operaciones CRUD de usuarios en el sistema. Implementa
la lógica de negocio para creación, lectura, actualización y eliminación de usuarios,
aplicando validaciones y transformaciones necesarias.

🔧 **Métodos principales**:
- `findAll()` - Lista usuarios con paginación y filtros
- `findOne(id)` - Busca usuario por ID
- `create(dto)` - Crea nuevo usuario validando datos
- `update(id, dto)` - Actualiza usuario existente
- `remove(id)` - Eliminación lógica de usuario

---
*Último refactor: SystemTime { tv_sec: 1706198400, tv_nsec: 0 }*
```

Esta documentación se actualiza automáticamente cada vez que el archivo pasa las pruebas.

### `docs/DAILY_REPORT.md`

Cuando presionas **'r'** en la consola, Sentinel genera un reporte de productividad diario analizando todos los commits realizados desde las 00:00:00. El reporte usa Claude AI para:

- Resumir logros principales del día
- Identificar aspectos técnicos relevantes (NestJS, Rust, etc.)
- Sugerir próximos pasos basándose en el progreso

**Ubicación:** `docs/DAILY_REPORT.md`

**Contenido:**

```markdown
✨ Logros Principales
- Sistema de autenticación JWT completamente funcional
- Migración de MongoDB a PostgreSQL finalizada
- Implementación de caché con Redis

🛠️ Aspectos Técnicos
- Integración de Passport.js con estrategias JWT y Local
- Implementación de Guards personalizados en NestJS
- Configuración de TypeORM con migrations
- Optimización de queries con índices compuestos

🚀 Próximos Pasos
- Añadir tests de integración para endpoints de autenticación
- Documentar API con Swagger/OpenAPI
- Implementar refresh tokens para mejorar seguridad
- Configurar CI/CD con GitHub Actions
```

**Uso recomendado:**
- Ejecutar al final del día de trabajo (comando 'r')
- Compartir con el equipo en stand-ups
- Usar como base para reportes semanales
- Mantener registro histórico del progreso del proyecto

## Troubleshooting

### Error: "Input watch path is neither a file nor a directory"

Este error ocurre cuando:
- El proyecto seleccionado **no tiene** un directorio `src/`
- La ruta del proyecto no existe o no es válida

**Solución:**
1. Asegúrate de que el proyecto tenga una carpeta `src/`:
   ```bash
   mkdir src
   ```
2. O selecciona un proyecto diferente que ya tenga esta estructura

Sentinel ahora valida automáticamente la existencia del directorio `src/` y muestra mensajes de error descriptivos.

### Error: "Falta ANTHROPIC_AUTH_TOKEN"

Asegúrate de configurar las variables de entorno:

```bash
export ANTHROPIC_AUTH_TOKEN="tu-token"
export ANTHROPIC_BASE_URL="https://api.anthropic.com"
```

### Error: "No se puede conectar a la API"

Verifica tu conexión a internet y que la URL base sea correcta:

```bash
curl -I https://api.anthropic.com
```

### Sentinel no detecta cambios

- Verifica que estás modificando archivos `.ts` en el directorio `src/`
- Archivos `.spec.ts` y `.suggested` son ignorados intencionalmente
- Revisa que el watcher esté activo (no pausado con 'p' o `.sentinel-pause`)
- El debounce ignora eventos del mismo archivo dentro de 15 segundos; espera antes de guardar de nuevo

### Tests no se ejecutan

- Verifica que existe el archivo de test correspondiente en `test/module/file.spec.ts`
- Asegúrate de que `npm run test` funciona en tu proyecto
- Verifica que Jest esté configurado correctamente en tu proyecto NestJS

### Commits no se crean

- Verifica que tienes git inicializado en el proyecto
- Asegúrate de tener permisos de escritura
- Revisa que no haya hooks de git bloqueando el commit

### No se genera reporte diario (comando 'r')

- Asegúrate de tener commits realizados en el día actual (desde las 00:00:00)
- Verifica que git está instalado: `git --version`
- Confirma que estás en un repositorio git válido: `git status`
- Si el error persiste, revisa que `ANTHROPIC_AUTH_TOKEN` esté configurado correctamente

## Dependencias

| Crate | Versión | Uso |
|-------|---------|-----|
| `notify` | 6.1.1 | Monitoreo del sistema de archivos |
| `reqwest` | 0.11 | Cliente HTTP para la API de Claude |
| `serde` | 1.0 | Serialización de datos |
| `serde_json` | 1.0 | Procesamiento de JSON |
| `anyhow` | 1.0 | Manejo robusto de errores |
| `colored` | 2.0 | Salida con colores en terminal |

## Roadmap

### Fase 1: Fundamentos (Completada ✅)
**Enfoque:** Monitoreo básico y análisis de código

- [x] Monitoreo en tiempo real con file watcher (notify)
- [x] Análisis de arquitectura con Claude AI (SOLID, Clean Code)
- [x] Ejecución automática de tests con Jest
- [x] Generación de mensajes de commit inteligentes
- [x] Flujo interactivo de commits con Git

### Fase 2: Productividad y Documentación (Completada ✅)
**Enfoque:** Automatización de tareas repetitivas

- [x] Auto-documentación de archivos (.md generados automáticamente) - v3.1
- [x] Reportes diarios de productividad - v3.2
- [x] Sugerencias de código en archivos `.suggested` - v3.3
- [x] Diagnóstico automático de fallos en tests - v3.3

### Fase 3: Optimización y Estabilidad (Completada ✅)
**Enfoque:** Mejoras de rendimiento y UX

- [x] Stdin centralizado sin conflictos entre hilos - v3.3
- [x] Tests de Jest visibles en consola en tiempo real - v3.3
- [x] Debounce y drenado de eventos duplicados del watcher - v3.3
- [x] Validación de estructura de proyecto (directorio `src/`) - v3.3.1
- [x] Manejo robusto de errores con mensajes descriptivos - v3.3.1
- [x] Configuración personalizable mediante archivo `.sentinelrc.toml` - v3.3
- [x] Sistema de estadísticas y métricas de productividad - v3.3

### Fase 4: El Guardián de Seguridad (SecOps) 🔒
**Enfoque:** Prevención de riesgos y blindaje de código

- [ ] **Módulo de Escaneo de Secretos**
  - Motor basado en Regex y entropía para detectar:
    - Llaves de API (AWS, Google Cloud, Azure)
    - Tokens JWT en texto plano
    - Contraseñas hardcodeadas
    - Credenciales de bases de datos
  - Bloqueo automático de commits con secretos detectados
  - Sugerencias de variables de entorno (.env)

- [ ] **Auditoría de Dependencias**
  - Lectura y análisis de `package-lock.json` / `Cargo.lock`
  - Consulta de bases de datos de vulnerabilidades (CVEs)
  - Integración con GitHub Security Advisories
  - Alertas de dependencias obsoletas o inseguras
  - Reporte de score de seguridad del proyecto

- [ ] **Sanitización Automática**
  - Prompt especializado para NestJS:
    - Detección de DTOs sin decoradores de validación (`class-validator`)
    - Validación de uso correcto de `ValidationPipe`
    - Prevención de inyección SQL en queries de TypeORM
    - Validación de sanitización de inputs en endpoints
  - Sugerencias automáticas de validadores faltantes
  - Análisis de vectores de ataque comunes (OWASP Top 10)

### Fase 5: El Revisor de Élite (PR Mode) 🔍
**Enfoque:** Colaboración y calidad colectiva

- [ ] **Integración con GitHub API**
  - Autenticación con tokens personales o GitHub Apps
  - Descarga automática de archivos de Pull Requests
  - Lectura de comentarios y conversaciones existentes
  - Capacidad de publicar revisiones directamente en GitHub

- [ ] **Análisis de Diff/Cambios**
  - Parser de diffs de Git para extraer solo líneas modificadas
  - Análisis contextual: la IA evalúa el cambio en relación al código existente
  - Detección de breaking changes (APIs modificadas, firmas de funciones)
  - Validación de que los cambios no rompen la lógica existente
  - Análisis de impacto en otras partes del código

- [ ] **Reporte de Revisión**
  - Generación de resumen ejecutivo estructurado:
    - ✅ **Aprobación:** "Este PR es seguro para mergear"
    - ⚠️ **Advertencias:** "El servicio de facturación perdió cobertura de tests"
    - ❌ **Bloqueos:** "Detectada vulnerabilidad de inyección SQL"
  - Comentarios en línea sobre código específico
  - Checklist automático de calidad (tests, docs, seguridad)
  - Sugerencias de mejoras opcionales vs. cambios obligatorios
  - Integración con sistemas de CI/CD para bloquear merges inseguros

### Fase 6: Expansión Multiplataforma 🌐
**Enfoque:** Compatibilidad con más tecnologías

- [ ] Soporte para otros frameworks JavaScript:
  - Angular (standalone components, signals)
  - React (hooks, Context API)
  - Vue 3 (Composition API)
  - SolidJS, Svelte
- [ ] Integración con otros test runners:
  - Vitest
  - Mocha + Chai
  - Playwright (E2E)
  - Cypress
- [ ] Soporte para otros lenguajes:
  - Python (pytest, Django, FastAPI)
  - Go (testing package)
  - Java (JUnit, Spring Boot)

### Fase 7: Empresa y Escalabilidad 🚀
**Enfoque:** Herramientas para equipos y organizaciones

- [ ] Modo daemon/servicio en segundo plano
- [ ] Soporte para múltiples proyectos simultáneos
- [ ] Dashboard web para visualización de métricas del equipo
- [ ] Integración con webhooks (Slack, Discord, Microsoft Teams)
- [ ] Métricas y reportes semanales/mensuales
- [ ] Sistema de permisos y roles (líder técnico, desarrollador, revisor)
- [ ] API REST para integración con herramientas externas
- [ ] Integración con Jira/Linear para tracking de tareas

## Contribuir

Las contribuciones son bienvenidas. Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/nueva-funcionalidad`)
3. Commit tus cambios (`git commit -am 'feat: add nueva funcionalidad'`)
4. Push a la rama (`git push origin feature/nueva-funcionalidad`)
5. Abre un Pull Request

## Licencia

Este proyecto está bajo la Licencia MIT. Ver el archivo `LICENSE` para más detalles.

## Autor

**Sergio Guadarrama**
📧 sguadarrama@tiprotec.com

---

<p align="center">
  Hecho con ❤️ usando Rust y Claude AI
</p>
