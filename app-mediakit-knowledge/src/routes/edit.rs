//! Edit routes.
//!
//! Phase 6 target (conditional on Q1 answer: is in-browser editing required?).
//!
//! Routes owned by this module:
//! - `GET  /edit/{*slug}`     — article editor page (CodeMirror 6 + SAA)
//! - `POST /api/edit/{*slug}` — submit article edit (atomic write + git commit)
//!
//! L25: `editor.js` (CodeMirror 6 + SAA) loads ONLY on these routes. All
//! other pages (article, home, search) load only `wiki.js`.
//!
//! NOTE: These routes are currently wired through `server::router()` via the
//! delegation in `routes/mod.rs`. The real implementations live in
//! `crate::edit` (edit.rs). Phase 6 gates on Q1 clearance.

use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Phase 6: GET /edit/{*slug} — article editor page.
///
/// Loads CodeMirror 6 + SAA for the article at {slug}. L25 enforcement:
/// editor.js is only referenced in the HTML emitted by this handler.
/// Real implementation: `crate::edit::get_edit`.
#[allow(dead_code)]
pub async fn edit_page() -> impl IntoResponse {
    // Phase 6: migrate get_edit handler from crate::edit.
    // Conditional on Q1 (in-browser editing requirement).
    StatusCode::NOT_IMPLEMENTED
}

/// Phase 6: POST /api/edit/{*slug} — submit article edit.
///
/// Accepts JSON body `{content: string}`, atomically writes to disk via
/// tempfile::NamedTempFile::persist, indexes via Tantivy, commits to git.
/// Real implementation: `crate::edit::post_edit`.
#[allow(dead_code)]
pub async fn submit_edit() -> impl IntoResponse {
    // Phase 6: migrate post_edit handler from crate::edit.
    StatusCode::NOT_IMPLEMENTED
}
