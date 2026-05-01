//! Integration test: GET /api/squiggle-rules returns the JSON rule set.

use http_body_util::BodyExt;
use tower::ServiceExt;

use app_mediakit_knowledge::search;
use app_mediakit_knowledge::server::{router, AppState};
use axum::{body::Body, http::Request};
use std::sync::Arc;

async fn fixture_state() -> (AppState, tempfile::TempDir, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = tempfile::tempdir().unwrap();
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
async fn api_squiggle_rules_returns_json_array() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/squiggle-rules")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let ct = resp
        .headers()
        .get("content-type")
        .map(|v| v.to_str().unwrap_or(""))
        .unwrap_or("");
    assert!(
        ct.starts_with("application/json"),
        "expected JSON content-type, got {ct}"
    );

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let parsed: serde_json::Value = serde_json::from_slice(&body).expect("body should be JSON");
    let arr = parsed.as_array().expect("body should be a JSON array");
    assert!(!arr.is_empty(), "rule set should be non-empty");

    // Each entry has the expected shape
    for rule in arr {
        for field in &["id", "severity", "pattern", "flags", "message", "citation"] {
            assert!(rule.get(*field).is_some(), "rule missing field {field}: {rule}");
        }
        let severity = rule["severity"].as_str().unwrap_or("");
        assert!(
            ["error", "warning", "info", "hint"].contains(&severity),
            "unexpected severity: {severity}"
        );
    }
}

#[tokio::test]
async fn api_squiggle_rules_includes_each_severity() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/squiggle-rules")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let arr: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    let severities: std::collections::HashSet<String> = arr
        .iter()
        .map(|r| r["severity"].as_str().unwrap_or("").to_string())
        .collect();

    for required in &["error", "warning", "info", "hint"] {
        assert!(
            severities.contains(*required),
            "rule set missing severity: {required}"
        );
    }
}
