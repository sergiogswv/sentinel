//! # Sentinel - AI-Powered Code Monitor
//!
//! Herramienta de monitoreo en tiempo real que vigila cambios en archivos TypeScript,
//! analiza el código con Claude AI, ejecuta tests y gestiona commits automáticamente.

use colored::*;
use config::SentinelConfig;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use stats::SentinelStats;
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Instant;

// Módulos
mod ai;
mod config;
mod docs;
mod git;
mod stats;
mod tests;
mod ui;

// --- MAIN ---

fn inicializar_sentinel(project_path: &Path) -> SentinelConfig {
    if let Some(config) = SentinelConfig::load(project_path) {
        println!(
            "{}",
            "🔄 Configuración cargada desde .sentinelrc.toml".green()
        );
        return config;
    }

    println!(
        "{}",
        "🚀 Configurando nuevo proyecto en Sentinel...".bright_cyan()
    );

    let gestor = SentinelConfig::detectar_gestor(project_path);
    let nombre = project_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let mut config = SentinelConfig::default(nombre, gestor);

    println!(
        "\n{}",
        "🤖 Configuración de Modelos AI".bright_magenta().bold()
    );

    // 1. Configurar Modelo Principal
    println!("\n--- MODELO PRINCIPAL ---");
    print!("👉 API Key: ");
    io::stdout().flush().unwrap();
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key).unwrap();
    config.primary_model.api_key = api_key.trim().to_string();

    print!("👉 URL [Enter para Anthropic]: ");
    io::stdout().flush().unwrap();
    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap();
    if !url.trim().is_empty() {
        config.primary_model.url = url.trim().to_string();
    }

    // Listar modelos si es Gemini
    if config.primary_model.url.contains("googleapis") {
        if let Ok(modelos) = ai::listar_modelos_gemini(&config.primary_model.api_key) {
            println!("{}", "📂 Modelos disponibles:".cyan());
            for (i, m) in modelos.iter().enumerate() {
                println!("{}. {}", i + 1, m);
            }
            print!("👉 Selecciona número: ");
            io::stdout().flush().unwrap();
            let mut sel = String::new();
            io::stdin().read_line(&mut sel).unwrap();
            if let Ok(idx) = sel.trim().parse::<usize>() {
                if idx > 0 && idx <= modelos.len() {
                    config.primary_model.name = modelos[idx - 1].clone();
                }
            }
        }
    }

    // 2. Configurar Modelo de Fallback (Opcional)
    println!("\n--- MODELO DE FALLBACK (Opcional) ---");
    print!("👉 ¿Configurar un modelo de respaldo por si falla el principal? (s/n): ");
    io::stdout().flush().unwrap();
    let mut use_fallback = String::new();
    io::stdin().read_line(&mut use_fallback).unwrap();

    if use_fallback.trim().to_lowercase() == "s" {
        let mut fb = config::ModelConfig::default();
        print!("👉 API Key: ");
        io::stdout().flush().unwrap();
        let mut ak = String::new();
        io::stdin().read_line(&mut ak).unwrap();
        fb.api_key = ak.trim().to_string();

        print!("👉 URL del modelo: ");
        io::stdout().flush().unwrap();
        let mut u = String::new();
        io::stdin().read_line(&mut u).unwrap();
        fb.url = u.trim().to_string();

        print!("👉 Nombre del modelo: ");
        io::stdout().flush().unwrap();
        let mut nm = String::new();
        io::stdin().read_line(&mut nm).unwrap();
        fb.name = nm.trim().to_string();

        config.fallback_model = Some(fb);
    }

    let _ = config.save(project_path);
    println!("{}", "✅ Configuración guardada.".green());
    config
}

fn main() {
    let project_path = ui::seleccionar_proyecto();
    if !project_path.exists() {
        std::process::exit(1);
    }

    let config = Arc::new(inicializar_sentinel(&project_path));
    let stats = Arc::new(Mutex::new(SentinelStats::cargar(&project_path)));

    let esta_pausado = Arc::new(Mutex::new(false));
    let pausa_loop = Arc::clone(&esta_pausado);
    let (tx, rx) = mpsc::channel::<PathBuf>();
    let (stdin_tx, stdin_rx) = mpsc::channel::<String>();
    let stdin_rx = Arc::new(Mutex::new(stdin_rx));
    let esperando_input = Arc::new(Mutex::new(false));

    // Hilo teclado
    let project_path_hilo = project_path.clone();
    let config_hilo = Arc::clone(&config);
    let stats_hilo = Arc::clone(&stats);
    let pausa_hilo = Arc::clone(&esta_pausado);
    let esperando_input_hilo = Arc::clone(&esperando_input);

    thread::spawn(move || {
        loop {
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                let cmd = input.trim().to_lowercase();
                if *esperando_input_hilo.lock().unwrap() {
                    let _ = stdin_tx.send(cmd);
                } else if cmd == "p" {
                    let mut p = pausa_hilo.lock().unwrap();
                    *p = !*p;
                    println!(
                        " ⌨️ SENTINEL: {}",
                        if *p {
                            "PAUSADO".yellow()
                        } else {
                            "ACTIVO".green()
                        }
                    );
                } else if cmd == "r" {
                    git::generar_reporte_diario(
                        &project_path_hilo,
                        &config_hilo,
                        Arc::clone(&stats_hilo),
                    );
                } else if cmd == "m" {
                    let s = stats_hilo.lock().unwrap();
                    println!(
                        "\n{}",
                        "📊 DASHBOARD DE RENDIMIENTO SENTINEL".bright_green().bold()
                    );
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!(
                        "🚫 Bugs Evitados:  {}",
                        s.bugs_criticos_evitados.to_string().red()
                    );
                    println!("💰 Costo Acumulado: ${:.4}", s.total_cost_usd);
                    println!("🎟️ Tokens Usados:   {}", s.total_tokens_used);
                    println!(
                        "⏳ Tiempo Ahorrado: {}h",
                        (s.tiempo_estimado_ahorrado_mins as f32 / 60.0)
                    );
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                } else if cmd == "c" {
                    SentinelConfig::abrir_en_editor(&project_path_hilo);
                } else if cmd == "x" {
                    print!("⚠️ ¿Reiniciar configuración? (s/n): ");
                    io::stdout().flush().unwrap();
                    let mut confirm = String::new();
                    if io::stdin().read_line(&mut confirm).is_ok()
                        && confirm.trim().to_lowercase() == "s"
                    {
                        let _ = SentinelConfig::eliminar(&project_path_hilo);
                        std::process::exit(0);
                    }
                }
            }
        }
    });

    // Watcher
    let config_watcher = Arc::clone(&config);
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            if let EventKind::Modify(_) = event.kind {
                for path in event.paths {
                    if !config_watcher.debe_ignorar(&path) {
                        let _ = tx.send(path);
                    }
                }
            }
        }
    })
    .unwrap();
    watcher
        .watch(&project_path.join("src"), RecursiveMode::Recursive)
        .unwrap();

    let leer_respuesta = move || -> Option<String> {
        *esperando_input.lock().unwrap() = true;
        let res = stdin_rx
            .lock()
            .unwrap()
            .recv_timeout(std::time::Duration::from_secs(30))
            .ok();
        *esperando_input.lock().unwrap() = false;
        res
    };

    println!(
        "\n{} {}",
        "🛡️ Sentinel activo en:".green(),
        project_path.display()
    );

    let mut ultimo_cambio: HashMap<PathBuf, Instant> = HashMap::new();
    while let Ok(changed_path) = rx.recv() {
        thread::sleep(std::time::Duration::from_millis(500));
        while rx.try_recv().is_ok() {}

        if *pausa_loop.lock().unwrap() {
            continue;
        }

        let ahora = Instant::now();
        if let Some(ultimo) = ultimo_cambio.get(&changed_path) {
            if ahora.duration_since(*ultimo) < std::time::Duration::from_secs(10) {
                continue;
            }
        }
        ultimo_cambio.insert(changed_path.clone(), ahora);

        let file_name = changed_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let base_name = file_name.split('.').next().unwrap();
        let test_rel_path = format!("test/{}/{}.spec.ts", base_name, base_name);

        if !project_path.join(&test_rel_path).exists() {
            continue;
        }

        println!("\n🔔 CAMBIO EN: {}", file_name.cyan().bold());

        if let Ok(codigo) = std::fs::read_to_string(&changed_path) {
            match ai::analizar_arquitectura(
                &codigo,
                &file_name,
                Arc::clone(&stats),
                &config,
                &project_path,
                &changed_path,
            ) {
                Ok(true) => {
                    if tests::ejecutar_tests(&test_rel_path, &project_path).is_ok() {
                        let _ = docs::actualizar_documentacion(
                            &codigo,
                            &changed_path,
                            &config,
                            Arc::clone(&stats),
                            &project_path,
                        );
                        let msg = git::generar_mensaje_commit(
                            &codigo,
                            &file_name,
                            &config,
                            Arc::clone(&stats),
                            &project_path,
                        );
                        println!("\n🚀 Mensaje: {}", msg.bright_cyan().bold());
                        print!("📝 ¿Commit? (s/n): ");
                        io::stdout().flush().unwrap();
                        if let Some(r) = leer_respuesta() {
                            git::preguntar_commit(&project_path, &msg, &r);
                        }
                    } else {
                        print!("\n🔍 ¿Ayuda con test? (s/n): ");
                        io::stdout().flush().unwrap();
                        if leer_respuesta().as_deref() == Some("s") {
                            let _ = tests::pedir_ayuda_test(
                                &codigo,
                                "Test falló",
                                &config,
                                Arc::clone(&stats),
                                &project_path,
                            );
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
