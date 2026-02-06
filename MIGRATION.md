# Migración de Configuración - Sentinel v4.4.2

## 🐛 Problema Resuelto

En versiones anteriores de Sentinel (v4.4.1 y anteriores), existía un bug crítico donde:

- Al hacer cambios en el proyecto, la configuración no se leía correctamente
- La aplicación pedía reconfigurar el proyecto en lugar de usar la configuración existente
- No había compatibilidad hacia adelante con nuevas versiones
- Los usuarios perdían tiempo reconfigurando API keys y preferencias

## ✅ Solución Implementada

La versión v4.4.2 implementa un sistema robusto de **versionado y migración automática** de configuraciones.

### Características Principales

1. **Campo `version` en configuración**: Cada archivo `.sentinelrc.toml` ahora incluye su versión
2. **Migración automática**: Detecta configs antiguas y las actualiza sin intervención del usuario
3. **Preservación de datos**: API keys y configuraciones personalizadas se mantienen intactas
4. **Validación con defaults**: Campos faltantes se completan con valores apropiados
5. **Versión dinámica**: La versión se lee desde `Cargo.toml` en tiempo de compilación

---

## 📋 Estructura de Configuración

### Configuración v4.4.2 (Actual)

```toml
version = "4.4.2"  # ← NUEVO: Campo de versión
project_name = "mi-proyecto"
framework = "React"
manager = "npm"
test_command = "npm run test"
architecture_rules = ["Clean Code", "SOLID Principles"]
file_extensions = ["js", "ts", "jsx"]
ignore_patterns = ["node_modules", "dist"]

[primary_model]
name = "claude-opus-4-5-20251101"
url = "https://api.anthropic.com"
api_key = "sk-ant-..."

[primary_model]
use_cache = true
```

### Configuración v4.4.1 (Antigua - Sin campo version)

```toml
project_name = "mi-proyecto"
framework = "React"
manager = "npm"
test_command = "npm run test"
# ... resto de campos sin 'version'
```

---

## 🔄 Proceso de Migración

### Flujo de Carga

```
┌─────────────────────────────┐
│  Iniciar Sentinel v4.4.2    │
└──────────────┬──────────────┘
               │
               ▼
┌─────────────────────────────┐
│  ¿Existe .sentinelrc.toml?  │
└──────────────┬──────────────┘
               │
     ┌─────────┴─────────┐
     │                   │
    NO                  SI
     │                   │
     ▼                   ▼
┌──────────┐   ┌─────────────────────┐
│ Crear   │   │  ¿Tiene campo       │
│ nueva   │   │  'version'?         │
│ config  │   └──────────┬──────────┘
└──────────┘              │
                  ┌───────┴───────┐
                  │               │
                 NO              SI
                  │               │
                  ▼               ▼
           ┌──────────────┐   ┌──────────────┐
           │ Migrar       │   │ ¿Versión =   │
           │ config v4.4.1│   │ 4.4.2?       │
           │ a v4.4.2     │   └──────┬───────┘
           └──────┬───────┘          │
                  │        ┌─────────┴─────────┐
                  │        │                   │
                  │       NO                  SI
                  │        │                   │
                  │        ▼                   │
                  │   ┌─────────┐              │
                  │   │ Migrar  │              │
                  │   │ a 4.4.2 │              │
                  │   └────┬────┘              │
                  │        │                   │
                  └────────┴───────────────────┘
                           │
                           ▼
                  ┌────────────────┐
                  │ Retornar       │
                  │ Configuración  │
                  │ Actualizada    │
                  └────────────────┘
```

### Detalles de la Migración

#### 1. Detección de Configuración Antigua

Si el archivo `.sentinelrc.toml` no tiene el campo `version`, se considera una configuración de v4.4.1 o anterior.

#### 2. Preservación de Datos Sensibles

La migración preserva estos campos críticos:
- ✅ `primary_model.api_key` - API key principal
- ✅ `fallback_model.api_key` - API key de respaldo (si existe)
- ✅ `test_command` - Comando personalizado de tests
- ✅ `ignore_patterns` - Patrones personalizados
- ✅ `use_cache` - Preferencia de caché
- ✅ `fallback_model` - Configuración completa de fallback

#### 3. Completar Campos Faltantes

Si faltan campos en la configuración antigua, se usan valores por defecto:

```rust
// Si no hay extensiones configuradas
if config.file_extensions.is_empty() {
    config.file_extensions = vec!["js".to_string(), "ts".to_string()];
}

// Si no hay comando de test
if config.test_command.is_empty() {
    config.test_command = format!("{} run test", config.manager);
}

// Si no hay reglas de arquitectura
if config.architecture_rules.is_empty() {
    config.architecture_rules = vec![
        "Clean Code".to_string(),
        "SOLID Principles".to_string(),
        "Best Practices".to_string(),
    ];
}
```

#### 4. Actualización de Versión

```rust
config.version = SENTINEL_VERSION.to_string(); // "4.4.2"
```

#### 5. Guardado Automático

La configuración migrada se guarda automáticamente en `.sentinelrc.toml` con el nuevo formato.

---

## 📝 Ejemplos de Migración

### Ejemplo 1: Migración Completa

**Antes (v4.4.1):**
```toml
project_name = "mi-app"
framework = "React"
manager = "npm"
test_command = "npm run test"
architecture_rules = ["Clean Code", "SOLID Principles"]
file_extensions = ["js", "ts", "jsx"]
ignore_patterns = ["node_modules", "dist"]

[primary_model]
name = "claude-opus-4-5-20251101"
url = "https://api.anthropic.com"
api_key = "sk-ant-api03-..."
```

**Después (v4.4.2):**
```toml
version = "4.4.2"  # ← AGREGADO
project_name = "mi-app"
framework = "React"
manager = "npm"
test_command = "npm run test"
architecture_rules = ["Clean Code", "SOLID Principles"]
file_extensions = ["js", "ts", "jsx"]
ignore_patterns = ["node_modules", "dist"]

[primary_model]
name = "claude-opus-4-5-20251101"
url = "https://api.anthropic.com"
api_key = "sk-ant-api03-..."  # ← PRESERVADO
```

### Ejemplo 2: Configuración con Campos Faltantes

**Antes (config incompleta):**
```toml
project_name = "otra-app"
framework = "TypeScript"

[primary_model]
name = "gemini-2.0-flash-exp"
url = "https://generativelanguage.googleapis.com"
api_key = "AIzaSy..."
```

**Después (completada):**
```toml
version = "4.4.2"  # ← AGREGADO
project_name = "otra-app"
framework = "TypeScript"
manager = "npm"  # ← COMPLETADO (detectado)
test_command = "npm run test"  # ← COMPLETADO (default)
architecture_rules = ["Clean Code", "SOLID Principles", "Best Practices"]  # ← COMPLETADO
file_extensions = ["js", "ts"]  # ← COMPLETADO (default)
ignore_patterns = ["node_modules", "dist", ".git", "build", ".next"]  # ← COMPLETADO

[primary_model]
name = "gemini-2.0-flash-exp"
url = "https://generativelanguage.googleapis.com"
api_key = "AIzaSy..."  # ← PRESERVADO

use_cache = true  # ← COMPLETADO (default)
```

---

## 🎯 Mensajes de Migración

### Configuración Antigua Detectada

```
🔄 Detectada configuración antigua, migrando...
   ✅ Configuración migrada exitosamente
```

### Actualización de Versión

```
   🔄 Migrando configuración de versión 4.4.1 a 4.4.2...
   ✅ Configuración migrada exitosamente
```

### No se Pudo Cargar

```
   ⚠️  No se pudo cargar la configuración. Se creará una nueva.
```

---

## 🧪 Testing de Migración

### Pasos para Probar la Migración

1. **Backup tu configuración actual:**
   ```bash
   cp .sentinelrc.toml .sentinelrc.toml.backup
   ```

2. **Simula una configuración antigua:**
   ```bash
   # Eliminar el campo 'version' del archivo
   sed -i '/^version =/d' .sentinelrc.toml
   ```

3. **Ejecuta Sentinel v4.4.2:**
   ```bash
   ./target/release/sentinel-rust
   ```

4. **Verifica:**
   - Deberías ver el mensaje de migración
   - El archivo `.sentinelrc.toml` debe tener `version = "4.4.2"`
   - Tu API key debe seguir ahí
   - Tus configuraciones personalizadas deben mantenerse

5. **Restaura el backup si es necesario:**
   ```bash
   cp .sentinelrc.toml.backup .sentinelrc.toml
   ```

---

## 🚀 Beneficios

| Beneficio | Descripción |
|-----------|-------------|
| ✅ **Sin pérdida de configuración** | API keys y settings personalizados se preservan |
| ✅ **Migración automática** | No requiere intervención del usuario |
| ✅ **Validación robusta** | Campos faltantes se completan automáticamente |
| ✅ **Compatibilidad hacia adelante** | Funcionará con futuras versiones |
| ✅ **Transparencia** | Se muestra mensaje cuando se migra una config |
| ✅ **Single source of truth** | La versión está solo en `Cargo.toml` |

---

## 🔧 Detalles Técnicos

### Versión Dinámica

```rust
// config.rs
pub const SENTINEL_VERSION: &str = env!("CARGO_PKG_VERSION");
```

Esto lee la versión desde `Cargo.toml` en tiempo de compilación:

```toml
# Cargo.toml
[package]
name = "sentinel-rust"
version = "4.4.2"  # ← ÚNICA fuente de verdad
```

### Función de Carga

```rust
pub fn load(path: &Path) -> Option<Self> {
    let content = fs::read_to_string(&config_path).ok()?;

    // Intenta deserializar como config actual
    if let Ok(mut config) = toml::from_str::<SentinelConfig>(&content) {
        if config.version != SENTINEL_VERSION {
            // Migrar si la versión es diferente
            config = Self::migrar_config(config, path);
            let _ = config.save(path);
        }
        return Some(config);
    }

    // Si falla, intenta como config antigua
    if let Ok(old_config) = toml::from_str::<SentinelConfigV1>(&content) {
        // Migrar a formato nuevo
        return Some(Self::migrar_config_v1(old_config, path));
    }

    None
}
```

---

## 📚 Compatibilidad de Versiones Futuras

El sistema de migración permite:

1. **Actualizar configs automáticamente** cuando se lanza una nueva versión
2. **Preservar siempre los datos sensibles** (API keys, preferencias)
3. **Mantener compatibilidad** con versiones anteriores
4. **Evitar que los usuarios tengan que reconfigurar** en cada actualización

### Ejemplo de Migración Futura (v4.5.0)

```rust
// En v4.5.0, si se agregan nuevos campos
fn migrar_config(mut config: SentinelConfig, _path: &Path) -> SentinelConfig {
    config.version = SENTINEL_VERSION.to_string();

    // Nuevos campos en v4.5.0
    if config.nuevo_campo.is_none() {
        config.nuevo_campo = Some(valor_por_defecto);
    }

    config
}
```

---

## ❓ Preguntas Frecuentes

### ¿Perderé mi API key al actualizar?

**No.** La migración preserva todas las API keys y configuraciones personalizadas.

### ¿Tengo que hacer algo manualmente?

**No.** La migración es completamente automática y transparente.

### ¿Puedo seguir usando una config antigua?

**Sí, pero se migrará automáticamente** la primera vez que ejecutes Sentinel v4.4.2.

### ¿Qué pasa si mi configuración está corrupta?

Si el archivo `.sentinelrc.toml` no se puede leer, Sentinel mostrará un mensaje y creará una nueva configuración.

### ¿La versión se actualiza en cada compilación?

**No.** La versión se lee desde `Cargo.toml` en tiempo de compilación. Solo cambia cuando actualizas `Cargo.toml`.

---

## 📖 Referencias

- **[CHANGELOG.md](CHANGELOG.md)** - Historial completo de cambios
- **[docs/configuration.md](docs/configuration.md)** - Guía de configuración detallada
- **[Cargo.toml](Cargo.toml)** - Fuente única de la versión

---

**Última actualización:** 2025-02-05
**Versión:** 4.4.2
