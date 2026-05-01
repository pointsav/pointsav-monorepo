//! Markdown rendering with frontmatter parsing.
//!
//! The frontmatter schema is documented in ARCHITECTURE.md §6. Phase
//! 1 reads only the fields needed for rendering chrome (title); the
//! rest are captured as a flat `extra` map for later phases (linter,
//! disclosure-mode validation, citation-graph).
//!
//! Phase 1.1 additions (additive — no removals):
//! - `hatnote`: optional italic note rendered above the article body
//! - `translations`: optional map of language code → slug for language switcher
//! - `categories`: optional list of category labels for footer rendering
//!
//! Iteration-2 additions (additive — no removals):
//! - `short_description`: one-sentence article summary; rendered as italic
//!   subtitle below the H1 (Wikipedia Vector 2022 article-subtitle pattern)

use comrak::{markdown_to_html, Options};
use serde::Deserialize;
use std::collections::BTreeMap;

/// Translation entry: language code (e.g. "es") → slug of sibling page.
pub type TranslationMap = BTreeMap<String, String>;

#[derive(Debug, Default, Deserialize)]
pub struct Frontmatter {
    #[serde(default)]
    pub title: Option<String>,

    #[serde(default)]
    pub slug: Option<String>,

    #[serde(default)]
    pub document_version: Option<String>,

    #[serde(default)]
    pub forward_looking: bool,

    #[serde(default)]
    pub disclosure_class: Option<String>,

    /// Italic note rendered at the top of the article body (above the infobox
    /// in source order, per Wikipedia hatnote convention). Phase 1.1 chrome.
    #[serde(default)]
    pub hatnote: Option<String>,

    /// Language code → slug map; drives the language-switcher button next to
    /// the title. Phase 1.1 chrome. Example: `{ es: "topic-hello.es" }`.
    #[serde(default)]
    pub translations: Option<TranslationMap>,

    /// Category labels for the end-of-article footer block. Phase 1.1 chrome.
    #[serde(default)]
    pub categories: Option<Vec<String>>,

    /// Home-page bucketing category per
    /// `content-wiki-documentation/.claude/rules/content-contract.md` §4.
    /// One of the 9 ratified categories (architecture, services, systems,
    /// applications, governance, infrastructure, company, reference, help)
    /// per naming-convention.md §10 Q5-A. The value `root` is reserved for
    /// `index.md` itself and is suppressed from category-panel bucketing.
    #[serde(default)]
    pub category: Option<String>,

    /// Date of the last meaningful edit in `YYYY-MM-DD` format.
    /// Drives the recent-additions feed on the home page. When absent,
    /// the engine falls back to git-commit-date via a shell-out to
    /// `git log -1 --format=%cI -- <path>`, then to filesystem mtime.
    #[serde(default)]
    pub last_edited: Option<String>,

    /// One-sentence article summary. Rendered as `<p class="topic-short-description"><em>…</em></p>`
    /// immediately below the article H1, matching Wikipedia Vector 2022's italic subtitle
    /// pattern. Also used in the featured-article panel on the home page.
    /// Omitted gracefully when absent.
    #[serde(default)]
    pub short_description: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

#[derive(Debug)]
pub struct ParsedPage {
    pub frontmatter: Frontmatter,
    pub body_md: String,
}

/// Split a Markdown file into frontmatter + body.
///
/// Frontmatter is delimited by lines containing only `---`. A file
/// without frontmatter is treated as body-only with a default
/// frontmatter struct.
pub fn parse_page(text: &str) -> Result<ParsedPage, serde_yaml::Error> {
    if let Some(rest) = text.strip_prefix("---\n") {
        if let Some(end_idx) = rest.find("\n---\n") {
            let yaml = &rest[..end_idx];
            let body = &rest[end_idx + "\n---\n".len()..];
            let fm: Frontmatter = serde_yaml::from_str(yaml)?;
            return Ok(ParsedPage {
                frontmatter: fm,
                body_md: body.to_string(),
            });
        }
    }
    Ok(ParsedPage {
        frontmatter: Frontmatter::default(),
        body_md: text.to_string(),
    })
}

/// Render Markdown body to HTML with wikilinks + GFM extensions enabled.
///
/// Phase 1.1: after the comrak pass, `inject_edit_pencils` walks the output
/// and inserts a right-floated `[edit]` anchor after every h2–h6 opening tag.
/// The anchors use `href="#"` placeholders; Phase 2 wires them to the edit
/// surface.
///
/// Callers that need to extract headings for TOC generation should call
/// `render_html_raw` first (for heading extraction), then `inject_edit_pencils`
/// for the final body HTML — or use the convenience wrapper pair
/// `render_html_with_toc`. The edit-pencil pass happens after heading
/// extraction so that TOC text is clean (no "[edit]" fragments).
pub fn render_html(body_md: &str) -> String {
    let raw = render_html_raw(body_md);
    inject_edit_pencils(&raw)
}

/// Like `render_html` but returns the raw comrak output without edit-pencil
/// injection. Use this as the input to `extract_headings` for TOC generation.
pub fn render_html_raw(body_md: &str) -> String {
    let mut options = Options::default();
    options.extension.wikilinks_title_after_pipe = true;
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.extension.autolink = true;
    options.extension.header_ids = Some("h-".to_string());
    // unsafe_ stays false; we don't want raw HTML from authors yet.
    options.render.unsafe_ = false;
    markdown_to_html(body_md, &options)
}

/// Walk rendered HTML and insert a right-floated `[edit]` span after every
/// h2–h6 opening tag (h1 is the page title — it gets its own tab chrome).
///
/// This is a straightforward string-level pass; a proper HTML parser is
/// overkill for a constrained tag set and would add a build dependency.
/// The transform is additive and idempotent when the edit-pencil class is
/// already present.
pub fn inject_edit_pencils(html: &str) -> String {
    const PENCIL: &str =
        r##"<span class="edit-pencil"><a href="#" title="Edit this section">[edit]</a></span>"##;

    let mut out = String::with_capacity(html.len() + 64);
    let mut rest = html;

    while !rest.is_empty() {
        // Look for any h2–h6 opening tag (comrak emits lowercase tags).
        let tag_start = rest
            .find("<h2")
            .into_iter()
            .chain(rest.find("<h3").into_iter())
            .chain(rest.find("<h4").into_iter())
            .chain(rest.find("<h5").into_iter())
            .chain(rest.find("<h6").into_iter())
            .min();

        match tag_start {
            None => {
                out.push_str(rest);
                break;
            }
            Some(pos) => {
                // Find the end of this opening tag so we can append the pencil
                // immediately inside the heading element (before its text).
                if let Some(close) = rest[pos..].find('>') {
                    let tag_end = pos + close + 1; // index after '>'
                    out.push_str(&rest[..tag_end]);
                    out.push_str(PENCIL);
                    rest = &rest[tag_end..];
                } else {
                    // Malformed — emit as-is and stop.
                    out.push_str(rest);
                    break;
                }
            }
        }
    }

    out
}

/// Extract a flat list of `(id, text, level)` heading triples from rendered
/// HTML for TOC generation.  Only h2–h6 are included (h1 is the page title).
///
/// comrak with `header_ids = Some(...)` emits an inner anchor inside the
/// heading element rather than putting the id on the heading tag itself,
/// e.g. `<h2><a id="h-alpha" ...></a>Alpha</h2>`. So this scan extracts the
/// id from anywhere inside the heading element, not just the opening tag.
/// Text is the heading content with nested tags stripped so the TOC shows
/// plain text only.
pub fn extract_headings(html: &str) -> Vec<(String, String, u8)> {
    let mut headings = Vec::new();
    let mut rest = html;

    loop {
        // Find the nearest h2–h6 opening tag.
        let candidates: Vec<_> = [
            (rest.find("<h2"), 2u8),
            (rest.find("<h3"), 3),
            (rest.find("<h4"), 4),
            (rest.find("<h5"), 5),
            (rest.find("<h6"), 6),
        ]
        .into_iter()
        .filter_map(|(pos, lvl)| pos.map(|p| (p, lvl)))
        .collect();

        let Some((pos, level)) = candidates.into_iter().min_by_key(|(p, _)| *p) else {
            break;
        };

        // Find the matching closing tag.
        let closing_tag = format!("</h{level}>");
        let Some(close_rel) = rest[pos..].find(&closing_tag) else {
            break;
        };
        let close_abs = pos + close_rel;
        let element_html = &rest[pos..close_abs];

        // Extract id from anywhere within the heading element (comrak puts it
        // on the inner <a> when header_ids is configured). Leading space
        // avoids false matches against attribute names ending in -id.
        let id = if let Some(id_start) = element_html.find(r#" id=""#) {
            let after = &element_html[id_start + 5..];
            if let Some(id_end) = after.find('"') {
                after[..id_end].to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Extract text by stripping inner tags. Content starts after the
        // first '>' (end of the heading's opening tag).
        let text = if let Some(content_start_rel) = element_html.find('>') {
            let content = &element_html[content_start_rel + 1..];
            content
                .split('<')
                .enumerate()
                .map(|(i, part)| {
                    if i == 0 {
                        part.to_string()
                    } else if let Some(gt) = part.find('>') {
                        part[gt + 1..].to_string()
                    } else {
                        String::new()
                    }
                })
                .collect::<String>()
                .trim()
                .to_string()
        } else {
            String::new()
        };

        if !id.is_empty() && !text.is_empty() {
            headings.push((id, text, level));
        }

        rest = &rest[close_abs + closing_tag.len()..];
    }

    headings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_page_with_frontmatter() {
        let text = "---\ntitle: Hello\n---\n# body\n";
        let parsed = parse_page(text).unwrap();
        assert_eq!(parsed.frontmatter.title.as_deref(), Some("Hello"));
        assert_eq!(parsed.body_md, "# body\n");
    }

    #[test]
    fn parses_page_without_frontmatter() {
        let text = "# body only\n";
        let parsed = parse_page(text).unwrap();
        assert!(parsed.frontmatter.title.is_none());
        assert_eq!(parsed.body_md, "# body only\n");
    }

    #[test]
    fn renders_wikilinks() {
        let html = render_html("see [[Other Page]] for context");
        assert!(html.contains("Other Page"), "wikilink text should be in output: {html}");
        assert!(html.contains("href"), "wikilink should produce an anchor: {html}");
    }

    #[test]
    fn renders_gfm_table() {
        let md = "| a | b |\n|---|---|\n| 1 | 2 |\n";
        let html = render_html(md);
        assert!(html.contains("<table>"), "GFM table should render: {html}");
    }

    // Phase 1.1 tests — additive; all existing tests remain unchanged.

    /// Edit pencils appear on h2+ but not on h1.
    #[test]
    fn edit_pencils_injected_on_h2_not_h1() {
        let md = "# Title\n\n## Section\n\ntext\n";
        let html = render_html(md);
        // The h1 should not carry an edit pencil.
        let h1_pos = html.find("<h1").unwrap();
        let h1_end = html[h1_pos..].find("</h1>").unwrap() + h1_pos;
        assert!(
            !html[h1_pos..h1_end].contains("edit-pencil"),
            "h1 should not have an edit pencil: {html}"
        );
        // The h2 should carry an edit pencil.
        assert!(
            html.contains("edit-pencil"),
            "h2 should have an edit pencil: {html}"
        );
    }

    /// Headings are extracted correctly from comrak output.
    #[test]
    fn extracts_headings_from_html() {
        let md = "## Alpha\n\ntext\n\n### Beta\n\nmore\n";
        let raw = render_html_raw(md);
        let headings = extract_headings(&raw);
        assert_eq!(headings.len(), 2, "should extract 2 headings: {:?}", headings);
        assert_eq!(headings[0].1, "Alpha");
        assert_eq!(headings[0].2, 2);
        assert_eq!(headings[1].1, "Beta");
        assert_eq!(headings[1].2, 3);
    }

    /// TOC text is clean — no "[edit]" fragments from pencil injection.
    #[test]
    fn toc_text_has_no_edit_fragments() {
        let md = "## A Section\n\ntext\n";
        let raw = render_html_raw(md);
        let headings = extract_headings(&raw);
        assert_eq!(headings.len(), 1);
        assert!(
            !headings[0].1.contains("[edit]"),
            "TOC text must not contain [edit]: {:?}",
            headings
        );
    }

    /// Hatnote and categories fields deserialise from frontmatter.
    #[test]
    fn parses_phase11_frontmatter_fields() {
        let text = "---\ntitle: Test\nhatnote: \"See elsewhere.\"\ncategories:\n  - Foo\n  - Bar\ntranslations:\n  es: test.es\n---\nbody\n";
        let parsed = parse_page(text).unwrap();
        assert_eq!(
            parsed.frontmatter.hatnote.as_deref(),
            Some("See elsewhere.")
        );
        let cats = parsed.frontmatter.categories.unwrap();
        assert_eq!(cats, vec!["Foo", "Bar"]);
        let trans = parsed.frontmatter.translations.unwrap();
        assert_eq!(trans.get("es").map(|s| s.as_str()), Some("test.es"));
    }

    /// `short_description` field deserialises from frontmatter.
    #[test]
    fn parses_short_description() {
        let text = "---\ntitle: Substrate\nshort_description: \"The five structural properties that define the platform.\"\n---\nbody\n";
        let parsed = parse_page(text).unwrap();
        assert_eq!(
            parsed.frontmatter.short_description.as_deref(),
            Some("The five structural properties that define the platform.")
        );
    }
}
