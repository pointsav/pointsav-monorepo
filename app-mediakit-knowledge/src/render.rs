//! Markdown rendering with frontmatter parsing.
//!
//! The frontmatter schema is documented in ARCHITECTURE.md §6. Phase
//! 1 reads only the fields needed for rendering chrome (title); the
//! rest are captured as a flat `extra` map for later phases (linter,
//! disclosure-mode validation, citation-graph).

use comrak::{markdown_to_html, Options};
use serde::Deserialize;
use std::collections::BTreeMap;

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
pub fn render_html(body_md: &str) -> String {
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
}
