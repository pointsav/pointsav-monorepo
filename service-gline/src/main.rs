// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//
// service-gline — Rust-native GLiNER entity extraction (gline-rs / ONNX Runtime).
// Replaces the Python service-gliner microservice for seL4-hosted guest
// deployment — same HTTP contract (/healthz, /v1/extract, /v1/batch-extract),
// same domain-label CSV + entity_hints augmentation logic, no Python/PyTorch
// at runtime. See BRIEF-os-totebox-platform.md Session 18 (G2.5 / gline-rs).

use axum::extract::{DefaultBodyLimit, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use gliner::model::input::text::TextInput;
use gliner::model::params::Parameters;
use gliner::model::pipeline::span::SpanMode;
use gliner::model::GLiNER;
use orp::params::RuntimeParameters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

const DEFAULT_DOMAIN: &str = "projects";
const SCORE_THRESHOLD: f32 = 0.5;
const PROBE_TEXT: &str = "Alice flew to Paris.";
const RETRY_AFTER_SECS: &str = "5";

#[derive(Clone, Debug)]
struct Config {
    max_chars: usize,
    timeout: Duration,
    max_inflight: usize,
    queue_wait: Duration,
    health_timeout: Duration,
    health_cache: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_chars: 8000,
            timeout: Duration::from_secs(25),
            max_inflight: 1,
            queue_wait: Duration::from_secs(5),
            health_timeout: Duration::from_secs(5),
            health_cache: Duration::from_secs(5),
        }
    }
}

impl Config {
    fn from_env() -> Self {
        let d = Self::default();
        Self {
            max_chars: env_or("GLINER_MAX_CHARS", d.max_chars),
            timeout: env_secs("GLINER_TIMEOUT_SECS", d.timeout, 1),
            max_inflight: env_or("GLINER_MAX_INFLIGHT", d.max_inflight).max(1),
            queue_wait: env_secs("GLINER_QUEUE_WAIT_SECS", d.queue_wait, 0),
            health_timeout: env_secs("GLINER_HEALTH_TIMEOUT_SECS", d.health_timeout, 1),
            health_cache: d.health_cache,
        }
    }

    /// Raw UTF-8 is at most 4 bytes per character; 4 KiB covers the JSON
    /// envelope, domain_id and entity_hints.
    fn body_limit(&self) -> usize {
        self.max_chars.saturating_mul(4).saturating_add(4096)
    }
}

fn env_or<T: FromStr>(name: &str, default: T) -> T {
    match std::env::var(name) {
        Ok(v) => v.trim().parse().unwrap_or_else(|_| {
            tracing::warn!("{}={:?} is not valid; using the default", name, v);
            default
        }),
        Err(_) => default,
    }
}

fn env_secs(name: &str, default: Duration, min: u64) -> Duration {
    Duration::from_secs(env_or(name, default.as_secs()).max(min))
}

struct RawSpan {
    text: String,
    class: String,
    probability: f32,
}

/// The model call, behind a trait so the HTTP layer can be tested without
/// loading weights.
trait Extractor: Send + Sync {
    fn extract(&self, texts: &[&str], labels: &[&str]) -> Result<Vec<RawSpan>, String>;
}

struct GlinerExtractor(GLiNER<SpanMode>);

impl Extractor for GlinerExtractor {
    fn extract(&self, texts: &[&str], labels: &[&str]) -> Result<Vec<RawSpan>, String> {
        let input = TextInput::from_str(texts, labels)
            .map_err(|e| format!("TextInput::from_str failed: {}", e))?;
        let output = self
            .0
            .inference(input)
            .map_err(|e| format!("gline-rs inference failed: {}", e))?;
        Ok(output
            .spans
            .into_iter()
            .flatten()
            .map(|span| RawSpan {
                text: span.text().to_string(),
                class: span.class().to_string(),
                probability: span.probability(),
            })
            .collect())
    }
}

struct AppState {
    extractor: Box<dyn Extractor>,
    domain_labels: HashMap<String, HashMap<String, String>>,
    model_name: String,
    config: Config,
    permits: Arc<Semaphore>,
    inflight: AtomicUsize,
    /// Blocking calls still running after their caller gave up on them.
    stuck: AtomicUsize,
    timeouts_total: AtomicU64,
    last_probe: Mutex<Option<(Instant, u64)>>,
}

const CALL_RUNNING: u8 = 0;
const CALL_DONE: u8 = 1;
const CALL_TIMED_OUT: u8 = 2;

enum CallError {
    Timeout,
    Panicked,
}

impl AppState {
    fn new(
        extractor: Box<dyn Extractor>,
        domain_labels: HashMap<String, HashMap<String, String>>,
        model_name: String,
        config: Config,
    ) -> Self {
        Self {
            extractor,
            domain_labels,
            model_name,
            permits: Arc::new(Semaphore::new(config.max_inflight)),
            config,
            inflight: AtomicUsize::new(0),
            stuck: AtomicUsize::new(0),
            timeouts_total: AtomicU64::new(0),
            last_probe: Mutex::new(None),
        }
    }

    async fn acquire(&self, wait: Duration) -> Option<OwnedSemaphorePermit> {
        match tokio::time::timeout(wait, Arc::clone(&self.permits).acquire_owned()).await {
            Ok(Ok(permit)) => Some(permit),
            _ => None,
        }
    }

    /// Runs `f` on the blocking pool and waits at most `limit` for it. A
    /// blocking thread cannot be cancelled, so on timeout the call keeps
    /// running and keeps its permit until it returns.
    async fn call<T, F>(
        self: &Arc<Self>,
        permit: OwnedSemaphorePermit,
        limit: Duration,
        f: F,
    ) -> Result<T, CallError>
    where
        T: Send + 'static,
        F: FnOnce(&AppState) -> T + Send + 'static,
    {
        let phase = Arc::new(AtomicU8::new(CALL_RUNNING));
        let state = Arc::clone(self);
        let task_phase = Arc::clone(&phase);
        let handle = tokio::task::spawn_blocking(move || {
            // The permit is dropped here, when the model call returns, and not
            // when the caller's timeout fires: a wedged model then shows as
            // busy (503) instead of collecting more work behind it.
            let _permit = permit;
            state.inflight.fetch_add(1, Ordering::SeqCst);
            let _finish = CallFinish {
                state: &state,
                phase: &task_phase,
            };
            f(&state)
        });
        match tokio::time::timeout(limit, handle).await {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(e)) => {
                tracing::error!("extraction task failed: {}", e);
                Err(CallError::Panicked)
            }
            Err(_) => {
                self.timeouts_total.fetch_add(1, Ordering::SeqCst);
                self.stuck.fetch_add(1, Ordering::SeqCst);
                if phase
                    .compare_exchange(
                        CALL_RUNNING,
                        CALL_TIMED_OUT,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    )
                    .is_err()
                {
                    self.stuck.fetch_sub(1, Ordering::SeqCst);
                }
                tracing::warn!("extraction exceeded {:?}; still running", limit);
                Err(CallError::Timeout)
            }
        }
    }

    fn fresh_probe_ms(&self) -> Option<u64> {
        let last = *self.last_probe.lock().unwrap_or_else(|e| e.into_inner());
        last.filter(|(at, _)| at.elapsed() < self.config.health_cache)
            .map(|(_, ms)| ms)
    }

    fn last_probe_ms(&self) -> Option<u64> {
        self.last_probe
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .map(|(_, ms)| ms)
    }
}

/// Runs on the blocking thread, including when `f` panics.
struct CallFinish<'a> {
    state: &'a AppState,
    phase: &'a AtomicU8,
}

impl Drop for CallFinish<'_> {
    fn drop(&mut self) {
        self.state.inflight.fetch_sub(1, Ordering::SeqCst);
        if self.phase.swap(CALL_DONE, Ordering::SeqCst) == CALL_TIMED_OUT {
            self.state.stuck.fetch_sub(1, Ordering::SeqCst);
        }
    }
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
    inflight: usize,
    timeouts_total: u64,
    last_probe_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<&'static str>,
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
) -> Result<Vec<Entity>, String> {
    if texts.iter().all(|t| t.trim().is_empty()) {
        return Ok(Vec::new());
    }
    let (labels, desc_to_key) = labels_with_hints(&state.domain_labels, domain_id, entity_hints);
    if labels.is_empty() {
        return Ok(Vec::new());
    }
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
    let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();

    let spans = state.extractor.extract(&text_refs, &label_refs)?;
    Ok(spans
        .into_iter()
        .filter(|span| span.probability >= SCORE_THRESHOLD)
        .map(|span| Entity {
            classification: desc_to_key.get(&span.class).cloned().unwrap_or(span.class),
            entity_name: span.text,
        })
        .collect())
}

fn error_response(status: StatusCode, message: &str) -> Response {
    (status, Json(serde_json::json!({ "error": message }))).into_response()
}

fn busy_response() -> Response {
    let mut response = error_response(
        StatusCode::SERVICE_UNAVAILABLE,
        "extraction busy; retry later",
    );
    response.headers_mut().insert(
        header::RETRY_AFTER,
        header::HeaderValue::from_static(RETRY_AFTER_SECS),
    );
    response
}

async fn extract_texts(
    state: Arc<AppState>,
    texts: Vec<String>,
    domain_id: String,
    entity_hints: Option<HashMap<String, Vec<String>>>,
) -> Response {
    if texts.iter().all(|t| t.trim().is_empty()) {
        return Json(ExtractResponse {
            entities: Vec::new(),
        })
        .into_response();
    }
    let chars: usize = texts.iter().map(|t| t.chars().count()).sum();
    if chars > state.config.max_chars {
        return error_response(
            StatusCode::PAYLOAD_TOO_LARGE,
            &format!(
                "text is {} characters; the limit is {}",
                chars, state.config.max_chars
            ),
        );
    }
    let Some(permit) = state.acquire(state.config.queue_wait).await else {
        return busy_response();
    };
    let result = state
        .call(permit, state.config.timeout, move |s| {
            run_extraction(s, &texts, &domain_id, &entity_hints)
        })
        .await;
    match result {
        Ok(Ok(entities)) => Json(ExtractResponse { entities }).into_response(),
        Ok(Err(e)) => {
            tracing::error!("{}", e);
            Json(ExtractResponse {
                entities: Vec::new(),
            })
            .into_response()
        }
        Err(CallError::Timeout) => {
            error_response(StatusCode::GATEWAY_TIMEOUT, "extraction timed out")
        }
        Err(CallError::Panicked) => {
            error_response(StatusCode::INTERNAL_SERVER_ERROR, "extraction failed")
        }
    }
}

fn health_response(
    state: &AppState,
    ok: bool,
    last_probe_ms: Option<u64>,
    detail: Option<&'static str>,
) -> Response {
    let (code, status) = if ok {
        (StatusCode::OK, "ok")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "degraded")
    };
    let body = HealthResponse {
        status,
        model: state.model_name.clone(),
        inflight: state.inflight.load(Ordering::SeqCst),
        timeouts_total: state.timeouts_total.load(Ordering::SeqCst),
        last_probe_ms,
        detail,
    };
    (code, Json(body)).into_response()
}

/// Runs one small extraction through the same permit and blocking pool as
/// real requests. Never waits for a permit.
async fn health(State(state): State<Arc<AppState>>) -> Response {
    if state.stuck.load(Ordering::SeqCst) > 0 {
        return health_response(
            &state,
            false,
            state.last_probe_ms(),
            Some("inference running past its timeout"),
        );
    }
    if let Some(ms) = state.fresh_probe_ms() {
        return health_response(&state, true, Some(ms), None);
    }
    let Ok(permit) = Arc::clone(&state.permits).try_acquire_owned() else {
        return health_response(
            &state,
            true,
            state.last_probe_ms(),
            Some("busy; probe skipped"),
        );
    };
    let started = Instant::now();
    let result = state
        .call(permit, state.config.health_timeout, |s| {
            let (labels, _) = labels_with_hints(&s.domain_labels, DEFAULT_DOMAIN, &None);
            let label_refs: Vec<&str> = labels.iter().map(|l| l.as_str()).collect();
            s.extractor.extract(&[PROBE_TEXT], &label_refs).map(|_| ())
        })
        .await;
    match result {
        Ok(Ok(())) => {
            let ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
            *state.last_probe.lock().unwrap_or_else(|e| e.into_inner()) =
                Some((Instant::now(), ms));
            health_response(&state, true, Some(ms), None)
        }
        Ok(Err(e)) => {
            tracing::error!("health probe failed: {}", e);
            health_response(&state, false, state.last_probe_ms(), Some("probe failed"))
        }
        Err(CallError::Timeout) => health_response(
            &state,
            false,
            state.last_probe_ms(),
            Some("probe timed out"),
        ),
        Err(CallError::Panicked) => {
            health_response(&state, false, state.last_probe_ms(), Some("probe failed"))
        }
    }
}

async fn extract(State(state): State<Arc<AppState>>, Json(req): Json<ExtractRequest>) -> Response {
    extract_texts(state, vec![req.text], req.domain_id, req.entity_hints).await
}

async fn batch_extract(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BatchExtractRequest>,
) -> Response {
    let non_empty: Vec<String> = req
        .texts
        .into_iter()
        .filter(|t| !t.trim().is_empty())
        .collect();
    extract_texts(state, non_empty, req.domain_id, req.entity_hints).await
}

fn app(state: Arc<AppState>) -> Router {
    let body_limit = state.config.body_limit();
    Router::new()
        .route("/healthz", get(health))
        .route("/v1/extract", post(extract))
        .route("/v1/batch-extract", post(batch_extract))
        .layer(DefaultBodyLimit::max(body_limit))
        .with_state(state)
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

    let config = Config::from_env();
    tracing::info!("{:?}", config);
    let state = Arc::new(AppState::new(
        Box::new(GlinerExtractor(model)),
        domain_labels,
        model_name,
        config,
    ));

    let addr = format!("{}:{}", host, port);
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app(state)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{HeaderMap, Request};
    use http_body_util::BodyExt;
    use std::sync::mpsc;
    use tower::ServiceExt;

    struct FastExtractor {
        calls: Arc<AtomicUsize>,
    }

    impl Extractor for FastExtractor {
        fn extract(&self, _texts: &[&str], labels: &[&str]) -> Result<Vec<RawSpan>, String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(vec![
                RawSpan {
                    text: "Alice".to_string(),
                    class: labels[0].to_string(),
                    probability: 0.9,
                },
                RawSpan {
                    text: "flew".to_string(),
                    class: labels[0].to_string(),
                    probability: 0.1,
                },
            ])
        }
    }

    struct FailingExtractor;

    impl Extractor for FailingExtractor {
        fn extract(&self, _texts: &[&str], _labels: &[&str]) -> Result<Vec<RawSpan>, String> {
            Err("model error".to_string())
        }
    }

    /// Blocks until the test drops its sender (or 10 s pass).
    struct StuckExtractor {
        release: Mutex<mpsc::Receiver<()>>,
        calls: Arc<AtomicUsize>,
    }

    impl Extractor for StuckExtractor {
        fn extract(&self, _texts: &[&str], _labels: &[&str]) -> Result<Vec<RawSpan>, String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let rx = self.release.lock().unwrap();
            let _ = rx.recv_timeout(Duration::from_secs(10));
            Ok(Vec::new())
        }
    }

    fn test_config() -> Config {
        Config {
            max_chars: 100,
            timeout: Duration::from_millis(100),
            max_inflight: 1,
            queue_wait: Duration::from_millis(50),
            health_timeout: Duration::from_millis(100),
            health_cache: Duration::from_secs(5),
        }
    }

    fn test_state(extractor: Box<dyn Extractor>, config: Config) -> Arc<AppState> {
        let labels: HashMap<String, String> =
            [("Person".to_string(), "a named person".to_string())].into();
        let domains = [(DEFAULT_DOMAIN.to_string(), labels)].into();
        Arc::new(AppState::new(
            extractor,
            domains,
            "test-model".to_string(),
            config,
        ))
    }

    fn stuck_state(config: Config) -> (Arc<AppState>, mpsc::Sender<()>, Arc<AtomicUsize>) {
        let (tx, rx) = mpsc::channel();
        let calls = Arc::new(AtomicUsize::new(0));
        let extractor = StuckExtractor {
            release: Mutex::new(rx),
            calls: Arc::clone(&calls),
        };
        (test_state(Box::new(extractor), config), tx, calls)
    }

    async fn send(state: &Arc<AppState>, req: Request<Body>) -> (StatusCode, HeaderMap, Vec<u8>) {
        let response = app(Arc::clone(state)).oneshot(req).await.unwrap();
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        (status, headers, body.to_vec())
    }

    fn extract_request(text: &str) -> Request<Body> {
        Request::post("/v1/extract")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::json!({ "text": text }).to_string()))
            .unwrap()
    }

    fn health_request() -> Request<Body> {
        Request::get("/healthz").body(Body::empty()).unwrap()
    }

    fn json(body: &[u8]) -> serde_json::Value {
        serde_json::from_slice(body).unwrap()
    }

    fn fast() -> Box<FastExtractor> {
        Box::new(FastExtractor {
            calls: Arc::new(AtomicUsize::new(0)),
        })
    }

    #[tokio::test]
    async fn valid_text_returns_entities_in_same_shape() {
        let state = test_state(fast(), test_config());
        let (status, _, body) = send(&state, extract_request("Alice flew to Paris.")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            json(&body),
            serde_json::json!({
                "entities": [{ "entity_name": "Alice", "classification": "Person" }]
            })
        );
    }

    #[tokio::test]
    async fn oversize_text_is_413() {
        let state = test_state(fast(), test_config());
        let (status, _, body) = send(&state, extract_request(&"a".repeat(101))).await;
        assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
        assert!(json(&body)["error"].is_string());
    }

    #[tokio::test]
    async fn oversize_batch_total_is_413() {
        let state = test_state(fast(), test_config());
        let req = Request::post("/v1/batch-extract")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::json!({ "texts": ["a".repeat(60), "b".repeat(60)] }).to_string(),
            ))
            .unwrap();
        let (status, _, _) = send(&state, req).await;
        assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn body_over_limit_is_413() {
        let state = test_state(fast(), test_config());
        let body_limit = state.config.body_limit();
        let (status, _, _) = send(&state, extract_request(&"a".repeat(body_limit))).await;
        assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn slow_extractor_times_out_with_504() {
        let (state, _release, _) = stuck_state(test_config());
        let (status, _, body) = send(&state, extract_request("Alice")).await;
        assert_eq!(status, StatusCode::GATEWAY_TIMEOUT);
        assert!(json(&body)["error"].is_string());
        assert_eq!(state.timeouts_total.load(Ordering::SeqCst), 1);
        assert_eq!(state.inflight.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn request_while_first_is_stuck_is_503_with_retry_after() {
        let (state, _release, calls) = stuck_state(test_config());
        let (status, _, _) = send(&state, extract_request("Alice")).await;
        assert_eq!(status, StatusCode::GATEWAY_TIMEOUT);
        let (status, headers, _) = send(&state, extract_request("Bob")).await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(headers.get(header::RETRY_AFTER).unwrap(), "5");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn permit_returns_when_stuck_call_finishes() {
        let (state, release, _) = stuck_state(test_config());
        let (status, _, _) = send(&state, extract_request("Alice")).await;
        assert_eq!(status, StatusCode::GATEWAY_TIMEOUT);
        drop(release);
        let permit = state.acquire(Duration::from_secs(5)).await;
        assert!(permit.is_some());
        assert_eq!(state.inflight.load(Ordering::SeqCst), 0);
        assert_eq!(state.stuck.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn health_ok_with_fast_extractor() {
        let state = test_state(fast(), test_config());
        let (status, _, body) = send(&state, health_request()).await;
        assert_eq!(status, StatusCode::OK);
        let body = json(&body);
        assert_eq!(body["status"], "ok");
        assert_eq!(body["model"], "test-model");
        assert_eq!(body["inflight"], 0);
        assert_eq!(body["timeouts_total"], 0);
        assert!(body["last_probe_ms"].is_u64());
    }

    #[tokio::test]
    async fn health_probe_is_cached() {
        let extractor = fast();
        let calls = Arc::clone(&extractor.calls);
        let state = test_state(extractor, test_config());
        send(&state, health_request()).await;
        let (status, _, _) = send(&state, health_request()).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn health_degraded_with_stuck_extractor() {
        let (state, _release, _) = stuck_state(test_config());
        let (status, _, body) = send(&state, health_request()).await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        let body = json(&body);
        assert_eq!(body["status"], "degraded");
        assert_eq!(body["timeouts_total"], 1);
    }

    #[tokio::test]
    async fn health_degraded_when_probe_fails() {
        let state = test_state(Box::new(FailingExtractor), test_config());
        let (status, _, body) = send(&state, health_request()).await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(json(&body)["status"], "degraded");
    }

    #[tokio::test]
    async fn health_does_not_queue_behind_stuck_inference() {
        let mut config = test_config();
        config.health_timeout = Duration::from_secs(5);
        let (state, _release, calls) = stuck_state(config);
        let (status, _, _) = send(&state, extract_request("Alice")).await;
        assert_eq!(status, StatusCode::GATEWAY_TIMEOUT);
        let started = Instant::now();
        let (status, _, body) = send(&state, health_request()).await;
        assert!(started.elapsed() < Duration::from_secs(1));
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        let body = json(&body);
        assert_eq!(body["status"], "degraded");
        assert_eq!(body["inflight"], 1);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn config_defaults() {
        let config = Config::default();
        assert_eq!(config.max_chars, 8000);
        assert_eq!(config.timeout, Duration::from_secs(25));
        assert_eq!(config.max_inflight, 1);
        assert_eq!(config.queue_wait, Duration::from_secs(5));
        assert_eq!(config.health_timeout, Duration::from_secs(5));
        assert_eq!(config.body_limit(), 8000 * 4 + 4096);
    }
}
