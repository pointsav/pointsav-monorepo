# RD.1 — Live BIM Object Library Audit (Round 1)
Audited: `http://127.0.0.1:9096/` (confirmed serving) via `curl`. Pages fetched: `/` (homepage) and `/tokens/spatial` (category page). Chrome (head, utility bar, header, sidebar, footer) is byte-identical between the two except `<title>`.

## 1. Full Chrome DOM Structure

### `<head>`
```html
<link rel="stylesheet" href="/static/fonts.css">
<link rel="stylesheet" href="/static/tokens.css">
<link rel="stylesheet" href="/static/carbon.min.css">        <!-- vendored IBM Carbon Design System -->
<link rel="stylesheet" href="/static/carbon-overrides.css">  <!-- Woodfine brand → Carbon token bridge -->
<link rel="stylesheet" href="/static/bim-layout.css">
<link rel="stylesheet" href="/static/bim-components.css">
<script>
  (function () {
    var stored = null;
    try { stored = localStorage.getItem('bim-theme'); } catch (e) {}
    var theme = stored || (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
    document.documentElement.setAttribute('data-theme', theme);
  })();
</script>
<script type="module" src="/static/carbon.esm.js"></script>
<script type="module" src="/static/bim.js"></script>
```
- The inline `<script>` is a FOUC-avoidance pattern: reads `localStorage['bim-theme']`, falls back to `prefers-color-scheme`, sets `data-theme` on `<html>` before first paint. (Per git history, this is skipped/hardcoded to `data-theme="light"` on `/edit/*` routes because those embed real Carbon web components only styled for light theme.)
- `<body class="bim-body">` — flex column, `min-height: 100vh`.

### Top-level chrome wrappers (in DOM order)

1. **`.bim-utility`** — fixed, `height: var(--bim-utility-height)` = 32px, top of viewport.
   - `.bim-utility__inner` → `.bim-utility__home` (link to `woodfinegroup.com`, text "Woodfine Capital Projects") + `nav.bim-utility__nav` with 3 `.bim-utility__link`s: Corporate, Projects, GitHub (all `target="_blank"`).
2. **`header.bim-header`** — fixed below utility bar, `height: var(--bim-header-height)` = 64px.
   - `.bim-header__inner` contains, in order:
     - `button.bim-topbar__toggle` (hamburger `&#9776;`, `display:none` above 1056px — mobile-only)
     - `a.bim-header__brand` → inline SVG wordmark (`.bim-header__logo`) + `.bim-header__lockup` (`.bim-header__word` "Woodfine", `display:none` since SVG already renders it, + `.bim-header__subtitle` "BIM Object Library" in mono uppercase)
     - `form.bim-search` (`action="/search" method="get"`) → hidden label, `.bim-search__form` div wrapping a magnifier `.bim-search__icon` svg, `input#bim-search-input.bim-search__input`, `button.bim-search__button`
     - `button.bim-theme-toggle` (`aria-pressed`, sun/moon SVG pair swapped via `data-theme` CSS)
3. **`.bim-shell`** — flex row, `margin-top: var(--bim-header-stack)` (utility+header height).
   - `nav.bim-side-nav` — fixed, `width: 16rem`, containing 4 `.bim-nav-group` blocks (see Section 5).
   - `main#bim-main-content.bim-main` — page content, `max-width: 920px`, `margin-left: 16rem`.
4. **`section.bim-disclosure`** (sibling of `.bim-shell`, full-width, `margin-left: 16rem` to respect sidebar) — a native `<details>/<summary>` "Important Information" band, collapsed by default, linking to `/disclaimers`.
5. **`footer.bim-footer`** — dark chrome (`background:#111827`), `margin-left: 16rem`, `margin-top: auto` (pinned to bottom). Full structure in Section 4.

## 2. CSS — Files, Custom Properties, Typography, Breakpoints

### Stylesheet files (`GET /static/*`)
| File | Size | Purpose |
|---|---|---|
| `fonts.css` | 3.7KB / 81 lines | `@font-face` (self-hosted woff2) + font-family CSS vars |
| `tokens.css` | 3.9KB / 87 lines | `:root` custom properties, light/dark palette |
| `carbon.min.css` | vendor | IBM Carbon Design System base |
| `carbon-overrides.css` | 0.7KB / 22 lines | Carbon `--cds-*` token bridge to Woodfine navy |
| `bim-layout.css` | 21KB / 1114 lines | Structural layout (utility bar, header, sidebar, footer, page templates) |
| `bim-components.css` | 8.4KB / 320 lines | Component-level styles (chips, tables, spec cards, badges) |

### Color palette (`tokens.css`, `:root`)
```css
--bim-accent: #164679;            /* = --wf-blue, home.woodfinegroup.com's brand blue */
--bim-accent-hover: #0F2E54;
--bim-accent-active: #0A1F3D;
--bim-accent-subtle: #E8EFF7;
--bim-fg: #111827;  --bim-fg-secondary: #374151;  --bim-fg-muted: #6B7280;  --bim-fg-faint: #9CA3AF;
--bim-border: #E6E7E8;  --bim-border-subtle: rgba(17,24,39,.05);  --bim-border-strong: #9CA3AF;
--bim-bg-page: #FFFFFF;  --bim-bg-sidebar: #F7F9FA;  --bim-bg-subtle: #F7F9FA;  --bim-bg-tertiary: #E6E7E8;
--bim-topbar-bg: #164679;  --bim-topbar-fg: #FFFFFF;   /* hardcoded, NOT theme-derived — utility/header intentionally stay dark-chrome-permanent per the header rebuild's comment, though the actual rendered header is now a LIGHT bar (see below) */
--bim-utility-height: 32px;  --bim-header-height: 64px;  --bim-header-stack: calc(utility+header);
--bim-radius-sm: 2px;  --bim-radius-md: 2px;
/* AEC semantic status colors, brand-governed, for regulation/compliance chips */
--bim-safe:#54924E  --bim-warning:#B54708  --bim-mep:#0E7490  --bim-error:#B42318  (+ *-bg tints)
```
Dark mode (`:root[data-theme="dark"]`) overrides `--bim-accent` (lightened to `#6FA8DC` for AA contrast), fg/border/bg surfaces to a `#0F1218` / `#171B24` / `#1F2430` near-black scale. **Utility strip and footer are explicitly NOT overridden** — they stay permanently dark/light chrome in both themes per the CSS comment (`tokens.css` lines 65-67).

The `tokens.css` file header comment states these values are an *exact 1:1 copy* of `home.woodfinegroup.com`'s brand tokens (`theme-woodfine-wcp.css`), replacing an earlier "BB.14 AEC-differentiated palette."

### Typography (`fonts.css`)
```css
--bim-font-display: 'Oswald', 'Trade Gothic LT Std', 'Barlow Condensed', 'Helvetica Neue', Arial, sans-serif;   /* h1/h2, nav headings, kickers */
--bim-font-sans:    'Nunito Sans', 'Avenir LT Std', 'Avenir Next', 'Mulish', -apple-system, 'Segoe UI', Helvetica, Arial, sans-serif;  /* body */
--bim-font-serif:   'Roboto Slab', 'Caecilia LT Std', 'Zilla Slab', Georgia, serif;   /* long-form prose, sparingly */
--bim-font-mono:    ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;         /* IFC class names, JSON, chips */
```
Font files are self-hosted `.woff2`, vendored "from `app-mediakit-knowledge`'s self-hosted copies of the same Google Fonts `home.woodfinegroup.com` loads" (per the file header comment) — i.e., the *delivery mechanism* (self-hosted subset woff2 files) is wiki-derived, but the *typeface choice* (Oswald/Nunito Sans/Roboto Slab) matches the corporate home site, not the wiki. Confirmed by commit `2ef545f5`'s message: the wiki itself uses a different pairing (Inter + Source Serif 4) that BIM deliberately did **not** port, since that pairing doesn't match `home.woodfinegroup.com` either.

### `@media` breakpoints (all in `bim-layout.css`; `bim-components.css` has none)
| Breakpoint | Line | What changes |
|---|---|---|
| `max-width: 1056px` | 331 | `.bim-topbar__toggle` becomes visible (hamburger shown); `.bim-shell` flips to `flex-direction: column`; `.bim-side-nav` becomes `position:fixed`, full-width, `display:none` by default (toggled via `.bim-side-nav--open`), gets a drop-shadow and `z-index:90`; `.bim-main` loses its `margin-left`/drops padding to `1rem`; `.bim-nav-link` gets larger tap padding; `.bim-search` `max-width` shrinks to `16rem` |
| `max-width: 1056px` (footer) | 910 | `.bim-footer { margin-left: 0; }` (drops the 16rem sidebar offset) |
| `max-width: 1056px` (disclosure) | 1063 | `.bim-disclosure { margin-left: 0; }` |
| `max-width: 768px` | 375 | `--bim-header-stack` redefined to just header height (utility bar folds away); `.bim-utility { display:none }`; `.bim-nav-group--mobile-only { display:block }` — the utility bar's Corporate/Projects/GitHub links get duplicated into a mobile-only sidebar nav group instead of disappearing |
| `max-width: 600px` | 387 | `.bim-header__lockup` (wordmark subtitle) hidden; header padding tightens; `.bim-search` `max-width` shrinks to `8.5rem` |
| `max-width: 480px` | 416 | `.bim-hero__statline` font-size drops to `1.25rem`; `.bim-category-grid` collapses to `grid-template-columns: 1fr` (single column) |
| `max-width: 420px` (×2 rules) | 407, 425 | Search icon hidden entirely; `.bim-search` `max-width` shrinks to `6.5rem`; `.bim-search__button-label` text hidden (icon-only submit button) |

No `min-width` (desktop-up) queries exist — the whole system is mobile-adapted from a fixed-sidebar desktop base, single-breakpoint-cascade style (1056 → 768 → 600 → 480 → 420), not a modern container-query or fluid-first approach.

## 3. Wiki-borrowed vs. BIM-native classification

This is grounded directly in the site's own CSS comments and the git commit messages for the shell-redesign series (`7ae34ad1` → `68e3406f`, all dated 2026-07-02/03), which are unusually explicit about their wiki lineage. Quoting commit `2ef545f5` directly: *"replaces the solid-navy app-shell topbar with a light header + utility bar, matching the structural pattern of the live wiki instances (app-mediakit-knowledge-2 at corporate/projects.woodfinegroup.com)."*

### (a) Wiki-borrowed elements

| Element | DOM/CSS evidence | Source citation |
|---|---|---|
| **`.bim-utility` strip** (cross-links to Corporate/Projects/GitHub) | `bim-layout.css` line 29-30 comment: *"Utility bar — light strip above the header linking to sibling Woodfine properties (matches the wiki instances' pattern)."* | commit `2ef545f5`: *"matching the wiki's own utility-bar convention"* |
| **`.bim-header` as a light bar** (not the original solid-navy app-shell) | `bim-layout.css` line 78-79 comment: *"Header — light chrome (the wiki's proven pattern for this site type, not a solid-navy app-shell bar)."* | commit `2ef545f5` |
| **`.bim-search` form inside the header** | New in `2ef545f5`; GET `/search` form sitting directly in `.bim-header__inner` | commit `2ef545f5`: *"a real search form"* introduced as part of the wiki-matching header rebuild |
| **`.bim-theme-toggle` button in the header** (sun/moon SVG pair) | `.bim-header__inner` last child; `bim.js` lines 91-105 handle click/localStorage | commit `2ef545f5` + `7ae34ad1` ("dark-mode infrastructure," explicitly prompted by *"a direct operator comparison against home.woodfinegroup.com and the live Woodfine/PointSav wiki instances (app-mediakit-knowledge-2) — both have genuine dark-mode support and BIM had none"*) |
| **`.bim-category-card` accent-left-border hover treatment** on the homepage category grid | `bim-layout.css` lines 475-493 (`border-left: 3px solid var(--bim-accent)`, hover raises border/shadow) | commit `be407bee`: *"`.bim-category-card` restyled to the wiki's accent-left-border convention... instead of a plain uniform box border"* — also dropped the description line from cards to match *"the wiki's denser `.k-cat-card` pattern"* |
| **`.bim-side-nav .bim-nav-link` hover/active border treatment** (accent-left-border instead of solid fill) | `bim-layout.css` lines 300-321, explicit comment: *"Sidebar-specific link treatment — accent-left-border on hover/active, matching the wiki's convention, instead of a solid-fill background."* | commit `be407bee` |
| **Footer's overall column/badge/disclosure structure** (BROWSE-style content column + NETWORK column, cities line, badge chips, collapsible disclosure band pattern) | `.bim-footer__inner` 3-column grid, `.bim-footer__base-row` cities+badges, `section.bim-disclosure` `<details>` | commit `bc3bb9dc`: *"Fourth step of the shell redesign — matches the wiki's footer/disclosure convention (BROWSE/THIS-SITE/NETWORK columns, cities line, badge chips, collapsible disclosure band) rather than the previous flat 3-column license dump."* Footer's "Network" column specifically was added to *"match the utility bar"* |
| **Persistent one-line footer disclaimer** (`.bim-footer__disclaimer`, always visible even when the collapsible band is closed) | `bim-layout.css` lines 963-970 comment: *"Matches app-mediakit-knowledge's `.k-footer__disclaimer`."* | same rationale, "so a collapsed band never screenshots bare" — quoted verbatim from the wiki's own reference implementation per commit `68e3406f` |
| **CC BY-ND 4.0 badge icon set** (`cc.svg`, `cc-by.svg`, `cc-nd.svg`) | `.bim-badge--license` in footer | commit `68e3406f`: *"copied from the reference wiki engine's own assets — same icons, not reinvented"* |
| **Important Information disclosure band's structural pattern** (`<details>`/`<summary>` band between main content and footer, full-width) | `section.bim-disclosure` | commit `68e3406f`: built against *"a working reference implementation in the live wiki instances (app-mediakit-knowledge)"*, sourced from a Command inbox spec studying "home.*, Apollo, Apollo Academy, BCSC/EDGAR/SEDAR" |
| **Mobile nav-drawer fallback for the utility bar's links** (`.bim-nav-group--mobile-only`, "Woodfine Network" heading) | `bim-layout.css` lines 371-385 | commit `2ef545f5`: added *"a 'Woodfine Network' nav group repeating the utility bar's cross-property links, shown only below 768px"* |

### (b) BIM-native elements

| Element | DOM/CSS evidence | Notes |
|---|---|---|
| **Sidebar's fixed-position/scroll structure** (`.bim-side-nav`, 16rem fixed column) | `bim-layout.css` line 252 comment: *"Sidebar (ported from app-orchestration-bim)"* | Ported from a **sibling BIM-family app**, not the wiki — the *positioning/skeleton* is BIM-lineage even though its *hover/active link styling* was later reskinned to match the wiki (see (a) above). |
| **24-item category list grouped by IFC taxonomy** ("BIM Objects" nav group: Spatial, Elements, Systems, Materials, Assemblies, Performance, Identity + Codes, Relationships, Key Plans, etc.) | `<div class="bim-nav-group">` with `<p class="bim-nav-group__heading">BIM Objects</p>` | Content is specific to presenting the IFC 4.3 catalog; not present in any wiki instance. |
| **IFC/Uniclass chip row** (`.bim-chip`, `.bim-chip--accent`, `.bim-chip--muted`) showing `IFC IfcSpatialElement`, `UNICLASS SL`, `STANDARD IFC 4.3 · ISO 16739-1:2024`, `FORMAT DTCG` | `bim-components.css` lines 167-202 | BIM-domain metadata display, no wiki equivalent. |
| **AEC semantic status chip variants** (`.bim-chip--safe`, `--warning`, `--mep`, `--error`) for regulation/compliance | `bim-components.css` lines 204-228; colors sourced from `--bim-safe/-warning/-mep/-error` in `tokens.css`, explicitly noted as *"from woodfine-media-assets/css/theme-woodfine.css, brand-governed... not part of home.woodfinegroup.com's stack... but already brand-governed and unrelated to the visual-direction change"* | Kept intentionally as pre-existing brand-governed AEC color system. |
| **`.bim-spec-card` / `.bim-accordion`** (Specification / BIM Objects / Regulation / Climate Zone / Token Format collapsible sections on each category page) | `bim-components.css` lines 247-280 | Specific to presenting one IFC entity's spec data (property sets, IFC hierarchy breadcrumb, raw DTCG JSON token dump) — no wiki analog. |
| **`.bim-detail-table` / `.bim-token-table`** (IFC entity / Uniclass 2015 / IFC hierarchy rows; property-set/property/type tables; token-slug/IFC-class/description tables) | `bim-components.css` lines 116-138, 282-298 | DTCG token tables — core BIM-native content surface. |
| **Key-plan category color swatches** (`.bim-kp-card[data-category="..."]`) | `bim-components.css` lines 84-100 | 7 hardcoded pastel/dark fills per building-typology category (private-office, medical, business, laboratory, academic, civic, corporate-office) — BIM/AEC-specific. |
| **Footer's *content*** — trademark text, Apache-2.0 (BIM data) / AGPL-3.0-or-later (platform code) license lines, `/api/tokens.json` + `/mcp` machine-readable-surface links | `.bim-footer__list`, `.bim-footer__trademark` | The footer's *container* pattern is wiki-borrowed (see 3a), but its *content* (trademarks, dual-licensing split, MCP endpoint) is entirely BIM/Woodfine-product-specific. See Section 4 below — this is flagged as a "keep" regardless of container lineage, per the operator's explicit instruction. |
| **"Powered by PointSav Digital Systems" badge glyph** | `.bim-badge` (first of three), inline SVG document icon | commit `bc3bb9dc`: *"'Powered by PointSav Digital Systems' (inline SVG glyph, not the wiki's literal MediaKit graphic)"* — explicitly NOT reused wiki art, drawn fresh. |
| **Search input scope/copy** ("Search categories, entities, research…") | `bim-search__input` placeholder | Wiki-pattern *placement* (in-header, see 3a) but BIM-specific *copy/scope*. |

**Overall pattern:** the shell-redesign commit series (`7ae34ad1` → `68e3406f`) is a deliberate, systematic port of `app-mediakit-knowledge-2`'s (the live wiki's) structural conventions — utility bar, light header, in-header search, dark-mode toggle, card/sidebar hover treatment, and the entire footer/disclosure-band container pattern — onto BIM's own navy/Oswald/Nunito Sans brand skin and BIM-specific content. The operator's "wiki-style" judgment tracks accurately: nearly every *structural/interaction* decision in the current chrome (not just the three examples cited in the brief) traces to the wiki, while the *domain content* (IFC taxonomy sidebar list, chip taxonomy, spec-card/token-table system, key-plan swatches) is native to BIM.

## 4. Footer — Full Content Detail (reference point, not a critique target)

Structure: `footer.bim-footer` → `.bim-footer__inner` (3-column grid) → `.bim-footer__base` (cities/badges row + copyright/disclaimer/trademark paragraphs).

**Column 1 — "Woodfine BIM Object Library"**
- Specification BIM Objects for the built environment
- 24 BIM Object categories · 18 components · 3 research entries
- IFC 4.3 (ISO 16739-1:2024) · Uniclass 2015 · DTCG
- BIM Object data licensed **Apache-2.0**
- Platform code licensed **AGPL-3.0-or-later**
- Source (github.com/pointsav) → `https://github.com/pointsav/pointsav-monorepo`

**Column 2 — "Machine-readable surface"**
- `/api/tokens.json` — full DTCG bundle
- `/mcp` — MCP JSON-RPC endpoint
- `/research` — research backplane

**Column 3 — "Network"**
- home.woodfinegroup.com
- Corporate (→ corporate.woodfinegroup.com, `target=_blank`)
- Projects (→ projects.woodfinegroup.com, `target=_blank`)
- GitHub (→ github.com/pointsav, `target=_blank`)

**Base row** (`.bim-footer__base-row`, flex, space-between):
- `.bim-footer__cities`: "Vancouver | New York"
- `.bim-footer__badges` — three badges:
  1. `.bim-badge` (non-link): document-glyph SVG + "Powered by / PointSav Digital Systems"
  2. `.bim-badge` (link → `apache.org/licenses/LICENSE-2.0`): "BIM data licensed / Apache-2.0"
  3. `.bim-badge.bim-badge--license` (link → `creativecommons.org/licenses/by-nd/4.0/`, `aria-label="Editorial content licensed CC BY-ND 4.0"`): three stacked official CC icon SVGs (`cc.svg`, `cc-by.svg`, `cc-nd.svg`, sourced from the reference wiki engine's own assets) + "Editorial content / CC BY-ND 4.0"

**Legal paragraphs:**
- Copyright line: *"Copyright © 2026 Woodfine Capital Projects Inc. See LICENSE for terms. · https://bim.woodfinegroup.com"*
- Persistent disclaimer (`.bim-footer__disclaimer`, always visible regardless of the collapsible band's state): *"Provided for reference and coordination only — not a substitute for code review. See [Important Information](/disclaimers)."*
- Trademark block (`.bim-footer__trademark`, 0.6875rem, muted gray `#6f6f6f`, `max-width: 72ch`), verbatim:
  > "Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™, Totebox Orchestration™, Totebox Archive™, and Capability Geometry™ are trademarks of Woodfine Capital Projects Inc., used in Canada, the United States, Latin America, and Europe. Capability Geometry™ is an unregistered trademark of Woodfine Capital Projects Inc. All other trademarks are the property of their respective owners."

Also adjacent (not inside `.bim-footer` proper, but immediately above it): `section.bim-disclosure` — collapsed-by-default `<details>` "Important Information" band with two paragraphs (no-professional-advice / forward-looking-statements language, referencing BIM Object classification/regulatory-overlay verification requirements and the Sovereign Data Foundation) and a "Read the full disclaimer →" link to `/disclaimers`. Text is sourced at request time from a git-owned markdown file (`woodfine-bim-library/site-content/pages/important-information.md`), not hardcoded, per commit `68e3406f`.

Colors used: footer background `#111827` (near-black, distinct from the light `--bim-bg-page`/`--bim-bg-sidebar` tokens — intentionally NOT theme-toggled, i.e. permanently dark chrome regardless of light/dark mode), body text `#c6c6c6`, headings `#F7F9FA`, list links `#a8c8f8`, base-row muted text `#8d8d8d`, trademark text `#6f6f6f`.

Per the task brief, this entire footer (structure, content, and badge treatment) is explicitly a **keep-as-is** reference point — the operator likes it despite/regardless of its wiki-borrowed container pattern (see 3a).

## 5. Sidebar / Nav Structure

`nav.bim-side-nav` (`aria-label="BIM sidebar"`) is **not a single flat list** — it has 4 `.bim-nav-group` blocks, each with a `.bim-nav-group__heading`:

1. **"Overview"** — 3 links: "What are BIM Objects?" (`/`, marked `active`/`aria-current="page"` on homepage), "Browse All BIM Objects" (`/tokens`), "About BIM Objects" (`/about`)
2. **"BIM Objects"** — **24 links**, one per category, all children of `/tokens/*`, presented as a **flat unordered list within the group** (no sub-grouping by IFC entity supertype or any other taxonomy hierarchy despite the group heading implying IFC organization):
   Spatial, Elements, Systems, Materials, Assemblies, Performance, Identity + Codes, Relationships, Key Plans, Amenity Key Plans, Retail Select, Tech Industrial, Building Width Calculator, Floor Plate Standards, Interior, Landscape + Parking, Professional Office Subtypes, Tile System, Water Management, Climate Zones, Building Grid, Floor Plate Assembly Rules, Furniture, Tenant Mix
3. **"More"** — 3 links: "Key Plan Diagrams" (`/key-plans`), "Furniture Library" (`/furniture`), "Research" (`/research`)
4. **"Woodfine Network"** (`.bim-nav-group--mobile-only`, `display:none` above 768px) — 4 external links duplicating the utility bar's cross-property nav, injected only for mobile since the utility bar itself hides at that breakpoint.

Confirmed count: exactly **24** category links under the "BIM Objects" heading (matches the footer's "24 BIM Object categories" stat and the homepage's 24 `.bim-category-card` tiles). Total visible desktop sidebar links: 3 + 24 + 3 = 30 (the 4 mobile-only network links bring it to 34 on narrow viewports).

Link styling: base `.bim-nav-link` class is shared across sidebar links, breadcrumbs, category cards, and research-index items, but `.bim-side-nav .bim-nav-link` gets a scoped override — 2px left border, transparent by default, `var(--bim-border-strong)` on hover, `var(--bim-accent)` + `var(--bim-accent-subtle)` background fill when `.active`/`[aria-current="page"]`.
