// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Axum route handlers for the orchestration-slm chassis.
//!
//! Port :9180 — does not conflict with service-slm :9080.
//!
//! MVP endpoints:
//!   GET  /healthz                  — liveness
//!   GET  /readyz                   — readiness with Yo-Yo probe, circuit, gate, license state
//!   GET  /v1/fleet                 — registered Totebox listing
//!   POST /v1/discovery/register    — Totebox Doorman self-registration
//!   POST /v1/yoyo/proxy            — relay → Yo-Yo "default" node
//!   POST /v1/yoyo/trainer          — relay → Yo-Yo "trainer" node
//!   POST /v1/yoyo/graph            — relay → Yo-Yo "graph" node
//!   POST /v1/gate/:label           — operator kill switch (open / close a label gate)

use std::sync::Arc;

use axum::{
    extract::{Json as ExtractJson, Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use orchestration_slm::{
    ChassisError, ChassisFlowGate, CircuitRegistry, FleetRegistry, LicenseStatus, MembershipKey,
    MeteringLedger, YoyoProxyClient,
};
use orchestration_slm_core::{
    AdapterEntry, AdaptersResponse, AuditRollupResponse, FederatedGraphEntry,
    FederatedGraphRequest, FederatedGraphResponse, ReadyzResponse, RegistrationRequest,
    RegistrationResponseV2, TenantRollupEntry, TrainingScheduleRequest, TrainingScheduleResponse,
    CHASSIS_VERSION,
};
use serde_json::{json, Value};
use tracing::warn;

pub struct AppState {
    pub fleet: Arc<FleetRegistry>,
    pub proxy: Arc<YoyoProxyClient>,
    pub metering: Arc<MeteringLedger>,
    pub circuits: Arc<CircuitRegistry>,
    pub gates: Arc<ChassisFlowGate>,
    pub license: Arc<LicenseStatus>,
    pub membership: Arc<MembershipKey>,
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/v1/fleet", get(fleet_list))
        .route("/v1/discovery/register", post(discovery_register))
        .route("/v1/yoyo/proxy", post(yoyo_proxy))
        .route("/v1/yoyo/trainer", post(yoyo_trainer))
        .route("/v1/yoyo/graph", post(yoyo_graph))
        .route("/v1/gate/{label}", post(gate_set))
        .route("/v1/audit/rollup", get(audit_rollup))
        .route("/v1/graph/federated", post(graph_federated))
        .route("/v1/training/schedule", post(training_schedule))
        .route("/v1/adapters", get(adapters_list))
        .with_state(state)
}

// ── Liveness ──────────────────────────────────────────────────────────────────

async fn healthz() -> (StatusCode, &'static str) {
    (StatusCode::OK, "ok")
}

// ── Readiness ─────────────────────────────────────────────────────────────────

async fn readyz(State(state): State<Arc<AppState>>) -> Json<ReadyzResponse> {
    let fleet_members = state.fleet.member_count().await;
    let (trainer_cfg, graph_cfg) = state.proxy.endpoints.any_configured();

    // Only probe if configured — keeps readyz fast when Yo-Yo is absent.
    let yoyo_trainer_reachable = if trainer_cfg {
        state.proxy.probe("trainer").await
    } else {
        false
    };
    let yoyo_graph_reachable = if graph_cfg {
        state.proxy.probe("graph").await
    } else {
        false
    };

    let circuit_states = state
        .circuits
        .snapshot()
        .into_iter()
        .map(|(k, v)| (k, v.as_str().to_string()))
        .collect();

    let gate_states = state.gates.snapshot();

    Json(ReadyzResponse {
        status: "ok",
        yoyo_trainer_reachable,
        yoyo_graph_reachable,
        fleet_members,
        chassis_version: CHASSIS_VERSION,
        license_status: state.license.label().to_string(),
        circuit_states,
        gate_states,
    })
}

// ── Fleet ─────────────────────────────────────────────────────────────────────

async fn fleet_list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(state.fleet.list().await)
}

// ── Discovery ─────────────────────────────────────────────────────────────────

async fn discovery_register(
    State(state): State<Arc<AppState>>,
    ExtractJson(req): ExtractJson<RegistrationRequest>,
) -> Json<RegistrationResponseV2> {
    let member = state.fleet.register(req).await;
    let token = state.membership.issue(&member.module_id, &member.archive_id);
    Json(RegistrationResponseV2 {
        status: "registered",
        module_id: member.module_id,
        chassis_version: CHASSIS_VERSION,
        membership_token: Some(token),
    })
}

// ── Gate management ───────────────────────────────────────────────────────────

/// POST /v1/gate/:label — operator kill switch.
///
/// Body: `{"closed": true}` to block a label, `{"closed": false}` to open it.
/// The "global" label closes all labels simultaneously.
/// Returns 404 for unknown labels.
async fn gate_set(
    State(state): State<Arc<AppState>>,
    Path(label): Path<String>,
    ExtractJson(body): ExtractJson<Value>,
) -> Response {
    let closed = body
        .get("closed")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if state.gates.set_existing(&label, closed) {
        Json(json!({"label": label, "closed": closed})).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": format!("unknown gate label '{label}'")})),
        )
            .into_response()
    }
}

// ── Audit rollup ──────────────────────────────────────────────────────────────

/// GET /v1/audit/rollup — per-tenant metering summary (in-process, rebuilt on restart).
async fn audit_rollup(State(state): State<Arc<AppState>>) -> Json<AuditRollupResponse> {
    let all = state.metering.all().await;
    let mut entries: Vec<TenantRollupEntry> = all
        .into_iter()
        .map(|(module_id, stats)| TenantRollupEntry {
            module_id,
            total_requests: stats.total_requests,
            total_inference_ms: stats.total_inference_ms,
            total_cost_usd: stats.total_cost_usd,
        })
        .collect();
    entries.sort_by(|a, b| b.total_cost_usd.partial_cmp(&a.total_cost_usd).unwrap_or(std::cmp::Ordering::Equal));
    let total_tenants = entries.len();
    Json(AuditRollupResponse {
        entries,
        total_tenants,
    })
}

// ── Yo-Yo proxy helpers ───────────────────────────────────────────────────────

/// Extract `Authorization: Bearer <module-id>` from headers.
fn extract_bearer_module_id(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.trim().to_string())
}

fn extract_module_id_header(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-foundry-module-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

/// Shared proxy dispatch — routes to the named Yo-Yo label.
///
/// Gate order (fail-fast, no Yo-Yo call unless all pass):
///   1. Flow gate — operator kill switch for this label.
///   2. License — chassis-level Tier B gate (valid license required).
///   3. Auth — bearer token + fleet membership + subscription.
///   4. Circuit breaker — shared node health state across all archives.
async fn dispatch_yoyo(
    state: Arc<AppState>,
    headers: HeaderMap,
    body: Value,
    label: &str,
) -> Response {
    // 1. Flow gate — operator kill switch.
    if let Some(blocking) = state.gates.blocking_label(label) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": format!("flow gate '{}' is closed", blocking)})),
        )
            .into_response();
    }

    // 2. License — chassis-level Tier B gate.
    if !state.license.permits_tier_b() {
        return (
            StatusCode::PAYMENT_REQUIRED,
            Json(json!({"error": "Tier B brokering requires a valid chassis license"})),
        )
            .into_response();
    }

    // 3. Auth — bearer + fleet membership + subscription.
    let bearer_id = match extract_bearer_module_id(&headers) {
        Some(id) => id,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "missing Authorization: Bearer <module-id>"})),
            )
                .into_response();
        }
    };
    let header_id = extract_module_id_header(&headers);

    let member = match state
        .fleet
        .authenticate_proxy(&bearer_id, header_id.as_deref())
        .await
    {
        Ok(m) => m,
        Err(ChassisError::Unauthenticated) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "module-id not registered in fleet"})),
            )
                .into_response();
        }
        Err(ChassisError::ModuleIdMismatch) => {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({"error": "X-Foundry-Module-ID does not match bearer token"})),
            )
                .into_response();
        }
        Err(ChassisError::NotSubscribed(id)) => {
            return (
                StatusCode::PAYMENT_REQUIRED,
                Json(json!({"error": format!("{} is not subscribed for Tier B", id)})),
            )
                .into_response();
        }
        Err(e) => {
            warn!(label, error = %e, "auth error");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "internal error"})),
            )
                .into_response();
        }
    };

    // 4. Circuit breaker — shared node health across all archives.
    {
        let allowed = state
            .circuits
            .get(label)
            .map(|c| c.allow_request())
            .unwrap_or(true);
        if !allowed {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error": format!("circuit breaker open for '{label}'")})),
            )
                .into_response();
        }
    }

    // 5. Proxy to the Yo-Yo node.
    match state.proxy.proxy(label, &member.module_id, body).await {
        Ok((bytes, inference_ms)) => {
            state.circuits.get(label).map(|c| c.record_success());
            state
                .metering
                .record(
                    &member.module_id,
                    inference_ms,
                    state.proxy.endpoints.hourly_usd_rate,
                )
                .await;
            let mut resp_headers = HeaderMap::new();
            resp_headers.insert(
                axum::http::header::CONTENT_TYPE,
                "application/json".parse().unwrap(),
            );
            resp_headers.insert(
                "x-foundry-tier-used"
                    .parse::<axum::http::HeaderName>()
                    .unwrap(),
                "yoyo".parse().unwrap(),
            );
            (StatusCode::OK, resp_headers, bytes).into_response()
        }
        Err(ChassisError::YoyoNotConfigured(lbl)) => {
            state.circuits.get(label).map(|c| c.record_failure());
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(
                    json!({"error": format!("Yo-Yo label '{}' not configured on this chassis", lbl)}),
                ),
            )
                .into_response()
        }
        Err(ChassisError::YoyoUpstream(msg)) => {
            state.circuits.get(label).map(|c| c.record_failure());
            warn!(label, error = %msg, "upstream error");
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": "Yo-Yo upstream error", "detail": msg})),
            )
                .into_response()
        }
        Err(e) => {
            state.circuits.get(label).map(|c| c.record_failure());
            warn!(label, error = %e, "proxy error");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "proxy internal error"})),
            )
                .into_response()
        }
    }
}

async fn yoyo_proxy(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    ExtractJson(body): ExtractJson<Value>,
) -> Response {
    dispatch_yoyo(state, headers, body, "proxy").await
}

async fn yoyo_trainer(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    ExtractJson(body): ExtractJson<Value>,
) -> Response {
    dispatch_yoyo(state, headers, body, "trainer").await
}

async fn yoyo_graph(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    ExtractJson(body): ExtractJson<Value>,
) -> Response {
    dispatch_yoyo(state, headers, body, "graph").await
}

// ── Phase 2: Federated graph query ────────────────────────────────────────────

/// POST /v1/graph/federated — fan out a DataGraph query to all registered archives.
///
/// Calls `{doorman_endpoint}/v1/query` on each fleet member's Doorman using
/// a fire-and-forget reqwest client. Results from unreachable archives are
/// silently omitted; the caller sees `archives_queried` vs `archives_reachable`.
async fn graph_federated(
    State(state): State<Arc<AppState>>,
    ExtractJson(req): ExtractJson<FederatedGraphRequest>,
) -> impl IntoResponse {
    let members = state.fleet.list_full().await;
    let archives_queried = members.len();
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_default();

    let mut entries: Vec<FederatedGraphEntry> = Vec::new();
    for member in &members {
        let url = format!("{}/v1/query", member.doorman_endpoint.trim_end_matches('/'));
        let body = serde_json::json!({ "q": req.q, "limit": req.limit });
        if let Ok(resp) = client.post(&url).json(&body).send().await {
            if let Ok(result) = resp.json::<serde_json::Value>().await {
                entries.push(FederatedGraphEntry {
                    module_id: member.module_id.clone(),
                    archive_id: member.archive_id.clone(),
                    result,
                });
            }
        }
    }


    let archives_reachable = entries.len();
    Json(FederatedGraphResponse {
        entries,
        archives_queried,
        archives_reachable,
    })
}

// ── Phase 2: Training schedule ────────────────────────────────────────────────

/// POST /v1/training/schedule — relay a LoRA training job to the Yo-Yo trainer.
///
/// Proxies to `ORCHESTRATION_YOYO_TRAINER_ENDPOINT/v1/training/jobs`.
/// Returns the job-id from the trainer's response, or 503 if not configured.
async fn training_schedule(
    State(state): State<Arc<AppState>>,
    ExtractJson(req): ExtractJson<TrainingScheduleRequest>,
) -> Response {
    match state.proxy.endpoints.trainer_endpoint.as_deref() {
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "trainer Yo-Yo node not configured on this chassis"})),
        )
            .into_response(),
        Some(endpoint) => {
            let url = format!("{}/v1/training/jobs", endpoint.trim_end_matches('/'));
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default();
            match client.post(&url).json(&req).send().await {
                Ok(resp) => {
                    let status = resp.status();
                    match resp.json::<serde_json::Value>().await {
                        Ok(body) => {
                            let job_id = body
                                .get("job_id")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
                                .to_string();
                            Json(TrainingScheduleResponse {
                                job_id,
                                status: if status.is_success() { "queued" } else { "error" },
                                trainer_endpoint: Some(endpoint.to_string()),
                            })
                            .into_response()
                        }
                        Err(_) => (
                            StatusCode::BAD_GATEWAY,
                            Json(json!({"error": "trainer returned non-JSON response"})),
                        )
                            .into_response(),
                    }
                }
                Err(e) => {
                    warn!(error = %e, "training schedule: trainer unreachable");
                    (
                        StatusCode::SERVICE_UNAVAILABLE,
                        Json(json!({"error": "trainer Yo-Yo node unreachable", "detail": e.to_string()})),
                    )
                        .into_response()
                }
            }
        }
    }
}

// ── Phase 2: Adapter listing ──────────────────────────────────────────────────

/// GET /v1/adapters — list available LoRA adapters from the Yo-Yo fleet.
///
/// Queries `{trainer_endpoint}/v1/adapters` and `{graph_endpoint}/v1/adapters`.
/// Results from both nodes are merged and labelled with `node_label`.
async fn adapters_list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_default();

    let mut adapters: Vec<AdapterEntry> = Vec::new();

    let nodes: &[(&str, &str)] = &[
        (
            state.proxy.endpoints.trainer_endpoint.as_deref().unwrap_or(""),
            "trainer",
        ),
        (
            state.proxy.endpoints.graph_endpoint.as_deref().unwrap_or(""),
            "graph",
        ),
    ];

    for (endpoint, label) in nodes {
        if endpoint.is_empty() {
            continue;
        }
        let url = format!("{}/v1/adapters", endpoint.trim_end_matches('/'));
        if let Ok(resp) = client.get(&url).send().await {
            if let Ok(body) = resp.json::<serde_json::Value>().await {
                if let Some(arr) = body.get("adapters").and_then(|v| v.as_array()) {
                    for entry in arr {
                        adapters.push(AdapterEntry {
                            name: entry
                                .get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
                                .to_string(),
                            base_model: entry
                                .get("base_model")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
                                .to_string(),
                            node_label: label.to_string(),
                        });
                    }
                }
            }
        }
    }

    let total = adapters.len();
    Json(AdaptersResponse { adapters, total })
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use orchestration_slm::license::{LicensePayload, REQUIRED_PRODUCT};
    use orchestration_slm::yoyo_proxy::YoyoEndpoints;
    use orchestration_slm::DEFAULT_FAILURE_THRESHOLD;
    use orchestration_slm_core::RegistrationRequest;
    use tower::ServiceExt;

    fn make_endpoints() -> YoyoEndpoints {
        YoyoEndpoints {
            default_endpoint: None,
            trainer_endpoint: None,
            graph_endpoint: None,
            yoyo_bearer: None,
            hourly_usd_rate: 0.0,
        }
    }

    fn make_license_valid() -> LicenseStatus {
        LicenseStatus::Valid(LicensePayload {
            product: REQUIRED_PRODUCT.to_string(),
            issued_to: "Test Corp".into(),
            expiry: chrono::Utc::now() + chrono::TimeDelta::days(365),
            entitlements: vec!["tier-b-orchestration".into()],
        })
    }

    fn make_membership() -> Arc<orchestration_slm::MembershipKey> {
        Arc::new(orchestration_slm::MembershipKey::generate().unwrap())
    }

    fn test_state() -> Arc<AppState> {
        Arc::new(AppState {
            fleet: FleetRegistry::new(),
            proxy: Arc::new(YoyoProxyClient::new(make_endpoints())),
            metering: MeteringLedger::new(),
            circuits: Arc::new(CircuitRegistry::new(["proxy", "trainer", "graph"])),
            gates: Arc::new(ChassisFlowGate::new(["proxy", "trainer", "graph"])),
            license: Arc::new(make_license_valid()),
            membership: make_membership(),
        })
    }

    fn test_state_unlicensed() -> Arc<AppState> {
        Arc::new(AppState {
            fleet: FleetRegistry::new(),
            proxy: Arc::new(YoyoProxyClient::new(make_endpoints())),
            metering: MeteringLedger::new(),
            circuits: Arc::new(CircuitRegistry::new(["proxy", "trainer", "graph"])),
            gates: Arc::new(ChassisFlowGate::new(["proxy", "trainer", "graph"])),
            license: Arc::new(LicenseStatus::Absent),
            membership: make_membership(),
        })
    }

    async fn register_member(state: &Arc<AppState>, module_id: &str, subscribed: bool) {
        state
            .fleet
            .register(RegistrationRequest {
                module_id: module_id.to_string(),
                archive_id: "test".to_string(),
                doorman_endpoint: "http://127.0.0.1:9080".to_string(),
                tier_b_subscribed: subscribed,
            })
            .await;
    }

    fn proxy_body() -> Vec<u8> {
        serde_json::to_vec(&json!({"model": "olmo", "messages": []})).unwrap()
    }

    // ── Liveness / readyz ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn healthz_returns_ok() {
        let app = router(test_state());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/healthz")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn fleet_list_empty() {
        let app = router(test_state());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/fleet")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // ── Registration ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn discovery_register_and_list() {
        let state = test_state();
        let app = router(Arc::clone(&state));

        let body = serde_json::to_vec(&RegistrationRequest {
            module_id: "op::a::slm".to_string(),
            archive_id: "project-a".to_string(),
            doorman_endpoint: "http://10.0.0.1:9080".to_string(),
            tier_b_subscribed: true,
        })
        .unwrap();

        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/discovery/register")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(state.fleet.member_count().await, 1);
    }

    // ── Auth / subscription ───────────────────────────────────────────────────

    #[tokio::test]
    async fn proxy_unauthenticated_returns_401() {
        let app = router(test_state());
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/yoyo/proxy")
                    .header("content-type", "application/json")
                    .body(Body::from(proxy_body()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn proxy_spoofing_returns_403() {
        let state = test_state();
        register_member(&state, "op::a::slm", true).await;
        let app = router(Arc::clone(&state));
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/yoyo/proxy")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer op::a::slm")
                    .header("x-foundry-module-id", "op::b::slm")
                    .body(Body::from(proxy_body()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn proxy_not_subscribed_returns_402() {
        let state = test_state();
        register_member(&state, "op::a::slm", false).await;
        let app = router(Arc::clone(&state));
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/yoyo/proxy")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer op::a::slm")
                    .body(Body::from(proxy_body()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::PAYMENT_REQUIRED);
    }

    #[tokio::test]
    async fn proxy_not_configured_returns_503() {
        let state = test_state();
        register_member(&state, "op::a::slm", true).await;
        let app = router(Arc::clone(&state));
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/yoyo/proxy")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer op::a::slm")
                    .body(Body::from(proxy_body()))
                    .unwrap(),
            )
            .await
            .unwrap();
        // YoyoNotConfigured — circuit records failure, but 503 is correct.
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    // ── License gate ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn proxy_no_license_returns_402() {
        let state = test_state_unlicensed();
        register_member(&state, "op::a::slm", true).await;
        let app = router(Arc::clone(&state));
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/yoyo/proxy")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer op::a::slm")
                    .body(Body::from(proxy_body()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::PAYMENT_REQUIRED);
    }

    // ── Flow gate ─────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn proxy_gate_closed_returns_503() {
        let state = test_state();
        register_member(&state, "op::a::slm", true).await;
        state.gates.set_existing("proxy", true);
        let app = router(Arc::clone(&state));
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/yoyo/proxy")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer op::a::slm")
                    .body(Body::from(proxy_body()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn gate_endpoint_closes_and_opens() {
        let state = test_state();
        let app = router(Arc::clone(&state));

        // Close the trainer gate.
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/gate/trainer")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({"closed": true})).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(!state.gates.is_open("trainer"));

        // Re-open.
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/gate/trainer")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({"closed": false})).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(state.gates.is_open("trainer"));
    }

    #[tokio::test]
    async fn gate_endpoint_unknown_label_returns_404() {
        let app = router(test_state());
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/gate/no-such-node")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({"closed": true})).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    // ── Audit rollup ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn audit_rollup_empty_returns_ok() {
        let app = router(test_state());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/audit/rollup")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn audit_rollup_records_accumulate() {
        let state = test_state();
        state
            .metering
            .record("op::a::slm", Some(1000), 2.0)
            .await;
        state
            .metering
            .record("op::b::slm", Some(500), 2.0)
            .await;

        let app = router(Arc::clone(&state));
        use axum::body::to_bytes;
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/audit/rollup")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let rollup: AuditRollupResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(rollup.total_tenants, 2);
        // Sorted by cost descending: op::a has 2× the inference_ms so higher cost.
        assert_eq!(rollup.entries[0].module_id, "op::a::slm");
        assert_eq!(rollup.entries[0].total_requests, 1);
    }

    // ── Circuit breaker ───────────────────────────────────────────────────────

    #[tokio::test]
    async fn circuit_open_returns_503() {
        let state = test_state();
        register_member(&state, "op::a::slm", true).await;

        // Trip the proxy circuit open.
        {
            let c = state.circuits.get("proxy").unwrap();
            for _ in 0..DEFAULT_FAILURE_THRESHOLD {
                c.record_failure();
            }
        }

        let app = router(Arc::clone(&state));
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/yoyo/proxy")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer op::a::slm")
                    .body(Body::from(proxy_body()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn circuit_isolates_labels() {
        // Tripping "proxy" must not affect "trainer".
        let state = test_state();
        {
            let c = state.circuits.get("proxy").unwrap();
            for _ in 0..DEFAULT_FAILURE_THRESHOLD {
                c.record_failure();
            }
        }
        use orchestration_slm::CircuitState;
        assert_eq!(
            state.circuits.get("proxy").unwrap().state(),
            CircuitState::Open
        );
        assert_eq!(
            state.circuits.get("trainer").unwrap().state(),
            CircuitState::Closed
        );
    }
}
