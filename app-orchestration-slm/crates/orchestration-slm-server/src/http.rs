// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Axum route handlers for the orchestration-slm chassis.
//!
//! Port :9180 — does not conflict with service-slm :9080.
//!
//! MVP endpoints:
//!   GET  /healthz                  — liveness
//!   GET  /readyz                   — readiness with Yo-Yo probe state
//!   GET  /v1/fleet                 — registered Totebox listing
//!   POST /v1/discovery/register    — Totebox Doorman self-registration
//!   POST /v1/yoyo/proxy            — relay → Yo-Yo "default" node
//!   POST /v1/yoyo/trainer          — relay → Yo-Yo "trainer" node
//!   POST /v1/yoyo/graph            — relay → Yo-Yo "graph" node

use std::sync::Arc;

use axum::{
    extract::{Json as ExtractJson, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use orchestration_slm::{ChassisError, FleetRegistry, MeteringLedger, YoyoProxyClient};
use orchestration_slm_core::{
    ReadyzResponse, RegistrationRequest, RegistrationResponse, CHASSIS_VERSION,
};
use serde_json::{json, Value};
use tracing::warn;

pub struct AppState {
    pub fleet: Arc<FleetRegistry>,
    pub proxy: Arc<YoyoProxyClient>,
    pub metering: Arc<MeteringLedger>,
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

    Json(ReadyzResponse {
        status: "ok",
        yoyo_trainer_reachable,
        yoyo_graph_reachable,
        fleet_members,
        chassis_version: CHASSIS_VERSION,
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

// ── Yo-Yo proxy helpers ───────────────────────────────────────────────────────

/// Extract `Authorization: Bearer <module-id>` from headers.
/// Returns None if absent or malformed.
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
async fn dispatch_yoyo(
    state: Arc<AppState>,
    headers: HeaderMap,
    body: Value,
    label: &str,
) -> Response {
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

    match state.proxy.proxy(label, &member.module_id, body).await {
        Ok((bytes, inference_ms)) => {
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
        Err(ChassisError::YoyoNotConfigured(lbl)) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": format!("Yo-Yo label '{}' not configured on this chassis", lbl)})),
        )
            .into_response(),
        Err(ChassisError::YoyoUpstream(msg)) => {
            warn!(label, error = %msg, "upstream error");
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": "Yo-Yo upstream error", "detail": msg})),
            )
                .into_response()
        }
        Err(e) => {
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
    use orchestration_slm::yoyo_proxy::YoyoEndpoints;
    use orchestration_slm_core::RegistrationRequest;
    use tower::ServiceExt;

    fn test_state() -> Arc<AppState> {
        let endpoints = YoyoEndpoints {
            default_endpoint: None,
            trainer_endpoint: None,
            graph_endpoint: None,
            yoyo_bearer: None,
            hourly_usd_rate: 0.0,
        };
        Arc::new(AppState {
            fleet: FleetRegistry::new(),
            proxy: Arc::new(YoyoProxyClient::new(endpoints)),
            metering: MeteringLedger::new(),
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

    #[tokio::test]
    async fn proxy_unauthenticated_returns_401() {
        let app = router(test_state());
        let body = serde_json::to_vec(&json!({"model": "olmo", "messages": []})).unwrap();
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/yoyo/proxy")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
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
        let body = serde_json::to_vec(&json!({"model": "olmo", "messages": []})).unwrap();
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/yoyo/proxy")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer op::a::slm")
                    .header("x-foundry-module-id", "op::b::slm")
                    .body(Body::from(body))
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
        let body = serde_json::to_vec(&json!({"model": "olmo", "messages": []})).unwrap();
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/yoyo/proxy")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer op::a::slm")
                    .body(Body::from(body))
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
        let body = serde_json::to_vec(&json!({"model": "olmo", "messages": []})).unwrap();
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/yoyo/proxy")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer op::a::slm")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
