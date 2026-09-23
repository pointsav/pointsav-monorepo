// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! `/working-papers` — the JOURNAL corpus's 3 papers assigned to
//! software.pointsav.com (`GET /working-papers`, `GET /working-papers/:slug`).
//! Added 2026-09-15 as part of the workspace-wide `/working-papers` one-time
//! rendering bootstrap (`BRIEF-journal-research-rendering.md`, project-editorial).
//!
//! English-only, same as every other site's JOURNAL content (institutional
//! authorship, zero outbound links, journal-v2 schema) — there is no Spanish
//! sibling for paper bodies, matching every other non-MVL page on this site
//! (`render_page`'s `translated: false` path).
//!
//! Each paper's already-converted body HTML lives in
//! `static/working-papers/<slug>.html` (pandoc-converted from the journal-v2
//! markdown source, frontmatter stripped) and is spliced in via `PreEscaped` —
//! the same pattern this crate already uses for raw inline SVG strings, not a
//! new idiom.

use maud::{html, Markup, PreEscaped};

use super::lang::Lang;

pub struct WorkingPaper {
    pub slug: &'static str,
    pub tag: &'static str,
    pub imprint: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub abstract_html: &'static str,
    pub keywords: &'static str,
    pub body_html: &'static str,
}

pub const PAPERS: &[WorkingPaper] = &[
    WorkingPaper {
        slug: "cost-of-renting-software",
        tag: "Software Economics",
        imprint: "PDS-2026-01",
        title: "The Cost of Renting Software You Could Own",
        subtitle: "A total-cost-of-ownership model for subscription versus perpetual licensing",
        abstract_html: "A total-cost-of-ownership comparison between subscription \
            software and a one-time, perpetually-licensed purchase &mdash; matching this \
            site's own real distribution model (no subscription, no cloud dependency, no \
            kill switch).",
        keywords: "software licensing, total cost of ownership, subscription economics",
        body_html: include_str!("../../static/working-papers/cost-of-renting-software.html"),
    },
    WorkingPaper {
        slug: "open-source-survives-ai",
        tag: "Software Economics",
        imprint: "PDS-2026-02",
        title: "What Open Source Actually Is, and Whether It Survives AI",
        subtitle:
            "Reciprocity, provenance, and which historical function does the load-bearing work now",
        abstract_html: "Generative AI changes which of open source's historical functions \
            actually carries the weight &mdash; the free-distribution function weakens, the \
            verifiable-provenance function strengthens. A conditional, not absolute, \
            conclusion.",
        keywords: "open source economics, software licensing, generative AI",
        body_html: include_str!("../../static/working-papers/open-source-survives-ai.html"),
    },
    WorkingPaper {
        slug: "verifiable-ownership-without-vendor",
        tag: "Systems Architecture",
        imprint: "PDS-2026-03",
        title: "Verifiable Ownership Without a Vendor in the Loop",
        subtitle:
            "An architecture for license and payment proof that outlasts the vendor's own servers",
        abstract_html: "Describes this site's own deployed verification architecture &mdash; \
            a public payment record, an offline-verifiable Ed25519 license token, and a \
            hash-chained per-binary artifact ledger &mdash; and why all three together are \
            the technical precondition for a \"perpetual\" license claim to mean what it \
            says.",
        keywords: "license verification, cryptographic provenance, offline verification",
        body_html: include_str!(
            "../../static/working-papers/verifiable-ownership-without-vendor.html"
        ),
    },
];

pub fn find(slug: &str) -> Option<&'static WorkingPaper> {
    PAPERS.iter().find(|p| p.slug == slug)
}

const PRINT_CSS: &str = include_str!("../../static/working-papers-print.css");

pub fn index_markup(_lang: Lang) -> Markup {
    html! {
        div."sw-working-papers" {
            style { (PreEscaped(PRINT_CSS)) }
            div."sw-legal__lede" {
                "Working papers in preparation for intended submission. All papers carry \
                CC BY 4.0 licences. Results are preliminary and subject to revision. \
                Forward-looking statements reflect current expectations; actual outcomes \
                may differ materially."
            }
            @for p in PAPERS {
                div."sw-wp-card" {
                    div."sw-wp-card__meta" {
                        span."sw-wp-card__tag" { (p.tag) }
                        span."sw-wp-card__imprint" { (p.imprint) " · v1.0.0" }
                    }
                    h2."sw-wp-card__title" { a href=(format!("/working-papers/{}", p.slug)) { (p.title) } }
                    p."sw-wp-card__subtitle" { (p.subtitle) }
                    p."sw-wp-card__authors" { "Woodfine Research — PointSav Digital Systems" }
                    p."sw-wp-card__abstract" { (PreEscaped(p.abstract_html)) }
                    p."sw-wp-card__keywords" { strong { "Keywords: " } (p.keywords) }
                    a."sw-wp-card__readmore" href=(format!("/working-papers/{}", p.slug)) { "Read Paper →" }
                }
            }
        }
        (PreEscaped(CARD_STYLE))
    }
}

pub fn item_markup(paper: &WorkingPaper) -> Markup {
    html! {
        article."sw-working-paper working-papers-print-root" {
            style { (PreEscaped(PRINT_CSS)) }
            header."sw-wp-header no-print" {
                div."sw-wp-header__kicker" { (paper.tag) " · Working Paper " (paper.imprint) }
                h1 { (paper.title) }
                p."sw-wp-header__subtitle" { (paper.subtitle) }
                p."sw-wp-header__meta" { "Woodfine Research — PointSav Digital Systems · v1.0.0 · CC BY 4.0" }
            }
            div."sw-wp-notice no-print" {
                "Working paper — preliminary, subject to revision. Forward-looking "
                "statements reflect current expectations; actual outcomes may differ "
                "materially."
            }
            div."sw-wp-body" { (PreEscaped(paper.body_html)) }
        }
        (PreEscaped(ITEM_STYLE))
    }
}

const CARD_STYLE: &str = r#"<style>
.sw-working-papers{max-width:760px;margin:0 auto;padding:8px 0 40px}
.sw-wp-card{background:#fff;border:1px solid #e6e7e8;border-radius:10px;padding:24px 24px 20px;margin-bottom:18px}
.sw-wp-card__meta{display:flex;gap:10px;align-items:center;margin-bottom:8px;font-size:11px;color:#6b7280;flex-wrap:wrap}
.sw-wp-card__tag{font-weight:700;text-transform:uppercase;letter-spacing:.06em;font-size:10px;padding:3px 8px;border-radius:4px;background:rgba(22,70,121,.08);color:#164679}
.sw-wp-card__title{font-size:18px;margin-bottom:4px}
.sw-wp-card__title a{color:#111827;text-decoration:none}
.sw-wp-card__title a:hover{text-decoration:underline}
.sw-wp-card__subtitle{font-size:13px;font-style:italic;color:#6b7280;margin-bottom:10px}
.sw-wp-card__authors{font-size:13px;color:#6b7280;margin-bottom:10px}
.sw-wp-card__abstract{font-size:14px;line-height:1.65;color:#374151;margin-bottom:12px}
.sw-wp-card__keywords{font-size:12px;color:#6b7280;margin-bottom:14px}
.sw-wp-card__readmore{display:inline-block;background:#164679;color:#fff;font-size:13px;font-weight:700;padding:8px 16px;border-radius:6px;text-decoration:none}
</style>"#;

const ITEM_STYLE: &str = r#"<style>
.sw-working-paper{max-width:720px;margin:0 auto;padding:8px 0 48px}
.sw-wp-header__kicker{font-size:11px;text-transform:uppercase;letter-spacing:.08em;color:#6b7280;margin-bottom:8px}
.sw-wp-header h1{font-size:24px;line-height:1.3;margin-bottom:8px}
.sw-wp-header__subtitle{font-size:14px;font-style:italic;color:#6b7280;margin-bottom:8px}
.sw-wp-header__meta{font-size:12px;color:#6b7280;margin-bottom:20px}
.sw-wp-notice{font-size:12.5px;color:#78350f;background:#fffbeb;border:1px solid #fde68a;border-radius:6px;padding:10px 14px;margin-bottom:24px}
.sw-wp-body h2{font-size:19px;margin:30px 0 12px;color:#164679}
.sw-wp-body h3{font-size:16px;margin:22px 0 8px}
.sw-wp-body p{font-size:15px;line-height:1.75;color:#26303c;margin-bottom:14px}
.sw-wp-body table{width:100%;border-collapse:collapse;margin:16px 0;font-size:13px}
.sw-wp-body th,.sw-wp-body td{border:1px solid #e6e7e8;padding:8px 10px;text-align:left;vertical-align:top}
.sw-wp-body th{background:rgba(22,70,121,.06)}
.sw-wp-body hr{border:none;border-top:1px solid #e6e7e8;margin:26px 0}
</style>"#;
