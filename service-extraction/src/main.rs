use notify::{Watcher, RecursiveMode, Result as NotifyResult, Event};
use std::path::Path;
use std::sync::mpsc::channel;
use std::fs;
use std::thread;
use std::time::Duration;
use serde_json::Value;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STD};
use mailparse::{parse_mail, MailHeaderMap};
use regex::Regex;
use std::collections::HashSet;

#[derive(serde::Serialize, Clone)]
struct ExtractedEntity {
    entity_name: String,
    classification: String,
    role_vector: String,
    confidence: f32,
    context_anchor: String,
    location_vector: String,
    latent_vectors: Vec<String>,
}

fn main() -> NotifyResult<()> {
    println!("================================================================");
    println!("[SYSTEM] PointSav Cryptographic Router (Dumb Vault Mode)");
    println!("[SYSTEM] Protocol: Consuming Edge Wasm Intelligence...");
    println!("================================================================");

    let base_dir = std::env::var("EXTRACTION_BASE_DIR")
        .unwrap_or_else(|_| "/home/mathew/deployments/woodfine-fleet-deployment".to_string());
    let watch_dir = std::env::var("EXTRACTION_WATCH_DIR")
        .unwrap_or_else(|_| format!("{}/cluster-totebox-personnel-1/service-fs/data/service-people/source", base_dir));
    // Optional: emit CORPUS_*.json for service-content DataGraph ingestion
    let corpus_emit_dir = std::env::var("EXTRACTION_EMIT_CORPUS_DIR").ok();
    // Optional: set module_id in emitted CORPUS JSON (falls back to SERVICE_CONTENT_MODULE_ID env var in service-content)
    let corpus_module_id = std::env::var("EXTRACTION_CORPUS_MODULE_ID").ok();

    println!("[SYSTEM] Base dir: {}", base_dir);
    println!("[SYSTEM] Watch dir: {}", watch_dir);
    if let Some(dir) = &corpus_emit_dir {
        println!("[SYSTEM] Corpus emit dir: {} (module_id: {})", dir,
            corpus_module_id.as_deref().unwrap_or("(from service-content env)"));
    }

    if !Path::new(&watch_dir).exists() { fs::create_dir_all(&watch_dir).unwrap(); }

    let mut processed_ledgers: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(Path::new(&watch_dir)) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                if process_payload(&path, &base_dir, corpus_emit_dir.as_deref(), corpus_module_id.as_deref()) {
                    processed_ledgers.push(filename);
                }
            }
        }
    }

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(&watch_dir), RecursiveMode::NonRecursive)?;

    println!("================================================================");
    println!("[SYSTEM] Active Surveillance Engaged: {}", watch_dir);

    loop {
        match rx.recv() {
            Ok(Ok(Event { paths, .. })) => {
                for path in paths {
                    if let Some(extension) = path.extension() {
                        if extension == "json" {
                            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                            if !processed_ledgers.contains(&filename) {
                                thread::sleep(Duration::from_millis(250));
                                if process_payload(&path, &base_dir, corpus_emit_dir.as_deref(), corpus_module_id.as_deref()) {
                                    processed_ledgers.push(filename);
                                }
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

fn process_payload(
    filepath: &Path,
    base_dir: &str,
    corpus_emit_dir: Option<&str>,
    corpus_module_id: Option<&str>,
) -> bool {
    let content = match fs::read_to_string(filepath) { Ok(c) => c, Err(_) => return false };
    let payload: Value = match serde_json::from_str(&content) { Ok(v) => v, Err(_) => return false };

    let file_obj = &payload["file"];
    let original_filename = file_obj["filename"].as_str().unwrap_or("unknown_asset");
    let base64_data = file_obj["data"].as_str().unwrap_or("");

    let b64_str = if let Some(idx) = base64_data.find(',') { &base64_data[idx + 1..] } else { base64_data };
    let raw_bytes = match BASE64_STD.decode(b64_str) { Ok(b) => b, Err(_) => return false };

    let dest_archive = payload["destination_archive"].as_str().unwrap_or("cluster-totebox-personnel-1");
    let target_service = payload["target_service"].as_str().unwrap_or("service-people");
    let worm_id = filepath.file_stem().unwrap().to_str().unwrap();

    let parsed_mail = match parse_mail(&raw_bytes) { Ok(m) => m, Err(_) => return false };
    let headers = parsed_mail.get_headers();
    let sender = headers.get_first_value("From").unwrap_or_else(|| "Unknown".to_string());

    let mut graph_entities: Vec<ExtractedEntity> = Vec::new();
    let mut seen_names = HashSet::new();
    let mut corpus_parts: Vec<String> = Vec::new();

    corpus_parts.push(format!("Document: {}", original_filename));

    // 1. PURE CRYPTOGRAPHIC ORIGIN ANCHORING
    let re_sender = Regex::new(r#"(?i)"?([^"(<]+)(?:\(([^)]+)\))?"?\s*<([^>]+)>"#).unwrap();
    if let Some(caps) = re_sender.captures(&sender) {
        let raw_name = caps.get(1).map_or("", |m| m.as_str()).trim().to_string();
        let name = raw_name.replace('"', "");
        if !name.is_empty() {
            corpus_parts.push(format!("From: {}", sender));
            seen_names.insert(name.clone());
            graph_entities.push(ExtractedEntity {
                entity_name: name,
                classification: "ORIGIN SENDER".to_string(),
                role_vector: "Cryptographic Anchor".to_string(),
                confidence: 1.0,
                context_anchor: format!("HEADER: {}", sender),
                location_vector: "UNVERIFIED".to_string(),
                latent_vectors: vec![],
            });
        }
    }

    if let Some(subject) = headers.get_first_value("Subject") {
        corpus_parts.push(format!("Subject: {}", subject));
    }

    if let Ok(body) = parsed_mail.get_body() {
        let trimmed = body.trim().to_string();
        if !trimmed.is_empty() {
            corpus_parts.push(trimmed);
        }
    }

    // 2. EDGE AI INGESTION (Trusting the WebAssembly payload blindly)
    if let Some(edge_entities) = payload.get("edge_entities").and_then(|v| v.as_array()) {
        for ent in edge_entities {
            let name = ent["entity_name"].as_str().unwrap_or("").trim().to_string();
            let class = ent["classification"].as_str().unwrap_or("UNKNOWN").to_string();
            let conf = ent["confidence"].as_f64().unwrap_or(0.9) as f32;

            if name.len() > 2 && !seen_names.contains(&name) {
                seen_names.insert(name.clone());
                corpus_parts.push(format!("{}: {}", class, name));
                graph_entities.push(ExtractedEntity {
                    entity_name: name,
                    classification: class,
                    role_vector: "Edge AI Inference".to_string(),
                    confidence: conf,
                    context_anchor: "WASM LOCAL MATRIX".to_string(),
                    location_vector: "UNVERIFIED".to_string(),
                    latent_vectors: vec![],
                });
            }
        }
    }

    let write_ledger = |service: &str, suffix: &str, content: &str| {
        let dir = format!("{}/{}/service-fs/data/{}/ledgers", base_dir, dest_archive, service);
        fs::create_dir_all(&dir).unwrap();
        fs::write(format!("{}/{}_{}.json", dir, suffix, worm_id), content).unwrap();
    };

    if !graph_entities.is_empty() {
        let people_ledger = serde_json::json!({
            "worm_id": worm_id,
            "source_asset": original_filename,
            "extracted_crm_entities": graph_entities,
        });
        write_ledger(target_service, "CRM", &people_ledger.to_string());
        println!("  -> [VAULT] Successfully secured {} entities evaluated by Edge AI.", graph_entities.len());
    }

    // ── CORPUS bridge ─────────────────────────────────────────────────────────
    // When EXTRACTION_EMIT_CORPUS_DIR is set, write a CORPUS_*.json alongside
    // the CRM ledger. service-content watches this dir and feeds the text to
    // Doorman for grammar-constrained entity extraction into LadybugDB.
    if let Some(emit_dir) = corpus_emit_dir {
        let corpus_text = corpus_parts.join("\n");
        if !corpus_text.is_empty() {
            let mut corpus_json = serde_json::json!({
                "worm_id": worm_id,
                "corpus": corpus_text,
            });
            if let Some(mid) = corpus_module_id {
                corpus_json["module_id"] = serde_json::json!(mid);
            }
            let out_path = format!("{}/CORPUS_{}.json", emit_dir, worm_id);
            match fs::write(&out_path, corpus_json.to_string()) {
                Ok(_) => println!("  -> [CORPUS] Emitted CORPUS_{}.json for DataGraph ingestion.", worm_id),
                Err(e) => println!("  -> [CORPUS] Write failed ({}): {}", out_path, e),
            }
        }
    }

    true
}
