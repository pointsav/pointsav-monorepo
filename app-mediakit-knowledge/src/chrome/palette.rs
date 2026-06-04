//! Command palette chrome — Cmd+K search-and-navigate dialog.
//!
//! Phase 1 stub. Phase 3 implements the Cmd+K palette using a `<dialog>`
//! element with keyboard shortcut wiring in `static/wiki.js`.
//!
//! The palette appears on all three instances. It provides:
//! - Live article title search (queries `/api/complete`)
//! - Category navigation shortcuts
//! - Recent articles (from localStorage history)
//!
//! Borrowed from Tailwind CSS docs pattern (§13 borrow list, P0).

use maud::Markup;

/// Render the Cmd+K command palette `<dialog>` element.
///
/// The dialog is injected into every page by `base_chrome()` and activated
/// by the keyboard shortcut handler in `wiki.js`. It is hidden by default
/// (`<dialog>` closed state).
///
/// Phase 1 stub. Phase 3 emits the full `<dialog>` with search input,
/// results list, and ARIA attributes.
pub fn cmd_palette() -> Markup {
    todo!("Phase 3: implement Cmd+K command palette dialog")
}
