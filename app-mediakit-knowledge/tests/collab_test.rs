//! Integration tests for Phase 2 Step 7 — collab WebSocket relay gating.
//!
//! Because WebSocket upgrades are awkward to exercise via tower::oneshot,
//! these tests verify the route MOUNT behaviour — when --enable-collab
//! is set, the `/ws/collab/{slug}` route exists; when not, it's a 404.
//!
//! Round-trip yjs sync between two browsers is verified manually on the
//! deploy host (not in this test suite).

use app_mediakit_knowledge::server::{router, AppState};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

async fn build_state(enable_collab: bool) -> (AppState, tempfile::TempDir, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = tempfile::tempdir().unwrap();
    let index = app_mediakit_knowledge::search::build_index(dir.path(), state_dir.path())
        .await
        .unwrap();
    let repo = app_mediakit_knowledge::git::open_or_init(dir.path()).unwrap();
    let state = AppState {
        content_dir: dir.path().to_path_buf(),
        guide_dir: None,
        guide_dir_2: None,
        citations_yaml: std::path::PathBuf::from("/nonexistent/citations.yaml"),
        search: Arc::new(index),
        git: Arc::new(Mutex::new(repo)),
        collab: Arc::new(app_mediakit_knowledge::collab::CollabRooms::new()),
        enable_collab,
        site_title: "PointSav Documentation Wiki".to_string(),
        git_tenant: "pointsav".to_string(),
        mcp_enabled: false,
        glossary: Arc::new(app_mediakit_knowledge::glossary::Glossary::default()),
                links: app_mediakit_knowledge::links::LinkGraph::for_testing(),
                brand_theme: None,
                db: None,
    };
    (state, dir, state_dir)
}

#[tokio::test]
async fn ws_collab_route_404_when_collab_disabled() {
    let (state, _d, _s) = build_state(false).await;
    let app = router(state);
    // GET without WebSocket upgrade headers — should still hit the routing
    // layer and return 404 because the route isn't mounted.
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/ws/collab/topic-foo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn ws_collab_route_mounted_when_collab_enabled() {
    let (state, _d, _s) = build_state(true).await;
    let app = router(state);
    // GET without upgrade headers — route is mounted, but WebSocketUpgrade
    // extractor refuses non-upgrade requests with 426 Upgrade Required (or
    // 400 depending on axum version). Either way, NOT 404.
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/ws/collab/topic-foo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_ne!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "route should be mounted when --enable-collab is set"
    );
}

#[tokio::test]
async fn editor_page_omits_collab_flag_when_disabled() {
    use http_body_util::BodyExt;
    let (state, dir, _s) = build_state(false).await;
    tokio::fs::write(
        dir.path().join("topic-x.md"),
        "---\ntitle: X\nslug: topic-x\n---\nbody\n",
    )
    .await
    .unwrap();
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/edit/topic-x")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let html = String::from_utf8(
        resp.into_body().collect().await.unwrap().to_bytes().to_vec(),
    )
    .unwrap();
    assert!(
        !html.contains("WIKI_COLLAB_ENABLED"),
        "WIKI_COLLAB_ENABLED should NOT appear when --enable-collab is unset: {html}"
    );
}

#[tokio::test]
async fn editor_page_includes_collab_flag_when_enabled() {
    use http_body_util::BodyExt;
    let (state, dir, _s) = build_state(true).await;
    tokio::fs::write(
        dir.path().join("topic-y.md"),
        "---\ntitle: Y\nslug: topic-y\n---\nbody\n",
    )
    .await
    .unwrap();
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/edit/topic-y")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let html = String::from_utf8(
        resp.into_body().collect().await.unwrap().to_bytes().to_vec(),
    )
    .unwrap();
    assert!(
        html.contains("window.WIKI_COLLAB_ENABLED=true"),
        "WIKI_COLLAB_ENABLED should appear when --enable-collab is set: {html}"
    );
}
