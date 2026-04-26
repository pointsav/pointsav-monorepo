//! Integration tests for Phase 2 Step 2 — edit endpoint, atomic write,
//! path-traversal hardening.

use http_body_util::BodyExt;
use tower::ServiceExt;

use app_mediakit_knowledge::server::{router, AppState};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};

async fn fixture_state() -> (AppState, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    tokio::fs::write(
        dir.path().join("topic-existing.md"),
        "---\ntitle: Existing\nslug: topic-existing\n---\n# Original\n",
    )
    .await
    .unwrap();
    let state = AppState {
        content_dir: dir.path().to_path_buf(),
    };
    (state, dir)
}

#[tokio::test]
async fn get_edit_returns_editor_page_for_existing_slug() {
    let (state, _dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/edit/topic-existing")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let html = std::str::from_utf8(&body).unwrap();
    assert!(
        html.contains("topic-existing"),
        "editor page should mention slug: {html}"
    );
    assert!(
        html.contains("Edit"),
        "editor page should contain Edit chrome: {html}"
    );
}

#[tokio::test]
async fn get_edit_returns_editor_page_for_nonexistent_slug() {
    // GET /edit/<new-slug> should return the editor (so the user can start
    // typing before /create is called); the placeholder explains the file
    // doesn't exist yet.
    let (state, _dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/edit/topic-not-yet")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let html = std::str::from_utf8(&body).unwrap();
    assert!(
        html.contains("does not exist"),
        "editor for nonexistent slug should say so: {html}"
    );
}

#[tokio::test]
async fn post_edit_writes_to_existing_file_atomically() {
    let (state, dir) = fixture_state().await;
    let new_content = "---\ntitle: Updated\nslug: topic-existing\n---\n# New body\n";
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit/topic-existing")
                .body(Body::from(new_content))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let on_disk = tokio::fs::read_to_string(dir.path().join("topic-existing.md"))
        .await
        .unwrap();
    assert_eq!(on_disk, new_content);
}

#[tokio::test]
async fn post_edit_404_for_nonexistent_file() {
    let (state, _dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit/topic-not-here")
                .body(Body::from("anything"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn post_edit_rejects_invalid_slug_shape() {
    let (state, _dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit/Foo%20Bar")
                .body(Body::from("x"))
                .unwrap(),
        )
        .await
        .unwrap();
    // 400 Bad Request from validate_slug
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn post_create_writes_new_file_with_explicit_slug() {
    let (state, dir) = fixture_state().await;
    let app = router(state);
    let body = serde_json::json!({
        "title": "Brand New Topic",
        "slug": "topic-brand-new"
    })
    .to_string();
    let resp = app
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
    let on_disk = tokio::fs::read_to_string(dir.path().join("topic-brand-new.md"))
        .await
        .unwrap();
    assert!(on_disk.contains("title: \"Brand New Topic\""));
    assert!(on_disk.contains("slug: topic-brand-new"));
}

#[tokio::test]
async fn post_create_409_if_already_exists() {
    let (state, _dir) = fixture_state().await;
    let app = router(state);
    let body = serde_json::json!({
        "title": "Already Exists",
        "slug": "topic-existing"
    })
    .to_string();
    let resp = app
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
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn post_create_derives_slug_from_title() {
    let (state, dir) = fixture_state().await;
    let app = router(state);
    let body = serde_json::json!({
        "title": "Auto Slug Test"
    })
    .to_string();
    let resp = app
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
    let on_disk = tokio::fs::read_to_string(dir.path().join("auto-slug-test.md"))
        .await
        .unwrap();
    assert!(on_disk.contains("slug: auto-slug-test"));
}
