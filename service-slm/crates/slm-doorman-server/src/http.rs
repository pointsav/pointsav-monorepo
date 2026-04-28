// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Axum routes for the Doorman.
//!
//! Endpoints:
//!   GET  /healthz                → liveness, always 200
//!   GET  /readyz                 → readiness, 200 once Doorman is built
//!   GET  /v1/contract            → Doorman version + YoYo contract version
//!                                  + tier configuration summary
//!   POST /v1/chat/completions    → forwards through Doorman::route
//!   POST /v1/audit/proxy         → audited external provider call (PS.4
//!                                  steps 1-3); two-entry ledger design
//!   POST /v1/audit/capture       → caller pushes local-work audit event
//!                                  (PS.4 step 4); single-entry ledger write
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
    ApprenticeshipAttempt, ApprenticeshipBrief, AuditCaptureRequest, AuditCaptureResponse,
    AuditProxyRequest, ChatMessage, Complexity, ComputeRequest, ComputeResponse, ModuleId,
    RequestId,
};
use slm_doorman::ledger::{
    ENTRY_TYPE_AUDIT_CAPTURE, ENTRY_TYPE_AUDIT_PROXY, ENTRY_TYPE_AUDIT_PROXY_STUB,
};
use slm_doorman::{
    ApprenticeshipConfig, ApprenticeshipDispatcher, AuditCaptureEntry, AuditProxyClient,
    AuditProxyEntry, AuditProxyPurposeAllowlist, AuditProxyStubEntry, BriefCache, Doorman,
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
    /// `POST /v1/audit/proxy` relay client (PS.4 step 2). `Some` when at
    /// least one Tier C provider (Anthropic / Gemini / OpenAI) is configured
    /// via `SLM_TIER_C_*_ENDPOINT` env vars at boot; `None` returns 503 with
    /// an "unconfigured" message rather than the step-1 placeholder message.
    pub audit_proxy_client: Option<AuditProxyClient>,
    /// Purpose allowlist for `POST /v1/audit/proxy` (PS.4 step 3).
    /// Requests with a purpose not in this list are rejected 403 FORBIDDEN
    /// BEFORE any upstream provider call or audit-ledger stub write.
    ///
    /// Empty allowlist = fail-closed: all purposes are denied. Use
    /// `FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST` for the four documented purposes.
    pub audit_proxy_purpose_allowlist: AuditProxyPurposeAllowlist,
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
        .route("/v1/audit/capture", post(audit_capture))
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

/// `POST /v1/audit/proxy` — audited external provider call (PS.4 step 2).
///
/// Two-entry ledger design:
///   1. Stub entry written immediately after validation, before the upstream
///      call. Status: "inbound". This ensures a paper trail exists for every
///      inbound attempt even if the upstream call fails or the process
///      crashes mid-relay.
///   2. Full `AuditProxyEntry` written after the upstream call returns (Ok or
///      Err). Status: "ok" or "upstream-error". This entry carries token
///      counts, cost, latency, and (on error) the error message.
///
/// When `AppState.audit_proxy_client` is `None` (no Tier C providers
/// configured at startup): step 1 stub is still written, then a 503 with
/// "audit_proxy unconfigured" is returned. The "pending PS.4 step 2"
/// message from the scaffold phase is retired; callers now see a clear
/// configuration-gap message instead.
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

    // 1c. Validate purpose — non-empty.
    if body.purpose.trim().is_empty() {
        return ApiError::bad_request("audit_proxy purpose must be non-empty").into_response();
    }

    // 1d. Purpose allowlist check (PS.4 step 3).
    //
    // Runs AFTER the non-empty check (an empty purpose is a separate
    // validation error) and BEFORE audit_id generation / stub ledger write.
    //
    // Ordering rationale: an un-allowlisted purpose means "this call
    // should not be recorded as a legitimate audit entry". Writing a stub
    // ledger entry for every policy-denied request would pollute the audit
    // trail with noise. The allowlist check is the caller-side policy gate;
    // the stub write is the server-side paper trail for calls that pass
    // policy. The two are in the correct order.
    //
    // When audit_proxy_client is None (503-unconfigured path), the allowlist
    // check still runs: a request with an un-allowlisted purpose is 403 even
    // if no providers are configured. This prevents callers from probing the
    // allowlist via the unconfigured path.
    if !state
        .audit_proxy_purpose_allowlist
        .is_allowed(&body.purpose)
    {
        let err: ApiError = DoormanError::AuditProxyPurposeNotAllowlisted {
            purpose: body.purpose.clone(),
        }
        .into();
        return err.into_response();
    }

    // 1e. Validate messages — at least one required.
    if body.messages.is_empty() {
        return ApiError::bad_request("audit_proxy messages must be non-empty").into_response();
    }

    // 2. Generate a UUIDv7 audit_id.
    let audit_id = RequestId::new().to_string();
    let inbound_at = Utc::now();

    // 3. Write the ledger stub entry (entry #1 of the two-entry design).
    //    Written before the upstream call so we have a paper trail even if
    //    the relay call fails or the process crashes.
    let stub = AuditProxyStubEntry {
        entry_type: ENTRY_TYPE_AUDIT_PROXY_STUB.to_string(),
        audit_id: audit_id.clone(),
        inbound_at,
        module_id: module_id.clone(),
        purpose: body.purpose.clone(),
        provider: provider_lc.clone(),
        model: body.model.clone(),
        caller_request_id: body.caller_request_id.clone(),
        request_messages_count: body.messages.len(),
        status: "inbound".to_string(),
    };
    if let Err(e) = state.doorman.ledger().append_proxy_stub(&stub) {
        let err: ApiError = e.into();
        return err.into_response();
    }

    // 4. Relay or return unconfigured 503.
    let client = match &state.audit_proxy_client {
        Some(c) => c,
        None => {
            // No Tier C providers configured at startup. The stub entry was
            // already written (preserves inbound paper trail). Return 503
            // with a clear configuration-gap message.
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "audit_id": audit_id,
                    "caller_request_id": body.caller_request_id,
                    "error": "audit_proxy unconfigured: no Tier C providers found in environment"
                })),
            )
                .into_response();
        }
    };

    // 5. Call the relay. Capture start time for latency.
    let relay_start = std::time::Instant::now();
    let relay_result = client.relay(&body, &audit_id).await;
    let latency_ms = relay_start.elapsed().as_millis() as u64;
    let completed_at = Utc::now();

    // 6. Write the final outcome entry (entry #2 of the two-entry design).
    match &relay_result {
        Ok(resp) => {
            let entry = AuditProxyEntry {
                entry_type: ENTRY_TYPE_AUDIT_PROXY.to_string(),
                audit_id: audit_id.clone(),
                completed_at,
                module_id: module_id.clone(),
                purpose: body.purpose.clone(),
                provider: provider_lc,
                model: body.model.clone(),
                caller_request_id: body.caller_request_id.clone(),
                prompt_tokens: resp.usage.prompt_tokens,
                completion_tokens: resp.usage.completion_tokens,
                cost_usd: resp.usage.cost_usd,
                latency_ms,
                status: "ok".to_string(),
                error_message: None,
            };
            if let Err(e) = state.doorman.ledger().append_proxy_entry(&entry) {
                // Ledger write failure after a successful relay: surface as
                // 500. The response content is discarded to avoid sending a
                // success response without a corresponding ledger entry.
                let err: ApiError = e.into();
                return err.into_response();
            }
            (
                StatusCode::OK,
                Json(serde_json::to_value(resp).expect("AuditProxyResponse is serialisable")),
            )
                .into_response()
        }
        Err(e) => {
            let entry = AuditProxyEntry {
                entry_type: ENTRY_TYPE_AUDIT_PROXY.to_string(),
                audit_id: audit_id.clone(),
                completed_at,
                module_id: module_id.clone(),
                purpose: body.purpose.clone(),
                provider: provider_lc,
                model: body.model.clone(),
                caller_request_id: body.caller_request_id.clone(),
                prompt_tokens: 0,
                completion_tokens: 0,
                cost_usd: 0.0,
                latency_ms,
                status: "upstream-error".to_string(),
                error_message: Some(e.to_string()),
            };
            // Best-effort final ledger write on error. Log but do not
            // shadow the original error if the ledger write also fails.
            if let Err(ledger_err) = state.doorman.ledger().append_proxy_entry(&entry) {
                tracing::warn!(
                    target: "slm_doorman::audit_proxy",
                    audit_id = %audit_id,
                    error = %ledger_err,
                    "failed to append final audit_proxy entry after upstream error"
                );
            }
            let api_err: ApiError = DoormanError::UpstreamShape(e.to_string()).into();
            api_err.into_response()
        }
    }
}

/// Maximum permitted size of the `payload` field in an `AuditCaptureRequest`.
/// Payloads larger than this limit are rejected 413 PAYLOAD_TOO_LARGE before
/// any ledger write, preventing denial-of-service via giant payloads.
pub const AUDIT_CAPTURE_MAX_PAYLOAD_BYTES: usize = 16 * 1024; // 16 KiB

/// Accepted `event_type` values for `POST /v1/audit/capture`.
const AUDIT_CAPTURE_VALID_EVENT_TYPES: &[&str] = &[
    "prose-edit",
    "design-edit",
    "graph-mutation",
    "anchor-event",
    "verdict-issued",
];

/// `POST /v1/audit/capture` — caller pushes a local-work audit event (PS.4
/// step 4).
///
/// The inverse direction of `audit_proxy`: cross-cluster callers push audit
/// events to the Doorman for work they performed LOCALLY without routing
/// through the Doorman. Examples:
///   - project-data anchor-emitter ingesting a new file batch
///   - project-language editorial gateway running a local prose-edit pass
///
/// Validation order:
///   1. Parse `module_id` as `ModuleId`; reject 400 on failure.
///   2. Validate `event_type` against the five accepted values; reject 400.
///   3. Validate `source` is non-empty; reject 400.
///   4. Validate `status` is non-empty; reject 400.
///   5. Parse `event_at` as RFC 3339; reject 400 on failure.
///   6. Check payload size ≤ `AUDIT_CAPTURE_MAX_PAYLOAD_BYTES`; reject 413.
///
/// On success: write one `AuditCaptureEntry` to the ledger; return 200 with
/// `AuditCaptureResponse { audit_id, caller_request_id, status: "captured" }`.
async fn audit_capture(
    State(state): State<Arc<AppState>>,
    Json(body): Json<AuditCaptureRequest>,
) -> impl IntoResponse {
    // 1. Validate module_id.
    let module_id = match ModuleId::from_str(&body.module_id) {
        Ok(mid) => mid,
        Err(e) => {
            return ApiError::bad_request(format!("invalid module_id: {e}")).into_response();
        }
    };

    // 2. Validate event_type.
    if !AUDIT_CAPTURE_VALID_EVENT_TYPES.contains(&body.event_type.as_str()) {
        let err: ApiError = DoormanError::AuditCaptureUnknownEventType {
            event_type: body.event_type.clone(),
        }
        .into();
        return err.into_response();
    }

    // 3. Validate source is non-empty.
    if body.source.trim().is_empty() {
        return ApiError::bad_request("audit_capture source must be non-empty").into_response();
    }

    // 4. Validate status is non-empty.
    if body.status.trim().is_empty() {
        return ApiError::bad_request("audit_capture status must be non-empty").into_response();
    }

    // 5. Parse event_at as RFC 3339.
    let event_at: chrono::DateTime<Utc> = match body.event_at.parse() {
        Ok(ts) => ts,
        Err(_) => {
            let err: ApiError = DoormanError::AuditCaptureInvalidTimestamp {
                value: body.event_at.clone(),
            }
            .into();
            return err.into_response();
        }
    };

    // 6. Check payload size.
    let payload_bytes = body.payload.to_string().len();
    if payload_bytes > AUDIT_CAPTURE_MAX_PAYLOAD_BYTES {
        let err: ApiError = DoormanError::AuditCapturePayloadTooLarge {
            size_bytes: payload_bytes,
            max_bytes: AUDIT_CAPTURE_MAX_PAYLOAD_BYTES,
        }
        .into();
        return err.into_response();
    }

    // 7. Write the capture entry to the ledger.
    let captured_at = Utc::now();
    let entry = AuditCaptureEntry {
        entry_type: ENTRY_TYPE_AUDIT_CAPTURE.to_string(),
        audit_id: body.audit_id.clone(),
        module_id,
        event_type: body.event_type.clone(),
        source: body.source.clone(),
        status: body.status.clone(),
        event_at,
        captured_at,
        payload: body.payload.clone(),
        caller_request_id: body.caller_request_id.clone(),
    };
    if let Err(e) = state.doorman.ledger().append_capture_entry(&entry) {
        let err: ApiError = e.into();
        return err.into_response();
    }

    // 8. Return 200 with confirmation.
    let resp = AuditCaptureResponse {
        audit_id: body.audit_id,
        caller_request_id: body.caller_request_id,
        status: "captured".to_string(),
    };
    (StatusCode::OK, Json(resp)).into_response()
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
            // The audit_proxy targeted a provider that is not configured at
            // startup (PS.4 step 2). Server-side configuration gap — 503
            // SERVICE_UNAVAILABLE (not 403; the caller did nothing wrong).
            DoormanError::AuditProxyProviderUnavailable { .. } => StatusCode::SERVICE_UNAVAILABLE,
            // Caller submitted a purpose not on the purpose allowlist (PS.4
            // step 3). Caller-side policy violation — 403 FORBIDDEN, same
            // classification as ExternalNotAllowlisted which mirrors this
            // pattern for Tier C task labels.
            DoormanError::AuditProxyPurposeNotAllowlisted { .. } => StatusCode::FORBIDDEN,
            // audit_capture validation failures (PS.4 step 4).
            // Unknown event_type → 400 BAD_REQUEST (caller-side input error).
            DoormanError::AuditCaptureUnknownEventType { .. } => StatusCode::BAD_REQUEST,
            // Oversized payload → 413 PAYLOAD_TOO_LARGE.
            DoormanError::AuditCapturePayloadTooLarge { .. } => StatusCode::PAYLOAD_TOO_LARGE,
            // Unparseable timestamp → 400 BAD_REQUEST.
            DoormanError::AuditCaptureInvalidTimestamp { .. } => StatusCode::BAD_REQUEST,
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
