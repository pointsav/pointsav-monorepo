//! Article chrome — tabs, TOC, hatnote, infobox, status badge, related articles.
//!
//! Phase 1 stub. Phase 3 implements the full article page shell with:
//! - Wikipedia Vector 2022 tab model (Article / Talk / Edit / History)
//! - Scroll-spy right-rail TOC (L26 borrow from Stripe/Vercel docs)
//! - Hatnote block (italic, indented; from frontmatter `hatnote:`)
//! - Article status badge (`status:` → Seedling/In Development/Active/Evergreen)
//! - Category chip (clickable, links to category landing page)
//! - `relates_to` rendered as "See Also" sidebar card
//! - Backlinks portlet ("Referenced by N articles" from redb graph)
//! - Next/previous within category links (ordered by `position:`)

use maud::Markup;
use crate::server::AppState;

/// Render output bundle produced by the render pipeline.
pub struct RenderOutput {
    /// Rendered HTML body (Markdown → HTML; wikilinks resolved).
    pub html: String,
    /// Extracted table-of-contents entries for the right-rail TOC.
    pub toc: Vec<TocEntry>,
}

/// A single table-of-contents entry.
pub struct TocEntry {
    /// Heading level (1–6).
    pub level: u8,
    /// Heading display text (plain text, no HTML).
    pub text: String,
    /// CSS `id` attribute value for in-page anchor linking.
    pub id: String,
}

/// Render the full article page chrome wrapping the rendered article body.
///
/// Phase 1 stub. Phase 3 emits the full article template including tabs,
/// TOC rail, status badge, category chip, see-also, and backlinks portlet.
///
/// # Parameters
/// - `render` — output of `render::render_page()`
/// - `state` — shared application state (used for backlinks and navigation)
/// - `locale` — locale for chrome string localisation (L22)
#[allow(unused_variables)]
pub fn article_chrome(render: RenderOutput, state: &AppState, locale: &str) -> Markup {
    todo!("Phase 3: implement article_chrome — tabs, TOC, status badge, see-also, backlinks")
}
