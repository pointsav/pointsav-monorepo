use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use tempfile::TempDir;

use app_mediakit_knowledge::server::{router, AppState};

async fn fixture_state() -> (AppState, TempDir, TempDir) {
    let content_dir = tempfile::tempdir().unwrap();
    let state_dir = tempfile::tempdir().unwrap();
    
    // Initialise git repo
    let repo = app_mediakit_knowledge::git::open_or_init(content_dir.path()).unwrap();
    app_mediakit_knowledge::git::ensure_commit_identity_from_env(&repo).unwrap();

    let search = app_mediakit_knowledge::search::build_index(content_dir.path(), state_dir.path())
        .await
        .unwrap();

    (
        AppState {
            content_dir: content_dir.path().to_path_buf(),
            guide_dir: None,
            guide_dir_2: None,
            citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
            search: Arc::new(search),
            git: Arc::new(Mutex::new(repo)),
            collab: Arc::new(app_mediakit_knowledge::collab::CollabRooms::new()),
            enable_collab: false,
            site_title: "Test Wiki".to_string(),
        },
        content_dir,
        state_dir,
    )
}

#[tokio::test]
async fn test_history_list() {
    let (state, _content_dir, _state_dir) = fixture_state().await;
    let app = router(state.clone());
    let slug = "test-topic";

    // 1. Create a topic
    let create_payload = serde_json::json!({
        "title": "Test Topic",
        "slug": slug,
        "body": "Version 1"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/create")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(serde_json::to_vec(&create_payload).unwrap()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // 2. Edit the topic
    let edit_payload = serde_json::json!({
        "body": "Version 2"
    });
    let req = Request::builder()
        .method("POST")
        .uri(format!("/edit/{}", slug))
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(serde_json::to_vec(&edit_payload).unwrap()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // 3. Check history
    let req = Request::builder()
        .method("GET")
        .uri(format!("/history/{}", slug))
        .body(axum::body::Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8_lossy(&body);
    
    assert!(html.contains("History: test-topic"));
    assert!(html.contains("create: test-topic"));
    assert!(html.contains("edit: test-topic"));
}

#[tokio::test]
async fn test_blame_annotation() {
    let (state, _content_dir, _state_dir) = fixture_state().await;
    let app = router(state.clone());
    let slug = "blame-topic";

    // Create topic
    let create_payload = serde_json::json!({
        "title": "Blame Topic",
        "slug": slug,
        "body": "Line 1\nLine 2"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/create")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(serde_json::to_vec(&create_payload).unwrap()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Edit to add lines
    let edit_payload = serde_json::json!({
        "body": "Line 1\nLine 2"
    });
    let req = Request::builder()
        .method("POST")
        .uri(format!("/edit/{}", slug))
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(serde_json::to_vec(&edit_payload).unwrap()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Check blame
    let req = Request::builder()
        .method("GET")
        .uri(format!("/blame/{}", slug))
        .body(axum::body::Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8_lossy(&body);
    
    assert!(html.contains("Blame: blame-topic"));
    assert!(html.contains("Line 1"));
    assert!(html.contains("Line 2"));
}

#[tokio::test]
async fn test_empty_history() {
    let (state, content_dir, _state_dir) = fixture_state().await;
    let app = router(state.clone());
    let slug = "no-history";
    
    // Create file WITHOUT git commit (manual write)
    std::fs::write(content_dir.path().join(format!("{}.md", slug)), "No history").unwrap();

    let req = Request::builder()
        .method("GET")
        .uri(format!("/history/{}", slug))
        .body(axum::body::Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8_lossy(&body);
    
    assert!(html.contains("No revision history yet."));
}

#[tokio::test]
async fn test_unknown_slug() {
    let (state, _content_dir, _state_dir) = fixture_state().await;
    let app = router(state.clone());

    let req = Request::builder()
        .method("GET")
        .uri("/history/nonexistent")
        .body(axum::body::Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
