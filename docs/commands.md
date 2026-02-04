# Commands Reference

Sentinel includes keyboard commands for real-time control. The command list is automatically displayed when Sentinel starts.

## Available Commands

| Command | Action |
|---------|--------|
| `p` | Pause/Resume monitoring |
| `r` | Generate daily productivity report |
| `m` | View metrics dashboard (bugs, costs, tokens, time) |
| `l` | Clear AI response cache |
| `h` / `help` | Show command help |
| `x` | Reset configuration from scratch |

> **Note**: The command list is automatically shown when starting Sentinel. Use `h` or `help` to see it again at any time.

---

## Command Details

### Pause/Resume (command 'p')

Pause or resume file monitoring.

**Method 1: Press `p` in the terminal:**
```
⌨️  SENTINEL: PAUSED
⌨️  SENTINEL: ACTIVE
```

**Method 2: Create `.sentinel-pause` file in project directory:**
```bash
touch .sentinel-pause  # Pause
rm .sentinel-pause     # Resume
```

**Use cases:**
- Taking a break from development
- Making large refactors without triggering analysis
- Temporarily disabling monitoring

---

### View Metrics (command 'm')

Press `m` to view the real-time performance dashboard:

```
📊 DASHBOARD DE RENDIMIENTO SENTINEL
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🚫 Bugs Evitados:  12
💰 Costo Acumulado: $0.4523
🎟️ Tokens Usados:   45230
⏳ Tiempo Ahorrado: 6.5h
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Tracked metrics:**
- Critical bugs prevented by AI analysis
- Accumulated API usage cost
- Total tokens consumed
- Estimated time saved in debugging

Metrics are persisted in `.sentinel_stats.json` and accumulate across sessions.

**Example usage:**

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

---

### Generate Daily Report (command 'r')

Press `r` in the terminal to generate a daily productivity report:

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

**Notes:**
- The report analyzes all commits made since 00:00:00 of the current day
- Automatically saved to `docs/DAILY_REPORT.md`
- If there are no commits for the day, shows a warning and doesn't generate report

**Complete example:**

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
```

---

### Clear Cache (command 'l')

Press `l` to delete all AI response cache:

```
⚠️  ¿Limpiar todo el caché? Esto eliminará las respuestas guardadas (s/n): s
   🗑️  Caché limpiado exitosamente.
   💡 El caché se regenerará automáticamente en las próximas consultas.
```

**When to use:**
- You've changed AI model and want fresh responses
- You suspect the cache has outdated responses
- You want to free up disk space
- You're troubleshooting issues related to incorrect responses

**Note:** The cache regenerates automatically, so clearing the cache doesn't affect functionality.

**Example:**

```
l  ← [User presses 'l']

⚠️  ¿Limpiar todo el caché? Esto eliminará las respuestas guardadas (s/n): s
   🗑️  Caché limpiado exitosamente.
   💡 El caché se regenerará automáticamente en las próximas consultas.
```

> **Note:** Useful when you change AI model or want to force fresh responses.

---

### Show Help (command 'h' or 'help')

Display the command reference:

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

---

### Reset Configuration (command 'x')

Press `x` to delete the current configuration and start over:

```
⚠️ ¿Reiniciar configuración? (s/n): s
🗑️  Configuración eliminada correctamente.
```

Sentinel will close and when run again, it will start the configuration assistant.

**Use cases:**
- You want to change API provider
- You need to update API keys
- You want to reconfigure architecture rules
- Configuration file is corrupted

---

## Interactive Flows

### Making Commits

When tests pass:
```
🚀 Mensaje sugerido: feat: add user authentication service
📝 ¿Quieres hacer commit? (s/n, timeout 30s): s
   ✅ Commit exitoso!
```

**With timeout:**
```
🚀 Mensaje sugerido: feat: add user validation
📝 ¿Quieres hacer commit? (s/n, timeout 30s):
   ⏭️  Timeout, commit omitido.
```

### Analyzing Test Errors

When tests fail:
```
   ❌ Tests fallaron
🔍 ¿Quieres que Claude analice el error? (s/n, timeout 15s): s
💡 SOLUCIÓN SUGERIDA:
[Detailed diagnosis from Claude]
```

---

**Navigation:**
- [← Previous: Configuration](configuration.md)
- [Next: AI Providers →](ai-providers.md)
