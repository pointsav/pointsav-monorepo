use notify::{Watcher, RecursiveMode, Result as NotifyResult, Event};
use std::path::Path;
use std::sync::mpsc::channel;
use std::fs;
use std::thread;
use std::time::Duration;
use serde_json::Value;

const BASE_DIR: &str = "/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-personnel-1/service-fs/data";
const SLM_ENDPOINT: &str = "http://127.0.0.1:8082/api/semantic-extract";

fn main() -> NotifyResult<()> {
    println!("================================================================");
    println!("[SYSTEM] PointSav Semantic Watcher (Rust Edition) Activated");
    println!("[SYSTEM] Protocol: Schema Expansion Routing");
    println!("================================================================");

    let corpus_dir = format!("{}/service-content/ledgers", BASE_DIR);
    let crm_dir = format!("{}/service-people/ledgers", BASE_DIR);

    if !Path::new(&corpus_dir).exists() { fs::create_dir_all(&corpus_dir).unwrap(); }
    if !Path::new(&crm_dir).exists() { fs::create_dir_all(&crm_dir).unwrap(); }

    let mut processed_ledgers: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(Path::new(&corpus_dir)) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                if filename.starts_with("CORPUS_") {
                    process_corpus(&path, &crm_dir);
                    processed_ledgers.push(filename);
                }
            }
        }
    }

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(&corpus_dir), RecursiveMode::NonRecursive)?;

    println!("================================================================");
    println!("[SYSTEM] Active Kernel Surveillance Engaged on Corpus Plane...");
    
    loop {
        match rx.recv() {
            Ok(Ok(Event { paths, .. })) => {
                for path in paths {
                    if let Some(extension) = path.extension() {
                        if extension == "json" {
                            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                            if filename.starts_with("CORPUS_") && !processed_ledgers.contains(&filename) {
                                println!("\n[WATCHER] New Corpus Detected: {}", filename);
                                thread::sleep(Duration::from_millis(250)); 
                                process_corpus(&path, &crm_dir);
                                processed_ledgers.push(filename);
                            }
                        }
                    }
                }
            },
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

fn process_corpus(filepath: &Path, crm_dir: &str) {
    let content = match fs::read_to_string(filepath) { Ok(c) => c, Err(_) => return, };
    let payload: Value = match serde_json::from_str(&content) { Ok(v) => v, Err(_) => return, };

    let worm_id = payload["worm_id"].as_str().unwrap_or("UNKNOWN");
    let corpus_text = payload["corpus"].as_str().unwrap_or("");

    if corpus_text.is_empty() { return; }

    println!("  -> [WATCHER] Routing payload to SLM Core (Port 8082)...");
    
    let client = reqwest::blocking::Client::new();
    let mut map = std::collections::HashMap::new();
    map.insert("corpus", corpus_text);

    let res = client.post(SLM_ENDPOINT)
        .json(&map)
        .timeout(Duration::from_secs(120))
        .send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(semantic_entities) = response.json::<Vec<Value>>() {
                    let mut enriched_crm = Vec::new();
                    
                    for ent in semantic_entities {
                        let mut new_ent = serde_json::Map::new();
                        new_ent.insert("entity_name".to_string(), ent["entity_name"].clone());
                        new_ent.insert("classification".to_string(), ent["classification"].clone());
                        new_ent.insert("role_vector".to_string(), ent.get("role_vector").cloned().unwrap_or(serde_json::json!("UNVERIFIED")));
                        new_ent.insert("confidence".to_string(), serde_json::json!(0.95));
                        new_ent.insert("context_anchor".to_string(), serde_json::json!("SLM NEURAL INFERENCE"));
                        
                        // Catch the new Location Vector
                        let loc = ent.get("location_vector").cloned().unwrap_or(serde_json::json!("UNVERIFIED"));
                        new_ent.insert("location_vector".to_string(), loc);
                        
                        // Catch the Contact Vector and push to Latent Vectors array
                        let mut latent = Vec::new();
                        let contact = ent.get("contact_vector").and_then(|v| v.as_str()).unwrap_or("UNVERIFIED");
                        if contact != "UNVERIFIED" && !contact.is_empty() {
                            if contact.contains('@') {
                                latent.push(format!("mailto:{}", contact));
                            } else {
                                latent.push(format!("tel:{}", contact));
                            }
                        }
                        new_ent.insert("latent_vectors".to_string(), serde_json::json!(latent));
                        
                        enriched_crm.push(Value::Object(new_ent));
                    }

                    let semantic_ledger = serde_json::json!({
                        "worm_id": format!("{}_SEMANTIC", worm_id),
                        "source_asset": "SLM_INFERENCE",
                        "extracted_crm_entities": enriched_crm
                    });

                    let out_file = format!("{}/SEMANTIC_{}.json", crm_dir, worm_id);
                    fs::write(&out_file, semantic_ledger.to_string()).unwrap();
                    println!("  -> [WATCHER] Semantic Integration Complete: {} Nodes Secured.", enriched_crm.len());
                } else {
                    println!("  -> [SYS_HALT] SLM Core returned invalid JSON format.");
                }
            } else {
                println!("  -> [SYS_HALT] SLM Core rejected payload: {}", response.status());
            }
        },
        Err(e) => {
            println!("  -> [SYS_HALT] SLM Routing failed: {}", e);
        }
    }
}
