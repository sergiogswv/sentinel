# Architecture

This document explains Sentinel's internal architecture, components, and system design.

## Main Flow (File Monitoring)

```
┌─────────────────┐
│  File Watcher   │ (notify crate)
└────────┬────────┘
         │ Detects change in .ts
         ▼
┌─────────────────┐
│ AI Analysis     │ (consultar_claude)
└────────┬────────┘
         │ Code approved
         ▼
┌─────────────────┐
│  Jest Tests     │ (ejecutar_tests)
└────────┬────────┘
         │ Tests pass
         ▼
┌──────────────────────┐
│ Auto-Documentation   │ (actualizar_documentacion)
│ Generate .md file    │
└────────┬─────────────┘
         ▼
┌─────────────────┐
│  Git Commit     │ (preguntar_commit)
└─────────────────┘
```

### Flow Description

1. **File Watcher**: Monitors `src/` directory for changes in `.ts` files
2. **AI Analysis**: Sends code to configured AI provider for architecture review
3. **Jest Tests**: If code is approved, runs corresponding test file
4. **Auto-Documentation**: If tests pass, generates markdown documentation
5. **Git Commit**: Prompts user to commit changes with AI-generated message

---

## Keyboard Thread (Centralized stdin)

```
┌─────────────────┐
│  User (stdin)   │  ← Single point for stdin reading
└────────┬────────┘
         │
         ├─ [waiting for input] ──▶ Forwards response to main loop (s/n)
         │
         ├─ 'p' ──▶ Pause/Resume
         │
         ├─ 'm' ──▶ Show metrics dashboard
         │
         ├─ 'l' ──▶ Clear cache
         │
         ├─ 'h'/'help' ──▶ Show help
         │
         ├─ 'x' ──▶ Reset configuration
         │
         └─ 'r' ──▶ ┌────────────────────┐
                    │ Daily Report       │
                    │ (generar_reporte_  │
                    │  diario)           │
                    └────────┬───────────┘
                             │
                             ├─▶ git log --since=00:00:00
                             │
                             ├─▶ AI Analysis
                             │
                             └─▶ docs/DAILY_REPORT.md
```

### Thread Safety

- **Single stdin reader**: Only one thread reads from stdin to prevent conflicts
- **Message passing**: Commands are sent between threads using channels
- **Atomic operations**: Pause state managed with atomic boolean

---

## Debounce and Event Draining

Sentinel implements an intelligent event handling system to avoid duplicate processing:

### Debounce Window

- Duplicate events from the same file are ignored within a **15-second window**
- This prevents reprocessing when editors generate multiple write events

### Event Draining

- After processing a file, all pending events in the channel are drained
- This clears the queue of redundant notifications
- Ensures the next event processed is genuinely new

### Implementation

```rust
// Simplified pseudocode
let mut last_processed = HashMap::new();

loop {
    let event = rx.recv();
    let now = Instant::now();

    // Check debounce
    if let Some(last_time) = last_processed.get(&file_path) {
        if now.duration_since(*last_time) < Duration::from_secs(15) {
            continue; // Skip duplicate
        }
    }

    // Process file
    process_file(&file_path);
    last_processed.insert(file_path.clone(), now);

    // Drain pending events
    while let Ok(_) = rx.try_recv() {
        // Discard
    }
}
```

---

## Main Components

| Component | Description |
|-----------|-------------|
| `consultar_ia_dinamico()` | Intelligent system with cache, fallback, and multi-provider support |
| `consultar_ia()` | Direct communication with AI APIs (Anthropic, Gemini, etc.) |
| `ejecutar_con_fallback()` | Executes query with primary model and automatic fallback |
| `listar_modelos_gemini()` | Retrieves list of available Gemini models |
| `analizar_arquitectura()` | Code evaluation based on SOLID and Clean Code |
| `ejecutar_tests()` | Jest test execution with visible console output |
| `pedir_ayuda_test()` | Diagnosis of failures with AI |
| `actualizar_documentacion()` | Generates ".md pocket manual" next to each .ts file |
| `generar_mensaje_commit()` | Generation of messages following Conventional Commits |
| `preguntar_commit()` | Executes commit if user accepts (receives response from main loop) |
| `obtener_resumen_git()` | Gets commits from the day using git log |
| `generar_reporte_diario()` | Creates productivity report with AI based on commits |
| `SentinelStats` | Management of persistent metrics and statistics |
| `SentinelConfig` | Project configuration (.sentinelrc.toml) |

---

## Component Details

### File Watcher (`notify` crate)

**Purpose:** Monitor file system for changes

**Features:**
- Watches `src/` directory recursively
- Filters for `.ts` files (excludes `.spec.ts`, `.suggested`)
- Sends events to processing channel

**Configuration:**
```rust
let (tx, rx) = channel();
let mut watcher = notify::recommended_watcher(tx)?;
watcher.watch(&src_path, RecursiveMode::Recursive)?;
```

---

### AI Analysis System

**Multi-Provider Architecture:**

```
consultar_ia_dinamico()
    │
    ├─▶ Check cache
    │   └─▶ If hit, return cached response
    │
    └─▶ ejecutar_con_fallback()
        │
        ├─▶ Try primary model
        │   └─▶ consultar_ia(primary_model)
        │
        └─▶ If fails, try fallback
            └─▶ consultar_ia(fallback_model)
```

**Cache System:**
- Hash-based key generation (file content + prompt)
- Stored in `.sentinel/cache/`
- Automatic invalidation on content change

**Fallback Logic:**
1. Attempt primary model
2. On failure (timeout, error, rate limit):
   - Log failure reason
   - Automatically switch to fallback
   - Continue workflow seamlessly

---

### Test Execution

**Process:**
1. Determine test file path (`src/module/file.ts` → `test/module/file.spec.ts`)
2. Execute `npm run test -- <test-file>` with Jest
3. Stream output to console in real-time
4. Parse exit code for pass/fail status

**Features:**
- Real-time console output
- 30-second timeout
- Error capture for AI diagnosis

---

### Documentation Generation

**When:** After tests pass successfully

**Process:**
1. Read modified `.ts` file
2. Send to AI with documentation prompt
3. Generate ultra-concise "pocket manual" (max 150 words)
4. Save as `.md` file in same directory

**Content:**
- Summary of functionality
- List of main methods
- Timestamp of last update

---

### Git Integration

**Commit Message Generation:**
- Follows Conventional Commits format
- Based on file changes and AI analysis
- Examples: `feat:`, `fix:`, `refactor:`

**Interactive Flow:**
- 30-second timeout for user response
- Auto-skip on timeout
- Confirmation before executing commit

---

### Metrics System

**Tracked Data:**
```json
{
  "bugs_criticos_evitados": 12,
  "sugerencias_aplicadas": 8,
  "tests_fallidos_corregidos": 3,
  "total_analisis": 45,
  "tiempo_estimado_ahorrado_mins": 390,
  "total_cost_usd": 0.4523,
  "total_tokens_used": 45230
}
```

**Persistence:**
- Stored in `.sentinel_stats.json`
- Updated after each analysis
- Accumulated across sessions

**Display:**
- Real-time dashboard (command 'm')
- Formatted output with emojis

---

## Concurrency Model

### Threads

1. **Main Thread**: File watching and processing
2. **Keyboard Thread**: stdin reading and command handling
3. **Test Execution Thread**: Spawned for each test run
4. **AI Query Thread**: Async runtime (Tokio)

### Communication

- **Channels**: For event passing between threads
- **Atomic Booleans**: For pause state
- **Shared State**: Configuration and stats (protected by locks)

### Error Handling

- **Anyhow**: For error propagation
- **Graceful degradation**: Fallback on AI failures
- **User feedback**: Clear error messages with suggested actions

---

## File Structure

```
sentinel-rust/
├── src/
│   ├── main.rs           # Entry point, main loop
│   ├── ai.rs             # AI provider communication
│   ├── config.rs         # Configuration management
│   ├── stats.rs          # Metrics tracking
│   ├── tests.rs          # Test execution
│   ├── git.rs            # Git operations
│   └── docs.rs           # Documentation generation
├── target/
│   └── release/
│       └── sentinel-rust # Compiled binary
└── .sentinel/
    └── cache/            # AI response cache
```

---

## Data Flow Example

**User saves `users.service.ts`:**

```
1. File Watcher detects change
   └─▶ Debounce check (15s window)
       └─▶ Event passes

2. Read file content
   └─▶ Generate cache key (hash)
       └─▶ Check cache
           ├─▶ Cache hit: Use cached response
           └─▶ Cache miss: Query AI

3. AI Analysis
   └─▶ Send to primary model (Claude Opus)
       ├─▶ Success: Parse response
       └─▶ Failure: Try fallback (Gemini Flash)

4. Evaluate Response
   ├─▶ CRITICO: Stop, show warning
   └─▶ SEGURO: Continue to tests

5. Execute Tests
   └─▶ Run test/users/users.spec.ts
       ├─▶ Pass: Continue to docs
       └─▶ Fail: Offer AI diagnosis

6. Generate Documentation
   └─▶ Create users.service.md

7. Prompt for Commit
   └─▶ Generate commit message
       └─▶ Wait for user input (30s timeout)
           ├─▶ 's': Execute commit
           ├─▶ 'n': Skip
           └─▶ Timeout: Skip

8. Update Metrics
   └─▶ Save to .sentinel_stats.json

9. Drain Event Queue
   └─▶ Ready for next change
```

---

## Performance Optimizations

### Cache System
- **Hit rate**: Typically 40-70% for repeated analyses
- **Storage**: Minimal (compressed JSON)
- **Invalidation**: Automatic on file change

### Debounce
- **Reduces API calls**: By 50-80% during active editing
- **Window**: 15 seconds (configurable)

### Async Operations
- **Non-blocking**: AI queries don't block file watching
- **Concurrent**: Multiple operations can run in parallel

---

**Navigation:**
- [← Previous: Troubleshooting](troubleshooting.md)
- [Next: Examples →](examples.md)
