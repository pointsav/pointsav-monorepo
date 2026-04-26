//! Integration tests for Phase 3 Step 3.3 — Atom + JSON Feed syndication.
//!
//! Verifies that:
//! - `GET /feed.atom` returns 200 with `application/atom+xml` Content-Type
//!   and well-formed XML containing an Atom `<feed>` element.
//! - `GET /feed.json` returns 200, parseable JSON, and the `version` field
//!   matches the JSON Feed 1.1 identifier.
//! - Both feeds list TOPIC entries that were seeded into the fixture dir.

use app_mediakit_knowledge::search;
use app_mediakit_knowledge::server::{router, AppState};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use std::sync::Arc;
use tower::ServiceExt;

/// Shared fixture: two TOPIC files, no bilingual siblings.
async fn fixture_state() -> (AppState, tempfile::TempDir, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = tempfile::tempdir().unwrap();

    tokio::fs::write(
        dir.path().join("topic-alpha.md"),
        "---\ntitle: \"Alpha\"\nslug: topic-alpha\n---\nAlpha is the first topic.\n",
    )
    .await
    .unwrap();

    tokio::fs::write(
        dir.path().join("topic-beta.md"),
        "---\ntitle: \"Beta\"\nslug: topic-beta\n---\nBeta is the second topic.\n",
    )
    .await
    .unwrap();

    // Bilingual sibling — must NOT appear in feeds.
    tokio::fs::write(
        dir.path().join("topic-alpha.es.md"),
        "---\ntitle: \"Alfa\"\nslug: topic-alpha.es\n---\nAlfa es el primer tema.\n",
    )
    .await
    .unwrap();

    let index = search::build_index(dir.path(), state_dir.path())
        .await
        .unwrap();

    let state = AppState {
        content_dir: dir.path().to_path_buf(),
        citations_yaml: std::path::PathBuf::from("/nonexistent/citations.yaml"),
        search: Arc::new(index),
    };

    (state, dir, state_dir)
}

// ─── Atom feed ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn atom_feed_returns_200_with_xml_content_type() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/feed.atom")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let ct = resp
        .headers()
        .get("content-type")
        .expect("content-type header should be present")
        .to_str()
        .unwrap();
    assert!(
        ct.contains("application/atom+xml"),
        "content-type should be application/atom+xml: {ct}"
    );
}

#[tokio::test]
async fn atom_feed_body_is_parseable_xml_with_feed_element() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/feed.atom")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let xml = std::str::from_utf8(&bytes).unwrap();

    // Must contain the Atom `<feed` element and XML declaration.
    assert!(xml.contains("<feed"), "Atom body should contain <feed: {xml}");
    assert!(
        xml.contains("PointSav Knowledge"),
        "Atom body should contain feed title: {xml}"
    );
}

#[tokio::test]
async fn atom_feed_lists_expected_topics() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/feed.atom")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let xml = std::str::from_utf8(&bytes).unwrap();

    // Both seeded TOPICs should appear.
    assert!(
        xml.contains("topic-alpha"),
        "Atom feed should contain topic-alpha: {xml}"
    );
    assert!(
        xml.contains("topic-beta"),
        "Atom feed should contain topic-beta: {xml}"
    );

    // The bilingual sibling must NOT appear.
    assert!(
        !xml.contains("topic-alpha.es"),
        "Atom feed must not contain bilingual sibling: {xml}"
    );
}

// ─── JSON Feed ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn json_feed_returns_200_with_json_content_type() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/feed.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let ct = resp
        .headers()
        .get("content-type")
        .expect("content-type header should be present")
        .to_str()
        .unwrap();
    assert!(
        ct.contains("application/json"),
        "content-type should be application/json: {ct}"
    );
}

#[tokio::test]
async fn json_feed_version_field_starts_with_jsonfeed_url() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/feed.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&bytes)
        .expect("JSON feed body should parse as JSON");

    let version = json["version"]
        .as_str()
        .expect("version field should be a string");
    assert!(
        version.starts_with("https://jsonfeed.org/"),
        "version should start with https://jsonfeed.org/: {version}"
    );
}

#[tokio::test]
async fn json_feed_lists_expected_topics() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/feed.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    // `items` array must be present and contain both TOPICs.
    let items = json["items"].as_array().expect("items should be an array");
    assert!(
        items.len() >= 2,
        "items should contain at least 2 entries: {json}"
    );

    let ids: Vec<&str> = items
        .iter()
        .filter_map(|i| i["id"].as_str())
        .collect();
    let has_alpha = ids.iter().any(|id| id.contains("topic-alpha"));
    let has_beta = ids.iter().any(|id| id.contains("topic-beta"));
    assert!(has_alpha, "items should include topic-alpha: {json}");
    assert!(has_beta, "items should include topic-beta: {json}");

    // Bilingual sibling must not appear.
    let has_es = ids.iter().any(|id| id.contains("topic-alpha.es"));
    assert!(!has_es, "items must not include bilingual sibling: {json}");
}
