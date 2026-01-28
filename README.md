# Sentinel

Herramienta de monitoreo de archivos escrita en Rust que analiza cambios de codigo usando Claude AI y gestiona el flujo de trabajo con Git. Diseñada para integrarse con proyectos NestJS/TypeScript como asistente de desarrollo.

## Funcionalidades

- Monitoreo en tiempo real del sistema de archivos (directorio `src/`)
- Analisis automatico de codigo con Claude AI (principios SOLID, Clean Code, buenas practicas NestJS)
- Deteccion y ejecucion de tests con Jest
- Flujo interactivo de commits en Git con timeout de 30 segundos
- Generacion de sugerencias de codigo guardadas en archivos `.suggested`
- Mecanismo de pausa mediante archivo `.sentinel-pause`

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- Credenciales de la API de Anthropic

## Variables de entorno

| Variable | Descripcion |
|----------|-------------|
| `ANTHROPIC_AUTH_TOKEN` | API key de Anthropic |
| `ANTHROPIC_BASE_URL` | URL base de la API (ej. `https://api.anthropic.com`) |

## Instalacion

```bash
git clone https://github.com/<tu-usuario>/sentinel-rust.git
cd sentinel-rust
cargo build --release
```

## Uso

```bash
export ANTHROPIC_AUTH_TOKEN="tu-token"
export ANTHROPIC_BASE_URL="https://api.anthropic.com"
cargo run
```

Sentinel comenzara a monitorear el directorio `src/` del proyecto objetivo. Al detectar cambios en archivos `.ts`:

1. Envia el codigo a Claude para analisis
2. Si no hay problemas criticos, ejecuta los tests relacionados con Jest
3. Si los tests pasan, ofrece hacer commit automaticamente

Para pausar el monitoreo, crea el archivo `.sentinel-pause` en el directorio del proyecto objetivo.

## Dependencias

| Crate | Uso |
|-------|-----|
| `notify` | Monitoreo del sistema de archivos |
| `reqwest` | Cliente HTTP para la API de Claude |
| `serde` / `serde_json` | Serializacion JSON |
| `anyhow` | Manejo de errores |
| `colored` | Salida con colores en terminal |

## Autor

Sergio Guadarrama - sguadarrama@tiprotec.com
