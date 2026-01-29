# Estructura del Proyecto Sentinel

## Organización de Módulos

El proyecto ha sido refactorizado en módulos especializados para mejorar la mantenibilidad y claridad del código.

### Módulos

```
src/
├── main.rs        # Punto de entrada y loop principal del watcher
├── ai.rs          # Comunicación con Claude AI
├── git.rs         # Operaciones de Git
├── tests.rs       # Ejecución y diagnóstico de tests
├── docs.rs        # Generación de documentación
└── ui.rs          # Interfaz de usuario
```

## Descripción de Módulos

### `main.rs`
**Responsabilidad**: Punto de entrada y orquestación principal

- Configuración del file watcher (notify)
- Loop principal de detección de cambios
- Coordinación entre módulos
- Gestión de hilos (pausa/reporte)
- Manejo de estado compartido (Arc/Mutex)

**Funciones**:
- `main()` - Punto de entrada principal

---

### `ai.rs`
**Responsabilidad**: Comunicación con Claude AI

**Funciones públicas**:
- `consultar_claude(prompt: String) -> Result<String>`
  - Realiza consultas al API de Anthropic
  - Variables de entorno: ANTHROPIC_AUTH_TOKEN, ANTHROPIC_BASE_URL

- `analizar_arquitectura(codigo: &str, file_name: &str) -> Result<bool>`
  - Analiza código TypeScript/NestJS
  - Evalúa SOLID, Clean Code y buenas prácticas
  - Genera archivos `.suggested` con código mejorado

- `extraer_codigo(texto: &str) -> String`
  - Extrae bloques de código TypeScript de respuestas de Claude
  - Busca delimitadores \`\`\`typescript...\`\`\`

**Dependencias**:
- `reqwest` - Cliente HTTP
- `serde_json` - Serialización JSON

---

### `git.rs`
**Responsabilidad**: Operaciones de Git

**Funciones públicas**:
- `obtener_resumen_git(project_path: &Path) -> String`
  - Obtiene commits del día (desde 00:00:00)
  - Ejecuta `git log --since=00:00:00`

- `generar_mensaje_commit(codigo: &str, file_name: &str) -> String`
  - Genera mensajes siguiendo Conventional Commits
  - Usa Claude AI para crear mensajes descriptivos

- `generar_reporte_diario(project_path: &Path)`
  - Analiza commits del día con Claude AI
  - Genera reporte dividido en: Logros, Aspectos Técnicos, Próximos Pasos
  - Guarda en `docs/DAILY_REPORT.md`

- `preguntar_commit(project_path: &Path, mensaje: &str)`
  - Flujo interactivo de commit con timeout de 30s
  - Ejecuta `git add .` y `git commit -m` si el usuario acepta

**Dependencias**:
- `crate::ai` - Para análisis con IA

---

### `tests.rs`
**Responsabilidad**: Ejecución y diagnóstico de tests

**Funciones públicas**:
- `ejecutar_tests(test_path: &str, project_path: &Path) -> Result<(), String>`
  - Ejecuta Jest con `npm run test -- --findRelatedTests`
  - Retorna Ok si tests pasan, Err con salida de error si fallan

- `pedir_ayuda_test(codigo: &str, error_jest: &str) -> Result<()>`
  - Solicita diagnóstico a Claude AI cuando tests fallan
  - Muestra solución sugerida al usuario

**Dependencias**:
- `crate::ai` - Para diagnóstico con IA

---

### `docs.rs`
**Responsabilidad**: Generación de documentación

**Funciones públicas**:
- `actualizar_documentacion(codigo: &str, file_path: &Path) -> Result<()>`
  - Genera "manuales de bolsillo" en formato Markdown
  - Resúmenes ultra-concisos (máximo 150 palabras)
  - Crea archivos .md junto a cada .ts modificado
  - Ejemplo: `src/users/users.service.ts` → `src/users/users.service.md`

**Dependencias**:
- `crate::ai` - Para generar resúmenes técnicos

---

### `ui.rs`
**Responsabilidad**: Interfaz de usuario en terminal

**Funciones públicas**:
- `seleccionar_proyecto() -> PathBuf`
  - Muestra menú interactivo de proyectos disponibles
  - Escanea directorio padre (`../`)
  - Retorna PathBuf del proyecto seleccionado

---

## Flujo de Datos

```
┌──────────────┐
│   main.rs    │
│  (watcher)   │
└──────┬───────┘
       │
       ├──▶ ui::seleccionar_proyecto()
       │
       ├──▶ ai::analizar_arquitectura()
       │
       ├──▶ tests::ejecutar_tests()
       │        └──▶ tests::pedir_ayuda_test() [si falla]
       │
       ├──▶ docs::actualizar_documentacion()
       │
       └──▶ git::generar_mensaje_commit()
            └──▶ git::preguntar_commit()

Comandos interactivos:
  'p' ──▶ Pausa/Reanuda
  'r' ──▶ git::generar_reporte_diario()
```

## Ventajas de esta Arquitectura

### Separación de Responsabilidades
Cada módulo tiene una responsabilidad clara y bien definida.

### Reusabilidad
Las funciones pueden ser reutilizadas en otros contextos o proyectos.

### Mantenibilidad
Es fácil localizar y modificar funcionalidades específicas.

### Testabilidad
Cada módulo puede ser testeado independientemente.

### Escalabilidad
Fácil agregar nuevas funcionalidades sin afectar código existente.

## Convenciones

### Visibilidad
- Funciones públicas: `pub fn` - Expuestas al resto del proyecto
- Funciones privadas: `fn` - Uso interno del módulo

### Documentación
- Comentarios de módulo: `//!` al inicio del archivo
- Comentarios de función: `///` antes de cada función pública
- Incluye: descripción, argumentos, retornos, efectos secundarios, ejemplos

### Imports
- Módulos internos: `use crate::nombre_modulo`
- Crates externos: `use nombre_crate`
- Agrupación por tipo (std, externos, internos)

## Próximos Pasos

Posibles mejoras a la arquitectura:

- [ ] Agregar módulo `config.rs` para configuración centralizada
- [ ] Crear módulo `errors.rs` con tipos de error personalizados
- [ ] Implementar traits para abstraer funcionalidades comunes
- [ ] Agregar tests unitarios para cada módulo
- [ ] Documentación con `cargo doc`
