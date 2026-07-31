// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tier X — isolated external-LLM research/code-assist bridge.
//!
//! Per `BRIEF-os-totebox-platform.md` §7/§14 #12 (approved 2026-07-14): a
//! separate process from the core loop (never embedded in service-content or
//! service-slm), used only for advisory research/code-writing assistance for
//! this archive's own development — never for entity extraction, DataGraph
//! writes, or training-signal generation. Compatible with SYS-ADR-07 by
//! construction: this crate has no DataGraph client, no graph-write
//! dependency, and no code path back into any structured-data store.
//!
//! Design, mirrored from two existing precedents rather than invented fresh:
//! - Sidecar credential isolation (`gh-aw-firewall` pattern): this process
//!   alone holds the real upstream API key (`BridgeConfig::provider_api_key`).
//!   Callers authenticate with a *separate*, local-only `access_token` and
//!   never see the real key — it is injected onto the upstream request here,
//!   never echoed back in any response.
//! - Per-label allowlist (mirrors `service-slm`'s existing Tier C design in
//!   `slm-doorman/src/tier/external.rs`): a request must carry a `label` on
//!   the configured allowlist, checked *before* any network attempt. There
//!   is no default-allow path.
//!
//! Additionally, every request carries an explicit `tag`
//! (`RequestTag::LocalOnly` / `RequestTag::CloudAllowed`). Only
//! `CloudAllowed`-tagged requests may ever reach the upstream provider — this
//! turns the local-only/cloud-allowed boundary into a structural check, not a
//! caller convention, mirroring LiteLLM's tag-based routing pattern.

use std::collections::HashSet;
use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// A request must be explicitly tagged. Only `CloudAllowed` may ever reach
/// the upstream provider. There is no default — an absent or malformed tag
/// is a request-shape error, not an implicit `LocalOnly`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RequestTag {
    LocalOnly,
    CloudAllowed,
}

/// Bridge configuration. `provider_api_key` is the one secret this process
/// exists to isolate — it must never appear in any response body, log line,
/// or error message returned to a caller.
#[derive(Clone, Default)]
pub struct BridgeConfig {
    /// Local-only bearer token callers present to reach this bridge at all.
    /// Distinct from, and unrelated to, `provider_api_key` — proves the
    /// caller is a locally-authorized Tier X client, not the open internet.
    pub access_token: String,
    /// The single allowlisted upstream base URL (no trailing slash).
    pub provider_endpoint: String,
    /// The real upstream credential. Never returned to any caller.
    pub provider_api_key: String,
    /// Compile-time-style allowlist of permitted task labels. Empty by
    /// default (`BridgeConfig::default()` — see below) — no label is
    /// permitted until explicitly configured, mirroring
    /// `ExternalAllowlist::EMPTY` in `service-slm`'s Tier C.
    pub allowed_labels: HashSet<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<BridgeConfig>,
    pub http: reqwest::Client,
}

impl AppState {
    pub fn new(config: BridgeConfig) -> Self {
        Self {
            config: Arc::new(config),
            http: reqwest::Client::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BridgeRequest {
    pub tag: RequestTag,
    pub label: String,
    /// Opaque request body forwarded verbatim to the upstream provider.
    /// This bridge does not interpret or validate its shape beyond the
    /// `tag`/`label` envelope above — the caller is responsible for
    /// producing a request the configured provider understands.
    pub payload: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct BridgeError {
    pub error: &'static str,
    pub detail: String,
}

impl IntoResponse for BridgeError {
    fn into_response(self) -> Response {
        let status = match self.error {
            "unauthorized" => StatusCode::UNAUTHORIZED,
            "local_only_tag_refused" | "label_not_allowlisted" => StatusCode::FORBIDDEN,
            "upstream_not_configured" => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::BAD_GATEWAY,
        };
        (status, Json(self)).into_response()
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/bridge/complete", post(complete))
        .with_state(state)
}

async fn healthz() -> &'static str {
    "ok"
}

fn check_access_token(state: &AppState, headers: &HeaderMap) -> Result<(), BridgeError> {
    let presented = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    match presented {
        Some(token)
            if !state.config.access_token.is_empty() && token == state.config.access_token =>
        {
            Ok(())
        }
        _ => Err(BridgeError {
            error: "unauthorized",
            detail: "missing or invalid local bridge access token".to_string(),
        }),
    }
}

async fn complete(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<BridgeRequest>,
) -> Result<Json<serde_json::Value>, BridgeError> {
    // Step 1 — local bridge access token. Refuses before any policy check,
    // so an unauthenticated caller learns nothing about label/tag policy.
    check_access_token(&state, &headers)?;

    // Step 2 — the tag gate. LocalOnly requests are refused unconditionally;
    // this is the structural half of "never used for extraction/DataGraph
    // writes/training-signal generation" — those call sites must never tag
    // a request CloudAllowed in the first place, and even if one did by
    // mistake, this bridge has no DataGraph write capability to exploit.
    if req.tag != RequestTag::CloudAllowed {
        warn!(
            target: "tool_llm_bridge",
            label = %req.label,
            "refused: request not tagged cloud-allowed"
        );
        return Err(BridgeError {
            error: "local_only_tag_refused",
            detail: "only cloud-allowed-tagged requests may reach the upstream provider"
                .to_string(),
        });
    }

    // Step 3 — label allowlist, checked before any network attempt.
    if !state.config.allowed_labels.contains(&req.label) {
        warn!(
            target: "tool_llm_bridge",
            label = %req.label,
            "refused: label not on allowlist"
        );
        return Err(BridgeError {
            error: "label_not_allowlisted",
            detail: format!("label {:?} is not on the configured allowlist", req.label),
        });
    }

    if state.config.provider_endpoint.is_empty() || state.config.provider_api_key.is_empty() {
        return Err(BridgeError {
            error: "upstream_not_configured",
            detail: "no upstream provider endpoint/key configured".to_string(),
        });
    }

    // Step 4 — forward to upstream, injecting the real key. The real key
    // never appears in anything returned to the caller from this point on.
    info!(
        target: "tool_llm_bridge",
        label = %req.label,
        "dispatching to upstream"
    );
    let resp = state
        .http
        .post(&state.config.provider_endpoint)
        .bearer_auth(&state.config.provider_api_key)
        .header("X-Foundry-Bridge-Label", req.label.as_str())
        .json(&req.payload)
        .send()
        .await
        .map_err(|e| BridgeError {
            error: "upstream_request_failed",
            detail: e.to_string(),
        })?;

    let body: serde_json::Value = resp.json().await.map_err(|e| BridgeError {
        error: "upstream_response_shape",
        detail: e.to_string(),
    })?;

    Ok(Json(body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use serde_json::json;
    use tower::ServiceExt;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn config(server_uri: String, labels: &[&str]) -> BridgeConfig {
        BridgeConfig {
            access_token: "local-test-token".to_string(),
            provider_endpoint: format!("{server_uri}/v1/chat/completions"),
            provider_api_key: "sk-REAL-SECRET-DO-NOT-LEAK".to_string(),
            allowed_labels: labels.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn app(cfg: BridgeConfig) -> Router {
        router(AppState::new(cfg))
    }

    fn req_body(tag: &str, label: &str) -> String {
        json!({
            "tag": tag,
            "label": label,
            "payload": {"prompt": "research this architecture question"}
        })
        .to_string()
    }

    /// No / wrong bearer token — refused before any policy check, zero
    /// upstream requests.
    #[tokio::test]
    async fn missing_access_token_refused_before_any_upstream_call() {
        let server = MockServer::start().await;
        let cfg = config(server.uri(), &["research"]);
        let app = app(cfg);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/bridge/complete")
                    .header("content-type", "application/json")
                    .body(Body::from(req_body("cloud-allowed", "research")))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            server.received_requests().await.unwrap_or_default().len(),
            0
        );
    }

    /// LocalOnly-tagged request is refused even with a valid token and an
    /// allowlisted label — the tag gate is structural, not advisory.
    #[tokio::test]
    async fn local_only_tag_never_reaches_upstream() {
        let server = MockServer::start().await;
        let cfg = config(server.uri(), &["research"]);
        let app = app(cfg);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/bridge/complete")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer local-test-token")
                    .body(Body::from(req_body("local-only", "research")))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert_eq!(
            server.received_requests().await.unwrap_or_default().len(),
            0
        );
    }

    /// Valid token, cloud-allowed tag, but an un-allowlisted label — refused
    /// before any network attempt.
    #[tokio::test]
    async fn unallowlisted_label_refused_before_any_upstream_call() {
        let server = MockServer::start().await;
        let cfg = config(server.uri(), &["research"]);
        let app = app(cfg);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/bridge/complete")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer local-test-token")
                    .body(Body::from(req_body("cloud-allowed", "not-a-real-label")))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert_eq!(
            server.received_requests().await.unwrap_or_default().len(),
            0
        );
    }

    /// Happy path: valid token + cloud-allowed tag + allowlisted label
    /// forwards to upstream with the REAL key injected server-side, and the
    /// real key never appears anywhere in the response returned to the
    /// caller.
    #[tokio::test]
    async fn happy_path_injects_real_key_upstream_and_never_leaks_it_to_caller() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .and(header("authorization", "Bearer sk-REAL-SECRET-DO-NOT-LEAK"))
            .and(header("x-foundry-bridge-label", "research"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "content": "here is the research summary",
                "label_echo": "research"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let cfg = config(server.uri(), &["research"]);
        let app = app(cfg);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/bridge/complete")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer local-test-token")
                    .body(Body::from(req_body("cloud-allowed", "research")))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(bytes.to_vec()).unwrap();

        assert!(
            !body_str.contains("sk-REAL-SECRET-DO-NOT-LEAK"),
            "the real upstream key must never appear in the response returned \
             to the caller, got: {body_str}"
        );
        assert!(body_str.contains("research summary"));
    }
}
