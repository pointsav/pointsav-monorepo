//! Mobile chrome — bottom navigation bar with safe-area insets.
//!
//! Phase 1 stub. Phase 3 implements the mobile bottom bar.
//!
//! L24 enforcement: every fixed/sticky bottom chrome element must use
//! `calc(N + env(safe-area-inset-bottom))` — bare `padding-bottom: 56px`
//! on bottom chrome is a lint error. The CSS for this bar lives in the
//! `## 4. Chrome` section of `static/style.css`.
//!
//! The bottom bar contains:
//! - Home icon → `/`
//! - Search icon → opens Cmd+K palette
//! - Article (current) indicator
//! - Menu icon → side navigation drawer (Phase 3)
//!
//! `viewport-fit=cover` is required in the viewport meta tag (L17).

use maud::Markup;

/// Render the mobile bottom navigation bar.
///
/// Phase 1 stub. Phase 3 emits the full bottom bar HTML with:
/// - `position: fixed; bottom: 0` container
/// - `padding-bottom: calc(12px + env(safe-area-inset-bottom))` (L24)
/// - Thumb-zone navigation icons
/// - `overscroll-behavior: contain` on the scroll container
#[allow(dead_code)]
pub fn mobile_chrome() -> Markup {
    todo!("Phase 3: implement mobile bottom bar with safe-area insets (L24)")
}
