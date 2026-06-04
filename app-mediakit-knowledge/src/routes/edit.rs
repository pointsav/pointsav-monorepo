//! Edit routes.
//!
//! Phase 1 stub. Phase 4 migrates edit handlers from `server.rs`.
//! Phase 6 adds CodeMirror + SAA editor (conditional on Q1).
//!
//! Routes owned by this module (Phase 4/6 target):
//! - `GET /edit/{*slug}`
//! - `POST /api/edit/{*slug}`
//! - `POST /create`

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 4/6: GET /edit/{*slug} — article editor page.
/// Loads `editor.js` (CodeMirror 6 + SAA). L25: only this route loads editor.js.
#[allow(dead_code)]
pub async fn get_edit() -> impl IntoResponse {
    todo!("Phase 6: migrate get_edit handler from server.rs (conditional on Q1)");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 4/6: POST /api/edit/{*slug} — submit article edit.
#[allow(dead_code)]
pub async fn post_edit() -> impl IntoResponse {
    todo!("Phase 6: migrate post_edit handler from server.rs (conditional on Q1)");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 4/6: POST /create — create new article.
#[allow(dead_code)]
pub async fn post_create() -> impl IntoResponse {
    todo!("Phase 6: migrate post_create handler from server.rs (conditional on Q1)");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}
