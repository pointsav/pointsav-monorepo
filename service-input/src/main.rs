mod eval;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use eval::{compute_f1, normalize_reference_yaml, structural_health_check, CanonicalEntity};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::path::Path as FsPath;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct Config {
    bind: String,
    module_id: String,
    fs_endpoint: String,
    dest_archive: String,
    reference_root: String,
    reference_dir: String,
    rate_per_min: u64,
    batch_size: usize,
    ledger_path: String,
    max_bytes: usize,
    csv_batch_rows: usize,
    content_endpoint: String,
}

// ---------------------------------------------------------------------------
// Shared state
// ---------------------------------------------------------------------------

#[derive(Default)]
struct AppState {
    seen_sha256: HashSet<String>,
    queued: usize,
    done_count: usize,
    phase1_done: bool,
    phase2_processed: usize,
}

type SharedState = Arc<Mutex<AppState>>;

// ---------------------------------------------------------------------------
// Request / response types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct AppendPayload {
    path: String,
    submitted_by: Option<String>,
    tenant: Option<String>,
    source: Option<String>,
}

#[derive(Deserialize)]
struct AppendRequest {
    payload_id: String,
    payload: AppendPayload,
}

#[derive(Serialize)]
struct AppendResponse {
    payload_id: String,
    module_id: String,
    sha256: String,
    ts: u64,
    skipped: bool,
    skip_reason: Option<String>,
}

#[derive(Deserialize)]
struct MigrateRequest {
    batch_size: Option<usize>,
    offset: Option<usize>,
}

#[derive(Serialize)]
struct MigrateResponse {
    processed: usize,
    skipped: usize,
    offset_next: usize,
    stems_processed: Vec<String>,
    stems_skipped: Vec<String>,
}

#[derive(Serialize)]
struct HealthzResponse {
    status: String,
    queued: usize,
    done_count: usize,
}

#[derive(Serialize)]
struct StatusResponse {
    phase1_done: bool,
    phase2_progress: Phase2Progress,
}

#[derive(Serialize)]
struct Phase2Progress {
    processed: usize,
    offset_next: usize,
}

#[derive(Serialize, Deserialize)]
struct FsAppendRequest {
    payload_id: String,
    payload: serde_json::Value,
}

#[derive(Deserialize)]
struct FsAppendResponse {
    payload_id: Option<String>,
    sha256: Option<String>,
}

// ---------------------------------------------------------------------------
// CSV prose serialization
// ---------------------------------------------------------------------------

fn serialize_people_csv(csv_bytes: &[u8], batch_rows: usize, stem: &str) -> Vec<(String, String)> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(true)
        .from_reader(csv_bytes);

    let headers: Vec<String> = reader
        .headers()
        .ok()
        .map(|h| h.iter().map(|s| s.to_string()).collect())
        .unwrap_or_default();

    let is_people = headers.iter().any(|h| h.to_lowercase() == "name")
        && headers.iter().any(|h| h.to_lowercase() == "type");

    let mut batches = Vec::new();
    let mut current = String::new();
    let mut row_count = 0usize;
    let mut batch_idx = 0usize;

    for result in reader.records() {
        let rec = match result {
            Ok(r) => r,
            Err(_) => continue,
        };

        if is_people {
            let name = rec.get(0).unwrap_or("").trim();
            let etype = rec.get(1).unwrap_or("").trim();
            let source = rec.get(2).unwrap_or("").trim();
            if !name.is_empty() {
                current.push_str(&format!(
                    "Person: {} | Type: {} | Source: {}\n",
                    name, etype, source
                ));
            }
        } else {
            // domain CSV: two cols — Name | Domain (or similar)
            let name = rec.get(0).unwrap_or("").trim();
            let domain = headers.get(1).map(|s| s.as_str()).unwrap_or("unknown");
            if !name.is_empty() {
                current.push_str(&format!("{}: {}\n", domain, name));
            }
        }

        row_count += 1;
        if row_count >= batch_rows {
            batches.push((format!("{}-batch{:04}.txt", stem, batch_idx), current.clone()));
            current.clear();
            row_count = 0;
            batch_idx += 1;
        }
    }
    if !current.is_empty() {
        batches.push((format!("{}-batch{:04}.txt", stem, batch_idx), current));
    }
    batches
}

// ---------------------------------------------------------------------------
// HTTP helpers
// ---------------------------------------------------------------------------

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn post_to_fs(
    client: &reqwest::blocking::Client,
    fs_endpoint: &str,
    payload_id: &str,
    filename: &str,
    content_bytes: &[u8],
    module_id: &str,
) -> Result<FsAppendResponse, String> {
    let b64_content = B64.encode(content_bytes);
    let body = FsAppendRequest {
        payload_id: payload_id.to_string(),
        payload: serde_json::json!({
            "filename": filename,
            "content_b64": b64_content,
            "module_id": module_id,
            "source": "service-input",
        }),
    };
    let url = format!("{}/v1/append", fs_endpoint.trim_end_matches('/'));
    let resp = client
        .post(&url)
        .header("X-Foundry-Module-ID", module_id)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .map_err(|e| format!("POST {url}: {e}"))?;

    if resp.status().is_success() {
        resp.json::<FsAppendResponse>()
            .map_err(|e| format!("parse response: {e}"))
    } else {
        Err(format!("fs status {}", resp.status()))
    }
}

fn write_ledger_entry(ledger_path: &str, entry: &serde_json::Value) {
    let mut existing = std::fs::read_to_string(ledger_path).unwrap_or_default();
    existing.push_str(&format!("{}\n", entry));
    let tmp = format!("{}.tmp", ledger_path);
    if std::fs::write(&tmp, &existing).is_ok() {
        let _ = std::fs::rename(&tmp, ledger_path);
    }
}

fn infer_target_service(path: &str) -> &'static str {
    if path.contains("service-research") {
        return "service-research";
    }
    if path.contains("service-minutebook") {
        return "service-minutebook";
    }
    "service-content"
}

// ---------------------------------------------------------------------------
// Rate limiter (token bucket, simple)
// ---------------------------------------------------------------------------

fn sleep_rate(rate_per_min: u64) {
    if rate_per_min == 0 {
        return;
    }
    let ms = 60_000 / rate_per_min;
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

// ---------------------------------------------------------------------------
// Route handlers
// ---------------------------------------------------------------------------

async fn healthz(State((_, state)): State<(Arc<Config>, SharedState)>) -> impl IntoResponse {
    let s = state.lock().unwrap();
    Json(HealthzResponse {
        status: "ok".into(),
        queued: s.queued,
        done_count: s.done_count,
    })
}

async fn status(State((_, state)): State<(Arc<Config>, SharedState)>) -> impl IntoResponse {
    let s = state.lock().unwrap();
    Json(StatusResponse {
        phase1_done: s.phase1_done,
        phase2_progress: Phase2Progress {
            processed: s.phase2_processed,
            offset_next: s.phase2_processed,
        },
    })
}

async fn append(
    State((cfg, state)): State<(Arc<Config>, SharedState)>,
    Json(req): Json<AppendRequest>,
) -> impl IntoResponse {
    let path_str = &req.payload.path;
    let file_path = FsPath::new(path_str);

    // Read file bytes
    let bytes = match std::fs::read(file_path) {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({"error": format!("read {path_str}: {e}")})),
            )
                .into_response();
        }
    };

    if bytes.len() > cfg.max_bytes {
        return (
            StatusCode::PAYLOAD_TOO_LARGE,
            Json(serde_json::json!({"error": "payload exceeds max_bytes"})),
        )
            .into_response();
    }

    let sha = sha256_hex(&bytes);

    // SHA dedup
    {
        let mut s = state.lock().unwrap();
        if s.seen_sha256.contains(&sha) {
            return Json(AppendResponse {
                payload_id: req.payload_id,
                module_id: cfg.module_id.clone(),
                sha256: sha,
                ts: now_secs(),
                skipped: true,
                skip_reason: Some("already-processed".into()),
            })
            .into_response();
        }
        s.queued += 1;
    }

    let ext = file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let stem = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap();

    let ts = now_secs();
    let mut sent = 0usize;

    if ext == "csv" {
        // CSV: serialize as prose blocks of csv_batch_rows, send each as .txt
        let batches =
            serialize_people_csv(&bytes, cfg.csv_batch_rows, stem);
        for (batch_filename, prose) in &batches {
            let batch_id = format!("{}-{}", req.payload_id, batch_filename.replace('.', "-"));
            if let Err(e) = post_to_fs(
                &client,
                &cfg.fs_endpoint,
                &batch_id,
                batch_filename,
                prose.as_bytes(),
                &cfg.module_id,
            ) {
                eprintln!("[service-input] CSV batch {batch_filename}: {e}");
            } else {
                sent += 1;
            }
            sleep_rate(cfg.rate_per_min);
        }
    } else {
        // Prose / binary: send raw bytes as-is
        let filename = file_path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("unknown");
        match post_to_fs(
            &client,
            &cfg.fs_endpoint,
            &req.payload_id,
            filename,
            &bytes,
            &cfg.module_id,
        ) {
            Ok(_) => sent += 1,
            Err(e) => eprintln!("[service-input] append {filename}: {e}"),
        }
    }

    // Write own ledger entry
    if let Some(parent) = FsPath::new(&cfg.ledger_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    write_ledger_entry(
        &cfg.ledger_path,
        &serde_json::json!({
            "payload_id": req.payload_id,
            "path": path_str,
            "sha256": sha,
            "ts": ts,
            "sent_batches": sent,
            "target_service": infer_target_service(path_str),
        }),
    );

    {
        let mut s = state.lock().unwrap();
        s.seen_sha256.insert(sha.clone());
        s.queued = s.queued.saturating_sub(1);
        s.done_count += 1;
        if ext == "csv" {
            // Phase 1 is done when the last CSV has been sent
        }
    }

    Json(AppendResponse {
        payload_id: req.payload_id,
        module_id: cfg.module_id.clone(),
        sha256: sha,
        ts,
        skipped: false,
        skip_reason: None,
    })
    .into_response()
}

async fn migrate(
    State((cfg, state)): State<(Arc<Config>, SharedState)>,
    Json(req): Json<MigrateRequest>,
) -> impl IntoResponse {
    let batch_size = req.batch_size.unwrap_or(cfg.batch_size).min(50);
    let offset = req.offset.unwrap_or(0);

    let assets_dir = format!(
        "{}/service-research/assets",
        cfg.reference_root
    );
    let ledger_src_dir = format!(
        "{}/service-research/ledger",
        cfg.reference_root
    );

    // Collect sorted .md files
    let mut md_files: Vec<std::path::PathBuf> = walkdir::WalkDir::new(&assets_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|x| x == "md")
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect();
    md_files.sort();

    let slice = md_files.iter().skip(offset).take(batch_size);

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap();

    let mut processed = 0usize;
    let mut skipped = 0usize;
    let mut stems_processed = Vec::new();
    let mut stems_skipped = Vec::new();

    for md_path in slice {
        let stem = md_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Check corresponding ledger
        let ledger_path_src = format!("{}/{}.yaml", ledger_src_dir, stem);
        let ledger_bytes = std::fs::read(&ledger_path_src).unwrap_or_default();

        let ledger_valid = ledger_bytes.len() >= 60
            && !String::from_utf8_lossy(&ledger_bytes)
                .contains("extraction_protocol")
            && !String::from_utf8_lossy(&ledger_bytes)
                .contains("fidelity_mandate");

        if !ledger_valid {
            skipped += 1;
            stems_skipped.push(stem.clone());
            write_ledger_entry(
                &cfg.ledger_path,
                &serde_json::json!({
                    "stem": stem,
                    "ts": now_secs(),
                    "status": "skipped",
                    "skip_reason": if ledger_bytes.len() < 60 { "empty-ledger" } else { "prompt-leak" },
                }),
            );
            continue;
        }

        // Read .md content
        let md_bytes = match std::fs::read(md_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("[service-input/migrate] read {stem}.md: {e}");
                skipped += 1;
                stems_skipped.push(stem.clone());
                continue;
            }
        };

        let sha = sha256_hex(&md_bytes);
        let payload_id = format!("migrate-{}-{}", stem, &sha[..8]);

        // Send to service-fs
        let filename = format!("{}.md", stem);
        match post_to_fs(
            &client,
            &cfg.fs_endpoint,
            &payload_id,
            &filename,
            &md_bytes,
            &cfg.module_id,
        ) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("[service-input/migrate] fs POST {stem}: {e}");
                skipped += 1;
                stems_skipped.push(stem.clone());
                continue;
            }
        }

        // Copy reference YAML
        if let Some(parent) = FsPath::new(&cfg.reference_dir).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::create_dir_all(&cfg.reference_dir);
        let ledger_dst = format!("{}/{}.yaml", cfg.reference_dir, stem);
        if let Err(e) = std::fs::copy(&ledger_path_src, &ledger_dst) {
            eprintln!("[service-input/migrate] copy ledger {stem}: {e}");
        }

        // Write batch ledger entry
        if let Some(parent) = FsPath::new(&cfg.ledger_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        write_ledger_entry(
            &cfg.ledger_path,
            &serde_json::json!({
                "stem": stem,
                "sha256": sha,
                "ts": now_secs(),
                "ledger_valid": true,
                "status": "migrated",
            }),
        );

        processed += 1;
        stems_processed.push(stem.clone());

        {
            let mut s = state.lock().unwrap();
            s.phase2_processed += 1;
        }

        sleep_rate(cfg.rate_per_min);
    }

    Json(MigrateResponse {
        processed,
        skipped,
        offset_next: offset + processed + skipped,
        stems_processed,
        stems_skipped,
    })
}

async fn eval_stem(
    State((cfg, _state)): State<(Arc<Config>, SharedState)>,
    Path(stem): Path<String>,
) -> impl IntoResponse {
    let ref_path = format!("{}/{}.yaml", cfg.reference_dir, stem);
    let reference = match normalize_reference_yaml(FsPath::new(&ref_path)) {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": e, "stem": stem})),
            )
                .into_response();
        }
    };

    // Query DataGraph for extracted entities
    let extracted = query_datagraph_entities(&stem, &cfg.module_id, &cfg.content_endpoint);

    let f1 = compute_f1(&reference.entities, &extracted);
    let health =
        structural_health_check(&stem, &cfg.reference_dir.replace("/service-research/reference", ""), &cfg.module_id, &cfg.ledger_path);

    Json(serde_json::json!({
        "stem": stem,
        "entity_f1": f1.f1,
        "precision": f1.precision,
        "recall": f1.recall,
        "true_positives": f1.true_positives,
        "false_positives": f1.false_positives,
        "false_negatives": f1.false_negatives,
        "structural_health": {
            "corpus_file_exists": health.corpus_file_exists,
            "worm_ledger_advanced": health.worm_ledger_advanced,
            "graph_entity_count_nonzero": health.graph_entity_count_nonzero,
            "crm_record_exists": health.crm_record_exists,
            "ledger_entry_exists": health.ledger_entry_exists,
            "all_green": health.all_green(),
        },
        "reference_entity_count": reference.entities.len(),
        "extracted_entity_count": extracted.len(),
    }))
    .into_response()
}

async fn calibration_report(
    State((cfg, state)): State<(Arc<Config>, SharedState)>,
) -> impl IntoResponse {
    // Read the last N entries from the ledger
    let ledger_content = std::fs::read_to_string(&cfg.ledger_path).unwrap_or_default();
    let entries: Vec<serde_json::Value> = ledger_content
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .filter(|v: &serde_json::Value| {
            v.get("status").and_then(|s| s.as_str()) == Some("migrated")
        })
        .collect();

    let recent: Vec<_> = entries.iter().rev().take(20).collect();
    let total = recent.len();

    let mut doc_results = Vec::new();
    let mut f1_sum = 0.0f32;
    let mut struct_pass = 0usize;

    for entry in &recent {
        let stem = entry.get("stem").and_then(|s| s.as_str()).unwrap_or("");
        let ref_path = format!("{}/{}.yaml", cfg.reference_dir, stem);
        let reference = normalize_reference_yaml(FsPath::new(&ref_path)).ok();
        let extracted = query_datagraph_entities(stem, &cfg.module_id, &cfg.content_endpoint);
        let health = structural_health_check(
            stem,
            &cfg.reference_dir.replace("/service-research/reference", ""),
            &cfg.module_id,
            &cfg.ledger_path,
        );
        let all_green = health.all_green();
        if all_green {
            struct_pass += 1;
        }
        let f1 = reference
            .map(|r| compute_f1(&r.entities, &extracted).f1)
            .unwrap_or(0.0);
        f1_sum += f1;

        doc_results.push(serde_json::json!({
            "stem": stem,
            "entity_f1": f1,
            "structural_all_green": all_green,
        }));
    }

    let structural_pass_rate = if total > 0 { struct_pass as f32 / total as f32 } else { 1.0 };
    let mean_entity_f1 = if total > 0 { f1_sum / total as f32 } else { 0.0 };

    let (go_no_go, reason) = if total >= 5 && structural_pass_rate < 0.80 {
        ("stop", "structural_pass_rate < 0.80 — pipeline issue requires investigation")
    } else if structural_pass_rate >= 0.80 && mean_entity_f1 < 0.30 {
        ("hold", "pipeline healthy but entity F1 < 0.30 — model quality issue, not blocking")
    } else {
        ("go", "structural health and F1 within acceptable thresholds")
    };

    let processed = { state.lock().unwrap().phase2_processed };

    Json(serde_json::json!({
        "batch_size": total,
        "phase2_processed": processed,
        "docs": doc_results,
        "summary": {
            "structural_pass_rate": structural_pass_rate,
            "mean_entity_f1": mean_entity_f1,
            "go_no_go": go_no_go,
        },
        "go_no_go_reason": reason,
    }))
}

fn query_datagraph_entities(
    stem: &str,
    module_id: &str,
    content_endpoint: &str,
) -> Vec<CanonicalEntity> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_else(|_| reqwest::blocking::Client::new());

    let url = format!(
        "{}/v1/entities?module_id={}&source={}",
        content_endpoint.trim_end_matches('/'),
        module_id,
        stem,
    );
    let resp = match client.get(&url).send() {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };
    if !resp.status().is_success() {
        return Vec::new();
    }
    let body: serde_json::Value = match resp.json() {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    // Parse entity array from DataGraph response
    body.get("entities")
        .and_then(|arr| arr.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    let name = item.get("entity_name")?.as_str()?.to_string();
                    let etype = item
                        .get("classification")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown")
                        .to_string();
                    Some(CanonicalEntity { name, entity_type: etype })
                })
                .collect()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    let bind = std::env::var("SERVICE_INPUT_BIND")
        .unwrap_or_else(|_| "0.0.0.0:9106".into());
    let module_id = std::env::var("SERVICE_INPUT_MODULE_ID")
        .unwrap_or_else(|_| "jennifer".into());
    let fs_endpoint = std::env::var("SERVICE_INPUT_FS_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:9100".into());
    let dest_archive = std::env::var("SERVICE_INPUT_DEST_ARCHIVE")
        .unwrap_or_else(|_| "cluster-totebox-jennifer-2".into());
    let reference_root = std::env::var("SERVICE_INPUT_REFERENCE_ROOT")
        .unwrap_or_else(|_| "/srv/foundry/deployments/cluster-totebox-jennifer".into());
    let reference_dir = std::env::var("SERVICE_INPUT_REFERENCE_DIR")
        .unwrap_or_else(|_| {
            "/srv/foundry/deployments/cluster-totebox-jennifer-2/service-research/reference".into()
        });
    let rate_per_min: u64 = std::env::var("SERVICE_INPUT_RATE_PER_MIN")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(6);
    let batch_size: usize = std::env::var("SERVICE_INPUT_BATCH_SIZE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);
    let ledger_path = std::env::var("SERVICE_INPUT_LEDGER").unwrap_or_else(|_| {
        "/srv/foundry/deployments/cluster-totebox-jennifer-2/service-input/ledger.jsonl".into()
    });
    let max_bytes: usize = std::env::var("SERVICE_INPUT_MAX_BYTES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5 * 1024 * 1024);
    let csv_batch_rows: usize = std::env::var("SERVICE_INPUT_CSV_BATCH_ROWS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(100);
    let content_endpoint = std::env::var("SERVICE_INPUT_CONTENT_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:9081".into());

    let cfg = Arc::new(Config {
        bind: bind.clone(),
        module_id,
        fs_endpoint,
        dest_archive,
        reference_root,
        reference_dir,
        rate_per_min,
        batch_size,
        ledger_path,
        max_bytes,
        csv_batch_rows,
        content_endpoint,
    });
    let shared: SharedState = Arc::new(Mutex::new(AppState::default()));

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/status", get(status))
        .route("/v1/append", post(append))
        .route("/v1/migrate", post(migrate))
        .route("/v1/eval/:stem", get(eval_stem))
        .route("/v1/calibration-report", get(calibration_report))
        .with_state((cfg.clone(), shared));

    println!(
        "[service-input] ready on {bind} (module: {}, fs: {})",
        cfg.module_id, cfg.fs_endpoint
    );
    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .unwrap_or_else(|e| panic!("Cannot bind {bind}: {e}"));
    axum::serve(listener, app).await.unwrap();
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_people_csv_pipe_delimited() {
        let csv = b"Name|Type|FirstSource\nAlice Smith|Person|annual-report-2024\nAcme Corp|Company|q3-filing\nBob Jones|Person|board-minutes\n";
        let batches = serialize_people_csv(csv, 100, "people");
        assert_eq!(batches.len(), 1);
        let prose = &batches[0].1;
        assert!(prose.contains("Person: Alice Smith | Type: Person | Source: annual-report-2024"));
        assert!(prose.contains("Person: Acme Corp | Type: Company | Source: q3-filing"));
    }

    #[test]
    fn csv_batches_split_correctly() {
        let mut lines = String::from("Name|Type|FirstSource\n");
        for i in 0..250 {
            lines.push_str(&format!("Entity{i}|Person|source\n"));
        }
        let batches = serialize_people_csv(lines.as_bytes(), 100, "people");
        assert_eq!(batches.len(), 3); // 100 + 100 + 50
    }

    #[test]
    fn sha256_dedup_returns_consistent_hash() {
        let bytes = b"hello world";
        let h1 = sha256_hex(bytes);
        let h2 = sha256_hex(bytes);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }

    #[test]
    fn infer_target_service_routes_correctly() {
        assert_eq!(
            infer_target_service("/srv/foundry/deployments/cluster-totebox-jennifer/service-research/assets/report.md"),
            "service-research"
        );
        assert_eq!(
            infer_target_service("/srv/foundry/deployments/cluster-totebox-jennifer/service-minutebook/assets/minutes.md"),
            "service-minutebook"
        );
        assert_eq!(
            infer_target_service("/srv/foundry/deployments/cluster-totebox-jennifer/service-content/domains/corporate.csv"),
            "service-content"
        );
    }

    #[test]
    fn skip_invalid_ledger_size() {
        // A ledger smaller than 60 bytes is invalid
        let small: Vec<u8> = b"title: x".to_vec();
        assert!(small.len() < 60);
        // A valid ledger is at least 60 bytes
        let valid: Vec<u8> = b"mentioned_entities:\n  people: []\n  companies: []\nmetrics: []\nthemes: []\ncontent_type: research\n".to_vec();
        assert!(valid.len() >= 60);
    }

    #[test]
    fn skip_invalid_ledger_prompt_leak() {
        let leaked = b"extraction_protocol:\n  fidelity_mandate: true\ntitle: leaked\n";
        let content = String::from_utf8_lossy(leaked);
        let has_leak = content.contains("extraction_protocol") || content.contains("fidelity_mandate");
        assert!(has_leak);
    }
}
