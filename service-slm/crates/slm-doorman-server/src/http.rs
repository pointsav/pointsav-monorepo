// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Axum routes for the Doorman.
//!
//! Endpoints:
//!   GET  /healthz                → liveness, always 200
//!   GET  /readyz                 → readiness, 200 once Doorman is built
//!   GET  /v1/contract            → Doorman version + YoYo contract version
//!                                  + tier configuration summary
//!   POST /v1/chat/completions    → forwards through Doorman::route
//!   POST /v1/audit/proxy         → audited external provider call (PS.4;
//!                                  step 1 scaffold — upstream relay pending
//!                                  PS.4 step 2; returns 503 placeholder)
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
use chrono::Utc;
use serde::{Deserialize, Serialize};
use slm_core::{
    ApprenticeshipAttempt, ApprenticeshipBrief, AuditProxyRequest, ChatMessage, Complexity,
    ComputeRequest, ComputeResponse, ModuleId, RequestId,
};
use slm_doorman::{
    ApprenticeshipConfig, ApprenticeshipDispatcher, AuditProxyStubEntry, BriefCache, Doorman,
    DoormanError, VerdictDispatchOutcome, VerdictDispatcher, VerdictWireBody,
};

pub struct AppState {
    pub doorman: Doorman,
    /// `Some` when `SLM_APPRENTICESHIP_ENABLED=true` at boot; `None`
    /// disables the three apprenticeship endpoints (they return 404).
    /// Per design-pass Q9 + Master's brief.
    pub apprenticeship: Option<ApprenticeshipConfig>,
    pub brief_cache: Arc<BriefCache>,
    /// AS-3 verdict pipeline. `Some` only when apprenticeship is
    /// enabled (the dispatcher's verifier needs the workspace
    /// `allowed_signers` to be discoverable).
    pub verdict_dispatcher: Option<VerdictDispatcher>,
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/v1/contract", get(contract))
        .route("/v1/chat/completions", post(chat_completions))
        .route("/v1/brief", post(brief))
        .route("/v1/verdict", post(verdict))
        .route("/v1/shadow", post(shadow))
        .route("/v1/audit/proxy", post(audit_proxy))
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

    let tier_c_label = headers
        .get("x-foundry-tier-c-label")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
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
        tier_c_label,
        grammar: None,
    };

    state
        .doorman
        .route(&req)
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn brief(
    State(state): State<Arc<AppState>>,
    Json(brief): Json<ApprenticeshipBrief>,
) -> Result<Json<ApprenticeshipAttempt>, ApiError> {
    let cfg = state.apprenticeship.as_ref().ok_or_else(|| {
        ApiError::not_found("apprenticeship endpoints disabled (SLM_APPRENTICESHIP_ENABLED unset)")
    })?;
    let dispatcher = ApprenticeshipDispatcher::with_cache(
        &state.doorman,
        cfg.clone(),
        state.brief_cache.clone(),
    );
    let attempt = dispatcher.dispatch_brief(&brief).await?;
    Ok(Json(attempt))
}

async fn verdict(
    State(state): State<Arc<AppState>>,
    Json(wire): Json<VerdictWireBody>,
) -> Result<Json<VerdictDispatchOutcome>, ApiError> {
    let dispatcher = state.verdict_dispatcher.as_ref().ok_or_else(|| {
        ApiError::not_found("apprenticeship endpoints disabled (SLM_APPRENTICESHIP_ENABLED unset)")
    })?;
    let outcome = dispatcher.dispatch(wire).await?;
    Ok(Json(outcome))
}

/// `POST /v1/shadow` wire shape — brief + actual_diff.
#[derive(Deserialize)]
struct ShadowWireBody {
    brief: ApprenticeshipBrief,
    /// The diff that the senior actually committed (the post-hoc
    /// reference). Convention §7 path P2.
    actual_diff: String,
}

async fn shadow(
    State(state): State<Arc<AppState>>,
    Json(wire): Json<ShadowWireBody>,
) -> Result<StatusCode, ApiError> {
    let cfg = state.apprenticeship.as_ref().ok_or_else(|| {
        ApiError::not_found("apprenticeship endpoints disabled (SLM_APPRENTICESHIP_ENABLED unset)")
    })?;
    let dispatcher = ApprenticeshipDispatcher::with_cache(
        &state.doorman,
        cfg.clone(),
        state.brief_cache.clone(),
    );
    // Per Master's brief: 200 OK with empty body on success.
    // ShadowOutcome is captured internally; idempotency on retry is
    // a no-op same-200.
    let _outcome = dispatcher
        .dispatch_shadow(&wire.brief, &wire.actual_diff)
        .await?;
    Ok(StatusCode::OK)
}

/// `POST /v1/audit/proxy` — audited external provider call (PS.4 step 1).
///
/// This handler scaffolds the endpoint shape + input validation + ledger stub.
/// The upstream provider relay is not yet wired (PS.4 step 2). In the scaffold
/// phase:
///   1. Parse and validate the request (module_id, provider, purpose, messages).
///   2. Generate a UUIDv7 `audit_id`.
///   3. Write a stub entry to the audit ledger capturing the inbound request
///      shape (status: "scaffold-stub-no-relay-yet").
///   4. Return `503 SERVICE_UNAVAILABLE` with the `audit_id` and a clear
///      "pending PS.4 step 2" message. Callers see 503 rather than 501 because
///      the service IS available — it processes the request through to the
///      ledger — but the upstream relay specifically is not yet implemented.
///      501 would imply the endpoint does not exist at all; 503 communicates
///      "known limitation, retry after step 2 is deployed."
///
/// Validation failures return `400 BAD_REQUEST` with a descriptive message.
async fn audit_proxy(
    State(state): State<Arc<AppState>>,
    Json(body): Json<AuditProxyRequest>,
) -> impl IntoResponse {
    // 1a. Validate module_id.
    let module_id = match ModuleId::from_str(&body.module_id) {
        Ok(mid) => mid,
        Err(e) => {
            return ApiError::bad_request(format!("invalid module_id: {e}")).into_response();
        }
    };

    // 1b. Validate provider — "anthropic", "gemini", or "openai".
    let provider_lc = body.provider.to_ascii_lowercase();
    if !matches!(provider_lc.as_str(), "anthropic" | "gemini" | "openai") {
        let err: ApiError = DoormanError::AuditProxyInvalidProvider {
            provider: body.provider.clone(),
        }
        .into();
        return err.into_response();
    }

    // 1c. Validate purpose — non-empty (allowlist enforcement is PS.4 step 2).
    if body.purpose.trim().is_empty() {
        return ApiError::bad_request(
            "audit_proxy purpose must be non-empty; \
             allowlist enforcement lands in PS.4 step 2",
        )
        .into_response();
    }

    // 1d. Validate messages — at least one required.
    if body.messages.is_empty() {
        return ApiError::bad_request("audit_proxy messages must be non-empty").into_response();
    }

    // 2. Generate a UUIDv7 audit_id.
    let audit_id = RequestId::new().to_string();
    let inbound_at = Utc::now();

    // 3. Write the ledger stub entry so we have a paper trail for every
    //    attempted proxy call even during the scaffold phase.
    let stub = AuditProxyStubEntry {
        audit_id: audit_id.clone(),
        inbound_at,
        module_id,
        purpose: body.purpose.clone(),
        provider: provider_lc,
        model: body.model.clone(),
        caller_request_id: body.caller_request_id.clone(),
        request_messages_count: body.messages.len(),
        status: "scaffold-stub-no-relay-yet".to_string(),
    };
    if let Err(e) = state.doorman.ledger().append_proxy_stub(&stub) {
        let err: ApiError = e.into();
        return err.into_response();
    }

    // 4. Return 503 — step 2 converts this placeholder to a live response.
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "audit_id": audit_id,
            "caller_request_id": body.caller_request_id,
            "error": "audit_proxy upstream relay pending PS.4 step 2"
        })),
    )
        .into_response()
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

    fn not_found(msg: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
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
            DoormanError::ExternalNotAllowlisted { .. } | DoormanError::VerifySignature(_) => {
                StatusCode::FORBIDDEN
            }
            DoormanError::Upstream(_)
            | DoormanError::UpstreamShape(_)
            | DoormanError::ContractMajorMismatch { .. }
            | DoormanError::BearerToken(_) => StatusCode::BAD_GATEWAY,
            DoormanError::VerdictParse(_) => StatusCode::BAD_REQUEST,
            // Caller submitted an unsupported grammar dialect for the selected
            // tier. Both Tier A (e.g. Lark) and Tier C (any grammar) map to
            // 400 BAD_REQUEST: the error is on the caller's side — they must
            // either change the grammar dialect or route to a supported tier.
            DoormanError::TierAGrammarUnsupported { .. }
            | DoormanError::TierCGrammarUnsupported { .. } => StatusCode::BAD_REQUEST,
            // Caller submitted a syntactically malformed Lark grammar (PS.3
            // step 5). The parse-error message from llguidance is included in
            // the response body so the caller can fix the grammar without
            // re-routing. 400 BAD_REQUEST: error is entirely on the caller's side.
            DoormanError::MalformedLarkGrammar { .. } => StatusCode::BAD_REQUEST,
            // Caller submitted an unrecognised provider string to audit_proxy
            // (PS.4 step 1). Error is entirely on the caller's side.
            DoormanError::AuditProxyInvalidProvider { .. } => StatusCode::BAD_REQUEST,
            DoormanError::BriefCacheMiss => StatusCode::GONE,
            DoormanError::LedgerIo(_)
            | DoormanError::LedgerSerde(_)
            | DoormanError::HomeUnset
            | DoormanError::LedgerLock(_)
            | DoormanError::CorpusWrite { .. } => StatusCode::INTERNAL_SERVER_ERROR,
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
