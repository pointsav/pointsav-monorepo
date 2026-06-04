//! Wiki article routes.
//!
//! Phase 1 stub. Phase 4 migrates the `wiki_page` and `wiki_page_es`
//! handlers from `server.rs` into this module.
//!
//! Routes owned by this module (Phase 4 target):
//! - `GET /wiki/{*slug}`
//! - `GET /es/wiki/{*slug}`

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 4: GET /wiki/{*slug} handler.
#[allow(dead_code)]
pub async fn wiki_page() -> impl IntoResponse {
    todo!("Phase 4: migrate wiki_page handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 4: GET /es/wiki/{*slug} handler.
#[allow(dead_code)]
pub async fn wiki_page_es() -> impl IntoResponse {
    todo!("Phase 4: migrate wiki_page_es handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}
