//! Integration tests for Phase 5 — authentication routes.
//!
//! Verifies:
//! - `GET /special/login` returns 200 with a login form
//! - `POST /special/login` with bad credentials redirects with `error=`
//! - `POST /special/login` with good credentials sets `wiki_session` cookie
//! - `POST /special/logout` clears the session cookie
//! - `GET /special/create-account` accessible with a valid admin session

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

/// AppState with no DB — auth-less mode; all auth extractors treat requests as admin.
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
        brand_theme: None,
        db: None,
    };
    (state, dir, state_dir)
}

/// AppState with an in-memory SQLite DB. Admin "wikiadmin"/"wikiadmin" is seeded.
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
        brand_theme: None,
        db: Some(db),
    };
    (state, token, dir, state_dir)
}

// ─── Login page ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn login_page_returns_200() {
    let (state, _dir, _state_dir) = authless_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/special/login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let body = std::str::from_utf8(&bytes).unwrap();
    assert!(
        body.contains("<form"),
        "login page should contain a <form element: {body}"
    );
}

// ─── POST /special/login ─────────────────────────────────────────────────────

#[tokio::test]
async fn login_with_invalid_credentials_redirects_with_error() {
    let (state, _token, _dir, _state_dir) = auth_state_with_token().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/special/login")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from("username=wikiadmin&password=wrongpassword&next=%2F"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let location = resp
        .headers()
        .get("location")
        .expect("Location header should be present after failed login")
        .to_str()
        .unwrap();
    assert!(
        location.contains("error="),
        "failed login should redirect with error= in Location: {location}"
    );
}

#[tokio::test]
async fn login_with_valid_credentials_sets_session_cookie() {
    let (state, _token, _dir, _state_dir) = auth_state_with_token().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/special/login")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from("username=wikiadmin&password=wikiadmin&next=%2F"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let cookie = resp
        .headers()
        .get("set-cookie")
        .expect("set-cookie header should be present after successful login")
        .to_str()
        .unwrap();
    assert!(
        cookie.contains("wiki_session="),
        "set-cookie should contain wiki_session: {cookie}"
    );
    let session_value = cookie
        .split("wiki_session=")
        .nth(1)
        .unwrap_or("")
        .split(';')
        .next()
        .unwrap_or("");
    assert!(
        !session_value.is_empty(),
        "wiki_session cookie value should not be empty after successful login"
    );
}

// ─── POST /special/logout ────────────────────────────────────────────────────

#[tokio::test]
async fn logout_clears_session_cookie() {
    let (state, _dir, _state_dir) = authless_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/special/logout")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let cookie = resp
        .headers()
        .get("set-cookie")
        .expect("set-cookie header should be present after logout")
        .to_str()
        .unwrap();
    assert!(
        cookie.contains("wiki_session=;") || cookie.contains("Max-Age=0"),
        "logout should clear the session cookie: {cookie}"
    );
}

// ─── GET /special/create-account ─────────────────────────────────────────────

#[tokio::test]
async fn create_account_page_accessible_as_admin() {
    let (state, token, _dir, _state_dir) = auth_state_with_token().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/special/create-account")
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
        body.contains("Create account"),
        "create-account page should contain the heading: {body}"
    );
}
