// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! `/working-papers` — the JOURNAL corpus's 4 final papers assigned to
//! software.pointsav.com (`GET /working-papers`, `GET /working-papers/:slug`).
//! Added 2026-09-15 as part of the workspace-wide `/working-papers` one-time
//! rendering bootstrap (`BRIEF-journal-research-rendering.md`, project-editorial);
//! refreshed 2026-09-24 from the old 3-stub draft set to the real final corpus
//! (buy-once-own-it, aggregation-is-the-commercial-boundary,
//! verifiable-ownership-without-vendor, business-software-not-enterprise) — the
//! 2 retired stub titles (cost-of-renting-software, open-source-survives-ai)
//! never made the final corpus and are fully removed, not just hidden.
//!
//! English-only, same as every other site's JOURNAL content (institutional
//! authorship, zero outbound links, journal-v2 schema) — there is no Spanish
//! sibling for paper bodies, matching every other non-MVL page on this site
//! (`render_page`'s `translated: false` path).
//!
//! Each paper's already-converted body HTML lives in
//! `static/working-papers/<slug>.html` (pandoc-converted from the journal-v2
//! markdown source, frontmatter + leading `# Title`/`*subtitle*` lines
//! stripped — the same convention already staged for design.pointsav.com's own
//! vault, confirmed by reading its live `dtcg-vault/working-papers/*.md`
//! directly rather than guessing) and is spliced in via `PreEscaped` — the same
//! pattern this crate already uses for raw inline SVG strings, not a new idiom.
//!
//! Sidebar + TOC (2026-09-24): this crate's `.sw-masthead` is a normal,
//! non-sticky/non-fixed element (confirmed directly in `layout.rs` — no
//! `position: sticky`/`fixed` anywhere on it), so the whole page scrolls as one
//! normal document; the sticky-header-covers-the-heading bug class
//! (`BRIEF-journal-research-rendering.md` items 11/16/17) cannot occur here by
//! construction. `scroll-margin-top`/the sidebar's own sticky `top` still use
//! the same 24px (`--ps-space-6`-equivalent) cushion every other site converged
//! on today, for visual consistency across all 6 sites, not because this
//! crate's own architecture requires it for correctness. `id="top"` sits
//! directly on `<h1>` in this file's own header (not a separate marker div) —
//! same reasoning as every other site: a marker div placed before the title
//! would put the title itself out of the revealed viewport on a `#top` jump.
//!
//! Extra chrome-level notices removed 2026-09-24 (operator-reported, live
//! review): the index page's `.sw-legal__lede` ("Working papers in
//! preparation for intended submission...") and the per-paper `.sw-wp-notice`
//! yellow box ("Working paper — preliminary, subject to revision...") were
//! both leftover from an earlier framing where these papers were being
//! prepared for outside submission — they are not, and each paper already
//! carries its own real forward-looking-statement disclaimer as the first
//! blockquote in its body (from the canonical journal-v2 source, present on
//! every site). Both site-chrome notices duplicated that disclaimer and
//! implied a submission process that was never real; removed rather than
//! reworded.

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
    /// (heading id, heading text) for every real H2 section, in document
    /// order, back-matter headings excluded — extracted directly from each
    /// paper's own pandoc-generated HTML (not re-derived from the heading
    /// text independently) so the sidebar's `href="#id"` values are
    /// guaranteed to match what pandoc actually emitted; pandoc's own
    /// slugifier drops apostrophes differently than this workspace's other
    /// sites' custom Rust slugifiers (e.g. "reader's" → "readers", not
    /// "reader-s"), so hand-deriving these separately would have silently
    /// produced dead anchor links.
    pub toc: &'static [(&'static str, &'static str)],
}

pub const PAPERS: &[WorkingPaper] = &[
    WorkingPaper {
        slug: "buy-once-own-it",
        tag: "Software Economics",
        imprint: "PDS-012",
        title: "Software That Is Sold, Not Rented",
        subtitle: "Full charge up front, a long stable release cycle, and no switch the \
            seller can throw on a copy already running",
        abstract_html: "A licensing architecture where enforcement sits only on the paid \
            layer &mdash; the free software holding a customer's own records carries no \
            licence check at all, so an expired licence ends new releases and paid \
            features, never access to what is already installed.",
        keywords: "perpetual licensing, software distribution, subscription economics",
        body_html: include_str!("../../static/working-papers/buy-once-own-it.html"),
        toc: &[
            ("1-the-thesis", "1. The thesis"),
            (
                "2-the-problem-in-the-readers-terms",
                "2. The problem, in the reader's terms",
            ),
            (
                "3-what-a-software-licence-actually-is",
                "3. What a software licence actually is",
            ),
            (
                "4-where-the-enforcement-actually-sits",
                "4. Where the enforcement actually sits",
            ),
            ("5-the-price-paid-once", "5. The price, paid once"),
            (
                "6-the-honest-economic-trade-off",
                "6. The honest economic trade-off",
            ),
            (
                "7-what-this-changes-for-the-buyer",
                "7. What this changes for the buyer",
            ),
            ("8-an-open-invitation", "8. An open invitation"),
            ("9-conclusion", "9. Conclusion"),
        ],
    },
    WorkingPaper {
        slug: "aggregation-is-the-commercial-boundary",
        tag: "Licensing Architecture",
        imprint: "PDS-013",
        title: "A Licence Boundary Drawn by Architecture, Not by Features",
        subtitle: "One archive runs free; the layer that asks a single question of many \
            archives at once is the part that costs money",
        abstract_html: "Argues the free/paid line should be drawn by architecture, not a \
            feature list &mdash; one archive runs free under permissive terms; the \
            aggregation layer that queries many archives at once is where the commercial \
            boundary, and a stronger copyleft term, actually belongs.",
        keywords: "open-core licensing, software architecture, licence boundary design",
        body_html: include_str!(
            "../../static/working-papers/aggregation-is-the-commercial-boundary.html"
        ),
        toc: &[
            ("1-the-thesis", "1. The thesis"),
            (
                "2-the-problem-in-the-readers-terms",
                "2. The problem, in the reader's terms",
            ),
            (
                "3-what-open-source-licensing-actually-is",
                "3. What open-source licensing actually is",
            ),
            (
                "4-where-the-boundary-actually-sits",
                "4. Where the boundary actually sits",
            ),
            (
                "5-which-licence-each-component-carries--a-different-question",
                "5. Which licence each component carries — a different question",
            ),
            (
                "6-what-this-changes-for-the-reader",
                "6. What this changes for the reader",
            ),
            ("7-an-open-invitation", "7. An open invitation"),
            ("8-conclusion", "8. Conclusion"),
        ],
    },
    WorkingPaper {
        slug: "verifiable-ownership-without-vendor",
        tag: "Systems Architecture",
        imprint: "PDS-014",
        title: "Ownership You Can Prove Without the Seller",
        subtitle: "A sealed licence, a payment recorded in public, and no activation \
            server that has to outlive the company",
        abstract_html: "Describes this site's own deployed verification architecture \
            &mdash; a public payment record, an offline-verifiable Ed25519 licence token, \
            and a hash-chained per-binary artifact ledger &mdash; and why a buyer should \
            be able to prove ownership without asking the seller.",
        keywords: "license verification, cryptographic provenance, offline verification",
        body_html: include_str!(
            "../../static/working-papers/verifiable-ownership-without-vendor.html"
        ),
        toc: &[
            ("1-the-thesis", "1. The thesis"),
            (
                "2-the-problem-in-the-readers-terms",
                "2. The problem, in the reader's terms",
            ),
            (
                "3-what-a-signature-anyone-can-check-actually-is",
                "3. What a signature anyone can check actually is",
            ),
            (
                "4-what-a-payment-recorded-in-public-is",
                "4. What a payment recorded in public is",
            ),
            (
                "5-what-an-expiring-licence-actually-ends",
                "5. What an expiring licence actually ends",
            ),
            (
                "6-what-this-changes-for-the-buyer",
                "6. What this changes for the buyer",
            ),
            ("7-an-open-invitation", "7. An open invitation"),
            ("8-conclusion", "8. Conclusion"),
        ],
    },
    WorkingPaper {
        slug: "business-software-not-enterprise",
        tag: "Product Strategy",
        imprint: "PDS-015",
        title: "Software for a Reporting Issuer, Not for a Multinational",
        subtitle: "Comparable operational complexity, incomparable organisational scale \
            — and why that distinction, not company size, is the design brief",
        abstract_html: "Argues the right design brief is a reporting issuer's real \
            operational complexity, not enterprise-multinational scale &mdash; a \
            distinction the software's actual target customer makes load-bearing, not \
            company size.",
        keywords: "reporting issuer software, transaction cost economics, product scope",
        body_html: include_str!(
            "../../static/working-papers/business-software-not-enterprise.html"
        ),
        toc: &[
            ("1-the-thesis", "1. The thesis"),
            (
                "2-the-problem-in-the-readers-terms",
                "2. The problem, in the reader's terms",
            ),
            (
                "3-what-enterprise-software-actually-is",
                "3. What enterprise software actually is",
            ),
            (
                "4-what-the-target-company-actually-looks-like",
                "4. What the target company actually looks like",
            ),
            (
                "5-comparable-complexity-incomparable-scale",
                "5. Comparable complexity, incomparable scale",
            ),
            (
                "6-what-follows-in-what-we-build",
                "6. What follows in what we build",
            ),
            (
                "7-what-this-changes-for-the-reader",
                "7. What this changes for the reader",
            ),
            ("8-an-open-invitation", "8. An open invitation"),
            ("9-conclusion", "9. Conclusion"),
        ],
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
                // Always-visible back link — not gated behind the sidebar's own
                // >=768px breakpoint, so a mobile reader is never stranded
                // without a way back to the index (BRIEF item 4's mobile-
                // fallback lesson, applied from the start rather than found
                // live and patched in afterward).
                a."sw-wp-header__back" href="/working-papers" { "← Working Papers" }
                h1 id="top" { (paper.title) }
                p."sw-wp-header__subtitle" { (paper.subtitle) }
                p."sw-wp-header__meta" { "Woodfine Research — PointSav Digital Systems · v1.0.0 · CC BY 4.0" }
            }
            div."sw-wp-layout" {
                aside."sw-wp-sidebar no-print" {
                    // Sidebar's own back-link (2026-09-24, operator-reported live
                    // audit) — matches bim/gis/design's identical pattern
                    // (`wp-sidebar__back` as the sidebar's first item, before the
                    // TOC nav). This crate's header already has one
                    // (`.sw-wp-header__back` above) for the mobile case where the
                    // sidebar itself is hidden, but the header isn't sticky/fixed
                    // on this site — scroll past it while reading and it, and the
                    // only way back to the index, disappears with it. The sidebar
                    // itself IS sticky, so its own back-link stays reachable at
                    // any scroll depth, closing that gap.
                    a."sw-wp-sidebar__back" href="/working-papers" { "← Back to Working Papers" }
                    nav aria-label="Sections in this paper" {
                        span."sw-wp-toc__label" { "On this page" }
                        a."sw-wp-toc__link" href="#top" { "Top of this paper" }
                        @for (id, text) in paper.toc {
                            a."sw-wp-toc__link" href=(format!("#{id}")) { (text) }
                        }
                    }
                }
                div."sw-wp-body" { (PreEscaped(paper.body_html)) }
            }
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

// scroll-margin-top: 24px on the body's own h2/h3 — the same cushion every
// other site's working-paper sections converged on today
// (BRIEF-journal-research-rendering.md item 23's follow-up, design.pointsav.com)
// — kept for cross-site visual consistency even though this crate's own
// non-sticky header means the flush-landing failure class can't occur here.
// Two-column grid (sidebar + body) below the header, sidebar sticky at the
// same 24px offset, hidden under 768px (matches `.wp-sidebar`'s own
// breakpoint on the other 5 sites) with the always-visible header back-link
// (above, in `item_markup`) covering the mobile case instead of stacking.
const ITEM_STYLE: &str = r#"<style>
.sw-working-paper{max-width:960px;margin:0 auto;padding:8px 0 48px}
.sw-wp-header__kicker{font-size:11px;text-transform:uppercase;letter-spacing:.08em;color:#6b7280;margin-bottom:8px}
.sw-wp-header__back{display:inline-block;font-size:13px;font-weight:600;color:#164679;text-decoration:none;margin-bottom:12px}
.sw-wp-header__back:hover{text-decoration:underline}
.sw-wp-header h1{font-size:24px;line-height:1.3;margin-bottom:8px;scroll-margin-top:24px}
.sw-wp-header__subtitle{font-size:14px;font-style:italic;color:#6b7280;margin-bottom:8px}
.sw-wp-header__meta{font-size:12px;color:#6b7280;margin-bottom:20px}
.sw-wp-layout{display:grid;grid-template-columns:220px minmax(0,1fr);gap:40px;align-items:start}
.sw-wp-sidebar{position:sticky;top:24px;display:flex;flex-direction:column}
.sw-wp-sidebar__back{display:inline-block;font-size:13px;font-weight:600;color:#164679;text-decoration:none;margin-bottom:16px}
.sw-wp-sidebar__back:hover{text-decoration:underline}
.sw-wp-toc__label{font-size:11px;font-weight:700;letter-spacing:.06em;text-transform:uppercase;color:#6b7280;margin-bottom:10px}
.sw-wp-toc__link{display:block;font-size:13px;line-height:1.5;color:#374151;text-decoration:none;padding:3px 0}
.sw-wp-toc__link:hover{color:#164679;text-decoration:underline}
.sw-wp-body{max-width:720px}
.sw-wp-body h2{font-size:19px;margin:30px 0 12px;color:#164679;scroll-margin-top:24px}
.sw-wp-body h3{font-size:16px;margin:22px 0 8px;scroll-margin-top:24px}
.sw-wp-body p{font-size:15px;line-height:1.75;color:#26303c;margin-bottom:14px}
.sw-wp-body table{width:100%;border-collapse:collapse;margin:16px 0;font-size:13px}
.sw-wp-body th,.sw-wp-body td{border:1px solid #e6e7e8;padding:8px 10px;text-align:left;vertical-align:top}
.sw-wp-body th{background:rgba(22,70,121,.06)}
.sw-wp-body hr{border:none;border-top:1px solid #e6e7e8;margin:26px 0}
@media (max-width:768px){
.sw-wp-layout{display:block}
.sw-wp-sidebar{display:none}
}
</style>"#;
