use notify::{Watcher, RecursiveMode, Result as NotifyResult, Event};
use std::path::Path;
use std::sync::mpsc::channel;
use std::fs;
use std::thread;
use std::time::Duration;
use serde_json::Value;

fn main() -> NotifyResult<()> {
    println!("================================================================");
    println!("[SYSTEM] PointSav Semantic Watcher (Rust Edition) Activated");
    println!("[SYSTEM] Protocol: Schema Expansion Routing");
    println!("================================================================");

    let doorman_endpoint = std::env::var("SLM_DOORMAN_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:9080".to_string());
    let base_dir = std::env::var("SERVICE_CONTENT_BASE_DIR")
        .unwrap_or_else(|_| "/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-personnel-1/service-fs/data".to_string());
    let module_id = std::env::var("SERVICE_CONTENT_MODULE_ID")
        .unwrap_or_else(|_| "woodfine".to_string());

    println!("[SYSTEM] Doorman endpoint: {}", doorman_endpoint);
    println!("[SYSTEM] Base dir: {}", base_dir);
    println!("[SYSTEM] Module ID: {}", module_id);

    let corpus_dir = format!("{}/service-content/ledgers", base_dir);
    let crm_dir = format!("{}/service-people/ledgers", base_dir);

    if !Path::new(&corpus_dir).exists() { fs::create_dir_all(&corpus_dir).unwrap(); }
    if !Path::new(&crm_dir).exists() { fs::create_dir_all(&crm_dir).unwrap(); }

    let mut processed_ledgers: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(Path::new(&corpus_dir)) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                if filename.starts_with("CORPUS_") {
                    process_corpus(&path, &crm_dir, &doorman_endpoint, &module_id);
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
                                process_corpus(&path, &crm_dir, &doorman_endpoint, &module_id);
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

fn process_corpus(filepath: &Path, crm_dir: &str, doorman_endpoint: &str, module_id: &str) {
    let content = match fs::read_to_string(filepath) { Ok(c) => c, Err(_) => return, };
    let payload: Value = match serde_json::from_str(&content) { Ok(v) => v, Err(_) => return, };

    let worm_id = payload["worm_id"].as_str().unwrap_or("UNKNOWN");
    let corpus_text = payload["corpus"].as_str().unwrap_or("");

    if corpus_text.is_empty() { return; }

    println!("  -> [WATCHER] Routing payload to Doorman ({})/v1/chat/completions...", doorman_endpoint);

    let request_id = format!("sc-{}-{}", worm_id,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos());

    let system_prompt = "You are a semantic entity extractor for a real estate property management archive. \
        Given a corpus of text, extract all named entities as a JSON array. \
        Each object must have these fields: \
        entity_name (string), \
        classification (string: Person|Company|Project|Account|Location), \
        role_vector (string or null), \
        location_vector (string or null), \
        contact_vector (string: email address or phone number or null). \
        Return ONLY a valid JSON array with no explanation or markdown.";

    let body = serde_json::json!({
        "model": "local",
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": corpus_text}
        ],
        "temperature": 0.1,
        "max_tokens": 2048
    });

    let url = format!("{}/v1/chat/completions", doorman_endpoint);
    let client = reqwest::blocking::Client::new();
    let res = client.post(&url)
        .header("X-Foundry-Module-ID", module_id)
        .header("X-Foundry-Request-ID", &request_id)
        .header("X-Foundry-Complexity", "medium")
        .json(&body)
        .timeout(Duration::from_secs(300))
        .send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(completion) = response.json::<serde_json::Value>() {
                    let content = completion["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap_or("");
                    // Strip markdown code fences if present
                    let content = content.trim();
                    let content = content.strip_prefix("```json").unwrap_or(content);
                    let content = content.strip_prefix("```").unwrap_or(content);
                    let content = content.strip_suffix("```").unwrap_or(content);
                    let content = content.trim();

                    if let Ok(semantic_entities) = serde_json::from_str::<Vec<Value>>(content) {
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
                        println!("  -> [SYS_HALT] Doorman response was not a valid entity JSON array.");
                    }
                } else {
                    println!("  -> [SYS_HALT] Doorman returned invalid JSON format.");
                }
            } else {
                println!("  -> [SYS_HALT] Doorman rejected payload: {}", response.status());
            }
        },
        Err(e) => {
            println!("  -> [SYS_HALT] Doorman routing failed: {}", e);
        }
    }
}
