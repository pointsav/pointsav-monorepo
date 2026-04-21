// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.
//
// Integration smoke test: exercises the public API across the crate boundary.

#![allow(missing_docs, clippy::disallowed_methods)]

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt as _;

#[tokio::test]
async fn health_returns_200() {
    let app = slm_api::router();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn unknown_route_returns_404() {
    let app = slm_api::router();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/not-a-real-route")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
