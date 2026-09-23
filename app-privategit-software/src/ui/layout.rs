// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Sovereign Editorial chrome: dark navy masthead (wordmark left / search centre /
//! utility right) and near-black institutional footer with the verbatim WCP Inc.
//! trademark line.
//!
//! The chrome is emitted as maud `Markup` and spliced into the storefront's
//! prerendered static HTML by [`wrap_static_html`] — the P1 static-file-reading
//! logic in `main.rs` is preserved; this only swaps the page's own light chrome
//! for the Sovereign masthead + footer and injects the scoped chrome stylesheet.
//!
//! Structure follows the app-mediakit-knowledge-2 `src/ui/layout.rs` pattern:
//! free `-> Markup` functions driven by the [`SoftwareSurface`] enum.

use maud::{html, Markup, PreEscaped, DOCTYPE};

use super::lang::{lang_switch, Lang};
use super::surface::SoftwareSurface;
use super::tokens;

// ── Inline SVG glyphs (currentColor → inherit the container's ink token) ────────

const SEARCH_ICON: &str = r##"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>"##;

const HAMBURGER_ICON: &str = r##"<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" aria-hidden="true" focusable="false"><line x1="3" y1="6" x2="21" y2="6"/><line x1="3" y1="12" x2="21" y2="12"/><line x1="3" y1="18" x2="21" y2="18"/></svg>"##;

const BADGE_GLYPH: &str = r##"<svg viewBox="0 0 24 24" width="14" height="14" aria-hidden="true" focusable="false"><path fill="currentColor" d="M3 5.5A1.5 1.5 0 0 1 4.5 4h15A1.5 1.5 0 0 1 21 5.5v13A1.5 1.5 0 0 1 19.5 20h-15A1.5 1.5 0 0 1 3 18.5v-13zM6 8v8l3.2-2.4L6 8zm7 6.5h5V13h-5v1.5zm0-3h5V10h-5v1.5z"/></svg>"##;

// ── Scoped chrome stylesheet ────────────────────────────────────────────────────
//
// All selectors are `sw-` prefixed so they never collide with the storefront's
// own inline styles. Token custom properties are emitted from `tokens.rs` below
// so the single source of truth is the Rust consts. The mobile drawer is a
// pure-CSS off-canvas panel (checkbox toggle) — no JavaScript asset in this phase.
//
// Body font: self-hosted Inter (400/600, latin + latin-ext), byte-identical copy
// from `app-mediakit-knowledge/static/fonts/` — the same family/sourcing
// convention already used by the wiki and app-mediakit-marketing-2 (OFL 1.1,
// zero third-party font CDN). Added 2026-07-12: this crate previously had no
// `body`/`html` font-family rule at all, so every non-Georgia element was
// silently falling back to the browser's default serif font — a real,
// visible inconsistency with the rest of the family, not a deliberate choice.
// Georgia stays for display headings (unaffected). Only 400/600 weights are
// fetched (matches what's actually used); the family's own font set only
// defines up to 600 for Inter, so `font-weight:700` selectors that had no
// Georgia override were adjusted to 600 to use a real face rather than
// browser-synthesized bold.

fn chrome_style() -> Markup {
    let css = format!(
        r#"@font-face{{font-family:"Inter";src:url("/static/fonts/Inter-400-normal-latin.woff2") format("woff2");font-weight:400;font-style:normal;font-display:swap;unicode-range:U+0000-00FF,U+0131,U+0152-0153,U+02BB-02BC,U+2000-206F,U+2074,U+20AC,U+2122,U+2191,U+2193,U+2212,U+2215,U+FEFF,U+FFFD;}}
@font-face{{font-family:"Inter";src:url("/static/fonts/Inter-400-normal-latin-ext.woff2") format("woff2");font-weight:400;font-style:normal;font-display:swap;unicode-range:U+0100-024F,U+0259,U+1E00-1EFF,U+2020,U+20A0-20AB,U+20AD-20CF,U+2113,U+2C60-2C7F,U+A720-A7FF;}}
@font-face{{font-family:"Inter";src:url("/static/fonts/Inter-600-normal-latin.woff2") format("woff2");font-weight:600;font-style:normal;font-display:swap;unicode-range:U+0000-00FF,U+0131,U+0152-0153,U+02BB-02BC,U+2000-206F,U+2074,U+20AC,U+2122,U+2191,U+2193,U+2212,U+2215,U+FEFF,U+FFFD;}}
@font-face{{font-family:"Inter";src:url("/static/fonts/Inter-600-normal-latin-ext.woff2") format("woff2");font-weight:600;font-style:normal;font-display:swap;unicode-range:U+0100-024F,U+0259,U+1E00-1EFF,U+2020,U+20A0-20AB,U+20AD-20CF,U+2113,U+2C60-2C7F,U+A720-A7FF;}}
@font-face{{font-family:"Source Serif 4";src:url("/static/fonts/Source-Serif-4-400-normal-latin.woff2") format("woff2");font-weight:400;font-style:normal;font-display:swap;unicode-range:U+0000-00FF,U+0131,U+0152-0153,U+2000-206F,U+2074,U+20AC,U+2122,U+2212,U+FEFF,U+FFFD;}}
@font-face{{font-family:"Source Serif 4";src:url("/static/fonts/Source-Serif-4-400-normal-latin-ext.woff2") format("woff2");font-weight:400;font-style:normal;font-display:swap;unicode-range:U+0100-024F,U+1E00-1EFF,U+20A0-20AB,U+2C60-2C7F,U+A720-A7FF;}}
@font-face{{font-family:"Source Serif 4";src:url("/static/fonts/Source-Serif-4-700-normal-latin.woff2") format("woff2");font-weight:700;font-style:normal;font-display:swap;unicode-range:U+0000-00FF,U+0131,U+0152-0153,U+2000-206F,U+2074,U+20AC,U+2122,U+2212,U+FEFF,U+FFFD;}}
@font-face{{font-family:"Source Serif 4";src:url("/static/fonts/Source-Serif-4-700-normal-latin-ext.woff2") format("woff2");font-weight:700;font-style:normal;font-display:swap;unicode-range:U+0100-024F,U+1E00-1EFF,U+20A0-20AB,U+2C60-2C7F,U+A720-A7FF;}}
:root{{
--sw-topnav-bg:{topnav};
--sw-on-chrome:{on_chrome};
--sw-on-chrome-muted:{on_chrome_muted};
--sw-accent:{accent};
--sw-accent-hover:{accent_hover};
--sw-footer-bg:{footer_bg};
--sw-footer-fg:{footer_fg};
--sw-footer-fg-muted:{footer_fg_muted};
--sw-footer-divider:{footer_div};
--sw-ink:{ink};
--sw-wordmark:{wordmark};
}}
body{{font-family:"Inter","Sans Fallback",system-ui,-apple-system,"Segoe UI",Arial,sans-serif;}}
.sw-masthead{{background:var(--sw-topnav-bg);color:var(--sw-on-chrome);width:100%;}}
.sw-masthead__inner{{display:flex;align-items:center;gap:24px;height:64px;max-width:1280px;margin:0 auto;padding:0 24px;box-sizing:border-box;}}
.sw-wordmark{{font-family:"Source Serif 4","Serif Fallback",Georgia,serif;color:var(--sw-on-chrome);text-decoration:none;font-weight:700;font-size:21px;letter-spacing:.005em;flex:0 0 auto;}}
.sw-masthead__nav{{display:flex;align-items:center;gap:20px;flex:0 0 auto;font-size:13px;letter-spacing:.01em;margin-left:auto;}}
.sw-masthead__nav a{{color:var(--sw-on-chrome-muted);text-decoration:none;padding-block:4px;}}
.sw-masthead__nav a:hover,.sw-masthead__nav a:focus-visible{{color:var(--sw-on-chrome);}}
.sw-search{{flex:0 0 auto;display:flex;}}
.sw-search__form{{display:flex;width:260px;background:rgba(255,255,255,.10);border:1px solid rgba(255,255,255,.18);border-radius:6px;overflow:hidden;}}
.sw-search__input{{flex:1;background:transparent;border:0;color:#fff;padding:8px 12px;font-size:13px;outline:none;}}
.sw-search__input::placeholder{{color:rgba(255,255,255,.6);}}
.sw-search__btn{{background:transparent;border:0;color:rgba(255,255,255,.85);padding:0 12px;cursor:pointer;display:inline-flex;align-items:center;}}
/* Pixel-matched against home.pointsav.com's `.m-lang-switch` (padding
   0.25rem/0.75rem, gap 0.5rem, radius --m-radius-control, border alpha .35 —
   its own tokens.css, not guessed) — operator found ours visibly different at
   mobile width even though the glyph path itself was already identical. */
.sw-lang-switch{{display:inline-flex;align-items:center;gap:8px;flex:0 0 auto;color:var(--sw-on-chrome);text-decoration:none;font-size:13px;white-space:nowrap;padding:4px 12px;border:1px solid rgba(255,255,255,.35);border-radius:2px;}}
.sw-lang-switch:hover,.sw-lang-switch:focus-visible{{border-color:#fff;}}
.sw-lang-switch__glyph{{width:14px;height:14px;flex-shrink:0;}}
.sw-lang-switch--drawer{{margin:12px 24px;align-self:flex-start;}}
.sw-hamburger{{display:none;background:transparent;border:0;color:var(--sw-on-chrome);padding:6px;cursor:pointer;align-items:center;}}
.sw-hamburger:focus-visible{{outline:2px solid #fff;outline-offset:2px;}}
.sw-mobile-nav{{display:none;background:var(--sw-topnav-bg);}}
.sw-mobile-nav a{{display:block;padding:12px 24px;color:var(--sw-on-chrome);text-decoration:none;font-size:14px;border-top:1px solid rgba(255,255,255,.12);}}
.sw-mobile-nav a:hover{{background:rgba(255,255,255,.06);}}
.sw-footer{{background:var(--sw-footer-bg);color:var(--sw-footer-fg);width:100%;border-top:1px solid var(--sw-footer-divider);}}
.sw-footer__inner{{max-width:1280px;margin:0 auto;padding:48px 24px 28px;box-sizing:border-box;}}
.sw-footer__top{{display:grid;grid-template-columns:1.4fr 1fr 1fr;gap:32px;}}
.sw-footer__brand-name{{color:var(--sw-ink);font-family:Georgia,"Times New Roman",serif;font-weight:700;font-size:18px;}}
.sw-footer__tagline{{margin-top:8px;font-size:13px;max-width:30ch;color:var(--sw-footer-fg-muted);}}
.sw-footer__col h2{{color:var(--sw-ink);font-size:12px;letter-spacing:.12em;text-transform:uppercase;margin:0 0 12px;}}
.sw-footer__col ul{{list-style:none;margin:0;padding:0;}}
.sw-footer__col li{{margin-bottom:8px;}}
.sw-footer__col a{{color:var(--sw-accent);text-decoration:none;font-size:13px;}}
.sw-footer__col a:hover{{color:var(--sw-accent-hover);text-decoration:underline;}}
.sw-footer__ext{{font-size:11px;margin-left:2px;}}
.sw-footer__disclosure{{margin-top:32px;border:1px solid var(--sw-footer-divider);border-radius:6px;}}
.sw-footer__disclosure-summary{{cursor:pointer;padding:14px 18px;color:var(--sw-ink);font-size:12px;letter-spacing:.06em;text-transform:uppercase;list-style:none;}}
.sw-footer__disclosure-summary::-webkit-details-marker{{display:none;}}
.sw-footer__disclosure-summary::after{{content:"\25be";margin-left:8px;display:inline-block;}}
.sw-footer__disclosure[open] .sw-footer__disclosure-summary::after{{transform:rotate(180deg);}}
.sw-footer__slot{{padding:0 18px 16px;}}
.sw-footer__slot-label{{font-size:11px;letter-spacing:.08em;text-transform:uppercase;color:var(--sw-accent);margin:0 0 8px;}}
.sw-footer__slot-body{{font-size:12.5px;line-height:1.6;color:var(--sw-footer-fg);max-width:70ch;}}
.sw-footer__slot-body p{{margin:0 0 10px;}}
.sw-footer__slot-body p:last-child{{margin-bottom:0;}}
.sw-footer__slot-body a{{color:var(--sw-accent);}}
.sw-footer__slot-body strong{{color:var(--sw-ink);}}
.sw-footer__persistent-disclaimer{{margin-top:12px;font-size:11px;line-height:1.5;color:var(--sw-footer-fg-muted);}}
.sw-footer__persistent-disclaimer a{{color:var(--sw-accent);}}
@media print{{
.sw-footer__disclosure:not([open]) .sw-footer__slot{{display:block!important;}}
}}
.sw-footer__cities{{margin-top:20px;padding-top:20px;border-top:1px solid var(--sw-footer-divider);font-size:12px;letter-spacing:.08em;text-transform:uppercase;color:var(--sw-footer-fg-muted);}}
.sw-footer__badge-row{{text-align:right;}}
.sw-footer__badge{{display:inline-flex;align-items:center;gap:6px;margin-top:14px;padding:5px 10px;background:#fff;border:1px solid var(--sw-footer-divider);border-radius:3px;text-decoration:none;color:var(--sw-footer-fg);}}
.sw-footer__badge-glyph{{display:inline-flex;color:var(--sw-accent);}}
.sw-footer__badge-text{{display:flex;flex-direction:column;line-height:1.1;}}
.sw-footer__badge-label{{font-size:9px;letter-spacing:.06em;text-transform:uppercase;color:var(--sw-footer-fg-muted);}}
.sw-footer__badge-name{{font-size:12px;font-weight:600;color:var(--sw-footer-fg);}}
.sw-footer__legal{{margin-top:20px;padding-top:18px;border-top:1px solid var(--sw-footer-divider);font-size:12px;line-height:1.6;}}
.sw-footer__copyright{{color:var(--sw-footer-fg-muted);margin:0 0 12px;}}
.sw-footer__trademark{{margin:0;color:var(--sw-footer-fg-muted);max-width:80ch;}}
.sw-legal{{max-width:70ch;margin:0 auto;padding:40px 24px 64px;line-height:1.65;}}
.sw-legal h1{{font-family:Georgia,"Times New Roman",serif;font-size:32px;margin:0 0 8px;}}
.sw-legal h2{{font-size:16px;margin:28px 0 8px;}}
.sw-legal__lede{{color:#555;margin:0 0 20px;}}
.sw-legal hr{{border:none;border-top:1px solid #ddd;margin:32px 0 20px;}}
.sw-legal__copyright,.sw-legal__trademark{{font-size:12px;color:#666;max-width:80ch;}}
/* Masthead-row breakpoint is 1024px, not the crate's usual 768px, and wider
   than home.pointsav.com's own 60rem/960px: same reasoning as that site's
   documented choice (its nav needs ~918px minimum to sit on one row without
   wrapping/overflowing) but ours needs still more room, because this crate's
   masthead also carries a search box home.pointsav.com's masthead does not.
   Measured live, not assumed: at the initial 768px value this row overflowed
   horizontally anywhere from 769px up through ~1020px, pushing the
   lang-toggle pill off-screen entirely (`scrollWidth > clientWidth` — first
   found at 960px still overflowing by 54px at 961px, tapering to 0px only at
   1020px). 1024px (a round, standard breakpoint) clears that with margin. */
@media (max-width:1024px){{
.sw-search{{display:none;}}
.sw-masthead__nav{{display:none;}}
.sw-masthead__inner{{gap:12px;}}
/* Real bug, only visible at wider mobile widths (found at 850x870 — a phone-
   width viewport like 390px is too narrow for the gap this produced to show):
   `margin-left:auto` was on `.sw-hamburger` ALONE, so only the hamburger got
   pushed to the far right — the toggle, with no auto-margin of its own, just
   sat in normal flow right after the wordmark, stranding it far from the
   hamburger with a growing gap as the viewport widens. Matches exactly the
   mechanism home.pointsav.com's own CSS uses: the auto-margin belongs on
   `.sw-lang-switch` (it claims the free space and gets pushed right), and the
   hamburger's own default auto-margin is cancelled by the adjacent-sibling
   rule below so it just follows the toggle normally instead of competing for
   the same space. */
.sw-hamburger{{display:inline-flex;margin-left:auto;}}
.sw-lang-switch:not(.sw-lang-switch--drawer){{margin-left:auto;}}
.sw-lang-switch:not(.sw-lang-switch--drawer) + .sw-hamburger{{margin-left:0;}}
/* No separate icon-only padding override — home.pointsav.com uses the SAME
   padding (4px/12px) whether the label is showing or not; the pill just
   shrinks to fit the glyph alone once the label's `display:none` below
   removes it. A tighter padding override here would be an unmatched guess. */
.sw-lang-switch:not(.sw-lang-switch--drawer) .sw-lang-switch__label{{display:none;}}
/* Second real bug found at the same 850x870 report: this rule was stuck in
   the 768px block below while the hamburger that opens it is reachable up to
   1024px — between 769-1024px the click handler fired correctly (class +
   aria-expanded both flipped, confirmed via automation) but nothing was
   visible, because `.sw-mobile-nav` still defaulted to `display:none` with no
   override active at that width. Must live in the SAME breakpoint as the
   hamburger that controls it. */
.sw-mobile-nav.sw-mobile-nav--open{{display:flex;flex-direction:column;}}
}}
@media (max-width:768px){{
.sw-footer__top{{grid-template-columns:1fr;gap:24px;}}
}}"#,
        topnav = tokens::TOPNAV_BG,
        on_chrome = tokens::ON_CHROME,
        on_chrome_muted = tokens::ON_CHROME_MUTED,
        accent = tokens::ACCENT,
        accent_hover = tokens::ACCENT_HOVER,
        footer_bg = tokens::FOOTER_BG,
        footer_fg = tokens::FOOTER_FG,
        footer_fg_muted = tokens::FOOTER_FG_MUTED,
        footer_div = tokens::FOOTER_DIVIDER,
        ink = tokens::INK,
        wordmark = tokens::WORDMARK,
    );
    html! { style { (PreEscaped(css)) } }
}

// ── Masthead + off-canvas drawer ────────────────────────────────────────────────

/// The navy masthead: a single flat-text wordmark (left) and product search
/// (right) — no icon, no stacked descriptor, no account/language controls, no
/// off-canvas drawer.
///
/// **Redesigned 2026-07-07 (second pass, same day)** — the prior icon + two-line
/// "PointSav / Software" lockup was, byte-for-byte, the same structural pattern as
/// `documentation.pointsav.com`'s own masthead: verified directly against its
/// served HTML, its `<svg>` glyph path is character-for-character identical to
/// this crate's `GLYPH_SVG`, and its lockup is the same "brand name + small-caps
/// descriptor stacked below it" shape. That's the concrete, verified reason this
/// site kept reading as the wiki. `home.pointsav.com`'s real masthead is
/// structurally simpler and distinct from the wiki's: a single flat-text wordmark
/// link, no icon at all (`<a class="m-masthead__wordmark">PointSav Digital
/// Systems</a>`) — checked directly against its served HTML, not assumed. This
/// follows that verified precedent: one flat wordmark naming the site's actual
/// identity, no icon. Also drops Account and the EN/ES language toggle (operator
/// instruction) and, since the wordmark already links to `/` (which redirects to
/// `/software` — see `main.rs`'s `root()`), the separate one-item nav row from the
/// prior pass is now itself redundant with the wordmark and is removed along with
/// the drawer/burger system that existed only to hold it and the now-removed
/// Account link on mobile.
pub fn masthead(surface: SoftwareSurface, lang: Lang, lang_toggle_href: &str) -> Markup {
    let nav = lang.nav_labels();
    html! {
        header."sw-masthead" role="banner" {
            div."sw-masthead__inner" {
                a."sw-wordmark" href=(lang.localize("/")) aria-label=(surface.home_label()) {
                    (surface.home_label())
                }
                // Real inline nav — verified live on home.pointsav.com that its own
                // masthead shows its primary destinations directly (not just via the
                // footer), hidden below 768px and reachable there only through the
                // hamburger drawer, same breakpoint this crate already uses for
                // search/hamburger. Mirrors home's own primary/legal split: only the
                // four commercial pages sit in the masthead nav; Disclaimer/Privacy/
                // Accessibility stay footer + drawer only, matching how home.pointsav.com
                // leaves its own Privacy/Disclaimer out of ITS masthead nav too.
                nav."sw-masthead__nav" aria-label="Primary" {
                    a href=(lang.localize("/software")) { (nav.products) }
                    a href=(lang.localize("/pricing")) { (nav.pricing) }
                    a href=(lang.localize("/licensing")) { (nav.licensing) }
                    a href="/working-papers" { (nav.working_papers) }
                    a href=(lang.localize("/page/contact")) { (nav.contact) }
                }
                div."sw-search" {
                    form."sw-search__form" role="search" action=(lang.localize("/software")) method="get" {
                        label."sw-search__input" style="display:none" for="sw-q" { (nav.search_label) }
                        input."sw-search__input" #"sw-q" type="search" name="q"
                            placeholder=(nav.search_placeholder) autocomplete="off"
                            aria-label=(nav.search_label);
                        button."sw-search__btn" type="submit" aria-label=(nav.search_label) {
                            (PreEscaped(SEARCH_ICON))
                        }
                    }
                }
                // MVL Spanish toggle (operator-approved 2026-07-12) — same globe-glyph
                // + reciprocal-label pattern verified live on home.pointsav.com. Only
                // /software, /pricing, /licensing have a real ES sibling; every other
                // page's toggle falls back to the other language's /software (see
                // `render_page`'s `toggle_href`), same as visiting an untranslated
                // page on the home-* sites falls back to that site's ES/EN root.
                // Placed BEFORE the hamburger (not after) so the hamburger's own
                // `margin-left:auto` (mobile only) is the only auto-margin claimant
                // in this row — two competing auto-margins on adjacent siblings is
                // exactly the mobile-overflow bug home.pointsav.com's own CSS
                // comment documents having to fix after shipping this pattern first.
                // Collapses to icon-only below 768px (its text label hidden via CSS,
                // `chrome_style`'s `.sw-lang-switch__label` mobile rule) — verified
                // live, home.pointsav.com does the same at its own breakpoint. The
                // full-label version reappears inside the drawer below for a bigger,
                // labeled mobile tap target, matching that site's masthead/drawer split.
                (lang_switch(lang, lang_toggle_href, false))
                // Real <button>, not a checkbox+label: natively focusable and
                // Enter/Space-activatable with no extra JS, and can carry
                // aria-expanded/aria-controls correctly. A pure-CSS checkbox
                // version was tried first and found keyboard/AT-inaccessible
                // (WCAG 2.1.1) by this session's browser-in-the-loop re-audit —
                // `display:none` on the checkbox removed it from tab order
                // entirely, and a <label> alone doesn't natively respond to
                // Enter/Space the way a button does.
                button."sw-hamburger" type="button" aria-label=(nav.menu_label)
                    aria-expanded="false" aria-controls="sw-mobile-nav" {
                    (PreEscaped(HAMBURGER_ICON))
                }
            }
            // Same 7 links as the footer's "Site" column — the only masthead-level
            // path to Pricing/Licensing/Contact/Disclaimer/Privacy/Accessibility on
            // mobile, where the footer is otherwise reachable only by scrolling to
            // the bottom of the page. Every page now has a real ES sibling
            // (2026-07-13 full-site-parity pass), so all seven localize.
            nav."sw-mobile-nav" #"sw-mobile-nav" aria-label="Mobile navigation" {
                a href=(lang.localize("/software")) { (nav.products) }
                a href=(lang.localize("/pricing")) { (nav.pricing) }
                a href=(lang.localize("/licensing")) { (nav.licensing) }
                a href="/working-papers" { (nav.working_papers) }
                a href=(lang.localize("/page/contact")) { (nav.contact) }
                a href=(lang.localize("/page/disclaimer")) { (nav.disclaimer) }
                a href=(lang.localize("/page/privacy")) { (nav.privacy) }
                a href=(lang.localize("/page/accessibility")) { (nav.accessibility) }
                // Labeled duplicate of the masthead's icon-only pill — same
                // masthead-collapses/drawer-keeps-label split verified live on
                // home.pointsav.com (`.m-drawer .m-lang-switch`).
                (lang_switch(lang, lang_toggle_href, true))
            }
            (mobile_nav_script())
        }
    }
}

/// Toggles the mobile nav drawer and keeps `aria-expanded` in sync — same
/// self-contained inline-script pattern as `catalog::rail_script()`/
/// `catalog::clipboard_script()`. A real `<button>` already gets Enter/Space
/// activation for free; this only needs to handle the click/keypress result.
fn mobile_nav_script() -> Markup {
    let js = r#"(function(){
var btn=document.querySelector('.sw-hamburger');
var nav=document.getElementById('sw-mobile-nav');
if(!btn||!nav){return;}
btn.addEventListener('click',function(){
  var open=nav.classList.toggle('sw-mobile-nav--open');
  btn.setAttribute('aria-expanded',open?'true':'false');
});
})();"#;
    html! { script { (PreEscaped(js)) } }
}

// ── Footer ──────────────────────────────────────────────────────────────────────

/// Light institutional footer: Site / Network link columns, cities line, meta row,
/// and the legal block (copyright + verbatim WCP Inc. trademark line).
///
/// **Corrected 2026-07-07**: restructured from the ad-hoc "Catalog" / "Legal &
/// Policy" columns to the exact two-column **Site / Network** pattern verified
/// live on `home.pointsav.com` (`m-footer__columns`, `Site` + `Network` nav) —
/// operator-confirmed. Site = this site's own pages; Network = links out to the
/// other PointSav/Woodfine properties. External links carry `target="_blank"`,
/// `rel="noopener"`, an `↗` glyph, and an `"(opens in new tab)"` aria-label
/// suffix, matching `home.pointsav.com`'s exact external-link pattern verbatim.
///
/// Note: the brand lockup reads "PointSav Software" WITHOUT a ™ — that exact string
/// is not one of the enumerated marks in TRADEMARK.md v1.1, so asserting a mark on
/// it would be inaccurate. The enumerated marks carry ™ in the trademark line only.
pub fn footer(surface: SoftwareSurface, lang: Lang) -> Markup {
    let nav = lang.nav_labels();
    html! {
        footer."sw-footer" role="contentinfo" {
            div."sw-footer__inner" {
                div."sw-footer__top" {
                    div."sw-footer__brand" {
                        div."sw-footer__brand-name" { "PointSav Software" }
                        p."sw-footer__tagline" {
                            "Binary distribution and licensing for the PointSav platform."
                        }
                    }
                    div."sw-footer__col" {
                        h2 { (nav.site_col) }
                        ul {
                            li { a href=(lang.localize("/software")) { (nav.products) } }
                            li { a href=(lang.localize("/pricing")) { (nav.pricing) } }
                            li { a href=(lang.localize("/licensing")) { (nav.licensing) } }
                            li { a href="/working-papers" { (nav.working_papers) } }
                            li { a href=(lang.localize("/page/contact")) { (nav.contact) } }
                            li { a href=(lang.localize("/page/disclaimer")) { (nav.disclaimer) } }
                            li { a href=(lang.localize("/page/privacy")) { (nav.privacy) } }
                            li { a href=(lang.localize("/page/accessibility")) { (nav.accessibility) } }
                        }
                    }
                    div."sw-footer__col" {
                        h2 { (nav.network_col) }
                        ul {
                            li {
                                a href="https://home.pointsav.com/" target="_blank" rel="noopener"
                                    aria-label="PointSav Digital Systems (opens in new tab)" {
                                    "PointSav Digital Systems" span."sw-footer__ext" aria-hidden="true" { "\u{2197}" }
                                }
                            }
                            li {
                                a href="https://documentation.pointsav.com/" target="_blank" rel="noopener"
                                    aria-label="Documentation (opens in new tab)" {
                                    (nav.documentation) span."sw-footer__ext" aria-hidden="true" { "\u{2197}" }
                                }
                            }
                            li {
                                a href="https://design.pointsav.com/" target="_blank" rel="noopener"
                                    aria-label="Design System (opens in new tab)" {
                                    (nav.design_system) span."sw-footer__ext" aria-hidden="true" { "\u{2197}" }
                                }
                            }
                            li {
                                a href="https://pointsav.com/" target="_blank" rel="noopener"
                                    aria-label="Newsroom (opens in new tab)" {
                                    (nav.newsroom) span."sw-footer__ext" aria-hidden="true" { "\u{2197}" }
                                }
                            }
                            li {
                                a href="https://home.woodfinegroup.com/" target="_blank" rel="noopener"
                                    aria-label="Woodfine Capital Projects (opens in new tab)" {
                                    "Woodfine Capital Projects" span."sw-footer__ext" aria-hidden="true" { "\u{2197}" }
                                }
                            }
                        }
                    }
                }
                // Collapsed by default — matches the pattern live on the wiki/home
                // sites (`app-mediakit-marketing-2`'s `DisclosureSlot` accordion,
                // operator-directed 2026-07-02): on-page and readable without JS
                // (no hidden-behind-a-link disclosure), but collapsed so it doesn't
                // read as a second copy of the trademark paragraph sitting directly
                // below it. This site's one slot covers what it actually does that
                // the wiki/marketing sites don't: sell software licenses paid for in
                // on-chain USDC.
                details."sw-footer__disclosure" {
                    summary."sw-footer__disclosure-summary" { (nav.important_information) }
                    div."sw-footer__slot" {
                        p."sw-footer__slot-label" { (surface.disclosure_label(lang)) }
                        div."sw-footer__slot-body" { (super::disclosure_body(lang)) }
                    }
                }
                // Persistent one-line disclaimer, always visible even with the
                // accordion above collapsed (the "Apollo Academy" pattern —
                // flagged by project-knowledge, msg-id
                // command-20260702-important-information-footer-structure-w):
                // a collapsed disclosure should never leave the footer reading
                // as if no disclaimer exists at all, e.g. in a cropped screenshot.
                p."sw-footer__persistent-disclaimer" {
                    (nav.persistent_disclaimer_lede)
                    a href=(lang.localize("/page/disclaimer")) { (nav.full_disclaimer_link) } "."
                }
                div."sw-footer__cities" {
                    @for (i, c) in surface.cities().iter().enumerate() {
                        @if i > 0 { span aria-hidden="true" { " | " } }
                        span { (c) }
                    }
                }
                // "Powered by" badge — the family's own attribution-mark pattern
                // (`home.pointsav.com`'s "Powered by MediaKit" / `documentation.
                // pointsav.com`'s equivalent, verified against their served HTML:
                // `<a class="m-badge"><span class="m-badge__glyph">…</span>
                // <span class="m-badge__label">Powered by</span><span
                // class="m-badge__name">…</span></a>`). This site is credited to
                // the engine that actually built it, `PrivateGit`
                // (`app-privategit-source-2` + `app-privategit-marketplace-2`),
                // linking to its public source the same way the Network column's
                // own `Source` link does — no dedicated marketing page exists for
                // it yet, so this reuses the one real, live destination.
                div."sw-footer__badge-row" {
                    a."sw-footer__badge" href="https://github.com/pointsav" target="_blank" rel="noopener"
                        aria-label="Powered by PrivateGit (opens in new tab)" {
                        span."sw-footer__badge-glyph" aria-hidden="true" { (PreEscaped(BADGE_GLYPH)) }
                        span."sw-footer__badge-text" {
                            span."sw-footer__badge-label" { (nav.powered_by) }
                            span."sw-footer__badge-name" { "PrivateGit" }
                        }
                    }
                }
                div."sw-footer__legal" {
                    p."sw-footer__copyright" {
                        "\u{00a9} 2026 " (surface.copyright_holder()) " " (nav.all_rights_reserved)
                    }
                    p."sw-footer__trademark" { (surface.trademark_line(lang)) }
                }
            }
        }
    }
}

// ── Full-document render: dynamic body content + Sovereign chrome ───────────────

/// Render a complete HTML document with the Sovereign Editorial chrome around
/// caller-supplied `content` [`Markup`].
///
/// This is the dynamic counterpart to [`wrap_static_html`]: instead of splicing chrome
/// into a prerendered static file, it builds the whole page — the same masthead, scoped
/// chrome stylesheet, and near-black footer — around content generated at request time
/// (e.g. the dynamic product catalog in `ui::catalog`). The `/software` route uses this
/// so its cards can never drift from `products.yaml`. `/licensing` keeps using
/// [`wrap_static_html`] because it is a static legal document, not catalog data.
/// Canonical public host — used to build `og:url`/`twitter:*`/`rel=canonical`
/// absolute URLs. This is the real public site, not this dev host's `127.0.0.1`
/// bind address (see `CLAUDE.md`'s foundry-prod/foundry-workspace split).
const SITE_URL: &str = "https://software.pointsav.com";

/// Site-default social preview image. Real 1200×630 asset, sourced from
/// `pointsav-media-assets/og-images/ASSET-OG-IMAGE-SOFTWARE.png` (commit `d21ad9e`,
/// Command, 2026-08-02) and vendored byte-identical into `static/og-default.png` —
/// same vendoring convention this crate already uses for fonts, since this repo has
/// no live cross-repo asset-mounting pipeline of its own. Previously a live 404;
/// closes the one blocking SEO defect `BRIEF-software-handoff-readiness.md` flagged.
const OG_IMAGE_PATH: &str = "/static/og-default.png";

/// `translated`: true only for the three MVL Spanish pages (`/software`,
/// `/pricing`, `/licensing`) — controls whether `hreflang` alternate `<link>`s
/// are emitted and whether the masthead's lang-switch pill targets the real ES/EN
/// sibling of `path` (vs. falling back to the other language's `/software`).
pub fn render_page(
    surface: SoftwareSurface,
    lang: Lang,
    title: &str,
    description: &str,
    path: &str,
    translated: bool,
    content: Markup,
) -> Markup {
    let url = format!("{SITE_URL}{path}");
    let image = format!("{SITE_URL}{OG_IMAGE_PATH}");
    let unprefixed = path.strip_prefix(lang.prefix()).unwrap_or(path);
    let toggle_href = if translated {
        lang.other().localize(unprefixed)
    } else {
        lang.other().localize("/software")
    };
    html! {
        (DOCTYPE)
        html lang=(lang.code()) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                link rel="icon" type="image/svg+xml" href="/static/favicon.svg";
                meta name="description" content=(description);
                link rel="canonical" href=(url);
                @if translated {
                    link rel="alternate" hreflang="en" href=(format!("{SITE_URL}{}", Lang::En.localize(unprefixed)));
                    link rel="alternate" hreflang="es" href=(format!("{SITE_URL}{}", Lang::Es.localize(unprefixed)));
                    link rel="alternate" hreflang="x-default" href=(format!("{SITE_URL}{}", Lang::En.localize(unprefixed)));
                }
                meta property="og:type" content="website";
                meta property="og:site_name" content="PointSav Software";
                meta property="og:title" content=(title);
                meta property="og:description" content=(description);
                meta property="og:url" content=(url);
                meta property="og:image" content=(image);
                meta name="twitter:card" content="summary";
                meta name="twitter:title" content=(title);
                meta name="twitter:description" content=(description);
                meta name="twitter:image" content=(image);
                (chrome_style())
            }
            body {
                (masthead(surface, lang, &toggle_href))
                main { (content) }
                (footer(surface, lang))
            }
        }
    }
}

/// Content block for a chromed error page (404/500/etc.) — pass to [`render_page`]
/// so error responses carry the same masthead/footer as every other page instead of
/// a bare `text/plain` body (closes the M2/S-error-schema gap: error paths used to
/// throw the visitor out of the site chrome entirely).
pub fn error_markup(lang: Lang, heading: &str, message: &str) -> Markup {
    let home_label = match lang {
        Lang::En => "Back to the software catalog",
        Lang::Es => "Volver al cat\u{e1}logo de software",
    };
    let home_href = lang.localize("/software");
    html! {
        div."sw-legal" {
            h1 { (heading) }
            p."sw-legal__lede" { (message) }
            p { a href=(home_href) { (home_label) } }
        }
    }
}

// ── Splice: wrap prerendered static HTML with the Sovereign chrome ──────────────

/// Wrap a storefront static page with the Sovereign Editorial chrome.
///
/// Preserves the P1 static-file read (the caller still reads the file from disk);
/// this only rewrites the served bytes: it strips the page's own light `topnav`
/// header and thin footer, injects the scoped chrome stylesheet before `</head>`,
/// mounts the navy masthead immediately after `<body …>`, and the near-black
/// footer immediately before `</body>`.
///
/// Defensive: if the essential anchors are missing the page is served unchanged.
///
/// `path`/`translated` mirror `render_page`'s own params — used only to compute
/// the masthead's lang-switch toggle target (`/licensing` <-> `/es/licensing`).
pub fn wrap_static_html(
    raw: &str,
    surface: SoftwareSurface,
    lang: Lang,
    path: &str,
    translated: bool,
) -> String {
    if !raw.contains("</head>") || !raw.contains("</body>") {
        tracing::warn!("static page missing </head> or </body>; serving without Sovereign chrome");
        return raw.to_string();
    }

    let unprefixed = path.strip_prefix(lang.prefix()).unwrap_or(path);
    let toggle_href = if translated {
        lang.other().localize(unprefixed)
    } else {
        lang.other().localize("/software")
    };

    // 1. Strip the page's own light chrome (single header/footer per file).
    let mut out = remove_between(raw, "<header class=\"topnav\">", "</header>");
    out = remove_between(&out, "<footer>", "</footer>");

    // 2. Inject the scoped chrome stylesheet before </head>.
    let style = chrome_style().into_string();
    out = out.replacen("</head>", &format!("{style}\n</head>"), 1);

    // 3. Mount the masthead (+ drawer) right after the <body …> open tag.
    out = insert_after_body_open(&out, &masthead(surface, lang, &toggle_href).into_string());

    // 4. Mount the footer right before </body>.
    out = out.replacen(
        "</body>",
        &format!("{}\n</body>", footer(surface, lang).into_string()),
        1,
    );

    out
}

/// Remove the first `open …close` span (inclusive). Returns the input unchanged if
/// either delimiter is absent.
fn remove_between(s: &str, open: &str, close: &str) -> String {
    if let Some(i) = s.find(open) {
        if let Some(rel) = s[i..].find(close) {
            let j = i + rel + close.len();
            let mut out = String::with_capacity(s.len() - (j - i));
            out.push_str(&s[..i]);
            out.push_str(&s[j..]);
            return out;
        }
    }
    s.to_string()
}

/// Insert `insert` immediately after the `<body …>` open tag. Falls back to
/// prepending if no `<body>` tag is found.
fn insert_after_body_open(s: &str, insert: &str) -> String {
    if let Some(i) = s.find("<body") {
        if let Some(gt_rel) = s[i..].find('>') {
            let pos = i + gt_rel + 1;
            let mut out = String::with_capacity(s.len() + insert.len());
            out.push_str(&s[..pos]);
            out.push('\n');
            out.push_str(insert);
            out.push_str(&s[pos..]);
            return out;
        }
    }
    format!("{insert}{s}")
}

// ── Tests ─────────────────────────────────────────────────────────────────────
//
// Pure string/markup functions — no server, no filesystem, no network.
#[cfg(test)]
mod tests {
    use super::*;

    const SURFACE: SoftwareSurface = SoftwareSurface::Marketplace;

    #[test]
    fn render_page_wraps_content_in_full_chrome() {
        let page = render_page(
            SURFACE,
            Lang::En,
            "Test Title",
            "Test description",
            "/test",
            false,
            html! { p { "probe content" } },
        )
        .into_string();

        assert!(page.starts_with("<!DOCTYPE html>"));
        assert!(page.contains("<title>Test Title</title>"));
        assert!(page.contains("probe content"));

        // SEO meta/OG/Twitter tags (BRIEF-seo-cross-site-strategy.md gap closure).
        assert!(
            page.contains(r#"<link rel="icon" type="image/svg+xml" href="/static/favicon.svg">"#)
        );
        assert!(page.contains(r#"<meta name="description" content="Test description">"#));
        assert!(
            page.contains(r#"<link rel="canonical" href="https://software.pointsav.com/test">"#)
        );
        assert!(page.contains(r#"<meta property="og:title" content="Test Title">"#));
        assert!(page
            .contains(r#"<meta property="og:url" content="https://software.pointsav.com/test">"#));
        assert!(page.contains(r#"<meta name="twitter:card" content="summary">"#));

        // Masthead markers.
        assert!(page.contains("sw-masthead"));
        assert!(page.contains(SURFACE.home_label()));

        // Wordmark font — corrected 2026-07-13 from a sans fallback (Georgia/Inter
        // stand-ins) to the real, self-hosted Source Serif 4, verified byte-for-byte
        // against home.pointsav.com's own served wordmark font (computed style
        // check, not assumed) — the prior mismatch was visibly "not the same site."
        assert!(page.contains(r#"@font-face{font-family:"Source Serif 4""#));
        assert!(page.contains(r#".sw-wordmark{font-family:"Source Serif 4""#));

        // Footer markers: verbatim trademark line, copyright holder, cities.
        assert!(page.contains(SURFACE.trademark_line(Lang::En)));
        assert!(page.contains(SURFACE.copyright_holder()));
        for c in SURFACE.cities() {
            assert!(page.contains(c), "missing footer city {c}");
        }

        // All six canonical TRADEMARK.md §13 marks present (regression guard,
        // corrected 2026-07-07 — the prior version of this test asserted seven
        // fabricated marks that have never appeared in TRADEMARK.md at any point in
        // its history, introduced by a 2026-07-04 "correction" that itself
        // mis-checked git log. Verified against the current file directly, and
        // against home.pointsav.com's real live trademark line, not assumed.
        for mark in [
            "Woodfine Capital Projects\u{2122}",
            "MCorp\u{2122}",
            "PointSav Digital Systems\u{2122}",
            "Totebox Orchestration\u{2122}",
            "Totebox Archive\u{2122}",
            "Capability Geometry\u{2122}",
        ] {
            assert!(page.contains(mark), "missing canonical mark {mark}");
        }

        // Footer disclosure accordion (operator instruction 2026-07-02, matching the
        // wiki/home sites' "Important information" pattern): present, with the site's
        // one disclosure slot. `<details>` without an `open` attribute renders
        // collapsed by default in every browser — nothing to assert there.
        assert!(page.contains("sw-footer__disclosure"));
        assert!(page.contains("Important information"));
        assert!(page.contains(SURFACE.disclosure_label(Lang::En)));

        // Persistent one-line disclaimer, always visible regardless of accordion state
        // (project-knowledge's "Apollo Academy" pattern, 2026-07-02) -- a collapsed
        // accordion must never leave the footer looking bare of any disclaimer at all.
        assert!(page.contains("sw-footer__persistent-disclaimer"));
        assert!(page.contains("USDC payments on Polygon are irreversible"));

        // Document order: masthead element, then content, then footer element.
        // (Search for the tags, not the class names — the scoped CSS in <head>
        // legitimately contains `.sw-masthead` / `.sw-footer` selector text.)
        let m = page.find("<header").unwrap();
        let c = page.find("probe content").unwrap();
        let f = page.find("<footer").unwrap();
        assert!(m < c && c < f, "chrome must bracket the content");
    }

    #[test]
    fn render_page_translated_emits_hreflang_alternates_and_es_html_lang() {
        let page = render_page(
            SURFACE,
            Lang::Es,
            "Productos \u{2014} PointSav Software",
            "Descripci\u{f3}n de prueba",
            "/es/software",
            true,
            html! { p { "contenido de prueba" } },
        )
        .into_string();

        assert!(page.starts_with("<!DOCTYPE html>"));
        assert!(page.contains(r#"<html lang="es">"#));
        assert!(page.contains(
            r#"<link rel="alternate" hreflang="en" href="https://software.pointsav.com/software">"#
        ));
        assert!(page.contains(r#"<link rel="alternate" hreflang="es" href="https://software.pointsav.com/es/software">"#));
        assert!(page.contains(r#"<link rel="alternate" hreflang="x-default" href="https://software.pointsav.com/software">"#));
        // Toggle in the masthead points back at the real English sibling.
        assert!(page.contains(r#"class="sw-lang-switch" href="/software""#));
        assert!(page.contains("English"));
    }

    #[test]
    fn render_page_untranslated_page_omits_hreflang_and_toggle_falls_back_to_software() {
        let page = render_page(
            SURFACE,
            Lang::En,
            "Contact us \u{2014} PointSav Software",
            "Test description",
            "/page/contact",
            false,
            html! { p { "probe" } },
        )
        .into_string();

        assert!(!page.contains(r#"rel="alternate" hreflang"#));
        // No ES sibling for /page/contact -> toggle falls back to /es/software.
        assert!(page.contains(r#"class="sw-lang-switch" href="/es/software""#));
    }

    #[test]
    fn masthead_mobile_nav_covers_every_page_unreachable_without_it() {
        // Investigation this session confirmed Pricing/Licensing/Contact/Disclaimer/
        // Privacy/Accessibility were unreachable from mobile except by scrolling to
        // the footer (search collapses to nothing at <=768px, masthead had no nav at
        // all). The hamburger drawer must carry the same links as the footer's Site
        // column so nothing is orphaned on mobile.
        let html = masthead(SURFACE, Lang::En, "/es/software").into_string();
        // Real <button>, not a checkbox+label — keyboard/AT-accessible (WCAG 2.1.1
        // fix): browser-in-the-loop re-audit found the earlier checkbox version
        // unreachable by keyboard (`display:none` removes it from tab order).
        assert!(html.contains(r#"class="sw-hamburger""#));
        assert!(html.contains("aria-expanded=\"false\""));
        assert!(html.contains("aria-controls=\"sw-mobile-nav\""));
        assert!(html.contains(r#"class="sw-mobile-nav""#));
        assert!(html.contains(r#"id="sw-mobile-nav""#));
        for href in [
            "/software",
            "/pricing",
            "/licensing",
            "/page/contact",
            "/page/disclaimer",
            "/page/privacy",
            "/page/accessibility",
        ] {
            assert!(
                html.contains(&format!("href=\"{href}\"")),
                "mobile nav missing link to {href}"
            );
        }
    }

    #[test]
    fn masthead_carries_a_real_desktop_nav_matching_home_pointsav_com_pattern() {
        // Live browser-in-the-loop comparison against home.pointsav.com found our
        // masthead had zero visible navigation at desktop width — only reachable
        // via the footer or the mobile drawer. home.pointsav.com's own masthead
        // shows its primary destinations inline at desktop and hides them behind
        // the hamburger only below its breakpoint; this closes the same gap here,
        // reusing the crate's existing 768px breakpoint (chrome_style hides
        // `.sw-masthead__nav` there, alongside search/hamburger).
        let html = masthead(SURFACE, Lang::En, "/es/software").into_string();
        assert!(html.contains(r#"class="sw-masthead__nav""#));
        for href in ["/software", "/pricing", "/licensing", "/page/contact"] {
            assert!(
                html.contains(&format!("href=\"{href}\"")),
                "desktop masthead nav missing link to {href}"
            );
        }
        // Matches home.pointsav.com's own primary/legal split — Disclaimer/Privacy/
        // Accessibility stay footer + drawer only, not in the desktop nav row.
        let nav_start = html.find(r#"class="sw-masthead__nav""#).unwrap();
        let nav_end = html[nav_start..].find("</nav>").unwrap() + nav_start;
        let nav_slice = &html[nav_start..nav_end];
        for absent in ["/page/disclaimer", "/page/privacy", "/page/accessibility"] {
            assert!(
                !nav_slice.contains(absent),
                "desktop nav should not carry legal link {absent}"
            );
        }
    }

    #[test]
    fn footer_site_column_links_privacy_and_accessibility() {
        // Regression guard: an earlier cleanup pass removed a footer meta line that
        // (unintentionally) was Privacy's only link anywhere on the site, and
        // Accessibility had never been linked from the footer at all. Both must be
        // reachable from the footer's Site column independent of the mobile nav.
        let html = footer(SURFACE, Lang::En).into_string();
        assert!(html.contains(r#"href="/page/privacy""#));
        assert!(html.contains(r#"href="/page/accessibility""#));
    }

    #[test]
    fn wrap_static_html_strips_light_chrome_and_mounts_sovereign() {
        let raw = r#"<!doctype html>
<html><head><title>Licensing</title></head>
<body class="page">
<header class="topnav">OLD NAV</header>
<main>Legal body text.</main>
<footer>OLD FOOTER</footer>
</body></html>"#;

        let out = wrap_static_html(raw, SURFACE, Lang::En, "/licensing", true);

        // Original content preserved; old light chrome removed.
        assert!(out.contains("Legal body text."));
        assert!(!out.contains("OLD NAV"));
        assert!(!out.contains("OLD FOOTER"));

        // Sovereign chrome present, including the verbatim trademark line and the
        // "Important information" disclosure accordion (2026-07-02).
        assert!(out.contains("sw-masthead"));
        assert!(out.contains(SURFACE.trademark_line(Lang::En)));
        assert!(out.contains("sw-footer__disclosure"));

        // Scoped chrome stylesheet injected before </head>.
        let style = out.find("--sw-topnav-bg").unwrap();
        assert!(style < out.find("</head>").unwrap());

        // Masthead mounted after the <body …> open tag; footer before </body>.
        let body_open = out.find("<body class=\"page\">").unwrap();
        let masthead = out.find("<header class=\"sw-masthead\"").unwrap();
        assert!(masthead > body_open);
        let footer = out.find("<footer class=\"sw-footer\"").unwrap();
        assert!(footer < out.find("</body>").unwrap());
        assert!(masthead < footer);
    }

    #[test]
    fn wrap_static_html_missing_anchors_serves_page_unchanged() {
        // Defensive path: no </head> or </body> -> bytes served verbatim.
        let fragment = "<p>fragment without head or body</p>";
        assert_eq!(
            wrap_static_html(fragment, SURFACE, Lang::En, "/licensing", true),
            fragment
        );

        let head_only = "<html><head></head><p>no body close</p>";
        assert_eq!(
            wrap_static_html(head_only, SURFACE, Lang::En, "/licensing", true),
            head_only
        );
    }

    #[test]
    fn splice_helpers_edge_cases() {
        // remove_between: inclusive removal of the first span.
        assert_eq!(remove_between("a<x>b</x>c", "<x>", "</x>"), "ac");
        // Missing delimiters -> unchanged.
        assert_eq!(remove_between("no delims", "<x>", "</x>"), "no delims");
        assert_eq!(remove_between("a<x>b", "<x>", "</x>"), "a<x>b");

        // insert_after_body_open: after the open tag, attribute-tolerant.
        assert_eq!(
            insert_after_body_open("<body class=\"p\">rest", "INS"),
            "<body class=\"p\">\nINSrest"
        );
        // Fallback: no <body> tag -> prepended.
        assert_eq!(insert_after_body_open("rest", "INS"), "INSrest");
    }
}
