//! Integration tests for Phase 4 Step 4.1 — git2 wiring + commit-on-edit.

use http_body_util::BodyExt;
use tower::ServiceExt;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

use app_mediakit_knowledge::server::{router, AppState};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};

async fn fixture_state() -> (AppState, tempfile::TempDir, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = tempfile::tempdir().unwrap();
    
    // Initialize git repo in content_dir
    let repo = app_mediakit_knowledge::git::open_or_init(dir.path()).unwrap();
    
    let index = app_mediakit_knowledge::search::build_index(dir.path(), state_dir.path())
        .await
        .unwrap();
    let state = AppState {
        content_dir: dir.path().to_path_buf(),
        guide_dir: None,
        guide_dir_2: None,
        citations_yaml: PathBuf::from("/nonexistent/citations.yaml"),
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
async fn git_commit_on_create() {
    let (state, dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let body = serde_json::json!({
        "title": "Git Create Test",
        "slug": "git-create"
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
    
    // Check git log
    let output = std::process::Command::new("git")
        .args(["-C", dir.path().to_str().unwrap(), "log", "-1", "--format=%s"])
        .output()
        .expect("git log failed");
    let msg = String::from_utf8_lossy(&output.stdout).trim().to_string();
    assert_eq!(msg, "create: git-create");
}

#[tokio::test]
async fn git_commit_on_edit() {
    let (state, dir, _state_dir) = fixture_state().await;
    
    // Create first file so it's ready for edit
    tokio::fs::write(dir.path().join("git-edit.md"), "# Initial").await.unwrap();
    
    let app = router(state);
    let new_content = "# Updated content";
    let json_body = serde_json::json!({"body": new_content, "edit_summary": ""});
    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit/git-edit")
                .header("content-type", "application/json")
                .body(Body::from(json_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    
    // Check git log
    let output = std::process::Command::new("git")
        .args(["-C", dir.path().to_str().unwrap(), "log", "-1", "--format=%s"])
        .output()
        .expect("git log failed");
    let msg = String::from_utf8_lossy(&output.stdout).trim().to_string();
    assert_eq!(msg, "edit: git-edit");
}

#[tokio::test]
async fn open_or_init_idempotency() {
    let dir = tempfile::tempdir().unwrap();
    
    // First call
    let _repo1 = app_mediakit_knowledge::git::open_or_init(dir.path()).unwrap();
    assert!(dir.path().join(".git").exists());
    
    // Second call
    let _repo2 = app_mediakit_knowledge::git::open_or_init(dir.path()).unwrap();
}

#[tokio::test]
async fn git_identity_alternation() {
    let (state, dir, _state_dir) = fixture_state().await;
    
    // Mock toggle file in a temp home dir
    let home_dir = tempfile::tempdir().unwrap();
    let toggle_path = home_dir.path().join("Foundry/identity/.toggle");
    std::fs::create_dir_all(toggle_path.parent().unwrap()).unwrap();
    
    // Set HOME to our temp home dir
    let original_home = std::env::var("HOME").ok();
    std::env::set_var("HOME", home_dir.path());
    
    let app = router(state);
    
    // Test identity 0 (Jennifer)
    std::fs::write(&toggle_path, "0").unwrap();
    let _ = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri("/create")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::json!({"title": "T1", "slug": "t1"}).to_string()))
            .unwrap(),
    ).await.unwrap();
    
    let output = std::process::Command::new("git")
        .args(["-C", dir.path().to_str().unwrap(), "log", "-1", "--format=%an <%ae>"])
        .output()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "Jennifer Woodfine <jwoodfine@users.noreply.github.com>");

    // Test identity 1 (Peter)
    std::fs::write(&toggle_path, "1").unwrap();
    let _ = app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/create")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::json!({"title": "T2", "slug": "t2"}).to_string()))
            .unwrap(),
    ).await.unwrap();
    
    let output = std::process::Command::new("git")
        .args(["-C", dir.path().to_str().unwrap(), "log", "-1", "--format=%an <%ae>"])
        .output()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "Peter Woodfine <pwoodfine@users.noreply.github.com>");
    
    // Restore HOME
    if let Some(h) = original_home {
        std::env::set_var("HOME", h);
    }
}
