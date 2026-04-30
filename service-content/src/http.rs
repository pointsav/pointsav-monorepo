use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::graph::{GraphEntity, GraphStore};

// ── request / response types ──────────────────────────────────────────────────

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

#[derive(Debug, Deserialize)]
pub struct MutateRequest {
    pub module_id: String,
    pub entities: Vec<GraphEntity>,
}

#[derive(Debug, Serialize)]
pub struct MutateResponse {
    pub upserted: usize,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

// ── handlers ──────────────────────────────────────────────────────────────────

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn graph_context(
    State(store): State<Arc<dyn GraphStore>>,
    Query(params): Query<ContextQuery>,
) -> Result<Json<Vec<GraphEntity>>, (StatusCode, String)> {
    store
        .query_context(&params.module_id, &params.q, params.limit)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn graph_mutate(
    State(store): State<Arc<dyn GraphStore>>,
    Json(body): Json<MutateRequest>,
) -> Result<Json<MutateResponse>, (StatusCode, String)> {
    store
        .upsert_entities(&body.module_id, &body.entities)
        .map(|upserted| Json(MutateResponse { upserted }))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// ── server entrypoint ─────────────────────────────────────────────────────────

pub async fn run_server(store: Arc<dyn GraphStore>, bind_addr: String) {
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/graph/context", get(graph_context))
        .route("/v1/graph/mutate", post(graph_mutate))
        .with_state(store);

    let listener = match tokio::net::TcpListener::bind(&bind_addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[HTTP] Failed to bind {}: {}", bind_addr, e);
            return;
        }
    };
    println!("[HTTP] Graph API listening on {}", bind_addr);
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("[HTTP] Server error: {}", e);
    }
}
