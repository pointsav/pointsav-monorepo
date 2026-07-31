// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//
// service-gline — Rust-native GLiNER entity extraction (gline-rs / ONNX Runtime).
// Replaces the Python service-gliner microservice for seL4-hosted guest
// deployment — same HTTP contract (/healthz, /v1/extract, /v1/batch-extract),
// same domain-label CSV + entity_hints augmentation logic, no Python/PyTorch
// at runtime. See BRIEF-os-totebox-platform.md Session 18 (G2.5 / gline-rs).

use axum::{extract::State, routing::{get, post}, Json, Router};
use gliner::model::input::text::TextInput;
use gliner::model::params::Parameters;
use gliner::model::pipeline::span::SpanMode;
use gliner::model::GLiNER;
use orp::params::RuntimeParameters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

const DEFAULT_DOMAIN: &str = "projects";
const SCORE_THRESHOLD: f32 = 0.5;

struct AppState {
    model: GLiNER<SpanMode>,
    domain_labels: HashMap<String, HashMap<String, String>>,
    model_name: String,
}

#[derive(Deserialize)]
struct ExtractRequest {
    text: String,
    #[serde(default = "default_domain")]
    domain_id: String,
    entity_hints: Option<HashMap<String, Vec<String>>>,
}

#[derive(Deserialize)]
struct BatchExtractRequest {
    texts: Vec<String>,
    #[serde(default = "default_domain")]
    domain_id: String,
    entity_hints: Option<HashMap<String, Vec<String>>>,
}

fn default_domain() -> String {
    DEFAULT_DOMAIN.to_string()
}

#[derive(Serialize)]
struct Entity {
    entity_name: String,
    classification: String,
}

#[derive(Serialize)]
struct ExtractResponse {
    entities: Vec<Entity>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    model: String,
}

/// Same fallback table as service-gliner/main.py's _FALLBACK_DOMAIN_LABELS —
/// used only if entity_types.csv is missing/malformed, so a misconfigured
/// deployment degrades rather than crashes.
fn fallback_domain_labels() -> HashMap<String, HashMap<String, String>> {
    let mut domains = HashMap::new();

    let projects: HashMap<String, String> = [
        ("Person", "a named human individual — executive, broker, developer, or professional"),
        ("Company", "a named company, fund, REIT, or investment firm"),
        ("Project", "a named real estate development, building, property, or investment fund"),
        ("Location", "a named city, address, district, or country"),
        ("Account", "a named financial account, lease, or contract"),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect();
    domains.insert("projects".to_string(), projects);

    let corporate: HashMap<String, String> = [
        ("Person", "a named human individual"),
        ("Company", "a named company or organisation"),
        ("Project", "a named business initiative, building, or fund"),
        ("Location", "a named city or country"),
        ("Account", "a named financial account or contract"),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect();
    domains.insert("corporate".to_string(), corporate);

    let documentation: HashMap<String, String> = [
        ("Person", "a named developer, engineer, or contributor"),
        ("Company", "a named company or technology organisation"),
        ("Project", "a named software project, service, crate, or library"),
        ("Account", "a named running service, system account, or API endpoint"),
        ("Location", "a named server, deployment environment, or infrastructure location"),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect();
    domains.insert("documentation".to_string(), documentation);

    domains
}

/// Load entity type labels from ontology/entity_types.csv — mirrors
/// service-gliner/main.py's _load_domain_labels() exactly, same fallback
/// discipline (missing/unreadable/incomplete CSV -> known-good fallback,
/// never a crash).
fn load_domain_labels(ontology_dir: &str) -> HashMap<String, HashMap<String, String>> {
    let csv_path = format!("{}/entity_types.csv", ontology_dir);
    let domain_columns: HashMap<&str, &str> = [
        ("projects", "description_projects"),
        ("corporate", "description_corporate"),
        ("documentation", "description_documentation"),
    ]
    .into_iter()
    .collect();

    let mut domains: HashMap<String, HashMap<String, String>> =
        domain_columns.keys().map(|d| (d.to_string(), HashMap::new())).collect();

    let result: Result<(), Box<dyn std::error::Error>> = (|| {
        let mut reader = csv::Reader::from_path(&csv_path)?;
        for record in reader.deserialize() {
            let row: HashMap<String, String> = record?;
            let label = row.get("label").map(|s| s.trim()).unwrap_or("");
            if label.is_empty() {
                continue;
            }
            for (domain_id, column) in &domain_columns {
                if let Some(desc) = row.get(*column) {
                    let desc = desc.trim();
                    if !desc.is_empty() {
                        domains
                            .get_mut(*domain_id)
                            .unwrap()
                            .insert(label.to_string(), desc.to_string());
                    }
                }
            }
        }
        Ok(())
    })();

    let all_populated = domain_columns.keys().all(|d| {
        domains.get(*d).map(|m| !m.is_empty()).unwrap_or(false)
    });

    match result {
        Ok(()) if all_populated => {
            tracing::info!("loaded entity type labels from {}", csv_path);
            domains
        }
        Ok(()) => {
            tracing::warn!(
                "{} missing rows for one or more domains; using fallback labels",
                csv_path
            );
            fallback_domain_labels()
        }
        Err(e) => {
            tracing::warn!("failed to load {}: {}; using fallback labels", csv_path, e);
            fallback_domain_labels()
        }
    }
}

/// Build the (labels, desc_to_key) pair for a domain, appending KoGNER-style
/// concrete entity-name examples to each label's description when hints are
/// available — mirrors service-gliner/main.py's _labels_with_hints() exactly.
fn labels_with_hints(
    domain_labels: &HashMap<String, HashMap<String, String>>,
    domain_id: &str,
    entity_hints: &Option<HashMap<String, Vec<String>>>,
) -> (Vec<String>, HashMap<String, String>) {
    let label_map = domain_labels
        .get(domain_id)
        .or_else(|| domain_labels.get(DEFAULT_DOMAIN))
        .cloned()
        .unwrap_or_default();

    let mut labels = Vec::new();
    let mut desc_to_key = HashMap::new();
    for (key, desc) in &label_map {
        let augmented = match entity_hints.as_ref().and_then(|h| h.get(key)) {
            Some(hints) if !hints.is_empty() => {
                format!("{} (examples: {})", desc, hints.join(", "))
            }
            _ => desc.clone(),
        };
        desc_to_key.insert(augmented.clone(), key.clone());
        labels.push(augmented);
    }
    (labels, desc_to_key)
}

fn run_extraction(
    state: &AppState,
    texts: &[String],
    domain_id: &str,
    entity_hints: &Option<HashMap<String, Vec<String>>>,
) -> Vec<Entity> {
    if texts.iter().all(|t| t.trim().is_empty()) {
        return Vec::new();
    }
    let (labels, desc_to_key) = labels_with_hints(&state.domain_labels, domain_id, entity_hints);
    if labels.is_empty() {
        return Vec::new();
    }
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
    let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();

    let input = match TextInput::from_str(&text_refs, &label_refs) {
        Ok(i) => i,
        Err(e) => {
            tracing::error!("TextInput::from_str failed: {}", e);
            return Vec::new();
        }
    };
    let output = match state.model.inference(input) {
        Ok(o) => o,
        Err(e) => {
            tracing::error!("gline-rs inference failed: {}", e);
            return Vec::new();
        }
    };

    let mut entities = Vec::new();
    for spans in output.spans {
        for span in spans {
            if span.probability() < SCORE_THRESHOLD {
                continue;
            }
            let classification = desc_to_key
                .get(span.class())
                .cloned()
                .unwrap_or_else(|| span.class().to_string());
            entities.push(Entity {
                entity_name: span.text().to_string(),
                classification,
            });
        }
    }
    entities
}

async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok", model: state.model_name.clone() })
}

async fn extract(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ExtractRequest>,
) -> Json<ExtractResponse> {
    let entities = run_extraction(&state, &[req.text], &req.domain_id, &req.entity_hints);
    Json(ExtractResponse { entities })
}

async fn batch_extract(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BatchExtractRequest>,
) -> Json<ExtractResponse> {
    let non_empty: Vec<String> = req.texts.into_iter().filter(|t| !t.trim().is_empty()).collect();
    let entities = run_extraction(&state, &non_empty, &req.domain_id, &req.entity_hints);
    Json(ExtractResponse { entities })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    let model_name =
        std::env::var("GLINER_MODEL").unwrap_or_else(|_| "onnx-community/gliner_medium-v2.1".to_string());
    let weights_dir =
        std::env::var("GLINER_WEIGHTS_DIR").unwrap_or_else(|_| "/var/lib/local-gliner/weights".to_string());
    let ontology_dir = std::env::var("GLINER_ONTOLOGY_DIR")
        .unwrap_or_else(|_| "/srv/foundry/clones/project-totebox/service-content/ontology".to_string());
    let host = std::env::var("GLINER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("GLINER_PORT").unwrap_or_else(|_| "9085".to_string());

    let tokenizer_path = format!("{}/tokenizer.json", weights_dir);
    let model_path = format!("{}/model.onnx", weights_dir);

    tracing::info!("loading model from {} / {}", tokenizer_path, model_path);
    let model = GLiNER::<SpanMode>::new(
        Parameters::default(),
        RuntimeParameters::default(),
        &tokenizer_path,
        &model_path,
    )?;

    let domain_labels = load_domain_labels(&ontology_dir);

    let state = Arc::new(AppState { model, domain_labels, model_name });

    let app = Router::new()
        .route("/healthz", get(health))
        .route("/v1/extract", post(extract))
        .route("/v1/batch-extract", post(batch_extract))
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
