//! Search routes.
//!
//! Phase 1 stub. Phase 4 migrates search handlers from `server.rs`.
//!
//! Routes owned by this module (Phase 4 target):
//! - `GET /api/search?q={query}`
//! - `GET /api/complete?q={query}`
//! - `GET /search?q={query}` (full search results page — hard requirement §17.8)

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 4: GET /search?q={query} — full search results page.
/// Hard requirement per §17.8 (currently missing from live sites).
#[allow(dead_code)]
pub async fn search_page() -> impl IntoResponse {
    todo!("Phase 4: implement full search results page");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 4: GET /api/search?q={query} — JSON search API.
#[allow(dead_code)]
pub async fn search_api() -> impl IntoResponse {
    todo!("Phase 4: migrate search_api handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 4: GET /api/complete?q={query} — autocomplete endpoint.
#[allow(dead_code)]
pub async fn search_complete() -> impl IntoResponse {
    todo!("Phase 4: migrate search_complete handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}
