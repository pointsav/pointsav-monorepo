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
    ChassisError, ChassisFlowGate, CircuitRegistry, FleetRegistry, LicenseStatus, MeteringLedger,
    YoyoProxyClient,
};
use orchestration_slm_core::{
    ReadyzResponse, RegistrationRequest, RegistrationResponse, CHASSIS_VERSION,
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
) -> Json<RegistrationResponse> {
    let member = state.fleet.register(req).await;
    Json(RegistrationResponse {
        status: "registered",
        module_id: member.module_id,
        chassis_version: CHASSIS_VERSION,
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

    fn test_state() -> Arc<AppState> {
        Arc::new(AppState {
            fleet: FleetRegistry::new(),
            proxy: Arc::new(YoyoProxyClient::new(make_endpoints())),
            metering: MeteringLedger::new(),
            circuits: Arc::new(CircuitRegistry::new(["proxy", "trainer", "graph"])),
            gates: Arc::new(ChassisFlowGate::new(["proxy", "trainer", "graph"])),
            license: Arc::new(make_license_valid()),
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
