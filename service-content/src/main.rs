mod config_http;
mod graph;
mod http;
mod taxonomy;

use graph::{GraphEntity, GraphStore, LbugGraphStore};
use notify::{Event, RecursiveMode, Result as NotifyResult, Watcher};
use std::fs;
use std::path::Path;
use std::sync::Arc;
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

    // Ontology directory: service-content/ontology/ relative to the binary's parent,
    // or overridden via SERVICE_CONTENT_ONTOLOGY_DIR.
    let ontology_dir = std::env::var("SERVICE_CONTENT_ONTOLOGY_DIR").unwrap_or_else(|_| {
        // Default: sibling ontology/ directory relative to the binary location.
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .map(|p| p.join("ontology").to_string_lossy().to_string())
            .unwrap_or_else(|| "ontology".to_string())
    });

    println!("[SYSTEM] Doorman endpoint: {}", doorman_endpoint);
    println!("[SYSTEM] Base dir: {}", base_dir);
    println!("[SYSTEM] Module ID: {}", module_id);
    println!("[SYSTEM] Ontology dir: {}", ontology_dir);

    let corpus_dir = format!("{}/service-content/ledgers", base_dir);
    let crm_dir = format!("{}/service-people/ledgers", base_dir);

    if !Path::new(&corpus_dir).exists() { fs::create_dir_all(&corpus_dir).unwrap(); }
    if !Path::new(&crm_dir).exists() { fs::create_dir_all(&crm_dir).unwrap(); }

    // ── Graph store initialisation ────────────────────────────────────────────
    let graph_dir = std::env::var("SERVICE_CONTENT_GRAPH_DIR")
        .unwrap_or_else(|_| format!("{}/service-content/graph", base_dir));
    fs::create_dir_all(&graph_dir).unwrap();
    let graph_db_path = format!("{}/entities.lbug", graph_dir);

    let graph_store: Arc<dyn GraphStore> = Arc::new(
        LbugGraphStore::new(&graph_db_path)
            .expect("[SYSTEM] Failed to open LadybugDB graph store"),
    );
    graph_store.init_schema().expect("[SYSTEM] Failed to initialise graph schema");
    println!("[SYSTEM] Graph store ready: {}", graph_db_path);

    // ── Startup taxonomy load ─────────────────────────────────────────────────
    match taxonomy::load_taxonomy_from_dir(&ontology_dir) {
        Ok(bundle) => {
            let entities = taxonomy::bundle_to_entities(&bundle);
            let total = entities.len();
            match graph_store.upsert_entities("__taxonomy__", &entities) {
                Ok(n) => println!(
                    "[TAXONOMY] Loaded: {} archetypes, {} coa-profiles, {} domains, \
                     {} glossary-terms, {} themes, {} topics, {} guides → {} entities upserted",
                    bundle.archetypes.len(),
                    bundle.coa.len(),
                    bundle.domains.len(),
                    bundle.glossary.len(),
                    bundle.themes.len(),
                    bundle.topics.len(),
                    bundle.guides.len(),
                    n
                ),
                Err(e) => println!("[TAXONOMY] Graph write failed: {}", e),
            }
            let _ = total;
        }
        Err(e) => println!("[TAXONOMY] Load failed (non-fatal): {}", e),
    }

    // ── HTTP server (dedicated thread + own tokio runtime) ───────────────────
    // Cannot use reqwest::blocking inside a #[tokio::main] context (nested
    // runtime panic). Keep main synchronous; HTTP server owns its own runtime.
    let http_bind = std::env::var("SERVICE_CONTENT_HTTP_BIND")
        .unwrap_or_else(|_| "127.0.0.1:9081".to_string());
    let graph_for_http = Arc::clone(&graph_store);
    let doorman_for_http = doorman_endpoint.clone();
    let ontology_for_http = ontology_dir.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to build HTTP tokio runtime");
        rt.block_on(http::run_server(graph_for_http, http_bind, doorman_for_http, ontology_for_http));
    });

    // ── Process any pre-existing CORPUS_* files ───────────────────────────────
    let mut processed_ledgers: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(Path::new(&corpus_dir)) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                if filename.starts_with("CORPUS_") {
                    let _ = process_corpus(&path, &crm_dir, &doorman_endpoint, &module_id, &graph_store);
                    processed_ledgers.push(filename);
                }
            }
        }
    }

    // ── Watcher loop (blocking — runs on the main task) ───────────────────────
    // std::sync::mpsc is fine here; recv() blocks the main async task's thread
    // but the HTTP server lives on a separate tokio worker thread.
    let (tx, rx) = std::sync::mpsc::channel();
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
                                processed_ledgers.push(filename.clone());
                                if !process_corpus(&path, &crm_dir, &doorman_endpoint, &module_id, &graph_store) {
                                    println!("  -> [WATCHER] Extraction failed for {} — skipping until restart.", filename);
                                }
                            }
                        }
                    }
                }
            }
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

fn process_corpus(
    filepath: &Path,
    crm_dir: &str,
    doorman_endpoint: &str,
    module_id: &str,
    graph_store: &Arc<dyn GraphStore>,
) -> bool {
    let content = match fs::read_to_string(filepath) { Ok(c) => c, Err(_) => return false };
    let payload: Value = match serde_json::from_str(&content) { Ok(v) => v, Err(_) => return false };

    let worm_id = payload["worm_id"].as_str().unwrap_or("UNKNOWN");
    let corpus_text = payload["corpus"].as_str().unwrap_or("");
    // Per-file module_id override: CORPUS JSON may carry a "module_id" field to
    // route workspace artifacts into a separate graph namespace (e.g. "foundry-workspace")
    // without requiring a separate service-content instance.
    let effective_module_id: &str = payload["module_id"]
        .as_str()
        .filter(|s| !s.is_empty())
        .unwrap_or(module_id);

    if corpus_text.is_empty() { return false; }

    println!("  -> [WATCHER] Routing payload to Doorman ({})/v1/chat/completions...", doorman_endpoint);

    let system_prompt = "You are a semantic entity extractor for a real estate property management archive. \
        Given a corpus of text, extract all named entities as a JSON array. \
        Each object must have these fields: \
        entity_name (string), \
        classification (string: Person|Company|Project|Account|Location), \
        role_vector (string or null), \
        location_vector (string or null), \
        contact_vector (string: email address or phone number or null). \
        Return ONLY a valid JSON array with no explanation or markdown.";

    // JSON Schema enforced by Yo-Yo #2 (Graph Extractor, H100/Llama 3.3 70B)
    // via Doorman grammar substrate (PS.3 Tier B path).
    let entity_schema = serde_json::json!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "entity_name": {"type": "string"},
                "classification": {
                    "type": "string",
                    "enum": ["Person", "Company", "Project", "Account", "Location"]
                },
                "role_vector": {"type": ["string", "null"]},
                "location_vector": {"type": ["string", "null"]},
                "contact_vector": {"type": ["string", "null"]}
            },
            "required": ["entity_name", "classification"]
        }
    });

    let body = serde_json::json!({
        "model": "local",
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": corpus_text}
        ],
        "temperature": 0.1,
        "max_tokens": 2048,
        "grammar": {"type": "json-schema", "value": entity_schema}
    });

    let url = format!("{}/v1/chat/completions", doorman_endpoint);
    let client = reqwest::blocking::Client::new();
    let res = client.post(&url)
        .header("X-Foundry-Module-ID", effective_module_id)
        .header("X-Foundry-Complexity", "high")
        .header("X-Foundry-Yoyo-Label", "graph")
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
                        let mut graph_entities: Vec<GraphEntity> = Vec::new();

                        for ent in &semantic_entities {
                            let entity_name = ent["entity_name"].as_str().unwrap_or("").to_string();
                            let classification = ent["classification"].as_str().unwrap_or("").to_string();
                            let role_vector = ent.get("role_vector")
                                .and_then(|v| v.as_str())
                                .filter(|s| !s.is_empty() && *s != "null")
                                .map(str::to_string);
                            let location_vector = ent.get("location_vector")
                                .and_then(|v| v.as_str())
                                .filter(|s| !s.is_empty() && *s != "null")
                                .map(str::to_string);
                            let contact_vector = ent.get("contact_vector")
                                .and_then(|v| v.as_str())
                                .filter(|s| !s.is_empty() && *s != "null")
                                .map(str::to_string);

                            // Build GraphEntity for the graph write path
                            graph_entities.push(GraphEntity {
                                entity_name: entity_name.clone(),
                                classification: classification.clone(),
                                role_vector: role_vector.clone(),
                                location_vector: location_vector.clone(),
                                contact_vector: contact_vector.clone(),
                                module_id: effective_module_id.to_string(),
                                confidence: 0.95,
                            });

                            // Build the legacy JSON CRM record
                            let mut new_ent = serde_json::Map::new();
                            new_ent.insert("entity_name".to_string(), serde_json::json!(entity_name));
                            new_ent.insert("classification".to_string(), serde_json::json!(classification));
                            new_ent.insert("role_vector".to_string(),
                                role_vector.as_deref().map(|s| serde_json::json!(s))
                                    .unwrap_or(serde_json::json!("UNVERIFIED")));
                            new_ent.insert("confidence".to_string(), serde_json::json!(0.95));
                            new_ent.insert("context_anchor".to_string(), serde_json::json!("SLM NEURAL INFERENCE"));

                            let loc = location_vector.as_deref().map(|s| serde_json::json!(s))
                                .unwrap_or(serde_json::json!("UNVERIFIED"));
                            new_ent.insert("location_vector".to_string(), loc);

                            let mut latent = Vec::new();
                            if let Some(contact) = contact_vector.as_deref() {
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

                        // ── Graph write path ──────────────────────────────────
                        if let Err(e) = graph_store.upsert_entities(effective_module_id, &graph_entities) {
                            println!("  -> [GRAPH] Write failed: {}", e);
                            return false;
                        } else {
                            println!("  -> [GRAPH] {} entities written to graph (module: {}).", graph_entities.len(), effective_module_id);
                            return true;
                        }
                    } else {
                        println!("  -> [SYS_HALT] Doorman response was not a valid entity JSON array.");
                        return false;
                    }
                } else {
                    println!("  -> [SYS_HALT] Doorman returned invalid JSON format.");
                    return false;
                }
            } else {
                println!("  -> [SYS_HALT] Doorman rejected payload: {}", response.status());
                return false;
            }
        }
        Err(e) => {
            println!("  -> [SYS_HALT] Doorman routing failed: {}", e);
            return false;
        }
    }
}
