//! Admin routes.
//!
//! Phase 6 target.
//!
//! Routes owned by this module:
//! - `GET  /admin/pending`               — pending edit review queue
//! - `POST /admin/pending/{id}/{action}` — approve or reject a pending edit
//!
//! NOTE: These routes are currently wired through `server::router()` via the
//! delegation in `routes/mod.rs`. The real implementations live in
//! `crate::pending`. Phase 6 gates on Q1 clearance and BP5 auth clearance.

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 6: GET /admin/pending — pending edit review queue.
///
/// Lists all pending edits awaiting review. Requires admin session cookie.
/// Real implementation: `crate::pending::review_queue`.
#[allow(dead_code)]
pub async fn pending_list() -> impl IntoResponse {
    // Phase 6: migrate review_queue handler from crate::pending.
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 6: POST /admin/pending/{id}/{action} — approve or reject edit.
///
/// Action is one of: `accept` | `reject`. Requires admin session cookie.
/// Real implementation: `crate::pending::accept_edit` / `reject_edit`.
#[allow(dead_code)]
pub async fn pending_action() -> impl IntoResponse {
    // Phase 6: migrate accept/reject handlers from crate::pending.
    StatusCode::NOT_IMPLEMENTED
}
