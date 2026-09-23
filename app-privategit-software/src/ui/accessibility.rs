// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Accessibility page for software.pointsav.com — a self-contained page, not
//! a link out to the wiki's or marketing sites' own pages (same operator
//! instruction as `disclaimer.rs`).
//!
//! Targets WCAG 2.1 AA (the standard already stated for a sibling PointSav
//! product in this corpus, kept consistent rather than invented fresh). Does
//! not claim a completed compliance certification — no formal audit re-score
//! is on record for this crate specifically, only the original 2026-06-24
//! audit finding (74 axe-violation nodes, full mobile layout breakdown) that
//! this crate's Sovereign Editorial rebuild addressed.

use crate::ui::Lang;
use maud::{html, Markup};

/// The full self-contained accessibility page (`GET /page/accessibility`,
/// `GET /es/page/accessibility`).
pub fn accessibility_markup(lang: Lang) -> Markup {
    match lang {
        Lang::En => accessibility_markup_en(),
        Lang::Es => accessibility_markup_es(),
    }
}

fn accessibility_markup_en() -> Markup {
    html! {
        div."sw-legal" {
            h1 { "Accessibility" }
            p."sw-legal__lede" {
                strong { "PointSav Digital Systems" } " — a trade name of Woodfine Capital "
                "Projects Inc."
            }

            h2 { "1. Standard targeted" }
            p {
                "We target WCAG 2.1 Level AA, the same standard we use across "
                "PointSav's other properties."
            }

            h2 { "2. What has been addressed" }
            p {
                "A 2026-06-24 audit of this site found a complete mobile layout "
                "breakdown and a high density of automated accessibility-check findings. "
                "As part of the 2026-07 storefront rebuild, we addressed contrast tokens, "
                "responsive layout, and semantic landmarks across the site's chrome "
                "and content pages. That's a factual description of work completed, not "
                "a claim of full WCAG 2.1 AA conformance — we haven't recorded a formal "
                "re-audit score for this rebuild yet."
            }

            h2 { "3. Known gaps and how to report one" }
            p {
                "If you encounter a barrier using this site, we want to know about it. "
                "Reports are reviewed and addressed on an ongoing basis; we do not claim "
                "the site is free of accessibility issues."
            }

            h2 { "4. Contact for accessibility issues" }
            p {
                "Report accessibility issues to " a href="mailto:open.source@pointsav.com" { "open.source@pointsav.com" } "."
            }
        }
    }
}

fn accessibility_markup_es() -> Markup {
    html! {
        div."sw-legal" {
            h1 { "Accesibilidad" }
            p."sw-legal__lede" {
                strong { "PointSav Digital Systems" } " \u{2014} nombre comercial de Woodfine "
                "Capital Projects Inc."
            }

            h2 { "1. Est\u{e1}ndar objetivo" }
            p {
                "Nuestro objetivo es el nivel WCAG 2.1 AA, el mismo est\u{e1}ndar "
                "que usamos en las dem\u{e1}s plataformas de PointSav."
            }

            h2 { "2. Qu\u{e9} se ha abordado" }
            p {
                "Una auditor\u{ed}a del 2026-06-24 de este sitio encontr\u{f3} una falla "
                "completa del dise\u{f1}o m\u{f3}vil y una alta densidad de hallazgos "
                "autom\u{e1}ticos de accesibilidad. Como parte de la reconstrucci\u{f3}n de la "
                "tienda de 2026-07, abordamos los tokens de contraste, el dise\u{f1}o "
                "responsivo y los puntos de referencia sem\u{e1}nticos en todo el chrome y "
                "las p\u{e1}ginas de contenido del sitio. Esa es una descripci\u{f3}n factual "
                "del trabajo realizado, no una afirmaci\u{f3}n de conformidad total con "
                "WCAG 2.1 AA \u{2014} todav\u{ed}a no hemos registrado una puntuaci\u{f3}n de "
                "reauditor\u{ed}a formal para esta reconstrucci\u{f3}n."
            }

            h2 { "3. Brechas conocidas y c\u{f3}mo reportarlas" }
            p {
                "Si encuentra una barrera al usar este sitio, queremos saberlo. Los "
                "reportes se revisan y abordan de forma continua; no afirmamos que el "
                "sitio est\u{e9} libre de problemas de accesibilidad."
            }

            h2 { "4. Contacto para problemas de accesibilidad" }
            p {
                "Reporte problemas de accesibilidad a "
                a href="mailto:open.source@pointsav.com" { "open.source@pointsav.com" } "."
            }
        }
    }
}
