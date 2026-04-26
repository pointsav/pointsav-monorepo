// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Axum routes for the Doorman.
//!
//! Endpoints:
//!   GET  /healthz                → liveness, always 200
//!   GET  /readyz                 → readiness, 200 once Doorman is built
//!   GET  /v1/contract            → Doorman version + YoYo contract version
//!                                  + tier configuration summary
//!   POST /v1/chat/completions    → forwards through Doorman::route
//!
//! The /v1/chat/completions handler accepts an OpenAI-compatible body
//! plus optional X-Foundry-* headers (Module-ID, Request-ID,
//! Complexity). When headers are absent, it generates safe defaults so
//! ad-hoc curl probes work in development; production callers SHOULD
//! supply them per CONTRACT.md.

use std::str::FromStr;
use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Json};
use axum::routing::{get, post};
use axum::Router;
use serde::{Deserialize, Serialize};
use slm_core::{ChatMessage, Complexity, ComputeRequest, ComputeResponse, ModuleId, RequestId};
use slm_doorman::{Doorman, DoormanError};

pub struct AppState {
    pub doorman: Doorman,
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/v1/contract", get(contract))
        .route("/v1/chat/completions", post(chat_completions))
        .with_state(state)
}

async fn healthz() -> &'static str {
    "ok"
}

async fn readyz(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // The Doorman is always ready in B1; B5+ may add upstream-tier
    // readiness checks (e.g., probe Tier A /healthz) before flipping.
    let body = serde_json::json!({
        "ready": true,
        "has_local": state.doorman.has_local(),
        "has_yoyo": state.doorman.has_yoyo(),
        "has_external": state.doorman.has_external(),
    });
    (StatusCode::OK, Json(body))
}

#[derive(Serialize)]
struct ContractInfo {
    doorman_version: &'static str,
    yoyo_contract_version: &'static str,
    has_local: bool,
    has_yoyo: bool,
    has_external: bool,
}

async fn contract(State(state): State<Arc<AppState>>) -> Json<ContractInfo> {
    Json(ContractInfo {
        doorman_version: slm_doorman::DOORMAN_VERSION,
        yoyo_contract_version: slm_doorman::YOYO_CONTRACT_VERSION,
        has_local: state.doorman.has_local(),
        has_yoyo: state.doorman.has_yoyo(),
        has_external: state.doorman.has_external(),
    })
}

#[derive(Deserialize)]
struct ChatCompletionsBody {
    model: Option<String>,
    messages: Vec<ChatMessage>,
    #[serde(default)]
    stream: bool,
    #[serde(default)]
    max_tokens: Option<u32>,
    #[serde(default)]
    temperature: Option<f32>,
}

async fn chat_completions(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<ChatCompletionsBody>,
) -> Result<Json<ComputeResponse>, ApiError> {
    let module_id = match headers
        .get("x-foundry-module-id")
        .and_then(|v| v.to_str().ok())
    {
        Some(s) => ModuleId::from_str(s)
            .map_err(|e| ApiError::bad_request(format!("invalid X-Foundry-Module-ID: {e}")))?,
        None => ModuleId::from_str("foundry").expect("compile-time-valid default moduleId"),
    };
    let request_id = match headers
        .get("x-foundry-request-id")
        .and_then(|v| v.to_str().ok())
    {
        Some(s) => RequestId::from_str(s)
            .map_err(|e| ApiError::bad_request(format!("invalid X-Foundry-Request-ID: {e}")))?,
        None => RequestId::new(),
    };
    let complexity = headers
        .get("x-foundry-complexity")
        .and_then(|v| v.to_str().ok())
        .map(|s| match s {
            "low" => Complexity::Low,
            "high" => Complexity::High,
            _ => Complexity::Medium,
        })
        .unwrap_or_default();

    let req = ComputeRequest {
        request_id,
        module_id,
        model: body.model,
        messages: body.messages,
        complexity,
        tier_hint: None,
        stream: body.stream,
        max_tokens: body.max_tokens,
        temperature: body.temperature,
        sanitised_outbound: false,
    };

    state
        .doorman
        .route(&req)
        .await
        .map(Json)
        .map_err(Into::into)
}

struct ApiError {
    status: StatusCode,
    body: serde_json::Value,
}

impl ApiError {
    fn bad_request(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            body: serde_json::json!({ "error": { "message": msg.into() } }),
        }
    }
}

impl From<DoormanError> for ApiError {
    fn from(e: DoormanError) -> Self {
        let status = match &e {
            DoormanError::TierUnavailable(_) | DoormanError::NotImplemented { .. } => {
                StatusCode::SERVICE_UNAVAILABLE
            }
            DoormanError::ExternalNotAllowlisted { .. } => StatusCode::FORBIDDEN,
            DoormanError::Upstream(_)
            | DoormanError::UpstreamShape(_)
            | DoormanError::ContractMajorMismatch { .. }
            | DoormanError::BearerToken(_) => StatusCode::BAD_GATEWAY,
            DoormanError::LedgerIo(_) | DoormanError::LedgerSerde(_) | DoormanError::HomeUnset => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        Self {
            status,
            body: serde_json::json!({ "error": { "message": e.to_string() } }),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self.body)).into_response()
    }
}
