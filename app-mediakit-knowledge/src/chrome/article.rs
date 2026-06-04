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
use crate::mounts::Mount;
use crate::render::{extract_headings, render_html_raw, inject_edit_pencils, Frontmatter};
use crate::server::AppState;

/// Render output bundle produced by the render pipeline.
pub struct RenderOutput {
    /// Rendered HTML body (Markdown → HTML; wikilinks resolved).
    pub html: String,
    /// Extracted table-of-contents entries for the right-rail TOC.
    pub toc: Vec<TocEntry>,
    /// Slugs referenced by `[[wikilinks]]` in the source — used for dead-link detection.
    pub wikilinks: Vec<String>,
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

/// Render a Markdown article body to HTML, extracting TOC entries and tracking
/// wikilink slugs referenced by the source.
///
/// This is the canonical render entry-point for the modular pipeline. It wraps
/// the existing `render_html_raw` + `extract_headings` + `inject_edit_pencils`
/// pipeline and additionally collects the set of `[[wikilink]]` slugs referenced
/// in the rendered output — used by dead-link detection in Phase 3.
///
/// # Parameters
/// - `content` — raw Markdown body (no frontmatter)
/// - `meta` — article frontmatter (used to plumb future layout options)
/// - `mounts` — federated content mounts used to resolve wikilink existence
pub fn render_page(content: &str, meta: &Frontmatter, mounts: &[Mount]) -> RenderOutput {
    // Collect extra roots (all mounts except the first) for wikilink resolution.
    let extra_roots: Vec<&std::path::Path> = mounts.iter().map(|m| m.path.as_path()).collect();
    let (primary, extra) = match extra_roots.as_slice() {
        [] => (std::path::Path::new("."), &[][..]),
        [first, rest @ ..] => (*first, rest),
    };

    // Render HTML and extract headings from the raw (pre-pencil) output.
    let raw_html = render_html_raw(content, primary, extra);
    let headings = extract_headings(&raw_html);

    // Collect wikilink slugs from the rendered HTML by scanning for the data
    // attribute emitted by comrak's wikilinks extension. Both resolved and
    // unresolved wikilinks carry this attribute; the `wikilink-missing` class
    // marks unresolved ones.
    let wikilinks = collect_wikilink_slugs(&raw_html);

    let toc: Vec<TocEntry> = headings
        .into_iter()
        .map(|(id, text, level)| TocEntry { level, text, id })
        .collect();

    let html = inject_edit_pencils(&raw_html);

    // Suppress unused warning when `meta` carries no layout branches yet.
    let _ = meta;

    RenderOutput { html, toc, wikilinks }
}

/// Extract the set of unique wikilink target slugs from rendered HTML.
///
/// comrak's wikilinks extension emits `data-wikilink="true"` on every anchor
/// it produces. The `inject_wiki_prefixes` post-processor rewrites each href
/// to `/wiki/<slug>` and adds `class="wikilink"` or `class="wikilink-missing"`.
/// This scanner reads back those normalised slugs from the `href` attribute.
fn collect_wikilink_slugs(html: &str) -> Vec<String> {
    const MARKER: &str = " data-wikilink=\"true\">";
    const HREF_PREFIX: &str = "/wiki/";
    let mut slugs = Vec::new();
    let mut rest = html;
    while let Some(pos) = rest.find(MARKER) {
        let before = &rest[..pos];
        if let Some(href_pos) = before.rfind("href=\"") {
            let after_href = &before[href_pos + 6..];
            // href value ends at the closing `"`
            if let Some(quote_end) = after_href.find('"') {
                let href_val = &after_href[..quote_end];
                let slug = href_val
                    .strip_prefix(HREF_PREFIX)
                    .unwrap_or(href_val)
                    .to_string();
                if !slug.is_empty() && !slugs.contains(&slug) {
                    slugs.push(slug);
                }
            }
        }
        rest = &rest[pos + MARKER.len()..];
    }
    slugs
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
