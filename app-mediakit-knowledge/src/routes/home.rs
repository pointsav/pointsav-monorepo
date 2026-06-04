//! Homepage routes.
//!
//! Phase 1 stub. Phase 4 migrates the `index` and `home_es` handlers from
//! `server.rs` into this module.
//!
//! Routes owned by this module (Phase 4 target):
//! - `GET /`
//! - `GET /es/`

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 4: GET / handler.
#[allow(dead_code)]
pub async fn index() -> impl IntoResponse {
    todo!("Phase 4: migrate index handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 4: GET /es/ handler.
#[allow(dead_code)]
pub async fn home_es() -> impl IntoResponse {
    todo!("Phase 4: migrate home_es handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}
