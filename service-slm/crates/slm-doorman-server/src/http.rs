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

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use axum::body::Bytes;
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
