//! Feed and sitemap routes.
//!
//! Phase 1 stub. Phase 4 migrates feed handlers from `server.rs`.
//!
//! Routes owned by this module (Phase 4 target):
//! - `GET /feed.atom`
//! - `GET /feed.json`
//! - `GET /sitemap.xml`
//! - `GET /robots.txt`
//! - `GET /llms.txt`

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 4: GET /feed.atom — Atom RFC 4287 feed.
#[allow(dead_code)]
pub async fn atom_feed() -> impl IntoResponse {
    todo!("Phase 4: migrate atom_feed handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 4: GET /feed.json — JSON Feed v1.
#[allow(dead_code)]
pub async fn json_feed() -> impl IntoResponse {
    todo!("Phase 4: migrate json_feed handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 4: GET /sitemap.xml
#[allow(dead_code)]
pub async fn sitemap() -> impl IntoResponse {
    todo!("Phase 4: migrate sitemap handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 4: GET /robots.txt
#[allow(dead_code)]
pub async fn robots_txt() -> impl IntoResponse {
    todo!("Phase 4: migrate robots_txt handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 4: GET /llms.txt
#[allow(dead_code)]
pub async fn llms_txt() -> impl IntoResponse {
    todo!("Phase 4: migrate llms_txt handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}
