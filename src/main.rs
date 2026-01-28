use notify::{Watcher, RecursiveMode, Event, EventKind};
use std::process::Command;
use std::path::Path;
use std::fs;
use std::io::{self, Write}; // Para forzar el vaciado de la terminal
use reqwest::blocking::Client;
use serde_json::json;
use std::thread;
use std::time::Duration;
use colored::*;

fn analizar_con_claude(codigo: &str, file_name: &str) -> anyhow::Result<bool> {
    let api_key = std::env::var("ANTHROPIC_AUTH_TOKEN").expect("❌ No se encontró la variable ANTHROPIC_AUTH_TOKEN");
    let base_url = std::env::var("ANTHROPIC_BASE_URL").expect("❌ No se encontró la variable ANTHROPIC_BASE_URL");

    let client = Client::new();
    let url = format!("{}/v1/messages", base_url.trim_end_matches('/'));

    io::stdout().flush()?;

    let response = client.post(&url)
        .header("x-api-key", api_key)
        .header("content-type", "application/json")
        .json(&json!({
            "model": "claude-opus-4-5-20251101",
            "max_tokens": 1024,
            "messages": [{
                "role": "user", 
                "content": format!(
                    "Actúa como un Arquitecto de Software experto en NestJS y Clean Code. 
                    Analiza el archivo {}.
                    
                    Tu análisis debe enfocarse en:
                    1. SOLID: ¿Se está rompiendo el Principio de Responsabilidad Única (SRP)?
                    2. CLEAN CODE: ¿Hay funciones muy largas, nombres poco claros o 'magic numbers'?
                    3. NESTJS BEST PRACTICES: ¿El servicio está demasiado acoplado o faltan DTOs?

                    REGLAS DE RESPUESTA:
                    - Si hay un bug que romperá la app o una violación de SOLID grave, inicia con 'CRITICO'.
                    - Si el código es funcional pero puede mejorar en legibilidad, inicia con 'SEGURO' pero añade sugerencias.
                    - Sé breve, usa puntos (bullet points) y ve al grano.

                    Código:\n{}", 
                    file_name, codigo
                )
            }]
        }))
        .send()?;

    let body: serde_json::Value = response.json()?;
    
    // Si la API de Claude devuelve un error (ej: saldo insuficiente), saldrá aquí
    if let Some(text) = body["content"][0]["text"].as_str() {
        println!("\n✨ CONSEJO DE CLAUDE:\n{}", text);

        let sugerencia = extraer_codigo(text);
        let path_sugerido = format!("{}.suggested", file_name);
        fs::write(&path_sugerido, sugerencia)?;
        println!("\n✨ SUGERENCIA GUARDADA EN: {}", path_sugerido);

        if text.trim().to_uppercase().starts_with("CRITICO") {
            println!("   ❌ CRITICO: {}", text);
            return Ok(false);
        }
    }

    Ok(true)
}

fn ejecutar_tests(test_path: &str, project_path: &str) -> bool {
    println!("\n🧪 Ejecutando Jest para: {}", test_path);

    let args = [
        "run",
        "test",
        "--",
        "--findRelatedTests",
        test_path,
    ];

    match Command::new("npm")
        .args(&args)
        .current_dir(project_path)
        .status()
    {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

fn preguntar_commit(file_name: &str, project_path: &str) {
    print!("\n📝 ¿Quieres hacer commit de {}? (s/n, timeout 30s): ", file_name);
    io::stdout().flush().unwrap();

    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        let mut respuesta = String::new();
        if io::stdin().read_line(&mut respuesta).is_ok() {
            let _ = tx.send(respuesta.trim().to_string());
        }
    });

    // Esperar respuesta con timeout de 30s
    let respuesta = rx.recv_timeout(Duration::from_secs(30));

    if let Ok(r) = respuesta {
        if r == "s" || r == "S" {
            let commit_msg = format!("sentinel: {}", file_name);

            println!("   📦 Ejecutando git add...");
            let _ = Command::new("git")
                .args(&["add", "."])
                .current_dir(project_path)
                .status();

            println!("   💾 Ejecutando git commit...");
            match Command::new("git")
                .args(&["commit", "-m", &commit_msg])
                .current_dir(project_path)
                .status()
            {
                Ok(_) => println!("   ✅ Commit exitoso: {}", commit_msg),
                Err(e) => println!("   ❌ Error en git commit: {}", e),
            }
        } else {
            println!("   ⏭️  Commit cancelado");
        }
    } else {
        println!("   ⏭️  Timeout - Commit cancelado");
    }
}

fn extraer_codigo(texto: &str) -> String {
    if let Some(start) = texto.find("```typescript") {
        let resto = &texto[start + 13..];
        if let Some(end) = resto.find("```") {
            return resto[..end].trim().to_string();
        }
    }
    texto.to_string()
}

fn main() {
    let nest_project_path = "../../gestion-leads-operaciones-backend";
    let path_to_watch = format!("{}/src", nest_project_path);
    
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            if let EventKind::Modify(_) = event.kind {
                for path in event.paths {
                    if path.extension().map_or(false, |ext| ext == "ts") && 
                       !path.to_str().unwrap().contains(".spec.ts") &&
                       !path.to_str().unwrap().contains(".suggested") {
                        let _ = tx.send(path);
                    }
                }
            }
        }
    }).unwrap();

    watcher.watch(Path::new(&path_to_watch), RecursiveMode::Recursive).unwrap();
    println!("🛡️  Sentinel v2.5 Online.");

    let pause_file = format!("{}/.sentinel-pause", nest_project_path);

    for changed_path in rx {
        // Verificar si el sentinel está pausado
        if Path::new(&pause_file).exists() {
            continue;
        }

        let file_name = changed_path.file_name().unwrap().to_str().unwrap();

        // Solo procesar si existe el archivo de test correspondiente en test/
        let base_name = file_name.split('.').next().unwrap();
        let test_file_name = file_name.replace(".ts", ".spec.ts");
        let test_rel_path = format!("test/{}/{}", base_name, test_file_name);
        let test_file = Path::new(nest_project_path).join(&test_rel_path);
        if !test_file.exists() {
            println!("\n⏭️  IGNORADO (sin test): {}", file_name);
            continue;
        }

        println!("\n🔔 ARCHIVO DETECTADO: {}", file_name);

        // pausa estrategica de 100ms para esperar al SO
        thread::sleep(Duration::from_millis(100));

        // LEER ARCHIVO
        match fs::read_to_string(&changed_path) {
            Ok(codigo) => {
                if !codigo.trim().is_empty() {
                    match analizar_con_claude(&codigo, file_name) {
                        Ok(true) => {
                            println!("{}","   ✅ Todo bien, ejecutando tests...".green().bold());
                            if ejecutar_tests(&test_rel_path, nest_project_path) {
                                println!("{}","   ✅ Tests pasados".green().bold());
                                preguntar_commit(file_name, nest_project_path);
                            } else {
                                println!("{}","   ❌ Tests fallaron".red().bold());
                            }
                        },
                        Ok(false) => println!("{}","   ❌ CRITICO".red().bold()),
                        Err(e) => {
                            println!("   ❌ Error leyendo archivo: {}", e);
                            if ejecutar_tests(&test_rel_path, nest_project_path) {
                                println!("{}","   ✅ Tests pasados".green().bold());
                                preguntar_commit(file_name, nest_project_path);
                            } else {
                                println!("{}","   ❌ Tests fallaron".red().bold());
                            }
                        },
                    }
                }
            },
            Err(e) => println!("   ❌ Error leyendo archivo: {}", e),
        }
    }
}