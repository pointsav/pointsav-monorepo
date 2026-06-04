//! Admin routes.
//!
//! Phase 1 stub. Phase 4/6 migrates admin handlers from `server.rs`.
//!
//! Routes owned by this module (Phase 4/6 target):
//! - `GET /admin/pending` — pending edit review queue
//! - `POST /admin/pending/{id}/{action}` — approve or reject a pending edit

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 6: GET /admin/pending — list pending edits for review.
#[allow(dead_code)]
pub async fn get_pending() -> impl IntoResponse {
    todo!("Phase 6: migrate admin pending handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 6: POST /admin/pending/{id}/{action} — approve or reject edit.
#[allow(dead_code)]
pub async fn post_pending_action() -> impl IntoResponse {
    todo!("Phase 6: migrate admin pending action handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}
