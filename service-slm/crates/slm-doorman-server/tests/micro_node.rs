// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Micro-node integration tests — validates service-slm behavior on the
//! $7/mo e2-micro fleet node (DOCTRINE.md claims #49, #54).
//!
//! Tests verify:
//!   - `/readyz` is honest: `tier_a: false`, `node_class: "micro"`, `ai_available: false`
//!   - `/v1/chat/completions` returns 503 cleanly — no model-load attempt
//!   - `/healthz` always 200 (liveness independent of node class)
//!   - `SLM_FORCE_BROKER_MODE` simulation: Hardware node with broker override
//!     reports the correct `tier_a_reason`
//!
//! Tests construct `AppState` directly (no env-var mutation) so they are
//! safe under Rust's parallel test runner.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::Value;
use slm_doorman::{BriefCache, Doorman, DoormanConfig, FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST};
use slm_doorman_server::http::{router, AppState};
use slm_doorman_server::test_helpers::{temp_ledger, temp_queue_config};
use tower::ServiceExt;

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Build a Micro-class `AppState`: no local client, no Yo-Yo, no External.
/// `node_class` and `tier_a_reason` are set to what `build_doorman()` produces
/// on a real $7/mo e2-micro node.
fn micro_state() -> Arc<AppState> {
    Arc::new(AppState {
        doorman: Doorman::new(DoormanConfig::default(), temp_ledger()),
        apprenticeship: None,
        brief_cache: Arc::new(BriefCache::default()),
        verdict_dispatcher: None,
        audit_proxy_client: None,
        audit_proxy_purpose_allowlist: FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST,
        audit_tenant_concurrency: Arc::new(Mutex::new(HashMap::new())),
        audit_tenant_concurrency_cap: 100,
        queue_config: temp_queue_config(),
        service_content_endpoint: String::new(),
        node_class: "micro",
        tier_a_reason: "micro-node-class",
    })
}

async fn body_json(resp: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(resp.into_body(), 64 * 1024)
        .await
        .expect("read response body");
    serde_json::from_slice(&bytes).expect("response body is JSON")
}

// ── Tests ─────────────────────────────────────────────────────────────────────

/// /readyz must truthfully report Micro class and Tier A unavailability.
/// A Micro node must NEVER attempt a model-load; this probe verifies the
/// Doorman reports accurately from the node-class probe, not from a
/// model-load attempt (DOCTRINE.md claim #54).
#[tokio::test]
async fn micro_readyz_reports_node_class_tier_a_unavailable() {
    let resp = router(micro_state())
        .oneshot(Request::get("/readyz").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let json = body_json(resp).await;

    assert_eq!(json["node_class"], "micro");
    assert_eq!(json["tier_a"], false, "Tier A must be unavailable on Micro");
    assert_eq!(json["tier_a_reason"], "micro-node-class");
    assert_eq!(
        json["ai_available"], false,
        "no AI tiers configured on bare Micro"
    );
    assert_eq!(json["has_local"], false);
    assert_eq!(json["has_yoyo"], false);
    assert_eq!(json["has_external"], false);
    assert_eq!(json["ready"], true, "Doorman is ready even without AI");
}

/// An AI request on a Micro node must return 503 SERVICE_UNAVAILABLE —
/// clean rejection with no model-load attempt.
#[tokio::test]
async fn micro_chat_completions_returns_503_no_model_load() {
    let body = serde_json::json!({
        "model": "olmo-1b",
        "messages": [{"role": "user", "content": "hello"}]
    });
    let resp = router(micro_state())
        .oneshot(
            Request::post("/v1/chat/completions")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        StatusCode::SERVICE_UNAVAILABLE,
        "Micro node must return 503 for AI requests — TierUnavailable(Local)"
    );
}

/// /healthz is the liveness probe — must return 200 regardless of node class.
/// A $7/mo node that can't do AI is still a live, functioning Totebox.
#[tokio::test]
async fn micro_healthz_always_200() {
    let resp = router(micro_state())
        .oneshot(Request::get("/healthz").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

/// Simulate `SLM_FORCE_BROKER_MODE=true` on a Hardware node.
/// build_doorman() sets local=None and tier_a_reason="force-broker-mode".
/// /readyz must surface the override reason so operators can diagnose it.
#[tokio::test]
async fn force_broker_mode_readyz_surfaces_override_reason() {
    let state = Arc::new(AppState {
        doorman: Doorman::new(DoormanConfig::default(), temp_ledger()),
        apprenticeship: None,
        brief_cache: Arc::new(BriefCache::default()),
        verdict_dispatcher: None,
        audit_proxy_client: None,
        audit_proxy_purpose_allowlist: FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST,
        audit_tenant_concurrency: Arc::new(Mutex::new(HashMap::new())),
        audit_tenant_concurrency_cap: 100,
        queue_config: temp_queue_config(),
        service_content_endpoint: String::new(),
        node_class: "hardware",             // detected as Hardware
        tier_a_reason: "force-broker-mode", // but SLM_FORCE_BROKER_MODE=true overrode it
    });

    let resp = router(state)
        .oneshot(Request::get("/readyz").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let json = body_json(resp).await;

    assert_eq!(json["node_class"], "hardware");
    assert_eq!(
        json["tier_a"], false,
        "force-broker disables Tier A even on Hardware"
    );
    assert_eq!(json["tier_a_reason"], "force-broker-mode");
    assert_eq!(json["ai_available"], false);
}

/// The readyz response for a Hardware node with Tier A available must show
/// tier_a: true — sanity-check that the Micro path doesn't accidentally
/// disable Tier A on capable nodes.
#[tokio::test]
async fn hardware_readyz_shows_tier_a_available() {
    use slm_doorman::tier::{LocalTierClient, LocalTierConfig};

    let local = LocalTierClient::new(LocalTierConfig {
        endpoint: "http://127.0.0.1:8080".to_string(),
        default_model: "olmo-2-1b".to_string(),
    });
    let doorman = Doorman::new(
        DoormanConfig {
            local: Some(local),
            yoyo: HashMap::new(),
            external: None,
            lark_validator: None,
            graph_context_client: None,
            tier_a_first: false,
            daily_yoyo_cap_usd: None,
            cost_ledger: None,
        },
        temp_ledger(),
    );
    let state = Arc::new(AppState {
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
        node_class: "hardware",
        tier_a_reason: "available",
    });

    let resp = router(state)
        .oneshot(Request::get("/readyz").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let json = body_json(resp).await;

    assert_eq!(json["node_class"], "hardware");
    assert_eq!(json["tier_a"], true);
    assert_eq!(json["tier_a_reason"], "available");
    assert_eq!(json["ai_available"], true);
    assert_eq!(json["has_local"], true);
}
