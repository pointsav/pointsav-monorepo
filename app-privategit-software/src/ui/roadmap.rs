// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Roadmap data + card renderer — consumed by `catalog::catalog_markup` (`/software`),
//! not a page of its own.
//!
//! **Phase 1b, revised.** An earlier version of this module rendered a structurally
//! separate `/roadmap` page specifically because an adversarial review found real
//! risks in mixing real and not-yet-real products on one page (drift, honesty,
//! complexity). The operator later reviewed that shipped result and asked for it
//! merged back into one page. This module still carries the two safety properties
//! that review named — they're now enforced by a **module boundary + a type-level
//! guarantee**, not by a URL boundary:
//!
//! 1. **No drift.** `ROADMAP_ITEMS` below is a `pub(crate)` const, never
//!    `products.yaml`/`Catalog` — the only two data sources `catalog.rs`'s
//!    `TierCard` enum can build a card from. A roadmap item cannot be hand-authored
//!    into the page; there is no third source.
//! 2. **No fake purchase signal.** This module has no access to `install_command`,
//!    `download_command`, `paid_product_card`'s checkout-link builder, or the
//!    page-level `SoftwareApplication` JSON-LD — all private to `catalog.rs`. A
//!    roadmap card is *structurally incapable* of emitting a price, an install/
//!    download block, or a checkout link, because the functions that emit one are
//!    not in scope here. `roadmap_item_card_alone_emits_no_purchase_flow_signal`
//!    (below) is the test that keeps this true.
//!
//! Every item is described by its intended architectural ROLE, never by its current
//! code/prototype state — grounded in real project-totebox documentation
//! (`BRIEF-os-console-platform.md`, `os-infrastructure/CLAUDE.md`), not invented.
//! See `.agent/briefs/BRIEF-storefront-architecture-education.md` for the full
//! research trail (including the two prior design iterations) behind this shape.

use crate::ui::Lang;
use crate::ArchTier;
use maud::{html, Markup};

/// How real a roadmap item is — deliberately borrowed from GitLab's public
/// Direction maturity ladder and Google Cloud's release-stage semantics (real
/// precedent researched this session for "show real progress without a purchase
/// flow"), not invented vocabulary. Each variant carries its own full explanatory
/// sentence (`explained()`) so a card is self-contained — no separate glossary
/// block to cross-reference, per direct operator feedback that the earlier
/// maturity legend was clunky.
// `InActiveDevelopment` is a real rung of the researched 3-rung ladder, not currently
// constructed by any item in `ROADMAP_ITEMS` — its last user, os-console, graduated to
// a real catalog product 2026-08-06. Kept, not deleted: the ladder is deliberately
// researched vocabulary (see doc comment above), and shrinking it to match today's
// roadmap slate would just mean re-adding it the next time a fourth item lands mid-build
// (as `app-privategit-design` may, once it needs a public-facing roadmap entry before
// its own catalog listing is ready).
#[derive(Clone, Copy)]
pub(crate) enum Readiness {
    #[allow(dead_code)]
    InActiveDevelopment,
    InDesign,
    Planned,
}

impl Readiness {
    fn explained(self, lang: Lang) -> &'static str {
        match (self, lang) {
            (Readiness::InActiveDevelopment, Lang::En) =>
                "In active development \u{2014} running in the lab today, not yet packaged for download.",
            (Readiness::InDesign, Lang::En) =>
                "In design \u{2014} the architecture is settled; engineering starts next.",
            (Readiness::Planned, Lang::En) =>
                "Planned \u{2014} a committed direction, no engineering yet.",
            (Readiness::InActiveDevelopment, Lang::Es) =>
                "En desarrollo activo \u{2014} funciona en el laboratorio hoy, a\u{fa}n no empaquetado para descarga.",
            (Readiness::InDesign, Lang::Es) =>
                "En dise\u{f1}o \u{2014} la arquitectura est\u{e1} definida; la ingenier\u{ed}a empieza a continuaci\u{f3}n.",
            (Readiness::Planned, Lang::Es) =>
                "Planificado \u{2014} una direcci\u{f3}n comprometida, sin ingenier\u{ed}a todav\u{ed}a.",
        }
    }
}

/// A single roadmap entry, built to the same card "slot contract" real products use
/// (`catalog.rs`'s `free_product_card`/`appliance_product_card`): kicker → name →
/// headline → deck → status → up to 3 labelled facts → terminal. All prose here is
/// deliberately English-only — no translation source exists for it, matching the
/// same "product data stays untranslated on /es/" convention already established
/// for `products.yaml` descriptions and license-tier labels.
pub(crate) struct RoadmapItem {
    pub(crate) id: &'static str,
    name: &'static str,
    pub(crate) tier: ArchTier,
    readiness: Readiness,
    /// Short, confident, present/future-tense sentence — the roadmap counterpart to
    /// a real product's `becomes:` line. Promoted to the card's dominant sentence,
    /// same visual weight as a real product's headline.
    headline: &'static str,
    /// ≤30 words. The compressed architectural role — what this item's real
    /// intended job is, never a description of its current prototype code.
    deck: &'static str,
    /// Up to 3 labelled facts, same shape and cap as `Installer::facts` — the
    /// "substantial through structure, not prose length" mechanism. One entry is
    /// conventionally a "LICENSING INTENT" row.
    facts: &'static [(&'static str, &'static str)],
    /// Deep-link fragment into `/licensing`'s real tier sections (`#fsl`/`#agpl`/
    /// `#proprietary`/`#apache`), when the intent clearly implies one. `None` when
    /// genuinely undecided (e.g. os-workplace) — no anchor is better than a guess.
    license_anchor: Option<&'static str>,
}

pub(crate) const ROADMAP_ITEMS: &[RoadmapItem] = &[
    RoadmapItem {
        id: "os-infrastructure",
        name: "Self-Hosted Infrastructure Substrate",
        tier: ArchTier::Infrastructure,
        readiness: Readiness::InDesign,
        headline: "Will turn hardware you own into somewhere a Totebox archive can run.",
        deck: "A host operating system that boots straight onto bare metal, a leased \
               server, or a commodity cloud node, and anchors every workload to the \
               formally-verified seL4 microkernel.",
        facts: &[
            ("REMOVES", "Your dependency on infrastructure a vendor owns"),
            ("JOINS", "The institution's own private encrypted mesh"),
            (
                "LICENSING INTENT",
                "AGPL-3.0-or-later — source is freely available under copyleft; a \
                 purchase buys the signed binary and commercial redistribution \
                 rights, not access to the code",
            ),
        ],
        license_anchor: Some("agpl"),
    },
    RoadmapItem {
        id: "os-workplace",
        name: "Staff Desktop Environment",
        tier: ArchTier::Platform,
        readiness: Readiness::Planned,
        headline: "Will turn a machine into the desktop staff use to work directly \
                   with an archive.",
        deck: "Native applications for day-to-day records, correspondence, and \
               bookkeeping work, running on hardware the institution controls \
               rather than a vendor's cloud desktop.",
        facts: &[
            ("RUNS AS", "Native desktop applications, not a browser tab"),
            ("FOR", "Staff working directly with an archive day to day"),
            (
                "LICENSING INTENT",
                "Not yet finalized; expected one-time purchase, no subscription",
            ),
        ],
        license_anchor: None,
    },
    // os-console REMOVED from the roadmap 2026-08-06 — graduated to a real catalog
    // product (id: os-console, "The Operator's Console") this session: Linux is
    // live-verified and shipping, not just planned. See catalog/products.yaml.
];

/// The status block shared visual language: an availability chip plus one
/// self-explanatory sentence. `pub(crate)` so `catalog.rs`'s real-product cards can
/// render the "Available now" counterpart with the identical CSS classes — one
/// shared visual contract for both card kinds, not two.
pub(crate) fn status_block(chip_class: &str, chip_text: &str, note: &str) -> Markup {
    html! {
        div."sw-cat-status" {
            span class=(format!("sw-cat-chip {chip_class}")) { (chip_text) }
            p."sw-cat-status__note" { (note) }
        }
    }
}

/// The fact-row block shared visual language — up to 3 labelled rows, reused
/// verbatim by `catalog.rs`'s real-product cards (their 4th, always-derived
/// "Distribution" row is appended by the caller, not here).
pub(crate) fn fact_rows(facts: &[(&str, &str)]) -> Markup {
    html! {
        dl."sw-cat-facts" {
            @for (label, value) in facts {
                div."sw-cat-fact" {
                    dt { (*label) }
                    dd { (*value) }
                }
            }
        }
    }
}

pub(crate) fn roadmap_item_card(item: &RoadmapItem, lang: Lang) -> Markup {
    let (not_yet_available, nothing_to_download, full_terms_label) = match lang {
        Lang::En => (
            "Not yet available",
            "Nothing to download yet.",
            "Full terms \u{2192}",
        ),
        Lang::Es => (
            "A\u{fa}n no disponible",
            "Todav\u{ed}a no hay nada para descargar.",
            "T\u{e9}rminos completos \u{2192}",
        ),
    };
    html! {
        article."sw-rm-item" {
            span."sw-cat-card__id" { (item.id) }
            h3."sw-cat-card__name" { (item.name) }
            p."sw-cat-card__becomes" { (item.headline) }
            p."sw-cat-card__desc" { (item.deck) }
            (status_block("sw-cat-chip--soon", not_yet_available, item.readiness.explained(lang)))
            (fact_rows(item.facts))
            div."sw-rm-item__status" {
                p."sw-rm-item__status-h" { (nothing_to_download) }
                @if let Some(anchor) = item.license_anchor {
                    a href=(format!("{}#{anchor}", lang.localize("/licensing"))) {
                        (full_terms_label)
                    }
                }
            }
        }
    }
}

/// Card-scoped CSS for `.sw-rm-item` plus the shared `.sw-cat-status`/
/// `.sw-cat-facts`/`.sw-cat-chip` rules both card kinds use — `pub(crate)` so
/// `catalog.rs`'s `catalog_style()` can concatenate this into the page's single
/// `<style>` block (Phase 1b: this module renders no page shell of its own).
pub(crate) const ROADMAP_CARD_CSS: &str = r#".sw-rm-item{min-width:0;border:1px dashed #d0d5dd;border-radius:10px;padding:20px;background:#f8f9fa;display:flex;flex-direction:column;}
.sw-rm-item__status{margin-top:auto;padding:12px 14px;border:1px dashed #d0d5dd;border-radius:6px;background:#fcfcfd;}
.sw-rm-item__status-h{margin:0 0 6px;font-size:12.5px;font-weight:600;color:#475467;}
.sw-cat-status{display:flex;flex-wrap:wrap;align-items:center;gap:8px;margin:0 0 12px;}
.sw-cat-status__note{flex:1 0 100%;margin:0;font-size:12px;line-height:1.45;color:#667085;}
.sw-cat-chip{font-size:11px;font-weight:600;letter-spacing:.02em;padding:3px 8px;border-radius:999px;background:#ecfdf3;color:#067647;}
.sw-cat-chip--soon{background:#f2f4f7;color:#667085;}
.sw-cat-facts{display:grid;grid-template-columns:auto 1fr;column-gap:12px;row-gap:6px;margin:0 0 14px;padding:12px 0;border-top:1px solid #f2f4f7;border-bottom:1px solid #f2f4f7;}
.sw-cat-fact{display:contents;}
.sw-cat-fact dt{font-size:10.5px;font-weight:600;letter-spacing:.06em;text-transform:uppercase;color:#98a2b3;}
.sw-cat-fact dd{margin:0;font-size:12.5px;line-height:1.5;color:#344054;}"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_roadmap_items_render_their_headline_deck_and_facts() {
        for lang in [Lang::En, Lang::Es] {
            for item in ROADMAP_ITEMS {
                let html = roadmap_item_card(item, lang).into_string();
                assert!(html.contains(item.id));
                assert!(html.contains(item.name));
                assert!(html.contains(item.headline));
                assert!(html.contains(item.deck));
                for (label, value) in item.facts {
                    assert!(html.contains(*label), "missing fact label {label}");
                    assert!(html.contains(*value), "missing fact value {value}");
                }
            }
        }
    }

    #[test]
    fn no_legend_block_renders_anywhere() {
        // Phase 1b: the maturity legend was deleted per direct operator feedback —
        // each card now carries its own self-explanatory status sentence instead.
        for item in ROADMAP_ITEMS {
            let html = roadmap_item_card(item, Lang::En).into_string();
            assert!(!html.contains("sw-rm-legend"));
            assert!(html.contains(item.readiness.explained(Lang::En)));
        }
    }

    #[test]
    fn roadmap_items_deep_link_to_real_licensing_anchors_when_known() {
        let html_infra = roadmap_item_card(&ROADMAP_ITEMS[0], Lang::En).into_string();
        assert!(html_infra.contains(r#"href="/licensing#agpl""#));
        let html_workplace = roadmap_item_card(&ROADMAP_ITEMS[1], Lang::En).into_string();
        // os-workplace has no `license_anchor` — must not fabricate one.
        assert!(!html_workplace.contains("href=\"/licensing#"));
        let html_es = roadmap_item_card(&ROADMAP_ITEMS[0], Lang::Es).into_string();
        assert!(html_es.contains(r#"href="/es/licensing#agpl""#));
    }

    /// Synthetic item, not indexed off `ROADMAP_ITEMS` — the agpl-anchor code path
    /// deserves coverage independent of whether any *current* real roadmap item
    /// happens to use it (os-console did, until it graduated to a real catalog
    /// product 2026-08-06 and was removed from this list; the mechanism this test
    /// guards didn't change, only which production item exercised it).
    #[test]
    fn agpl_license_anchor_deep_links_correctly() {
        let item = RoadmapItem {
            id: "test-agpl-item",
            name: "Test AGPL Item",
            tier: ArchTier::Delivery,
            readiness: Readiness::Planned,
            headline: "headline",
            deck: "deck",
            facts: &[],
            license_anchor: Some("agpl"),
        };
        let html = roadmap_item_card(&item, Lang::En).into_string();
        assert!(html.contains(r#"href="/licensing#agpl""#));
    }

    /// The direct successor to the deleted page-level roadmap tests — the actual
    /// safety property this module exists to guarantee, verified per-card.
    #[test]
    fn roadmap_item_card_alone_emits_no_purchase_flow_signal() {
        for lang in [Lang::En, Lang::Es] {
            for item in ROADMAP_ITEMS {
                let html = roadmap_item_card(item, lang).into_string();
                assert!(!html.contains("sw-cat-price"));
                assert!(!html.contains("sw-cat-install"));
                assert!(!html.contains("sw-cat-cmd"));
                assert!(!html.contains("data-sw-clip"));
                assert!(!html.contains("/checkout/"));
                assert!(!html.contains("/software/"));
                assert!(!html.contains("install.sh"));
                assert!(!html.contains("SoftwareApplication"));
                assert!(!html.contains('$'));
            }
        }
    }
}
