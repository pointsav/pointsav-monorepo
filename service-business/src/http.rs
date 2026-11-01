// SPDX-License-Identifier: Apache-2.0

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;

use crate::business_store::BusinessStore;
use crate::ledger_backing::LedgerBacking;
use crate::mcp;

#[derive(Clone)]
pub struct AppState {
    pub module_id: String,
    pub ledger_backing: Arc<LedgerBacking>,
    pub business_store: Arc<BusinessStore>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/mcp", post(mcp_endpoint))
        .with_state(state)
}

async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn readyz(State(state): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "ready": true,
            "module_id": state.module_id
        })),
    )
}

async fn mcp_endpoint(State(state): State<AppState>, body: String) -> impl IntoResponse {
    mcp::handler(state, &body).await
}
