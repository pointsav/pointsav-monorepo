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
//!   POST /v1/graph/query         → graph-context proxy; forwards to
//!                                  service-content /v1/graph/context and
//!                                  audit-logs as event_type=graph-query
//!   POST /v1/graph/mutate        → graph-mutate proxy; forwards to
//!                                  service-content /v1/graph/mutate and
//!                                  audit-logs as event_type=graph-mutation
//!
//! The /v1/chat/completions handler accepts an OpenAI-compatible body
//! plus optional X-Foundry-* headers (Module-ID, Request-ID,
//! Complexity). When headers are absent, it generates safe defaults so
//! ad-hoc curl probes work in development; production callers SHOULD
//! supply them per CONTRACT.md.

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Json, Response};
use axum::routing::{get, post};
use axum::Router;
use reqwest::Client as ReqwestClient;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use slm_core::{
    ApprenticeshipAttempt, ApprenticeshipBrief, AuditCaptureRequest, AuditCaptureResponse,
    AuditProxyRequest, ChatMessage, Complexity, ComputeRequest, DeferReason, ExtractionRequest,
    ExtractionResponse, GrammarConstraint, ModuleId, RequestId, Tier,
};
use slm_doorman::ledger::{
    ENTRY_TYPE_AUDIT_CAPTURE, ENTRY_TYPE_AUDIT_PROXY, ENTRY_TYPE_AUDIT_PROXY_STUB,
    ENTRY_TYPE_EXTRACT,
};
use slm_doorman::{
    ApprenticeshipConfig, ApprenticeshipDispatcher, AuditCaptureEntry, AuditProxyClient,
    AuditProxyEntry, AuditProxyPurposeAllowlist, AuditProxyStubEntry, BriefCache, Doorman,
    DoormanError, ExtractionAuditEntry, VerdictDispatchOutcome, VerdictDispatcher, VerdictWireBody,
};
use tokio::sync::Semaphore;

use crate::queue::QueueConfig;

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
    /// Per-tenant (moduleId) in-flight request semaphores shared across BOTH
    /// `/v1/audit/proxy` AND `/v1/audit/capture`.
    ///
    /// Keyed by `ModuleId`. The inner `Arc<Semaphore>` holds N permits where N
    /// is `SLM_AUDIT_TENANT_CONCURRENCY_CAP` (default 4). A new entry is
    /// created lazily on the first request from a tenant (`lazy-init`).
    ///
    /// Using `Arc<Mutex<HashMap<...>>>` (no new dep; `dashmap` is not in the
    /// workspace). The lock is held only for map lookup / insertion (O(1)); it
    /// is released before the semaphore acquire, so no long-held lock.
    pub audit_tenant_concurrency: Arc<Mutex<HashMap<ModuleId, Arc<Semaphore>>>>,
    /// Maximum number of concurrent in-flight audit requests per tenant.
    /// Configurable via `SLM_AUDIT_TENANT_CONCURRENCY_CAP`; default 4.
    pub audit_tenant_concurrency_cap: u32,
    /// Queue configuration for `POST /v1/shadow` — the brief is enqueued
    /// here and the drain worker dispatches to the apprentice asynchronously.
    /// Injected so tests can use a tempdir-backed queue without coupling to
    /// `SLM_APPRENTICESHIP_BASE_DIR` / `FOUNDRY_ROOT` env vars.
    pub queue_config: Arc<QueueConfig>,
    /// Base URL for service-content's HTTP API used by the graph proxy
    /// handlers (`POST /v1/graph/query` and `POST /v1/graph/mutate`).
    /// Defaults to `http://127.0.0.1:9081` when `SERVICE_CONTENT_ENDPOINT`
    /// is not set. Set to an empty string to mark the proxy as unconfigured
    /// (handlers return 503 with `GraphProxyServiceUnavailable`).
    pub service_content_endpoint: String,
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/v1/contract", get(contract))
        .route("/v1/messages", post(anthropic_messages))
        .route("/v1/chat/completions", post(chat_completions))
        .route("/v1/brief", post(brief))
        .route("/v1/verdict", post(verdict))
        .route("/v1/shadow", post(shadow))
        .route("/v1/extract", post(extract))
        .route("/v1/audit/proxy", post(audit_proxy))
        .route("/v1/audit/capture", post(audit_capture))
        .route("/v1/graph/query", post(graph_query))
        .route("/v1/graph/mutate", post(graph_mutate))
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
    /// Optional decode-time grammar constraint forwarded to the selected
    /// tier backend. Callers that don't set this field get `None` (the
    /// default), leaving tier routing behaviour unchanged.
    #[serde(default)]
    grammar: Option<GrammarConstraint>,
}

async fn chat_completions(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<ChatCompletionsBody>,
) -> Result<impl IntoResponse, ApiError> {
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
    let yoyo_label = headers
        .get("x-foundry-yoyo-label")
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
        yoyo_label,
        grammar: body.grammar,
        speculation: None,
        graph_context_enabled: None,
    };

    let resp = state.doorman.route(&req).await.map_err(ApiError::from)?;
    let tier_str = resp.tier_used.as_str().to_string();
    let mut resp_headers = HeaderMap::new();
    if let Ok(v) = tier_str.parse() {
        resp_headers.insert("x-foundry-tier-used", v);
    }
    Ok((resp_headers, Json(resp)))
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

/// Response body for a successful `POST /v1/shadow` enqueue (202 ACCEPTED).
///
/// Per apprenticeship-substrate.md §7C step 3: the handler returns
/// immediately after durable disk-write, without blocking on apprentice
/// execution. The drain worker dispatches to the apprentice and writes
/// the corpus tuple on completion.
#[derive(Serialize)]
struct ShadowAcceptedBody {
    /// UUIDv7 identifier for this shadow brief — matches `brief.brief_id`
    /// from the request (generated by the caller; the handler preserves it).
    audit_id: String,
    /// Approximate position in the queue at enqueue time (best-effort;
    /// concurrent enqueues may shift the actual order). Useful as a
    /// caller hint; not a strict reservation.
    queue_position: usize,
    /// Echoes the brief_id from the wire body for caller convenience.
    brief_id: String,
}

/// `POST /v1/shadow` — Brief Queue Substrate entry point (§7C step 3).
///
/// Validates the brief, enqueues it to disk via `queue::enqueue()`, and
/// returns `202 ACCEPTED` with `{audit_id, queue_position, brief_id}`.
///
/// The apprentice dispatch and corpus-tuple write happen in the background
/// drain worker (iter-22 main.rs), NOT in this handler. This decouples
/// HTTP-handler latency (milliseconds) from apprentice execution latency
/// (seconds–minutes on Tier A CPU, or on Yo-Yo wake time).
///
/// Audit-ledger writes are deferred to the drain worker — single entry per
/// brief, written when the apprentice completes. The handler's responsibility
/// is durable enqueue only: if the queue write succeeds, the brief will
/// eventually be dispatched even through process restart or Yo-Yo idle-shutdown.
///
/// Validation failures (400/403/404) still return synchronously — those are
/// caller-side errors that do not benefit from async handling.
async fn shadow(
    State(state): State<Arc<AppState>>,
    Json(wire): Json<ShadowWireBody>,
) -> Result<(StatusCode, Json<ShadowAcceptedBody>), ApiError> {
    // 404 if apprenticeship is disabled — same gate as /v1/brief and /v1/verdict.
    let _cfg = state.apprenticeship.as_ref().ok_or_else(|| {
        ApiError::not_found("apprenticeship endpoints disabled (SLM_APPRENTICESHIP_ENABLED unset)")
    })?;

    // Preserve the caller's brief_id as the audit_id. The brief_id is the
    // deterministic idempotency key for the queue file
    // (`<brief_id>.brief.jsonl`); using it as audit_id lets callers correlate
    // the 202 response with the eventual corpus tuple by brief_id alone.
    let audit_id = wire.brief.brief_id.clone();
    let brief_id = wire.brief.brief_id.clone();

    // Enqueue. This writes `<queue_dir>/<brief_id>.brief.jsonl` atomically.
    // The brief carries the `actual_diff` embedded via the queue file; the
    // worker reads both when it dequeues and dispatches.
    //
    // NOTE: `ApprenticeshipBrief` does not carry `actual_diff` (it is a
    // corpus-capture-side field, not part of the brief wire format). The
    // worker in main.rs calls `dispatch_shadow(&brief, "")` and relies on
    // the full ShadowWireBody shape for the actual_diff. To preserve
    // `actual_diff` through the queue, we embed it in the brief's `body`
    // field as a JSON envelope, OR we store the full ShadowWireBody.
    //
    // Chosen approach: serialise the ShadowWireBody (brief + actual_diff)
    // as the queue file content so the worker has both fields. The queue
    // file is identified by `brief_id`; the wire type is the payload.
    // `queue::enqueue` expects `ApprenticeshipBrief`; we route around this
    // by serialising the full ShadowWireBody directly into the queue file
    // without using `queue::enqueue`. This is a thin wrapper that reuses
    // the queue dir layout and naming convention but writes the wider type.
    // The drain worker already does `dispatch_shadow(&leased.brief, "")` —
    // we need to change this to pass the actual_diff too.
    //
    // Alternative chosen to avoid redesigning the queue API: store the
    // `actual_diff` in the brief's existing `body` field using a sentinel
    // prefix, OR add a separate `.actual_diff` sidecar file. Simpler:
    // extend the queue to accept a `ShadowQueueEntry` that wraps the
    // existing brief plus `actual_diff`. We add a `enqueue_shadow()` variant
    // to queue.rs that serialises a `ShadowQueueEntry` struct (brief + diff)
    // under the same filename convention.
    //
    // See `queue::ShadowQueueEntry` and `queue::enqueue_shadow()` added in
    // this iter.
    let shadow_entry = crate::queue::ShadowQueueEntry {
        brief: wire.brief.clone(),
        actual_diff: wire.actual_diff.clone(),
    };
    let entry =
        crate::queue::enqueue_shadow(&state.queue_config, &shadow_entry).map_err(ApiError::from)?;

    // Best-effort queue position — count files AFTER the write so the
    // caller's file is included in the count.
    let queue_position = crate::queue::pending_count(&state.queue_config);

    tracing::info!(
        brief_id = %brief_id,
        queue_path = %entry.queue_path.display(),
        queue_position,
        "shadow brief enqueued (202 ACCEPTED); drain worker will dispatch"
    );

    Ok((
        StatusCode::ACCEPTED,
        Json(ShadowAcceptedBody {
            audit_id,
            queue_position,
            brief_id,
        }),
    ))
}

/// Attempt to acquire a per-tenant concurrency permit for the audit endpoints.
///
/// Both `/v1/audit/proxy` and `/v1/audit/capture` share the same per-tenant
/// semaphore map. The total count of in-flight requests across BOTH endpoints
/// counts against the per-tenant cap (`audit_tenant_concurrency_cap`).
///
/// Implementation:
///   1. Lock the map (O(1) lookup), retrieve or lazily-create the tenant's
///      `Arc<Semaphore>`.
///   2. Release the map lock immediately so we do not hold it during the
///      semaphore acquire.
///   3. Call `try_acquire_owned()` — non-blocking; fails immediately if no
///      permits available.
///   4. On failure, return `DoormanError::AuditTenantConcurrencyExhausted`.
///
/// The returned `OwnedSemaphorePermit` is held for the rest of the caller's
/// scope and is automatically released (RAII) when the handler returns.
fn acquire_tenant_permit(
    state: &AppState,
    module_id: &ModuleId,
) -> Result<tokio::sync::OwnedSemaphorePermit, DoormanError> {
    let semaphore = {
        let mut map = state
            .audit_tenant_concurrency
            .lock()
            .expect("audit_tenant_concurrency Mutex poisoned");
        map.entry(module_id.clone())
            .or_insert_with(|| {
                Arc::new(Semaphore::new(state.audit_tenant_concurrency_cap as usize))
            })
            .clone()
        // map lock released here
    };

    semaphore
        .try_acquire_owned()
        .map_err(|_| DoormanError::AuditTenantConcurrencyExhausted {
            module_id: module_id.to_string(),
            cap: state.audit_tenant_concurrency_cap,
        })
}

/// Maximum permitted raw body size for `POST /v1/extract`.
/// 256 KiB: large enough for a 3000-token corpus; small enough to resist DoS.
pub const EXTRACTION_MAX_REQUEST_BYTES: usize = 256 * 1024; // 256 KiB

/// `POST /v1/extract` — dedicated entity extraction endpoint.
///
/// Routes exclusively to Yo-Yo "trainer" (OLMo 3 32B-Think) via
/// `route_yoyo_only()`. Does NOT fall back to Tier A — OLMo 7B cannot
/// produce structured JSON arrays reliably and must never serve as a
/// fallback for extraction (SYS-ADR-07).
///
/// Response is always HTTP 200:
/// - `extraction_ok: true`  → `entities` contains the extracted array
/// - `deferred: true`       → Yo-Yo unavailable; caller retries with backoff
///
/// ADR-07 boundary: `ExtractionRequest.text` is prose only. The `schema`
/// field constrains the OUTPUT; structured graph data must never cross the
/// AI boundary as prompt input.
async fn extract(State(state): State<Arc<AppState>>, raw: Bytes) -> impl IntoResponse {
    // 0. Body-size cap — before deserialisation.
    if raw.len() > EXTRACTION_MAX_REQUEST_BYTES {
        return ApiError::bad_request(format!(
            "extract request is {} bytes, exceeding the {}-byte limit; reduce payload",
            raw.len(),
            EXTRACTION_MAX_REQUEST_BYTES,
        ))
        .into_response();
    }

    // 1. Deserialise — deny_unknown_fields enforced by ExtractionRequest (ADR-07).
    let req: ExtractionRequest = match serde_json::from_slice(&raw) {
        Ok(r) => r,
        Err(e) => return ApiError::bad_request(format!("invalid JSON body: {e}")).into_response(),
    };

    // 2. Validate module_id.
    let module_id = match ModuleId::from_str(&req.module_id) {
        Ok(mid) => mid,
        Err(e) => {
            return ApiError::bad_request(format!("invalid module_id: {e}")).into_response()
        }
    };

    // 3. Per-tenant concurrency permit (shared semaphore with audit endpoints).
    let _permit = match acquire_tenant_permit(&state, &module_id) {
        Ok(p) => p,
        Err(e) => {
            let mut resp = ApiError::from(e).into_response();
            resp.headers_mut().insert(
                axum::http::header::RETRY_AFTER,
                axum::http::HeaderValue::from_static("5"),
            );
            return resp;
        }
    };

    // 4. Build ComputeRequest targeting Yo-Yo "trainer" with JsonSchema grammar.
    let request_id = RequestId::new();
    let compute_req = ComputeRequest {
        request_id,
        module_id: module_id.clone(),
        model: None,
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: "Extract named entities. Return a JSON array matching the schema exactly.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: req.text,
            },
        ],
        complexity: Complexity::High,
        tier_hint: Some(Tier::Yoyo),
        stream: false,
        max_tokens: Some(2048),
        temperature: Some(0.1),
        sanitised_outbound: true,
        tier_c_label: None,
        yoyo_label: Some("trainer".to_string()),
        grammar: Some(GrammarConstraint::JsonSchema(req.schema)),
        speculation: None,
        graph_context_enabled: None,
    };

    // 5. Route — no Tier A fallback.
    let start = std::time::Instant::now();
    let result = state.doorman.route_yoyo_only(&compute_req, "trainer").await;
    let latency_ms = start.elapsed().as_millis() as u64;

    // Capture error message before moving result.
    let error_message_for_audit = result.as_ref().err().map(|e| e.to_string());

    // 6. Parse result into response fields.
    let (entities, tier_used, model, extraction_ok, deferred, defer_reason_str) = match result {
        Ok(compute_resp) => {
            // Strip markdown fences if the model wrapped its output.
            let raw_content = compute_resp.content.trim().to_string();
            let stripped = raw_content
                .strip_prefix("```json")
                .unwrap_or(&raw_content)
                .strip_prefix("```")
                .unwrap_or(&raw_content);
            let stripped = stripped.strip_suffix("```").unwrap_or(stripped).trim();
            match serde_json::from_str::<Vec<serde_json::Value>>(stripped) {
                Ok(ents) => (
                    ents,
                    "yoyo_trainer".to_string(),
                    compute_resp.model,
                    true,
                    false,
                    None::<String>,
                ),
                Err(_) => (
                    vec![],
                    "deferred".to_string(),
                    "none".to_string(),
                    false,
                    true,
                    Some("yoyo-transient".to_string()),
                ),
            }
        }
        Err(DoormanError::TierUnavailable(_)) => (
            vec![],
            "deferred".to_string(),
            "none".to_string(),
            false,
            true,
            Some("yoyo-circuit-open".to_string()),
        ),
        Err(_) => (
            vec![],
            "deferred".to_string(),
            "none".to_string(),
            false,
            true,
            Some("yoyo-transient".to_string()),
        ),
    };

    // 7. Audit entry (non-fatal if write fails — never block the response).
    let audit_entry = ExtractionAuditEntry {
        entry_type: ENTRY_TYPE_EXTRACT.to_string(),
        timestamp_utc: Utc::now(),
        request_id,
        module_id,
        extraction_ok,
        deferred,
        entities_count: entities.len(),
        tier_used: tier_used.clone(),
        latency_ms,
        defer_reason: defer_reason_str.clone(),
        error_message: error_message_for_audit,
    };
    if let Err(write_err) = state.doorman.ledger().append_extract_entry(&audit_entry) {
        tracing::warn!(
            target: "slm_doorman::extract",
            error = %write_err,
            request_id = %request_id,
            "failed to write extraction audit entry"
        );
    }

    // 8. Build typed response with DeferReason enum.
    let defer_reason_enum = defer_reason_str.as_deref().map(|s| match s {
        "yoyo-circuit-open" => DeferReason::YoyoCircuitOpen,
        "yoyo-label-unconfigured" => DeferReason::YoyoLabelUnconfigured,
        _ => DeferReason::YoyoTransient,
    });

    Json(ExtractionResponse {
        entities,
        tier_used,
        model,
        extraction_ok,
        deferred,
        defer_reason: defer_reason_enum,
    })
    .into_response()
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
/// Hardening (added post-PS.4):
///   - `AUDIT_PROXY_MAX_REQUEST_BYTES` (64 KiB) raw body-size cap checked
///     BEFORE JSON deserialisation → 413 on violation.
///   - Per-tenant (moduleId) in-flight concurrency cap (default 4) shared
///     with `/v1/audit/capture`. Excess → 503 with `Retry-After: 5`.
///
/// Validation failures return `400 BAD_REQUEST` with a descriptive message.
async fn audit_proxy(State(state): State<Arc<AppState>>, raw: Bytes) -> impl IntoResponse {
    // 0. Body-size cap — checked BEFORE deserialisation.
    //    This is the primary DoS guard: reject oversized bodies early without
    //    allocating heap memory for the JSON value. `Bytes` extraction does
    //    NOT allocate a serde tree; the size check is O(1) against the buffer
    //    length already present in the pre-read bytes.
    if raw.len() > AUDIT_PROXY_MAX_REQUEST_BYTES {
        let err: ApiError = DoormanError::AuditProxyPayloadTooLarge {
            size_bytes: raw.len(),
            max_bytes: AUDIT_PROXY_MAX_REQUEST_BYTES,
        }
        .into();
        return err.into_response();
    }

    // Deserialise from the raw bytes.
    let body: AuditProxyRequest = match serde_json::from_slice(&raw) {
        Ok(b) => b,
        Err(e) => {
            return ApiError::bad_request(format!("invalid JSON body: {e}")).into_response();
        }
    };

    // 1a. Validate module_id.
    let module_id = match ModuleId::from_str(&body.module_id) {
        Ok(mid) => mid,
        Err(e) => {
            return ApiError::bad_request(format!("invalid module_id: {e}")).into_response();
        }
    };

    // 1a'. Acquire per-tenant concurrency permit.
    //     Checked immediately after module_id is parsed (so we have a valid
    //     ModuleId key) and before any expensive work — purpose validation,
    //     audit_id generation, ledger writes, and upstream call. The permit is
    //     held for the rest of the handler's lifetime (RAII drop on return).
    let _permit = match acquire_tenant_permit(&state, &module_id) {
        Ok(p) => p,
        Err(e) => {
            let mut resp = ApiError::from(e).into_response();
            resp.headers_mut().insert(
                axum::http::header::RETRY_AFTER,
                axum::http::HeaderValue::from_static("5"),
            );
            return resp;
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

/// Maximum permitted raw body size for `POST /v1/audit/proxy` requests.
/// Set at 64 KiB — 4× `AUDIT_CAPTURE_MAX_PAYLOAD_BYTES` because the proxy
/// carries full chat-completion `messages` arrays with potentially long user
/// prompts. The check fires BEFORE JSON deserialisation so an oversized request
/// is rejected early without allocating heap memory for the payload.
pub const AUDIT_PROXY_MAX_REQUEST_BYTES: usize = 64 * 1024; // 64 KiB

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

    // 1'. Acquire per-tenant concurrency permit — shared with audit_proxy.
    //     Both audit endpoints count against the same per-tenant cap so a
    //     tenant flooding either endpoint is rate-limited across both.
    let _permit = match acquire_tenant_permit(&state, &module_id) {
        Ok(p) => p,
        Err(e) => {
            let mut resp = ApiError::from(e).into_response();
            resp.headers_mut().insert(
                axum::http::header::RETRY_AFTER,
                axum::http::HeaderValue::from_static("5"),
            );
            return resp;
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

// ── Graph proxy constants ─────────────────────────────────────────────────────

/// Default service-content endpoint used when `SERVICE_CONTENT_ENDPOINT` is absent.
pub const DEFAULT_SERVICE_CONTENT_ENDPOINT: &str = "http://127.0.0.1:9081";

// ── Graph proxy request types ────────────────────────────────────────────────

/// Body for `POST /v1/graph/query`. The `module_id` comes from the mandatory
/// `X-Foundry-Module-ID` header; it is injected as a query parameter when
/// forwarding to service-content's GET `/v1/graph/context`.
#[derive(Deserialize)]
struct GraphQueryBody {
    q: String,
    #[serde(default = "default_graph_query_limit")]
    limit: u32,
}

fn default_graph_query_limit() -> u32 {
    10
}

/// `POST /v1/graph/query` — proxy to service-content `/v1/graph/context`.
///
/// 1. Requires `X-Foundry-Module-ID` header → 400 if absent.
/// 2. Parses `{"q": "...", "limit": N}` body.
/// 3. Forwards to `{service_content_endpoint}/v1/graph/context?q=…&module_id=…&limit=…`.
/// 4. Audit-logs as `event_type = "graph-query"` via `AuditCaptureEntry`.
/// 5. Returns service-content response verbatim.
async fn graph_query(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<GraphQueryBody>,
) -> impl IntoResponse {
    // 1. Module-ID is mandatory.
    let module_id = match headers
        .get("x-foundry-module-id")
        .and_then(|v| v.to_str().ok())
    {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => {
            let err: ApiError = DoormanError::GraphProxyMissingModuleId.into();
            return err.into_response();
        }
    };

    // 2. Service-content must be configured.
    if state.service_content_endpoint.is_empty() {
        let err: ApiError = DoormanError::GraphProxyServiceUnavailable.into();
        return err.into_response();
    }

    let url = format!(
        "{}/v1/graph/context?q={}&module_id={}&limit={}",
        state.service_content_endpoint,
        urlencoding_encode(&body.q),
        urlencoding_encode(&module_id),
        body.limit,
    );

    // 3. Forward to service-content.
    let client = ReqwestClient::new();
    let sc_resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(_) => {
            let err: ApiError = DoormanError::GraphProxyServiceUnavailable.into();
            return err.into_response();
        }
    };

    let sc_status = sc_resp.status();
    let sc_body: serde_json::Value = match sc_resp.json().await {
        Ok(v) => v,
        Err(_) => serde_json::Value::Array(vec![]),
    };

    // 4. Audit-log (non-fatal — proxy succeeds even if ledger write fails).
    let entry = AuditCaptureEntry {
        entry_type: ENTRY_TYPE_AUDIT_CAPTURE.to_string(),
        audit_id: RequestId::new().to_string(),
        module_id: slm_core::ModuleId::from_str(&module_id)
            .unwrap_or_else(|_| slm_core::ModuleId::from_str("unknown").unwrap()),
        event_type: "graph-query".to_string(),
        source: format!("graph-proxy:{}", body.q),
        status: if sc_status.is_success() { "ok" } else { "upstream-error" }.to_string(),
        event_at: Utc::now(),
        captured_at: Utc::now(),
        payload: serde_json::json!({ "q": body.q, "limit": body.limit, "module_id": module_id }),
        caller_request_id: None,
    };
    let _ = state.doorman.ledger().append_capture_entry(&entry);

    // 5. Return service-content response.
    let status = StatusCode::from_u16(sc_status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    (status, Json(sc_body)).into_response()
}

/// `POST /v1/graph/mutate` — proxy to service-content `/v1/graph/mutate`.
///
/// 1. Requires `X-Foundry-Module-ID` header → 400 if absent.
/// 2. Forwards body verbatim to service-content.
/// 3. Audit-logs as `event_type = "graph-mutation"` via `AuditCaptureEntry`.
/// 4. Returns service-content response verbatim.
async fn graph_mutate(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body_bytes: Bytes,
) -> impl IntoResponse {
    // 1. Module-ID is mandatory.
    let module_id = match headers
        .get("x-foundry-module-id")
        .and_then(|v| v.to_str().ok())
    {
        Some(s) if !s.is_empty() => s.to_string(),
        _ => {
            let err: ApiError = DoormanError::GraphProxyMissingModuleId.into();
            return err.into_response();
        }
    };

    // 2. Service-content must be configured.
    if state.service_content_endpoint.is_empty() {
        let err: ApiError = DoormanError::GraphProxyServiceUnavailable.into();
        return err.into_response();
    }

    let url = format!("{}/v1/graph/mutate", state.service_content_endpoint);

    // 3. Forward body to service-content.
    let client = ReqwestClient::new();
    let sc_resp = match client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body_bytes.to_vec())
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => {
            let err: ApiError = DoormanError::GraphProxyServiceUnavailable.into();
            return err.into_response();
        }
    };

    let sc_status = sc_resp.status();
    let sc_body: serde_json::Value = match sc_resp.json().await {
        Ok(v) => v,
        Err(_) => serde_json::json!({}),
    };

    // 4. Audit-log (non-fatal).
    let entry = AuditCaptureEntry {
        entry_type: ENTRY_TYPE_AUDIT_CAPTURE.to_string(),
        audit_id: RequestId::new().to_string(),
        module_id: slm_core::ModuleId::from_str(&module_id)
            .unwrap_or_else(|_| slm_core::ModuleId::from_str("unknown").unwrap()),
        event_type: "graph-mutation".to_string(),
        source: "graph-proxy".to_string(),
        status: if sc_status.is_success() { "ok" } else { "upstream-error" }.to_string(),
        event_at: Utc::now(),
        captured_at: Utc::now(),
        payload: serde_json::json!({ "module_id": module_id }),
        caller_request_id: None,
    };
    let _ = state.doorman.ledger().append_capture_entry(&entry);

    // 5. Return service-content response.
    let status = StatusCode::from_u16(sc_status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    (status, Json(sc_body)).into_response()
}

/// Simple percent-encoding for URL query parameters (encodes spaces, special chars).
fn urlencoding_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            b' ' => out.push('+'),
            b => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

// =============================================================================
// POST /v1/messages — Anthropic Messages API shim (Sprint 0a)
//
// Enables Claude Code (and any Anthropic SDK client) to route through Doorman
// by pointing ANTHROPIC_BASE_URL=http://127.0.0.1:9080. Sprint 0 uses fake SSE
// streaming (buffer full response, emit all events at once) — real token
// streaming lands in Sprint 0b.
//
// Model → tier routing:
//   claude-haiku-*  → Complexity::Low   → Tier A (local, always-on)
//   claude-sonnet-* → Complexity::High  → Tier B "trainer" (Yo-Yo #1)
//   claude-opus-*   → Complexity::High  → Tier C passthrough
//   anything else   → Complexity::Medium → Tier B "trainer" fallback
//
// graph_context_enabled: Some(false) on all requests — DataGraph entity rows
// must not bloat Claude Code prompts (the shim is the routing boundary).
// =============================================================================

/// Inbound: Anthropic Messages API request body.
#[derive(Deserialize)]
struct AnthropicMessagesBody {
    model: String,
    #[serde(default)]
    system: Option<String>,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
    #[serde(default)]
    stream: bool,
    #[serde(default)]
    temperature: Option<f32>,
    #[serde(default)]
    metadata: Option<serde_json::Value>,
    #[serde(default)]
    stop_sequences: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct AnthropicMessage {
    role: String,
    content: AnthropicContent,
}

/// Anthropic content may be a plain string or an array of typed blocks.
#[derive(Deserialize)]
#[serde(untagged)]
enum AnthropicContent {
    Text(String),
    Blocks(Vec<AnthropicContentBlock>),
}

#[derive(Deserialize)]
struct AnthropicContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    thinking: Option<String>,
    // id / name / input / tool_use_id present in tool_use / tool_result blocks;
    // ignored in Sprint 0 (content is flattened to text).
    #[serde(default)]
    id: Option<serde_json::Value>,
    #[serde(default)]
    name: Option<serde_json::Value>,
    #[serde(default)]
    input: Option<serde_json::Value>,
    #[serde(default)]
    tool_use_id: Option<serde_json::Value>,
    #[serde(default)]
    content: Option<serde_json::Value>,
}

/// Flatten Anthropic content (text or blocks) to a plain string.
/// Tool-use and tool-result blocks are omitted; thinking blocks are wrapped
/// in `<thinking>` tags. This is the Sprint 0 simplification — Sprint 1
/// replaces ChatMessage with CanonicalMessage and preserves all block types.
fn flatten_anthropic_content(content: AnthropicContent) -> String {
    match content {
        AnthropicContent::Text(s) => s,
        AnthropicContent::Blocks(blocks) => blocks
            .into_iter()
            .filter_map(|b| match b.block_type.as_str() {
                "text" => b.text,
                "thinking" => b.thinking.map(|t| format!("<thinking>{}</thinking>", t)),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

/// Translate an Anthropic Messages request into a canonical `ComputeRequest`.
fn anthropic_to_compute_request(
    body: AnthropicMessagesBody,
    module_id: ModuleId,
    request_id: RequestId,
) -> ComputeRequest {
    let mut messages: Vec<ChatMessage> = Vec::new();

    if let Some(system) = body.system {
        if !system.is_empty() {
            messages.push(ChatMessage { role: "system".to_string(), content: system });
        }
    }

    for msg in body.messages {
        messages.push(ChatMessage {
            role: msg.role,
            content: flatten_anthropic_content(msg.content),
        });
    }

    let (complexity, yoyo_label) = if body.model.starts_with("claude-haiku") {
        (Complexity::Low, None)
    } else if body.model.starts_with("claude-sonnet") {
        (Complexity::High, Some("trainer".to_string()))
    } else if body.model.starts_with("claude-opus") {
        (Complexity::High, None)
    } else {
        (Complexity::Medium, Some("trainer".to_string()))
    };

    ComputeRequest {
        request_id,
        module_id,
        model: Some(body.model),
        messages,
        complexity,
        tier_hint: None,
        stream: false, // Doorman always returns buffered; SSE is assembled by the handler
        max_tokens: Some(body.max_tokens),
        temperature: body.temperature,
        sanitised_outbound: false,
        tier_c_label: None,
        yoyo_label,
        grammar: None,
        speculation: None,
        graph_context_enabled: Some(false),
    }
}

/// Build a non-streaming Anthropic Messages API response body.
fn compute_to_anthropic_response(resp: &slm_core::ComputeResponse, model: &str) -> serde_json::Value {
    let output_tokens = resp.content.split_whitespace().count() as u32;
    serde_json::json!({
        "id": format!("msg_{}", resp.request_id),
        "type": "message",
        "role": "assistant",
        "content": [{"type": "text", "text": resp.content}],
        "model": model,
        "stop_reason": "end_turn",
        "stop_sequence": null,
        "usage": {
            "input_tokens": 0,
            "output_tokens": output_tokens
        }
    })
}

/// Build a fake-SSE response: buffer the full content, emit all 6 events at once.
/// Claude Code's streaming UX receives the full response in a single burst rather
/// than token-by-token. Real per-token streaming lands in Sprint 0b.
fn anthropic_sse_body(resp: &slm_core::ComputeResponse, model: &str) -> String {
    let msg_id = format!("msg_{}", resp.request_id);
    let output_tokens = resp.content.split_whitespace().count() as u32;

    let e_start = serde_json::json!({
        "type": "message_start",
        "message": {
            "id": &msg_id, "type": "message", "role": "assistant",
            "content": [], "model": model,
            "stop_reason": null, "stop_sequence": null,
            "usage": {"input_tokens": 0, "output_tokens": 0}
        }
    });
    let e_cb_start = serde_json::json!({"type": "content_block_start", "index": 0, "content_block": {"type": "text", "text": ""}});
    let e_cb_delta = serde_json::json!({"type": "content_block_delta", "index": 0, "delta": {"type": "text_delta", "text": resp.content}});
    let e_cb_stop  = serde_json::json!({"type": "content_block_stop",  "index": 0});
    let e_msg_delta = serde_json::json!({"type": "message_delta", "delta": {"stop_reason": "end_turn", "stop_sequence": null}, "usage": {"output_tokens": output_tokens}});
    let e_msg_stop  = serde_json::json!({"type": "message_stop"});

    format!(
        "event: message_start\ndata: {}\n\n\
         event: content_block_start\ndata: {}\n\n\
         event: content_block_delta\ndata: {}\n\n\
         event: content_block_stop\ndata: {}\n\n\
         event: message_delta\ndata: {}\n\n\
         event: message_stop\ndata: {}\n\n",
        e_start, e_cb_start, e_cb_delta, e_cb_stop, e_msg_delta, e_msg_stop
    )
}

async fn anthropic_messages(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<AnthropicMessagesBody>,
) -> Result<Response, ApiError> {
    let module_id = match headers
        .get("x-foundry-module-id")
        .and_then(|v| v.to_str().ok())
    {
        Some(s) => ModuleId::from_str(s)
            .map_err(|e| ApiError::bad_request(format!("invalid X-Foundry-Module-ID: {e}")))?,
        None => ModuleId::from_str("foundry").expect("compile-time-valid default moduleId"),
    };
    let request_id = RequestId::new();
    let model = body.model.clone();
    let stream = body.stream;

    let req = anthropic_to_compute_request(body, module_id, request_id);
    let resp = state.doorman.route(&req).await.map_err(ApiError::from)?;

    if stream {
        let sse_body = anthropic_sse_body(&resp, &model);
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/event-stream; charset=utf-8")
            .header("cache-control", "no-cache")
            .header("x-accel-buffering", "no")
            .body(axum::body::Body::from(sse_body))
            .expect("build SSE response"))
    } else {
        let body = compute_to_anthropic_response(&resp, &model);
        Ok((StatusCode::OK, Json(body)).into_response())
    }
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
            // audit_proxy request body too large → 413 PAYLOAD_TOO_LARGE.
            DoormanError::AuditProxyPayloadTooLarge { .. } => StatusCode::PAYLOAD_TOO_LARGE,
            // Per-tenant concurrency cap hit → 503 SERVICE_UNAVAILABLE.
            // The caller may retry after in-flight requests from the same
            // tenant complete; Retry-After: 5 header is set by the handler.
            DoormanError::AuditTenantConcurrencyExhausted { .. } => StatusCode::SERVICE_UNAVAILABLE,
            DoormanError::BriefCacheMiss => StatusCode::GONE,
            // No shadow corpus tuple exists for the brief_id in the
            // verdict POST. Per §7B, no tuple is created; the caller
            // should ensure the shadow brief was dispatched before
            // signing a verdict. HTTP 410 GONE — the resource was never
            // captured (same as BriefCacheMiss; the distinction is
            // caller-visible via the error message).
            DoormanError::OrphanVerdictNoCorpusTuple { .. } => StatusCode::GONE,
            DoormanError::LedgerIo(_)
            | DoormanError::LedgerSerde(_)
            | DoormanError::HomeUnset
            | DoormanError::LedgerLock(_)
            | DoormanError::CorpusWrite { .. }
            | DoormanError::QueueIo { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            // Queue lock unavailable — transient; caller may retry.
            DoormanError::QueueLockFailed { .. } => StatusCode::SERVICE_UNAVAILABLE,
            // Malformed brief detected and moved to poison bucket.
            DoormanError::QueueMalformedBrief { .. } => StatusCode::BAD_REQUEST,
            // Graph proxy — caller omitted the mandatory X-Foundry-Module-ID
            // header. Error is on the caller's side.
            DoormanError::GraphProxyMissingModuleId => StatusCode::BAD_REQUEST,
            // Graph proxy — service-content is unreachable or unconfigured.
            // Server-side transient condition; caller may retry.
            DoormanError::GraphProxyServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            // Tier B resilience — circuit open or outer deadline fired.
            // In the normal path these are caught by the router and trigger
            // Tier A fallback; they surface here only when Tier A is also
            // absent. 503 SERVICE_UNAVAILABLE; caller may retry.
            DoormanError::TierBTimeout | DoormanError::TierBCircuitOpen => {
                StatusCode::SERVICE_UNAVAILABLE
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
