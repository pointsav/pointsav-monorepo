//! Edit routes.
//!
//! Phase 6: in-browser editing with CodeMirror 6 + SAA. Wires route stubs
//! to real handler implementations in `crate::edit`.
//!
//! Routes owned by this module (registered in server::router):
//! - `GET  /edit/{*slug}`     — article editor page (CodeMirror 6 + SAA)
//! - `POST /edit/{*slug}`     — submit article edit (atomic write + git commit)
//!
//! L25 enforcement: `editor.js` (CodeMirror 6 + SAA) is referenced ONLY in
//! the HTML emitted by `crate::edit::get_edit`. All other pages (article, home,
//! search, category) load only `wiki.js`. The L25 gate is structural — the
//! script tag appears only in the edit handler's output, not in the article
//! chrome (see `src/chrome/article.rs` `article_page` `is_edit_page` gate).

pub use crate::edit::get_edit as edit_page;
pub use crate::edit::post_edit as submit_edit;
