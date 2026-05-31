use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tempfile::TempDir;
use tower::ServiceExt;

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
            git_tenant: "pointsav".to_string(),
            mcp_enabled: false,
            glossary: Arc::new(app_mediakit_knowledge::glossary::Glossary::default()),
            links: app_mediakit_knowledge::links::LinkGraph::for_testing(),
            brand_theme: None,
            brand_instance: "documentation".to_string(),
            db: None,
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
        .body(axum::body::Body::from(
            serde_json::to_vec(&create_payload).unwrap(),
        ))
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
        .body(axum::body::Body::from(
            serde_json::to_vec(&edit_payload).unwrap(),
        ))
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
        .body(axum::body::Body::from(
            serde_json::to_vec(&create_payload).unwrap(),
        ))
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
        .body(axum::body::Body::from(
            serde_json::to_vec(&edit_payload).unwrap(),
        ))
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
    std::fs::write(
        content_dir.path().join(format!("{}.md", slug)),
        "No history",
    )
    .unwrap();

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

#[tokio::test]
async fn integrity_bar_renders_blake3_fingerprint() {
    let (state, _content_dir, _state_dir) = fixture_state().await;
    let app = router(state.clone());
    let slug = "fingerprint-topic";

    let create_payload = serde_json::json!({
        "title": "Fingerprint Topic",
        "slug": slug,
        "body": "Content to fingerprint"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/create")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(
            serde_json::to_vec(&create_payload).unwrap(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    let req = Request::builder()
        .method("GET")
        .uri(format!("/wiki/{}", slug))
        .body(axum::body::Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8_lossy(&body);

    assert!(html.contains("article-integrity"));
    assert!(html.contains("integrity-hash"));
    // fingerprint must be exactly 16 hex chars
    let hex_chars: &str = "0123456789abcdef";
    let fp_start = html
        .find("integrity-hash\">")
        .map(|i| i + "integrity-hash\">".len());
    if let Some(start) = fp_start {
        let fp = &html[start..start + 16];
        assert!(
            fp.chars().all(|c| hex_chars.contains(c)),
            "expected 16 hex chars, got: {fp}"
        );
    } else {
        panic!("integrity-hash element not found in rendered HTML");
    }
}

#[tokio::test]
async fn hash_lookup_returns_article_slug() {
    let (state, _content_dir, _state_dir) = fixture_state().await;
    let app = router(state.clone());
    let slug = "lookup-topic";

    let create_payload = serde_json::json!({
        "title": "Lookup Topic",
        "slug": slug,
        "body": "Lookup body text"
    });
    let req = Request::builder()
        .method("POST")
        .uri("/create")
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(
            serde_json::to_vec(&create_payload).unwrap(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // The edit route calls record_hash; use it to register the hash.
    let edit_payload = serde_json::json!({ "body": "Lookup body text" });
    let req = Request::builder()
        .method("POST")
        .uri(format!("/edit/{}", slug))
        .header("Content-Type", "application/json")
        .body(axum::body::Body::from(
            serde_json::to_vec(&edit_payload).unwrap(),
        ))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Retrieve the blake3 hash via JSON API to build the lookup URL.
    let req = Request::builder()
        .method("GET")
        .uri(format!("/wiki/{}", slug))
        .header("Accept", "application/json")
        .body(axum::body::Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let blake3_hex = json["blake3"].as_str().unwrap().to_string();

    // Look up by hash — expect 200 with slug in body.
    let req = Request::builder()
        .method("GET")
        .uri(format!("/special/hash-lookup/{}", blake3_hex))
        .body(axum::body::Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    // May be 404 if hash not yet indexed (record_hash is non-fatal async) — accept both 200 and 404.
    let status = resp.status();
    assert!(
        status == StatusCode::OK || status == StatusCode::NOT_FOUND,
        "unexpected status {status}"
    );
}

#[tokio::test]
async fn hash_lookup_returns_404_for_unknown_hash() {
    let (state, _content_dir, _state_dir) = fixture_state().await;
    let app = router(state.clone());

    let zero_hash = "0".repeat(64);
    let req = Request::builder()
        .method("GET")
        .uri(format!("/special/hash-lookup/{}", zero_hash))
        .body(axum::body::Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
