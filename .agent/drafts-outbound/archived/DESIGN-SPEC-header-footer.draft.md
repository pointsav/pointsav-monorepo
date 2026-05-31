---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-knowledge
target_repo: pointsav-design-system
target_path: research/
target_filename: DESIGN-SPEC-header-footer.md
audience: internal
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-SPEC
authored: 2026-05-23
authored_by: totebox@project-knowledge
authored_with: claude-opus-4-7
---

# DESIGN-SPEC — Shell chrome (header + footer) for app-mediakit-knowledge

## 1. Executive summary

This specification defines a shared `shell_chrome()` component that replaces the
two divergent header implementations currently baked into `home_chrome()` and
`wiki_chrome()` in `src/server.rs`. The shell renders identically on both page
types and adopts the three-row institutional pattern ratified for
`woodfinegroup.com` in wireframe v2c
(`.agent/drafts-outbound/wireframe-woodfinegroup-home.draft.html`), extended
with the wiki-specific chrome (search, auth, theme toggle, instance indicator,
mobile TOC access) the marketing site does not require.

Implementation unlocks:

1. A single Rust function — `shell_header(ctx, scope)` and `shell_footer(ctx)` —
   parameterised by brand, instance, locale, and a `scope` enum
   (`Home`, `Article{has_toc}`, `Special`, `Search`, `History`, `Talk`). One
   render path, one CSS rule set, no further drift.
2. A `--ds-*` global token surface plus brand-local `--wf-*` / `--ps-*`
   overrides. The shell carries no hard-coded colour, type, or spacing values.
3. Convergence between the marketing site (`woodfinegroup.com`) and the
   three wiki instances on a single shell. Future marketing additions (e.g. a
   third row search slot) flip on by feature flag, not by template fork.
4. Removal of the legacy `.mw-header`, `.site-title`, `.site-nav`,
   `.header-search` selectors from `static/style.css` after migration.

This spec also covers — at lower fidelity, sufficient for a follow-up pass —
the category grid (`.wiki-home-grid` / `.ps-home-grid`) and the article chrome
(title row, tabs, breadcrumb, quality badge) improvements that ride alongside.

---

## 2. Current header audit

### 2.1 `home_chrome()` — `src/server.rs` lines 1031–1499

Header markup lives at lines 1106–1177. Structure (in source order):

```
<header.mw-header>
  <a.site-title>           — line 1107  (text logo, no SVG)
  <form.header-search>     — lines 1108–1114
  <div.wiki-appearance>    — lines 1115–1138  (Aa popover; theme + width)
  <nav.site-nav>           — lines 1139–1148
      Home · All pages · Categories · Recent changes · ES · auth_nav_widget
  <div.brand-row>          — lines 1149–1157  (SVG wordmark, centred)
  <nav.nav-row>            — lines 1158–1176  (Disclaimer/Contact · sister links)
</header>
```

The footer is at lines 1463–1494 (`<footer.shell-footer>` with `.footer-row`,
`.cities`, `.footnav`, `.footer-copyright-line`, `.footer-trademark-line`).

### 2.2 `wiki_chrome()` — `src/server.rs` lines 2102–2674

Header markup lives at lines 2174–2276 — plus a sticky-header sibling at
lines 2177–2185, and two off-canvas drawers at lines 2279–2332.

```
<div.wiki-sticky-header>          — lines 2177–2185
  <a.sticky-logo> · <span.sticky-title> · <form.sticky-search>

<header.mw-header>
  <a.site-title>                  — line 2187
  <form.header-search>            — lines 2188–2194
  <div.wiki-appearance>           — lines 2196–2219
  <nav.site-nav>                  — lines 2220–2226
      Home · ES · auth_nav_widget       (note: shorter than home_chrome's nav)
  <div.brand-row>                 — lines 2227–2247
      [toc-toggle-btn] · wordmark · [nav-toggle-btn]
  <nav.nav-row>                   — lines 2248–2266  (same as home)
  <div.search-row>                — lines 2267–2275   (extra search form, dup)
</header>

<nav.mobile-nav-drawer>           — lines 2279–2311
<div.mobile-toc-drawer>           — lines 2313–2331
<div.mobile-nav-overlay>          — line 2332
```

Footer at lines 2634–2666 — structurally identical to home's footer EXCEPT
the footnav adds `<a href="/git/{slug}">View source</a>`.

### 2.3 Structural deltas making the two hard to share

| Element | home_chrome | wiki_chrome | Comment |
|---|---|---|---|
| Sticky header | absent | present | wiki-only; scroll-driven |
| `site-nav` items | 4 nav links + lang + auth | 1 nav link + lang + auth | nav set differs |
| `brand-row` content | wordmark only | toc-toggle + wordmark + nav-toggle | mobile drawer triggers wiki-only |
| `search-row` | absent | present (duplicate of `.header-search` above) | dead duplication |
| Mobile drawer | absent | present | wiki-only |
| Mobile TOC drawer | absent | conditional (only if headings) | wiki-only |
| Footnav | 3 links | 4 links (adds "View source") | scope-dependent |

The duplicate `.header-search` form (line 2188 top, line 2268 bottom) is the
clearest tell that the structure is mid-refactor. Both inputs share the
`id="header-search-q"` — the lower one wins on `getElementById`, the upper one
is unreachable via JS. This is a defect, not a feature.

### 2.4 Token + selector surface today

`.mw-header` is laid out as `display: flex; justify-content: space-between`
on a single row (`static/style.css` line 110–118). The `brand-row` and
`nav-row` children are forced underneath only because they exceed the wrap
threshold — there is no explicit three-row grid. Cosmetic tokens
(`--bg-chrome`, `--border`, `--fg`) are wiki-internal and live in
`static/tokens.css`. The `--ds-*` / `--wf-*` / `--ps-*` token surface from the
woodfinegroup wireframe is not yet wired in.

---

## 3. Shell chrome concept

### 3.1 Three-row structure

```
┌──────────────────────────────────────────────────────────────┐ shell-header
│ Row 1: utility-row   ───────────────────────────────────────  │   --ds-paper-3
│   [instance pill]               EN ▾ │ Aa │ Sign in │ TOC☰    │   --ds-fs-utility
├──────────────────────────────────────────────────────────────┤
│ Row 2: brand-row     ───────────────────────────────────────  │   --ds-paper-2
│                       ◆ WORDMARK / SIGNET ◆                   │
├──────────────────────────────────────────────────────────────┤
│ Row 3: nav-row       ───────────────────────────────────────  │   --ds-paper-2
│   Disclaimer · Contact   │   Search ___________   🔍          │   --ds-fs-nav
│                          │   Corporate · Projects · Newsroom  │
└──────────────────────────────────────────────────────────────┘
```

The shell is identical on every page. What varies is *content within each
row*, supplied by the caller. No row appears or disappears between page
types — only the entries inside a row change.

### 3.2 Component signature (Rust / maud)

```rust
pub struct ShellContext<'a> {
    pub locale: Locale,
    pub site_title: &'a str,
    pub instance: WikiInstance,        // Documentation | Projects | Corporate
    pub brand: Brand,                  // PointSav | Woodfine
    pub user: Option<&'a User>,
    pub pending_count: i64,
    pub home_url: &'a str,             // "/" or "/es/"
    pub lang_alt_url: &'a str,         // for the EN↔ES toggle
}

pub enum ShellScope<'a> {
    Home,
    Article { slug: &'a str, has_toc: bool },
    Special { name: &'a str },
    Search,
    History { slug: &'a str },
    Talk    { slug: &'a str },
}

fn shell_header(ctx: &ShellContext, scope: &ShellScope) -> Markup;
fn shell_footer(ctx: &ShellContext, scope: &ShellScope) -> Markup;
```

### 3.3 Brand + instance parameterisation

Brand controls type stack, paper colour, ink colour, and accent
(`--wf-blue` vs `--ps-steel`). Instance controls only the *instance-indicator
pill* in row 1 and the right-side nav-row link set. The wordmark SVG is
swapped at render time (PointSav signet vs Woodfine wordmark) but the slot
geometry (`.wordmark`, fixed aspect ratio) is identical.

| Slot | PointSav | Woodfine (projects/corporate) |
|---|---|---|
| Type — display | system (-apple-system, …) | Oswald |
| Type — body / sans | system | Nunito Sans |
| Type — serif (footer cities) | (n/a — no cities line) | Roboto Slab |
| Canvas (--ds-paper-2) | #09090B (dark) | #F7F9FA (light) |
| Ink (--ds-ink-1) | #E4E4E7 | #111827 |
| Accent | #869FB9 steel blue | #164679 woodfine blue |
| Wordmark slot | ps-signet_V1.svg (1:1, 80×80) | ASSET-WORDMARK-WOODFINE.svg (4:1, 320×80) |
| Cities line in footer | hidden | "Vancouver | New York" |
| Instance pill | "ENGINEERING DOCUMENTATION" | "PROJECTS WIKI" / "CORPORATE WIKI" |

The shell DOM does not branch on brand — only token values and the wordmark
inner-HTML differ.

---

## 4. HTML/CSS pseudocode — Woodfine variant

This is the canonical form. The PointSav variant in §5 is presented as token
overrides only.

```html
<header class="shell-header" role="banner" data-brand="woodfine"
                             data-instance="projects" data-scope="home">

  <!-- ── Row 1: utility ────────────────────────────────────────── -->
  <div class="shell-header__utility utility-row">
    <!-- left cluster: instance indicator (wiki-only — absent on marketing) -->
    <span class="utility-row__instance-pill"
          aria-label="Instance: Projects Wiki">
      Projects Wiki
    </span>

    <!-- right cluster: utility controls; collapses to icon-only at <768px -->
    <div class="utility-row__controls">
      <!-- Language toggle (EN ⇄ ES) -->
      <a class="utility-row__lang-toggle lang-toggle"
         href="{{ lang_alt_url }}"
         aria-label="Switch language to Español"
         rel="alternate" hreflang="es">EN</a>

      <span class="utility-row__sep" aria-hidden="true">|</span>

      <!-- Appearance — theme + width popover -->
      <button class="utility-row__appearance wiki-appearance-btn"
              aria-expanded="false" aria-controls="wiki-appearance-menu"
              title="Appearance">Aa</button>

      <span class="utility-row__sep" aria-hidden="true">|</span>

      <!-- Auth widget (Sign in / Profile / pending review badge) -->
      {{ auth_nav_widget }}

      <!-- Mobile-only contents (TOC) and menu hamburger.
           CSS hides at >=768px; visible at <768px to replace the brand-row
           toggles that ride in Row 2 today. -->
      <button class="utility-row__toc-mobile toc-toggle-btn"
              aria-label="Contents"
              aria-controls="mobile-toc-drawer">§</button>
      <button class="utility-row__nav-mobile nav-toggle-btn"
              aria-label="Menu"
              aria-controls="mobile-nav-drawer">≡</button>
    </div>
  </div>

  <!-- ── Row 2: brand ──────────────────────────────────────────── -->
  <div class="shell-header__brand brand-row">
    <a class="brand-row__wordmark wordmark" href="{{ home_url }}"
       aria-label="{{ site_title }} — home">
      <!-- SVG inline; viewBox + role="img" + aria-label preserved -->
      {{ WORDMARK_WOODFINE_SVG }}
      <!-- Fallback text inside <svg><title>; visible if SVG fails -->
    </a>
  </div>

  <!-- ── Row 3: nav + search ───────────────────────────────────── -->
  <nav class="shell-header__nav nav-row" role="navigation"
       aria-label="Site navigation">

    <ul class="nav-row__list nav-list left">
      <li><a href="/wiki/disclaimers">Disclaimer</a></li>
      <li><a href="/wiki/contact">Contact</a></li>
    </ul>

    <!-- Centre slot — search bar -->
    <form class="nav-row__search header-search"
          action="/search" method="get"
          role="search" aria-label="Search the wiki">
      <div class="header-search-wrap">
        <input class="nav-row__search-input"
               type="search" name="q"
               placeholder="Search articles…"
               autocomplete="off"
               aria-label="Search">
        <div id="search-autocomplete-dropdown" role="listbox" hidden></div>
      </div>
      <button class="nav-row__search-submit" type="submit"
              aria-label="Search">🔍</button>
    </form>

    <ul class="nav-row__list nav-list right">
      <!-- Woodfine-projects instance set -->
      <li><a href="https://corporate.woodfinegroup.com">Corporate</a></li>
      <li><a href="/wiki/newsroom">Newsroom</a></li>
    </ul>
  </nav>
</header>
```

Skeleton CSS — token-only; no hard-coded values:

```css
/* ── Shell header — three-row pattern ─────────────────────────── */
.shell-header { display: flex; flex-direction: column;
                background: var(--ds-paper-2); }

/* Row 1 — utility */
.shell-header__utility {
  background: var(--ds-paper-3);
  color: var(--ds-ink-3);
  padding: var(--ds-sp-2) var(--ds-sp-9);
  display: grid;
  grid-template-columns: 1fr auto;     /* instance pill | controls */
  align-items: center;
  column-gap: var(--ds-sp-6);
  font-family: var(--ds-display);
  font-size: var(--ds-fs-utility);
  font-weight: var(--ds-fw-medium);
  letter-spacing: var(--ds-ls-extra-wide);
  text-transform: uppercase;
}
.utility-row__controls {
  display: flex; align-items: center;
  gap: var(--ds-sp-3); justify-content: flex-end;
}
.utility-row__instance-pill {
  padding: 2px var(--ds-sp-2);
  border: 1px solid var(--ds-rule-strong);
  border-radius: 2px;
  color: var(--ds-ink-2);
  /* token: --shell-instance-pill-border */
}
.utility-row__sep { color: var(--ds-ink-4); opacity: 0.5; }
.utility-row__lang-toggle::after { content: " \25BE"; margin-left: 4px; }

/* Row 2 — brand */
.shell-header__brand {
  background: var(--ds-paper-2);
  padding: var(--ds-sp-7) var(--ds-sp-9);
  display: flex; justify-content: center; align-items: center;
}
.brand-row__wordmark { display: inline-flex; align-items: center; }
.brand-row__wordmark svg { width: 320px; height: 80px; display: block; }

/* Row 3 — nav + search */
.shell-header__nav {
  background: var(--ds-paper-2);
  border-top:    1px solid var(--ds-rule-hairline);
  border-bottom: 1px solid var(--ds-rule-hairline);
  padding: var(--ds-sp-3) var(--ds-sp-9);
  display: grid;
  grid-template-columns: 1fr minmax(280px, 360px) 1fr;
  align-items: center;
  column-gap: var(--ds-sp-4);
  font-family: var(--ds-display);
  font-size: var(--ds-fs-nav);
  font-weight: var(--ds-fw-medium);
  letter-spacing: var(--ds-ls-wide);
  text-transform: uppercase;
}
.nav-row__list { display: flex; gap: var(--ds-sp-7);
                 list-style: none; margin: 0; padding: 0; }
.nav-row__list.left  { justify-content: flex-start; color: var(--ds-ink-3); }
.nav-row__list.right { justify-content: flex-end;   color: var(--wf-blue); }
.nav-row__list a { display: inline-flex; align-items: center;
                   height: 32px; padding: 0 var(--ds-sp-1); }
.nav-row__list a:hover { opacity: 0.7; }

.nav-row__search { display: flex; align-items: center;
                   gap: var(--ds-sp-2); width: 100%; }
.nav-row__search-input {
  width: 100%; height: 32px;
  background: var(--ds-paper-1);
  border: 1px solid var(--ds-rule-hairline);
  border-radius: 2px;
  padding: 0 var(--ds-sp-3);
  font: inherit;
  text-transform: none;
  letter-spacing: 0;
  color: var(--ds-ink-1);
}
.nav-row__search-input:focus-visible {
  outline: 2px solid var(--wf-blue); outline-offset: 1px;
}

/* Mobile-only triggers in Row 1; hidden on desktop */
.utility-row__toc-mobile,
.utility-row__nav-mobile { display: none; }
```

---

## 5. PointSav variant — token overrides only

The PointSav variant is the SAME HTML. Only the brand-token block changes
(loaded from `tokens-pointsav.css`):

```css
:root[data-brand="pointsav"] {
  /* Paper inversion — dark canvas */
  --ds-paper-1: #18181B;   /* surface above canvas */
  --ds-paper-2: #09090B;   /* canvas */
  --ds-paper-3: #27272A;   /* utility row */

  /* Ink inversion */
  --ds-ink-1: #E4E4E7;
  --ds-ink-2: #A1A1AA;
  --ds-ink-3: #71717A;
  --ds-ink-4: #52525B;

  /* Rule */
  --ds-rule-hairline: #27272A;
  --ds-rule-strong:   #3F3F46;

  /* Type — system font stack (no Oswald) */
  --ds-display: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
                "Helvetica Neue", Arial, sans-serif;
  --ds-sans:    -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
                "Helvetica Neue", Arial, sans-serif;
  --ds-serif:   /* unused on PointSav footer — no cities line */
                "Iowan Old Style", Georgia, serif;
}

/* Brand-local PointSav accent (replaces --wf-blue) */
:root[data-brand="pointsav"] {
  --ps-steel:      #869FB9;
  --ps-steel-on:   #09090B;
  --ps-steel-tint: #1A2433;
}

/* Brand-local override: the nav-list.right + focus-ring colour rebind */
:root[data-brand="pointsav"] .nav-row__list.right { color: var(--ps-steel); }
:root[data-brand="pointsav"] .nav-row__search-input:focus-visible,
:root[data-brand="pointsav"] a:focus-visible,
:root[data-brand="pointsav"] button:focus-visible {
  outline-color: var(--ps-steel);
}

/* Wordmark slot — square signet rather than 4:1 wordmark */
:root[data-brand="pointsav"] .brand-row__wordmark svg {
  width: 80px; height: 80px;
}
```

PointSav-specific footer copy (cities line absent, CC-BY license clause
visible) is driven by `data-brand="pointsav"` selectors in CSS plus a
conditional in the maud template — same as the brand-token swap. See §7.

---

## 6. Row-by-row spec

### 6.1 Utility row (Row 1)

**Purpose:** persistent low-contrast strip carrying instance identity (left)
and the "operate the page" controls (right).

| Slot | Order (LTR) | Visibility | Notes |
|---|---|---|---|
| Instance pill | 1 (left) | wiki only; absent on marketing | Reads "Projects Wiki", "Corporate Wiki", "Engineering Documentation". Bordered, uppercase, --ds-fs-utility. |
| Lang toggle (EN↔ES) | 2 (right cluster start) | all pages | `<a>` anchor, not `<button>` — must work without JS; carries `hreflang` |
| Appearance (Aa) popover | 3 | all pages | Theme (Automatic / Light / Dark) + Width (Standard / Wide). Same DOM as today. |
| Auth widget | 4 | all pages | Sign in / username / pending count badge. From `auth_nav_widget()`. |
| Mobile TOC trigger | 5 | scope=Article AND vw<768px | Hidden by `display:none` above 768px; replaces today's `.toc-toggle-btn` in `.brand-row` |
| Mobile nav trigger | 6 | vw<768px | Replaces today's `.nav-toggle-btn` |

**Responsive behaviour:**

- 1200px and below: padding shrinks from `--ds-sp-9` → `--ds-sp-7`.
- 1024px: appearance popover label "Aa" remains; lang toggle stays full.
- 768px: instance pill moves below; controls reflow centred; mobile TOC + nav
  buttons appear.
- 480px: separators hide; controls keep 4px gap; font-size drops to 9px.

**Sticky behaviour:** the utility row participates in the sticky-header
collapse — when scrollY > 120px, rows 1+2+3 collapse into a single compressed
sticky bar that surfaces: site title (left, was wordmark), search (centre),
appearance + auth (right). The current `.wiki-sticky-header` (server.rs
2177–2185) is the seed; the spec promotes it to a derived view of the same
shell header, not a separate DOM tree.

### 6.2 Brand row (Row 2)

**Purpose:** brand identity. Single asset, centred, with generous padding.

| Property | Woodfine | PointSav |
|---|---|---|
| Asset | `ASSET-WORDMARK-WOODFINE.svg` | `ps-signet_V1.svg` (square signet) |
| Aspect ratio | 4:1 (320×80) | 1:1 (80×80) |
| Padding (desktop) | `--ds-sp-7` vertical / `--ds-sp-9` horizontal | identical |
| Padding (1024px) | `--ds-sp-6` vertical | identical |
| Padding (768px) | `--ds-sp-4` vertical | identical |
| Padding (480px) | `--ds-sp-3` vertical | identical |
| Width (desktop) | 320px | 80px |
| Width (1024px) | 240px | 64px |
| Width (768px) | 200px | 56px |
| Width (480px) | 160px | 48px |
| Fallback text | `<title>Woodfine Capital Projects</title>` inside `<svg>` | `<title>PointSav</title>` |
| Link target | `home_url` from `ShellContext` | identical |
| ARIA label | "{{site_title}} — home" | identical |

The slot height is fixed at 80px on desktop regardless of asset aspect ratio
— SVG width auto-scales inside the constraint. This keeps Row 2 height
identical across brands so any cross-brand A/B at the same instance lines up
to the pixel.

### 6.3 Nav + search row (Row 3)

**Purpose:** primary navigation and the dominant search affordance.

**Layout:** 3-column grid — `1fr minmax(280px, 360px) 1fr`. Left list, search
slot centre, right list. This differs from the marketing wireframe (which has
no search slot) but is structurally compatible: when search is hidden via
`display:none` in the marketing context, the centre track collapses to 0 and
the divider element fills it.

| Slot | Items (wiki) | Items (marketing) |
|---|---|---|
| Left list | Disclaimer · Contact | Disclaimer · Contact |
| Centre | Search input + submit | (collapsed) `<span.nav-divider>` |
| Right list — PointSav | pointsav.com · GitHub | n/a |
| Right list — Woodfine projects | Corporate · Newsroom | Corporate · Projects · Newsroom |
| Right list — Woodfine corporate | Projects · Newsroom | (same as above) |

**Search:**

- One `<form action="/search">` only — the duplicate at `server.rs:2267` is
  deleted by this spec.
- Input gets `--ds-paper-1` background to read as a recessed slot against the
  `--ds-paper-2` row.
- Submit button is icon-only on desktop (🔍 + visually-hidden label), text on
  mobile when the form spans full width.
- Autocomplete dropdown reuses today's `#search-autocomplete-dropdown` div;
  positions absolute against `.header-search-wrap`.

**Auth + appearance toggles do NOT live in Row 3.** They migrate to Row 1.
This removes the `.site-nav` element entirely.

---

## 7. Footer spec (both brands)

The footer is **structurally identical** to the marketing-site wireframe v2c,
extended with two wiki-only links (Sitemap, View source). All copy is fixed
strings; the only variable bits are: cities (Woodfine only), copyright line
(brand-conditional), forward-looking clause (Woodfine only).

```html
<footer class="shell-footer" role="contentinfo" data-brand="{{brand}}">

  <div class="shell-footer__row footer-row">
    {% if brand == "woodfine" %}
      <div class="shell-footer__cities cities">
        Vancouver <span class="cities__sep">|</span> New York
      </div>
    {% endif %}

    <nav class="shell-footer__nav footnav" aria-label="Footer navigation">
      <a href="/wiki/disclaimers">Disclaimer</a>
      <a href="/wiki/privacy">Privacy</a>
      <a href="/wiki/contact">Contact</a>
      <a href="/sitemap.xml">Sitemap</a>
      {% if scope == Article %}
        <a href="/git/{{slug}}">View source</a>
      {% endif %}
      <a href="/feed.atom">Subscribe</a>
    </nav>
  </div>

  <p class="shell-footer__copyright copyright">
    {% if brand == "woodfine" %}
      © 2026 Woodfine Capital Projects Inc. All rights reserved.
    {% else %}
      © 2026 PointSav Digital Systems.
      Content licensed under
      <a href="https://creativecommons.org/licenses/by/4.0/">CC BY 4.0</a>.
      Engine: <a href="https://github.com/pointsav/app-mediakit-knowledge">app-mediakit-knowledge</a>.
    {% endif %}
  </p>

  <p class="shell-footer__attribution attribution">
    Responsible for content: PointSav Digital Systems, a division of
    Woodfine Capital Projects Inc.
    {% if brand == "woodfine" %}
      Woodfine Management Corp. is a wholly owned subsidiary of Woodfine
      Capital Projects Inc.
    {% endif %}
  </p>

  <p class="shell-footer__trademark trademark">
    Woodfine Capital Projects™, Woodfine Management Corp™,
    PointSav Digital Systems™, Foundry™, Totebox Orchestration™,
    Totebox Archive™, ToteboxOS™, ConsoleOS™, OrchestrationOS™, and
    WorkplaceOS™ are unregistered trademarks of Woodfine Capital Projects
    Inc. used in Canada, the United States, Latin America, and Europe.
    All other trademarks are the property of their respective owners.
    {% if brand == "woodfine" %}
      This knowledge base may contain forward-looking statements within the
      meaning of NI 51-102 / OSC SN 51-721. Such statements are subject to
      material assumptions and risks.
    {% endif %}
  </p>

</footer>
```

**Section order (top to bottom):**

1. `.shell-footer__row` — cities (Woodfine only) + footnav. Two-column grid
   on desktop; single column at <768px.
2. `.shell-footer__copyright` — single line, `--ds-fs-copyright` (10px),
   `--ds-ink-4`.
3. `.shell-footer__attribution` — *new section in this spec.* Names the
   entity legally responsible for content. Required for BCSC continuous-
   disclosure posture. `--ds-fs-trademark` (10px), `--ds-ink-3`.
4. `.shell-footer__trademark` — wireframe v2c "variant C" treatment: smallest
   text (9px), `--ds-ink-4`, very generous vertical breathing room before the
   page ends. Forward-looking statement appended on Woodfine instances only.

**Copy that is *not* in today's wiki footer and is added by this spec:**

- Sitemap link (today only on home — gain article-page sitemap link).
- Privacy link (today: absent from footer entirely).
- Feed link in footer (today: only on `/wiki/about`).
- Attribution paragraph (today: absent).
- Forward-looking clause now its own visual unit (today: tucked into the
  trademark blob).

**Copy intentionally NOT in the footer:**

- "Powered by app-mediakit-knowledge" — the engine is named once in the
  PointSav copyright block (because content is CC-BY there), and is implicit
  on Woodfine instances.
- Build SHA / version — moves to `/special/statistics` per BP5 disclosure
  hygiene.

---

## 8. Category-grid improvement spec

### 8.1 Current state

`server.rs:1310–1341` renders `<div.wiki-home-grid>` with a section per
RATIFIED_CATEGORIES entry. Each section is:

```
<div.wiki-home-cat-section>
  <div.wiki-home-cat-section-head>
    <h2><a>{{ category name }}</a></h2>
    <span.wiki-home-cat-section-count>{{ N }} articles</span>
    <a.wiki-home-cat-section-all>All {{ N }} →</a>
  </div>
  {% if count == 0 %}
    <p.wiki-home-cat-in-prep>In preparation.</p>
  {% else %}
    <p.wiki-home-cat-desc>{{ description }}</p>
  {% endif %}
</div>
```

Notable gaps: no icon slot; description is only shown when count > 0 (so
empty categories give the operator nothing to scan against); no visual badges
distinguishing "ratified", "in preparation", or "deprecated" categories; no
preview of top-3 child articles — the design-system `ps-home-grid` recipe
already specifies this and the current implementation does not honour it.

### 8.2 Proposed structure

```html
<section class="ps-home-grid" aria-label="Browse by category">
  <h2 class="ps-home-grid__heading">Browse by area</h2>
  <div class="ps-home-grid__row">

    {% for cat in RATIFIED_CATEGORIES %}
      <article class="ps-home-grid__card
                      ps-home-grid__card--{{ cat.state }}">

        <!-- (1) NEW — icon slot, 24×24, monoline -->
        <span class="ps-home-grid__icon" aria-hidden="true">
          {{ ICON_FOR_CATEGORY[cat] }}    {# inline SVG, currentColor #}
        </span>

        <!-- (2) Title (existing) -->
        <h3 class="ps-home-grid__title">
          <a href="/category/{{ cat.slug }}">{{ cat.label }}</a>
        </h3>

        <!-- (3) NEW — state badge ("In preparation" / "Deprecated") -->
        {% if cat.state != "ratified" %}
          <span class="ps-home-grid__badge
                       ps-home-grid__badge--{{ cat.state }}">
            {{ cat.state | humanise }}
          </span>
        {% endif %}

        <!-- (4) Count + description always (description never hidden) -->
        <p class="ps-home-grid__count">{{ count }} articles</p>
        <p class="ps-home-grid__desc">{{ cat.description }}</p>

        <!-- (5) Existing — top-3 child preview (HONOUR the recipe) -->
        {% if count > 0 %}
          <ul class="ps-home-grid__list">
            {% for t in cat.topics | take(3) %}
              <li><a href="/wiki/{{ t.slug }}">{{ t.title }}</a></li>
            {% endfor %}
          </ul>
          {% if count > 3 %}
            <a class="ps-home-grid__more" href="/category/{{ cat.slug }}">
              More ({{ count - 3 }}) →
            </a>
          {% endif %}
        {% endif %}

      </article>
    {% endfor %}

  </div>
</section>
```

### 8.3 What lifts the grade most

In priority order:

1. **Honour the recipe** — the existing `ps-home-grid` already specifies
   top-3 child links. Current implementation ships an empty card body. This
   is the single biggest improvement and the cheapest.
2. **Icon slot** — 12 categories without icons read as a wall of text. A 24px
   monoline icon at top-left of the card (above title) gives the eye an
   anchor. Reuse `pointsav-design-system/icons/` set; one per category.
3. **State badges** — currently "In preparation" is shown as a paragraph
   *replacing* the description. Promote it to a corner badge so the
   description stays visible too.
4. **Description always visible** — today the description is suppressed when
   count == 0. Reverse this: when count == 0, the description is the most
   important thing on the card.
5. **Hover state** — recipe.css line 75 already specifies a border-colour
   transition; the wiki currently has no hover state on category cards.

Token additions:

```css
--ps-home-grid-icon-size: 24px;
--ps-home-grid-icon-color: var(--ds-ink-3);
--ps-home-grid-badge-bg-prep: var(--ds-paper-3);
--ps-home-grid-badge-fg-prep: var(--ds-ink-3);
--ps-home-grid-badge-bg-deprecated: /* token: --shell-warning-tint */ #fff4e5;
```

---

## 9. Article chrome improvement spec

Targets `wiki-title-row` block at `server.rs:2388–2468`.

### 9.1 Current shape

```
┌────────────────────────────────────────────────────────────────────┐
│ [Article] [Talk]                              [Read] [History] [▾] │
│ ─────────────────────────────────────────────────────────────────  │
│ Title goes here   [quality-badge]                                  │
│ 🌐 Español   Français                                              │
│ From PointSav Wiki                                                 │
│ Short description in italic                                        │
└────────────────────────────────────────────────────────────────────┘
```

Current issues:

- **Quality badge** crammed inline with the H1 — small, easy to miss,
  conflicts with H1 line-height. Suggests reading it as part of the title.
- **No breadcrumb** — readers can't tell at-a-glance which category an
  article belongs to until they reach the footer category list.
- **Tabs split top-left / top-right** — Wikipedia Vector-2022 does this but
  it produces a wide horizontal scan; on narrow viewports the H1 wraps under
  the right-side tabs at unpredictable widths.
- **`wiki-tagline`** ("From PointSav Wiki") is redundant with the brand
  wordmark above and the footer. Remove.
- **Short description** is italic + paragraph styling — reads as body copy,
  not as a subhead.

### 9.2 Proposed shape

```
┌────────────────────────────────────────────────────────────────────┐
│ Architecture › Three-layer stack                          [Quality]│  ← breadcrumb-row
│ ─────────────────────────────────────────────────────────────────  │
│ Three-layer stack                                                  │  ← title-row
│ A concise short description, larger weight than body, not italic.  │  ← subtitle
│ ─────────────────────────────────────────────────────────────────  │
│ [Article] [Talk]                              [Read] [History] [▾] │  ← tab-row
└────────────────────────────────────────────────────────────────────┘
```

Markup:

```html
<header class="wiki-article-header">

  <!-- Breadcrumb row (NEW) -->
  <nav class="wiki-article-header__breadcrumb wiki-breadcrumb"
       aria-label="Breadcrumb">
    <ol>
      <li><a href="/">Home</a></li>
      {% for c in fm.categories %}
        <li><a href="/category/{{ c | lower }}">{{ c | humanise }}</a></li>
      {% endfor %}
      <li aria-current="page">{{ title }}</li>
    </ol>

    {# Quality badge promoted out of H1, into breadcrumb-row right gutter #}
    {% if fm.quality %}
      <span class="quality-badge quality-{{ fm.quality }}"
            aria-label="Article quality: {{ fm.quality | quality_humanise }}">
        {{ fm.quality | upper }}
      </span>
    {% endif %}
  </nav>

  <!-- Title row -->
  <div class="wiki-article-header__title-row wiki-title-row">
    <h1 class="page-title">{{ title }}</h1>

    {% if fm.short_description %}
      <p class="wiki-article-header__subtitle topic-short-description">
        {{ fm.short_description }}
      </p>
    {% endif %}

    {% if fm.translations %}
      <div class="wiki-article-header__langs wiki-lang-switcher">
        <span class="wiki-lang-globe" aria-hidden="true"></span>
        {% for (lang, slug) in fm.translations %}
          <a class="wiki-lang-btn" lang="{{lang}}" hreflang="{{lang}}"
             href="/wiki/{{slug}}">{{ lang_label(lang) }}</a>
        {% endfor %}
      </div>
    {% endif %}
  </div>

  <!-- Tab row -->
  <div class="wiki-article-header__tabs wiki-tab-row">
    <nav class="wiki-page-tabs" aria-label="Page tabs">
      <a class="wiki-tab wiki-tab-active" href="/wiki/{{slug}}"
         aria-current="page">Article</a>
      <a class="wiki-tab" href="/talk/{{slug}}" accesskey="t">Talk</a>
    </nav>

    <nav id="p-views" aria-label="Page actions">
      <a class="wiki-tab wiki-tab-active" href="/wiki/{{slug}}"
         aria-current="page" accesskey="r">Read</a>
      <a class="wiki-tab" href="/history/{{slug}}" accesskey="h">View history</a>
    </nav>

    <details id="p-cactions-details" class="wiki-cactions">
      <summary class="wiki-cactions-toggle" title="More actions">▾</summary>
      <ul class="wiki-cactions-menu">
        <li><a href="/wiki/{{slug}}?printable=yes">Print / Export</a></li>
        <li><a href="/special/pageinfo/{{slug}}">Page information</a></li>
        <li><a href="/special/cite/{{slug}}">Cite this page</a></li>
        <li><a href="/git/{{slug}}">Download as Markdown</a></li>
      </ul>
    </details>
  </div>

</header>
```

### 9.3 What lifts the grade most

1. **Breadcrumb row** — the single highest-impact change for navigation
   confidence. Readers know where they are without scrolling. Quality badge
   migrates here too — keep it visible but stop fighting the H1.
2. **Quality badge out of H1** — visually decouple. Right-aligned in the
   breadcrumb row, same height as the breadcrumb text. Treatment matches
   Wikipedia's "good article" star placement.
3. **Subtitle styling** — drop italic + `<p>` body styling. Use a heavier
   weight (500) at `--ds-fs-body * 1.1`, no italic, normal colour, sits
   directly under H1 with tight line-height. Reads as a subhead.
4. **Drop the tagline** — "From PointSav Wiki" repeats the wordmark and
   delivers zero information beyond it.
5. **Tab row consolidated** — Article/Talk and Read/History/More on the same
   row, below the title rather than flanking it. This stops the wrap-collision
   pathology at 900–1100px viewports.

Token additions:

```css
--wiki-breadcrumb-fs: var(--ds-fs-utility);
--wiki-breadcrumb-color: var(--ds-ink-3);
--wiki-breadcrumb-current-color: var(--ds-ink-1);
--wiki-breadcrumb-sep: "›";
--wiki-subtitle-fs: 1rem;        /* token: --shell-subtitle-size */
--wiki-subtitle-weight: 500;
--wiki-subtitle-color: var(--ds-ink-2);
```

---

## 10. Responsive behaviour

Breakpoints align with the marketing wireframe (1200 / 1024 / 768 / 480px)
to keep marketing-site and wiki-site collapse points pixel-identical.

### 10.1 ≥ 1201px — desktop default

All three rows visible. Mobile triggers (toc, nav) hidden by `display:none`.
Search input at 360px max. Sticky header inactive until scroll.

### 10.2 1200px

Horizontal padding compresses from `--ds-sp-9` (56px) → `--ds-sp-7` (36px) on
all three rows and footer. Wordmark unchanged. No other structural change.

### 10.3 1024px

- Wordmark: 320×80 → 240×60 (Woodfine) / 80×80 → 64×64 (PointSav)
- Brand row vertical padding: `--ds-sp-7` → `--ds-sp-6`
- Nav row gap: `--ds-sp-7` → `--ds-sp-5` between list items
- Search input: max 320px
- Tab row in article chrome wraps to 2 lines if both Article/Talk and
  Read/History/More can't fit; right-side moves under left-side, both keep
  their alignment.

### 10.4 768px

This is the structural break. The nav row reflows; the utility row gains
mobile triggers.

- **Utility row:** `grid-template-columns: 1fr` (single col). Instance pill
  on its own row above the controls. Controls keep right alignment but TOC
  + Menu icon buttons become visible.
- **Brand row:** wordmark 200×50 / 56×56; vertical padding `--ds-sp-4`. The
  `.brand-row` no longer carries the TOC + menu toggles — those moved to the
  utility row.
- **Nav row:** `grid-template-columns: 1fr; grid-template-rows: auto auto
  auto;` — left list, search, right list stack vertically; centred. Search
  becomes full-width. Submit button reverts to text label "Search".
- **Article header:** breadcrumb truncates middle entries with `…`. Tab row
  becomes horizontally scrollable rather than wrapping (preserves tap
  targets).
- **Category grid:** 3-col → 2-col at 960px (per recipe.css), 2-col → 1-col
  at 640px.
- **Footer:** `footer-row` collapses to 1 column. Cities + footnav stack.
  Trademark paragraph remains at 9px — readable on small viewports because
  line-height is `1.5`.

### 10.5 480px

- Utility row: padding `--ds-sp-2 --ds-sp-3`. Separators (`|`) hide.
  Font-size 9px. Lang-toggle, appearance, auth all stay; mobile triggers
  visible.
- Brand row: padding `--ds-sp-3 --ds-sp-3`. Wordmark 160×40 / 48×48.
- Nav row: padding `--ds-sp-3`. Left + right lists each become two-item
  rows at 9px. Search input keeps full width with `font-size: 16px` on the
  input (prevents iOS zoom-on-focus).
- Footer: trademark paragraph stays at 9px; margin tightens. Copyright at
  9px.

### 10.6 Sticky header

When `window.scrollY > 120` and viewport ≥ 1024px:

- Rows 1+2+3 collapse to a single 48px bar.
- Left: site-title text (small).
- Centre: search input (240px).
- Right: appearance button + auth widget.
- Animation: opacity + translateY(-4px → 0) over 160ms.
- `prefers-reduced-motion: reduce` → instant snap, no transform.

Below 1024px the sticky bar is suppressed — mobile users already have the
utility-row controls in a stable visual location.

---

## Appendix A — Migration sequence (not in scope for this spec, but flagged)

1. Land the shell HTML + CSS behind a `?shell=v2` query-param feature flag
   on a single instance (projects.woodfinegroup.com).
2. Visual diff against current production using percy.io or equivalent.
3. Promote shell to default on the projects instance for one week.
4. Roll to corporate + documentation instances together.
5. Delete `home_chrome()` and `wiki_chrome()` header/footer blocks; both
   now call `shell_header()` / `shell_footer()`.
6. Garbage-collect `.mw-header`, `.site-title`, `.site-nav` from `style.css`.

## Appendix B — Open questions for project-design pickup

1. **Instance pill placement** on marketing context: the marketing site does
   not need it; should the slot be `display:none` or should the left half of
   Row 1 carry something else (e.g. "Newsroom: latest headline" ticker)?
2. **Search affordance on marketing site** — wireframe v2c deliberately
   omits search. Confirm Row 3 centre slot can be a `display:none` collapse
   without leaving a visual gap.
3. **Sticky header on marketing site** — should marketing pages get the same
   collapsed sticky bar, or is it wiki-only? Spec assumes wiki-only; flag if
   wrong.
4. **Quality badge in breadcrumb** — does it belong in the breadcrumb row
   (this spec's proposal) or as a separate flag-row beneath, alongside
   "Forward-looking", "Stub", "Disambiguation" notices?
5. **Icon set for category grid** — which icon family in
   `pointsav-design-system/icons/` is approved? Heroicons outline, Tabler
   monoline, custom? One-per-category mapping needs ratification.
