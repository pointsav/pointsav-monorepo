//! The typed Section component vocabulary — the contract an AI author writes
//! against, and the only set of building blocks a page may be composed from.
//!
//! A page manifest names a section `type` and binds data to that type's
//! fields. Validation is structural: a manifest either deserializes into one
//! of these variants (all required fields present, correct types) or it is
//! rejected. There is no path to arbitrary HTML or CSS — that is what keeps
//! AI-authored pages on-brand and mobile-correct (each component owns its CSS
//! in `static/sections.css`).
//!
//! This scaffold ships a minimal-but-real subset — `hero`, `prose`, `cta` —
//! enough to render a homepage and prove the contract. The full catalogue
//! (card-grid, feature, media, …) is a later phase.

use maud::{html, Markup, PreEscaped};
use serde::{Deserialize, Serialize};

use crate::render::markdown_to_fragment;

/// A reusable call-to-action button (used inside a hero or a standalone CTA).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtaButton {
    /// Button text.
    pub label: String,
    /// Destination URL or path.
    pub href: String,
}

impl CtaButton {
    fn render(&self) -> Markup {
        html! { a class="cta-button" href=(self.href) { (self.label) } }
    }
}

/// A typed page section. The `type` discriminator selects the variant; the
/// remaining fields are that variant's data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Section {
    /// Statement headline + one support line + optional CTA (mobile research
    /// §3: statement headline + one support line).
    Hero(HeroSection),
    /// Free Markdown prose (CommonMark + GFM).
    Prose(ProseSection),
    /// A standalone call-to-action band.
    Cta(CtaSection),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroSection {
    pub headline: String,
    #[serde(default)]
    pub subhead: Option<String>,
    #[serde(default)]
    pub cta: Option<CtaButton>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProseSection {
    /// Section body in Markdown.
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtaSection {
    pub heading: String,
    pub cta: CtaButton,
}

impl Section {
    /// Human-readable kind label (used in diffs, MCP listings, logs).
    pub fn kind(&self) -> &'static str {
        match self {
            Section::Hero(_) => "hero",
            Section::Prose(_) => "prose",
            Section::Cta(_) => "cta",
        }
    }

    /// Render this section to an HTML fragment. No CSS is emitted inline —
    /// styling comes entirely from `static/sections.css`.
    pub fn render(&self) -> Markup {
        match self {
            Section::Hero(h) => html! {
                section class="section section-hero" {
                    h1 class="hero-headline" { (h.headline) }
                    @if let Some(sub) = &h.subhead {
                        p class="hero-subhead" { (sub) }
                    }
                    @if let Some(cta) = &h.cta { (cta.render()) }
                }
            },
            Section::Prose(p) => html! {
                section class="section section-prose" {
                    div class="prose-body" { (PreEscaped(markdown_to_fragment(&p.body))) }
                }
            },
            Section::Cta(c) => html! {
                section class="section section-cta" {
                    h2 class="cta-heading" { (c.heading) }
                    (c.cta.render())
                }
            },
        }
    }
}

/// A machine-readable description of the section vocabulary, returned by the
/// MCP `list_section_types` tool so an AI author discovers what it may emit.
/// Shaped like a JSON-Schema `oneOf` over the section types.
pub fn section_catalog() -> serde_json::Value {
    serde_json::json!({
        "hero": {
            "required": ["headline"],
            "optional": ["subhead", "cta{label,href}"],
            "note": "Statement headline + one support line + optional CTA."
        },
        "prose": {
            "required": ["body"],
            "note": "Markdown body (CommonMark + GFM)."
        },
        "cta": {
            "required": ["heading", "cta{label,href}"],
            "note": "Standalone call-to-action band."
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hero_renders_headline_and_cta() {
        let s = Section::Hero(HeroSection {
            headline: "Sovereign sites, simply".into(),
            subhead: Some("One support line.".into()),
            cta: Some(CtaButton {
                label: "Enquire".into(),
                href: "/contact".into(),
            }),
        });
        let html = s.render().into_string();
        assert!(html.contains("hero-headline"));
        assert!(html.contains("Sovereign sites, simply"));
        assert!(html.contains("cta-button"));
        assert_eq!(s.kind(), "hero");
    }

    #[test]
    fn prose_renders_markdown() {
        let s = Section::Prose(ProseSection {
            body: "## Heading\n\nA paragraph with **bold**.".into(),
        });
        let html = s.render().into_string();
        assert!(html.contains("<h2>"));
        assert!(html.contains("<strong>bold</strong>"));
    }
}
