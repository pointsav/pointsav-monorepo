mod config_http;
mod graph;
mod http;
mod taxonomy;

use graph::{GraphEntity, GraphStore, LbugGraphStore};
use notify::{Event, RecursiveMode, Watcher};
use std::fs;
use std::io::{BufRead, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use serde_json::Value;
use tracing::{error, info, warn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Structured JSON logging — RUST_LOG controls filter (default: info).
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!(service = "service-content", "PointSav Semantic Watcher activated");

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

    info!(doorman_endpoint, base_dir, module_id, ontology_dir, "startup configuration");

    let corpus_dir = format!("{}/service-content/ledgers", base_dir);
    let crm_dir = format!("{}/service-people/ledgers", base_dir);

    if !Path::new(&corpus_dir).exists() {
        fs::create_dir_all(&corpus_dir)?;
    }
    if !Path::new(&crm_dir).exists() {
        fs::create_dir_all(&crm_dir)?;
    }

    // ── Graph store initialisation ────────────────────────────────────────────
    let graph_dir = std::env::var("SERVICE_CONTENT_GRAPH_DIR")
        .unwrap_or_else(|_| format!("{}/service-content/graph", base_dir));
    fs::create_dir_all(&graph_dir)?;
    let graph_db_path = format!("{}/entities.lbug", graph_dir);

    let graph_store: Arc<dyn GraphStore> = Arc::new(
        LbugGraphStore::new(&graph_db_path)
            .expect("[SYSTEM] Failed to open LadybugDB graph store"),
    );
    graph_store.init_schema().expect("[SYSTEM] Failed to initialise graph schema");
    info!(graph_db_path, "graph store ready");

    // ── Processed-ledger persistence ─────────────────────────────────────────
    // STATE_DIR defaults to graph_dir so the JSONL lives alongside the graph DB.
    // Override with SERVICE_CONTENT_STATE_DIR.
    // Each line is the filename of a successfully-processed CORPUS_*.json file.
    // Files not present in this list are retried on the next restart.
    let state_dir = std::env::var("SERVICE_CONTENT_STATE_DIR")
        .unwrap_or_else(|_| graph_dir.clone());
    fs::create_dir_all(&state_dir)?;
    let processed_ledgers_path = Path::new(&state_dir).join("processed_ledgers.jsonl");
    let mut processed_ledgers = load_processed_ledgers(&processed_ledgers_path);
    info!(
        count = processed_ledgers.len(),
        path = %processed_ledgers_path.display(),
        "loaded processed ledger entries"
    );

    // ── Startup taxonomy load ─────────────────────────────────────────────────
    match taxonomy::load_taxonomy_from_dir(&ontology_dir) {
        Ok(bundle) => {
            let entities = taxonomy::bundle_to_entities(&bundle);
            let total = entities.len();
            match graph_store.upsert_entities("__taxonomy__", &entities) {
                Ok(n) => info!(
                    archetypes = bundle.archetypes.len(),
                    domains = bundle.domains.len(),
                    entities_upserted = n,
                    "taxonomy loaded"
                ),
                Err(e) => error!(error = %e, "taxonomy graph write failed"),
            }
            let _ = total;
        }
        Err(e) => warn!(error = %e, "taxonomy load failed (non-fatal)"),
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
    if let Ok(entries) = fs::read_dir(Path::new(&corpus_dir)) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                if filename.starts_with("CORPUS_") && !processed_ledgers.contains(&filename) {
                    if process_corpus(&path, &crm_dir, &doorman_endpoint, &module_id, &graph_store) {
                        append_processed_ledger(&processed_ledgers_path, &filename);
                    }
                    // Always push to in-memory list to prevent same-session re-triggers.
                    // Failed files are not in the JSONL, so they will retry on next restart.
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

    info!(corpus_dir, "corpus watcher active");

    loop {
        match rx.recv() {
            Ok(Ok(Event { paths, .. })) => {
                for path in paths {
                    if let Some(extension) = path.extension() {
                        if extension == "json" {
                            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                            if filename.starts_with("CORPUS_") && !processed_ledgers.contains(&filename) {
                                info!(corpus_file = %filename, "new corpus detected");
                                thread::sleep(Duration::from_millis(250));
                                processed_ledgers.push(filename.clone());
                                if process_corpus(&path, &crm_dir, &doorman_endpoint, &module_id, &graph_store) {
                                    append_processed_ledger(&processed_ledgers_path, &filename);
                                } else {
                                    warn!(corpus_file = %filename, "extraction failed — will retry on next restart");
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

/// Returns true if `s` is a valid per-file module_id override:
/// non-empty, ≤64 chars, only lowercase ASCII letters / digits / hyphens.
/// The reserved __ prefix is checked separately before calling this.
fn validate_module_id(s: &str) -> bool {
    !s.is_empty() && s.len() <= 64 && s.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '-'))
}

/// Load the set of already-processed CORPUS filenames from the sidecar JSONL.
/// Returns an empty Vec if the file does not exist or cannot be read — a missing
/// file is not an error; it just means all CORPUS files will be processed.
fn load_processed_ledgers(path: &Path) -> Vec<String> {
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    std::io::BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

/// Append one filename to the sidecar JSONL. Non-fatal on failure.
fn append_processed_ledger(path: &Path, filename: &str) {
    match fs::OpenOptions::new().create(true).append(true).open(path) {
        Ok(mut f) => {
            if let Err(e) = writeln!(f, "{}", filename) {
                eprintln!("[SYSTEM] Warning: could not append to {}: {}", path.display(), e);
            }
        }
        Err(e) => eprintln!("[SYSTEM] Warning: could not open {} for append: {}", path.display(), e),
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

    // Per-file module_id validation.
    // Absent or empty → use process-level module_id (trusted from env var).
    // Present and invalid → reject the file to prevent taxonomy-namespace injection.
    let effective_module_id: String = match payload["module_id"].as_str().filter(|s| !s.is_empty()) {
        None => module_id.to_string(),
        Some(s) if s.starts_with("__") => {
            warn!(corpus_file = %filepath.display(), module_id = s, "rejecting: reserved __ prefix");
            return false;
        }
        Some(s) if !validate_module_id(s) => {
            warn!(corpus_file = %filepath.display(), module_id = s, "rejecting: invalid module_id format");
            return false;
        }
        Some(s) => s.to_string(),
    };

    if corpus_text.is_empty() { return false; }

    // Sprint 1: write Source node before calling Doorman.
    // Graph grows regardless of Ring 3 (Doorman/Tier B) reachability.
    let source_node = GraphEntity {
        entity_name: worm_id.to_string(),
        classification: "Source".to_string(),
        role_vector: None,
        location_vector: None,
        contact_vector: None,
        module_id: effective_module_id.clone(),
        confidence: 1.0,
    };
    if let Err(e) = graph_store.upsert_entities(&effective_module_id, &[source_node]) {
        warn!(module_id = %effective_module_id, worm_id, error = %e, "source node write failed (non-fatal)");
    } else {
        info!(module_id = %effective_module_id, worm_id, "source node written");
    }

    info!(module_id = %effective_module_id, doorman_endpoint, "routing to Doorman /v1/extract");

    // POST /v1/extract — Tier B only (route_yoyo_only). Doorman returns
    // {deferred: true} when Tier B is unavailable instead of falling back
    // to Tier A. This prevents the KV-cache bloat and retry storm that
    // caused VM crashes when Tier B was down for extended periods.
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
        "text": corpus_text,
        "schema": entity_schema,
        "module_id": effective_module_id
    });

    let url = format!("{}/v1/extract", doorman_endpoint);
    let client = reqwest::blocking::Client::new();
    let res = client.post(&url)
        .json(&body)
        .timeout(Duration::from_secs(300))
        .send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(extract_resp) = response.json::<serde_json::Value>() {
                    // Tier B unavailable — graceful defer, no retry this session.
                    // File is not written to processed_ledgers JSONL; next boot retries.
                    if extract_resp["deferred"].as_bool().unwrap_or(false) {
                        let reason = extract_resp["defer_reason"].as_str().unwrap_or("unknown");
                        warn!(defer_reason = reason, "extraction deferred — tier B unavailable; will retry next boot");
                        return true;
                    }

                    if extract_resp["extraction_ok"].as_bool().unwrap_or(false) {
                        let semantic_entities = extract_resp["entities"]
                            .as_array()
                            .cloned()
                            .unwrap_or_default();

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
                                module_id: effective_module_id.clone(),
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
                        if let Err(e) = fs::write(&out_file, semantic_ledger.to_string()) {
                            error!(out_file, error = %e, "failed to write semantic ledger");
                            return false;
                        }
                        info!(entities = enriched_crm.len(), module_id = %effective_module_id, "semantic integration complete");

                        if let Err(e) = graph_store.upsert_entities(&effective_module_id, &graph_entities) {
                            error!(module_id = %effective_module_id, error = %e, "graph write failed");
                            return false;
                        } else {
                            info!(module_id = %effective_module_id, entities = graph_entities.len(), "entities written to graph");
                            return true;
                        }
                    } else {
                        error!(module_id = %effective_module_id, "extraction_ok false with no defer reason");
                        return false;
                    }
                } else {
                    error!("doorman returned invalid JSON");
                    return false;
                }
            } else {
                error!(status = %response.status(), "doorman rejected payload");
                return false;
            }
        }
        Err(e) => {
            error!(error = %e, "doorman routing failed");
            return false;
        }
    }
}
