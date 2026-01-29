# Sentinel

<p align="center">
  <strong>🛡️ Asistente de desarrollo impulsado por IA para proyectos NestJS/TypeScript</strong>
</p>

Herramienta de monitoreo de archivos escrita en Rust que analiza cambios de código usando Claude AI y gestiona el flujo de trabajo con Git. Diseñada para integrarse con proyectos NestJS/TypeScript como asistente de desarrollo en tiempo real.

## Características principales

- 🔍 **Monitoreo en tiempo real** del sistema de archivos (directorio `src/`)
- 🤖 **Análisis automático de código con Claude AI**
  - Principios SOLID
  - Clean Code
  - Buenas prácticas NestJS
- 🧪 **Detección y ejecución de tests con Jest**
- 📝 **Flujo interactivo de commits en Git** con timeout de 30 segundos
- 💡 **Generación de sugerencias de código** guardadas en archivos `.suggested`
- ⏸️ **Mecanismo de pausa** mediante archivo `.sentinel-pause` o comando 'p'
- ✨ **Mensajes de commit inteligentes** siguiendo Conventional Commits
- 🔧 **Diagnóstico automático de fallos en tests**

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
├── src/
│   └── users/
│       └── users.service.ts
└── test/
    └── users/
        └── users.spec.ts
```

Para cada archivo `src/module/file.ts`, debe existir `test/module/file.spec.ts`.

### Controles interactivos

#### Pausar/Reanudar

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
SEGURO - El código sigue correctamente el patrón Repository...

   ✅ Arquitectura aprobada.
🧪 Ejecutando Jest para: test/users/users.spec.ts
   ✅ Tests pasados con éxito

📝 Generando mensaje de commit inteligente...
🚀 Mensaje sugerido: feat: add findAll method to users service
📝 ¿Quieres hacer commit? (s/n, timeout 30s):
```

### Ejemplo 2: Problemas detectados

```
🔔 CAMBIO EN: products.controller.ts

✨ CONSEJO DE CLAUDE:
CRITICO - Violación del principio de responsabilidad única (SRP)...

   ❌ CRITICO: Corrige SOLID/Bugs
```

### Ejemplo 3: Tests fallidos

```
🔔 CAMBIO EN: auth.service.ts
   ✅ Arquitectura aprobada.
🧪 Ejecutando Jest para: test/auth/auth.spec.ts
   ❌ Tests fallaron

🔍 ¿Quieres que Claude analice el error? (s/n, timeout 15s): s

🔍 Analizando fallo en tests...
💡 SOLUCIÓN SUGERIDA:
El problema está en que el método `validateUser` no está manejando...
```

## Arquitectura

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
┌─────────────────┐
│  Git Commit     │ (preguntar_commit)
└─────────────────┘
```

### Componentes principales

| Componente | Descripción |
|------------|-------------|
| `consultar_claude()` | Comunicación con API de Claude AI (Anthropic) |
| `analizar_arquitectura()` | Evaluación de código basada en SOLID y Clean Code |
| `ejecutar_tests()` | Ejecución de tests de Jest relacionados |
| `pedir_ayuda_test()` | Diagnóstico de fallos con IA |
| `generar_mensaje_commit()` | Generación de mensajes siguiendo Conventional Commits |
| `preguntar_commit()` | Flujo interactivo de commits con timeout |

## Archivos generados

### `.suggested` files

Cuando Claude analiza un archivo, genera una versión mejorada guardada como:

```
users.service.ts.suggested
```

Este archivo contiene el código refactorizado siguiendo las recomendaciones de Claude.

## Troubleshooting

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
- Revisa que el watcher esté activo (no pausado)

### Tests no se ejecutan

- Verifica que existe el archivo de test correspondiente en `test/module/file.spec.ts`
- Asegúrate de que `npm run test` funciona en tu proyecto
- Verifica que Jest esté configurado correctamente en tu proyecto NestJS

### Commits no se crean

- Verifica que tienes git inicializado en el proyecto
- Asegúrate de tener permisos de escritura
- Revisa que no haya hooks de git bloqueando el commit

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

- [ ] Soporte para otros frameworks (Angular, React, Vue)
- [ ] Configuración personalizable mediante archivo `.sentinelrc`
- [ ] Integración con otros runners de tests (Vitest, Mocha)
- [ ] Métricas y reportes de análisis de código
- [ ] Modo daemon/servicio en segundo plano
- [ ] Soporte para múltiples proyectos simultáneos

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
