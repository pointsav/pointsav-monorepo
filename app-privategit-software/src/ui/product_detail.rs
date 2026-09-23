// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Product detail page — `GET /software/:product_id` (S136, requested by
//! Command 2026-06-30). One page per catalog entry: BETA badge, tier badge,
//! platform table, curl install command, version, SHA256, and an optional
//! GUIDE link.
//!
//! Two fields the catalog does not carry are handled explicitly rather than
//! fabricated:
//!
//! - **Platform table** ships as a single row today (`i.platform` display
//!   label + the existing `linux-x86_64` download-slug convention already
//!   used by the paid download flow, `main.rs`'s `order_download`). Real
//!   per-product multi-platform data would need a schema change
//!   (`platforms: Vec<PlatformArtifact>` on `Installer`) — tracked as a
//!   follow-up in `NEXT.md`, not blocking this page.
//! - **SHA256** is fetched client-side from `app-privategit-source-2`'s
//!   `GET /releases/:product/:version/MANIFEST` endpoint (same source
//!   `ui::catalog`'s doc comment already names as the correct home for this
//!   data). This avoids adding a new server-side HTTP client dependency and a
//!   new marketplace→source-2 network coupling at render time. Degrades to a
//!   visible "verify via MANIFEST" link if the fetch fails or JS is disabled.

use crate::ui::Lang;
use crate::{Installer, LicenseTier};
use maud::{html, Markup, PreEscaped};
use serde_json::json;

/// Duplicated from `ui::catalog` rather than made `pub(crate)` — matches this
/// crate's established preference for small per-page duplication over
/// cross-module coupling (see `checkout.rs`/`order.rs`, which each carry
/// their own `_style()` rather than sharing one).
fn install_command(source_base_url: &str, id: &str) -> String {
    let base = source_base_url.trim_end_matches('/');
    format!("curl -fsSL {base}/{id}/install.sh | bash")
}

fn tier_badge(tier: LicenseTier) -> Markup {
    html! { span."sw-cat-badge sw-cat-badge--tier" { (tier.label()) } }
}

fn arch_tier_badge(tier: crate::ArchTier) -> Markup {
    html! { span."sw-cat-badge sw-cat-badge--arch" { (tier.label()) } }
}

/// Vanilla-JS SHA256 fetch — populates `#sw-pd-sha-value` from the per-version
/// MANIFEST endpoint. The visible "verify via MANIFEST" link stays in the
/// markup regardless, so the page degrades honestly if this fails or JS is
/// disabled, rather than fabricating a value.
fn sha_fetch_script(manifest_url: &str) -> Markup {
    let url_json = serde_json::to_string(manifest_url).unwrap_or_else(|_| "\"\"".to_string());
    let js = format!(
        r#"(function(){{
fetch({url_json}).then(function(r){{return r.json();}}).then(function(m){{
  var el=document.getElementById('sw-pd-sha-value');
  if(el&&m&&m.sha256){{el.textContent=m.sha256;}}
}}).catch(function(){{}});
}})();"#
    );
    html! { script { (PreEscaped(js)) } }
}

fn product_detail_style() -> Markup {
    let css = r#".sw-pd-wrap{max-width:760px;margin:0 auto;padding:40px 24px 64px;box-sizing:border-box;}
.sw-pd-card{border:1px solid #e4e7ec;border-radius:10px;padding:28px;background:#fff;box-shadow:0 1px 2px rgba(16,24,40,.04);}
.sw-pd-id{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:11px;color:#667085;letter-spacing:.02em;}
.sw-pd-name{font-family:Georgia,"Times New Roman",serif;font-size:28px;line-height:1.2;color:#111827;margin:6px 0 10px;}
.sw-pd-desc{font-size:14px;line-height:1.6;color:#475467;margin:0 0 20px;max-width:64ch;}
.sw-pd-badges{display:flex;flex-wrap:wrap;gap:6px;margin:0 0 28px;}
.sw-pd-h2{font-size:13px;letter-spacing:.1em;text-transform:uppercase;color:#234ed8;margin:0 0 12px;padding-bottom:6px;border-bottom:1px solid #e4e7ec;}
.sw-pd-table{width:100%;border-collapse:collapse;margin:0 0 28px;font-size:13.5px;}
.sw-pd-table th{text-align:left;color:#667085;font-size:11px;letter-spacing:.06em;text-transform:uppercase;padding:0 0 8px;}
.sw-pd-table td{padding:8px 0;border-top:1px solid #e4e7ec;color:#344054;}
.sw-pd-table a{color:#234ed8;font-weight:600;text-decoration:none;}
.sw-pd-table a:hover{color:#173ab1;}
.sw-pd-version{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:13px;color:#344054;margin:0 0 10px;}
.sw-pd-sha{font-size:12.5px;color:#475467;line-height:1.6;margin:0 0 28px;word-break:break-all;}
.sw-pd-sha__label{font-weight:600;color:#344054;}
.sw-pd-sha__value{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;}
.sw-pd-sha__fallback{color:#234ed8;}
.sw-pd-back{margin-top:28px;font-size:13px;}
.sw-pd-back a{color:#234ed8;text-decoration:none;}
.sw-pd-back a:hover{color:#173ab1;}
.sw-cat-badge{font-size:11px;font-weight:600;letter-spacing:.02em;padding:3px 8px;border-radius:999px;background:#f2f4f7;color:#344054;white-space:nowrap;}
.sw-cat-badge--free{background:#ecfdf3;color:#067647;}
.sw-cat-badge--ver{background:#eef3ff;color:#234ed8;font-family:ui-monospace,SFMono-Regular,Menlo,monospace;}
.sw-cat-badge--tier{background:#eef3ff;color:#234ed8;}
.sw-cat-badge--arch{background:#f2f4f7;color:#475467;border:1px solid #e4e7ec;}
.sw-cat-card__becomes{font-size:12.5px;line-height:1.5;color:#344054;margin:0 0 20px;}
.sw-cat-cmd{display:flex;align-items:stretch;background:#0e1117;border-radius:6px;overflow:hidden;}
.sw-cat-cmd__text{flex:1;min-width:0;color:#e6edf3;font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:12px;line-height:1.4;padding:9px 11px;overflow-x:auto;white-space:nowrap;}
.sw-cat-cmd__copy{border:0;background:#234ed8;color:#fff;font-size:11px;font-weight:600;padding:0 14px;cursor:pointer;letter-spacing:.04em;flex:0 0 auto;}
.sw-cat-cmd__copy:hover{background:#173ab1;}"#;
    html! { style { (PreEscaped(css)) } }
}

/// Static-label translations for this page — installer name/description stay
/// English regardless of `lang` (no translation source exists for catalog data,
/// same convention as `catalog.rs`/`checkout.rs`).
struct ProductDetailLabels {
    platform_h2: &'static str,
    platform_col: &'static str,
    download_col: &'static str,
    install_h2: &'static str,
    copy_label: &'static str,
    copy_aria: &'static str,
    version_h2: &'static str,
    sha_label: &'static str,
    sha_verifying: &'static str,
    sha_fallback: &'static str,
    guide_h2: &'static str,
    guide_link: &'static str,
    back_link: &'static str,
    boot_h2: &'static str,
}

impl ProductDetailLabels {
    fn for_lang(lang: Lang) -> Self {
        match lang {
            Lang::En => Self {
                platform_h2: "Platform",
                platform_col: "Platform",
                download_col: "Download",
                install_h2: "Install",
                copy_label: "Copy",
                copy_aria: "Copy install command to clipboard",
                version_h2: "Version & checksum",
                sha_label: "SHA256: ",
                sha_verifying: "verifying\u{2026}",
                sha_fallback: "verify via MANIFEST",
                guide_h2: "Guide",
                guide_link: "Operational guide",
                back_link: "\u{2190} All products",
                boot_h2: "Boot",
            },
            Lang::Es => Self {
                platform_h2: "Plataforma",
                platform_col: "Plataforma",
                download_col: "Descarga",
                install_h2: "Instalaci\u{f3}n",
                copy_label: "Copiar",
                copy_aria: "Copiar comando de instalaci\u{f3}n al portapapeles",
                version_h2: "Versi\u{f3}n y suma de verificaci\u{f3}n",
                sha_label: "SHA256: ",
                sha_verifying: "verificando\u{2026}",
                sha_fallback: "verificar via MANIFEST",
                guide_h2: "Gu\u{ed}a",
                guide_link: "Gu\u{ed}a operativa",
                back_link: "\u{2190} Todos los productos",
                boot_h2: "Arranque",
            },
        }
    }
}

// Duplicated from `ui::layout` rather than made `pub(crate)` — matches this crate's
// established per-page-duplication preference (see `install_command` above).
const SITE_URL: &str = "https://software.pointsav.com";

/// `SoftwareApplication` + `BreadcrumbList` JSON-LD for a product-detail page (Tier 4,
/// closing the gap `BRIEF-software-handoff-readiness.md` flagged: only `/software`
/// carried JSON-LD, despite this page being the archetype's natural home for
/// per-product structured data). Uses `serde_json::json!` rather than hand-formatted
/// strings, unlike `catalog.rs`'s single static JSON-LD block — installer name/
/// description are dynamic, untrusted-shape strings that need real JSON escaping.
fn json_ld_script(i: &Installer, lang: Lang, page_path: &str) -> Markup {
    let price = i.price_usdc as f64 / 1_000_000.0;
    let data = json!({
        "@context": "https://schema.org",
        "@type": "SoftwareApplication",
        "name": i.name,
        "description": i.description,
        "applicationCategory": "BusinessApplication",
        "softwareVersion": i.edition,
        "offers": {
            "@type": "Offer",
            "price": format!("{price:.2}"),
            "priceCurrency": "USD",
        },
        "provider": {"@type": "Organization", "@id": "https://pointsav.com/#organization"},
    });
    // L3 fix: "Home" and "Software" previously both pointed at the same
    // `/software` URL -- a BreadcrumbList entry that doesn't actually advance
    // toward the page is invalid per Google's structured-data guidance ("each
    // item should represent a page in the hierarchy"). "Home" now points at the
    // site root instead of duplicating the "Software" entry's target.
    let breadcrumbs = json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": [
            {"@type": "ListItem", "position": 1, "name": "Home", "item": format!("{SITE_URL}{}", lang.localize(""))},
            {"@type": "ListItem", "position": 2, "name": "Software", "item": format!("{SITE_URL}{}", lang.localize("/software"))},
            {"@type": "ListItem", "position": 3, "name": i.name, "item": format!("{SITE_URL}{page_path}")},
        ],
    });
    let data_json = escape_for_script_tag(&serde_json::to_string(&data).unwrap_or_default());
    let breadcrumbs_json =
        escape_for_script_tag(&serde_json::to_string(&breadcrumbs).unwrap_or_default());
    html! {
        script type="application/ld+json" { (PreEscaped(data_json)) }
        script type="application/ld+json" { (PreEscaped(breadcrumbs_json)) }
    }
}

/// L2 fix: the doc comment above claims `serde_json` gives "real JSON escaping" for
/// the untrusted-shape name/description fields, but JSON-string escaping alone
/// doesn't protect the embedding script tag -- serde_json doesn't escape the
/// less-than sign, so an installer name containing a literal close-script-tag /
/// open-script-tag sequence would close the JSON-LD block early and inject a
/// sibling script element. Replacing that raw byte with its `<` unicode
/// escape is semantically a no-op for any JSON parser (identical decoded string)
/// but makes a close-script-tag un-splicable in the surrounding HTML -- the
/// standard mitigation for JSON-in-script-tag embedding (matches e.g. Django's
/// `json_script` / OWASP's JSON-in-HTML guidance).
fn escape_for_script_tag(json: &str) -> String {
    json.replace('<', "\\u003c")
}

pub fn product_detail_markup(i: &Installer, source_base_url: &str, lang: Lang) -> Markup {
    let base = source_base_url.trim_end_matches('/');
    let manifest_url = format!("{base}/{}/{}/MANIFEST", i.id, i.edition);
    let platform_slug = i.platform_slug();
    let download_url = format!("{base}/{}/{}/{platform_slug}", i.id, i.edition);
    let command = install_command(source_base_url, &i.id);
    // Duplicated from `ui::catalog::download_command` rather than made `pub(crate)` —
    // matches this crate's established per-page-duplication preference.
    let download_command = format!("curl -fsSL {download_url} -o {platform_slug}");
    let l = ProductDetailLabels::for_lang(lang);
    let all_products_href = lang.localize("/software");
    let page_path = lang.localize(&format!("/software/{}", i.id));

    html! {
        (product_detail_style())
        (json_ld_script(i, lang, &page_path))
        div."sw-pd-wrap" {
            article."sw-pd-card" {
                span."sw-pd-id" { (i.id) }
                h1."sw-pd-name" { (i.name) }
                p."sw-pd-desc" { (i.description) }
                div."sw-pd-badges" {
                    @if i.price_usdc == 0 {
                        span."sw-cat-badge sw-cat-badge--free" { "BETA \u{00b7} free" }
                    }
                    (tier_badge(i.license_tier))
                    (arch_tier_badge(i.tier))
                    span."sw-cat-badge sw-cat-badge--ver" { "v" (i.edition) }
                }
                @if let Some(becomes) = &i.becomes {
                    p."sw-cat-card__becomes" { (becomes) }
                }

                h2."sw-pd-h2" { (l.platform_h2) }
                table."sw-pd-table" {
                    thead { tr { th { (l.platform_col) } th { (l.download_col) } } }
                    tbody {
                        tr {
                            td { (i.platform) }
                            td { a href=(download_url) { (platform_slug) } }
                        }
                    }
                }

                @if i.artifact == crate::ArtifactKind::ApplianceImage {
                    h2."sw-pd-h2" { (l.boot_h2) }
                    div."sw-cat-install" {
                        div."sw-cat-cmd" {
                            code."sw-cat-cmd__text" { (download_command) }
                            button."sw-cat-cmd__copy" type="button"
                                data-sw-clip=(download_command) data-sw-label=(l.copy_label)
                                aria-label=(l.copy_aria) { (l.copy_label) }
                        }
                    }
                    @if let Some(notes) = &i.boot_notes {
                        pre { (notes) }
                    }
                } @else {
                    h2."sw-pd-h2" { (l.install_h2) }
                    div."sw-cat-install" {
                        div."sw-cat-cmd" {
                            code."sw-cat-cmd__text" { (command) }
                            button."sw-cat-cmd__copy" type="button"
                                data-sw-clip=(command) data-sw-label=(l.copy_label)
                                aria-label=(l.copy_aria) { (l.copy_label) }
                        }
                    }
                }

                h2."sw-pd-h2" { (l.version_h2) }
                p."sw-pd-version" { "v" (i.edition) }
                p."sw-pd-sha" {
                    span."sw-pd-sha__label" { (l.sha_label) }
                    span."sw-pd-sha__value" #"sw-pd-sha-value" { (l.sha_verifying) }
                    " \u{2014} "
                    a."sw-pd-sha__fallback" href=(manifest_url) { (l.sha_fallback) }
                }

                @if let Some(url) = &i.guide_url {
                    h2."sw-pd-h2" { (l.guide_h2) }
                    p { a href=(url) { (l.guide_link) } }
                }

                p."sw-pd-back" { a href=(all_products_href) { (l.back_link) } }
            }
        }
        (sha_fetch_script(&manifest_url))
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
//
// Pure rendering function — no server, no filesystem, no network.
#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "https://example.invalid/releases";

    fn fixture(price_usdc: u64, guide_url: Option<String>) -> Installer {
        Installer {
            id: "os-mediakit".into(),
            name: "MediaKit OS".into(),
            description: "Sovereign media workstation image.".into(),
            edition: "1.2.0".into(),
            platform: "linux-x86_64".into(),
            size_mb: 812,
            path: "os-mediakit/1.2.0/installer.run".into(),
            license_tier: LicenseTier::Fsl,
            price_usdc,
            fsl_conversion_date: None,
            guide_url,
            artifact: crate::ArtifactKind::CliBinary,
            platform_slug: None,
            boot_notes: None,
            tier: crate::ArchTier::Platform,
            facts: vec![],
            bundled_registry_products: vec![],
            family: crate::ProductFamily::Independent,
            becomes: None,
        }
    }

    #[test]
    fn arch_tier_badge_and_becomes_line_render_when_set() {
        let mut i = fixture(0, None);
        i.tier = crate::ArchTier::Delivery;
        i.becomes = Some("Turns a machine into a test wiki.".into());
        let html = product_detail_markup(&i, BASE, Lang::En).into_string();
        assert!(html.contains("sw-cat-badge--arch"));
        assert!(html.contains("Delivery"));
        assert!(html.contains("Turns a machine into a test wiki."));
        // Same "product/taxonomy data stays untranslated" convention as license-tier
        // labels and descriptions (see `spanish_translates_static_labels_not_product_data`).
        let html_es = product_detail_markup(&i, BASE, Lang::Es).into_string();
        assert!(html_es.contains("Delivery"));
        assert!(html_es.contains("Turns a machine into a test wiki."));
    }

    #[test]
    fn arch_tier_badge_always_renders_but_becomes_line_is_still_optional() {
        // Phase 1b: `tier` became a required field, so the arch-tier badge is now
        // unconditional on product_detail.rs (kept here — unlike catalog.rs's cards,
        // there's no section heading here to convey tier another way). `becomes`
        // stays genuinely optional; this asserts that half still behaves correctly.
        let html = product_detail_markup(&fixture(0, None), BASE, Lang::En).into_string();
        assert!(html.contains(r#"class="sw-cat-badge sw-cat-badge--arch""#));
        assert!(!html.contains(r#"class="sw-cat-card__becomes""#));
    }

    #[test]
    fn renders_badge_platform_install_and_version() {
        let html = product_detail_markup(&fixture(0, None), BASE, Lang::En).into_string();
        assert!(html.contains("os-mediakit"));
        assert!(html.contains("MediaKit OS"));
        assert!(html.contains("BETA \u{00b7} free"));
        assert!(html.contains("FSL"));
        assert!(html.contains("v1.2.0"));
        assert!(html.contains("linux-x86_64"));
        assert!(html
            .contains("curl -fsSL https://example.invalid/releases/os-mediakit/install.sh | bash"));
        assert!(html.contains("https://example.invalid/releases/os-mediakit/1.2.0/MANIFEST"));
        assert!(html.contains("https://example.invalid/releases/os-mediakit/1.2.0/linux-x86_64"));
    }

    #[test]
    fn paid_product_omits_beta_badge() {
        let html = product_detail_markup(&fixture(1_000_000, None), BASE, Lang::En).into_string();
        assert!(!html.contains("BETA \u{00b7} free"));
    }

    #[test]
    fn appliance_image_shows_boot_notes_instead_of_an_install_command() {
        let mut installer = fixture(0, None);
        installer.artifact = crate::ArtifactKind::ApplianceImage;
        installer.platform_slug = Some("loader.img".into());
        installer.boot_notes =
            Some("qemu-system-aarch64 -machine virt -device loader,file=loader.img".into());
        let html = product_detail_markup(&installer, BASE, Lang::En).into_string();
        // No curl-pipe-sh install command for an appliance image.
        assert!(!html.contains("| bash"));
        // Boot heading + real boot_notes content present instead.
        assert!(html.contains("Boot"));
        assert!(html.contains("qemu-system-aarch64"));
        // The platform table's download link uses the real platform_slug, not a
        // hardcoded "linux-x86_64".
        assert!(html.contains("https://example.invalid/releases/os-mediakit/1.2.0/loader.img"));
        assert!(!html.contains("os-mediakit/1.2.0/linux-x86_64"));
    }

    #[test]
    fn guide_link_renders_only_when_present() {
        let without = product_detail_markup(&fixture(0, None), BASE, Lang::En).into_string();
        assert!(!without.contains("Operational guide"));

        let with = product_detail_markup(
            &fixture(0, Some("https://docs.example.invalid/guide".into())),
            BASE,
            Lang::En,
        )
        .into_string();
        assert!(with.contains("Operational guide"));
        assert!(with.contains("https://docs.example.invalid/guide"));
    }

    #[test]
    fn sha_placeholder_and_fetch_script_present() {
        let html = product_detail_markup(&fixture(0, None), BASE, Lang::En).into_string();
        assert!(html.contains("sw-pd-sha-value"));
        assert!(html.contains("verify via MANIFEST"));
        assert!(html.contains("fetch("));
    }

    // ── JSON-LD / BreadcrumbList (Tier 4) ───────────────────────────────────────

    #[test]
    fn json_ld_software_application_and_breadcrumbs_present_and_valid() {
        let html = product_detail_markup(&fixture(1_000_000, None), BASE, Lang::En).into_string();
        let blocks: Vec<&str> = html
            .split("<script type=\"application/ld+json\">")
            .skip(1)
            .map(|s| s.split("</script>").next().unwrap())
            .collect();
        assert_eq!(blocks.len(), 2, "expected exactly 2 JSON-LD blocks");

        let app: serde_json::Value = serde_json::from_str(blocks[0]).unwrap();
        assert_eq!(app["@type"], "SoftwareApplication");
        assert_eq!(app["name"], "MediaKit OS");
        assert_eq!(app["offers"]["price"], "1.00");
        assert_eq!(app["offers"]["priceCurrency"], "USD");

        let breadcrumbs: serde_json::Value = serde_json::from_str(blocks[1]).unwrap();
        assert_eq!(breadcrumbs["@type"], "BreadcrumbList");
        let items = breadcrumbs["itemListElement"].as_array().unwrap();
        assert_eq!(items.len(), 3);
        assert_eq!(items[2]["name"], "MediaKit OS");
        assert!(items[2]["item"]
            .as_str()
            .unwrap()
            .ends_with("/software/os-mediakit"));
    }

    #[test]
    fn json_ld_breadcrumb_home_and_software_items_are_distinct_urls() {
        // L3: "Home" and "Software" previously pointed at the identical URL --
        // not a real breadcrumb hierarchy.
        let html = product_detail_markup(&fixture(1_000_000, None), BASE, Lang::En).into_string();
        let blocks: Vec<&str> = html
            .split("<script type=\"application/ld+json\">")
            .skip(1)
            .map(|s| s.split("</script>").next().unwrap())
            .collect();
        let breadcrumbs: serde_json::Value = serde_json::from_str(blocks[1]).unwrap();
        let items = breadcrumbs["itemListElement"].as_array().unwrap();
        assert_eq!(items[0]["name"], "Home");
        assert_eq!(items[1]["name"], "Software");
        assert_ne!(items[0]["item"], items[1]["item"]);
        assert_eq!(items[0]["item"], "https://software.pointsav.com");
    }

    #[test]
    fn json_ld_breadcrumb_localizes_to_es_path() {
        let html = product_detail_markup(&fixture(0, None), BASE, Lang::Es).into_string();
        assert!(html.contains("\"item\":\"https://software.pointsav.com/es/software/os-mediakit\""));
    }

    #[test]
    fn json_ld_name_containing_closing_script_tag_cannot_break_out_of_the_script_element() {
        // L2: serde_json's string escaping alone doesn't protect a <script> embedding
        // -- it never escapes '<'. A name/description containing a literal
        // "</script><script>...", if not additionally escaped, would close the
        // JSON-LD block early and splice in a sibling script element.
        let mut i = fixture(0, None);
        i.name = "</script><script>alert(1)</script>".into();
        let html = product_detail_markup(&i, BASE, Lang::En).into_string();
        assert!(
            !html.contains("</script><script>alert(1)"),
            "a literal close/open script sequence in installer data must not survive \
             into the rendered HTML unescaped: {html}"
        );
        // And the JSON is still valid / round-trips to the original string once parsed.
        let blocks: Vec<&str> = html
            .split("<script type=\"application/ld+json\">")
            .skip(1)
            .map(|s| s.split("</script>").next().unwrap())
            .collect();
        let app: serde_json::Value = serde_json::from_str(blocks[0]).unwrap();
        assert_eq!(app["name"], "</script><script>alert(1)</script>");
    }

    // ── /es/* extension (Spanish localization follow-up, BRIEF-software-spanish-localization.md) ──

    #[test]
    fn spanish_translates_static_labels_not_product_data() {
        let html = product_detail_markup(
            &fixture(0, Some("https://docs.example.invalid/guide".into())),
            BASE,
            Lang::Es,
        )
        .into_string();
        // Static labels translated.
        assert!(html.contains("Plataforma"));
        assert!(html.contains("Descarga"));
        assert!(html.contains("Instalaci\u{f3}n"));
        assert!(html.contains("Versi\u{f3}n y suma de verificaci\u{f3}n"));
        assert!(html.contains("verificando\u{2026}"));
        assert!(html.contains("verificar via MANIFEST"));
        assert!(html.contains("Gu\u{ed}a operativa"));
        assert!(html.contains("Todos los productos"));
        // Product name/description are NOT translated — no translation source exists.
        assert!(html.contains("MediaKit OS"));
        assert!(html.contains("Sovereign media workstation image."));
    }

    #[test]
    fn spanish_back_link_localizes_to_es_software() {
        let html = product_detail_markup(&fixture(0, None), BASE, Lang::Es).into_string();
        assert!(html.contains("href=\"/es/software\""));
    }

    #[test]
    fn english_back_link_is_unprefixed() {
        let html = product_detail_markup(&fixture(0, None), BASE, Lang::En).into_string();
        assert!(html.contains("href=\"/software\""));
    }
}
