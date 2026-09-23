// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Pricing page — `GET /pricing`.
//!
//! Phase 4: catalog-driven (like `software_page`), not a static file — the
//! product counts per tier are always live, so this page cannot drift from
//! `products.yaml` the way the old `static/licensing.html` did (see that file's
//! Phase 4 rewrite for the fictional content it used to carry). Leads with the
//! buy-to-own framing established in the BRIEF's Positioning Pivot; carries the
//! BC tax line and AGPL §13 source link required by `BRIEF-software-distribution-
//! substrate.md` and previously missing anywhere in this crate.
//!
//! **Rebuilt 2026-07-07** to match `BRIEF-software-licensing-structure.md`'s
//! tier model (`Proprietary`/`Agpl`/`Apache` are rendered here; the `Fsl`
//! variant still exists on `LicenseTier` in `main.rs` but is no longer given a
//! card on this page — see that file's doc comment) and, separately, to stop
//! displaying a per-tier dollar figure. The prior version showed "$1.00 USDC" / "$19.00 USDC" as each tier's
//! "canonical future price" — that BRIEF's §4 explicitly states **no non-zero
//! future price has been ratified for any tier**: "Actual, non-zero future
//! pricing for any product — this BRIEF only ratifies $0-for-now; real pricing
//! is a separate, later decision per product." Those two numbers were this
//! crate's own earlier invention, not a ratified fact, and showing them as tier
//! pricing on a real storefront page was a real accuracy problem independent of
//! the naming mismatch — removed rather than relabeled.

use crate::ui::Lang;
use crate::{Catalog, LicenseTier};
use maud::{html, Markup, PreEscaped};

/// Tier descriptions are the one piece of chrome copy that's genuinely legal/
/// licensing-adjacent — translated here (not deferred to the
/// factory-release-engineering pipeline) because they describe THIS crate's own
/// `LicenseTier` variants, not the shared Woodfine disclaimer/trademark
/// text that pipeline is producing a canonical Spanish source for. (`Fsl` still
/// exists as a variant but is not rendered as a card on this page — three tiers
/// are shown.)
fn tier_card(tier: LicenseTier, catalog: &Catalog, lang: Lang) -> Markup {
    let count = catalog
        .installers
        .iter()
        .filter(|i| i.license_tier == tier)
        .count();
    let desc = match (tier, lang) {
        (LicenseTier::Proprietary, Lang::En) => {
            "No source grant. PointSav's stated commercial moat — solo use is \
             free during BETA; aggregation or resale requires a separate \
             commercial agreement, permanently."
        }
        (LicenseTier::Proprietary, Lang::Es) => {
            "Sin concesi\u{f3}n de c\u{f3}digo fuente. El resguardo comercial declarado de \
             PointSav: el uso individual es gratuito durante la fase BETA; la agregaci\u{f3}n o \
             reventa requiere un acuerdo comercial independiente, de forma permanente."
        }
        (LicenseTier::Fsl, Lang::En) => {
            "Source-readable now. A two-year non-compete restriction, then \
             automatic conversion to Apache-2.0 for that release."
        }
        (LicenseTier::Fsl, Lang::Es) => {
            "C\u{f3}digo fuente legible desde ahora. Una restricci\u{f3}n de no competencia de \
             dos a\u{f1}os, tras la cual se convierte autom\u{e1}ticamente a Apache-2.0 para esa \
             versi\u{f3}n."
        }
        (LicenseTier::Agpl, Lang::En) => {
            "AGPL-3.0-or-later. Source is freely available under copyleft terms; \
             a purchase buys the official signed binary and commercial \
             redistribution rights, not access to the code itself."
        }
        (LicenseTier::Agpl, Lang::Es) => {
            "AGPL-3.0-or-later. El c\u{f3}digo fuente est\u{e1} disponible libremente bajo \
             t\u{e9}rminos copyleft; una compra adquiere el binario oficial firmado y \
             derechos de redistribuci\u{f3}n comercial, no el acceso al c\u{f3}digo en s\u{ed}."
        }
        (LicenseTier::Apache, Lang::En) => {
            "A genuine, unconditional Apache-2.0 grant — free permanently, not \
             free for now. No purchase required: claiming a product issues a \
             free signed key for the download, and the source itself needs no \
             key at all."
        }
        (LicenseTier::Apache, Lang::Es) => {
            "Una concesi\u{f3}n Apache-2.0 aut\u{e9}ntica e incondicional \u{2014} gratuita de \
             forma permanente, no gratuita solo por ahora. No se requiere compra: reclamar \
             un producto emite una clave firmada gratuita para la descarga, y el propio \
             c\u{f3}digo fuente no necesita ninguna clave."
        }
    };
    let product_word = match (lang, count) {
        (Lang::En, 1) => "product".to_string(),
        (Lang::En, _) => "products".to_string(),
        (Lang::Es, 1) => "producto".to_string(),
        (Lang::Es, _) => "productos".to_string(),
    };
    html! {
        article."sw-pr-tier" {
            h2."sw-pr-tier__name" { (tier.label()) }
            p."sw-pr-tier__count" { (count) " " (product_word) }
            p."sw-pr-tier__desc" { (desc) }
        }
    }
}

pub fn pricing_markup(catalog: &Catalog, lang: Lang) -> Markup {
    match lang {
        Lang::En => pricing_markup_en(catalog),
        Lang::Es => pricing_markup_es(catalog),
    }
}

fn pricing_markup_en(catalog: &Catalog) -> Markup {
    html! {
        (pricing_style())
        div."sw-pr-wrap" {
            h1."sw-pr-title" { "Pricing" }
            p."sw-pr-lede" {
                "Buy it once. Run it anywhere. Own it forever. No subscription, no cloud \
                 dependency, no kill switch."
            }
            div."sw-pr-trust" {
                span { "Air-gap capable" }
                span aria-hidden="true" { "\u{b7}" }
                span { "No telemetry" }
                span aria-hidden="true" { "\u{b7}" }
                span { "Runs offline" }
                span aria-hidden="true" { "\u{b7}" }
                span { "Your keys, your license" }
            }
            div."sw-pr-tiers" {
                (tier_card(LicenseTier::Proprietary, catalog, Lang::En))
                (tier_card(LicenseTier::Agpl, catalog, Lang::En))
                (tier_card(LicenseTier::Apache, catalog, Lang::En))
            }
            p."sw-pr-beta-note" {
                "Every product is currently free during BETA. Apache-2.0 products are free \
                 permanently, by license; pricing for AGPL and Proprietary tiers has not been \
                 set yet and may apply once BETA ends. See each product's own listing on the "
                a href="/software" { "Products" }
                " page for its current, active price."
            }
            p."sw-pr-tax" {
                "No tax collected — PointSav Digital Systems operates below the GST \
                 small-supplier threshold."
            }
            p."sw-pr-agpl" {
                "Source: "
                a href="https://github.com/pointsav/pointsav-monorepo" {
                    "github.com/pointsav/pointsav-monorepo"
                }
            }
        }
    }
}

fn pricing_markup_es(catalog: &Catalog) -> Markup {
    html! {
        (pricing_style())
        div."sw-pr-wrap" {
            h1."sw-pr-title" { "Precios" }
            p."sw-pr-lede" {
                "Cómprelo una vez. Ejecútelo donde quiera. Sea suyo para siempre. Sin \
                 suscripción, sin dependencia de la nube, sin interruptor de apagado remoto."
            }
            div."sw-pr-trust" {
                span { "Compatible con air-gap" }
                span aria-hidden="true" { "\u{b7}" }
                span { "Sin telemetr\u{ed}a" }
                span aria-hidden="true" { "\u{b7}" }
                span { "Funciona sin conexi\u{f3}n" }
                span aria-hidden="true" { "\u{b7}" }
                span { "Sus claves, su licencia" }
            }
            div."sw-pr-tiers" {
                (tier_card(LicenseTier::Proprietary, catalog, Lang::Es))
                (tier_card(LicenseTier::Agpl, catalog, Lang::Es))
                (tier_card(LicenseTier::Apache, catalog, Lang::Es))
            }
            p."sw-pr-beta-note" {
                "Todos los productos son actualmente gratuitos durante la fase BETA. Los \
                 productos Apache-2.0 son gratuitos de forma permanente, por licencia; el \
                 precio futuro de los niveles AGPL y Proprietary a\u{fa}n no se ha fijado y \
                 podr\u{ed}a aplicarse una vez finalice la fase BETA. Consulte la ficha de cada \
                 producto en la p\u{e1}gina de "
                a href="/es/software" { "Productos" }
                " para conocer su precio actual."
            }
            p."sw-pr-tax" {
                "No se cobran impuestos: PointSav Digital Systems opera por debajo del \
                 umbral de peque\u{f1}o proveedor del GST."
            }
            p."sw-pr-agpl" {
                "C\u{f3}digo fuente: "
                a href="https://github.com/pointsav/pointsav-monorepo" {
                    "github.com/pointsav/pointsav-monorepo"
                }
            }
        }
    }
}

fn pricing_style() -> Markup {
    let css = r#".sw-pr-wrap{max-width:840px;margin:0 auto;padding:40px 24px 64px;box-sizing:border-box;}
.sw-pr-title{margin:0 0 8px;font-family:Georgia,"Times New Roman",serif;font-size:30px;color:#111827;}
.sw-pr-lede{margin:0 0 16px;font-size:16px;line-height:1.5;color:#344054;max-width:56ch;}
.sw-pr-trust{margin:0 0 32px;font-size:12px;letter-spacing:.04em;color:#667085;display:flex;gap:10px;flex-wrap:wrap;}
.sw-pr-tiers{display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:20px;margin:0 0 32px;}
.sw-pr-tier{border:1px solid #e4e7ec;border-radius:10px;padding:24px;background:#fff;box-shadow:0 1px 2px rgba(16,24,40,.04);}
.sw-pr-tier__name{font-family:Georgia,"Times New Roman",serif;font-size:20px;margin:0 0 8px;color:#111827;}
.sw-pr-tier__count{font-size:12px;color:#234ed8;font-weight:600;margin:0 0 12px;}
.sw-pr-tier__desc{font-size:13.5px;line-height:1.55;color:#475467;margin:0;}
.sw-pr-beta-note{font-size:13px;line-height:1.55;color:#475467;margin:0 0 16px;padding:14px;border:1px dashed #d0d5dd;border-radius:8px;background:#fcfcfd;}
.sw-pr-tax{font-size:12.5px;color:#667085;margin:0 0 8px;}
.sw-pr-agpl{font-size:12.5px;color:#667085;margin:0;}"#;
    html! { style { (PreEscaped(css)) } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Installer;

    fn fixture() -> Catalog {
        Catalog {
            installers: vec![
                Installer {
                    id: "os-orchestration".into(),
                    name: "Orchestration OS".into(),
                    description: "d".into(),
                    edition: "1.0".into(),
                    platform: "linux".into(),
                    size_mb: 1,
                    path: "os-orchestration/1.0".into(),
                    license_tier: LicenseTier::Proprietary,
                    price_usdc: 0,
                    fsl_conversion_date: None,
                    guide_url: None,
                    artifact: crate::ArtifactKind::CliBinary,
                    platform_slug: None,
                    boot_notes: None,
                    tier: crate::ArchTier::Platform,
                    facts: vec![],
                    bundled_registry_products: vec![],
                    family: crate::ProductFamily::Independent,
                    becomes: None,
                },
                Installer {
                    id: "os-console".into(),
                    name: "Console".into(),
                    description: "d".into(),
                    edition: "1.0".into(),
                    platform: "linux".into(),
                    size_mb: 1,
                    path: "os-console/1.0".into(),
                    license_tier: LicenseTier::Agpl,
                    price_usdc: 0,
                    fsl_conversion_date: None,
                    guide_url: None,
                    artifact: crate::ArtifactKind::CliBinary,
                    platform_slug: None,
                    boot_notes: None,
                    tier: crate::ArchTier::Platform,
                    facts: vec![],
                    bundled_registry_products: vec![],
                    family: crate::ProductFamily::Independent,
                    becomes: None,
                },
                Installer {
                    id: "os-privategit".into(),
                    name: "PrivateGit".into(),
                    description: "d".into(),
                    edition: "1.0".into(),
                    platform: "linux".into(),
                    size_mb: 1,
                    path: "os-privategit/1.0".into(),
                    license_tier: LicenseTier::Fsl,
                    price_usdc: 0,
                    fsl_conversion_date: None,
                    guide_url: None,
                    artifact: crate::ArtifactKind::CliBinary,
                    platform_slug: None,
                    boot_notes: None,
                    tier: crate::ArchTier::Platform,
                    facts: vec![],
                    bundled_registry_products: vec![],
                    family: crate::ProductFamily::Independent,
                    becomes: None,
                },
                Installer {
                    id: "os-mediakit".into(),
                    name: "MediaKit".into(),
                    description: "d".into(),
                    edition: "1.0".into(),
                    platform: "linux".into(),
                    size_mb: 1,
                    path: "os-mediakit/1.0".into(),
                    license_tier: LicenseTier::Fsl,
                    price_usdc: 0,
                    fsl_conversion_date: None,
                    guide_url: None,
                    artifact: crate::ArtifactKind::CliBinary,
                    platform_slug: None,
                    boot_notes: None,
                    tier: crate::ArchTier::Platform,
                    facts: vec![],
                    bundled_registry_products: vec![],
                    family: crate::ProductFamily::Independent,
                    becomes: None,
                },
                Installer {
                    id: "tool-wallet".into(),
                    name: "PointSav Wallet".into(),
                    description: "d".into(),
                    edition: "1.0".into(),
                    platform: "linux".into(),
                    size_mb: 1,
                    path: "tool-wallet/1.0".into(),
                    license_tier: LicenseTier::Apache,
                    price_usdc: 0,
                    fsl_conversion_date: None,
                    guide_url: None,
                    artifact: crate::ArtifactKind::CliBinary,
                    platform_slug: None,
                    boot_notes: None,
                    tier: crate::ArchTier::Platform,
                    facts: vec![],
                    bundled_registry_products: vec![],
                    family: crate::ProductFamily::Independent,
                    becomes: None,
                },
            ],
        }
    }

    #[test]
    fn pricing_page_shows_the_three_rendered_tiers_with_live_counts() {
        let html = pricing_markup(&fixture(), Lang::En).into_string();
        assert!(html.contains("Proprietary"));
        assert!(html.contains("1 product<"));
        assert!(
            !html.contains("FSL-1.1-ALv2"),
            "FSL is not rendered as a card on the pricing page"
        );
        assert!(html.contains("AGPL-3.0-or-later"));
        assert!(html.contains("Apache-2.0"));
        assert!(
            !html.contains("sw-pr-tier__amt"),
            "must not display a fabricated per-tier dollar figure — \
             BRIEF-software-licensing-structure.md §4 ratifies no future price"
        );
    }

    #[test]
    fn pricing_page_carries_beta_tax_and_agpl_notes() {
        let html = pricing_markup(&fixture(), Lang::En).into_string();
        assert!(html.contains("currently free during BETA"));
        assert!(
            html.contains("Apache-2.0 products are free permanently"),
            "beta note must not contradict Apache-2.0's permanent $0 grant"
        );
        assert!(html.contains("No tax collected"));
        assert!(html.contains("github.com/pointsav/pointsav-monorepo"));
    }

    #[test]
    fn spanish_pricing_page_translates_title_and_tiers_without_fabricating_a_price() {
        let html = pricing_markup(&fixture(), Lang::Es).into_string();
        assert!(html.contains("<h1 class=\"sw-pr-title\">Precios</h1>"));
        assert!(html.contains("Proprietary")); // tier identifiers stay untranslated (legal labels)
        assert!(html.contains("1 producto"));
        assert!(
            !html.contains("FSL-1.1-ALv2"),
            "FSL is not rendered as a card on the ES pricing page either"
        );
        assert!(
            html.contains("Los productos Apache-2.0 son gratuitos de forma permanente"),
            "ES beta note must not contradict Apache-2.0's permanent $0 grant"
        );
        assert!(html.contains("href=\"/es/software\""));
        assert!(
            !html.contains("sw-pr-tier__amt"),
            "ES page must not display a fabricated per-tier dollar figure either"
        );
    }
}
