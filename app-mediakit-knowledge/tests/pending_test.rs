//! Integration tests for Phase 5 — edit review queue.
//!
//! Verifies:
//! - `GET /special/pending-changes` accessible in auth-less mode (AdminUser passthrough)
//! - `GET /special/pending-changes` shows "No pending edits" when queue is empty
//! - `GET /special/contributions/{username}` accessible in auth-less mode
//! - `GET /special/pending-changes` redirects unauthenticated requests to login

use app_mediakit_knowledge::{search, users};
use app_mediakit_knowledge::server::{router, AppState};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

/// AppState with no DB — auth-less mode; AdminUser and LoggedInUser pass through as admin.
async fn authless_state() -> (AppState, tempfile::TempDir, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = tempfile::tempdir().unwrap();
    let index = search::build_index(dir.path(), state_dir.path())
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
        enable_collab: false,
        site_title: "PointSav Documentation Wiki".to_string(),
        git_tenant: "pointsav".to_string(),
        mcp_enabled: false,
        glossary: Arc::new(app_mediakit_knowledge::glossary::Glossary::default()),
        links: app_mediakit_knowledge::links::LinkGraph::for_testing(),
        db: None,
    };
    (state, dir, state_dir)
}

/// AppState with an in-memory SQLite DB. Admin "wikiadmin"/"wikiadmin" seeded.
/// Returns the AppState plus a pre-created admin session token.
async fn auth_state_with_token() -> (AppState, String, tempfile::TempDir, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = tempfile::tempdir().unwrap();
    let index = search::build_index(dir.path(), state_dir.path())
        .await
        .unwrap();
    let repo = app_mediakit_knowledge::git::open_or_init(dir.path()).unwrap();

    let conn = Connection::open_in_memory().unwrap();
    users::init_schema(&conn).unwrap();
    let hash = users::hash_password("wikiadmin").unwrap();
    users::seed_admin_if_empty(&conn, "wikiadmin", &hash).unwrap();
    let user = users::get_by_username(&conn, "wikiadmin").unwrap().unwrap();
    let token = users::create_session(&conn, &user.id).unwrap();
    let db = Arc::new(Mutex::new(conn));

    let state = AppState {
        content_dir: dir.path().to_path_buf(),
        guide_dir: None,
        guide_dir_2: None,
        citations_yaml: std::path::PathBuf::from("/nonexistent/citations.yaml"),
        search: Arc::new(index),
        git: Arc::new(Mutex::new(repo)),
        collab: Arc::new(app_mediakit_knowledge::collab::CollabRooms::new()),
        enable_collab: false,
        site_title: "PointSav Documentation Wiki".to_string(),
        git_tenant: "pointsav".to_string(),
        mcp_enabled: false,
        glossary: Arc::new(app_mediakit_knowledge::glossary::Glossary::default()),
        links: app_mediakit_knowledge::links::LinkGraph::for_testing(),
        db: Some(db),
    };
    (state, token, dir, state_dir)
}

// ─── GET /special/pending-changes ────────────────────────────────────────────

#[tokio::test]
async fn pending_changes_accessible_in_authless_mode() {
    let (state, _dir, _state_dir) = authless_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/special/pending-changes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn pending_changes_shows_empty_queue_message() {
    let (state, token, _dir, _state_dir) = auth_state_with_token().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/special/pending-changes")
                .header("cookie", format!("wiki_session={}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&bytes).unwrap();
    assert!(
        body.contains("No pending edits"),
        "empty review queue should display no-pending message: {body}"
    );
}

#[tokio::test]
async fn pending_changes_redirects_unauthenticated_requests() {
    let (state, _token, _dir, _state_dir) = auth_state_with_token().await;
    // No session cookie — DB is present so real auth applies.
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/special/pending-changes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let location = resp
        .headers()
        .get("location")
        .expect("Location header should be present")
        .to_str()
        .unwrap();
    assert!(
        location.contains("/special/login"),
        "unauthenticated pending-changes should redirect to login: {location}"
    );
}

// ─── GET /special/contributions/{username} ────────────────────────────────────

#[tokio::test]
async fn contributions_accessible_in_authless_mode() {
    let (state, _dir, _state_dir) = authless_state().await;
    let app = router(state);
    // Auth-less synthetic user has username "admin" and is_admin() == true,
    // so it can view any user's contributions page.
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/special/contributions/admin")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}
