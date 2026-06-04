//! Category listing routes.
//!
//! Phase 1 stub. Phase 4 migrates the `category_page` handler from
//! `server.rs` into this module.
//!
//! Routes owned by this module (Phase 4 target):
//! - `GET /category/{name}`
//! - `GET /category/{name}?sort=recent`

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 4: GET /category/{name} handler.
#[allow(dead_code)]
pub async fn category_page() -> impl IntoResponse {
    todo!("Phase 4: migrate category_page handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}
