mod config_http;
mod graph;
mod http;
mod taxonomy;

use graph::{GraphEntity, GraphStore, LbugGraphStore};
use notify::{Event, RecursiveMode, Result as NotifyResult, Watcher};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::sync::mpsc::RecvTimeoutError;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
enum ExtractResult {
    Success,
    DeferTransient,
    DeferCircuitOpen,
    Failed,
}

/// Config for the rate-limited Tier A extraction fallback (Sprint 3B).
/// Active only when `SERVICE_CONTENT_TIER_A_FALLBACK_ENABLED=true`.
/// When the Yo-Yo circuit is open, the WATCHER can attempt extraction via
/// Tier A (/v1/chat/completions with JSON schema grammar constraint) at most
/// once per `interval_secs`. Quality is degraded vs Tier B but prevents
/// WATCHER stalling indefinitely.
#[derive(Clone)]
struct TierAFallbackConfig {
    enabled: bool,
    interval_secs: u64,
}

fn main() -> NotifyResult<()> {
    println!("================================================================");
    println!("[SYSTEM] PointSav Semantic Watcher (Rust Edition) Activated");
    println!("[SYSTEM] Protocol: Schema Expansion Routing");
    println!("================================================================");

    let doorman_endpoint = std::env::var("SLM_DOORMAN_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:9080".to_string());
    let infrastructure_root =
        std::env::var("INFRASTRUCTURE_ROOT").unwrap_or_else(|_| "/srv/foundry".to_string());
    let base_dir = std::env::var("SERVICE_CONTENT_BASE_DIR")
        .unwrap_or_else(|_| format!("{}/data", infrastructure_root));
    let module_id =
        std::env::var("SERVICE_CONTENT_MODULE_ID").unwrap_or_else(|_| "woodfine".to_string());

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

    let tier_a_fallback = TierAFallbackConfig {
        enabled: std::env::var("SERVICE_CONTENT_TIER_A_FALLBACK_ENABLED")
            .map(|v| matches!(v.trim(), "true" | "1"))
            .unwrap_or(false),
        interval_secs: std::env::var("SERVICE_CONTENT_TIER_A_FALLBACK_INTERVAL_SECS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(300),
    };
    let mut last_tier_a_attempt: Option<Instant> = None;

    println!("[SYSTEM] Doorman endpoint: {}", doorman_endpoint);
    println!("[SYSTEM] Base dir: {}", base_dir);
    println!("[SYSTEM] Module ID: {}", module_id);
    println!("[SYSTEM] Ontology dir: {}", ontology_dir);
    if tier_a_fallback.enabled {
        println!(
            "[SYSTEM] Tier A fallback enabled — interval {}s (degraded quality)",
            tier_a_fallback.interval_secs
        );
    }

    let corpus_dir = format!("{}/service-content/ledgers", base_dir);
    let crm_dir = format!("{}/service-people/ledgers", base_dir);

    if !Path::new(&corpus_dir).exists() {
        fs::create_dir_all(&corpus_dir).unwrap();
    }
    if !Path::new(&crm_dir).exists() {
        fs::create_dir_all(&crm_dir).unwrap();
    }

    // ── Graph store initialisation ────────────────────────────────────────────
    let graph_dir = std::env::var("SERVICE_CONTENT_GRAPH_DIR")
        .unwrap_or_else(|_| format!("{}/service-content/graph", base_dir));
    fs::create_dir_all(&graph_dir).unwrap();
    let graph_db_path = format!("{}/entities.lbug", graph_dir);

    let graph_store: Arc<dyn GraphStore> = Arc::new(
        LbugGraphStore::new(&graph_db_path).expect("[SYSTEM] Failed to open LadybugDB graph store"),
    );
    graph_store
        .init_schema()
        .expect("[SYSTEM] Failed to initialise graph schema");
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
    let http_bind =
        std::env::var("SERVICE_CONTENT_HTTP_BIND").unwrap_or_else(|_| "127.0.0.1:9081".to_string());
    let graph_for_http = Arc::clone(&graph_store);
    let doorman_for_http = doorman_endpoint.clone();
    let ontology_for_http = ontology_dir.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to build HTTP tokio runtime");
        rt.block_on(http::run_server(
            graph_for_http,
            http_bind,
            doorman_for_http,
            ontology_for_http,
        ));
    });

    // ── SC-3: Doorman startup health-check ───────────────────────────────────
    // Poll /healthz for up to 30 s before the first drain. If Doorman is
    // unreachable at boot, all CORPUS files would be deferred and lost until
    // restart. A brief wait here avoids that silent data-loss pattern.
    {
        let health_url = format!("{}/healthz", doorman_endpoint);
        let client = reqwest::blocking::Client::new();
        let mut wait_secs = 1u64;
        let deadline = std::time::Instant::now() + Duration::from_secs(30);
        let mut ready = false;
        while std::time::Instant::now() < deadline {
            match client
                .get(&health_url)
                .timeout(Duration::from_secs(2))
                .send()
            {
                Ok(r) if r.status().is_success() => {
                    ready = true;
                    break;
                }
                _ => {}
            }
            thread::sleep(Duration::from_secs(wait_secs));
            wait_secs = (wait_secs * 2).min(8);
        }
        if ready {
            println!("[SYSTEM] Doorman ready.");
        } else {
            println!("[SYSTEM] Warning: Doorman unreachable after 30 s — CORPUS drain will proceed (files may defer to retry queue).");
        }
    }

    // ── Process any pre-existing CORPUS_* files ───────────────────────────────
    // processed_ledgers: permanently done (success, or a hard parse/extract failure
    //   that retrying cannot fix)
    // deferred_ledgers: transient defer — retried every 30 s by the watcher timeout
    // circuit_deferred_ledgers: deferred because the Tier B (Yo-Yo) circuit is open.
    //   Held dormant — NOT retried every tick (that would storm the Doorman with the
    //   whole backlog during a GPU stockout). A single recovery probe per tick drains
    //   this list back into deferred_ledgers once Tier B is reachable again, so the
    //   backlog resumes WITHOUT a service restart. (Preemption-safe: nothing skipped.)
    let mut processed_ledgers: Vec<String> = Vec::new();
    let mut deferred_ledgers: Vec<String> = Vec::new();
    let mut circuit_deferred_ledgers: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(Path::new(&corpus_dir)) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                if filename.starts_with("CORPUS_") {
                    match process_corpus(
                        &path,
                        &crm_dir,
                        &doorman_endpoint,
                        &module_id,
                        &graph_store,
                        &tier_a_fallback,
                        &mut last_tier_a_attempt,
                    ) {
                        ExtractResult::Success | ExtractResult::Failed => {
                            processed_ledgers.push(filename);
                        }
                        ExtractResult::DeferCircuitOpen => {
                            circuit_deferred_ledgers.push(filename);
                        }
                        ExtractResult::DeferTransient => {
                            deferred_ledgers.push(filename);
                        }
                    }
                }
            }
        }
    }

    // ── Watcher loop (blocking — runs on the main task) ───────────────────────
    // Uses recv_timeout so the loop wakes every 30 s to retry deferred files.
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(&corpus_dir), RecursiveMode::NonRecursive)?;

    println!("================================================================");
    println!("[SYSTEM] Active Kernel Surveillance Engaged on Corpus Plane...");

    loop {
        match rx.recv_timeout(Duration::from_secs(30)) {
            Ok(Ok(Event { paths, .. })) => {
                for path in paths {
                    if let Some(extension) = path.extension() {
                        if extension == "json" {
                            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                            if filename.starts_with("CORPUS_")
                                && !processed_ledgers.contains(&filename)
                                && !deferred_ledgers.contains(&filename)
                                && !circuit_deferred_ledgers.contains(&filename)
                            {
                                println!("\n[WATCHER] New Corpus Detected: {}", filename);
                                thread::sleep(Duration::from_millis(250));
                                // Mark in-flight in deferred to prevent double-fire
                                // if the watcher emits multiple events for the same write.
                                deferred_ledgers.push(filename.clone());
                                match process_corpus(
                                    &path,
                                    &crm_dir,
                                    &doorman_endpoint,
                                    &module_id,
                                    &graph_store,
                                    &tier_a_fallback,
                                    &mut last_tier_a_attempt,
                                ) {
                                    ExtractResult::Success | ExtractResult::Failed => {
                                        deferred_ledgers.retain(|f| f != &filename);
                                        processed_ledgers.push(filename);
                                    }
                                    ExtractResult::DeferCircuitOpen => {
                                        deferred_ledgers.retain(|f| f != &filename);
                                        circuit_deferred_ledgers.push(filename);
                                    }
                                    ExtractResult::DeferTransient => {
                                        // stays in deferred_ledgers; retried on next timeout
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(_) => {}
            Err(RecvTimeoutError::Timeout) => {
                // Retry transient-deferred files
                if !deferred_ledgers.is_empty() {
                    println!(
                        "[RETRY] Retrying {} deferred CORPUS file(s)...",
                        deferred_ledgers.len()
                    );
                }
                let retry_queue: Vec<String> = std::mem::take(&mut deferred_ledgers);
                for filename in retry_queue {
                    let path = Path::new(&corpus_dir).join(&filename);
                    match process_corpus(
                        &path,
                        &crm_dir,
                        &doorman_endpoint,
                        &module_id,
                        &graph_store,
                        &tier_a_fallback,
                        &mut last_tier_a_attempt,
                    ) {
                        ExtractResult::Success | ExtractResult::Failed => {
                            processed_ledgers.push(filename);
                        }
                        ExtractResult::DeferCircuitOpen => {
                            // Tier B went down for this file — move it to the
                            // dormant circuit-deferred list (no per-tick storm).
                            circuit_deferred_ledgers.push(filename);
                        }
                        ExtractResult::DeferTransient => {
                            deferred_ledgers.push(filename);
                        }
                    }
                }

                // Tier-B recovery probe. Files deferred because the Yo-Yo circuit
                // is open sit dormant (not retried every tick — that would storm
                // the Doorman with the whole backlog during a GPU stockout). Each
                // tick we attempt exactly ONE of them: while Tier B is down the
                // probe fast-fails (circuit-open) and is returned to the list; the
                // moment Tier B recovers the probe succeeds (or defers transiently)
                // and the entire dormant backlog is promoted back into the active
                // retry queue — extraction resumes with no service restart.
                if !circuit_deferred_ledgers.is_empty() {
                    let probe = circuit_deferred_ledgers.remove(0);
                    let probe_path = Path::new(&corpus_dir).join(&probe);
                    match process_corpus(
                        &probe_path,
                        &crm_dir,
                        &doorman_endpoint,
                        &module_id,
                        &graph_store,
                        &tier_a_fallback,
                        &mut last_tier_a_attempt,
                    ) {
                        ExtractResult::DeferCircuitOpen => {
                            // Still down — keep it dormant.
                            circuit_deferred_ledgers.push(probe);
                        }
                        outcome => {
                            // Tier B is reachable again. Record the probe's own
                            // result, then resume the dormant backlog.
                            match outcome {
                                ExtractResult::DeferTransient => deferred_ledgers.push(probe),
                                _ => processed_ledgers.push(probe),
                            }
                            if !circuit_deferred_ledgers.is_empty() {
                                println!(
                                    "[RECOVERY] Tier B reachable again — resuming {} circuit-deferred CORPUS file(s).",
                                    circuit_deferred_ledgers.len()
                                );
                                deferred_ledgers.append(&mut circuit_deferred_ledgers);
                            }
                        }
                    }
                }
            }
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }

    Ok(())
}

fn process_corpus(
    filepath: &Path,
    crm_dir: &str,
    doorman_endpoint: &str,
    module_id: &str,
    graph_store: &Arc<dyn GraphStore>,
    tier_a_fallback: &TierAFallbackConfig,
    last_tier_a_attempt: &mut Option<Instant>,
) -> ExtractResult {
    // SC-5: log read failures instead of silently returning
    let content = match fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "  -> [ERROR] Failed to read CORPUS file {:?}: {}",
                filepath, e
            );
            return ExtractResult::Failed;
        }
    };
    let payload: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "  -> [ERROR] Malformed CORPUS JSON in {:?}: {}",
                filepath, e
            );
            return ExtractResult::Failed;
        }
    };

    let worm_id = payload["worm_id"].as_str().unwrap_or("UNKNOWN");
    let corpus_text = payload["corpus"].as_str().unwrap_or("");
    // Per-file module_id override: CORPUS JSON may carry a "module_id" field to
    // route workspace artifacts into a separate graph namespace (e.g. "foundry-workspace")
    // without requiring a separate service-content instance.
    let effective_module_id: &str = payload["module_id"]
        .as_str()
        .filter(|s| !s.is_empty())
        .unwrap_or(module_id);

    if corpus_text.is_empty() {
        return ExtractResult::Failed;
    }

    println!(
        "  -> [WATCHER] Routing payload to Doorman ({})/v1/extract...",
        doorman_endpoint
    );

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
    let res = client
        .post(&url)
        .json(&body)
        .timeout(Duration::from_secs(300))
        .send();

    match res {
        Ok(response) => {
            if !response.status().is_success() {
                println!(
                    "  -> [SYS_HALT] Doorman rejected payload: {}",
                    response.status()
                );
                return ExtractResult::Failed;
            }

            let extract_resp = match response.json::<serde_json::Value>() {
                Ok(v) => v,
                Err(_) => {
                    println!("  -> [SYS_HALT] Doorman returned invalid JSON format.");
                    return ExtractResult::Failed;
                }
            };

            // SC-2: differentiate defer reasons — circuit-open skips until restart,
            // transient retries after 30 s backoff.
            if extract_resp["deferred"].as_bool().unwrap_or(false) {
                let reason = extract_resp["defer_reason"].as_str().unwrap_or("unknown");
                return match reason {
                    "yoyo-circuit-open" => {
                        if !tier_a_fallback.enabled {
                            println!(
                                "  -> [WATCHER] Extraction deferred (circuit-open): skipping until restart."
                            );
                            return ExtractResult::DeferCircuitOpen;
                        }
                        // Rate limit: only attempt Tier A fallback once per interval.
                        if let Some(last) = *last_tier_a_attempt {
                            let elapsed = last.elapsed().as_secs();
                            if elapsed < tier_a_fallback.interval_secs {
                                println!(
                                    "  -> [WATCHER-TIER-A] Tier A fallback rate-limited — next eligible in {}s",
                                    tier_a_fallback.interval_secs.saturating_sub(elapsed)
                                );
                                return ExtractResult::DeferTransient;
                            }
                        }
                        println!(
                            "  -> [WATCHER-TIER-A] Tier A fallback extraction — degraded quality — next eligible in {}s",
                            tier_a_fallback.interval_secs
                        );
                        *last_tier_a_attempt = Some(Instant::now());
                        let chat_body = serde_json::json!({
                            "messages": [
                                {
                                    "role": "system",
                                    "content": "Extract named entities from the text. Classify each entity into exactly one category.\nCategories and examples:\n  Person — named human individual. Example: \"Jane Smith\".\n  Company — registered organisation or business. Example: \"Woodfine Management Corp.\".\n  Project — named initiative, programme, or system. Example: \"service-slm\".\n  Account — financial account, service account, or contract reference.\n  Location — geographic place or address. Example: \"Vancouver\".\nOmit: laws and regulations (not Location), dates and years (not Location), abstract concepts (not Company), regulatory bodies (not Company unless they are a named legal entity with a registered name).\nIf an entity does not clearly fit one category, omit it rather than guessing.\nReturn a JSON array matching the schema exactly."
                                },
                                {
                                    "role": "user",
                                    "content": corpus_text
                                }
                            ],
                            "grammar": {
                                "type": "json-schema",
                                "value": entity_schema
                            }
                        });
                        let chat_url = format!("{}/v1/chat/completions", doorman_endpoint);
                        let chat_client = reqwest::blocking::Client::new();
                        let chat_res = chat_client
                            .post(&chat_url)
                            .header("X-Foundry-Complexity", "low")
                            .json(&chat_body)
                            .timeout(Duration::from_secs(300))
                            .send();
                        let chat_entities: Vec<serde_json::Value> = match chat_res {
                            Ok(r) if r.status().is_success() => {
                                match r.json::<serde_json::Value>() {
                                    Ok(v) => {
                                        let content = v["choices"][0]["message"]["content"]
                                            .as_str()
                                            .unwrap_or("[]");
                                        serde_json::from_str(content).unwrap_or_default()
                                    }
                                    Err(_) => {
                                        println!(
                                            "  -> [WATCHER-TIER-A] Tier A response parse failed."
                                        );
                                        return ExtractResult::DeferCircuitOpen;
                                    }
                                }
                            }
                            Ok(r) => {
                                println!(
                                    "  -> [WATCHER-TIER-A] Tier A chat call rejected: {}",
                                    r.status()
                                );
                                return ExtractResult::DeferCircuitOpen;
                            }
                            Err(e) => {
                                println!("  -> [WATCHER-TIER-A] Tier A chat call error: {}", e);
                                return ExtractResult::DeferCircuitOpen;
                            }
                        };
                        if chat_entities.is_empty() {
                            println!("  -> [WATCHER-TIER-A] Tier A returned no entities.");
                            return ExtractResult::DeferCircuitOpen;
                        }
                        let mut graph_entities_ta: Vec<GraphEntity> = Vec::new();
                        for ent in &chat_entities {
                            let entity_name = ent["entity_name"].as_str().unwrap_or("").to_string();
                            let classification =
                                ent["classification"].as_str().unwrap_or("").to_string();
                            if entity_name.is_empty() || classification.is_empty() {
                                continue;
                            }
                            graph_entities_ta.push(GraphEntity {
                                entity_name,
                                classification,
                                role_vector: ent
                                    .get("role_vector")
                                    .and_then(|v| v.as_str())
                                    .filter(|s| !s.is_empty() && *s != "null")
                                    .map(str::to_string),
                                location_vector: ent
                                    .get("location_vector")
                                    .and_then(|v| v.as_str())
                                    .filter(|s| !s.is_empty() && *s != "null")
                                    .map(str::to_string),
                                contact_vector: ent
                                    .get("contact_vector")
                                    .and_then(|v| v.as_str())
                                    .filter(|s| !s.is_empty() && *s != "null")
                                    .map(str::to_string),
                                module_id: effective_module_id.to_string(),
                                confidence: 0.75,
                            });
                        }
                        if let Err(e) =
                            graph_store.upsert_entities(effective_module_id, &graph_entities_ta)
                        {
                            println!("  -> [WATCHER-TIER-A] Graph write failed: {}", e);
                            return ExtractResult::DeferCircuitOpen;
                        }
                        println!(
                            "  -> [WATCHER-TIER-A] {} entities written via Tier A fallback (module: {}).",
                            graph_entities_ta.len(),
                            effective_module_id
                        );
                        ExtractResult::Success
                    }
                    _ => {
                        println!(
                            "  -> [WATCHER] Extraction deferred ({}): will retry in 30 s.",
                            reason
                        );
                        ExtractResult::DeferTransient
                    }
                };
            }

            if !extract_resp["extraction_ok"].as_bool().unwrap_or(false) {
                println!(
                    "  -> [SYS_HALT] Extraction failed: extraction_ok false, no defer reason."
                );
                return ExtractResult::Failed;
            }

            let semantic_entities = extract_resp["entities"]
                .as_array()
                .cloned()
                .unwrap_or_default();

            let mut enriched_crm = Vec::new();
            let mut graph_entities: Vec<GraphEntity> = Vec::new();

            for ent in &semantic_entities {
                let entity_name = ent["entity_name"].as_str().unwrap_or("").to_string();
                let classification = ent["classification"].as_str().unwrap_or("").to_string();
                let role_vector = ent
                    .get("role_vector")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty() && *s != "null")
                    .map(str::to_string);
                let location_vector = ent
                    .get("location_vector")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty() && *s != "null")
                    .map(str::to_string);
                let contact_vector = ent
                    .get("contact_vector")
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
                new_ent.insert(
                    "classification".to_string(),
                    serde_json::json!(classification),
                );
                new_ent.insert(
                    "role_vector".to_string(),
                    role_vector
                        .as_deref()
                        .map(|s| serde_json::json!(s))
                        .unwrap_or(serde_json::json!("UNVERIFIED")),
                );
                new_ent.insert("confidence".to_string(), serde_json::json!(0.95));
                new_ent.insert(
                    "context_anchor".to_string(),
                    serde_json::json!("SLM NEURAL INFERENCE"),
                );

                let loc = location_vector
                    .as_deref()
                    .map(|s| serde_json::json!(s))
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

            // SC-3e: graph write first — if it fails, CRM is not written,
            // keeping the two stores consistent.
            if let Err(e) = graph_store.upsert_entities(effective_module_id, &graph_entities) {
                println!("  -> [GRAPH] Write failed: {}", e);
                return ExtractResult::Failed;
            }
            println!(
                "  -> [GRAPH] {} entities written to graph (module: {}).",
                graph_entities.len(),
                effective_module_id
            );

            // CRM write second — only reaches here if graph succeeded.
            let out_file = format!("{}/SEMANTIC_{}.json", crm_dir, worm_id);
            fs::write(&out_file, semantic_ledger.to_string()).unwrap();
            println!(
                "  -> [WATCHER] Semantic Integration Complete: {} Nodes Secured.",
                enriched_crm.len()
            );

            ExtractResult::Success
        }
        Err(e) => {
            // Transport error (Doorman unreachable, or the call interrupted —
            // e.g. Tier B preempted and the Doorman timed the request out) is
            // transient, not a permanent parse failure. Defer for retry rather
            // than marking the file processed, so an interrupted in-flight
            // extraction is never silently lost.
            println!(
                "  -> [SYS_HALT] Doorman routing failed (transient — will retry): {}",
                e
            );
            ExtractResult::DeferTransient
        }
    }
}
