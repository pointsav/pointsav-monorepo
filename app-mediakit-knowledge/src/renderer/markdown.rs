use super::{ArticleMeta, Reference, RenderedArticle};
use crate::renderer::{footnotes, toc, wikilinks};
use anyhow::{Context, Result};
use pulldown_cmark::{html, Options, Parser};

/// Parse raw file content (YAML front matter + Markdown body) into a
/// fully rendered RenderedArticle.
///
/// Pipeline:
///   1. Extract YAML front matter via gray_matter
///   2. Pre-process [[slug]] wikilinks → standard Markdown links
///   3. Parse Markdown with pulldown-cmark; extract TOC headings simultaneously
///   4. TODO: syntax-highlight fenced code blocks via syntect
///   5. Process [^n] footnote events → Wikipedia-style superscript anchors
///   6. Render HTML; build bibliography from front matter reference definitions
pub fn render(raw: &str, page_index: &wikilinks::PageIndex) -> Result<RenderedArticle> {
    // Stage 1 — front matter
    let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
    let parsed = matter.parse(raw);
    let meta: ArticleMeta = parsed
        .data
        .map(|d| d.deserialize())
        .transpose()
        .context("failed to deserialise front matter")?
        .unwrap_or_default();

    // Stage 2 — wikilinks
    let body_md = wikilinks::resolve(&parsed.content, page_index);

    // Stage 3 — Markdown parse + TOC extraction
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;

    let parser = Parser::new_ext(&body_md, options);
    let (toc_entries, events) = toc::extract(parser);

    // Stage 4 — syntax highlighting
    // TODO: intercept CodeBlock events, apply syntect HTML highlighter, re-emit.
    // Reference: https://github.com/trishume/syntect
    // Until implemented, code blocks render as plain <pre><code>.
    let events = events;

    // Stage 5 — footnote processing
    let (events, inline_anchors) = footnotes::process_inline(events, &meta.references);

    // Render HTML
    let mut body_html = String::with_capacity(body_md.len() * 2);
    html::push_html(&mut body_html, events.into_iter());

    // Build bibliography
    let references = footnotes::build_bibliography(&meta.references, &inline_anchors);
    if !references.is_empty() {
        body_html.push_str(&footnotes::render_bibliography(&references));
    }

    Ok(RenderedArticle { meta, body_html, toc: toc_entries, references })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::wikilinks::PageIndex;
    use std::collections::HashMap;

    fn empty_index() -> PageIndex { PageIndex(HashMap::new()) }

    #[test]
    fn renders_basic_article() {
        let raw = "---\ntitle: \"Test\"\nslug: test\ncategory: architecture\n---\nLead.\n\n## Section\n\nBody.\n";
        let r = render(raw, &empty_index()).unwrap();
        assert_eq!(r.meta.title, "Test");
        assert!(r.body_html.contains("Lead."));
    }

    #[test]
    fn missing_front_matter_uses_defaults() {
        let raw = "No front matter.\n\n## Heading\n";
        let r = render(raw, &empty_index()).unwrap();
        assert_eq!(r.meta.title, "");
    }
}
