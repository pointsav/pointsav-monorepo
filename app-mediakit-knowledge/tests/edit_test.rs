//! Integration tests for Phase 2 Step 2 — edit endpoint, atomic write,
//! path-traversal hardening.

use http_body_util::BodyExt;
use tower::ServiceExt;

use app_mediakit_knowledge::search;
use app_mediakit_knowledge::server::{router, AppState};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::sync::{Arc, Mutex};

async fn fixture_state() -> (AppState, tempfile::TempDir, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = tempfile::tempdir().unwrap();
    tokio::fs::write(
        dir.path().join("topic-existing.md"),
        "---\ntitle: Existing\nslug: topic-existing\n---\n# Original\n",
    )
    .await
    .unwrap();
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
        glossary: Arc::new(app_mediakit_knowledge::glossary::Glossary::default()),
                db: None,
    };
    (state, dir, state_dir)
}

#[tokio::test]
async fn get_edit_returns_editor_page_for_existing_slug() {
    let (state, _dir, _state_dir) = fixture_state().await;
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
    assert!(html.contains("topic-existing"), "slug: {html}");
    assert!(html.contains(r#"id="saa-editor""#), "editor mount slot missing");
    assert!(
        html.contains("/static/vendor/cm-saa.bundle.js"),
        "vendor bundle script tag missing"
    );
    assert!(html.contains("/static/saa-init.js"), "init script missing");
    assert!(html.contains("window.SAA_SLUG"), "SAA_SLUG injection missing");
    assert!(
        html.contains("window.SAA_INITIAL"),
        "SAA_INITIAL injection missing"
    );
}

#[tokio::test]
async fn get_edit_returns_editor_page_for_nonexistent_slug() {
    // GET /edit/<new-slug> returns the editor (so the user can start typing
    // before /create is called). Initial doc is empty.
    let (state, _dir, _state_dir) = fixture_state().await;
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
    assert!(html.contains(r#"id="saa-editor""#), "editor mount slot missing");
    // Initial doc for a nonexistent slug serialises to "" (empty string).
    assert!(
        html.contains(r#"window.SAA_INITIAL="""#),
        "SAA_INITIAL should be empty string for nonexistent slug: {html}"
    );
}

#[tokio::test]
async fn get_edit_initial_json_round_trips_special_chars() {
    // Markdown body with quotes, backslashes, and newlines must JSON-encode
    // cleanly into the editor state.
    let dir = tempfile::tempdir().unwrap();
    let state_dir = tempfile::tempdir().unwrap();
    let body = "---\ntitle: \"Quotes\"\nslug: tricky\n---\nLine 1\nLine 2 with \"quotes\"\nBackslash\\here\n";
    tokio::fs::write(dir.path().join("tricky.md"), body)
        .await
        .unwrap();
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
        glossary: Arc::new(app_mediakit_knowledge::glossary::Glossary::default()),
                db: None,
    };
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/edit/tricky")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let html_bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let html = std::str::from_utf8(&html_bytes).unwrap();

    // Find the SAA_INITIAL assignment and parse the JSON literal that follows.
    let needle = "window.SAA_INITIAL=";
    let start = html.find(needle).expect("SAA_INITIAL not present") + needle.len();
    // The script body looks like: window.SAA_SLUG="...";window.SAA_INITIAL="...";
    let semi = html[start..].find(';').expect("missing terminator");
    let json_literal = &html[start..start + semi];
    let decoded: String =
        serde_json::from_str(json_literal).expect("SAA_INITIAL must be valid JSON string");
    assert_eq!(decoded, body);
}

#[tokio::test]
async fn post_edit_writes_to_existing_file_atomically() {
    let (state, dir, _state_dir) = fixture_state().await;
    let new_content = "---\ntitle: Updated\nslug: topic-existing\n---\n# New body\n";
    let app = router(state);
    let json_body = serde_json::json!({"body": new_content, "edit_summary": ""});
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit/topic-existing")
                .header("content-type", "application/json")
                .body(Body::from(json_body.to_string()))
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
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let json_body = serde_json::json!({"body": "anything", "edit_summary": ""});
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit/topic-not-here")
                .header("content-type", "application/json")
                .body(Body::from(json_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn post_edit_rejects_invalid_slug_shape() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let json_body = serde_json::json!({"body": "x", "edit_summary": ""});
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit/Foo%20Bar")
                .header("content-type", "application/json")
                .body(Body::from(json_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    // 400 Bad Request from validate_slug
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn post_create_writes_new_file_with_explicit_slug() {
    let (state, dir, _state_dir) = fixture_state().await;
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
    let (state, _dir, _state_dir) = fixture_state().await;
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
    let (state, dir, _state_dir) = fixture_state().await;
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
