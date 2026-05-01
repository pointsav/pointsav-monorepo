//! Integration test: GET /wiki/{slug} embeds a JSON-LD script in <head>.

use http_body_util::BodyExt;
use tower::ServiceExt;

use app_mediakit_knowledge::search;
use app_mediakit_knowledge::server::{router, AppState};
use axum::{body::Body, http::Request};
use std::sync::Arc;

async fn fixture_state() -> (AppState, tempfile::TempDir, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("topic-test.md");
    tokio::fs::write(
        &path,
        "---\ntitle: \"JSON-LD Test\"\nslug: topic-test\nforward_looking: false\n---\n# Body\n",
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
async fn rendered_page_carries_jsonld_script() {
    let (state, _dir, _state_dir) = fixture_state().await;
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/wiki/topic-test")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let html = std::str::from_utf8(&body).unwrap();

    let prefix = r#"<script type="application/ld+json">"#;
    assert!(
        html.contains(prefix),
        "JSON-LD script tag should appear in rendered page: {html}"
    );

    let start = html.find(prefix).unwrap() + prefix.len();
    let end = html[start..].find("</script>").unwrap() + start;
    let json_str = &html[start..end];
    let parsed: serde_json::Value =
        serde_json::from_str(json_str).expect("JSON-LD body should parse");

    assert_eq!(parsed["@context"], "https://schema.org");
    assert_eq!(parsed["@type"], "TechArticle");
    assert_eq!(parsed["name"], "JSON-LD Test");
    assert_eq!(parsed["identifier"], "topic-test");
    assert_eq!(parsed["inLanguage"], "en");
    assert_eq!(parsed["isPartOf"]["name"], "PointSav Knowledge");
}

#[tokio::test]
async fn fli_topic_carries_additional_property() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = tempfile::tempdir().unwrap();
    tokio::fs::write(
        dir.path().join("topic-fli.md"),
        "---\ntitle: \"FLI Test\"\nslug: topic-fli\nforward_looking: true\n---\n# Body\n",
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
    let app = router(state);
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/wiki/topic-fli")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let html = std::str::from_utf8(&body).unwrap();

    let prefix = r#"<script type="application/ld+json">"#;
    let start = html.find(prefix).unwrap() + prefix.len();
    let end = html[start..].find("</script>").unwrap() + start;
    let parsed: serde_json::Value = serde_json::from_str(&html[start..end]).unwrap();

    assert!(
        parsed["additionalProperty"].is_array(),
        "FLI flag should produce additionalProperty array: {parsed}"
    );
    assert_eq!(parsed["additionalProperty"][0]["name"], "forward_looking");
}
