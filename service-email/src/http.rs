// SPDX-License-Identifier: Apache-2.0

//! Axum router and application state for service-email.
//!
//! Endpoints:
//!   GET  /healthz   → liveness, always 200
//!   GET  /readyz    → readiness; 200 with module_id
//!   POST /mcp       → MCP JSON-RPC 2.0 tool interface (email.ingest, email.send)

use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use serde::Serialize;

use crate::ledger_backing::LedgerBacking;
use crate::mcp::mcp_handler;

pub struct AppState {
    pub module_id: String,
    pub ledger_backing: Arc<LedgerBacking>,
    /// Outbound SMTP relay `(host, port)`, if configured
    /// (`EMAIL_SMTP_RELAY_HOST`/`EMAIL_SMTP_RELAY_PORT`). `email.send`
    /// returns an error when this is `None` rather than guessing a
    /// destination.
    pub smtp_relay: Option<(String, u16)>,
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/mcp", post(mcp_handler))
        .with_state(state)
}

async fn healthz() -> &'static str {
    "ok"
}

#[derive(Serialize)]
struct ReadyzBody {
    ready: bool,
    module_id: String,
}

async fn readyz(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        axum::Json(ReadyzBody {
            ready: true,
            module_id: state.module_id.clone(),
        }),
    )
}
