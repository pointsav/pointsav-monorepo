// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Site language — MVL (minimum viable localization), operator-approved
//! 2026-07-12. Only `/software`, `/pricing`, and `/licensing` get a Spanish
//! variant (highest-traffic pages); `/page/contact`, `/page/disclaimer`,
//! `/page/privacy`, `/page/accessibility`, `/checkout/*`, and `/order/*` stay
//! English-only for now, reachable from an ES page but not yet translated.
//!
//! Locale is generic `es` — NOT `es-MX`/`es-ES`. This matches the convention
//! already live on home.pointsav.com/home.woodfinegroup.com (verified directly
//! against their served HTML: `hreflang="es"`, no regional subtag), which is
//! genuinely international/neutral Spanish, not a regional variant. The toggle
//! markup below (globe glyph + text label, reciprocal both directions) is the
//! same live pattern, renamed to this crate's `sw-` prefix convention.
//!
//! Legal/trademark copy (`surface::trademark_line`, `disclosure_body`) stays in
//! English on ES pages: a canonical Spanish translation of that shared legal
//! source text is already separately in flight in `factory-release-engineering`
//! (staged draft, not yet ratified) — this crate should consume that once it
//! lands rather than hand-translate its own copy that would need to be redone.

use maud::{html, Markup, PreEscaped};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    En,
    Es,
}

impl Lang {
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Es => "es",
        }
    }

    /// URL path prefix for this language's translated routes.
    pub fn prefix(self) -> &'static str {
        match self {
            Lang::En => "",
            Lang::Es => "/es",
        }
    }

    pub fn other(self) -> Lang {
        match self {
            Lang::En => Lang::Es,
            Lang::Es => Lang::En,
        }
    }

    /// Prefix an untranslated-page-relative path (e.g. `/software`) with this
    /// language's prefix. Only meaningful for the three translated pages.
    pub fn localize(self, unprefixed_path: &str) -> String {
        format!("{}{}", self.prefix(), unprefixed_path)
    }

    fn toggle_label(self) -> &'static str {
        match self.other() {
            Lang::En => "English",
            Lang::Es => "Español",
        }
    }

    pub fn nav_labels(self) -> NavLabels {
        match self {
            Lang::En => NavLabels {
                products: "Products",
                pricing: "Pricing",
                licensing: "Licensing",
                working_papers: "Working Papers",
                contact: "Contact Us",
                disclaimer: "Disclaimer",
                privacy: "Privacy",
                accessibility: "Accessibility",
                site_col: "Site",
                network_col: "Network",
                search_placeholder: "Search products\u{2026}",
                search_label: "Search products",
                menu_label: "Menu",
                documentation: "Documentation",
                design_system: "Design System",
                newsroom: "Newsroom",
                important_information: "Important information",
                persistent_disclaimer_lede: "Software licenses only \u{2014} not an offer of \
                    securities or investment. USDC payments on Polygon are irreversible. ",
                full_disclaimer_link: "Full disclaimer",
                all_rights_reserved: "All rights reserved.",
                powered_by: "Powered by",
            },
            Lang::Es => NavLabels {
                products: "Productos",
                pricing: "Precios",
                licensing: "Licencias",
                working_papers: "Documentos de Trabajo",
                contact: "Cont\u{e1}ctenos",
                disclaimer: "Aviso legal",
                privacy: "Privacidad",
                accessibility: "Accesibilidad",
                site_col: "Sitio",
                network_col: "Red",
                search_placeholder: "Buscar productos\u{2026}",
                search_label: "Buscar productos",
                menu_label: "Men\u{fa}",
                documentation: "Documentaci\u{f3}n",
                design_system: "Sistema de dise\u{f1}o",
                newsroom: "Sala de prensa",
                important_information: "Informaci\u{f3}n importante",
                persistent_disclaimer_lede: "Solo licencias de software \u{2014} no es una oferta \
                    de valores ni de inversi\u{f3}n. Los pagos en USDC sobre Polygon son \
                    irreversibles. ",
                full_disclaimer_link: "Aviso legal completo",
                all_rights_reserved: "Todos los derechos reservados.",
                powered_by: "Desarrollado con",
            },
        }
    }
}

/// Chrome nav-string bundle, kept as one struct so `masthead`/`footer` read one
/// field each rather than re-matching on `Lang` throughout `layout.rs`.
pub struct NavLabels {
    pub products: &'static str,
    pub pricing: &'static str,
    pub licensing: &'static str,
    pub working_papers: &'static str,
    pub contact: &'static str,
    pub disclaimer: &'static str,
    pub privacy: &'static str,
    pub accessibility: &'static str,
    pub site_col: &'static str,
    pub network_col: &'static str,
    pub search_placeholder: &'static str,
    pub search_label: &'static str,
    pub menu_label: &'static str,
    pub documentation: &'static str,
    pub design_system: &'static str,
    pub newsroom: &'static str,
    pub important_information: &'static str,
    pub persistent_disclaimer_lede: &'static str,
    pub full_disclaimer_link: &'static str,
    pub all_rights_reserved: &'static str,
    pub powered_by: &'static str,
}

/// `/software` catalog page chrome strings. Product `name`/`description` text
/// itself comes from `products.yaml` and stays English on both language
/// variants — no translation source exists for per-product copy yet (flagged,
/// not fabricated); only the surrounding page chrome is translated here.
pub struct CatalogLabels {
    pub eyebrow: &'static str,
    pub title: &'static str,
    pub lede: &'static str,
    pub free_open_source: &'static str,
    pub beta_free: &'static str,
    pub available_now: &'static str,
    pub copy: &'static str,
    pub copied: &'static str,
    pub copy_aria: &'static str,
    pub install_hint: &'static str,
    pub download_hint: &'static str,
    pub price_unit: &'static str,
    pub pay_cta: &'static str,
    pub pay_aria_fmt: &'static str,
    pub pay_meta: &'static str,
}

impl Lang {
    pub fn catalog_labels(self) -> CatalogLabels {
        match self {
            Lang::En => CatalogLabels {
                eyebrow: "The Binary Library",
                title: "Products",
                lede: "Buy it once. Run it anywhere. Own it forever.",
                free_open_source: "Free \u{b7} open source",
                beta_free: "BETA \u{b7} free",
                available_now: "Available now",
                copy: "Copy",
                copied: "Copied",
                copy_aria: "Copy install command to clipboard",
                install_hint:
                    "Linux x86_64 \u{b7} verifies SHA256 against the per-version MANIFEST",
                download_hint:
                    "Bootable appliance image \u{b7} see the product page for boot instructions",
                price_unit: "USDC \u{2014} own it forever, no subscription",
                pay_cta: "Pay with Polygon USDC",
                pay_aria_fmt: "Pay for {} with Polygon USDC",
                pay_meta:
                    "Polygon PoS \u{b7} native USDC \u{b7} a permanent, portable record you hold",
            },
            Lang::Es => CatalogLabels {
                eyebrow: "La Biblioteca Binaria",
                title: "Productos",
                lede: "C\u{f3}mprelo una vez. Ej\u{e9}cutelo donde quiera. S\u{e9}alo suyo para \
                       siempre.",
                free_open_source: "Gratis \u{b7} c\u{f3}digo abierto",
                beta_free: "BETA \u{b7} gratis",
                available_now: "Disponible ahora",
                copy: "Copiar",
                copied: "Copiado",
                copy_aria: "Copiar el comando de instalaci\u{f3}n al portapapeles",
                install_hint:
                    "Linux x86_64 \u{b7} verifica SHA256 contra el MANIFEST de cada versi\u{f3}n",
                download_hint:
                    "Imagen de apliance arrancable \u{b7} vea la ficha del producto para las \
                       instrucciones de arranque",
                price_unit: "USDC \u{2014} su\u{f3}yalo para siempre, sin suscripci\u{f3}n",
                pay_cta: "Pagar con Polygon USDC",
                pay_aria_fmt: "Pagar {} con Polygon USDC",
                pay_meta: "Polygon PoS \u{b7} USDC nativo \u{b7} un registro permanente y \
                       port\u{e1}til que usted conserva",
            },
        }
    }
}

const LANG_SWITCH_GLYPH: &str = r##"<svg class="sw-lang-switch__glyph" viewBox="0 0 24 24" aria-hidden="true"><path fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" d="M12 3a9 9 0 100 18 9 9 0 000-18zM3 12h18M12 3c2.5 2.6 3.75 5.6 3.75 9S14.5 18.4 12 21c-2.5-2.6-3.75-5.6-3.75-9S9.5 5.6 12 3z"></path></svg>"##;

/// The language-switch pill: globe glyph + text label, linking to `target_href`
/// (the equivalent page in the other language — callers resolve this since only
/// they know whether the current page has a translated sibling or must fall back
/// to the other language's `/software` home).
///
/// Below the 768px breakpoint the masthead instance collapses to icon-only (its
/// text label is hidden via CSS) — verified live on home.pointsav.com, which
/// does the same at its own breakpoint. Pass `in_drawer: true` for the mobile
/// nav drawer's own copy, which keeps its label and a bigger, easier tap target
/// regardless of viewport — same icon-collapses-but-drawer-keeps-label split
/// home.pointsav.com uses (`.m-masthead .m-lang-switch__label` vs.
/// `.m-drawer .m-lang-switch`).
pub fn lang_switch(lang: Lang, target_href: &str, in_drawer: bool) -> Markup {
    let other = lang.other();
    let class = if in_drawer {
        "sw-lang-switch sw-lang-switch--drawer"
    } else {
        "sw-lang-switch"
    };
    html! {
        a class=(class) href=(target_href) lang=(other.code()) hreflang=(other.code())
            rel="alternate" aria-label=(lang.toggle_label()) {
            (PreEscaped(LANG_SWITCH_GLYPH))
            span."sw-lang-switch__label" { (lang.toggle_label()) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_and_localize_match_home_pointsav_com_convention() {
        assert_eq!(Lang::En.prefix(), "");
        assert_eq!(Lang::Es.prefix(), "/es");
        assert_eq!(Lang::Es.localize("/software"), "/es/software");
        assert_eq!(Lang::En.localize("/software"), "/software");
    }

    #[test]
    fn other_is_reciprocal() {
        assert_eq!(Lang::En.other(), Lang::Es);
        assert_eq!(Lang::Es.other(), Lang::En);
    }

    #[test]
    fn lang_switch_toggles_to_the_other_language_with_its_own_hreflang() {
        let html = lang_switch(Lang::En, "/es/software", false).into_string();
        assert!(html.contains(r#"href="/es/software""#));
        assert!(html.contains(r#"hreflang="es""#));
        assert!(html.contains("Español"));

        let html_es = lang_switch(Lang::Es, "/software", false).into_string();
        assert!(html_es.contains(r#"href="/software""#));
        assert!(html_es.contains(r#"hreflang="en""#));
        assert!(html_es.contains("English"));
    }

    #[test]
    fn drawer_variant_carries_the_modifier_class_masthead_variant_does_not() {
        let masthead = lang_switch(Lang::En, "/es/software", false).into_string();
        assert!(masthead.contains(r#"class="sw-lang-switch""#));
        assert!(!masthead.contains("sw-lang-switch--drawer"));

        let drawer = lang_switch(Lang::En, "/es/software", true).into_string();
        assert!(drawer.contains(r#"class="sw-lang-switch sw-lang-switch--drawer""#));
    }
}
