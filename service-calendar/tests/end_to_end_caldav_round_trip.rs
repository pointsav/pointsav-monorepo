// SPDX-License-Identifier: Apache-2.0
//
// A real CalDAV round trip against `service-calendar`'s full stack:
// this crate's own axum router, mounting `dav_server::DavHandler`
// against `WormCalendarFs`, backed by a real `PosixTileLedger` (not
// an in-memory stand-in) -- the same protocol machinery a real
// Thunderbird/Apple Calendar client would drive.
//
// Steps: PUT a real ICS VEVENT -> GET it back byte-identical ->
// PROPFIND (Depth: 1) the collection and confirm the event is listed
// -> DELETE it -> GET again and confirm 404.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

static TMPCTR: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn tmp_vocab() -> worm_ledger::retention::RetentionVocabulary {
    let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let path = std::env::temp_dir().join(format!(
        "service-calendar-e2e-vocab-{}-{}.csv",
        std::process::id(),
        n
    ));
    std::fs::write(
        &path,
        "class,duration_days,regime,notes\ncalendar-event,,internal calendar ledger,permanent\n",
    )
    .unwrap();
    worm_ledger::retention::load_retention_classes(path.to_str().unwrap())
}

fn tmp_ledger_dir() -> std::path::PathBuf {
    let n = TMPCTR.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "service-calendar-e2e-ledger-{}-{}",
        std::process::id(),
        n
    ))
}

const EVENT_ICS: &str = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//test//EN\r\nBEGIN:VEVENT\r\nUID:e2e-event-1\r\nDTSTART:20260101T120000Z\r\nDTEND:20260101T130000Z\r\nSUMMARY:New Year Meeting\r\nDESCRIPTION:Planning meeting for the new year\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";

#[tokio::test]
async fn put_get_propfind_delete_round_trip() {
    let module_id = "e2e-tenant";
    let ledger_root = tmp_ledger_dir();
    let ledger = worm_ledger::PosixTileLedger::open(&ledger_root, module_id, None::<&std::path::Path>)
        .expect("open posix ledger");
    let ledger_backing = Arc::new(service_calendar::LedgerBacking::new(
        Box::new(ledger),
        tmp_vocab(),
        module_id,
    ));
    let event_store = Arc::new(service_calendar::EventStore::new());
    let router = service_calendar::router(ledger_backing.clone(), event_store.clone());

    // ── Step 1: PUT the event ──
    let put_resp = router
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/e2e-event-1.ics")
                .header("content-type", "text/calendar")
                .body(Body::from(EVENT_ICS))
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(
        put_resp.status().is_success(),
        "PUT must succeed, got: {}",
        put_resp.status()
    );

    // ── Step 2: GET it back byte-identical ──
    let get_resp = router
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/e2e-event-1.ics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_resp.status(), StatusCode::OK);
    let get_body = axum::body::to_bytes(get_resp.into_body(), usize::MAX).await.unwrap();
    assert_eq!(
        get_body.as_ref(),
        EVENT_ICS.as_bytes(),
        "GET must return the exact bytes that were PUT, byte-for-byte"
    );

    // ── Step 3: PROPFIND the collection (Depth: 1) -- confirm the event is listed ──
    let propfind_body = r#"<?xml version="1.0" encoding="utf-8"?>
<D:propfind xmlns:D="DAV:">
  <D:prop><D:displayname/><D:getcontentlength/></D:prop>
</D:propfind>"#;
    let propfind_resp = router
        .clone()
        .oneshot(
            Request::builder()
                .method("PROPFIND")
                .uri("/")
                .header("depth", "1")
                .header("content-type", "application/xml")
                .body(Body::from(propfind_body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        propfind_resp.status(),
        StatusCode::MULTI_STATUS,
        "PROPFIND must return 207 Multi-Status"
    );
    let propfind_response_body = axum::body::to_bytes(propfind_resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let propfind_xml = String::from_utf8(propfind_response_body.to_vec()).unwrap();
    assert!(
        propfind_xml.contains("e2e-event-1.ics"),
        "PROPFIND response must list the event resource, got: {propfind_xml}"
    );

    // Also verifiable directly against the in-memory index (the
    // "queryable, no-UI-needed" proof the BRIEF asks for elsewhere in
    // this phase).
    let indexed = event_store.lookup_by_uid("e2e-event-1").unwrap();
    assert_eq!(indexed.summary, "New Year Meeting");
    assert_eq!(indexed.dtstart.to_rfc3339(), "2026-01-01T12:00:00+00:00");

    // ── Step 4: DELETE it ──
    let delete_resp = router
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/e2e-event-1.ics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(
        delete_resp.status().is_success(),
        "DELETE must succeed, got: {}",
        delete_resp.status()
    );

    // ── Step 5: GET again -- confirm 404 ──
    let get_after_delete_resp = router
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/e2e-event-1.ics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get_after_delete_resp.status(), StatusCode::NOT_FOUND);

    // ── Step 6: simulate a real restart -- fresh EventStore, replay
    // from the same on-disk ledger, confirm the tombstone survives
    // (the event stays deleted, not resurrected). ──
    let post_restart_ledger =
        worm_ledger::posix_tile_open(&ledger_root, module_id, None::<&std::path::Path>)
            .expect("reopen posix ledger for restart simulation");
    let post_restart_backing =
        service_calendar::LedgerBacking::new(post_restart_ledger, tmp_vocab(), module_id);
    let fresh_store = service_calendar::EventStore::new();
    let replayed = post_restart_backing.replay_into(&fresh_store).unwrap();
    assert_eq!(replayed, 2, "the PUT and the tombstone DELETE both replay");
    assert!(
        fresh_store.lookup_by_uid("e2e-event-1").is_none(),
        "the tombstone must survive a restart -- the event stays deleted"
    );
}
