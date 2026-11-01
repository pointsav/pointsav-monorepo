// SPDX-License-Identifier: Apache-2.0
//
// End-to-end pipeline test -- two businesses at the same place (a
// real cross-service reference to a service-places::Place id, not a
// duplicated address), verifying list_by_place, the Envelope's
// `location` entity role, and survival across a real restart.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

async fn body_json(body: Body) -> serde_json::Value {
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

static TMPCTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn tmp_vocab() -> worm_ledger::retention::RetentionVocabulary {
    let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let path = std::env::temp_dir().join(format!(
        "service-business-e2e-vocab-{}-{}.csv",
        std::process::id(),
        n
    ));
    std::fs::write(
        &path,
        "class,duration_days,regime,notes\nbusiness-record,,internal identity ledger,permanent\n",
    )
    .unwrap();
    worm_ledger::retention::load_retention_classes(path.to_str().unwrap())
}

fn tmp_ledger_dir(label: &str) -> std::path::PathBuf {
    let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "service-business-e2e-ledger-{label}-{}-{}",
        std::process::id(),
        n
    ))
}

#[tokio::test]
async fn two_businesses_at_the_same_place_both_list_and_survive_restart() {
    let module_id = "e2e-tenant";
    let mall_place_id = Uuid::new_v4(); // stands in for a real service-places::Place id

    let ledger_root = tmp_ledger_dir("posix");
    let ledger = worm_ledger::PosixTileLedger::open(&ledger_root, module_id, None::<&std::path::Path>)
        .expect("open posix ledger");
    let ledger_backing = Arc::new(service_business::LedgerBacking::new(
        Box::new(ledger),
        tmp_vocab(),
        module_id,
    ));
    let business_store = Arc::new(service_business::BusinessStore::new());
    let state = service_business::AppState {
        module_id: module_id.to_string(),
        ledger_backing: ledger_backing.clone(),
        business_store: business_store.clone(),
    };
    let router = service_business::router(state);

    async fn append(router: &axum::Router, name: &str, place_id: Uuid) -> String {
        let req = json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": "business.append",
                "arguments": { "name": name, "category": "retail", "place_id": place_id.to_string() }
            },
            "id": 1
        });
        let resp = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/mcp")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&req).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp.into_body()).await;
        assert!(body["error"].is_null(), "business.append must not error: {body}");
        body["result"]["business_id"].as_str().unwrap().to_string()
    }

    let cafe_id = append(&router, "Acme Cafe", mall_place_id).await;
    let bookstore_id = append(&router, "Bookstore", mall_place_id).await;
    assert_ne!(cafe_id, bookstore_id);

    // ── verify the Envelope carries a `location` entity pointing at
    // the Place id, not a duplicated address ──
    let reread = worm_ledger::posix_tile_open(&ledger_root, module_id, None::<&std::path::Path>)
        .expect("reopen posix ledger");
    let entries = reread.read_since(0).expect("read_since must succeed");
    assert_eq!(entries.len(), 2);
    let first_envelope: envelope::Envelope = serde_json::from_value(entries[0].payload.clone()).unwrap();
    assert!(first_envelope
        .entities
        .iter()
        .any(|e| e.role == "location" && e.entity_id == mall_place_id.to_string()));
    assert!(
        first_envelope.payload.body.get("address").is_none(),
        "the business record must reference the place by id, not duplicate an address field"
    );

    // ── business.list_by_place over HTTP returns both tenants ──
    let list_req = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": { "name": "business.list_by_place", "arguments": { "place_id": mall_place_id.to_string() } },
        "id": 3
    });
    let list_resp = router
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&list_req).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    let list_body = body_json(list_resp.into_body()).await;
    let businesses = list_body["result"]["businesses"].as_array().unwrap();
    assert_eq!(businesses.len(), 2);

    // ── real restart: fresh BusinessStore replayed from disk keeps both ──
    let post_restart_ledger =
        worm_ledger::posix_tile_open(&ledger_root, module_id, None::<&std::path::Path>)
            .expect("reopen posix ledger for restart simulation");
    let post_restart_backing =
        service_business::LedgerBacking::new(post_restart_ledger, tmp_vocab(), module_id);
    let fresh_store = service_business::BusinessStore::new();
    let replayed = post_restart_backing.replay_into(&fresh_store).expect("replay must succeed");
    assert_eq!(replayed, 2);
    assert_eq!(fresh_store.list_by_place(mall_place_id).len(), 2);
}
