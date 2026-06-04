//! MCP JSON-RPC 2.0 route.
//!
//! Phase 1 stub. Phase 7 migrates the MCP endpoint from `server.rs`.
//! The MCP route is mounted only when `--enable-mcp` is set (L10).
//!
//! Routes owned by this module (Phase 7 target):
//! - `POST /mcp`

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 7: POST /mcp — MCP JSON-RPC 2.0 endpoint.
/// Only mounted when `--enable-mcp` / `WIKI_ENABLE_MCP` is set.
#[allow(dead_code)]
pub async fn mcp_handler() -> impl IntoResponse {
    todo!("Phase 7: migrate MCP handler from server.rs");
    #[allow(unreachable_code)]
    StatusCode::NOT_IMPLEMENTED
}
