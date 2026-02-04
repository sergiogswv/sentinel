# Usage Examples

This guide provides real-world examples of Sentinel in action.

## Example 1: Approved Change

A successful workflow where code passes architecture review and tests:

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

> **Note:** Claude's advice shows only explanatory text. The suggested code is saved in `users.service.ts.suggested`.

**Key Points:**
- Architecture review passed
- Tests executed and passed
- Commit suggested but skipped by user
- Suggested code saved separately

---

## Example 2: Problems Detected

When architectural issues are found:

```
🔔 CAMBIO EN: products.controller.ts

✨ CONSEJO DE CLAUDE:
CRITICO - Violación del principio de responsabilidad única (SRP).
El controlador está accediendo directamente a la base de datos.

   ❌ CRITICO: Corrige SOLID/Bugs
```

**Key Points:**
- Critical issue detected
- Workflow stops (tests not run)
- User must fix issues before continuing
- Suggested fix saved in `.suggested` file

---

## Example 3: Failed Tests

When tests fail and AI diagnosis is requested:

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
El problema está en que el método `validateUser` no está manejando
correctamente el caso cuando el usuario no existe. Necesitas:

1. Agregar verificación null en línea 45
2. Lanzar UnauthorizedException apropiadamente
3. Actualizar el test para mockear UserService.findOne()

Código sugerido guardado en: auth.service.ts.suggested
```

**Key Points:**
- Architecture passed but tests failed
- User opted for AI diagnosis
- Detailed solution provided
- Specific line numbers and fixes suggested

---

## Example 4: Timeout Without Response

When user doesn't respond to commit prompt:

```
🚀 Mensaje sugerido: feat: add user validation
📝 ¿Quieres hacer commit? (s/n, timeout 30s):
   ⏭️  Timeout, commit omitido.
```

**Key Points:**
- 30-second timeout for commit prompt
- Auto-skip on timeout
- User can continue working without interruption

---

## Example 5: Cache in Action

When the same code is analyzed again:

```
🔔 CAMBIO EN: users.service.ts

   ♻️  Usando respuesta de caché...

✨ CONSEJO DE CLAUDE:
SEGURO - El código sigue correctamente el patrón Repository.
[... Código guardado en .suggested ...]

   ✅ Arquitectura aprobada.
```

> **Note:** If the same code is analyzed again, Sentinel reuses the previous response, saving time and costs.

**Key Points:**
- Instant response (no API call)
- Zero cost for cached query
- Identical quality to original response

---

## Example 6: Fallback Model in Action

When primary model fails and fallback takes over:

```
🔔 CAMBIO EN: auth.service.ts

   ⚠️  Modelo principal falló: Connection timeout. Intentando fallback con gemini-2.0-flash...

✨ CONSEJO DE CLAUDE:
SEGURO - La implementación de autenticación JWT es correcta.
[... Código guardado en .suggested ...]

   ✅ Arquitectura aprobada.
```

**Key Points:**
- Seamless failover to backup model
- User informed of model switch
- Workflow continues without interruption
- High availability ensured

---

## Example 7: Metrics Dashboard (command 'm')

Viewing real-time metrics:

```
m  ← [User presses 'm']

📊 DASHBOARD DE RENDIMIENTO SENTINEL
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🚫 Bugs Evitados:  12
💰 Costo Acumulado: $0.4523
🎟️ Tokens Usados:   45230
⏳ Tiempo Ahorrado: 6.5h
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Key Points:**
- Real-time tracking of productivity metrics
- Cost monitoring for budget control
- Time saved estimation
- Bugs prevented counter

---

## Example 8: Help Command (command 'h' or 'help')

Displaying available commands:

```
h  ← [User presses 'h' or types 'help']

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
⌨️  COMANDOS DISPONIBLES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  p       Pausar/Reanudar monitoreo
  r       Generar reporte diario de productividad
  m       Ver dashboard de métricas (bugs, costos, tokens)
  l       Limpiar caché de respuestas de IA
  x       Reiniciar configuración desde cero
  h/help  Mostrar esta ayuda
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

> **Note:** This help is also automatically displayed when starting Sentinel.

**Key Points:**
- Quick reference for all commands
- Available anytime during session
- Also shown automatically at startup

---

## Example 9: Clear Cache (command 'l')

Clearing AI response cache:

```
l  ← [User presses 'l']

⚠️  ¿Limpiar todo el caché? Esto eliminará las respuestas guardadas (s/n): s
   🗑️  Caché limpiado exitosamente.
   💡 El caché se regenerará automáticamente en las próximas consultas.
```

> **Note:** Useful when you change AI model or want to force fresh responses.

**Key Points:**
- Confirmation required before deletion
- Safe operation (cache regenerates automatically)
- Useful for troubleshooting or model changes

---

## Example 10: Daily Productivity Report

Generating end-of-day report:

```
🛡️  Sentinel activo en: C:\projects\mi-api-nestjs

[... you work during the day, making several commits ...]

r  ← [User presses 'r']

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

   ✅ Documento generado: docs/DAILY_REPORT.md
```

**Key Points:**
- Analyzes all commits since midnight
- AI-generated summary of accomplishments
- Technical aspects highlighted
- Suggested next steps
- Saved for future reference

---

## Complete Workflow Example

A typical development session with Sentinel:

```
# 1. Start Sentinel
./target/release/sentinel-rust

🛡️ Sentinel v4.1.1 activo en: /path/to/project

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
⌨️  COMANDOS DISPONIBLES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  p       Pausar/Reanudar monitoreo
  r       Generar reporte diario de productividad
  m       Ver dashboard de métricas (bugs, costos, tokens)
  l       Limpiar caché de respuestas de IA
  x       Reiniciar configuración desde cero
  h/help  Mostrar esta ayuda
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# 2. Edit a file
# User modifies src/users/users.service.ts

🔔 CAMBIO EN: users.service.ts
   ♻️  Usando respuesta de caché...
✨ CONSEJO DE CLAUDE: SEGURO - Código bien estructurado
   ✅ Arquitectura aprobada.
🧪 Ejecutando Jest para: test/users/users.spec.ts
   ✅ Tests pasados con éxito
🚀 Mensaje sugerido: feat: add pagination to users list
📝 ¿Quieres hacer commit? (s/n, timeout 30s): s
   ✅ Commit exitoso!

# 3. Check metrics after several changes
m

📊 DASHBOARD DE RENDIMIENTO SENTINEL
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🚫 Bugs Evitados:  3
💰 Costo Acumulado: $0.12
🎟️ Tokens Usados:   8420
⏳ Tiempo Ahorrado: 1.5h
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

# 4. End of day - generate report
r

📊 Generando reporte de productividad diaria...
   ✅ Documento generado: docs/DAILY_REPORT.md

# 5. Stop for the day
Ctrl+C
```

**Key Points:**
- Complete development cycle
- Multiple tools working together
- Automatic tracking and reporting
- Minimal user intervention needed

---

## Real-World Scenarios

### Scenario: Refactoring Legacy Code

```
# Day 1: Initial refactoring
🔔 CAMBIO EN: legacy-service.ts
✨ CONSEJO: CRITICO - Múltiples violaciones SOLID detectadas
   ❌ CRITICO: Corrige SOLID/Bugs

# Fix issues, save again
🔔 CAMBIO EN: legacy-service.ts
✨ CONSEJO: SEGURO - Refactorización correcta
   ✅ Arquitectura aprobada.
   ✅ Tests pasados
🚀 Commit: refactor: split legacy service into smaller modules
```

### Scenario: Adding New Feature

```
# Create new service
🔔 CAMBIO EN: notifications.service.ts
✨ CONSEJO: SEGURO - Implementación limpia
   ✅ Arquitectura aprobada.
   ❌ Tests fallaron
🔍 ¿Analizar error? (s/n): s
💡 SOLUCIÓN: Mock faltante para EmailService

# Fix tests
🔔 CAMBIO EN: notifications.service.ts
   ✅ Tests pasados
🚀 Commit: feat: add email notification service
```

### Scenario: Bug Fix

```
# Fix reported bug
🔔 CAMBIO EN: auth.middleware.ts
✨ CONSEJO: SEGURO - Fix correcto
   ✅ Arquitectura aprobada.
   ✅ Tests pasados
🚀 Commit: fix: handle missing JWT token gracefully
```

---

**Navigation:**
- [← Previous: Architecture](architecture.md)
- [Next: Roadmap →](roadmap.md)
