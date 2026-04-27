//! Integration tests for Phase 3 Step 3.2 — search HTTP route +
//! edit-triggers-reindex.
//!
//! Verifies that:
//! - `GET /search?q=...` returns an HTML page with results from the index
//! - The empty `q=` case returns the form without errors
//! - `POST /edit/{slug}` triggers a reindex so subsequent searches see the
//!   new body
//! - `POST /create` triggers a reindex so a freshly created TOPIC is
//!   immediately searchable

use app_mediakit_knowledge::search;
use app_mediakit_knowledge::server::{router, AppState};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use std::sync::Arc;
use tower::ServiceExt;

async fn fixture_state() -> (AppState, tempfile::TempDir, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = tempfile::tempdir().unwrap();
    // Seed three TOPICs covering distinct keywords for ranking + miss tests.
    tokio::fs::write(
        dir.path().join("topic-alpha.md"),
        "---\ntitle: \"Alpha Subject\"\nslug: topic-alpha\n---\nAlpha discusses the substrate. Substrate is the load-bearing concept.\n",
    )
    .await
    .unwrap();
    tokio::fs::write(
        dir.path().join("topic-beta.md"),
        "---\ntitle: \"Beta Subject\"\nslug: topic-beta\n---\nBeta covers continuous disclosure and the BCSC posture.\n",
    )
    .await
    .unwrap();
    tokio::fs::write(
        dir.path().join("topic-gamma.md"),
        "---\ntitle: \"Gamma Subject\"\nslug: topic-gamma\n---\nGamma is unrelated content for control.\n",
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
        collab: Arc::new(app_mediakit_knowledge::collab::CollabRooms::new()),
        enable_collab: false,
    };
    (state, dir, state_dir)
}

#[tokio::test]
async fn search_with_no_query_returns_empty_form() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/search")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let html = std::str::from_utf8(
        &resp.into_body().collect().await.unwrap().to_bytes(),
    )
    .unwrap()
    .to_string();
    assert!(html.contains("<form"), "search form missing: {html}");
    assert!(
        html.contains(r#"name="q""#),
        "search input missing: {html}"
    );
    // No results section yet
    assert!(
        !html.contains("search-results"),
        "results list should not appear for empty query: {html}"
    );
}

#[tokio::test]
async fn search_returns_matching_topic() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/search?q=substrate")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let html = std::str::from_utf8(
        &resp.into_body().collect().await.unwrap().to_bytes(),
    )
    .unwrap()
    .to_string();
    assert!(
        html.contains("topic-alpha"),
        "alpha should match 'substrate': {html}"
    );
    assert!(
        html.contains("search-results"),
        "results list should render: {html}"
    );
}

#[tokio::test]
async fn search_returns_empty_for_no_match() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/search?q=xyzzy-no-such-term")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let html = std::str::from_utf8(
        &resp.into_body().collect().await.unwrap().to_bytes(),
    )
    .unwrap()
    .to_string();
    assert!(
        html.contains("No results"),
        "no-match copy missing: {html}"
    );
}

#[tokio::test]
async fn post_edit_triggers_reindex() {
    let (state, dir, _state_dir) = fixture_state().await;
    let app = router(state);

    // Edit topic-alpha to remove "substrate" and add a unique new keyword.
    let new_body = "---\ntitle: \"Alpha v2\"\nslug: topic-alpha\n---\nAlpha now discusses tangerines and quokkas.\n";
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit/topic-alpha")
                .body(Body::from(new_body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Confirm the file changed on disk.
    let on_disk = tokio::fs::read_to_string(dir.path().join("topic-alpha.md"))
        .await
        .unwrap();
    assert_eq!(on_disk, new_body);

    // Search for the new keyword — should hit topic-alpha.
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/search?q=tangerines")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let html = std::str::from_utf8(
        &resp.into_body().collect().await.unwrap().to_bytes(),
    )
    .unwrap()
    .to_string();
    assert!(
        html.contains("topic-alpha"),
        "reindex should make new keyword searchable: {html}"
    );

    // Search for the old keyword — topic-alpha should no longer hit
    // (delete_term removed the prior body).
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/search?q=substrate")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let html = std::str::from_utf8(
        &resp.into_body().collect().await.unwrap().to_bytes(),
    )
    .unwrap()
    .to_string();
    // We only seeded topic-alpha with "substrate"; after reindex it's gone.
    assert!(
        html.contains("No results"),
        "old keyword should no longer match after reindex: {html}"
    );
}

#[tokio::test]
async fn post_create_triggers_reindex() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);

    // Create a brand-new TOPIC.
    let body = serde_json::json!({
        "title": "Brand New Topic",
        "slug": "topic-brand-new"
    })
    .to_string();
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // The created TOPIC's title should be immediately searchable.
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/search?q=Brand")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let html = std::str::from_utf8(
        &resp.into_body().collect().await.unwrap().to_bytes(),
    )
    .unwrap()
    .to_string();
    assert!(
        html.contains("topic-brand-new"),
        "newly-created TOPIC should be searchable by title: {html}"
    );
}
