//! JSON-LD baseline for TOPIC pages — schema.org markup for AEO eligibility.
//!
//! Per PHASE-2-PLAN.md §1 Step 1 + ARCHITECTURE.md §3 Phase 2:
//! - Every rendered TOPIC carries a `<script type="application/ld+json">` block
//!   in the page `<head>` from Phase 2 onward.
//! - Profile selection: TechArticle by default; DefinedTerm when frontmatter
//!   `disclosure_class: glossary` is set (extends ARCHITECTURE.md §6 enum).
//! - Cumulative — costs nothing in later phases, accumulates AEO-eligibility.
//!
//! The output is a complete `<script>` element including tags so callers can
//! emit it inside `<head>` via `maud::PreEscaped`.

use crate::render::Frontmatter;
use serde_json::{json, Value};

/// Build a `<script type="application/ld+json">…</script>` block for one TOPIC.
///
/// The page chrome calls this from inside `<head>`. Profile selection:
/// - `disclosure_class: glossary` → schema.org `DefinedTerm`
/// - else → schema.org `TechArticle`
pub fn jsonld_for_topic(meta: &Frontmatter, slug: &str) -> String {
    let title = meta.title.as_deref().unwrap_or(slug);
    let schema_type = match meta.disclosure_class.as_deref() {
        Some("glossary") => "DefinedTerm",
        _ => "TechArticle",
    };

    let mut obj: Value = json!({
        "@context": "https://schema.org",
        "@type": schema_type,
        "name": title,
        "identifier": slug,
        "inLanguage": "en",
        "isPartOf": {
            "@type": "WebSite",
            "name": "PointSav Knowledge"
        }
    });

    // Optional fields drawn from the catch-all `extra` map (these are not
    // first-class fields on Frontmatter today; ARCHITECTURE.md §6 lists them
    // as optional). Soft-fail if absent or wrong shape.
    if let Some(serde_yaml::Value::String(date)) = meta.extra.get("last_revised") {
        obj["datePublished"] = json!(date);
    }

    if let Some(authors_val) = meta.extra.get("authors") {
        if let Ok(authors) = serde_yaml::from_value::<Vec<String>>(authors_val.clone()) {
            obj["author"] = Value::Array(
                authors
                    .into_iter()
                    .map(|a| json!({ "@type": "Person", "name": a }))
                    .collect(),
            );
        }
    }

    // Forward-looking-information disclosure flag flows to JSON-LD as an
    // additionalProperty so AEO crawlers + downstream consumers see the
    // FLI label on the structured-data side, not just in rendered chrome.
    if meta.forward_looking {
        obj["additionalProperty"] = json!([{
            "@type": "PropertyValue",
            "name": "forward_looking",
            "value": true
        }]);
    }

    format!(
        r#"<script type="application/ld+json">{}</script>"#,
        serde_json::to_string(&obj).unwrap_or_default()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fm() -> Frontmatter {
        Frontmatter::default()
    }

    #[test]
    fn emits_script_tag() {
        let html = jsonld_for_topic(&fm(), "topic-x");
        assert!(html.starts_with(r#"<script type="application/ld+json">"#));
        assert!(html.ends_with("</script>"));
    }

    #[test]
    fn defaults_to_techarticle() {
        let html = jsonld_for_topic(&fm(), "topic-x");
        assert!(html.contains(r#""@type":"TechArticle""#));
    }

    #[test]
    fn glossary_emits_definedterm() {
        let mut f = fm();
        f.disclosure_class = Some("glossary".to_string());
        let html = jsonld_for_topic(&f, "topic-x");
        assert!(html.contains(r#""@type":"DefinedTerm""#));
    }

    #[test]
    fn includes_title_when_present() {
        let mut f = fm();
        f.title = Some("Hello World".to_string());
        let html = jsonld_for_topic(&f, "topic-x");
        assert!(html.contains(r#""name":"Hello World""#));
    }

    #[test]
    fn falls_back_to_slug_when_title_missing() {
        let html = jsonld_for_topic(&fm(), "topic-x");
        assert!(html.contains(r#""name":"topic-x""#));
    }

    #[test]
    fn fli_flag_emits_additional_property() {
        let mut f = fm();
        f.forward_looking = true;
        let html = jsonld_for_topic(&f, "topic-x");
        assert!(html.contains(r#""additionalProperty""#));
        assert!(html.contains(r#""forward_looking""#));
    }

    #[test]
    fn output_body_is_valid_json() {
        let html = jsonld_for_topic(&fm(), "topic-x");
        let prefix = r#"<script type="application/ld+json">"#;
        let suffix = "</script>";
        let body = &html[prefix.len()..html.len() - suffix.len()];
        let parsed: serde_json::Value = serde_json::from_str(body).unwrap();
        assert_eq!(parsed["@context"], "https://schema.org");
        assert_eq!(parsed["isPartOf"]["name"], "PointSav Knowledge");
    }
}
