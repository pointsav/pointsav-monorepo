//! Authentication routes.
//!
//! Phase 6 target (gated on BP5 clearance).
//!
//! Routes owned by this module:
//! - `GET  /auth/login`  — login page
//! - `POST /auth/login`  — process login form (argon2id verification)
//! - `GET  /auth/logout` — destroy session cookie
//!
//! NOTE: These routes are currently wired through `server::router()` via the
//! delegation in `routes/mod.rs`. The real implementations live in
//! `crate::auth`. Auth is currently mounted at `/special/login` and
//! `/special/logout` in the server router; `/auth/*` is the Phase 6 target
//! path. Phase 6 gates on Q1 and BP5.

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 6: GET /auth/login — login page.
///
/// Real implementation: `crate::auth::get_login`.
#[allow(dead_code)]
pub async fn login_page() -> impl IntoResponse {
    // Phase 6: migrate get_login handler from crate::auth.
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 6: POST /auth/login — process login form.
///
/// Verifies argon2id password hash, sets session cookie.
/// Real implementation: `crate::auth::post_login`.
#[allow(dead_code)]
pub async fn login_post() -> impl IntoResponse {
    // Phase 6: migrate post_login handler from crate::auth.
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 6: GET /auth/logout — destroy session.
///
/// Clears the session cookie and redirects to /.
/// Real implementation: `crate::auth::post_logout`.
#[allow(dead_code)]
pub async fn logout() -> impl IntoResponse {
    // Phase 6: migrate post_logout handler from crate::auth.
    StatusCode::NOT_IMPLEMENTED
}
