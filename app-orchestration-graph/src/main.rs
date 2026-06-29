// SPDX-License-Identifier: Apache-2.0 OR MIT

//! app-orchestration-graph — Cross-archive DataGraph federation gateway
//!
//! Listens on port 9181 (env: ORCHESTRATION_GRAPH_BIND, default 127.0.0.1:9181).
//! Fans out GET /v1/graph/context queries to all Totebox archives listed in
//! ORCHESTRATION_GRAPH_TARGETS, deduplicates entities by normalized name, and
//! returns the merged result.
//!
//! Activation: not deployed yet — scaffold only. Activated by Command Session
//! when the fleet has ≥2 Totebox archives with DataGraph endpoints.
//!
//! Routes:
//!   GET /healthz                    → liveness
//!   GET /v1/graph/context?q=&module_id= → federated entity query
//!   GET /v1/health                  → target list + per-archive status

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

// ── types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GraphEntity {
    pub entity_name: String,
    pub classification: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_vector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_vector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_vector: Option<String>,
    pub module_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_doc: Option<String>,
    /// Which archive this entity was returned from (injected by federation layer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_source: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContextQuery {
    pub q: String,
    pub module_id: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    20
}

#[derive(Debug, Serialize)]
pub struct FederatedContextResponse {
    pub entities: Vec<GraphEntity>,
    pub warnings: Vec<String>,
    pub archives_queried: usize,
    pub archives_responding: usize,
}

#[derive(Debug, Serialize)]
pub struct TargetStatus {
    pub url: String,
    pub reachable: bool,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub targets: Vec<TargetStatus>,
}

// ── server state ─────────────────────────────────────────────────────────────

pub struct AppState {
    /// One reqwest::Client with a 30-second per-request timeout.
    /// A single client reuses connection pools across fan-out requests.
    client: Client,
    /// Comma-separated list of service-content base URLs parsed from
    /// ORCHESTRATION_GRAPH_TARGETS env var at startup.
    targets: Vec<String>,
}

// ── handlers ─────────────────────────────────────────────────────────────────

async fn healthz() -> &'static str {
    "ok"
}

/// GET /v1/graph/context — federated entity query.
/// Fans out to all targets, merges, deduplicates by normalized entity name.
async fn graph_context(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ContextQuery>,
) -> Result<Json<FederatedContextResponse>, (StatusCode, String)> {
    let targets = &state.targets;
    if targets.is_empty() {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "ORCHESTRATION_GRAPH_TARGETS is empty — no archives configured".to_string(),
        ));
    }

    // Fan-out: query all targets concurrently.
    let futs: Vec<_> = targets
        .iter()
        .map(|base_url| {
            let client = state.client.clone();
            let url = format!(
                "{}/v1/graph/context?q={}&module_id={}&limit={}",
                base_url,
                urlencoding_basic(&params.q),
                urlencoding_basic(&params.module_id),
                params.limit,
            );
            let base_url = base_url.clone();
            async move {
                match client.get(&url).send().await {
                    Ok(resp) if resp.status().is_success() => {
                        match resp.json::<Vec<GraphEntity>>().await {
                            Ok(mut entities) => {
                                for e in &mut entities {
                                    e.federation_source = Some(base_url.clone());
                                }
                                Ok(entities)
                            }
                            Err(e) => Err(format!("{}: parse error — {}", base_url, e)),
                        }
                    }
                    Ok(resp) => Err(format!("{}: HTTP {}", base_url, resp.status())),
                    Err(e) => Err(format!("{}: {}", base_url, e)),
                }
            }
        })
        .collect();

    let results = futures_join_all(futs).await;

    let mut merged: Vec<GraphEntity> = Vec::new();
    let mut seen_keys: HashSet<String> = HashSet::new();
    let mut warnings: Vec<String> = Vec::new();
    let mut responding = 0usize;

    for result in results {
        match result {
            Ok(entities) => {
                responding += 1;
                for entity in entities {
                    let key = normalize_entity_key(&entity.entity_name);
                    if seen_keys.insert(key) {
                        merged.push(entity);
                    }
                }
            }
            Err(w) => warnings.push(w),
        }
    }

    // Sort by confidence descending; stable so order within equal confidence is archive order.
    merged.sort_by(|a, b| {
        b.confidence
            .unwrap_or(0.0)
            .partial_cmp(&a.confidence.unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(Json(FederatedContextResponse {
        entities: merged.into_iter().take(params.limit).collect(),
        warnings,
        archives_queried: targets.len(),
        archives_responding: responding,
    }))
}

/// GET /v1/health — list target archives and their reachability.
async fn health_targets(
    State(state): State<Arc<AppState>>,
) -> Json<HealthResponse> {
    let futs: Vec<_> = state
        .targets
        .iter()
        .map(|base_url| {
            let client = state.client.clone();
            let url = format!("{}/healthz", base_url);
            let base_url = base_url.clone();
            async move {
                let reachable = client.get(&url).send().await
                    .map(|r| r.status().is_success())
                    .unwrap_or(false);
                TargetStatus { url: base_url, reachable }
            }
        })
        .collect();

    let statuses = futures_join_all(futs).await;
    let all_ok = statuses.iter().all(|s| s.reachable);
    Json(HealthResponse {
        status: if all_ok { "ok" } else { "degraded" },
        targets: statuses,
    })
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Minimal percent-encoding for query string values (encodes space and & only).
fn urlencoding_basic(s: &str) -> String {
    s.replace('%', "%25")
        .replace('&', "%26")
        .replace(' ', "%20")
        .replace('+', "%2B")
}

/// Normalize an entity name for deduplication: lowercase, strip punctuation.
fn normalize_entity_key(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Run a Vec of futures concurrently and collect results.
/// Equivalent to `futures::future::join_all` without the `futures` dependency.
async fn futures_join_all<T: Send + 'static>(
    futs: Vec<impl std::future::Future<Output = T> + Send + 'static>,
) -> Vec<T> {
    let handles: Vec<_> = futs
        .into_iter()
        .map(|f| tokio::spawn(f))
        .collect();
    let mut results = Vec::with_capacity(handles.len());
    for h in handles {
        // JoinError only on panic — treat as a warning-worthy absent result.
        if let Ok(v) = h.await {
            results.push(v);
        }
    }
    results
}

// ── main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let bind_addr = std::env::var("ORCHESTRATION_GRAPH_BIND")
        .unwrap_or_else(|_| "127.0.0.1:9181".to_string());

    // Parse comma-separated target URLs; trim whitespace; skip empty entries.
    let targets: Vec<String> = std::env::var("ORCHESTRATION_GRAPH_TARGETS")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if targets.is_empty() {
        eprintln!(
            "[orchestration-graph] WARNING: ORCHESTRATION_GRAPH_TARGETS is not set. \
             Set to comma-separated service-content base URLs (e.g. \
             http://archive1:9081,http://archive2:9081)."
        );
    } else {
        println!(
            "[orchestration-graph] Federating {} archive(s): {}",
            targets.len(),
            targets.join(", ")
        );
    }

    // 30-second per-request timeout: one unreachable archive must not stall all queries.
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("build reqwest client");

    let state = Arc::new(AppState { client, targets });

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/graph/context", get(graph_context))
        .route("/v1/health", get(health_targets))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|e| panic!("[orchestration-graph] Failed to bind {}: {}", bind_addr, e));

    println!("[orchestration-graph] Listening on {}", bind_addr);
    axum::serve(listener, app)
        .await
        .expect("[orchestration-graph] Server error");
}
