// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Integration tests for the audit endpoint pair — `POST /v1/audit/proxy` and
//! `POST /v1/audit/capture` — exercising them together as cross-cluster
//! consumers will use them.
//!
//! These tests complement the per-endpoint unit coverage in `http_test.rs`.
//! Where `http_test.rs` exercises each endpoint in isolation (one request,
//! one ledger), the tests here exercise both endpoints together in a single
//! shared ledger, verifying:
//!   1. Cross-endpoint round-trip: capture + proxy entries land in the same
//!      JSONL file; audit_ids are distinct; entry shapes are independent.
//!   2. Failure isolation: a failed proxy call (upstream 500) writes stub +
//!      final-with-error entries, while a subsequent capture call still
//!      succeeds independently alongside those entries.
//!   3. Field-presence discrimination: the documented algorithm for
//!      identifying entry types by field presence (from the contract doc
//!      §3.2) works correctly on a deliberately mixed JSONL stream.
//!
//! All tests use `tower::ServiceExt::oneshot` without a real TCP socket.
//! No live API calls per the standing operator guardrail.

use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use slm_doorman::ledger::{
    ENTRY_TYPE_AUDIT_CAPTURE, ENTRY_TYPE_AUDIT_PROXY, ENTRY_TYPE_AUDIT_PROXY_STUB,
    ENTRY_TYPE_CHAT_COMPLETION,
};
use slm_doorman::tier::{TierCPricing, TierCProvider};
use slm_doorman::{
    AuditCaptureEntry, AuditLedger, AuditProxyClient, AuditProxyConfig, AuditProxyEntry,
    AuditProxyStubEntry, BriefCache, Doorman, DoormanConfig, FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST,
};
use slm_doorman_server::http::{router, AppState};
use tower::ServiceExt;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Build a POST request with a JSON body.
fn post_json(uri: &str, body: &serde_json::Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("build request")
}

/// Decode a response body to a `serde_json::Value`.
async fn body_json(resp: axum::response::Response) -> serde_json::Value {
    let bytes = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .expect("read response body");
    serde_json::from_slice(&bytes).expect("response body is JSON")
}

/// Create a unique temp dir for a test ledger so parallel tests do not race.
fn unique_ledger_dir(test_name: &str) -> std::path::PathBuf {
    let suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dir = std::env::temp_dir().join(format!("slm-audit-integration-{test_name}-{suffix}"));
    std::fs::create_dir_all(&dir).expect("create test ledger dir");
    dir
}

/// Build an `AppState` with an `AuditProxyClient` pointing at `server_uri`.
/// The Doorman's audit ledger is rooted at `ledger_dir` so callers can
/// inspect written JSONL after the request.
fn app_state_with_proxy_and_ledger_dir(
    server_uri: impl Into<String>,
    ledger_dir: &std::path::Path,
) -> Arc<AppState> {
    let ledger = AuditLedger::new(ledger_dir).expect("create test audit ledger");

    let mut endpoints = HashMap::new();
    endpoints.insert(TierCProvider::Anthropic, server_uri.into());
    let mut keys = HashMap::new();
    keys.insert(
        TierCProvider::Anthropic,
        "sk-ant-test-DO-NOT-USE-LIVE".to_string(),
    );
    let audit_config = AuditProxyConfig {
        provider_endpoints: endpoints,
        provider_api_keys: keys,
        pricing: TierCPricing {
            anthropic_input_per_mtok_usd: 0.25,
            anthropic_output_per_mtok_usd: 1.25,
            ..Default::default()
        },
        purpose_allowlist: FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST,
    };
    let audit_client = AuditProxyClient::new(audit_config);
    let doorman = Doorman::new(DoormanConfig::default(), ledger);

    Arc::new(AppState {
        doorman,
        apprenticeship: None,
        brief_cache: Arc::new(BriefCache::default()),
        verdict_dispatcher: None,
        audit_proxy_client: Some(audit_client),
        audit_proxy_purpose_allowlist: FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST,
    })
}

/// Read all JSONL lines from the ledger dir as `serde_json::Value` objects.
/// Asserts exactly one `.jsonl` file exists.
fn read_ledger_lines(ledger_dir: &std::path::Path) -> Vec<serde_json::Value> {
    let jsonl_files: Vec<_> = std::fs::read_dir(ledger_dir)
        .expect("read ledger dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "jsonl").unwrap_or(false))
        .collect();
    assert_eq!(
        jsonl_files.len(),
        1,
        "expected exactly one JSONL file in ledger dir; got {}",
        jsonl_files.len()
    );
    let contents = std::fs::read_to_string(jsonl_files[0].path()).expect("read JSONL file");
    contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("every ledger line must be valid JSON"))
        .collect()
}

// ---------------------------------------------------------------------------
// Test 1 — capture + proxy round-trip in the same ledger
// ---------------------------------------------------------------------------
//
// Scenario: project-language Task (A-4) caller does two things in sequence:
//   (1) Submits a `prose-edit` capture event for a local refinement pass it
//       already completed.
//   (2) Submits an `audit_proxy` request for a citation-grounding call to
//       Anthropic via the Doorman.
//
// Expected ledger state after both calls:
//   - Three JSONL entries in the same daily file:
//       (a) AuditCaptureEntry for the prose-edit
//       (b) AuditProxyStubEntry (status "inbound") for the proxy call
//       (c) AuditProxyEntry (status "ok", with tokens + cost) for the proxy call
//   - Entry (a) is identified by the `event_type` field.
//   - Entries (b) and (c) share the proxy `audit_id` (returned in response).
//   - The capture `audit_id` (caller-generated) is distinct from the proxy
//     `audit_id` (Doorman-generated).

/// Caller publishes a prose-edit capture event (local refinement), then
/// submits an audit_proxy citation-grounding call. Both land in the same
/// daily JSONL file; field-presence discrimination correctly identifies the
/// capture entry and both proxy entries.
#[tokio::test]
async fn audit_capture_then_audit_proxy_round_trip() {
    // Start a wiremock server to back the Anthropic provider in the proxy call.
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "choices": [
                { "message": { "role": "assistant", "content": "citation verified" } }
            ],
            "usage": {
                "prompt_tokens": 80,
                "completion_tokens": 32
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let ledger_dir = unique_ledger_dir("round-trip");
    let state = app_state_with_proxy_and_ledger_dir(server.uri(), &ledger_dir);

    // ── Step 1: POST /v1/audit/capture (prose-edit) ──────────────────────────

    let capture_audit_id = "01930b12-0000-7000-0000-000000000001";
    let capture_body = json!({
        "audit_id": capture_audit_id,
        "module_id": "woodfine",
        "event_type": "prose-edit",
        "source": "project-language",
        "status": "ok",
        "event_at": "2026-04-28T14:23:00Z",
        "payload": {
            "draft_id": "topic-doorman-protocol",
            "edit_pass": "structural-register",
            "word_delta": 42
        },
        "caller_request_id": "lang-seq-001"
    });

    let capture_resp = router(state.clone())
        .oneshot(post_json("/v1/audit/capture", &capture_body))
        .await
        .expect("capture oneshot");

    assert_eq!(
        capture_resp.status(),
        StatusCode::OK,
        "capture must return 200 OK"
    );
    let capture_resp_body = body_json(capture_resp).await;
    assert_eq!(
        capture_resp_body["audit_id"].as_str().unwrap_or(""),
        capture_audit_id,
        "capture response must echo the caller-generated audit_id"
    );
    assert_eq!(
        capture_resp_body["status"].as_str().unwrap_or(""),
        "captured"
    );

    // ── Step 2: POST /v1/audit/proxy (citation-grounding) ────────────────────

    let proxy_body = json!({
        "module_id": "woodfine",
        "purpose": "citation-grounding",
        "provider": "anthropic",
        "model": "claude-opus-4-7",
        "messages": [{"role": "user", "content": "Verify this citation: ..."}],
        "max_tokens": 128,
        "caller_request_id": "lang-seq-002"
    });

    let proxy_resp = router(state.clone())
        .oneshot(post_json("/v1/audit/proxy", &proxy_body))
        .await
        .expect("proxy oneshot");

    assert_eq!(
        proxy_resp.status(),
        StatusCode::OK,
        "proxy must return 200 OK"
    );
    let proxy_resp_body = body_json(proxy_resp).await;
    let proxy_audit_id = proxy_resp_body["audit_id"]
        .as_str()
        .expect("proxy audit_id must be present");
    assert!(!proxy_audit_id.is_empty());
    assert_ne!(
        proxy_audit_id, capture_audit_id,
        "proxy audit_id (Doorman-generated) must differ from capture audit_id (caller-generated)"
    );
    assert_eq!(
        proxy_resp_body["content"].as_str().unwrap_or(""),
        "citation verified"
    );
    assert_eq!(
        proxy_resp_body["usage"]["prompt_tokens"]
            .as_u64()
            .unwrap_or(0),
        80
    );

    // ── Step 3: Verify ledger state ───────────────────────────────────────────

    let lines = read_ledger_lines(&ledger_dir);
    assert_eq!(
        lines.len(),
        3,
        "ledger must have 3 entries: capture + stub + final; got {}",
        lines.len()
    );

    // Entry 0 is the capture (written first).
    let capture_entry = &lines[0];
    assert!(
        capture_entry.get("event_type").is_some(),
        "first entry must be AuditCaptureEntry (has event_type field)"
    );
    assert_eq!(
        capture_entry["audit_id"].as_str().unwrap_or(""),
        capture_audit_id
    );
    assert_eq!(
        capture_entry["event_type"].as_str().unwrap_or(""),
        "prose-edit"
    );
    assert_eq!(
        capture_entry["module_id"].as_str().unwrap_or(""),
        "woodfine"
    );

    // Entry 1 is the proxy stub.
    let stub_entry = &lines[1];
    assert!(
        stub_entry.get("provider").is_some(),
        "stub entry must have provider field"
    );
    assert_eq!(
        stub_entry["status"].as_str().unwrap_or(""),
        "inbound",
        "stub status must be 'inbound'"
    );
    assert_eq!(
        stub_entry["audit_id"].as_str().unwrap_or(""),
        proxy_audit_id
    );
    assert!(
        stub_entry.get("event_type").is_none(),
        "stub entry must NOT have event_type field (would be misidentified as AuditCaptureEntry)"
    );

    // Entry 2 is the proxy final outcome.
    let final_entry = &lines[2];
    assert!(
        final_entry.get("provider").is_some(),
        "final entry must have provider field"
    );
    assert!(
        final_entry.get("prompt_tokens").is_some(),
        "final entry must have prompt_tokens field"
    );
    assert_eq!(
        final_entry["audit_id"].as_str().unwrap_or(""),
        proxy_audit_id
    );
    assert_eq!(final_entry["status"].as_str().unwrap_or(""), "ok");
    assert_eq!(final_entry["prompt_tokens"].as_u64().unwrap_or(0), 80);

    // Confirm wiremock saw exactly one request.
    // (The MockServer drop at end-of-scope verifies expect(1).)
}

// ---------------------------------------------------------------------------
// Test 2 — proxy failure + subsequent capture succeed independently
// ---------------------------------------------------------------------------
//
// Scenario: an upstream provider returns 500, causing the proxy to write stub
// + final-with-error entries. Immediately after, a capture call from the same
// caller succeeds without interference. Demonstrates failure isolation:
//   - Proxy failure does not corrupt the ledger for capture entries.
//   - The stub entry (status "inbound") is always written even when the relay
//     call fails (paper trail integrity).
//   - The final entry carries status "upstream-error" and an error_message.
//   - The subsequent capture entry is written correctly after the failed proxy.

/// audit_proxy upstream returns 500 → stub + final-with-error in ledger;
/// subsequent audit_capture succeeds independently alongside those entries.
#[tokio::test]
async fn audit_proxy_failure_records_stub_only_then_capture_succeeds_independently() {
    // Wiremock returns 500 for the proxy call.
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(500).set_body_string("internal server error"))
        .expect(1)
        .mount(&server)
        .await;

    let ledger_dir = unique_ledger_dir("proxy-fail-capture-ok");
    let state = app_state_with_proxy_and_ledger_dir(server.uri(), &ledger_dir);

    // ── Step 1: POST /v1/audit/proxy → upstream 500 ──────────────────────────

    let proxy_body = json!({
        "module_id": "woodfine",
        "purpose": "editorial-refinement",
        "provider": "anthropic",
        "model": "claude-opus-4-7",
        "messages": [{"role": "user", "content": "Polish this paragraph."}],
        "caller_request_id": "fail-seq-001"
    });

    let proxy_resp = router(state.clone())
        .oneshot(post_json("/v1/audit/proxy", &proxy_body))
        .await
        .expect("proxy oneshot");

    // A relay 500 → Doorman surfaces as 502 BAD_GATEWAY (UpstreamShape).
    assert_eq!(
        proxy_resp.status(),
        StatusCode::BAD_GATEWAY,
        "upstream 500 must result in 502 BAD_GATEWAY"
    );

    // ── Step 2: POST /v1/audit/capture → success ─────────────────────────────

    let capture_audit_id = "01930c44-0000-7000-0000-000000000099";
    let capture_body = json!({
        "audit_id": capture_audit_id,
        "module_id": "foundry",
        "event_type": "anchor-event",
        "source": "project-data:anchor-emitter",
        "status": "ok",
        "event_at": "2026-04-28T02:00:07Z",
        "payload": {
            "batch_size": 128,
            "tree_root": "sha256:abc123"
        }
    });

    let capture_resp = router(state.clone())
        .oneshot(post_json("/v1/audit/capture", &capture_body))
        .await
        .expect("capture oneshot");

    assert_eq!(
        capture_resp.status(),
        StatusCode::OK,
        "capture must succeed even after a proxy failure"
    );
    let capture_resp_body = body_json(capture_resp).await;
    assert_eq!(
        capture_resp_body["status"].as_str().unwrap_or(""),
        "captured"
    );

    // ── Step 3: Verify ledger state ───────────────────────────────────────────

    let lines = read_ledger_lines(&ledger_dir);
    assert_eq!(
        lines.len(),
        3,
        "ledger must have 3 entries: proxy stub + proxy final-error + capture; got {}",
        lines.len()
    );

    // Entry 0: proxy stub (always written, even on relay failure).
    let stub = &lines[0];
    assert!(stub.get("provider").is_some(), "stub must have provider");
    assert_eq!(
        stub["status"].as_str().unwrap_or(""),
        "inbound",
        "stub status must be 'inbound'"
    );
    assert!(
        stub.get("event_type").is_none(),
        "stub must NOT have event_type"
    );

    // Entry 1: proxy final entry with upstream-error status.
    let final_entry = &lines[1];
    assert!(
        final_entry.get("provider").is_some(),
        "final entry must have provider"
    );
    assert!(
        final_entry.get("prompt_tokens").is_some(),
        "final entry must have prompt_tokens"
    );
    assert_eq!(
        final_entry["status"].as_str().unwrap_or(""),
        "upstream-error",
        "final entry status must be 'upstream-error'"
    );
    let error_msg = final_entry["error_message"].as_str().unwrap_or("");
    assert!(
        !error_msg.is_empty(),
        "final entry must carry a non-empty error_message on upstream-error"
    );

    // Entry 2: capture entry — written successfully despite prior proxy failure.
    let capture_entry = &lines[2];
    assert!(
        capture_entry.get("event_type").is_some(),
        "capture entry must have event_type"
    );
    assert_eq!(
        capture_entry["audit_id"].as_str().unwrap_or(""),
        capture_audit_id
    );
    assert_eq!(
        capture_entry["event_type"].as_str().unwrap_or(""),
        "anchor-event"
    );
    assert_eq!(capture_entry["module_id"].as_str().unwrap_or(""), "foundry");
}

// ---------------------------------------------------------------------------
// Test 3 — field-presence discrimination on a mixed JSONL stream
// ---------------------------------------------------------------------------
//
// The contract doc §3.2 defines how consumers identify entry types without an
// explicit `entry_type` discriminator:
//
//   (a) `event_type` field present           → AuditCaptureEntry
//   (b) `provider` + `status == "inbound"`   → AuditProxyStubEntry
//   (c) `provider` + `prompt_tokens` present → AuditProxyEntry (final)
//   (d) none of the above                    → AuditEntry (chat-completion)
//
// This test synthesises one entry of each type directly via the ledger API,
// then reads the JSONL stream back and verifies the discrimination algorithm
// correctly labels all four. This is the executable form of the contract doc's
// "distinguishing entries" section.
//
// This test uses the ledger API directly (no HTTP router) so it can
// inject all four entry types cheaply without requiring live compute
// tiers, upstream providers, or wiremock servers for the AuditEntry case.

/// Synthesise a four-entry mixed JSONL stream (one of each entry type) and
/// verify that field-presence discrimination correctly identifies every entry.
#[test]
fn mixed_entry_types_in_jsonl_stream_distinguishable_by_field_presence() {
    use chrono::Utc;
    use slm_core::{ModuleId, RequestId, Tier};
    use slm_doorman::ledger::{AuditEntry, CompletionStatus};
    use std::str::FromStr;

    let ledger_dir = unique_ledger_dir("mixed-stream");
    let ledger = AuditLedger::new(&ledger_dir).expect("create ledger");

    let module_id = ModuleId::from_str("woodfine").unwrap();
    let now = Utc::now();

    // ── Entry A: AuditEntry (chat-completion routing) ─────────────────────────

    let chat_entry = AuditEntry {
        entry_type: ENTRY_TYPE_CHAT_COMPLETION.to_string(),
        timestamp_utc: now,
        request_id: RequestId::new(),
        module_id: module_id.clone(),
        tier: Tier::Local,
        model: "olmo-3-7b-q4".to_string(),
        inference_ms: 412,
        cost_usd: 0.0,
        sanitised_outbound: true,
        completion_status: CompletionStatus::Ok,
        error_message: None,
    };
    ledger.append(&chat_entry).expect("append AuditEntry");

    // ── Entry B: AuditProxyStubEntry ──────────────────────────────────────────

    let stub = AuditProxyStubEntry {
        entry_type: ENTRY_TYPE_AUDIT_PROXY_STUB.to_string(),
        audit_id: "stub-audit-id-001".to_string(),
        inbound_at: now,
        module_id: module_id.clone(),
        purpose: "editorial-refinement".to_string(),
        provider: "anthropic".to_string(),
        model: "claude-opus-4-7".to_string(),
        caller_request_id: Some("stub-caller-001".to_string()),
        request_messages_count: 1,
        status: "inbound".to_string(),
    };
    ledger
        .append_proxy_stub(&stub)
        .expect("append AuditProxyStubEntry");

    // ── Entry C: AuditProxyEntry (final outcome, success) ────────────────────

    let proxy_final = AuditProxyEntry {
        entry_type: ENTRY_TYPE_AUDIT_PROXY.to_string(),
        audit_id: "stub-audit-id-001".to_string(),
        completed_at: now,
        module_id: module_id.clone(),
        purpose: "editorial-refinement".to_string(),
        provider: "anthropic".to_string(),
        model: "claude-opus-4-7".to_string(),
        caller_request_id: Some("stub-caller-001".to_string()),
        prompt_tokens: 50,
        completion_tokens: 20,
        cost_usd: 0.0000375,
        latency_ms: 234,
        status: "ok".to_string(),
        error_message: None,
    };
    ledger
        .append_proxy_entry(&proxy_final)
        .expect("append AuditProxyEntry");

    // ── Entry D: AuditCaptureEntry ────────────────────────────────────────────

    let capture = AuditCaptureEntry {
        entry_type: ENTRY_TYPE_AUDIT_CAPTURE.to_string(),
        audit_id: "capture-audit-id-001".to_string(),
        module_id: module_id.clone(),
        event_type: "verdict-issued".to_string(),
        source: "project-slm".to_string(),
        status: "ok".to_string(),
        event_at: now,
        captured_at: now,
        payload: json!({"verdict": "accept", "score": 0.92}),
        caller_request_id: None,
    };
    ledger
        .append_capture_entry(&capture)
        .expect("append AuditCaptureEntry");

    // ── Read back and discriminate ────────────────────────────────────────────

    let lines = read_ledger_lines(&ledger_dir);
    assert_eq!(lines.len(), 4, "ledger must have exactly 4 entries");

    for (i, line) in lines.iter().enumerate() {
        let identified = discriminate_entry_type(line);
        let expected = match i {
            0 => "AuditEntry",
            1 => "AuditProxyStubEntry",
            2 => "AuditProxyEntry",
            3 => "AuditCaptureEntry",
            _ => unreachable!(),
        };
        assert_eq!(
            identified, expected,
            "entry {i} must be identified as {expected}; field-presence algorithm said {identified}"
        );
    }
}

/// Field-presence discrimination algorithm from contract doc §3.2.
///
/// Returns the entry type name as a `&'static str` for use in assertions.
/// Consumers implement this logic in their own language; this function is the
/// Rust reference implementation.
///
/// Discrimination order (first match wins):
///   1. `event_type` field present            → "AuditCaptureEntry"
///   2. `provider` + `status == "inbound"`    → "AuditProxyStubEntry"
///   3. `provider` + `prompt_tokens` present  → "AuditProxyEntry"
///   4. fallback                              → "AuditEntry"
fn discriminate_entry_type(entry: &serde_json::Value) -> &'static str {
    // Rule 1: AuditCaptureEntry has `event_type`.
    if entry.get("event_type").is_some() {
        return "AuditCaptureEntry";
    }
    // Rule 2: AuditProxyStubEntry has `provider` AND status == "inbound".
    if entry.get("provider").is_some() && entry["status"].as_str() == Some("inbound") {
        return "AuditProxyStubEntry";
    }
    // Rule 3: AuditProxyEntry (final) has `provider` AND `prompt_tokens`.
    if entry.get("provider").is_some() && entry.get("prompt_tokens").is_some() {
        return "AuditProxyEntry";
    }
    // Rule 4: fallback — AuditEntry (chat-completion routing).
    "AuditEntry"
}

// ---------------------------------------------------------------------------
// Test 4 — explicit entry_type tag discriminates all four entry kinds
// ---------------------------------------------------------------------------
//
// Canonical path per contract doc §3.2 v0.2.0: the `entry_type` field
// provides a single-field discriminator for all four entry types. This test
// verifies that:
//   1. The ledger writes the correct canonical string for each entry type.
//   2. Consumers reading the JSONL stream can identify entry kind from a
//      single `entry_type` field without field-presence inference.
//   3. The canonical strings are:
//      - AuditEntry:           "chat-completion"
//      - AuditProxyStubEntry:  "audit-proxy-stub"
//      - AuditProxyEntry:      "audit-proxy"
//      - AuditCaptureEntry:    "audit-capture"

/// Build one entry of each kind via the ledger API; verify the serialised
/// `entry_type` field matches the canonical string for that kind.
#[test]
fn entry_type_tag_discriminates_all_entry_kinds() {
    use chrono::Utc;
    use slm_core::{ModuleId, RequestId, Tier};
    use slm_doorman::ledger::{AuditEntry, CompletionStatus};
    use std::str::FromStr;

    let ledger_dir = unique_ledger_dir("entry-type-tag");
    let ledger = AuditLedger::new(&ledger_dir).expect("create ledger");

    let module_id = ModuleId::from_str("woodfine").unwrap();
    let now = Utc::now();

    // Write one entry of each kind via the append_* API.
    // The append_* methods force the canonical entry_type at write time;
    // the value in the struct literal is irrelevant — the canonical constant
    // is what will appear in the serialised JSONL.

    // AuditEntry
    ledger
        .append(&AuditEntry {
            entry_type: ENTRY_TYPE_CHAT_COMPLETION.to_string(),
            timestamp_utc: now,
            request_id: RequestId::new(),
            module_id: module_id.clone(),
            tier: Tier::Local,
            model: "olmo-3-7b-q4".to_string(),
            inference_ms: 200,
            cost_usd: 0.0,
            sanitised_outbound: false,
            completion_status: CompletionStatus::Ok,
            error_message: None,
        })
        .expect("append AuditEntry");

    // AuditProxyStubEntry
    ledger
        .append_proxy_stub(&AuditProxyStubEntry {
            entry_type: ENTRY_TYPE_AUDIT_PROXY_STUB.to_string(),
            audit_id: "tag-test-audit-001".to_string(),
            inbound_at: now,
            module_id: module_id.clone(),
            purpose: "citation-grounding".to_string(),
            provider: "anthropic".to_string(),
            model: "claude-opus-4-7".to_string(),
            caller_request_id: None,
            request_messages_count: 2,
            status: "inbound".to_string(),
        })
        .expect("append AuditProxyStubEntry");

    // AuditProxyEntry (final)
    ledger
        .append_proxy_entry(&AuditProxyEntry {
            entry_type: ENTRY_TYPE_AUDIT_PROXY.to_string(),
            audit_id: "tag-test-audit-001".to_string(),
            completed_at: now,
            module_id: module_id.clone(),
            purpose: "citation-grounding".to_string(),
            provider: "anthropic".to_string(),
            model: "claude-opus-4-7".to_string(),
            caller_request_id: None,
            prompt_tokens: 100,
            completion_tokens: 40,
            cost_usd: 0.00005,
            latency_ms: 350,
            status: "ok".to_string(),
            error_message: None,
        })
        .expect("append AuditProxyEntry");

    // AuditCaptureEntry
    ledger
        .append_capture_entry(&AuditCaptureEntry {
            entry_type: ENTRY_TYPE_AUDIT_CAPTURE.to_string(),
            audit_id: "tag-test-cap-001".to_string(),
            module_id: module_id.clone(),
            event_type: "anchor-event".to_string(),
            source: "project-data".to_string(),
            status: "ok".to_string(),
            event_at: now,
            captured_at: now,
            payload: json!({"batch_size": 64}),
            caller_request_id: None,
        })
        .expect("append AuditCaptureEntry");

    // Read back the JSONL stream and assert the entry_type field.
    let lines = read_ledger_lines(&ledger_dir);
    assert_eq!(lines.len(), 4, "ledger must have exactly 4 entries");

    let expected_tags = [
        ENTRY_TYPE_CHAT_COMPLETION,
        ENTRY_TYPE_AUDIT_PROXY_STUB,
        ENTRY_TYPE_AUDIT_PROXY,
        ENTRY_TYPE_AUDIT_CAPTURE,
    ];

    for (i, (line, expected_tag)) in lines.iter().zip(expected_tags.iter()).enumerate() {
        let actual_tag = line["entry_type"].as_str().unwrap_or("<field missing>");
        assert_eq!(
            actual_tag, *expected_tag,
            "entry {i}: entry_type must be {expected_tag:?}; got {actual_tag:?}"
        );
    }
}
