//! Chrome — parameterised page shell emitters.
//!
//! All reader-visible chrome (header, tabs, TOC, bottom bar, palette) lives
//! here. L6 enforcement: "Chrome rendering lives in one parameterised chrome
//! emitter — never multiple inline *_chrome copies in the same handler file."
//!
//! Phase 1 stubs. Phase 3 implements all chrome modules with full HTML using
//! the maud templating library.
//!
//! Module layout:
//! - `mod.rs` — `base_chrome()` + `strings(locale)` locale map (L22)
//! - `article.rs` — article chrome: tabs, TOC, hatnote, infobox, status badge
//! - `home.rs` — homepage chrome: category grid / thematic clusters / due-diligence path
//! - `palette.rs` — Cmd+K command palette
//! - `mobile.rs` — mobile bottom bar with safe-area insets (L24)

pub mod article;
pub mod home;
pub mod mobile;
pub mod palette;

use maud::Markup;

/// Locale selector for bilingual chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Locale {
    #[default]
    En,
    Es,
}

/// Locale-keyed UI string map (L22).
///
/// All reader-visible chrome strings come from here — no hardcoded English
/// strings may appear in any chrome emitter. Acceptance test:
/// `cargo test es_homepage_chrome_is_spanish` (Phase 3).
pub fn strings(locale: Locale) -> &'static [(&'static str, &'static str)] {
    match locale {
        Locale::En => &[
            ("article", "Article"),
            ("talk", "Talk"),
            ("edit", "Edit"),
            ("history", "History"),
            ("search_placeholder", "Search articles…"),
            ("toc_heading", "Contents"),
            ("see_also", "See also"),
            ("categories", "Categories"),
            ("backlinks", "Referenced by"),
            ("recently_changed", "Recently changed"),
            ("status_stub", "Seedling"),
            ("status_pre_build", "In Development"),
            ("status_active", "Active"),
            ("status_complete", "Evergreen"),
            ("home", "Home"),
            ("start_here", "Start here"),
        ],
        Locale::Es => &[
            ("article", "Artículo"),
            ("talk", "Discusión"),
            ("edit", "Editar"),
            ("history", "Historial"),
            ("search_placeholder", "Buscar artículos…"),
            ("toc_heading", "Contenidos"),
            ("see_also", "Véase también"),
            ("categories", "Categorías"),
            ("backlinks", "Referenciado por"),
            ("recently_changed", "Modificado recientemente"),
            ("status_stub", "Semilla"),
            ("status_pre_build", "En desarrollo"),
            ("status_active", "Activo"),
            ("status_complete", "Permanente"),
            ("home", "Inicio"),
            ("start_here", "Empieza aquí"),
        ],
    }
}

/// Look up a single locale string by key. Returns an empty string if not found
/// (safe fallback that prevents panics in templates).
pub fn t(locale: Locale, key: &'static str) -> &'static str {
    strings(locale)
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
        .unwrap_or(key)
}

/// Render the full page shell wrapping `content`.
///
/// Phase 1 stub. Phase 3 emits:
/// - DOCTYPE + `<html lang="…" data-theme="light" data-instance="…">`
/// - `<head>` with two font preload tags (L23: Inter + Source Serif 4),
///   token CSS link (`tokens.css` or `tokens-woodfine.css`), `style.css`
/// - Sticky compact header with logo, site title, Cmd+K button
/// - The `content` Markup
/// - Footer with canonical trademark text (L7)
/// - `<script src="/static/wiki.js">` (L25: no editor.js on article pages)
#[allow(unused_variables)]
pub fn base_chrome(content: Markup, locale: &str, brand: &str) -> Markup {
    todo!("Phase 3: implement base_chrome with font preloads (L23), token CSS, footer (L7)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strings_en_has_expected_keys() {
        let s = strings(Locale::En);
        let keys: Vec<&str> = s.iter().map(|(k, _)| *k).collect();
        assert!(keys.contains(&"article"));
        assert!(keys.contains(&"toc_heading"));
        assert!(keys.contains(&"status_complete"));
    }

    #[test]
    fn strings_es_has_same_keys_as_en() {
        let en_keys: std::collections::HashSet<&str> =
            strings(Locale::En).iter().map(|(k, _)| *k).collect();
        let es_keys: std::collections::HashSet<&str> =
            strings(Locale::Es).iter().map(|(k, _)| *k).collect();
        assert_eq!(en_keys, es_keys, "EN and ES string maps must have identical keys");
    }

    #[test]
    fn t_returns_known_keys() {
        // Verify that known keys resolve correctly.
        assert_eq!(t(Locale::En, "article"), "Article");
        assert_eq!(t(Locale::En, "toc_heading"), "Contents");
    }

    #[test]
    fn es_article_chrome_is_not_english() {
        // Acceptance gate for L22: /es/ chrome must not contain hardcoded English.
        // Phase 3 will expand this to a full page render check.
        assert_ne!(t(Locale::Es, "article"), t(Locale::En, "article"));
        assert_ne!(t(Locale::Es, "toc_heading"), t(Locale::En, "toc_heading"));
    }
}
