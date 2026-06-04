//! Git history and smart-HTTP routes.
//!
//! Phase 1 stub. Phase 5 migrates git routes from `server.rs`.
//!
//! Routes owned by this module (Phase 5 target):
//! - `GET /git/{slug}` — article history page (gix blame/diff)
//! - `GET /git-server/{tenant}/...` — read-only git smart-HTTP protocol

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 5: GET /git/{slug} — article history page.
#[allow(dead_code)]
pub async fn git_history() -> impl IntoResponse {
    todo!("Phase 5: migrate git history handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 5: GET /git-server/{tenant}/... — read-only git smart-HTTP.
#[allow(dead_code)]
pub async fn git_server() -> impl IntoResponse {
    todo!("Phase 5: migrate git_protocol handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}
