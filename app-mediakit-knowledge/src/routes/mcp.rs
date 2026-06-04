//! MCP JSON-RPC 2.0 route.
//!
//! Phase 7 target.
//!
//! Routes owned by this module:
//! - `POST /mcp` — MCP JSON-RPC 2.0 endpoint (behind --enable-mcp flag)
//!
//! The MCP route is mounted ONLY when `--enable-mcp` / `WIKI_ENABLE_MCP` is
//! set (L10). When the flag is absent, the route is not registered and POST /mcp
//! returns 404.
//!
//! NOTE: This route is currently wired through `server::router()` via the
//! delegation in `routes/mod.rs`. The real implementation lives in `crate::mcp`.
//! Phase 7 gates on the MCP protocol stabilisation milestone.

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 7: POST /mcp — MCP JSON-RPC 2.0 endpoint.
///
/// Only mounted when `--enable-mcp` is set. The MCP server exposes wiki
/// content via the Model Context Protocol — article read, search, and
/// link-graph tools. No vendor SDK; native JSON-RPC 2.0 implementation
/// per Doctrine claim #54 ("We Own It").
///
/// Real implementation: `crate::mcp::handler`.
#[allow(dead_code)]
pub async fn mcp_handler() -> impl IntoResponse {
    // Phase 7: migrate MCP handler from crate::mcp.
    // Only reachable when --enable-mcp is set.
    StatusCode::NOT_IMPLEMENTED
}
