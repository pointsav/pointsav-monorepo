//! Route assembly — all HTTP routes visible in one place.
//!
//! Phase 1: delegates to `server::router()` while the route handlers live in
//! `server.rs`. Phase 4 migrates each handler group into its own submodule
//! here (`wiki.rs`, `home.rs`, `category.rs`, etc.) and retires `server.rs`.
//!
//! L20 compliance: no single file may exceed ~1,500 lines. The modular
//! layout enforces this structurally — each submodule owns one concern.

pub mod admin;
pub mod auth;
pub mod category;
pub mod edit;
pub mod feeds;
pub mod git;
pub mod home;
pub mod mcp;
pub mod search;
pub mod wiki;

use axum::Router;
use crate::state::AppState;

/// Assemble the full application router.
///
/// Phase 1: wraps `server::router()` unchanged. Phase 4 replaces this body
/// with submodule route registrations as each handler is migrated.
pub fn router(state: AppState) -> Router {
    // Delegate to the existing monolithic router until Phase 4 migration.
    crate::server::router(state)
}
