// SPDX-License-Identifier: Apache-2.0

//! `/v1/adapters/pull` and `/v1/adapters/local/{job_id}` reject job ids that
//! could escape the adapter storage directory, and non-http(s) source URLs.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use slm_doorman_server::http::router;
use slm_doorman_server::test_helpers::app_state_no_tiers;
use tower::ServiceExt;

async fn pull(job_id: &str, source_url: &str) -> StatusCode {
    let app = router(app_state_no_tiers());
    let req = Request::builder()
        .method("POST")
        .uri("/v1/adapters/pull")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({"job_id": job_id, "source_url": source_url}).to_string(),
        ))
        .unwrap();
    app.oneshot(req).await.unwrap().status()
}

#[tokio::test]
async fn pull_rejects_traversal_and_odd_job_ids() {
    for bad in ["../x", "..", "a/b", "/etc/passwd", "a\\b", "", "a b", "x.y"] {
        assert_eq!(
            pull(bad, "http://127.0.0.1:1/x").await,
            StatusCode::BAD_REQUEST,
            "job_id {bad:?} must be refused"
        );
    }
    let long = "a".repeat(129);
    assert_eq!(
        pull(&long, "http://127.0.0.1:1/x").await,
        StatusCode::BAD_REQUEST
    );
}

#[tokio::test]
async fn pull_rejects_non_http_source_url() {
    for bad in [
        "file:///etc/passwd",
        "ftp://h/x",
        "gopher://h/x",
        "/etc/passwd",
    ] {
        assert_eq!(pull("job-1", bad).await, StatusCode::BAD_REQUEST, "{bad}");
    }
}

#[tokio::test]
async fn read_back_rejects_traversal_job_id() {
    for uri in [
        "/v1/adapters/local/..%2F..%2Fetc%2Fpasswd",
        "/v1/adapters/local/a%2Fb",
        "/v1/adapters/local/a.b",
    ] {
        let app = router(app_state_no_tiers());
        let req = Request::builder().uri(uri).body(Body::empty()).unwrap();
        let status = app.oneshot(req).await.unwrap().status();
        assert_eq!(status, StatusCode::BAD_REQUEST, "{uri}");
    }
}

#[tokio::test]
async fn read_back_valid_unknown_id_is_404() {
    let app = router(app_state_no_tiers());
    let req = Request::builder()
        .uri("/v1/adapters/local/job-does-not-exist-123")
        .body(Body::empty())
        .unwrap();
    assert_eq!(
        app.oneshot(req).await.unwrap().status(),
        StatusCode::NOT_FOUND
    );
}
