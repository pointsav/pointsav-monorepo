//! Authentication routes.
//!
//! Phase 1 stub. Phase 6 migrates auth routes from `server.rs`.
//!
//! Routes owned by this module (Phase 6 target):
//! - `GET /auth/login`
//! - `POST /auth/login`
//! - `GET /auth/logout`

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 6: GET /auth/login — login page.
#[allow(dead_code)]
pub async fn get_login() -> impl IntoResponse {
    todo!("Phase 6: migrate login page handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 6: POST /auth/login — process login form.
#[allow(dead_code)]
pub async fn post_login() -> impl IntoResponse {
    todo!("Phase 6: migrate post_login handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 6: GET /auth/logout
#[allow(dead_code)]
pub async fn logout() -> impl IntoResponse {
    todo!("Phase 6: migrate logout handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}
