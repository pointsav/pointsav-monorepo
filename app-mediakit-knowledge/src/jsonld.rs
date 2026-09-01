// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Per-article JSON-LD (Phase 3.8 of `KNOWLEDGE-PLATFORM-PLAN.md`).
//!
//! The site-level `WebSite`/`Organization` block lives inline in
//! `ui::layout::doc_head` (unchanged by this module). This module adds the
//! per-article enrichment that block never carried: `TechArticle` with
//! `dateModified`/`description`/`citation`/`version`/`keywords`, and
//! `BreadcrumbList` for the category → article path. Both are plain
//! `serde_json::Value` trees serialized once per page — no schema crate
//! needed, and `PreEscaped`-safe (the `</script>` breakout guard mirrors the
//! same escaping the inline WebSite block already relies on being unnecessary
//! for, since none of these string fields can contain `<`/`>` from trusted
//! frontmatter, but citation URLs are operator-authored so we escape anyway).

use serde_json::{json, Value};

use crate::content::Frontmatter;

/// `TechArticle` JSON-LD for one article. `citations` is the resolved set of
/// citation URLs (from `citations.yaml`, keyed by the article's `cites:` and
/// any claim-level `cites`) — pass an empty slice if none resolved.
#[allow(clippy::too_many_arguments)]
pub fn article_jsonld(
    title: &str,
    canonical_url: &str,
    description: &str,
    date_modified: Option<&str>,
    date_published: Option<&str>,
    version: Option<&str>,
    keywords: &[String],
    citations: &[String],
    publisher_name: &str,
) -> Value {
    let mut obj = json!({
        "@context": "https://schema.org",
        "@type": "TechArticle",
        "headline": title,
        "url": canonical_url,
        "publisher": { "@type": "Organization", "name": publisher_name },
    });
    let map = obj.as_object_mut().expect("object literal");
    if !description.is_empty() {
        map.insert("description".into(), json!(description));
    }
    if let Some(dm) = date_modified {
        map.insert("dateModified".into(), json!(dm));
    }
    if let Some(dp) = date_published {
        map.insert("datePublished".into(), json!(dp));
    }
    if let Some(v) = version {
        map.insert("version".into(), json!(v));
    }
    if !keywords.is_empty() {
        map.insert("keywords".into(), json!(keywords.join(", ")));
    }
    if !citations.is_empty() {
        map.insert("citation".into(), json!(citations));
    }
    obj
}

/// Build `article_jsonld` directly from a parsed `Frontmatter` + the derived
/// fields the caller already has on hand (canonical URL, resolved citation
/// URLs, git-derived `dateModified`). Convenience wrapper for route handlers.
pub fn article_jsonld_from_frontmatter(
    fm: &Frontmatter,
    canonical_url: &str,
    date_modified: Option<&str>,
    citations: &[String],
    publisher_name: &str,
) -> Value {
    let title = fm.title.as_deref().unwrap_or("");
    let description = fm.short_description.as_deref().unwrap_or("");
    article_jsonld(
        title,
        canonical_url,
        description,
        date_modified,
        None,
        None,
        &fm.tags,
        citations,
        publisher_name,
    )
}

/// `BreadcrumbList` JSON-LD: Home → Category → Article. `category` is the
/// display label; pass `None` for pages with no category (e.g. special pages).
pub fn breadcrumb_jsonld(
    home_url: &str,
    home_label: &str,
    category: Option<(&str, &str)>, // (label, url)
    article_title: &str,
    article_url: &str,
) -> Value {
    let mut items = vec![json!({
        "@type": "ListItem",
        "position": 1,
        "name": home_label,
        "item": home_url,
    })];
    let mut position = 2;
    if let Some((label, url)) = category {
        items.push(json!({
            "@type": "ListItem",
            "position": position,
            "name": label,
            "item": url,
        }));
        position += 1;
    }
    items.push(json!({
        "@type": "ListItem",
        "position": position,
        "name": article_title,
        "item": article_url,
    }));

    json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": items,
    })
}

/// Serialize a JSON-LD value to a `<script>`-safe string: escapes `</` so an
/// authored citation URL or title containing a literal `</script>` substring
/// cannot break out of the surrounding `<script type="application/ld+json">`
/// tag (the same class of concern the convention docs flag for any
/// operator-authored string embedded in a script block).
pub fn to_script_safe_json(value: &Value) -> String {
    serde_json::to_string(value)
        .unwrap_or_default()
        .replace("</", "<\\/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article_jsonld_includes_all_provided_fields() {
        let v = article_jsonld(
            "Test Article",
            "https://documentation.pointsav.com/wiki/test",
            "A test description.",
            Some("2026-07-13"),
            Some("2026-01-01"),
            Some("1.2.0"),
            &["rust".to_string(), "wiki".to_string()],
            &["https://example.com/source".to_string()],
            "PointSav Digital Systems",
        );
        assert_eq!(v["@type"], "TechArticle");
        assert_eq!(v["headline"], "Test Article");
        assert_eq!(v["dateModified"], "2026-07-13");
        assert_eq!(v["datePublished"], "2026-01-01");
        assert_eq!(v["version"], "1.2.0");
        assert_eq!(v["keywords"], "rust, wiki");
        assert_eq!(v["citation"][0], "https://example.com/source");
    }

    #[test]
    fn article_jsonld_omits_absent_optional_fields() {
        let v = article_jsonld(
            "Minimal",
            "https://x/wiki/minimal",
            "",
            None,
            None,
            None,
            &[],
            &[],
            "PointSav",
        );
        assert!(v.get("dateModified").is_none());
        assert!(v.get("description").is_none());
        assert!(v.get("keywords").is_none());
        assert!(v.get("citation").is_none());
    }

    #[test]
    fn breadcrumb_jsonld_with_category_has_three_items() {
        let v = breadcrumb_jsonld(
            "https://x/",
            "PointSav Documentation",
            Some(("Architecture", "https://x/category/architecture")),
            "Test Article",
            "https://x/wiki/test",
        );
        let items = v["itemListElement"].as_array().unwrap();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0]["position"], 1);
        assert_eq!(items[1]["name"], "Architecture");
        assert_eq!(items[2]["position"], 3);
    }

    #[test]
    fn breadcrumb_jsonld_without_category_has_two_items() {
        let v = breadcrumb_jsonld(
            "https://x/",
            "Home",
            None,
            "Special Page",
            "https://x/special/foo",
        );
        assert_eq!(v["itemListElement"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn script_safe_json_escapes_script_breakout() {
        let v = json!({"headline": "</script><script>alert(1)</script>"});
        let s = to_script_safe_json(&v);
        assert!(!s.contains("</script>"));
        assert!(s.contains("<\\/script>"));
    }
}
