// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Markdown rendering.
//!
//! Bodies are CommonMark (via comrak) with one platform extension: `[[slug]]`
//! and `[[slug|label]]` wikilinks resolve to internal `/wiki/{slug}` anchors.
//! Section headings (h2/h3) are extracted for the table of contents.

use std::collections::HashMap;
use std::sync::OnceLock;

use comrak::options::Plugins;
use comrak::plugins::syntect::{SyntectAdapter, SyntectAdapterBuilder};
use comrak::{markdown_to_html_with_plugins, Options};
use syntect::highlighting::ThemeSet;
use syntect::html::{css_for_theme_with_class_style, ClassStyle};

use super::walk::{ContentIndex, Lang};
use crate::citations::CitationRegistry;

/// Syntax highlighter, built once. Class-based output (no inline colours) so the
/// palette can switch with the page theme via `syntax_css()`.
fn highlighter() -> &'static SyntectAdapter {
    static ADAPTER: OnceLock<SyntectAdapter> = OnceLock::new();
    // No theme set → the adapter emits CSS classes (not inline colours),
    // which `syntax_css()` themes for light and dark.
    ADAPTER.get_or_init(|| SyntectAdapterBuilder::new().build())
}

/// Token-colour CSS for code blocks: a light theme by default and a dark theme
/// scoped under `html[data-theme="dark"]`, so code follows the page like every
/// modern docs site. Backgrounds are stripped — the panel background is a token
/// (`--k-code-block-bg`), light or dark per mode. Generated once.
pub fn syntax_css() -> &'static str {
    static CSS: OnceLock<String> = OnceLock::new();
    CSS.get_or_init(|| {
        let ts = ThemeSet::load_defaults();
        let light = ts
            .themes
            .get("InspiredGitHub")
            .and_then(|t| css_for_theme_with_class_style(t, ClassStyle::Spaced).ok())
            .unwrap_or_default();
        let dark = ts
            .themes
            .get("base16-ocean.dark")
            .and_then(|t| css_for_theme_with_class_style(t, ClassStyle::Spaced).ok())
            .unwrap_or_default();
        format!(
            "/* light */\n{}\n/* dark */\n{}\n",
            transform_theme_css(&light, None),
            transform_theme_css(&dark, Some("html[data-theme=\"dark\"] ")),
        )
    })
}

/// Strip background rules and, optionally, prefix every selector with a scope so
/// a theme only applies in that mode. Keeps foreground token colours only.
fn transform_theme_css(css: &str, scope: Option<&str>) -> String {
    // Drop the leading header comment syntect emits.
    let css = match (css.find("/*"), css.find("*/")) {
        (Some(0), Some(end)) => &css[end + 2..],
        _ => css,
    };
    let no_bg: String = css
        .lines()
        .filter(|l| !l.trim_start().starts_with("background-color"))
        .collect::<Vec<_>>()
        .join("\n");
    let Some(prefix) = scope else { return no_bg };
    let mut out = String::new();
    for rule in no_bg.split_inclusive('}') {
        match rule.find('{') {
            Some(b) if !rule[..b].trim().is_empty() => {
                let (sel, body) = rule.split_at(b);
                let scoped = sel
                    .split(',')
                    .map(|s| format!("{prefix}{}", s.trim()))
                    .filter(|s| !s.trim().is_empty())
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&scoped);
                out.push(' ');
                out.push_str(body);
            }
            _ => out.push_str(rule),
        }
    }
    out
}

/// A rendered document body plus its heading outline.
#[derive(Debug, Clone)]
pub struct Rendered {
    pub html: String,
    pub headings: Vec<Heading>,
}

/// One section heading for the table of contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pub level: u8,
    pub id: String,
    pub text: String,
}

/// Render a Markdown body to HTML, resolving wikilinks and collecting headings.
///
/// `index`/`lang` let unlabeled `[[slug]]` wikilinks resolve to the target's
/// real title (see `resolve_wikilinks`) — every call site already has both
/// on hand (`ContentIndex` is built at startup, before any request-time
/// rendering happens).
pub fn render(body_md: &str, index: &ContentIndex, lang: Lang) -> Rendered {
    let with_links = resolve_wikilinks(body_md, index, lang);
    // Strip Kramdown/Hugo-style `{#id}` heading attributes before either comrak
    // or `extract_headings` sees the text — this engine has no extension that
    // understands that syntax, so left alone it renders as literal text (real
    // bug: Index Topic content authored with `## Group {#group-count-N}`).
    // Applying the strip here, not only inside `extract_headings`, keeps the
    // rendered `<h2>` and the TOC entry in lockstep.
    let with_links = strip_heading_attrs(&with_links);

    let mut opts = Options::default();
    opts.extension.strikethrough = true;
    opts.extension.table = true;
    opts.extension.autolink = true;
    opts.extension.tasklist = true;
    opts.extension.footnotes = true;
    // `header_ids` was renamed `header_id_prefix` (comrak 0.52) — the old
    // field is deprecated and, confirmed live, no longer actually emits an
    // `id` attribute at all (every heading came back with `id=""` until
    // this was caught by a test failure). Empty prefix matches the
    // pre-rename behavior (no prefix on generated ids).
    opts.extension.header_id_prefix = Some(String::new());
    opts.render.r#unsafe = true; // content is trusted (Git-authored, reviewed)

    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(highlighter());

    let html = markdown_to_html_with_plugins(&with_links, &opts, &plugins);
    // Extract headings from comrak's own rendered output (not by re-deriving
    // ids from raw markdown) — comrak's Anchorizer disambiguates duplicate
    // headings with numeric suffixes and strips inline markup differently
    // than a naive slugify, so a separately-computed id silently diverges
    // from the id actually present in the HTML the TOC links point at.
    let headings = extract_headings_from_html(&html);
    Rendered { html, headings }
}

/// Render a full document. When the frontmatter carries a `references:` list, it
/// appends synthesized footnote definitions (`[^id]: text <url>`) so the body's
/// `[^id]` markers resolve into a rendered reference list instead of dead text.
pub fn render_doc(doc: &super::frontmatter::ParsedDoc, index: &ContentIndex, lang: Lang) -> Rendered {
    if doc.frontmatter.references.is_empty() {
        return render(&doc.body_md, index, lang);
    }
    let mut body = doc.body_md.clone();
    body.push_str("\n\n");
    for r in &doc.frontmatter.references {
        body.push_str(&format!("[^{}]: {}", r.id, r.text));
        if let Some(u) = r.url.as_deref().filter(|u| !u.is_empty()) {
            body.push_str(&format!(" <{u}>"));
        }
        body.push('\n');
    }
    render(&body, index, lang)
}

/// Render a JOURNAL document (SPEC-journal-wiki-render-contract.md §1):
/// resolve bracket-ID citations against `registry`, then append a generated
/// `## References` section in first-appearance order — the author never
/// writes one (§1.2 item 3). Citations resolve before comrak sees the body,
/// same as `resolve_wikilinks` in `render()`. Unresolved citation ids are
/// still numbered and linked (a publish-gate finding, not a render failure —
/// see `CitationRegistry::check_citation_gate`) so a broken paper still
/// renders with the problem visible, not a blank page.
pub fn render_journal_doc(
    doc: &super::frontmatter::ParsedDoc,
    registry: &CitationRegistry,
    index: &ContentIndex,
    lang: Lang,
) -> Rendered {
    let (resolved_body, order, unresolved) = resolve_citations(&doc.body_md, registry);
    for id in &unresolved {
        tracing::warn!(
            "JOURNAL citation [{id}] does not resolve in citations.yaml — publish-gate finding, SPEC §7"
        );
    }
    let mut body = resolved_body;
    if !order.is_empty() {
        body.push_str("\n\n## References\n\n");
        for (i, id) in order.iter().enumerate() {
            let n = i + 1;
            let line = match registry.get(id) {
                Some(entry) => entry.bibliography_line(),
                None => format!("*Unresolved citation: `{id}`.*"),
            };
            body.push_str(&format!("{n}. <a id=\"ref-{n}\"></a>{line}\n"));
        }
    }
    render(&body, index, lang)
}

/// Replace bracket-ID citation tokens per SPEC-journal-wiki-render-contract.md
/// §1.1: `[id]` (single), `[id1][id2]` (adjacent — each independently
/// numbered; falls out of processing brackets one at a time, no special
/// casing needed), and `[id locator]` (pinpoint — the locator stays visible
/// after the linked number, not itself linked). Code-fence- and inline-code-
/// span-aware: a bracket inside ``` fences or a `code span` is left
/// untouched (`[u8; 32]` in a code sample is not a citation), matching the
/// bracket-hygiene rule in SPEC §1.2 item 8. Numbering is by first
/// appearance (IEEE-style), stable across renditions. Returns
/// (rewritten_body, first-appearance id order, ids that look like a citation
/// but don't resolve in `registry`). Exposed `pub` (not just crate-internal)
/// so the golden-fixture suite (`tests/journal_golden.rs`, SPEC §0.5) can
/// build its normalized comparison summary directly from the same id-order
/// data `render_journal_doc` uses, without re-parsing rendered HTML.
pub fn resolve_citations(
    md: &str,
    registry: &CitationRegistry,
) -> (String, Vec<String>, Vec<String>) {
    let mut order: Vec<String> = Vec::new();
    let mut numbers: HashMap<String, usize> = HashMap::new();
    let mut unresolved: Vec<String> = Vec::new();
    let mut out = String::with_capacity(md.len());
    let mut in_fence = false;
    for line in md.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            out.push_str(line);
            continue;
        }
        if in_fence {
            out.push_str(line);
            continue;
        }
        resolve_citations_in_line(
            line,
            registry,
            &mut order,
            &mut numbers,
            &mut unresolved,
            &mut out,
        );
    }
    (out, order, unresolved)
}

/// Single-line pass for `resolve_citations` — tracks inline code spans
/// (`` ` ``) so a citation-shaped bracket inside one is left untouched, same
/// rationale as the fence check one level up.
#[allow(clippy::too_many_arguments)]
fn resolve_citations_in_line(
    line: &str,
    registry: &CitationRegistry,
    order: &mut Vec<String>,
    numbers: &mut HashMap<String, usize>,
    unresolved: &mut Vec<String>,
    out: &mut String,
) {
    let bytes = line.as_bytes();
    let mut i = 0;
    let mut in_code_span = false;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'`' {
            in_code_span = !in_code_span;
            out.push('`');
            i += 1;
            continue;
        }
        if !in_code_span && b == b'[' {
            let escaped = i > 0 && bytes[i - 1] == b'\\';
            // A `[[...` (wikilink) or `[^...` (footnote marker) is never a
            // citation bracket — leave both for their own resolvers.
            let other_bracket_form = i + 1 < bytes.len() && matches!(bytes[i + 1], b'[' | b'^');
            if !escaped && !other_bracket_form {
                if let Some(close_rel) = line[i + 1..].find(']') {
                    let close_abs = i + 1 + close_rel;
                    let inner = &line[i + 1..close_abs];
                    let (token, locator) = match inner.split_once(char::is_whitespace) {
                        Some((t, l)) => (t, Some(l.trim())),
                        None => (inner, None),
                    };
                    // A real markdown link whose text happens to look like an
                    // id (e.g. someone hand-linking `[rfc-9162](url)`) must
                    // still be left alone. NOT checked against a following
                    // `[` — that's the adjacent-citation form `[id1][id2]`
                    // (SPEC §1.1), which must resolve both brackets.
                    let followed_by_link_syntax = line[close_abs + 1..].starts_with('(');
                    if !followed_by_link_syntax && is_citation_id_shape(token) {
                        let n = *numbers.entry(token.to_string()).or_insert_with(|| {
                            order.push(token.to_string());
                            order.len()
                        });
                        if registry.get(token).is_none() && !unresolved.iter().any(|u| u == token) {
                            unresolved.push(token.to_string());
                        }
                        out.push_str(&citation_link_markdown(n, locator));
                        i = close_abs + 1;
                        continue;
                    }
                }
            }
        }
        let ch_len = utf8_len(bytes[i]);
        out.push_str(&line[i..i + ch_len]);
        i += ch_len;
    }
}

/// `citation-id` per SPEC §1.1: `^[a-z0-9][a-z0-9-]*$`. Anything else (an
/// ordinary bracketed word, a footnote-style label, etc.) is left as plain
/// text — this is the guard that keeps the resolver from touching prose
/// brackets that were never meant to be citations.
fn is_citation_id_shape(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() || c.is_ascii_digit() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Markdown for one resolved citation occurrence: a linked, numbered
/// `[n]` — literal brackets are backslash-escaped so they survive as literal
/// characters around the link rather than being parsed as another link's
/// delimiters. A pinpoint locator (SPEC §1.1) stays visible, plain text,
/// inside the brackets but outside the link.
fn citation_link_markdown(number: usize, locator: Option<&str>) -> String {
    match locator {
        Some(loc) if !loc.is_empty() => format!("\\[[{number}](#ref-{number}), {loc}\\]"),
        _ => format!("\\[[{number}](#ref-{number})\\]"),
    }
}

/// Replace `[[slug]]` / `[[slug|label]]` with Markdown links to `/wiki/slug`.
/// A leading `#` in the target (e.g. `[[#section]]`) is treated as a same-page
/// anchor. Escaped `\[[` is left untouched.
/// Resolve `[[slug]]`/`[[slug|Label]]` wikilinks to real `/wiki/{slug}` links.
///
/// Code-fence- and inline-code-span-aware (Command/project-editorial finding,
/// 2026-08-26): runs before comrak sees the body, so without this a
/// `` `[[wikilink]]` `` written as a literal syntax example got corrupted
/// into a real (broken) link. Same per-line fence tracking + per-character
/// code-span tracking as `resolve_citations`/`resolve_citations_in_line`.
///
/// An unlabeled `[[slug]]` resolves its label to the target's real title via
/// `index` — previously the raw slug itself was used as the label (e.g.
/// `[[us-tx-plano]]` rendered literally as "us-tx-plano" instead of "Plano,
/// Texas"), the one place in the engine this asymmetry existed vs. routes
/// that already had `ContentIndex` access. Falls back to the raw slug text
/// when the target doesn't resolve (unknown/removed slug) — same
/// degrade-not-break behavior as before this fix, not a new failure mode.
fn resolve_wikilinks(md: &str, index: &ContentIndex, lang: Lang) -> String {
    let mut out = String::with_capacity(md.len());
    let mut in_fence = false;
    for line in md.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            out.push_str(line);
            continue;
        }
        if in_fence {
            out.push_str(line);
            continue;
        }
        resolve_wikilinks_in_line(line, index, lang, &mut out);
    }
    out
}

fn resolve_wikilinks_in_line(line: &str, index: &ContentIndex, lang: Lang, out: &mut String) {
    let bytes = line.as_bytes();
    let mut i = 0;
    let mut in_code_span = false;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'`' {
            in_code_span = !in_code_span;
            out.push('`');
            i += 1;
            continue;
        }
        if !in_code_span && b == b'[' && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            // Guard against escaped `\[[`.
            let escaped = i > 0 && bytes[i - 1] == b'\\';
            if !escaped {
                if let Some(close) = line[i + 2..].find("]]") {
                    let inner = &line[i + 2..i + 2 + close];
                    let (target, label): (&str, String) = match inner.split_once('|') {
                        Some((t, l)) => (t.trim(), l.trim().to_string()),
                        None => {
                            let t = inner.trim();
                            let resolved = index
                                .resolve(&slugify(t), lang)
                                .map(|d| d.title.clone());
                            (t, resolved.unwrap_or_else(|| t.to_string()))
                        }
                    };
                    let href = if let Some(anchor) = target.strip_prefix('#') {
                        // A throwaway Anchorizer, not the shared one `extract_headings`
                        // uses — this can't perfectly replicate cross-document dedup
                        // numbering for a same-page link targeting the *second* of two
                        // identically-worded headings, but neither did the old
                        // `slugify`-based version; no regression, just not a new fix.
                        format!("#{}", comrak::Anchorizer::new().anchorize(anchor))
                    } else {
                        // A bare `[[Zero Container Inference]]` (no `|label`) used
                        // to build the href from the raw bracket text verbatim —
                        // spaces and all — instead of the real slug. Slugifying is
                        // a no-op on an already-valid slug (explicit `[[slug|Label]]`
                        // form), so this is safe for both branches.
                        format!("/wiki/{}", slugify(target))
                    };
                    out.push('[');
                    out.push_str(&label);
                    out.push_str("](");
                    // CommonMark link destinations may not contain an
                    // unescaped space unless wrapped in angle brackets — a
                    // human-readable target like `[[Zero Container
                    // Inference]]` would otherwise render as literal text,
                    // not a link.
                    if href.contains(' ') {
                        out.push('<');
                        out.push_str(&href);
                        out.push('>');
                    } else {
                        out.push_str(&href);
                    }
                    out.push(')');
                    i = i + 2 + close + 2;
                    continue;
                }
            }
        }
        // Default: copy this byte through, respecting UTF-8 boundaries.
        let ch_len = utf8_len(bytes[i]);
        out.push_str(&line[i..i + ch_len]);
        i += ch_len;
    }
}

fn utf8_len(b: u8) -> usize {
    match b {
        0x00..=0x7f => 1,
        0xc0..=0xdf => 2,
        0xe0..=0xef => 3,
        _ => 4,
    }
}

/// Extract level-2/3 headings directly from comrak's rendered HTML — the
/// `id` on each `<h2>`/`<h3>` is whatever comrak's `header_ids` Anchorizer
/// actually assigned (including its numeric-suffix disambiguation for
/// duplicate headings), and `text` is the tag's inner content with any
/// nested markup (inline code, links) stripped and HTML entities decoded.
fn extract_headings_from_html(html: &str) -> Vec<Heading> {
    let mut headings = Vec::new();
    let mut rest = html;
    loop {
        let h2 = rest.find("<h2");
        let h3 = rest.find("<h3");
        let (level, pos) = match (h2, h3) {
            (Some(p2), Some(p3)) if p3 < p2 => (3u8, p3),
            (Some(p2), _) => (2u8, p2),
            (None, Some(p3)) => (3u8, p3),
            (None, None) => break,
        };
        let tag_onward = &rest[pos..];
        let Some(tag_end) = tag_onward.find('>') else {
            break;
        };
        let after_open = &tag_onward[tag_end + 1..];
        let close = format!("</h{level}>");
        let Some(close_pos) = after_open.find(&close) else {
            break;
        };
        let inner = &after_open[..close_pos];
        // comrak's header_id_prefix places the id on a nested, empty
        // `<a id="…" class="anchor" …></a>` inside the heading (immediately
        // before the visible text), not as an attribute on `<h2>`/`<h3>`
        // itself — confirmed live: `<h2><a href="#x" ... id="x"></a>Text</h2>`.
        let id = extract_attr(inner, "id").unwrap_or_default();
        let text = decode_entities(&strip_tags(inner));
        if !text.trim().is_empty() {
            headings.push(Heading {
                level,
                id,
                text: text.trim().to_string(),
            });
        }
        rest = &after_open[close_pos + close.len()..];
    }
    headings
}

/// Extract `attr="value"` from a raw opening-tag string (e.g. `<h2 id="x">`
/// without the trailing `>`).
fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let needle = format!("{attr}=\"");
    let start = tag.find(&needle)? + needle.len();
    let end = start + tag[start..].find('"')?;
    Some(tag[start..end].to_string())
}

/// Strip HTML tags, keeping only text content (used to recover a heading's
/// plain display text from its rendered, possibly inline-formatted, HTML).
fn strip_tags(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

/// Strip a trailing Kramdown/Hugo-style heading-attribute (`{#some-id}`) from
/// every ATX heading line in `md`, skipping fenced code blocks. This engine
/// has no extension that understands that syntax — comrak doesn't support it
/// and `header_ids` only auto-generates ids from heading text — so left alone
/// it renders as literal text in both the heading and the TOC.
fn strip_heading_attrs(md: &str) -> String {
    let mut out = String::with_capacity(md.len());
    let mut in_fence = false;
    for line in md.lines() {
        let t = line.trim_start();
        if t.starts_with("```") || t.starts_with("~~~") {
            in_fence = !in_fence;
            out.push_str(line);
            out.push('\n');
            continue;
        }
        if in_fence {
            out.push_str(line);
            out.push('\n');
            continue;
        }
        let level = t.bytes().take_while(|&b| b == b'#').count();
        if (1..=6).contains(&level) && t.as_bytes().get(level) == Some(&b' ') {
            out.push_str(&strip_trailing_heading_attr(line));
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

/// Decode the small set of HTML entities comrak actually emits in text nodes.
fn decode_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

/// If `line` ends with `{#id}` (id: ASCII alphanumerics/`_`/`-` only), strip it
/// and any whitespace immediately before it. Otherwise returns `line` as-is —
/// in particular, a heading that merely *mentions* `` `{#id}` `` inside
/// backticks is untouched (the line ends with a backtick, not `}`).
/// `pub(super)`: also used by `index_topic::parse_index_topic` to clean a
/// group heading's title independently of the shared `render()` pipeline.
pub(super) fn strip_trailing_heading_attr(line: &str) -> String {
    let trimmed_end = line.trim_end();
    if !trimmed_end.ends_with('}') {
        return line.to_string();
    }
    let Some(open) = trimmed_end.rfind("{#") else {
        return line.to_string();
    };
    let id_part = &trimmed_end[open + 2..trimmed_end.len() - 1];
    let valid_id = !id_part.is_empty()
        && id_part
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-');
    if !valid_id {
        return line.to_string();
    }
    trimmed_end[..open].trim_end().to_string()
}

/// Lowercase ASCII slug: alphanumerics kept, runs of other chars → single `-`.
pub fn slugify(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_dash = false;
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_basic_markdown() {
        let r = render(
            "# Title\n\nSome **bold** text.\n",
            &ContentIndex::default(),
            Lang::En,
        );
        assert!(r.html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn render_doc_turns_references_into_footnotes() {
        use super::super::frontmatter::{Frontmatter, ParsedDoc, Reference};
        let doc = ParsedDoc {
            frontmatter: Frontmatter {
                references: vec![Reference {
                    id: "1".into(),
                    text: "Yjs library.".into(),
                    url: Some("https://yjs.dev".into()),
                }],
                ..Default::default()
            },
            body_md: "This cites a source[^1].\n".into(),
        };
        let r = render_doc(&doc, &ContentIndex::default(), Lang::En);
        assert!(
            r.html.contains("footnotes"),
            "footnotes section: {}",
            r.html
        );
        assert!(r.html.contains("Yjs library."));
        assert!(
            !r.html.contains("[^1]"),
            "marker should be resolved, not literal"
        );
    }

    fn write(dir: &std::path::Path, rel: &str, body: &str) {
        let p = dir.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, body).unwrap();
    }

    /// A real, populated `ContentIndex` (one English article) for tests that
    /// need bare-`[[slug]]` title resolution to actually resolve to
    /// something — matches `walk.rs`'s own test convention (`ContentIndex`
    /// has no in-memory constructor other than a real filesystem walk).
    fn test_index() -> ContentIndex {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        write(
            &root,
            "architecture/zci.md",
            "---\ntitle: Zero-Container Inference\nslug: zero-container-inference\ncategory: architecture\n---\nBody\n",
        );
        let mounts = super::super::mount::MountSet {
            mounts: vec![super::super::mount::Mount {
                path: root,
                role: "primary".into(),
                blueprint_set: vec![],
            }],
        };
        // Leak the tempdir so its files outlive this function — the index
        // only stores paths/metadata, not open handles, so this is safe and
        // matches `tests/handlers.rs`'s own established leak-for-test-
        // duration convention.
        std::mem::forget(tmp);
        ContentIndex::build(&mounts)
    }

    #[test]
    fn resolves_wikilinks_with_and_without_label() {
        let r = render(
            "See [[zero-container-inference]] and [[yoyo-compute|GPU compute]].\n",
            &ContentIndex::default(),
            Lang::En,
        );
        assert!(r.html.contains(r#"href="/wiki/zero-container-inference""#));
        assert!(r.html.contains(r#"href="/wiki/yoyo-compute""#));
        assert!(r.html.contains(">GPU compute</a>"));
    }

    #[test]
    fn wikilink_target_with_a_space_slugifies_to_a_real_resolvable_href() {
        // A bare `[[Zero Container Inference]]` (no `|label`) used to build
        // the href from the raw bracket text verbatim — spaces and all,
        // percent-encoded by comrak into a URL no real slug ever matches
        // (`/wiki/Zero%20Container%20Inference`, a dead link). Real UX-review
        // finding, 2026-08-23: the target must be slugified like any other
        // slug, so the link actually resolves.
        let r = render(
            "See [[Zero Container Inference]].\n",
            &ContentIndex::default(),
            Lang::En,
        );
        assert!(
            r.html
                .contains("<a href=\"/wiki/zero-container-inference\""),
            "got: {}",
            r.html
        );
        // No index entry matches this target — falls back to the raw
        // bracket text as the label, same degrade-not-break behavior as
        // before the title-resolution fix below.
        assert!(r.html.contains(">Zero Container Inference</a>"));
    }

    #[test]
    fn bare_wikilink_resolves_label_to_the_real_title_not_the_raw_slug() {
        // Real bug, Command/project-editorial cross-model finding
        // (2026-08-26): a bare `[[slug]]` (no `|label`) rendered the raw
        // slug text itself as the visible label — e.g. `[[us-tx-plano]]`
        // showed literally "us-tx-plano" instead of "Plano, Texas" — on
        // every route except the one (`render_index_topic_category`) that
        // happened to already have `ContentIndex` access. 2,136 live
        // instances across 64% of pages on the 3 production wikis.
        let idx = test_index();
        let r = render("See [[zero-container-inference]] for background.\n", &idx, Lang::En);
        assert!(
            r.html.contains(">Zero-Container Inference</a>"),
            "expected the real title as the label, got: {}",
            r.html
        );
        assert!(
            !r.html.contains(">zero-container-inference</a>"),
            "raw slug must not leak through as the label, got: {}",
            r.html
        );
    }

    #[test]
    fn bare_wikilink_with_unresolvable_slug_falls_back_to_raw_text() {
        // Same degrade-not-break contract as the pre-fix behavior: an
        // unknown/removed slug must not panic or blank out, just fall back
        // to showing the raw bracket text as before.
        let r = render(
            "See [[this-slug-does-not-exist]] please.\n",
            &ContentIndex::default(),
            Lang::En,
        );
        assert!(r.html.contains(">this-slug-does-not-exist</a>"));
    }

    #[test]
    fn wikilink_inside_inline_code_or_fence_is_left_as_literal_text_not_a_link() {
        // Second bug in the same finding: resolve_wikilinks ran before
        // comrak, with no code-span/fence awareness, so a documentation
        // page using `[[wikilink]]` as a literal syntax example inside
        // backticks or a fenced block got corrupted into a real (broken)
        // link. Live example: documentation.pointsav.com/wiki/
        // substrate-native-compatibility — 151 instances across 77 files.
        let idx = test_index();
        let r = render(
            "Use `[[zero-container-inference]]` syntax like so:\n\n```\n[[zero-container-inference]]\n```\n",
            &idx,
            Lang::En,
        );
        assert!(
            !r.html.contains(r#"href="/wiki/zero-container-inference""#),
            "a wikilink inside code span/fence must not become a real link, got: {}",
            r.html
        );
        assert!(r.html.contains("[[zero-container-inference]]"));
    }

    #[test]
    fn extracts_h2_h3_headings_only() {
        let r = render("# H1\n\n## Why no containers\n\n### Detail\n\n#### Too deep\n", &ContentIndex::default(), Lang::En);
        let ids: Vec<_> = r.headings.iter().map(|h| h.id.as_str()).collect();
        assert_eq!(ids, vec!["why-no-containers", "detail"]);
        assert_eq!(r.headings[0].level, 2);
        assert_eq!(r.headings[1].level, 3);
    }

    #[test]
    fn headings_inside_code_fence_are_ignored() {
        let r = render("## Real\n\n```\n## Not a heading\n```\n", &ContentIndex::default(), Lang::En);
        assert_eq!(r.headings.len(), 1);
        assert_eq!(r.headings[0].id, "real");
    }

    #[test]
    fn duplicate_headings_get_comraks_actual_disambiguated_ids() {
        // The bug this guards: a naive local slugify produces "overview" for
        // BOTH headings, so a TOC built from it links #overview twice (the
        // second entry jumps to the first section). Extracting from
        // comrak's own output picks up its Anchorizer's real -1 suffix, so
        // the TOC ids the article() template renders always match what's
        // actually clickable in the body.
        let r = render("## Overview\n\ntext\n\n## Overview\n\nmore\n", &ContentIndex::default(), Lang::En);
        assert_eq!(r.headings.len(), 2);
        assert_ne!(
            r.headings[0].id, r.headings[1].id,
            "duplicate headings must not collide"
        );
        assert!(r.html.contains(&format!(r#"id="{}""#, r.headings[0].id)));
        assert!(r.html.contains(&format!(r#"id="{}""#, r.headings[1].id)));
    }

    #[test]
    fn heading_with_inline_markup_gets_matching_id_and_plain_text() {
        // A naive slugify over the raw markdown line would see the literal
        // `[...](...)`  syntax; the real id must come from the *rendered*
        // text, and the TOC's display text should be plain (no markup).
        let r = render("## See [the spec](/x)\n", &ContentIndex::default(), Lang::En);
        assert_eq!(r.headings.len(), 1);
        assert_eq!(r.headings[0].text, "See the spec");
        assert!(
            r.html.contains(&format!(r#"id="{}""#, r.headings[0].id)),
            "TOC id must match a real id in the rendered HTML"
        );
    }

    #[test]
    fn anchor_wikilink_stays_same_page() {
        let r = render("Jump to [[#Cold start|cold start]].\n", &ContentIndex::default(), Lang::En);
        assert!(r.html.contains(r##"href="#cold-start""##));
    }

    /// Build a `CitationRegistry` from inline YAML for a test — matches the
    /// tempfile-load pattern `citations.rs`'s own tests use, since
    /// `CitationRegistry` has no in-memory constructor.
    fn test_registry(yaml: &str) -> CitationRegistry {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("citations.yaml");
        std::fs::write(&path, yaml).unwrap();
        CitationRegistry::load(&path)
    }

    #[test]
    fn resolves_single_citation_with_linked_number() {
        let registry = test_registry(
            "citations:\n  rfc-9162:\n    type: technical-specification\n    title: RFC 9162\n    url: https://x\n",
        );
        let (body, order, unresolved) =
            resolve_citations("Certificate Transparency [rfc-9162] is a log.\n", &registry);
        assert_eq!(order, vec!["rfc-9162".to_string()]);
        assert!(unresolved.is_empty());
        assert!(body.contains(r"\[[1](#ref-1)\]"));
    }

    #[test]
    fn adjacent_multiple_citations_each_get_own_number() {
        let registry = test_registry(
            "citations:\n  a:\n    type: vendor-doc\n    title: A\n    url: https://x/a\n  b:\n    type: vendor-doc\n    title: B\n    url: https://x/b\n",
        );
        let (body, order, _) =
            resolve_citations("Supported by the literature [a][b].\n", &registry);
        assert_eq!(order, vec!["a".to_string(), "b".to_string()]);
        assert!(body.contains(r"\[[1](#ref-1)\]\[[2](#ref-2)\]"));
    }

    #[test]
    fn pinpoint_locator_stays_visible_outside_the_link() {
        let registry = test_registry(
            "citations:\n  ni-51-102:\n    type: regulatory-instrument\n    title: NI 51-102\n    url: https://x\n",
        );
        let (body, order, _) = resolve_citations("Per [ni-51-102 \u{a7}4A.2].\n", &registry);
        assert_eq!(order, vec!["ni-51-102".to_string()]);
        assert!(body.contains("[1](#ref-1), \u{a7}4A.2"));
    }

    #[test]
    fn same_id_cited_twice_reuses_its_first_appearance_number() {
        let registry = test_registry(
            "citations:\n  a:\n    type: vendor-doc\n    title: A\n    url: https://x/a\n  b:\n    type: vendor-doc\n    title: B\n    url: https://x/b\n",
        );
        let (body, order, _) =
            resolve_citations("First [b], then [a], then [b] again.\n", &registry);
        assert_eq!(order, vec!["b".to_string(), "a".to_string()]);
        assert!(body.contains(r"\[[1](#ref-1)\]"), "first [b] is ref 1");
        assert!(body.contains(r"\[[2](#ref-2)\]"), "[a] is ref 2");
        assert_eq!(
            body.matches(r"\[[1](#ref-1)\]").count(),
            2,
            "second [b] must reuse number 1, not mint a new one"
        );
    }

    #[test]
    fn citation_inside_code_fence_is_left_untouched() {
        let registry = test_registry("citations: {}\n");
        let (body, order, _) =
            resolve_citations("```\nlet x: [u8; 32] = [rfc-9162];\n```\n", &registry);
        assert!(order.is_empty());
        assert!(
            body.contains("[rfc-9162]"),
            "literal bracket text must survive: {body}"
        );
        assert!(!body.contains("#ref-1"));
    }

    #[test]
    fn citation_inside_inline_code_span_is_left_untouched() {
        let registry = test_registry("citations: {}\n");
        let (body, order, _) =
            resolve_citations("The type is `[u8; 32]`, not a citation.\n", &registry);
        assert!(order.is_empty());
        assert!(body.contains("`[u8; 32]`"));
    }

    #[test]
    fn ordinary_bracket_text_that_is_not_id_shaped_is_untouched() {
        let registry = test_registry("citations: {}\n");
        let (body, order, _) =
            resolve_citations("See [Note] below, and [Some Text] here.\n", &registry);
        assert!(order.is_empty());
        assert!(body.contains("[Note]"));
        assert!(body.contains("[Some Text]"));
    }

    #[test]
    fn real_markdown_link_with_id_shaped_text_is_untouched() {
        let registry = test_registry(
            "citations:\n  rfc-9162:\n    type: technical-specification\n    title: RFC 9162\n    url: https://x\n",
        );
        let (body, order, _) =
            resolve_citations("See [rfc-9162](https://example.com) directly.\n", &registry);
        assert!(
            order.is_empty(),
            "a real markdown link must not be treated as a citation"
        );
        assert!(body.contains("[rfc-9162](https://example.com)"));
    }

    #[test]
    fn unresolved_citation_is_still_numbered_and_flagged() {
        let registry = test_registry("citations: {}\n");
        let (body, order, unresolved) = resolve_citations("Per [nonexistent-id].\n", &registry);
        assert_eq!(order, vec!["nonexistent-id".to_string()]);
        assert_eq!(unresolved, vec!["nonexistent-id".to_string()]);
        assert!(body.contains(r"\[[1](#ref-1)\]"));
    }

    #[test]
    fn render_journal_doc_appends_generated_references_section() {
        use super::super::frontmatter::{Frontmatter, ParsedDoc};
        let registry = test_registry(
            "citations:\n  rfc-9162:\n    type: technical-specification\n    title: RFC 9162\n    url: https://x\n    authors: [\"IETF\"]\n    year: 2021\n",
        );
        let doc = ParsedDoc {
            frontmatter: Frontmatter::default(),
            body_md: "## 1. Introduction\n\nSee [rfc-9162] for details.\n".to_string(),
        };
        let r = render_journal_doc(&doc, &registry, &ContentIndex::default(), Lang::En);
        assert!(r.html.contains("References"));
        assert!(r.html.contains(r#"id="ref-1""#));
        assert!(r.html.contains("IETF"));
        assert!(r.html.contains("RFC 9162"));
    }

    #[test]
    fn render_journal_doc_omits_references_section_when_no_citations() {
        use super::super::frontmatter::{Frontmatter, ParsedDoc};
        let registry = test_registry("citations: {}\n");
        let doc = ParsedDoc {
            frontmatter: Frontmatter::default(),
            body_md: "## 1. Introduction\n\nNo citations here.\n".to_string(),
        };
        let r = render_journal_doc(&doc, &registry, &ContentIndex::default(), Lang::En);
        assert!(!r.html.contains("References"));
    }

    #[test]
    fn spanish_heading_gets_a_real_anchor_not_a_dead_one() {
        // The bug this fixes: the old hand-rolled `slugify` collapsed accented
        // letters differently than comrak's own anchor generator, so a TOC
        // link computed by this engine didn't match the id comrak actually put
        // on the rendered `<h2>` — every Spanish-language heading was a dead
        // anchor. `Anchorizer` keeps Unicode letters, so `<h2 id=...>` (comrak's
        // own output, via `header_ids`) and our TOC `Heading.id` must now agree.
        let r = render("## Estándares editoriales\n", &ContentIndex::default(), Lang::En);
        assert_eq!(r.headings[0].id, "estándares-editoriales");
        assert!(
            r.html.contains(r#"id="estándares-editoriales""#),
            "comrak's own rendered anchor: {}",
            r.html
        );
    }

    #[test]
    fn strips_group_count_heading_attribute() {
        let r = render("## Identity and permissions {#group-count-5}\n\nBody.\n", &ContentIndex::default(), Lang::En);
        assert_eq!(r.headings[0].text, "Identity and permissions");
        assert!(!r.html.contains("group-count-5"));
        assert!(r.html.contains("Identity and permissions"));
    }

    #[test]
    fn heading_attr_strip_ignores_fenced_examples() {
        let r = render("## Real {#real}\n\n```\n## Not a heading {#fake}\n```\n", &ContentIndex::default(), Lang::En);
        assert_eq!(r.headings.len(), 1);
        assert_eq!(r.headings[0].text, "Real");
        assert!(r.html.contains("Not a heading {#fake}"));
    }

    #[test]
    fn heading_attr_strip_spares_backticked_mentions() {
        // A heading that documents the syntax in backticks (no bare trailing
        // `}`) is left alone by `strip_trailing_heading_attr` — the line
        // doesn't end with `}` at all, so `{#id}` must survive intact, not
        // get mistaken for a real trailing heading-attribute and stripped
        // down to "The heading-attribute syntax".
        //
        // The literal backticks themselves don't survive into `.text`,
        // though — `render()`'s heading extraction now reads ids/text
        // straight from comrak's own rendered HTML (`extract_headings_from_html`,
        // fixing a real dead-anchor bug on non-ASCII headings), and comrak
        // renders `` `{#id}` `` as `<code>{#id}</code>`; stripping that tag
        // for plain display text loses the backtick markup the same way any
        // other inline formatting (bold, links) would. That's the correct,
        // visible-rendered-text behavior, not a regression of the guard this
        // test protects.
        let r = render("## The heading-attribute syntax `{#id}` explained\n", &ContentIndex::default(), Lang::En);
        assert_eq!(
            r.headings[0].text,
            "The heading-attribute syntax {#id} explained"
        );
        assert!(r.html.contains("<code>{#id}</code>"));
    }
}
