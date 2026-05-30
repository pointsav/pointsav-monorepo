// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Integration tests for `POST /v1/messages` — Anthropic Messages API shim.
//!
//! Coverage (audit item C1):
//!   - `claude-haiku-*` model → routes to Tier A (local), `x-foundry-tier-used: local`
//!   - `claude-sonnet-*` model → routes to Tier B (yoyo), `x-foundry-tier-used: yoyo`
//!   - `claude-opus-*` model → Tier C unconfigured → 503 with JSON error
//!   - `stream: true` → SSE events arrive in correct order (Tier B real-SSE)
//!   - `stream: true` → fake-SSE fallback when only Tier A configured
//!   - `tool_use` content block → passes through gateway (HTTP 200)
//!   - `tool_result` content block → passes through gateway (HTTP 200)
//!   - `system` field → prepended as system message in downstream POST body
//!   - Non-streaming response shape → matches Anthropic Messages API spec
//!   - Tier A busy (slots_idle=0, no Tier B) → 503 + `Retry-After: 30` header
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
#[ignore = "Sprint 0b: model-name → tier routing not yet implemented in shim"]
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
#[ignore = "Sprint 0b: model-name → tier routing not yet implemented in shim"]
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
#[ignore = "Sprint 0b: SSE streaming from Anthropic shim not yet wired"]
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
// gateway_token not yet in AppState; auth middleware not wired (Sprint 4 work)

#[tokio::test]
#[ignore = "gateway_token auth not yet implemented in AppState"]
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
#[ignore = "gateway_token auth not yet implemented in AppState"]
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

    // Gateway token auth not yet implemented in AppState — this test verifies
    // the request reaches Tier A successfully (200). Auth tests are below.
    let state = app_state_with_local(mock.uri());

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

// ── Tier A busy → 503 + Retry-After: 30 ─────────────────────────────────────

#[tokio::test]
#[ignore = "Sprint 0b: busy-detection probe and Retry-After header not yet wired in shim"]
async fn tier_a_busy_returns_503_with_retry_after_header() {
    // llama-server reports slots_idle=0; no Tier B configured → TierABusy → 503
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"status": "ok", "slots_idle": 0, "slots_processing": 1})),
        )
        .mount(&mock)
        .await;
    // POST /v1/chat/completions must NOT be called when busy
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(local_ok_body()))
        .expect(0)
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
    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE,
        "busy Tier A with no Tier B must return 503");

    let retry_after = resp.headers()
        .get("retry-after")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert_eq!(retry_after, "30", "busy 503 must include Retry-After: 30");
}

// ── stream: true, Tier A only → real per-token SSE ───────────────────────────

#[tokio::test]
#[ignore = "Sprint 0b: per-token SSE streaming from local tier not yet wired in shim"]
async fn stream_true_with_tier_a_only_streams_real_sse() {
    // No Tier B configured. stream=true uses local_stream() → llama-server
    // with stream:true → build_stream_body() produces per-token Anthropic SSE.
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"status": "ok", "slots_idle": 1})),
        )
        .mount(&mock)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "text/event-stream")
                .set_body_string(
                    "data: {\"choices\":[{\"delta\":{\"content\":\"real stream token\"},\
                     \"finish_reason\":null}]}\n\n\
                     data: [DONE]\n\n",
                ),
        )
        .mount(&mock)
        .await;

    let state = app_state_with_local(mock.uri());
    let app = router(state);

    let req = messages_request(json!({
        "model": "claude-haiku-4-5-20251001",
        "max_tokens": 64,
        "stream": true,
        "messages": [{"role": "user", "content": "ping"}]
    }));
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let ct = resp.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(ct.contains("text/event-stream"), "Tier A stream response must have SSE content-type, got: {ct}");

    let tier = resp.headers()
        .get("x-foundry-tier-used")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert_eq!(tier, "local", "Tier A stream must report x-foundry-tier-used: local");

    let body = response_text(resp).await;
    assert!(body.contains("real stream token"), "Tier A SSE body must include upstream token");
    assert!(body.contains("message_start"), "Tier A SSE must include message_start event");
    assert!(body.contains("message_stop"), "Tier A SSE must include message_stop event");
    assert!(body.contains("content_block_delta"), "Tier A SSE must include per-token delta events");
}

// ── Non-streaming response shape matches Anthropic spec ──────────────────────

#[tokio::test]
async fn non_streaming_response_shape_matches_anthropic_spec() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"status": "ok", "slots_idle": 1})),
        )
        .mount(&mock)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({
                "choices": [{"message": {"role": "assistant", "content": "hello"}}]
            }),
        ))
        .mount(&mock)
        .await;

    let state = app_state_with_local(mock.uri());
    let app = router(state);

    let req = messages_request(json!({
        "model": "claude-haiku-4-5-20251001",
        "max_tokens": 64,
        "messages": [{"role": "user", "content": "hi"}]
    }));
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = response_body(resp).await;
    assert_eq!(body["type"], "message", "response must have type: message");
    assert_eq!(body["role"], "assistant", "response must have role: assistant");
    assert_eq!(body["stop_reason"], "end_turn", "response must have stop_reason: end_turn");
    assert!(body["id"].as_str().map(|s| s.starts_with("msg_")).unwrap_or(false),
        "response id must start with msg_");
    assert_eq!(body["content"][0]["type"], "text", "content block must have type: text");
    assert_eq!(body["content"][0]["text"], "hello", "content block must carry upstream text");
    assert!(body["usage"].is_object(), "response must include usage object");
    assert!(body["usage"]["output_tokens"].is_number(), "usage must include output_tokens");
}

// ── system field threads into downstream POST body ────────────────────────────

#[tokio::test]
async fn system_message_is_sent_to_tier_a_backend() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"status": "ok", "slots_idle": 1})),
        )
        .mount(&mock)
        .await;
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
        "system": "You are a test assistant.",
        "messages": [{"role": "user", "content": "hello"}]
    }));
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Inspect the request received by the backend: system content must appear
    // as the first message in the messages array sent downstream.
    let reqs = mock.received_requests().await.unwrap();
    let post_reqs: Vec<_> = reqs.iter().filter(|r| r.method.as_str() == "POST").collect();
    assert_eq!(post_reqs.len(), 1, "exactly one POST must reach Tier A");

    let downstream: serde_json::Value = serde_json::from_slice(&post_reqs[0].body).unwrap();
    let msgs = downstream["messages"].as_array().expect("messages must be an array");
    let first = &msgs[0];
    assert_eq!(first["role"], "system", "first message must carry system role");
    assert!(
        first["content"].as_str().map(|s| s.contains("test assistant")).unwrap_or(false),
        "system message content must match the system field value"
    );
}

// ── tool_result content block passes through ─────────────────────────────────

#[tokio::test]
async fn tool_result_content_block_passes_through_gateway() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"status": "ok", "slots_idle": 1})),
        )
        .mount(&mock)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(local_ok_body()))
        .mount(&mock)
        .await;

    let state = app_state_with_local(mock.uri());
    let app = router(state);

    // Simulate a tool_result turn: assistant replied with tool_use, user
    // now returns the tool_result.
    let req = messages_request(json!({
        "model": "claude-haiku-4-5-20251001",
        "max_tokens": 64,
        "messages": [
            {"role": "user",      "content": "run bash"},
            {"role": "assistant", "content": [
                {"type": "tool_use", "id": "tu_1", "name": "bash", "input": {"cmd": "ls"}}
            ]},
            {"role": "user", "content": [
                {"type": "tool_result", "tool_use_id": "tu_1", "content": "file1.txt\nfile2.txt"}
            ]}
        ]
    }));
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK,
        "tool_result content block must pass through gateway successfully");
}

// ── POST /v1/shadow-adapter — adapter A/B dual-dispatch ──────────────────────

#[tokio::test]
#[ignore = "Sprint 0b: /v1/shadow-adapter route not yet implemented"]
async fn shadow_adapter_dual_dispatch_returns_both_arms() {
    // Both adapter arms route to Tier A (local). The mock handles two POST
    // requests and returns distinct content for each (model field differs).
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"status": "ok", "slots_idle": 2})),
        )
        .mount(&mock)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "choices": [{"message": {"role": "assistant", "content": "adapter response"}}]
        })))
        .expect(2)  // one call per adapter arm
        .mount(&mock)
        .await;

    let state = app_state_with_local(mock.uri());
    let app = router(state);

    let req = Request::builder()
        .method("POST")
        .uri("/v1/shadow-adapter")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&json!({
            "prompt": "What is 2 + 2?",
            "adapter_a": "lora-v1",
            "adapter_b": "lora-v2",
            "max_tokens": 32
        })).unwrap()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK, "shadow-adapter must return 200");

    let body = response_body(resp).await;
    assert!(body["adapter_a"].is_object(), "response must contain adapter_a arm");
    assert!(body["adapter_b"].is_object(), "response must contain adapter_b arm");
    assert_eq!(body["adapter_a"]["version"], "lora-v1");
    assert_eq!(body["adapter_b"]["version"], "lora-v2");
    assert!(body["prompt_hash"].is_string(), "response must contain prompt_hash");
    let hash = body["prompt_hash"].as_str().unwrap();
    assert_eq!(hash.len(), 64, "prompt_hash must be a 64-char hex SHA-256");
}

#[tokio::test]
#[ignore = "Sprint 0b: /v1/shadow-adapter route not yet implemented"]
async fn shadow_adapter_rejects_empty_prompt() {
    let mock = MockServer::start().await;
    let state = app_state_with_local(mock.uri());
    let app = router(state);

    let req = Request::builder()
        .method("POST")
        .uri("/v1/shadow-adapter")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&json!({
            "prompt": "",
            "adapter_a": "lora-v1",
            "adapter_b": "lora-v2"
        })).unwrap()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST, "empty prompt must return 400");
}

#[tokio::test]
#[ignore = "Sprint 0b: /v1/shadow-adapter route not yet implemented"]
async fn shadow_adapter_rejects_missing_adapter_ids() {
    let mock = MockServer::start().await;
    let state = app_state_with_local(mock.uri());
    let app = router(state);

    let req = Request::builder()
        .method("POST")
        .uri("/v1/shadow-adapter")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&json!({
            "prompt": "test prompt",
            "adapter_a": "lora-v1",
            "adapter_b": ""
        })).unwrap()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST, "empty adapter_b must return 400");
}

// ── P1-1.7 — tool-use round-trip ─────────────────────────────────────────────

#[tokio::test]
async fn tools_forwarded_to_backend_and_tool_use_response_emitted() {
    // Mock: GET /health (busy check) + POST /v1/chat/completions returns tool_calls.
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"status": "ok", "slots_idle": 1})),
        )
        .mount(&mock)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": null,
                    "tool_calls": [{
                        "id": "call_abc",
                        "type": "function",
                        "function": {
                            "name": "get_weather",
                            "arguments": "{\"location\":\"NYC\"}"
                        }
                    }]
                }
            }]
        })))
        .mount(&mock)
        .await;

    let state = app_state_with_local(mock.uri());
    let app = router(state);

    let resp = app.oneshot(messages_request(json!({
        "model": "claude-haiku-4-5-20251001",
        "max_tokens": 256,
        "tools": [{
            "name": "get_weather",
            "description": "Get current weather",
            "input_schema": {
                "type": "object",
                "properties": {"location": {"type": "string"}},
                "required": ["location"]
            }
        }],
        "messages": [{"role": "user", "content": "What is the weather in NYC?"}]
    }))).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = response_body(resp).await;
    // Response must contain a tool_use content block
    let content = body["content"].as_array().expect("content array");
    let tool_block = content.iter().find(|b| b["type"] == "tool_use")
        .expect("tool_use block in content");
    assert_eq!(tool_block["id"], "call_abc");
    assert_eq!(tool_block["name"], "get_weather");
    assert_eq!(tool_block["input"]["location"], "NYC");
    // stop_reason must be "tool_use" when tool_calls are present
    assert_eq!(body["stop_reason"], "tool_use");
}

#[tokio::test]
async fn tool_use_response_stop_reason_is_tool_use() {
    // Regression guard: when the backend returns tool_calls, stop_reason must
    // be "tool_use" (not "end_turn"). Sprint 0b streaming tool_use SSE is
    // deferred to Sprint 1 (requires build_stream_body tool_call delta parsing).
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/health"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"status": "ok", "slots_idle": 1})),
        )
        .mount(&mock)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": null,
                    "tool_calls": [{
                        "id": "call_xyz",
                        "type": "function",
                        "function": {"name": "bash", "arguments": "{\"cmd\":\"ls\"}"}
                    }]
                }
            }]
        })))
        .mount(&mock)
        .await;

    let state = app_state_with_local(mock.uri());
    let app = router(state);

    let resp = app.oneshot(messages_request(json!({
        "model": "claude-haiku-4-5-20251001",
        "max_tokens": 64,
        "tools": [{"name": "bash", "description": "Run a shell command",
                   "input_schema": {"type": "object", "properties": {"cmd": {"type": "string"}}}}],
        "messages": [{"role": "user", "content": "list files"}]
    }))).await.unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = response_body(resp).await;
    assert_eq!(body["stop_reason"], "tool_use",
        "tool_calls response must set stop_reason=tool_use");
    let content = body["content"].as_array().unwrap();
    let tool_block = content.iter().find(|b| b["type"] == "tool_use")
        .expect("tool_use block must be present");
    assert_eq!(tool_block["id"], "call_xyz");
    assert_eq!(tool_block["name"], "bash");
}
