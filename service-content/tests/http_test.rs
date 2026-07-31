// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Router-level integration tests for service-content's HTTP surface.
//!
//! Before this file, `service-content`'s entire HTTP layer had zero router-level
//! tests — only pure-logic unit tests existed (`truncate_at_char_boundary`,
//! `format_entity_block`, `scope_permits_request`). No test ever constructed the
//! real `Router` and drove an HTTP request through it, including `/v1/graph/mutate`
//! and `/v1/graph/context` — the two routes project-editorial's extraction pipeline
//! (`graph-committer.py`) actually depends on. See `BRIEF-datagraph-tenant-isolation.md`
//! Session 2/3 for the tenant-isolation fix these routes now enforce.
//!
//! Uses a real `LbugGraphStore` over a `std::env::temp_dir()`-based directory,
//! following the existing convention from `graph.rs`'s and `pipeline_tests.rs`'s
//! own test modules — no mock `GraphStore` exists anywhere in this codebase, and
//! none is introduced here.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::{json, Value};
use service_content::graph::GraphEntity;
use service_content::http::{router, HttpState};
use service_content::pairing::PairingKeypair;
use service_content::test_helpers::{
    make_capability_token, register_test_peer, temp_graph_store, test_http_state, test_peer_keypair,
};
use std::sync::Arc;
use tower::ServiceExt as _;

// ── shared test setup ───────────────────────────────────────────────────────────

/// Build a fresh `(Router, HttpState, base_dir)` for a test. Each call gets a
/// distinct temp directory (see `temp_graph_store`'s doc comment for why this
/// isn't the `tempfile` crate); directories are not auto-cleaned, matching
/// this crate's existing test convention (`pipeline_tests.rs`).
fn test_app() -> (axum::Router, Arc<HttpState>, std::path::PathBuf) {
    let (store, dir) = temp_graph_store();
    let corpus_dir = dir.join("corpus");
    std::fs::create_dir_all(&corpus_dir).expect("create corpus dir");
    let state = test_http_state(store, dir.to_str().unwrap(), corpus_dir.to_str().unwrap());
    let app = router(state.clone());
    (app, state, dir)
}

fn entity(name: &str, module_id: &str) -> GraphEntity {
    GraphEntity {
        entity_name: name.to_string(),
        classification: "Company".to_string(),
        role_vector: None,
        location_vector: None,
        contact_vector: None,
        module_id: module_id.to_string(),
        confidence: 0.95,
        source_doc: None,
    }
}

async fn body_json(resp: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(resp.into_body(), 10 * 1024 * 1024)
        .await
        .expect("read body");
    serde_json::from_slice(&bytes).expect("parse JSON body")
}

// ===========================================================================
// Priority path — GET /v1/graph/context, POST /v1/graph/mutate
// (project-editorial's `graph-committer.py` depends on POST /v1/graph/mutate
// specifically; it never calls graph_context directly — but both share the
// tenant-isolation contract, so both get full coverage here.)
// ===========================================================================

#[tokio::test]
async fn graph_mutate_happy_path_upserts_and_returns_count() {
    let (app, state, _dir) = test_app();

    let body = json!({
        "module_id": "pointsav",
        "entities": [entity("service-content", "pointsav")]
    });
    let req = Request::builder()
        .method("POST")
        .uri("/v1/graph/mutate")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["upserted"], json!(1));

    // Verify it actually landed in the store, not just a happy response.
    let stored = state.graph.list_entities("pointsav").expect("list");
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].entity_name, "service-content");
}

/// Regression test for a real integration gap found 2026-07-18: project-editorial's
/// `graph-committer.py` sends entities WITHOUT a per-entity `module_id` field (only the
/// top-level `MutateRequest.module_id`) — this is the exact real payload shape their
/// tool sends, reproduced verbatim rather than a synthetic minimal case. Before the fix,
/// this 422'd ("missing field `module_id`") on every one of their 25 real proposals.
#[tokio::test]
async fn graph_mutate_backfills_entity_module_id_from_request_top_level() {
    let (app, state, _dir) = test_app();

    // No "module_id" key on the entity object at all — matches graph-committer.py's
    // post_mutate() body construction exactly (entity_name/classification/role_vector/
    // confidence only).
    let body = json!({
        "module_id": "pointsav",
        "entities": [{
            "entity_name": "service-content",
            "classification": "architecture-reference",
            "role_vector": "Ring 2 taxonomy ledger and knowledge graph service",
            "confidence": 1.0
        }]
    });
    let req = Request::builder()
        .method("POST")
        .uri("/v1/graph/mutate")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "must not 422 on a missing per-entity module_id"
    );
    let body = body_json(resp).await;
    assert_eq!(body["upserted"], json!(1));

    // The critical assertion: the stored node's module_id property must be backfilled
    // to "pointsav", not left empty — an empty module_id would silently corrupt tenant
    // scoping for every future read of this entity.
    let stored = state.graph.list_entities("pointsav").expect("list");
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].module_id, "pointsav");
}

#[tokio::test]
async fn graph_mutate_malformed_body_returns_400() {
    let (app, _state, _dir) = test_app();
    let req = Request::builder()
        .method("POST")
        .uri("/v1/graph/mutate")
        .header("content-type", "application/json")
        .body(Body::from("{not valid json"))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    // axum's Json extractor rejects malformed bodies before the handler runs.
    // Verified against real behavior: 400, not 422 (this handler's Json<T>
    // extractor doesn't use a custom rejection remapping to 422).
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn graph_mutate_empty_entities_is_a_valid_noop() {
    let (app, _state, _dir) = test_app();
    let body = json!({"module_id": "pointsav", "entities": []});
    let req = Request::builder()
        .method("POST")
        .uri("/v1/graph/mutate")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["upserted"], json!(0));
}

#[tokio::test]
async fn graph_context_happy_path_returns_only_matching_module() {
    let (app, state, _dir) = test_app();
    state
        .graph
        .upsert_entities(
            "pointsav",
            &[entity("Woodfine Capital Projects", "pointsav")],
        )
        .expect("seed pointsav");
    state
        .graph
        .upsert_entities(
            "woodfine",
            &[entity("Woodfine Capital Projects", "woodfine")],
        )
        .expect("seed woodfine");

    let req = Request::builder()
        .method("GET")
        .uri("/v1/graph/context?q=Woodfine&module_id=pointsav&limit=10")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    let rows = body.as_array().expect("array response");
    assert!(
        !rows.is_empty(),
        "expected at least one entity for pointsav"
    );
    assert!(
        rows.iter().all(|r| r["module_id"] == "pointsav"),
        "graph_context must not leak rows from other tenants: {rows:?}"
    );
}

#[tokio::test]
async fn graph_context_missing_module_id_returns_400() {
    let (app, _state, _dir) = test_app();
    // `module_id` has no #[serde(default)] on ContextQuery — axum's Query
    // extractor itself must reject this before the handler runs.
    let req = Request::builder()
        .method("GET")
        .uri("/v1/graph/context?q=Woodfine")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn graph_context_unknown_tenant_returns_empty_not_error() {
    let (app, state, _dir) = test_app();
    state
        .graph
        .upsert_entities(
            "pointsav",
            &[entity("Woodfine Capital Projects", "pointsav")],
        )
        .expect("seed");

    let req = Request::builder()
        .method("GET")
        .uri("/v1/graph/context?q=Woodfine&module_id=nonexistent-tenant&limit=10")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body.as_array().unwrap().len(), 0);
}

// ===========================================================================
// capability_gate — end-to-end via real HTTP requests, not just the pure
// scope_permits_request helper (already unit-tested in src/http.rs).
// ===========================================================================

#[tokio::test]
async fn capability_gate_absent_header_passes_through_unchanged() {
    // The local Doorman's calls never send X-Foundry-Capability — must keep working.
    let (app, _state, _dir) = test_app();
    let req = Request::builder()
        .method("GET")
        .uri("/v1/graph/context?q=x&module_id=pointsav&limit=5")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn capability_gate_valid_scoped_token_permits_matching_target() {
    let (app, state, _dir) = test_app();
    state
        .graph
        .upsert_entities(
            "pointsav",
            &[entity("Woodfine Capital Projects", "pointsav")],
        )
        .expect("seed");

    let peer_key = test_peer_keypair(1);
    register_test_peer(
        &state,
        &peer_key,
        "project-editorial",
        "USER",
        vec!["pointsav".to_string()],
    );
    let token = make_capability_token(
        &peer_key,
        "project-editorial",
        "USER",
        vec!["pointsav".to_string()],
        chrono::Utc::now() + chrono::Duration::hours(1),
    );

    let req = Request::builder()
        .method("GET")
        .uri("/v1/graph/context?q=Woodfine&module_id=pointsav&limit=10")
        .header("X-Foundry-Capability", token)
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn capability_gate_scope_mismatch_rejected_403() {
    let (app, state, _dir) = test_app();
    let peer_key = test_peer_keypair(2);
    register_test_peer(
        &state,
        &peer_key,
        "project-editorial",
        "USER",
        vec!["woodfine".to_string()],
    );
    let token = make_capability_token(
        &peer_key,
        "project-editorial",
        "USER",
        vec!["woodfine".to_string()],
        chrono::Utc::now() + chrono::Duration::hours(1),
    );

    // Token scoped to "woodfine" but targeting "pointsav" — must be rejected.
    let req = Request::builder()
        .method("GET")
        .uri("/v1/graph/context?q=x&module_id=pointsav&limit=10")
        .header("X-Foundry-Capability", token)
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn capability_gate_wildcard_admin_permits_read() {
    let (app, state, _dir) = test_app();
    state
        .graph
        .upsert_entities("any-tenant", &[entity("Anything", "any-tenant")])
        .expect("seed");

    let peer_key = test_peer_keypair(3);
    register_test_peer(
        &state,
        &peer_key,
        "operator-console",
        "ADMIN",
        vec!["*".to_string()],
    );
    let token = make_capability_token(
        &peer_key,
        "operator-console",
        "ADMIN",
        vec!["*".to_string()],
        chrono::Utc::now() + chrono::Duration::hours(1),
    );

    let req = Request::builder()
        .method("GET")
        .uri("/v1/graph/context?q=Anything&module_id=any-tenant&limit=10")
        .header("X-Foundry-Capability", token)
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn capability_gate_wildcard_non_admin_rejected_403() {
    let (app, state, _dir) = test_app();
    let peer_key = test_peer_keypair(4);
    register_test_peer(
        &state,
        &peer_key,
        "project-editorial",
        "USER",
        vec!["*".to_string()],
    );
    let token = make_capability_token(
        &peer_key,
        "project-editorial",
        "USER", // not ADMIN
        vec!["*".to_string()],
        chrono::Utc::now() + chrono::Duration::hours(1),
    );

    let req = Request::builder()
        .method("GET")
        .uri("/v1/graph/context?q=x&module_id=anything&limit=10")
        .header("X-Foundry-Capability", token)
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn capability_gate_wildcard_admin_rejected_on_mutate_read_only_override() {
    // Locked decision (BRIEF-datagraph-tenant-isolation.md): the operator override
    // is read-only even for the operator — a wildcard token must never be honored
    // on /v1/graph/mutate, full stop, regardless of role.
    let (app, state, _dir) = test_app();
    let peer_key = test_peer_keypair(5);
    register_test_peer(
        &state,
        &peer_key,
        "operator-console",
        "ADMIN",
        vec!["*".to_string()],
    );
    let token = make_capability_token(
        &peer_key,
        "operator-console",
        "ADMIN",
        vec!["*".to_string()],
        chrono::Utc::now() + chrono::Duration::hours(1),
    );

    let body = json!({"module_id": "any-tenant", "entities": [entity("X", "any-tenant")]});
    let req = Request::builder()
        .method("POST")
        .uri("/v1/graph/mutate")
        .header("content-type", "application/json")
        .header("X-Foundry-Capability", token)
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn capability_gate_exact_scope_still_permits_mutate() {
    // Non-wildcard scopes are unaffected by the mutate-route restriction.
    let (app, state, _dir) = test_app();
    let peer_key = test_peer_keypair(6);
    register_test_peer(
        &state,
        &peer_key,
        "project-editorial",
        "USER",
        vec!["pointsav".to_string()],
    );
    let token = make_capability_token(
        &peer_key,
        "project-editorial",
        "USER",
        vec!["pointsav".to_string()],
        chrono::Utc::now() + chrono::Duration::hours(1),
    );

    let body = json!({"module_id": "pointsav", "entities": [entity("X", "pointsav")]});
    let req = Request::builder()
        .method("POST")
        .uri("/v1/graph/mutate")
        .header("content-type", "application/json")
        .header("X-Foundry-Capability", token)
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn capability_gate_unregistered_peer_rejected_403() {
    let (app, _state, _dir) = test_app();
    let peer_key = test_peer_keypair(7);
    // Deliberately never registered via register_test_peer.
    let token = make_capability_token(
        &peer_key,
        "unknown-peer",
        "USER",
        vec!["pointsav".to_string()],
        chrono::Utc::now() + chrono::Duration::hours(1),
    );
    let req = Request::builder()
        .method("GET")
        .uri("/v1/graph/context?q=x&module_id=pointsav&limit=10")
        .header("X-Foundry-Capability", token)
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn capability_gate_expired_token_rejected_401() {
    let (app, state, _dir) = test_app();
    let peer_key = test_peer_keypair(8);
    register_test_peer(
        &state,
        &peer_key,
        "project-editorial",
        "USER",
        vec!["pointsav".to_string()],
    );
    let token = make_capability_token(
        &peer_key,
        "project-editorial",
        "USER",
        vec!["pointsav".to_string()],
        chrono::Utc::now() - chrono::Duration::hours(1), // already expired
    );
    let req = Request::builder()
        .method("GET")
        .uri("/v1/graph/context?q=x&module_id=pointsav&limit=10")
        .header("X-Foundry-Capability", token)
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn capability_gate_nonce_replay_rejected_409() {
    let (app, state, _dir) = test_app();
    let peer_key = test_peer_keypair(9);
    register_test_peer(
        &state,
        &peer_key,
        "project-editorial",
        "USER",
        vec!["pointsav".to_string()],
    );
    let token = make_capability_token(
        &peer_key,
        "project-editorial",
        "USER",
        vec!["pointsav".to_string()],
        chrono::Utc::now() + chrono::Duration::hours(1),
    );

    let req1 = Request::builder()
        .method("GET")
        .uri("/v1/graph/context?q=x&module_id=pointsav&limit=10")
        .header("X-Foundry-Capability", token.clone())
        .body(Body::empty())
        .unwrap();
    let resp1 = app.clone().oneshot(req1).await.unwrap();
    assert_eq!(resp1.status(), StatusCode::OK);

    // Replay the identical token (same nonce) — must be rejected.
    let req2 = Request::builder()
        .method("GET")
        .uri("/v1/graph/context?q=x&module_id=pointsav&limit=10")
        .header("X-Foundry-Capability", token)
        .body(Body::empty())
        .unwrap();
    let resp2 = app.oneshot(req2).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::CONFLICT);
}

// ===========================================================================
// Concurrency safety — the untested claim in service-content/src/lib.rs
// ("N drain workers are safe without additional locking on the store itself").
// No test anywhere in the codebase previously exercised concurrent writes.
// ===========================================================================

#[tokio::test]
async fn concurrent_writers_different_tenants_no_cross_tenant_bleed() {
    let (store, _dir) = temp_graph_store();
    let n_writers = 8;
    let mut handles = Vec::new();

    for i in 0..n_writers {
        let store = store.clone();
        handles.push(tokio::task::spawn_blocking(move || {
            let module_id = format!("tenant-{i}");
            let entities: Vec<GraphEntity> = (0..10)
                .map(|j| entity(&format!("Entity-{i}-{j}"), &module_id))
                .collect();
            store
                .upsert_entities(&module_id, &entities)
                .unwrap_or_else(|e| panic!("writer {i} failed: {e}"))
        }));
    }

    let mut total_upserted = 0usize;
    for h in handles {
        total_upserted += h.await.expect("writer task panicked");
    }
    assert_eq!(
        total_upserted,
        n_writers * 10,
        "every concurrent writer's entities must be counted, none lost"
    );

    // Verify per-tenant isolation held under real concurrency, not just
    // sequentially (Session 2/3 already covers the sequential case).
    for i in 0..n_writers {
        let module_id = format!("tenant-{i}");
        let stored = store.list_entities(&module_id).expect("list");
        assert_eq!(
            stored.len(),
            10,
            "tenant-{i} must have exactly its own 10 entities, no bleed from other writers"
        );
        assert!(
            stored.iter().all(|e| e.module_id == module_id),
            "tenant-{i}'s stored entities must all carry its own module_id"
        );
    }

    let grand_total = store.count_all().expect("count_all");
    assert_eq!(
        grand_total,
        n_writers * 10,
        "count_all must reflect every writer's contribution exactly once, no duplication/loss"
    );
}

#[tokio::test]
async fn concurrent_writers_same_tenant_no_lost_updates() {
    // A harder case: multiple sources extracting concurrently for the SAME
    // archive (the real project-editorial + other-archive scenario) — every
    // writer's entities must survive, not just the last writer's.
    let (store, _dir) = temp_graph_store();
    let n_writers = 8;
    let module_id = "pointsav";
    let mut handles = Vec::new();

    for i in 0..n_writers {
        let store = store.clone();
        let module_id = module_id.to_string();
        handles.push(tokio::task::spawn_blocking(move || {
            // Distinct entity names per writer (real extraction from different
            // documents produces distinct entities, not the same one repeated) —
            // this isolates "are writes lost under concurrency" from ER-merge
            // behavior, which is a separate, already-tested concern.
            let e = entity(&format!("Concurrent-Entity-{i}"), &module_id);
            store
                .upsert_entities(&module_id, std::slice::from_ref(&e))
                .unwrap_or_else(|e| panic!("writer {i} failed: {e}"))
        }));
    }

    for h in handles {
        h.await.expect("writer task panicked");
    }

    let stored = store.list_entities(module_id).expect("list");
    assert_eq!(
        stored.len(),
        n_writers,
        "every concurrent writer's distinct entity must be present — none lost to a race"
    );
    for i in 0..n_writers {
        assert!(
            stored
                .iter()
                .any(|e| e.entity_name == format!("Concurrent-Entity-{i}")),
            "writer {i}'s entity must be present"
        );
    }
}

// ===========================================================================
// Remaining routes — smoke-level: route exists, correct status, correct shape.
// ===========================================================================

#[tokio::test]
async fn healthz_returns_ok_with_entity_count() {
    let (app, state, _dir) = test_app();
    state
        .graph
        .upsert_entities("pointsav", &[entity("X", "pointsav")])
        .expect("seed");
    let req = Request::builder()
        .uri("/healthz")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["status"], "ok");
    assert_eq!(body["entity_count"], json!(1));
}

#[tokio::test]
async fn graph_edges_returns_induced_subgraph() {
    let (app, state, _dir) = test_app();
    state
        .graph
        .upsert_entities(
            "pointsav",
            &[entity("A", "pointsav"), entity("B", "pointsav")],
        )
        .expect("seed");
    let req = Request::builder()
        .uri("/v1/graph/edges?module_id=pointsav&entities=A,B")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert!(body.is_array());
}

#[tokio::test]
async fn graph_delta_returns_entities_since_timestamp() {
    let (app, state, _dir) = test_app();
    state
        .graph
        .upsert_entities("pointsav", &[entity("X", "pointsav")])
        .expect("seed");
    let req = Request::builder()
        .uri("/v1/graph/delta?module_id=pointsav&since=2020-01-01T00:00:00Z")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert!(body.is_array());
}

#[tokio::test]
async fn graph_cleanup_defaults_to_dry_run() {
    let (app, state, _dir) = test_app();
    state
        .graph
        .upsert_entities("pointsav", &[entity("X", "pointsav")])
        .expect("seed");
    // dry_run omitted — must default to true (safe), per CleanupQuery's own default.
    let req = Request::builder()
        .uri("/v1/graph/cleanup?module_id=pointsav")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Entity must still be present — a dry run must never delete.
    let stored = state.graph.list_entities("pointsav").expect("list");
    assert_eq!(stored.len(), 1);
}

#[tokio::test]
async fn graph_enrich_defaults_to_dry_run_no_network_call() {
    let (app, state, _dir) = test_app();
    let mut e = entity("Jennifer Woodfine", "pointsav");
    e.classification = "Person".to_string();
    state.graph.upsert_entities("pointsav", &[e]).expect("seed");

    // dry_run omitted — must default to true, so no live Doorman call happens
    // (state.doorman_endpoint is empty in this test, which would fail loudly
    // if a live call were attempted).
    let req = Request::builder()
        .method("POST")
        .uri("/v1/graph/enrich?module_id=pointsav")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["dry_run"], json!(true));
    assert_eq!(body["enriched"], json!(0));
}

#[tokio::test]
async fn draft_generate_doorman_unreachable_returns_502() {
    // Documents the current real behavior with an unconfigured Doorman
    // endpoint (empty string in test state) — CLAUDE.md's stale "503 pre-D4"
    // claim doesn't match; the actual mapping is 502 Bad Gateway via
    // reqwest's connection failure, not a deliberate unconfigured-check.
    let (app, state, _dir) = test_app();
    state
        .graph
        .upsert_entities("pointsav", &[entity("X", "pointsav")])
        .expect("seed");
    let body = json!({"module_id": "pointsav", "query_hint": "test"});
    let req = Request::builder()
        .method("POST")
        .uri("/v1/draft/generate")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
}

#[tokio::test]
async fn ingest_document_writes_corpus_file_returns_202() {
    let (app, _state, dir) = test_app();
    let body = json!({"text": "Some real document text.", "module_id": "pointsav"});
    let req = Request::builder()
        .method("POST")
        .uri("/v1/ingest")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::ACCEPTED);
    let body = body_json(resp).await;
    assert_eq!(body["queued"], json!(true));

    let corpus_dir = dir.join("corpus");
    let files: Vec<_> = std::fs::read_dir(&corpus_dir)
        .expect("read corpus dir")
        .collect();
    assert_eq!(
        files.len(),
        1,
        "exactly one CORPUS_*.json file must be written"
    );
}

#[tokio::test]
async fn ingest_document_empty_text_returns_400() {
    let (app, _state, _dir) = test_app();
    let body = json!({"text": "   ", "module_id": "pointsav"});
    let req = Request::builder()
        .method("POST")
        .uri("/v1/ingest")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn pair_peer_new_pairing_returns_paired() {
    let (app, _state, dir) = test_app();
    // Separate keypair dir from the server's own — this is the CALLER's key.
    let caller_dir = dir.join("caller-key");
    std::fs::create_dir_all(&caller_dir).unwrap();
    let caller_kp = PairingKeypair::load_or_generate(caller_dir.to_str().unwrap()).unwrap();
    let token = caller_kp.issue_token("USER", vec!["pointsav".to_string()], "test-caller-node");

    let body = json!({
        "token": token,
        "public_key": caller_kp.verifying_key_b64,
        "node_label": "test-caller-node",
    });
    let req = Request::builder()
        .method("POST")
        .uri("/v1/pair")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert_eq!(body["status"], "paired");
}

#[tokio::test]
async fn pair_peer_repeat_pairing_returns_already_paired() {
    let (app, _state, dir) = test_app();
    let caller_dir = dir.join("caller-key");
    std::fs::create_dir_all(&caller_dir).unwrap();
    let caller_kp = PairingKeypair::load_or_generate(caller_dir.to_str().unwrap()).unwrap();
    let token = caller_kp.issue_token("USER", vec!["pointsav".to_string()], "test-caller-node");
    let body = json!({
        "token": token,
        "public_key": caller_kp.verifying_key_b64,
        "node_label": "test-caller-node",
    });

    let req1 = Request::builder()
        .method("POST")
        .uri("/v1/pair")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp1 = app.clone().oneshot(req1).await.unwrap();
    assert_eq!(resp1.status(), StatusCode::OK);

    let req2 = Request::builder()
        .method("POST")
        .uri("/v1/pair")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp2 = app.oneshot(req2).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::OK);
    let body2 = body_json(resp2).await;
    assert_eq!(body2["status"], "already_paired");
}

#[tokio::test]
async fn issue_pair_token_returns_signed_token() {
    let (app, _state, _dir) = test_app();
    let req = Request::builder()
        .uri("/v1/pair/token?role=USER&node_label=test-node&archive_scope=pointsav")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    assert!(
        body["token"].as_str().unwrap().contains('.'),
        "token must be payload.sig format"
    );
    assert!(!body["public_key"].as_str().unwrap().is_empty());
}

#[tokio::test]
async fn list_pairs_reflects_registered_peers() {
    let (app, state, _dir) = test_app();
    let peer_key = test_peer_keypair(10);
    register_test_peer(
        &state,
        &peer_key,
        "project-editorial",
        "USER",
        vec!["pointsav".to_string()],
    );
    let req = Request::builder()
        .uri("/v1/pairs")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_json(resp).await;
    let list = body.as_array().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0]["issuer"], "project-editorial");
}
