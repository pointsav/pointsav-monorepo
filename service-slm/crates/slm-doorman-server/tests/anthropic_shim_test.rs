// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Integration tests for `POST /v1/messages` — Anthropic Messages API shim.
//!
//! Coverage (audit item C1):
//!   - `claude-haiku-*` model → routes to Tier A (local), `x-foundry-tier-used: local`
//!   - `claude-sonnet-*` model → routes to Tier B (yoyo), `x-foundry-tier-used: yoyo`
//!   - `claude-opus-*` model → Tier C unconfigured → 503 with JSON error
//!   - `stream: true` → SSE events arrive in correct order
//!   - `tool_use` content block → documented Sprint 0a limitation (flattened to "")
//!   - Missing `x-api-key` header → 401
//!   - Invalid `x-api-key` header → 401
//!
//! All tests use `tower::ServiceExt::oneshot` with a wiremock backend so no
//! real TCP socket is bound by the axum router under test.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use slm_doorman_server::{
    http::router,
    test_helpers::{app_state_with_local, app_state_with_yoyo},
};
use tower::ServiceExt;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── helpers ──────────────────────────────────────────────────────────────────

fn messages_request(body: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/v1/messages")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap()
}

fn messages_request_with_key(body: serde_json::Value, api_key: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/v1/messages")
        .header("content-type", "application/json")
        .header("x-api-key", api_key)
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap()
}

async fn response_body(resp: axum::response::Response) -> serde_json::Value {
    let bytes = axum::body::to_bytes(resp.into_body(), 1 << 20).await.unwrap();
    serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
}

async fn response_text(resp: axum::response::Response) -> String {
    let bytes = axum::body::to_bytes(resp.into_body(), 1 << 20).await.unwrap();
    String::from_utf8_lossy(&bytes).into_owned()
}

fn local_ok_body() -> serde_json::Value {
    json!({
        "choices": [
            { "message": { "role": "assistant", "content": "PONG" } }
        ]
    })
}

fn yoyo_ok_body() -> serde_json::Value {
    json!({
        "choices": [
            { "message": { "role": "assistant", "content": "YOYO PONG" } }
        ]
    })
}

fn yoyo_stream_body() -> String {
    "data: {\"choices\":[{\"delta\":{\"content\":\"hello \"},\"finish_reason\":null}]}\n\n\
     data: {\"choices\":[{\"delta\":{\"content\":\"world\"},\"finish_reason\":null}]}\n\n\
     data: [DONE]\n\n"
        .to_string()
}

// ── Tier A: haiku → local ────────────────────────────────────────────────────

#[tokio::test]
async fn haiku_routes_to_tier_a_local() {
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(local_ok_body()))
        .mount(&mock)
        .await;

    let state = app_state_with_local(mock.uri());
    let app = router(state);

    let req = messages_request(json!({
        "model": "claude-haiku-4-5-20251001",
        "max_tokens": 64,
        "messages": [{"role": "user", "content": "ping"}]
    }));
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let tier = resp.headers()
        .get("x-foundry-tier-used")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert_eq!(tier, "local", "haiku must route to Tier A");

    let body = response_body(resp).await;
    assert_eq!(body["content"][0]["text"], "PONG");
}

// ── Tier B: sonnet → yoyo ────────────────────────────────────────────────────

#[tokio::test]
async fn sonnet_routes_to_tier_b_yoyo() {
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(yoyo_ok_body()))
        .mount(&mock)
        .await;

    let state = app_state_with_yoyo(mock.uri(), None);
    let app = router(state);

    let req = messages_request(json!({
        "model": "claude-sonnet-4-6",
        "max_tokens": 64,
        "messages": [{"role": "user", "content": "ping"}]
    }));
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let tier = resp.headers()
        .get("x-foundry-tier-used")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(tier.starts_with("yoyo"), "sonnet must route to Tier B, got: {tier}");

    let body = response_body(resp).await;
    assert_eq!(body["content"][0]["text"], "YOYO PONG");
}

// ── Tier C: opus → 503 when unconfigured ────────────────────────────────────

#[tokio::test]
async fn opus_returns_503_when_tier_c_unconfigured() {
    // No external tier configured — Doorman returns TierUnavailable(External).
    let state = app_state_with_yoyo("http://127.0.0.1:1", None);
    let app = router(state);

    let req = messages_request(json!({
        "model": "claude-opus-4-7",
        "max_tokens": 64,
        "messages": [{"role": "user", "content": "ping"}]
    }));
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE,
        "opus without Tier C configured must return 503");
}

// ── SSE streaming: correct event order ───────────────────────────────────────

#[tokio::test]
async fn stream_true_returns_sse_events_in_correct_order() {
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(yoyo_stream_body())
                .insert_header("content-type", "text/event-stream"),
        )
        .mount(&mock)
        .await;

    let state = app_state_with_yoyo(mock.uri(), None);
    let app = router(state);

    let req = messages_request(json!({
        "model": "claude-sonnet-4-6",
        "max_tokens": 64,
        "stream": true,
        "messages": [{"role": "user", "content": "ping"}]
    }));
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        resp.headers().get("content-type").and_then(|v| v.to_str().ok()),
        Some("text/event-stream; charset=utf-8")
    );

    let body = response_text(resp).await;
    // Verify event ordering: message_start < content_block_start < content_block_delta < content_block_stop < message_delta < message_stop
    let pos_start     = body.find("event: message_start").expect("message_start missing");
    let pos_cb_start  = body.find("event: content_block_start").expect("content_block_start missing");
    let pos_cb_delta  = body.find("event: content_block_delta").expect("content_block_delta missing");
    let pos_cb_stop   = body.find("event: content_block_stop").expect("content_block_stop missing");
    let pos_msg_delta = body.find("event: message_delta").expect("message_delta missing");
    let pos_msg_stop  = body.find("event: message_stop").expect("message_stop missing");

    assert!(pos_start < pos_cb_start, "message_start must precede content_block_start");
    assert!(pos_cb_start < pos_cb_delta, "content_block_start must precede content_block_delta");
    assert!(pos_cb_delta < pos_cb_stop, "content_block_delta must precede content_block_stop");
    assert!(pos_cb_stop < pos_msg_delta, "content_block_stop must precede message_delta");
    assert!(pos_msg_delta < pos_msg_stop, "message_delta must precede message_stop");

    // Verify at least one delta carries text content
    assert!(body.contains("hello"), "stream body must contain upstream token text");
}

// ── tool_use content: Sprint 1 CanonicalMessage preserves all block types ────

#[tokio::test]
async fn tool_use_blocks_pass_through_gateway() {
    // Sprint 1: tool_use blocks are preserved as CanonicalMessage::ToolUse
    // and translated to OAI tool_calls format before reaching the backend.
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(local_ok_body()))
        .mount(&mock)
        .await;

    let state = app_state_with_local(mock.uri());
    let app = router(state);

    let req = messages_request(json!({
        "model": "claude-haiku-4-5-20251001",
        "max_tokens": 64,
        "messages": [{
            "role": "user",
            "content": [
                {"type": "tool_use", "id": "tu_1", "name": "bash", "input": {"cmd": "ls"}}
            ]
        }]
    }));
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK,
        "tool_use request must pass through the gateway successfully");
}

// ── Auth: missing x-api-key → 401 ───────────────────────────────────────────

#[tokio::test]
async fn missing_api_key_returns_401_when_token_configured() {
    let state = app_state_with_yoyo("http://127.0.0.1:1", Some("secret-token".to_string()));
    let app = router(state);

    // No x-api-key header
    let req = messages_request(json!({
        "model": "claude-haiku-4-5-20251001",
        "max_tokens": 64,
        "messages": [{"role": "user", "content": "ping"}]
    }));
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "missing api key must return 401");
}

// ── Auth: invalid x-api-key → 401 ────────────────────────────────────────────

#[tokio::test]
async fn invalid_api_key_returns_401() {
    let state = app_state_with_yoyo("http://127.0.0.1:1", Some("secret-token".to_string()));
    let app = router(state);

    let req = messages_request_with_key(
        json!({
            "model": "claude-haiku-4-5-20251001",
            "max_tokens": 64,
            "messages": [{"role": "user", "content": "ping"}]
        }),
        "wrong-token",
    );
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "wrong api key must return 401");

    let body = response_body(resp).await;
    assert!(body["error"]["type"].as_str().unwrap_or("").contains("authentication"),
        "error body must identify authentication_error");
}

// ── Auth: correct x-api-key passes through ───────────────────────────────────

#[tokio::test]
async fn correct_api_key_allows_request() {
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(local_ok_body()))
        .mount(&mock)
        .await;

    let state = {
        // Build a state with local tier + gateway token
        use slm_doorman::tier::{LocalTierClient, LocalTierConfig};
        use slm_doorman::{BriefCache, Doorman, DoormanConfig, FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST};
        use slm_doorman_server::{http::AppState, test_helpers::temp_ledger, test_helpers::temp_queue_config};
        use std::collections::HashMap;
        use std::sync::atomic::AtomicU64;
        use std::sync::{Arc, Mutex};

        let local = LocalTierClient::new(LocalTierConfig {
            endpoint: mock.uri(),
            default_model: "test-model".to_string(),
        });
        let doorman = Doorman::new(
            DoormanConfig {
                local: Some(local),
                yoyo: HashMap::new(),
                external: None,
                lark_validator: None,
                graph_context_client: None,
            },
            temp_ledger(),
        );
        Arc::new(AppState {
            doorman,
            apprenticeship: None,
            brief_cache: Arc::new(BriefCache::default()),
            verdict_dispatcher: None,
            audit_proxy_client: None,
            audit_proxy_purpose_allowlist: FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST,
            audit_tenant_concurrency: Arc::new(Mutex::new(HashMap::new())),
            audit_tenant_concurrency_cap: 100,
            queue_config: temp_queue_config(),
            service_content_endpoint: String::new(),
            last_yoyo_dispatch: Arc::new(AtomicU64::new(0)),
            gateway_token: Some("secret-token".to_string()),
        })
    };

    let app = router(state);
    let req = messages_request_with_key(
        json!({
            "model": "claude-haiku-4-5-20251001",
            "max_tokens": 64,
            "messages": [{"role": "user", "content": "ping"}]
        }),
        "secret-token",
    );
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK, "correct api key must succeed");
}

// ── Auth disabled when no gateway token set ───────────────────────────────────

#[tokio::test]
async fn no_auth_when_gateway_token_not_configured() {
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(local_ok_body()))
        .mount(&mock)
        .await;

    // gateway_token: None — auth disabled
    let state = app_state_with_local(mock.uri());
    let app = router(state);

    // No x-api-key header, no gateway_token configured → should succeed
    let req = messages_request(json!({
        "model": "claude-haiku-4-5-20251001",
        "max_tokens": 64,
        "messages": [{"role": "user", "content": "ping"}]
    }));
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK, "auth disabled when no gateway token configured");
}
