// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! The continuous chrome that wraps every page: `<head>`, sitenotice, sticky
//! white header, off-canvas mobile nav, and the institutional footer.
//!
//! `page()` is the single public entry — it composes the whole document as one
//! `html!{}` tree so maud balances every tag. Structure follows Wikipedia
//! Vector 2022; the visual system (white header, brand-as-accent) lives in
//! `static/{tokens,app}.css`. Class names match the `k-*` manifest.

use maud::{html, Markup, PreEscaped, DOCTYPE};
use serde_json::json;

use super::tenant::Tenant;
use crate::content::frontmatter::Author;
use crate::content::render::Heading;
use crate::content::IndexTopic;
use crate::history::{FileDiff, Revision};
use crate::legal::LegalTokens;
use crate::notice_text::{fill_template, NoticeText};

/// Serialize a `serde_json::Value` for embedding inside a literal
/// `<script type="application/ld+json">` block. `serde_json` already
/// produces valid JSON escaping (quotes, backslashes, control characters) —
/// the one thing it does NOT know about is HTML context: a string value
/// containing the literal bytes `</script` would still correctly close the
/// surrounding `<script>` tag early once parsed by the browser's HTML
/// tokenizer, regardless of it being valid JSON. `</` → `<\/` (a valid JSON
/// escape for `/`) neutralizes that without touching any real content.
fn jsonld(v: serde_json::Value) -> String {
    v.to_string().replace("</", "<\\/")
}

/// "article" / "articles" for a count.
fn count_word(n: usize) -> &'static str {
    if n == 1 {
        "article"
    } else {
        "articles"
    }
}

/// `<head>` contents (not the `<head>` element itself — `page()` supplies that).
/// `description` may be empty (e.g. listing pages) — the home page always
/// supplies one (see `app::home`), so `og:description` on the highest-value
/// URL is never silently absent. `path` is the canonical site-relative path
/// for this page (e.g. `/wiki/foo`, `/category/bar`); pass `""` for pages
/// that shouldn't declare a canonical URL (e.g. the 404 handler). `noindex`
/// emits `<meta name="robots" content="noindex">` — for surfaces with an
/// unbounded/query-driven URL space (search) that shouldn't be crawled even
/// though they're reachable and carry a canonical URL.
pub fn doc_head(
    title: &str,
    description: &str,
    tenant: Tenant,
    path: &str,
    noindex: bool,
) -> Markup {
    // Don't double-brand when the page title already is the site name (home).
    let full_title = if title == tenant.home_label() {
        title.to_string()
    } else {
        format!("{title} — {}", tenant.home_label())
    };
    let base = tenant.home_url();
    let base = base.trim_end_matches('/');
    let canonical_url = if path.is_empty() {
        String::new()
    } else {
        format!("{base}{path}")
    };
    let og_image = format!("{base}/static/og-image-{}.png", tenant.instance_str());
    html! {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        meta name="color-scheme" content="light dark";
        meta name="theme-color" content=(tenant.accent());
        title { (full_title) }
        @if !description.is_empty() {
            meta name="description" content=(description);
        }
        @if noindex {
            meta name="robots" content="noindex";
        }
        @if !canonical_url.is_empty() {
            link rel="canonical" href=(canonical_url);
        }
        meta property="og:type" content="website";
        meta property="og:site_name" content=(tenant.home_label());
        meta property="og:title" content=(full_title);
        @if !description.is_empty() {
            meta property="og:description" content=(description);
        }
        @if !canonical_url.is_empty() {
            meta property="og:url" content=(canonical_url);
        }
        meta property="og:image" content=(og_image);
        meta name="twitter:card" content="summary_large_image";
        // schema.org structured data — identifies the site + its publisher to
        // search engines and AI agents (SYS-ADR-07-safe: static, no user data).
        // `publisher` references the brand's apex-domain Organization node by
        // `@id` (per project-editorial's cross-site SEO standard) rather than
        // declaring an inline copy on every one of the 3 wikis — the apex
        // property is what actually defines that node. `potentialAction`
        // (site search) is low-priority per that same standard — Google
        // retired the sitelinks searchbox 2024-11-21 — but the real
        // /search?q= endpoint already exists, so it's cheap to include for
        // whatever still reads it.
        script type="application/ld+json" {
            (PreEscaped(jsonld(json!({
                "@context": "https://schema.org",
                "@type": "WebSite",
                "name": tenant.home_label(),
                "url": base,
                "publisher": {"@type": "Organization", "@id": tenant.organization_id()},
                "potentialAction": {
                    "@type": "SearchAction",
                    "target": format!("{base}/search?q={{search_term_string}}"),
                    "query-input": "required name=search_term_string",
                },
            }))))
        }
        link rel="icon" type="image/svg+xml" href="/static/favicon.svg";
        link rel="stylesheet" href="/static/fonts.css";
        link rel="stylesheet" href="/static/tokens.css";
        link rel="stylesheet" href="/static/app.css";
        link rel="stylesheet" href="/static/content.css";
        link rel="stylesheet" href="/static/syntax.css";
        // Pre-paint theme guard — sets data-theme before first paint (no flash).
        // Key 'k-theme' is shared with app.js.
        script {
            (PreEscaped(r#"(function(){try{var t=localStorage.getItem('k-theme');if(t!=='light'&&t!=='dark'){t=matchMedia('(prefers-color-scheme: dark)').matches?'dark':'light';}document.documentElement.setAttribute('data-theme',t);}catch(e){}})();"#))
        }
        // Pre-paint nav-collapse guard — mirrors the theme guard above, but in
        // the opposite direction: collapsed is the CSS *default* (no rule to
        // race), so this only matters for a reader who previously chose
        // "open" (key 'k-nav', shared with app.js's initNavCollapse()).
        script {
            (PreEscaped(r#"(function(){try{if(localStorage.getItem('k-nav')==='open'){document.documentElement.setAttribute('data-nav','open');}}catch(e){}})();"#))
        }
        // No-JS fallback: without JavaScript there is no way to reach the
        // toggle button, so the browse nav must stay reachable regardless —
        // force it open and hide the now-inert (non-functional) toggle.
        noscript {
            style {
                ".k-sidebar--reading .k-sidenav__browse{display:block}.k-sidebar--reading .k-nav-toggle{display:none}"
            }
        }
    }
}

/// A search block. Header and drawer copies use different input ids.
fn search_block(input_id: &str, query: &str) -> Markup {
    html! {
        div."k-search" {
            form."k-search__form" role="search" action="/search" method="get" {
                svg."k-search__icon" viewBox="0 0 20 20" aria-hidden="true" focusable="false" {
                    path d="M12.9 14.32a8 8 0 1 1 1.41-1.41l5.35 5.33-1.42 1.42-5.33-5.34zM8 14A6 6 0 1 0 8 2a6 6 0 0 0 0 12z" {}
                }
                label."k-visually-hidden" for=(input_id) { "Search this registry" }
                input."k-search__input" id=(input_id) type="search" name="q" value=(query)
                    placeholder="Search" autocomplete="off" spellcheck="false"
                    role="combobox" aria-expanded="false" aria-autocomplete="list"
                    aria-controls={ (input_id) "-suggest" };
                button."k-search__button" type="submit" { "Search" }
            }
            // Populated client-side from `/api/search-suggest` — a real
            // UX-review finding: search had no suggestions/typeahead at all.
            ul."k-search__suggestions" id={ (input_id) "-suggest" } role="listbox" hidden {}
        }
    }
}

/// Logo mark — a document-of-record glyph (folded-corner page) in currentColor,
/// which inherits `--k-accent` from `.k-logo`.
fn logo_mark() -> Markup {
    html! {
        svg."k-logo__mark" viewBox="0 0 24 24" width="22" height="22"
            aria-hidden="true" focusable="false" {
            path fill="currentColor"
                d="M6 2h7.5L19 7.5V22H6a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1zm7 1.6V8h4.4L13 3.6zM8 12h8v1.5H8V12zm0 3.5h8V17H8v-1.5z" {}
        }
    }
}

/// Top utility strip — cross-property links, mirroring the marketing site's
/// right-hand nav (Home · Monorepo · Design System · GitHub, per tenant).
/// External links open in a new tab.
pub fn utility_bar(tenant: Tenant) -> Markup {
    html! {
        div."k-utility" {
            div."k-utility__inner" {
                // Left: the maintaining entity → its corporate home.
                a."k-utility__home" href=(tenant.marketing_home()) {
                    (tenant.entity_name())
                }
                // Right: the property links (GitHub · Software · Design System).
                nav."k-utility__nav" aria-label="Network" {
                    @for (label, url) in tenant.cross_property_links() {
                        a."k-utility__link" href=(url) target="_blank" rel="noopener" {
                            (label)
                        }
                    }
                }
            }
        }
    }
}

/// Sticky white header: logo · search · controls.
pub fn header(tenant: Tenant, _lang: &str, query: &str) -> Markup {
    html! {
        header."k-header" role="banner" {
            div."k-header__inner" {
                div."k-header__start" {
                    a."k-logo" href="/" aria-label=(tenant.home_label()) {
                        (logo_mark())
                        span."k-logo__lockup" {
                            span."k-logo__brand" { (tenant.brand_word()) }
                            span."k-logo__descriptor" { (tenant.descriptor()) }
                        }
                    }
                }
                div."k-header__center" { (search_block("k-search-input", query)) }
                div."k-header__end" {
                    nav."k-controls" aria-label="Site controls" {
                        // Language toggle hidden until the /es routes ship (no dead link).
                        button."k-control k-control--theme" type="button"
                               aria-pressed="false" aria-label="Switch theme" {
                            svg."k-control__icon k-icon-moon" viewBox="0 0 20 20" aria-hidden="true" focusable="false" {
                                path d="M17 12.3A7 7 0 0 1 7.7 3 7 7 0 1 0 17 12.3z" {}
                            }
                            svg."k-control__icon k-icon-sun" viewBox="0 0 20 20" aria-hidden="true" focusable="false" {
                                path d="M10 3a1 1 0 0 1 1 1v1a1 1 0 1 1-2 0V4a1 1 0 0 1 1-1zm0 10a3 3 0 1 1 0-6 3 3 0 0 1 0 6zm0 2a1 1 0 0 1 1 1v1a1 1 0 1 1-2 0v-1a1 1 0 0 1 1-1zm7-5a1 1 0 0 1-1 1h-1a1 1 0 1 1 0-2h1a1 1 0 0 1 1 1zM5 10a1 1 0 0 1-1 1H3a1 1 0 1 1 0-2h1a1 1 0 0 1 1 1zm10.07-5.07a1 1 0 0 1 0 1.41l-.7.71a1 1 0 1 1-1.42-1.42l.71-.7a1 1 0 0 1 1.41 0zM6.05 13.95a1 1 0 0 1 0 1.41l-.71.71A1 1 0 0 1 3.93 14.66l.7-.71a1 1 0 0 1 1.42 0zm9.02.71a1 1 0 0 1-1.42 1.42l-.7-.71a1 1 0 0 1 1.41-1.41l.71.7zM6.05 6.05a1 1 0 0 1-1.42 0l-.7-.71A1 1 0 0 1 5.34 3.93l.71.7a1 1 0 0 1 0 1.42z" {}
                            }
                        }
                        button."k-control k-control--menu" type="button"
                               aria-controls="k-nav-drawer" aria-expanded="false"
                               aria-label="Open menu" {
                            svg."k-control__icon" viewBox="0 0 20 20" aria-hidden="true" focusable="false" {
                                path d="M3 5h14v2H3V5zm0 4h14v2H3V9zm0 4h14v2H3v-2z" {}
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Off-canvas mobile nav drawer + overlay (ships hidden; app.js manages state).
pub fn mobile_nav(tenant: Tenant, query: &str, cats: &[(String, String, String)]) -> Markup {
    // Same split `sidebar()` uses — the drawer previously dropped the wiki
    // entirely at mobile widths (search/Navigate/Resources/external links
    // only, no category browse at all; a real UX-review finding).
    let topics: Vec<&(String, String, String)> =
        cats.iter().filter(|(_, _, kind)| kind != "guide").collect();
    let guides: Vec<&(String, String, String)> =
        cats.iter().filter(|(_, _, kind)| kind == "guide").collect();
    html! {
        div."k-overlay" #"k-overlay" hidden {}
        div."k-nav-drawer" #"k-nav-drawer" role="dialog" aria-modal="true"
            aria-label=(format!("{} menu", tenant.home_label())) aria-hidden="true" hidden {
            div."k-nav-drawer__header" {
                span."k-nav-drawer__title" { "Menu" }
                button."k-nav-drawer__close" type="button" aria-label="Close menu" {
                    svg."k-control__icon" viewBox="0 0 20 20" aria-hidden="true" focusable="false" {
                        path d="M5 5l10 10M15 5L5 15" stroke="currentColor" stroke-width="2" fill="none" {}
                    }
                }
            }
            div."k-nav-drawer__body" {
                div."k-nav-drawer__search" { (search_block("k-search-input-mobile", query)) }
                section."k-nav-section" {
                    h2."k-nav-section__title" { "Navigate" }
                    ul."k-nav-list" {
                        li { a."k-nav-link" href="/" { "Home" } }
                        li { a."k-nav-link" href="/special/all-pages" { "Index of record" } }
                        li { a."k-nav-link" href="/special/recent-changes" { "Recent changes" } }
                    }
                }
                @if !topics.is_empty() {
                    section."k-nav-section" {
                        h2."k-nav-section__title" { "Topics" }
                        ul."k-nav-list" {
                            @for (slug, label, _) in &topics {
                                li { a."k-nav-link" href={ "/category/" (slug) } { (label) } }
                            }
                        }
                    }
                }
                @if !guides.is_empty() {
                    section."k-nav-section" {
                        h2."k-nav-section__title" { "Guides" }
                        ul."k-nav-list" {
                            @for (slug, label, _) in &guides {
                                li { a."k-nav-link" href={ "/category/" (slug) } { (label) } }
                            }
                        }
                    }
                }
                section."k-nav-section" {
                    h2."k-nav-section__title" { "Resources" }
                    ul."k-nav-list" {
                        li { a."k-nav-link" href="/feed.atom" { "Atom feed" } }
                    }
                }
                section."k-nav-section" {
                    h2."k-nav-section__title" { "PointSav network" }
                    ul."k-nav-list" {
                        li {
                            a."k-nav-link" href=(tenant.marketing_home()) target="_blank" rel="noopener" {
                                (tenant.entity_name())
                            }
                        }
                        @for (label, url) in tenant.cross_property_links() {
                            li { a."k-nav-link" href=(url) target="_blank" rel="noopener" { (label) } }
                        }
                    }
                }
            }
        }
    }
}

/// Footer — mirrors the marketing footer (cities line) with plain-language
/// link columns. Disclaimer and Contact live here only. Copyright holder and
/// trademark notice come from `legal` (loaded from the canonical
/// `legal-tokens-{brand}.yaml`, falling back to `LegalTokens::default()`).
pub fn footer(
    tenant: Tenant,
    legal: &LegalTokens,
    site_description: Option<&str>,
    article_count: usize,
) -> Markup {
    html! {
        footer."k-footer" role="contentinfo" {
            div."k-footer__inner" {
                // Brand re-anchor — repeats the site identity once the masthead
                // has scrolled off-screen on a long article. Tagline reuses the
                // site's own canonical description (site-footer recipe,
                // pointsav-design-system: "deliberately not a second hand-
                // authored copy, to avoid a second copy drifting out of sync") —
                // falls back to Tenant::tagline() only when no index.md
                // short_description exists.
                div."k-footer__brand" {
                    div."k-footer__brand-mark" { (logo_mark()) }
                    div."k-footer__brand-text" {
                        p."k-footer__brand-name" { (tenant.home_label()) }
                        p."k-footer__brand-tagline" { (site_description.unwrap_or_else(|| tenant.tagline())) }
                    }
                }
                div."k-footer__grid" {
                    div."k-footer__col" {
                        h2."k-footer__col-title" { "Browse" }
                        // One editorial fact line under the heading — the
                        // bim.woodfinegroup.com pattern (a real fact per
                        // column instead of a bare link list) applied
                        // minimally: one line, one column, real live data.
                        p."k-footer__col-fact" {
                            @if article_count == 1 { "1 article" }
                            @else { (article_count) " articles" }
                        }
                        ul."k-footer__list" {
                            li { a."k-footer__link" href="/" { "Home" } }
                            li { a."k-footer__link" href="/special/all-pages" { "All articles" } }
                            li { a."k-footer__link" href="/special/recent-changes" { "Recent changes" } }
                        }
                    }
                    div."k-footer__col" {
                        h2."k-footer__col-title" { "This site" }
                        ul."k-footer__list" {
                            li { a."k-footer__link" href="/wiki/about" { "About" } }
                            li { a."k-footer__link" href="/wiki/disclaimers" { "Disclaimer" } }
                            li { a."k-footer__link" href="/wiki/contact" { "Contact us" } }
                            li { a."k-footer__link" href="/wiki/page-privacy" { "Privacy" } }
                        }
                    }
                    div."k-footer__col" {
                        h2."k-footer__col-title" { "Network" }
                        ul."k-footer__list" {
                            li { a."k-footer__link" href=(tenant.marketing_home()) target="_blank" rel="noopener" { (tenant.entity_name()) } }
                            @for (label, url) in tenant.cross_property_links() {
                                li { a."k-footer__link" href=(url) target="_blank" rel="noopener" { (label) } }
                            }
                            // Cross-company link last — related but separate org.
                            @let (other_label, other_url) = tenant.other_org();
                            li { a."k-footer__link" href=(other_url) target="_blank" rel="noopener" { (other_label) } }
                        }
                    }
                    // Added 2026-09-01 (operator-directed footer redesign, Fable+Opus
                    // "best of both" consult): both bim.woodfinegroup.com and
                    // design.pointsav.com declare a real machine-readable surface in
                    // their footer — this one links only routes that actually exist
                    // and actually resolve. NOT /mcp: both models independently
                    // verified against this crate's own src/main.rs that MCP here is
                    // stdio-only (`Command::Mcp`, provisional), no HTTP route — a
                    // literal bim/design-style "/mcp" link would 404.
                    div."k-footer__col" {
                        h2."k-footer__col-title" { "Machine-readable" }
                        ul."k-footer__list" {
                            li { a."k-footer__link" href="/llms.txt" { "llms.txt" } }
                            li { a."k-footer__link" href="/feed.atom" { "Atom feed" } }
                            li { a."k-footer__link" href="/sitemap.xml" { "Sitemap" } }
                            li { a."k-footer__link" href="/healthz" { "Health" } }
                        }
                    }
                }
                // Identity bar — collapsed to 2 rows 2026-08-25/26 (Command/
                // project-editorial UI-fix batch, exact diff per msg
                // command-20260826-footer-redesign-exact-css-layout-rs-mark;
                // real cross-model live measurement: the prior 4-row stack
                // measured 113px tall, 58-67% as tall as the entire nav grid
                // above it). Row 1 (.k-footer__baseline): 3 direct children
                // (cities, badges, copyright) — justify-content:
                // space-between spreads them without a manual trailing-
                // margin hack. Row 2 (.k-footer__fineprint): one full-width
                // paragraph (disclaimer + trademark merged, no 60ch cap —
                // the cap was what forced the trademark text to wrap to 5
                // lines on its own row). The real mobile-legibility fix
                // from the 2026-07-15 restructure (badges no longer buried
                // under legal text) is kept — mobile stacking is scoped to
                // <=768px only, see the media query below, not undone by
                // this collapse.
                div."k-footer__identity" {
                    div."k-footer__baseline" {
                        div."k-footer__cities" {
                            // Middot separator, not pipe — site-footer recipe's
                            // content_conventions.separator: "the live sites'
                            // current 'Vancouver | New York' is the one
                            // inconsistency this component corrects."
                            @for (i, city) in tenant.cities().iter().enumerate() {
                                @if i > 0 { span."k-footer__cities-sep" aria-hidden="true" { "\u{00b7}" } }
                                span { (city) }
                            }
                        }
                        div."k-footer__badges" {
                            // Powered by MediaKit (the engine).
                            a."k-badge" href="/wiki/about" {
                                span."k-badge__glyph" aria-hidden="true" {
                                    svg viewBox="0 0 24 24" width="15" height="15" {
                                        path fill="currentColor" d="M3 5.5A1.5 1.5 0 0 1 4.5 4h15A1.5 1.5 0 0 1 21 5.5v13A1.5 1.5 0 0 1 19.5 20h-15A1.5 1.5 0 0 1 3 18.5v-13zM6 8v8l3.2-2.4L6 8zm7 6.5h5V13h-5v1.5zm0-3h5V10h-5v1.5z" {}
                                    }
                                }
                                span."k-badge__text" {
                                    span."k-badge__lead" { "Powered by" }
                                    span."k-badge__name" { "MediaKit" }
                                }
                            }
                            // Content licence — per tenant (CC BY for the open docs
                            // library; CC BY-ND for the verbatim disclosure records).
                            a."k-badge k-badge--license" href=(tenant.license_url())
                              target="_blank" rel="noopener license"
                              aria-label={ "Content licensed " (tenant.license_name()) } {
                                span."k-badge__cc" aria-hidden="true" {
                                    img."k-cc-icon" src="/static/cc.svg" alt="" width="20" height="20";
                                    img."k-cc-icon" src="/static/cc-by.svg" alt="" width="20" height="20";
                                    @if tenant.license_nd() {
                                        img."k-cc-icon" src="/static/cc-nd.svg" alt="" width="20" height="20";
                                    }
                                }
                                span."k-badge__text" {
                                    span."k-badge__lead" { "Licensed" }
                                    span."k-badge__name" { (tenant.license_name()) }
                                }
                            }
                        }
                        p."k-footer__copyright" {
                            "\u{00a9} 2026 " (legal.copyright.holder)
                        }
                    }
                    // Fine-print paragraph — disclaimer + trademark notice merged
                    // (was 2 separate rows, one capped at 60ch which forced the
                    // trademark statement to wrap 5 lines). Trademark text is
                    // sourced from the canonical legal-tokens-{brand}.yaml
                    // (factory-release-engineering), not hardcoded here. The marks
                    // are reserved independently of the CC BY 4.0 content licence,
                    // so no blanket "all rights reserved" (content is openly
                    // licensed).
                    p."k-footer__fineprint" {
                        (tenant.disclaimer_line())
                        " \u{00b7} "
                        (legal.trademarks.statement)
                    }
                }
            }
        }
    }
}

/// Format an ISO `YYYY-MM-DD` as "25 May 2026"; pass anything else through.
fn format_date(iso: &str) -> String {
    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let p: Vec<&str> = iso.trim().split('-').collect();
    if p.len() == 3 {
        if let (Ok(y), Ok(m), Ok(d)) = (
            p[0].parse::<i32>(),
            p[1].parse::<usize>(),
            p[2].parse::<u32>(),
        ) {
            if (1..=12).contains(&m) {
                return format!("{d} {} {y}", MONTHS[m - 1]);
            }
        }
    }
    iso.to_string()
}

/// The article action-tab bar (Wikipedia Vector 2022 pattern). `active` is
/// "article" or "history"; the current one is a non-link span, the other links.
/// (No "Notes" placeholder — a reviewer-annotation channel ships as a real tab or
/// not at all; no dead controls in front of auditors.)
/// `active` is "article" or "history" — the current one is a non-link span,
/// the other links. `slug` is always the *content's own file slug* (used for
/// the History link regardless of `hub`; an Index Topic's own slug, e.g.
/// `security-index`, not its category id). `hub`, when `Some(category_slug)`,
/// is what makes this the Index Topic variant: the first tab reads
/// "Overview" and links to `/category/{category_slug}` — the canonical hub
/// URL a reader should land back on — instead of "Article" →
/// `/wiki/{slug}`. `None` is the ordinary-article behavior, unchanged.
fn tab_bar(slug: &str, active: &str, hub: Option<&str>) -> Markup {
    let (first_label, first_href) = match hub {
        Some(category_slug) => ("Overview", format!("/category/{category_slug}")),
        None => ("Article", format!("/wiki/{slug}")),
    };
    html! {
        nav."k-tabs" aria-label="Views" {
            @if active == "article" {
                span."k-tab k-tab--active" aria-current="page" { (first_label) }
            } @else {
                a."k-tab" href=(first_href) { (first_label) }
            }
            @if active == "history" {
                span."k-tab k-tab--active" aria-current="page" { "History" }
            } @else {
                a."k-tab" href={ "/history/" (slug) } { "History" }
            }
        }
    }
}

/// Wrap a rendered article body in the reading shell: action tabs (+ "Last
/// updated"), ruled title, prose column. `body_html` is trusted, pre-rendered.
/// `sha` is the short commit hash the render is drawn from (provenance line);
/// `asof` is set only for the point-in-time view (a historical revision) and
/// carries that revision's date, which switches the meta label + shows a banner.
/// `badge` is a plain-text content-type label next to the H1 (currently only
/// "Index"/"Índice" for Index Topics — ordinary TOPIC/GUIDE articles pass
/// `None`; there's no icon or color block, matching the site's understated
/// register). The caller resolves the exact label string (language included)
/// since it already knows which language the loaded file actually is.
#[allow(clippy::too_many_arguments)]
pub fn article(
    title: &str,
    slug: &str,
    updated: Option<&str>,
    sha: Option<&str>,
    asof: Option<&str>,
    alt_lang: Option<(&str, &str)>,
    badge: Option<&str>,
    body_html: &str,
) -> Markup {
    html! {
        article."k-article" {
            div."k-article-nav" {
                (tab_bar(slug, "article", None))
                @if updated.is_some() || asof.is_some() || sha.is_some() {
                    p."k-article__meta" {
                        @if let Some(d) = asof {
                            "Revision as of "
                            time."k-article__date" datetime=(d) { (format_date(d)) }
                        } @else if let Some(d) = updated.filter(|s| !s.trim().is_empty()) {
                            "Last updated "
                            time."k-article__date" datetime=(d) { (format_date(d)) }
                        }
                        @if let Some(s) = sha {
                            " \u{00b7} " code."k-article__sha" { (s) }
                        }
                        @if let Some((url, label)) = alt_lang {
                            " \u{00b7} " a."k-article__lang" href=(url) { (label) }
                        }
                    }
                }
            }
            @if let Some(d) = asof {
                div."k-asof" role="note" {
                    strong { "Historical revision" }
                    " — this record as it stood on " (format_date(d)) ", not the current version. "
                    a."k-asof__link" href={ "/wiki/" (slug) } { "View the current record \u{2192}" }
                }
            }
            h1."k-article__title" {
                (title)
                @if let Some(b) = badge {
                    span."k-content-badge" { (b) }
                }
            }
            div."k-prose" { (PreEscaped(body_html)) }
            @if asof.is_none() {
                // Print-only citation stamp (Phase 9 — .k-print-citation is
                // display:none on screen, shown only in @media print).
                // Placed after the body, not before it — Wikipedia's own
                // "Retrieved from ..." line is the article's closing record,
                // not a header (2026-07-15 revision, see BRIEF-print-mode.md).
                // A historical (asof) view already carries its own "Revision
                // as of" banner above, so this doesn't duplicate for that case.
                p."k-print-citation" {
                    "Cite this record: /wiki/" (slug)
                    @if let Some(s) = sha { " \u{2014} revision " code { (s) } }
                    @if let Some(d) = updated.filter(|s| !s.trim().is_empty()) {
                        ", last updated " (format_date(d))
                    }
                    "."
                }
            }
        }
    }
}

/// Article revision history — the git log of the article's file (the History tab).
pub fn history_page(title: &str, slug: &str, issuer: &str, revs: &[Revision]) -> Markup {
    html! {
        article."k-article" {
            div."k-article-nav" { (tab_bar(slug, "history", None)) }
            h1."k-article__title" { (title) }
            div."k-home__stat" { strong { (revs.len()) } " " (count_word_rev(revs.len())) }
            p."k-history__note" {
                "Maintained by " (issuer) ". Each revision is content-addressed by its commit hash."
            }
            @if revs.is_empty() {
                p."k-searchpage__hint" {
                    "No revision history found for this article — it may not yet be committed to the content repository."
                }
            } @else {
                ul."k-history" {
                    @for r in revs {
                        @if r.redacted {
                            li."k-history__item k-history__item--redacted" {
                                time."k-history__date" datetime=(r.date_iso) { (format_date(&r.date_iso)) }
                                span."k-history__msg k-history__msg--redacted" {
                                    "Revision redacted — superseded by a later correction"
                                }
                                span."k-history__meta" {
                                    code."k-history__sha" { (r.short_sha) }
                                }
                            }
                        } @else {
                            li."k-history__item" {
                                time."k-history__date" datetime=(r.date_iso) { (format_date(&r.date_iso)) }
                                a."k-history__msg" href={ "/history/" (slug) "?rev=" (r.sha) } { (r.message) }
                                span."k-history__meta" {
                                    code."k-history__sha" { (r.short_sha) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Shown in place of `diff_page` when the requested revision is covered by a
/// `redactions.yaml` entry (2026-09-06 history-exposure decision) — the
/// content/diff is hidden, but the page still confirms a correction exists
/// rather than 404ing, per "hide the content, never the fact."
pub fn redacted_notice(title: &str, slug: &str, reason: Option<&str>) -> Markup {
    html! {
        article."k-article" {
            div."k-article-nav" { (tab_bar(slug, "history", None)) }
            h1."k-article__title" { (title) }
            div."k-diff__head" {
                a."k-diff__back" href={ "/history/" (slug) } { "\u{2190} All revisions" }
            }
            aside."k-notice-banner" role="note" {
                p {
                    "This revision has been redacted and superseded by a later correction."
                    @if let Some(r) = reason.filter(|s| !s.is_empty()) {
                        " " (r)
                    }
                }
            }
        }
    }
}

/// "revision" / "revisions" for a count.
fn count_word_rev(n: usize) -> &'static str {
    if n == 1 {
        "revision"
    } else {
        "revisions"
    }
}

fn diff_line_class(origin: char) -> &'static str {
    match origin {
        '+' => "k-diff__line k-diff__line--add",
        '-' => "k-diff__line k-diff__line--del",
        'H' => "k-diff__line k-diff__line--hunk",
        _ => "k-diff__line",
    }
}

/// A single revision's diff for one article (reached from the History tab).
pub fn diff_page(title: &str, slug: &str, issuer: &str, diff: &FileDiff) -> Markup {
    html! {
        article."k-article" {
            div."k-article-nav" { (tab_bar(slug, "history", None)) }
            h1."k-article__title" { (title) }
            div."k-diff__head" {
                a."k-diff__back" href={ "/history/" (slug) } { "\u{2190} All revisions" }
                p."k-diff__meta" {
                    code."k-history__sha" { (diff.short_sha) }
                    " \u{00b7} " (issuer)
                    " \u{00b7} " time datetime=(diff.date_iso) { (format_date(&diff.date_iso)) }
                }
                p."k-diff__msg" { (diff.message) }
                p."k-diff__asof" {
                    a href={ "/wiki/" (slug) "?rev=" (diff.short_sha) } {
                        "View the full record as of this revision \u{2192}"
                    }
                }
            }
            @if diff.lines.is_empty() {
                p."k-searchpage__hint" { "No textual changes to this file in this revision." }
            } @else {
                pre."k-diff" {
                    @for l in &diff.lines {
                        span class=(diff_line_class(l.origin)) { (l.content) "\n" }
                    }
                }
            }
        }
    }
}

/// Home page — the front page (Main Page): title, the index lede, an article
/// count, and a "Browse by area" grid of category cards. `cats` is
/// `(slug, label, count)` in display order.
pub fn home_page(
    tenant: Tenant,
    lede_html: &str,
    total: usize,
    cats: &[(String, String, usize)],
    guides: &[(String, String, String)],
) -> Markup {
    let guides_shown = guides.len().min(8);
    html! {
        div."k-home" {
            h1."k-article__title" { (tenant.home_label()) }
            @if !lede_html.is_empty() {
                div."k-prose k-home__lede" { (PreEscaped(lede_html)) }
            }
            div."k-home__stat" {
                strong { (total) } " " (count_word(total)) " in the registry"
            }
            section."k-home__browse" aria-label="Browse by area" {
                h2."k-home__browse-title" { "Browse by area" }
                div."k-home__grid" {
                    @for (slug, label, count) in cats {
                        a."k-cat-card" href={ "/category/" (slug) } {
                            span."k-cat-card__name" { (label) }
                            span."k-cat-card__count" { (count) " " (count_word(*count)) }
                        }
                    }
                }
            }

            // How-to guides — operational runbooks, distinct from the reference topics.
            @if !guides.is_empty() {
                section."k-home__guides" aria-label="How-to guides" {
                    div."k-home__guides-head" {
                        h2."k-home__browse-title" { "How-to guides" }
                        a."k-home__browse-all" href="/category/how-to" {
                            "All " (guides.len()) " guides \u{2192}"
                        }
                    }
                    p."k-home__guides-lede" {
                        "Step-by-step operational runbooks — how to install, configure, and run the platform."
                    }
                    ul."k-guide-list" {
                        @for (slug, title, desc) in guides.iter().take(guides_shown) {
                            li."k-guide-card" {
                                a."k-guide-card__title" href={ "/wiki/" (slug) } {
                                    span."k-guide-list__icon" aria-hidden="true" {
                                        svg viewBox="0 0 16 16" width="14" height="14" {
                                            path fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" d="M6 3.5 10.5 8 6 12.5" {}
                                        }
                                    }
                                    (title)
                                }
                                @if !desc.is_empty() {
                                    p."k-guide-card__desc" { (desc) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Category listing — a category's articles as a scannable index.
/// `docs` is `(slug, title, description)` sorted.
pub fn category_index(label: &str, docs: &[(String, String, String)]) -> Markup {
    html! {
        div."k-catpage" {
            div."k-catpage__eyebrow" { "Category" }
            h1."k-article__title" { (label) }
            div."k-home__stat" { strong { (docs.len()) } " " (count_word(docs.len())) }
            ul."k-cat-list" {
                @for (slug, title, desc) in docs {
                    li."k-cat-entry" {
                        a."k-cat-entry__title" href={ "/wiki/" (slug) } { (title) }
                        @if !desc.is_empty() {
                            p."k-cat-entry__desc" { (desc) }
                        }
                    }
                }
            }
        }
    }
}

/// The header + chrome above an Index Topic rendered at its category's own
/// `/category/{slug}` URL. Unified with the ordinary Article chrome — tab
/// bar (Overview/History) + "Last updated · sha" — rather than the plain
/// `.k-catpage__eyebrow` label: an Index Topic is real, file-backed,
/// human-authored content with real git history, same as any other article,
/// so a plain metadata eyebrow with no history/provenance would be
/// dishonest chrome. The eyebrow itself is untouched and still used by
/// every category that does *not* have `index_type:` set — those are
/// genuinely engine-synthesized listings with no file/history/sha behind
/// them (see `category_index()`) — this is a deliberate scope boundary
/// ("chrome follows provenance"), not an inconsistency.
///
/// `index_slug` is the `_index.md` file's own slug (e.g. `security-index`)
/// — used for the History tab's link, since that resolves against the
/// file's real slug, not the category id. `category_slug` is only used for
/// the Overview tab's link and the safety-net link — the canonical URL a
/// reader should land back on is the category page, not the raw file route.
/// `total` is the category's real article count
/// (`ContentIndex::in_category(name).len()`), not the sum of the Index
/// Topic's own group counts — the two can legitimately differ.
pub fn index_topic_header(
    label: &str,
    total: usize,
    category_slug: &str,
    index_slug: &str,
    updated: Option<&str>,
    sha: Option<&str>,
) -> Markup {
    html! {
        div."k-article-nav" {
            (tab_bar(index_slug, "article", Some(category_slug)))
            @if updated.is_some() || sha.is_some() {
                p."k-article__meta" {
                    @if let Some(d) = updated.filter(|s| !s.trim().is_empty()) {
                        "Last updated "
                        time."k-article__date" datetime=(d) { (format_date(d)) }
                    }
                    @if let Some(s) = sha {
                        " \u{00b7} " code."k-article__sha" { (s) }
                    }
                }
            }
        }
        h1."k-article__title" { (label) }
        p."k-index-topic__see-all" {
            a href={ "/category/" (category_slug) "?view=all" } {
                "See all " (total) " " (count_word(total)) " in " (label) " \u{2192}"
            }
        }
    }
}

/// An Index Topic's body: the highlighted "start here" pick, each curated
/// group with a live member-count pill and calm annotated link list, then any
/// trailing prose ("What this is not", "See also") as authored.
///
/// The "start here" block is rendered as a styled card containing its
/// `prose_html` as-is — deliberately *not* also wrapped in a separate `<a
/// href=(sh.href)>`: `prose_html` already contains its own inline link (the
/// wikilink resolved as part of "**Start here:** [[...]]..."), and wrapping
/// that in another anchor would nest `<a>` inside `<a>` — invalid HTML and an
/// ambiguous click target. `StartHere.href`/`.label` exist for callers that
/// need the target structurally (e.g. future structured data), not for this
/// render.
///
/// Each member's wikilink, by contrast, *is* rendered as a distinct link
/// (`m.href`/`m.label`) followed by `annotation_html` — `parse_index_topic`
/// already strips the wikilink out of the annotation text, so there's no
/// duplication risk there.
pub fn index_topic_body(topic: &IndexTopic) -> Markup {
    html! {
        div."k-index-topic" {
            @if !topic.intro_html.is_empty() {
                div."k-prose" { (PreEscaped(topic.intro_html.clone())) }
            }
            @if let Some(sh) = &topic.start_here {
                aside."k-index-topic__start-here" aria-label="Start here" {
                    span."k-index-topic__start-here-eyebrow" { "Start here" }
                    div."k-prose" { (PreEscaped(sh.prose_html.clone())) }
                }
            }
            @for group in &topic.groups {
                section."k-index-group" {
                    div."k-index-group__head" {
                        h2."k-index-group__title" { (group.title) }
                        span."k-index-group__count" { (group.count()) " " (count_word(group.count())) }
                    }
                    @if let Some(intro) = &group.intro_html {
                        div."k-prose k-index-group__intro" { (PreEscaped(intro.clone())) }
                    }
                    ul."k-index-list" {
                        @for m in &group.members {
                            li."k-index-list__item" {
                                a."k-index-list__link" href=(m.href) { (m.label) }
                                span."k-index-list__annotation" { (PreEscaped(m.annotation_html.clone())) }
                            }
                        }
                    }
                }
            }
            @if !topic.tail_html.is_empty() {
                div."k-prose k-index-topic__tail" { (PreEscaped(topic.tail_html.clone())) }
            }
        }
    }
}

/// A generic index page — "Index of record" (A–Z all articles) and "Recent
/// changes". `items` is `(slug, title, meta)`; `meta` is a description or a date.
pub fn special_list(heading: &str, eyebrow: &str, items: &[(String, String, String)]) -> Markup {
    html! {
        div."k-catpage" {
            div."k-catpage__eyebrow" { (eyebrow) }
            h1."k-article__title" { (heading) }
            div."k-home__stat" { strong { (items.len()) } " " (count_word(items.len())) }
            ul."k-cat-list" {
                @for (slug, title, meta) in items {
                    li."k-cat-entry" {
                        a."k-cat-entry__title" href={ "/wiki/" (slug) } { (title) }
                        @if !meta.is_empty() { p."k-cat-entry__desc" { (meta) } }
                    }
                }
            }
        }
    }
}

/// A minimal chrome-wrapped message page (used for 404 — never a bare error).
pub fn simple_message(heading: &str, text: &str) -> Markup {
    html! {
        div."k-catpage" {
            h1."k-article__title" { (heading) }
            p."k-searchpage__hint" { (text) }
            p { a href="/" { "\u{2190} Back to the main page" } }
        }
    }
}

/// Search results page — a query box plus result cards (same card style as the
/// category listing). `results` is `(slug, title, description)`, ranked.
pub fn search_results(query: &str, results: &[(String, String, String)]) -> Markup {
    let q = query.trim();
    html! {
        div."k-catpage" {
            // No eyebrow here, unlike other `.k-catpage` renders — the H1
            // already reads "Search" one line below; the eyebrow would just
            // repeat it verbatim (a real UX-review finding, 2026-08-23).
            h1."k-article__title" { "Search" }
            // The header search bar carries the query — no second on-page box.
            @if q.is_empty() {
                p."k-searchpage__hint" { "Use the search bar above to search article titles and text." }
            } @else {
                div."k-home__stat" {
                    strong { (results.len()) } " " (count_word(results.len()))
                    " for \u{201c}" (q) "\u{201d}"
                }
                @if results.is_empty() {
                    p."k-searchpage__hint" { "No articles matched. Try different or fewer terms." }
                } @else {
                    ul."k-cat-list" {
                        @for (slug, title, desc) in results {
                            li."k-cat-entry" {
                                a."k-cat-entry__title" href={ "/wiki/" (slug) } { (title) }
                                @if !desc.is_empty() {
                                    p."k-cat-entry__desc" { (desc) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Article table of contents — a `k-sidenav__group` on its own, present only
/// on pages with headings. Extracted so `sidebar()` can place it either above
/// or below the browse nav depending on whether this is a "reading page".
fn toc_nav(toc: &[Heading]) -> Markup {
    html! {
        @if !toc.is_empty() {
            nav."k-sidenav__group k-toc" aria-label="Contents" {
                h2."k-sidenav__heading" { "Contents" }
                ul."k-toc__list" {
                    @for h in toc {
                        li."k-toc__item"."k-toc__item--sub"[h.level == 3] {
                            a."k-toc__link" href={ "#" (h.id) } { (h.text) }
                        }
                    }
                }
            }
        }
    }
}

/// The left navigation column (Wikipedia Vector 2022 pattern): Main page,
/// [Contents], Topics, Guides. Sticky on desktop; hidden below the tablet
/// breakpoint where the off-canvas drawer covers navigation. `cats` is
/// `(slug, label, kind)` — `kind` (`"topic"`/`"guide"`, from
/// `categories.yaml`, see `sitedata::Category`) is the section a category
/// renders under. Reading it structurally, rather than hardcoding a
/// `how-to`-id special case (the previous approach), fixes a real duplicate —
/// `how-to` used to render both in the flat category list AND in a separate
/// hardcoded "Guides" block — and means a future guide-category split needs
/// zero engine change: a new category just carries `kind: guide` and appears
/// in the right section automatically. Anything not `"guide"` renders under
/// Topics (not just `"topic"` exactly) so a missing/malformed `kind` value
/// never silently drops a category from the nav.
///
/// A page with a non-empty `toc` is a "reading page" (the current-article and
/// as-of-revision views — every other page type passes `&[]`, and Index Topic
/// pages do too, deliberately, so they keep the full browse sidebar "for
/// free" as a browsing surface). On a reading page: the TOC renders first
/// (the article's own contents, not buried under the full category list), a
/// `.k-sidebar--reading` marker class is added, and the Topics/Guides nav is
/// CSS-default-collapsed behind a toggle button — see `.k-sidebar--reading`
/// in `app.css` and `initNavCollapse()` in `app.js`.
/// `current_category` marks the active Topics/Guides link with `aria-current`
/// (a real UX-review finding: no "you are here" existed anywhere in the
/// sidebar). `siblings` — `(slug, title)`, current article already excluded
/// by the caller — renders an "In this topic" list above the Browse toggle,
/// the single structural feature every hyperscaler-docs benchmark shares
/// that this engine previously lacked entirely.
fn sidebar(
    cats: &[(String, String, String)],
    toc: &[Heading],
    current_category: Option<&str>,
    siblings: &[(String, String)],
) -> Markup {
    let reading = !toc.is_empty();
    let topics: Vec<&(String, String, String)> =
        cats.iter().filter(|(_, _, kind)| kind != "guide").collect();
    let guides: Vec<&(String, String, String)> =
        cats.iter().filter(|(_, _, kind)| kind == "guide").collect();
    let has_browse = !cats.is_empty();
    html! {
        aside."k-sidebar"."k-sidebar--reading"[reading] aria-label="Site navigation" {
            nav."k-sidenav" {
                a."k-sidenav__home" href="/" { "Main page" }
                (toc_nav(toc))
                @if !siblings.is_empty() {
                    div."k-sidenav__group" {
                        h2."k-sidenav__heading" { "In this topic" }
                        ul."k-sidenav__list" {
                            @for (slug, title) in siblings {
                                li { a."k-sidenav__link" href={ "/wiki/" (slug) } { (title) } }
                            }
                        }
                    }
                }
                @if has_browse {
                    button."k-nav-toggle" type="button"
                        aria-expanded="false" aria-controls="k-sidenav-browse" {
                        "Browse"
                    }
                    nav."k-sidenav__browse" #"k-sidenav-browse" {
                        @if !topics.is_empty() {
                            div."k-sidenav__group" {
                                h2."k-sidenav__heading" { "Topics" }
                                ul."k-sidenav__list" {
                                    @for (slug, label, _) in &topics {
                                        @let active = current_category == Some(slug.as_str());
                                        li {
                                            a."k-sidenav__link"
                                                href={ "/category/" (slug) }
                                                aria-current=[active.then_some("page")]
                                                { (label) }
                                        }
                                    }
                                }
                            }
                        }
                        @if !guides.is_empty() {
                            div."k-sidenav__group" {
                                h2."k-sidenav__heading" { "Guides" }
                                ul."k-sidenav__list" {
                                    @for (slug, label, _) in &guides {
                                        @let active = current_category == Some(slug.as_str());
                                        li {
                                            a."k-sidenav__link"
                                                href={ "/category/" (slug) }
                                                aria-current=[active.then_some("page")]
                                                { (label) }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// `<link rel="alternate" hreflang="...">` pair for a page with a genuine
/// translation counterpart — composed onto `doc_head`'s output (not a
/// `doc_head` parameter: only article pages ever have a translation, so
/// growing every call site's signature for one caller wasn't worth it).
/// `current`/`alt` are each `(lang_code, absolute_url)`.
pub fn hreflang_links(current: (&str, &str), alt: (&str, &str)) -> Markup {
    html! {
        link rel="alternate" hreflang=(current.0) href=(current.1);
        link rel="alternate" hreflang=(alt.0) href=(alt.1);
    }
}

/// A visible "Home › Category › Article" trail — a real finding: corporate's
/// 66-article/12-category taxonomy had no breadcrumb trail anywhere. `trail`
/// is `(href, label)` for every step except the final (current) one, which
/// is rendered as plain text, not a link.
pub fn breadcrumb(trail: &[(String, String)], current: &str) -> Markup {
    html! {
        nav."k-breadcrumb" aria-label="Breadcrumb" {
            ol."k-breadcrumb__list" {
                @for (href, label) in trail {
                    li."k-breadcrumb__item" {
                        a."k-breadcrumb__link" href=(href) { (label) }
                        span."k-breadcrumb__sep" aria-hidden="true" { "\u{203a}" }
                    }
                }
                li."k-breadcrumb__item" aria-current="page" { (current) }
            }
        }
    }
}

/// `BreadcrumbList` structured data for the same trail `breadcrumb()` renders
/// visibly — a real finding: corporate's 66-article/12-category taxonomy had
/// none. `items` is `(absolute_url, label)` for the FULL trail *including*
/// the current page (unlike `breadcrumb()`'s `trail`, which excludes it —
/// `ListItem.item` is required by the schema even for the last entry).
pub fn breadcrumb_jsonld(items: &[(String, String)]) -> Markup {
    let list: Vec<serde_json::Value> = items
        .iter()
        .enumerate()
        .map(|(i, (url, name))| {
            json!({
                "@type": "ListItem",
                "position": i + 1,
                "name": name,
                "item": url,
            })
        })
        .collect();
    html! {
        script type="application/ld+json" {
            (PreEscaped(jsonld(json!({
                "@context": "https://schema.org",
                "@type": "BreadcrumbList",
                "itemListElement": list,
            }))))
        }
    }
}

/// Page-level `TechArticle` JSON-LD — a real finding: only the site-level
/// `WebSite` entity existed anywhere, no per-article structured data, despite
/// the render context already carrying everything this needs (title,
/// description, last-updated date). `author` references the same brand
/// Organization `@id` as the site-level `WebSite.publisher` (never an inline
/// copy) — per project-editorial's cross-site SEO standard, explicitly
/// flagged there as the one place a copy-paste error would be easy to make
/// (`corporate.`/`projects.` reference woodfinegroup.com's node,
/// `documentation.` references pointsav.com's, despite sharing this exact
/// code path) — `Tenant::organization_id()` is the single source for both
/// call sites, so they cannot drift apart.
pub fn article_jsonld(
    tenant: Tenant,
    title: &str,
    description: &str,
    url: &str,
    date_modified: Option<&str>,
) -> Markup {
    let mut obj = json!({
        "@context": "https://schema.org",
        "@type": "TechArticle",
        "headline": title,
        "url": url,
        "author": {"@type": "Organization", "@id": tenant.organization_id()},
        "isPartOf": {"@type": "WebSite", "url": tenant.home_url().trim_end_matches('/')},
    });
    if !description.is_empty() {
        obj["description"] = json!(description);
    }
    if let Some(dm) = date_modified {
        obj["dateModified"] = json!(dm);
    }
    html! {
        script type="application/ld+json" {
            (PreEscaped(jsonld(obj)))
        }
    }
}

/// JOURNAL paper masthead (SPEC-journal-wiki-render-contract.md §5 item 1):
/// title + authors + affiliations + correspondence, generated from
/// frontmatter. A JOURNAL body has no h1 — this element IS the title (the
/// engine owns it, same as it owns References and the notice banners; see
/// §1.2 item 4 for why the body never writes any of these itself).
/// Correspondence renders for every author carrying an email — the frontmatter
/// schema has no dedicated "is corresponding author" flag, so this is a
/// simplification (all listed emails are contactable), not a spec-mandated
/// single-author pick.
pub fn masthead(title: &str, authors: &[Author]) -> Markup {
    html! {
        header."k-masthead" {
            h1."k-masthead__title" { (title) }
            @if !authors.is_empty() {
                p."k-masthead__authors" {
                    @for (i, author) in authors.iter().enumerate() {
                        @if i > 0 { span."k-masthead__author-sep" aria-hidden="true" { ", " } }
                        span."k-masthead__author" {
                            @if let Some(name) = author.name.as_deref() { (name) }
                            @if let Some(aff) = author.affiliation.as_deref().filter(|a| !a.is_empty()) {
                                sup."k-masthead__affiliation" { (aff) }
                            }
                        }
                    }
                }
            }
            @for author in authors {
                @if let Some(email) = author.email.as_deref().filter(|e| !e.is_empty()) {
                    p."k-masthead__correspondence" {
                        "Correspondence: "
                        a href={ "mailto:" (email) } { (email) }
                    }
                }
            }
        }
    }
}

/// Working-paper / disclosure notice banner (SPEC §4) — wired 2026-09-05 once
/// Command placed the canonical notice-text data source (routed 2026-07-10 by
/// project-editorial, placed `factory-release-engineering` commit `2ab879c`).
/// The banner text is disclosure copy this engine must never author locally —
/// every word loads verbatim from `notice_text::NoticeText`, the same
/// discipline `legal.rs` already follows for trademark/copyright text.
/// `notice` is `None` when the token file is absent/malformed — renders
/// nothing rather than fabricating disclosure text, same fallback discipline.
///
/// Which template renders is driven by the paper's own `state:` frontmatter
/// field: `draft`/`under-review` → working-paper notice + the static
/// forward-looking-statements advisory; `published` → citation banner. A doc
/// with no `state:` set, or any other value, renders no banner — there is
/// nothing yet to disclose. `archived`/superseded-notice is a **known,
/// deliberate gap**: its template needs a `revision_history.latest.*` value
/// this crate's `Frontmatter` has no field for yet (the notice-text file
/// itself flags this template "not yet needed" — no paper has reached
/// `archived` as of the 2026-07-10 draft) — add that frontmatter field before
/// wiring this state, don't guess at placeholder values.
#[allow(clippy::too_many_arguments)]
pub fn notice_banner(
    notice: Option<&NoticeText>,
    state: Option<&str>,
    version: Option<&str>,
    preprint_posted_date: Option<&str>,
    license: Option<&str>,
    corresponding_author: Option<&str>,
    cite_as: Option<&str>,
    doi: Option<&str>,
) -> Markup {
    let Some(notice) = notice else {
        return html! {};
    };
    let text = match state {
        Some("draft") | Some("under-review") => {
            let working_paper = fill_template(
                &notice.working_paper_notice.template,
                &[
                    ("version", version.unwrap_or("")),
                    ("preprint_posted_date", preprint_posted_date.unwrap_or("")),
                    ("license", license.unwrap_or("")),
                    ("corresponding_author", corresponding_author.unwrap_or("")),
                    ("cite_as", cite_as.unwrap_or("")),
                ],
            );
            let fls = notice
                .forward_looking_statements
                .template
                .trim()
                .to_string();
            format!("{working_paper}\n\n{fls}")
        }
        Some("published") => fill_template(
            &notice.citation_banner.template,
            &[
                ("cite_as", cite_as.unwrap_or("")),
                ("doi", doi.unwrap_or("")),
            ],
        ),
        _ => return html! {},
    };
    html! {
        aside."k-notice-banner" role="note" {
            p { (text) }
        }
    }
}

/// `/research/{slug}` landing page (SPEC §0 render model): masthead +
/// abstract + a link to the full-text rendition — **not** the full body
/// (that's `research_fulltext`). `notice` is the caller's already-rendered
/// `notice_banner()` output (renders empty when there's nothing to disclose).
pub fn research_landing(
    title: &str,
    authors: &[Author],
    abstract_html: &str,
    slug: &str,
    cite_as: Option<&str>,
    notice: Markup,
) -> Markup {
    html! {
        article."k-research k-research--landing" {
            (masthead(title, authors))
            (notice)
            @if !abstract_html.is_empty() {
                section."k-research__abstract" {
                    h2 { "Abstract" }
                    div."k-prose" { (PreEscaped(abstract_html)) }
                }
            }
            p."k-research__fulltext-link" {
                a."k-button" href={ "/research/" (slug) "/full" } { "Read the full text \u{2192}" }
            }
            (print_citation_stamp(slug, cite_as))
        }
    }
}

/// Print-only brand mark at the very top of the page — same `display:none`-
/// then-`@media print`-override mechanism as `.k-print-citation` below.
/// Added 2026-07-15 after comparing our print render directly against a real
/// Wikipedia print render: Wikipedia keeps a minimal top-of-page brand
/// element (small wordmark + one-line tagline, then a rule) even though it
/// hides all interactive header/search/nav chrome — a printed page identifies
/// its source at both the top and the bottom, not only via the closing
/// citation line. `site_description` is the same canonical description
/// `footer()`'s brand block uses (site-footer recipe, `pointsav-design-
/// system`: the tagline must reuse the site's own description, never a
/// second hand-authored copy) — falls back to `Tenant::tagline()` only when
/// no description exists (e.g. `index.md` has no `short_description`).
fn print_brand_mark(tenant: Tenant, site_description: Option<&str>) -> Markup {
    html! {
        div."k-print-brand" {
            p."k-print-brand__name" { (tenant.home_label()) }
            p."k-print-brand__tagline" { (site_description.unwrap_or_else(|| tenant.tagline())) }
        }
    }
}

/// Print-only citation stamp for JOURNAL pages — same purpose and CSS
/// mechanism as `article()`'s own (Phase 9 print mode, `.k-print-citation`
/// is `display:none` on screen, shown only in `@media print`). Prefers the
/// author-specified `cite_as` frontmatter string (SPEC §2 table) when
/// present; falls back to the landing page's own URL. The landing page,
/// not `/full`, is the citable record per academic-page convention (a DOI
/// landing page cites the abstract page, not the full-text rendition).
fn print_citation_stamp(slug: &str, cite_as: Option<&str>) -> Markup {
    html! {
        p."k-print-citation" {
            "Cite this record: "
            @if let Some(c) = cite_as.filter(|s| !s.is_empty()) {
                (c)
            } @else {
                "/research/" (slug)
            }
            "."
        }
    }
}

/// `/research/{slug}/full` — the full-text rendition (SPEC §0): the ~22-
/// section body plus the generated References section (already appended to
/// `body_html` by `content::render_journal_doc`), reachable in one click
/// from the landing page. `geospatial` (`Frontmatter::is_geospatial`, SPEC
/// §10.1) scopes the `.full-bleed`/`.wide` figure-width CSS classes (SPEC
/// §10.2) — those classes are usable today via hand-authored raw HTML
/// `<figure>` blocks (comrak's unsafe rendering already passes them
/// through); the Markdown attribute shorthand `{#fig-id .full-bleed}` SPEC
/// §10.2 also describes is deferred (no comrak attribute-extension exists to
/// build on — confirmed against comrak 0.52's `Extension` options — so it
/// needs a hand-rolled parser, not yet justified with zero real geospatial
/// papers locally to validate one against).
pub fn research_fulltext(
    title: &str,
    authors: &[Author],
    body_html: &str,
    geospatial: bool,
    slug: &str,
    cite_as: Option<&str>,
    notice: Markup,
) -> Markup {
    html! {
        article."k-research k-research--fulltext"."k-research--geospatial"[geospatial] {
            (masthead(title, authors))
            (notice)
            div."k-prose" { (PreEscaped(body_html)) }
            (print_citation_stamp(slug, cite_as))
        }
    }
}

/// Shift every `<h1...>`/`</h1>` in comrak-rendered HTML down to `<h2>` — used
/// for embedded content (the Important Information band) that must never
/// introduce a second `<h1>` alongside the page's own article title. Comrak's
/// output is consistent enough (`<h1>` or `<h1 id="...">`, always
/// self-closed with a plain `</h1>`) that a literal substring replace is
/// safe here without a full HTML parser.
fn demote_heading(html: &str) -> String {
    html.replace("<h1>", "<h2>")
        .replace("<h1 ", "<h2 ")
        .replace("</h1>", "</h2>")
}

/// The "Important Information" band above the footer (native `<details>`, no JS).
/// Content is the counsel-owned `important-information.md` when present, else a
/// safe tenant default; forced open in print so the record copy carries it.
///
/// `important_info` is rendered HTML from the content repo's own Markdown —
/// its authored heading (if any) renders as `<h1>` by default, which used to
/// collide with the article `<h1>` on every single page of all three sites
/// (the single most-reported finding in a 2026-07 audit, 15 hits across all
/// dimensions). `demote_heading` fixes that structurally, in the renderer,
/// rather than requiring every content file to avoid a leading `# heading`.
fn compliance_band(tenant: Tenant, important_info: Option<&str>) -> Markup {
    html! {
        section."k-compliance" aria-label="Important information" {
            details."k-compliance__details" {
                summary."k-compliance__summary" { "Important Information" }
                div."k-compliance__body k-prose" {
                    @if let Some(html) = important_info {
                        (PreEscaped(demote_heading(html)))
                    } @else {
                        p {
                            "This site presents records maintained by " (tenant.issuer())
                            ". The information is provided for general information only and does not "
                            "constitute an offer to sell, a solicitation of an offer to buy, or "
                            "investment, legal, tax, or accounting advice. Statements regarding "
                            "planned, intended, or targeted future activities are forward-looking and "
                            "subject to change without notice; they are not undertaken to be updated "
                            "except as required by law."
                        }
                    }
                    p."k-compliance__more" {
                        a href="/wiki/disclaimers" { "Read the full disclaimer \u{2192}" }
                    }
                }
            }
        }
    }
}

/// The full document as one balanced tree. `cats` drives the sidebar nav;
/// `disclaimer` is the Important Information band content (None → tenant default);
/// `legal` supplies the footer's copyright/trademark text.
#[allow(clippy::too_many_arguments)]
pub fn page(
    tenant: Tenant,
    lang: &str,
    head: Markup,
    body: Markup,
    cats: &[(String, String, String)],
    toc: &[Heading],
    query: &str,
    disclaimer: Option<&str>,
    legal: &LegalTokens,
    site_description: Option<&str>,
    article_count: usize,
    current_category: Option<&str>,
    siblings: &[(String, String)],
) -> Markup {
    html! {
        (DOCTYPE)
        html lang=(lang) data-instance=(tenant.instance_str()) {
            head { (head) }
            body {
                a."k-skip-link" href="#k-main" { "Skip to content" }
                (mobile_nav(tenant, query, cats))
                div."k-page" {
                    (print_brand_mark(tenant, site_description))
                    (utility_bar(tenant))
                    (header(tenant, lang, query))
                    div."k-shell" {
                        (sidebar(cats, toc, current_category, siblings))
                        main."k-page__body" #"k-main" tabindex="-1" { (body) }
                    }
                    (compliance_band(tenant, disclaimer))
                    (footer(tenant, legal, site_description, article_count))
                }
                script src="/static/app.js" defer {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breadcrumb_renders_trail_and_current_as_plain_text() {
        let trail = vec![
            ("/".to_string(), "PointSav Documentation".to_string()),
            (
                "/category/architecture".to_string(),
                "Architecture".to_string(),
            ),
        ];
        let html = breadcrumb(&trail, "Zero-container inference").into_string();
        assert!(html.contains(r#"href="/""#));
        assert!(html.contains(r#"href="/category/architecture""#));
        assert!(html.contains("Zero-container inference"));
        // The current page is aria-current="page" and NOT a link.
        assert!(html.contains(r#"aria-current="page""#));
        let current_item_start = html.find(r#"aria-current="page""#).unwrap();
        assert!(!html[current_item_start..].contains("<a "));
    }

    #[test]
    fn hreflang_links_emits_both_directions() {
        let html = hreflang_links(
            ("en", "https://documentation.pointsav.com/wiki/foo"),
            ("es", "https://documentation.pointsav.com/es/wiki/foo"),
        )
        .into_string();
        assert!(html.contains(r#"hreflang="en""#));
        assert!(html.contains(r#"hreflang="es""#));
        assert!(html.contains("https://documentation.pointsav.com/wiki/foo"));
        assert!(html.contains("https://documentation.pointsav.com/es/wiki/foo"));
    }

    #[test]
    fn breadcrumb_jsonld_emits_positioned_list() {
        let items = vec![
            (
                "https://documentation.pointsav.com/".to_string(),
                "PointSav Documentation".to_string(),
            ),
            (
                "https://documentation.pointsav.com/category/architecture".to_string(),
                "Architecture".to_string(),
            ),
            (
                "https://documentation.pointsav.com/wiki/foo".to_string(),
                "Foo".to_string(),
            ),
        ];
        let html = breadcrumb_jsonld(&items).into_string();
        let json_start = html.find('{').unwrap();
        let json_end = html.rfind('}').unwrap() + 1;
        let parsed: serde_json::Value =
            serde_json::from_str(&html[json_start..json_end]).expect("valid JSON-LD");
        assert_eq!(parsed["@type"], "BreadcrumbList");
        let list = parsed["itemListElement"].as_array().unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0]["position"], 1);
        assert_eq!(list[2]["position"], 3);
        assert_eq!(list[2]["name"], "Foo");
        // The last entry (current page) must still carry `item` — a real
        // finding the schema requires it even for non-linked breadcrumb steps.
        assert_eq!(
            list[2]["item"],
            "https://documentation.pointsav.com/wiki/foo"
        );
    }

    #[test]
    fn article_jsonld_uses_organization_id_not_inline_name() {
        let html = article_jsonld(
            Tenant::Documentation,
            "Zero-container inference",
            "How the runtime avoids containers.",
            "https://documentation.pointsav.com/wiki/zero-container-inference",
            Some("2026-06-01"),
        )
        .into_string();
        let json_start = html.find('{').unwrap();
        let json_end = html.rfind('}').unwrap() + 1;
        let parsed: serde_json::Value =
            serde_json::from_str(&html[json_start..json_end]).expect("valid JSON-LD");
        assert_eq!(parsed["@type"], "TechArticle");
        assert_eq!(parsed["headline"], "Zero-container inference");
        assert_eq!(
            parsed["author"]["@id"],
            "https://pointsav.com/#organization"
        );
        assert!(
            parsed["author"].get("name").is_none(),
            "author must be an @id reference, not an inline Organization"
        );
        assert_eq!(parsed["dateModified"], "2026-06-01");
    }

    #[test]
    fn jsonld_neutralizes_script_breakout() {
        let evil = article_jsonld(
            Tenant::Corporate,
            "</script><script>alert(1)</script>",
            "",
            "https://corporate.woodfinegroup.com/wiki/x",
            None,
        )
        .into_string();
        assert!(
            !evil.contains("</script><script>alert"),
            "raw script-breakout sequence must not survive into the HTML: {evil}"
        );
    }

    fn test_author(name: &str, affiliation: &str, email: &str) -> Author {
        Author {
            name: Some(name.to_string()),
            affiliation: Some(affiliation.to_string()),
            email: Some(email.to_string()),
            orcid: None,
            credit_roles: vec![],
        }
    }

    #[test]
    fn masthead_renders_title_as_h1_and_lists_authors() {
        let authors = vec![
            test_author("J. Woodfine", "PointSav Digital Systems", "j@example.com"),
            test_author("P. Woodfine", "Woodfine Management Corp", "p@example.com"),
        ];
        let html = masthead("Capability Geometry", &authors).into_string();
        assert!(html.contains("<h1"));
        assert!(html.contains("Capability Geometry"));
        assert!(html.contains("J. Woodfine"));
        assert!(html.contains("P. Woodfine"));
        assert!(html.contains("PointSav Digital Systems"));
    }

    #[test]
    fn masthead_renders_correspondence_for_every_author_with_an_email() {
        let authors = vec![
            test_author("A. One", "Org", "a@example.com"),
            test_author("B. Two", "Org", "b@example.com"),
        ];
        let html = masthead("Title", &authors).into_string();
        assert!(html.contains(r#"href="mailto:a@example.com""#));
        assert!(html.contains(r#"href="mailto:b@example.com""#));
    }

    #[test]
    fn masthead_with_no_authors_still_renders_the_title() {
        let html = masthead("Solo Title", &[]).into_string();
        assert!(html.contains("Solo Title"));
        assert!(!html.contains("k-masthead__authors"));
    }

    fn test_notice_text() -> NoticeText {
        serde_yaml::from_str(
            r#"
working_paper_notice:
  template: >
    This is a working paper (v{version}), posted {preprint_posted_date}, under
    {license}. Correspondence: {corresponding_author}. Cite as: {cite_as}.
forward_looking_statements:
  template: >
    Static FLS advisory text.
citation_banner:
  template: >
    Published version: {cite_as} DOI: {doi}
superseded_notice:
  template: >
    Superseded notice text.
"#,
        )
        .unwrap()
    }

    #[test]
    fn notice_banner_renders_nothing_without_a_data_source() {
        // Must not fabricate disclosure text locally when the file is absent/malformed.
        let html = notice_banner(
            None,
            Some("draft"),
            Some("0.4.0"),
            Some("2026-07-02"),
            Some("CC BY 4.0"),
            Some("a@example.com"),
            Some("Woodfine (2026)"),
            None,
        )
        .into_string();
        assert_eq!(html, "");
    }

    #[test]
    fn notice_banner_renders_nothing_with_no_state() {
        let notice = test_notice_text();
        let html =
            notice_banner(Some(&notice), None, None, None, None, None, None, None).into_string();
        assert_eq!(html, "");
    }

    #[test]
    fn notice_banner_renders_working_paper_notice_for_draft() {
        let notice = test_notice_text();
        let html = notice_banner(
            Some(&notice),
            Some("draft"),
            Some("0.4.0"),
            Some("2026-07-02"),
            Some("CC BY 4.0"),
            Some("a@example.com"),
            Some("Woodfine (2026)"),
            None,
        )
        .into_string();
        assert!(html.contains("k-notice-banner"));
        assert!(html.contains("v0.4.0"));
        assert!(html.contains("2026-07-02"));
        assert!(html.contains("CC BY 4.0"));
        assert!(html.contains("a@example.com"));
        assert!(html.contains("Woodfine (2026)"));
        assert!(html.contains("Static FLS advisory text"));
    }

    #[test]
    fn notice_banner_renders_citation_banner_for_published() {
        let notice = test_notice_text();
        let html = notice_banner(
            Some(&notice),
            Some("published"),
            None,
            None,
            None,
            None,
            Some("Woodfine (2026)"),
            Some("10.1/example"),
        )
        .into_string();
        assert!(html.contains("Woodfine (2026)"));
        assert!(html.contains("10.1/example"));
        assert!(!html.contains("working paper"));
    }

    #[test]
    fn research_fulltext_carries_geospatial_class_only_when_requested() {
        let with_class =
            research_fulltext("T", &[], "<p>body</p>", true, "slug", None, html! {}).into_string();
        assert!(with_class.contains("k-research--geospatial"));
        let without_class =
            research_fulltext("T", &[], "<p>body</p>", false, "slug", None, html! {}).into_string();
        assert!(!without_class.contains("k-research--geospatial"));
    }

    #[test]
    fn print_citation_stamp_prefers_cite_as_over_url_fallback() {
        let with_cite_as = research_fulltext(
            "T",
            &[],
            "<p>b</p>",
            false,
            "my-slug",
            Some("Woodfine (2026)"),
            html! {},
        )
        .into_string();
        assert!(with_cite_as.contains("Cite this record: Woodfine (2026)."));
        let without_cite_as =
            research_fulltext("T", &[], "<p>b</p>", false, "my-slug", None, html! {}).into_string();
        assert!(without_cite_as.contains("Cite this record: /research/my-slug."));
    }

    #[test]
    fn print_brand_mark_falls_back_to_tenant_tagline_when_no_site_description() {
        let html = print_brand_mark(Tenant::Documentation, None).into_string();
        assert!(html.contains("PointSav Documentation"));
        assert!(html.contains("Technical records for the PointSav platform."));
        assert!(html.contains("k-print-brand"));
    }

    #[test]
    fn print_brand_mark_prefers_site_description_over_tenant_tagline() {
        let html =
            print_brand_mark(Tenant::Documentation, Some("A record repository.")).into_string();
        assert!(html.contains("A record repository."));
        assert!(!html.contains("Technical records for the PointSav platform."));
    }

    #[test]
    fn demote_heading_shifts_h1_to_h2() {
        assert_eq!(
            demote_heading("<h1>Important Information</h1>"),
            "<h2>Important Information</h2>"
        );
        assert_eq!(
            demote_heading(r#"<h1 id="important-information">Text</h1><p>Body</p>"#),
            r#"<h2 id="important-information">Text</h2><p>Body</p>"#
        );
    }

    #[test]
    fn demote_heading_leaves_other_levels_alone() {
        let html = "<h1>A</h1><h2>B</h2><h3>C</h3>";
        assert_eq!(demote_heading(html), "<h2>A</h2><h2>B</h2><h3>C</h3>");
    }
}
