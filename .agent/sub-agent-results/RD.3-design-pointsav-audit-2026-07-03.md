# design.pointsav.com (`app-privategit-design`) — Structural & Historical Audit

Repo: `/srv/foundry/clones/project-design/pointsav-monorepo/app-privategit-design/`
Live instance: `http://127.0.0.1:9094/`

## 1. Current chrome DOM structure (live, 2026-07-03)

Every page (`/`, `/tokens`, `/components/button/usage`, `/elements/color/overview`) shares one shell:

```
<body>
  <a class="skip-link" href="#main-content">Skip to main content</a>
  <header>                                          ← 48px sticky, dark-brand background
    <a href="/">PointSav Design System</a>
    <div class="header-search-wrap">                ← search input + autocomplete dropdown
    <button id="theme-toggle">☾</button>            ← dark-mode toggle (P2)
    <a href="/tokens" class="header-tokens-link">Tokens</a>
  </header>
  <div class="layout">
    <nav class="sidebar" aria-label="Sections">      ← 256px, left, sections: Elements / Components / Guidelines
      <div class="nav-section">…</div> ×3
    </nav>
    <main class="main" id="main-content">
      <nav class="tab-bar" aria-label="Views">        ← per-item tabs (Usage/Accessibility/Code/Style, or Overview/Tokens)
      <h1 class="page-title">…</h1>
      <div class="content"> … <nav class="breadcrumb">…  </div>
    </main>
  </div>
  <footer class="ds-footer">                          ← 3-band: Canonical links / Machine-surface links / family+legal
  <script>EventSource('/sidebar/sse')…</script>        ← live-reload nav
  <script defer src="/static/drawer.js"></script>
  <script defer src="/static/edit.js"></script>
  <script defer src="/static/ai.js"></script>
  <script defer src="/static/tokens-gallery.js"></script>
  <script defer src="/static/code-copy.js"></script>
  <script defer src="/static/search.js"></script>
  <script defer src="/static/theme-toggle.js"></script>
</body>
```

Nav enumerates: **Elements** (Color, Motion, Org Chart Tokens, Spacing, Typography — 5 items), **Components** (37 items, including a `wiki-*` prefixed subset — see §5), **Guidelines** (Accessibility). Footer has a "Machine surface" column exposing `/tokens/search`, `/bundles/:name`, `/bundles/:name/download`, `/healthz` — a deliberate machine-readable-API surface distinct from the human nav.

Component pages (e.g. `/components/button/usage`) additionally render a **live component preview** block: sandboxed `<iframe srcdoc>` per variant, each iframe self-contained with the component's own CSS pulled from its `recipe.json` (not from site CSS) — this is how the catalog demos components without leaking page styles into them.

CSS referenced by every page: `/static/tokens.css` (2,442 bytes) and `/static/portal.css` (13,697 bytes).

## 2. CSS: palette, typography, breakpoints

**`static/tokens.css`** — Carbon Design System token layer, `--cds-*` custom properties, light + `[data-theme="dark"]` variants:

```css
:root {
  --cds-background: #fff;            --cds-layer: #f4f4f4;
  --cds-border-subtle: #e0e0e0;      --cds-border-strong: #8d8d8d;
  --cds-text-primary: #161616;       --cds-text-secondary: #393939;
  --cds-link-primary: #0e3a66;       --cds-interactive: #0050e6;
  --cds-focus: #0f62fe;              --cds-background-brand: #161616;
  --cds-selected-background: #e8f0f8; --cds-selected-text: #0e3a66;
  --cds-sidebar-bg: #f4f4f4;
  --cds-type-label: 0.75rem;  --cds-type-body: 0.875rem;  --cds-type-heading: 1.25rem;
  --cds-type-title: clamp(1.5rem, 1.3rem + 1vw, 1.75rem);      /* fluid, P2.7 */
  --cds-type-display: clamp(1.625rem, 1.35rem + 1.4vw, 2rem);
  --cds-space-1..12: 0.25rem … 3rem;
  --cds-radius: 4px;
}
[data-theme="dark"] {
  --cds-background: #161616; --cds-text-primary: #f4f4f4;
  --cds-link-primary: #78a9ff; --cds-interactive: #4589ff; --cds-focus: #4589ff;
  --cds-sidebar-bg: #1a1a1a; …
}
```

Typography: `'IBM Plex Sans', system-ui, sans-serif` body; `'IBM Plex Mono', monospace` code. Base 16px / 1.5 line-height.

**Only one breakpoint** in `portal.css`, `@media (max-width: 768px)` — collapses sidebar to a drawer, stacks footer columns, shrinks paddings. Plus `@media (forced-colors: active)` and `@media print` fallbacks (added Phase P2, non-color state cues for Windows High Contrast).

Layout skeleton (portal.css):
```css
.layout { display: flex; flex: 1; overflow: hidden; }
.sidebar { width: 256px; flex-shrink: 0; background: var(--cds-sidebar-bg);
  border-right: 1px solid var(--cds-border-subtle); overflow-y: auto; padding: 1rem 0; }
.main { flex: 1; overflow-y: auto; display: flex; flex-direction: column; }
.tab-bar { display: flex; border-bottom: 1px solid var(--cds-border-subtle); padding: 0 2rem; }
.content { padding: var(--cds-space-4) var(--cds-space-8) var(--cds-space-12); max-width: 860px; }
```

Note a naming inconsistency worth flagging: the *site's own chrome* uses `--cds-*` (Carbon-derived) tokens, while the individual component **recipes catalogued inside the site** (e.g. `wiki-toc-sidebar`) use a *different* `--pds-*` namespace — i.e., the site doesn't yet dogfood its own DTCG output for its own chrome; it dogfoods a separate, older Carbon extraction. This is itself a data point: even design.pointsav.com hasn't fully converged its own shell onto its published token system.

## 3. Git history — full 17-commit list (2026-06-05 → 2026-07-03)

```
06c60c6d  Fri Jun 5 01:39:58 2026 +0000   Peter Woodfine
  feat: Option B — rename scaffold + implement dynamic nav server (port 9094)

6bfe73b4  Fri Jun 5 11:32:59 2026 -0700   Jennifer Woodfine
  ops(fmt): apply cargo fmt to main.rs

28921bed  Mon Jun 8 09:54:55 2026 -0700   Peter Woodfine
  feat: dynamic nav + pulldown-cmark markdown — org-chart-tokens in Elements sidebar
  ("Carbon light shell: sticky header, 256px sidebar, tab bar, page title")

42e1d583  Mon Jun 8 22:06:40 2026 -0700   Peter Woodfine
  fix: clippy::ptr_arg cleanup

8e9fc803  Mon Jun 15 10:28:56 2026 -0700  Jennifer Woodfine
  feat: v0.2.0 multi-module rewrite + 4 moonshot stubs
  (D1 module split, D2 schema-aware rendering, D6 Carbon --cds-* tokens,
   D7 mobile responsive @672px)

78b069c1  Mon Jun 15 19:22:11 2026 -0700  Jennifer Woodfine
  feat: Phase A — D1 routes/ split + D8 sovereign inotify watcher + in-memory search index

402fee79  Mon Jun 15 19:32:36 2026 -0700  Jennifer Woodfine
  feat: Phase B — D4 SSE live-reload sidebar + static/ assets
  (extracts inline CSS to tokens.css + portal.css; adds drawer.js)

04d2e519  Mon Jun 15 19:37:51 2026 -0700  Jennifer Woodfine
  feat: Phase C — D3 WYSIWYG edit overlay + PUT vault save-back

7493c4c5  Mon Jun 15 19:49:20 2026 -0700  Jennifer Woodfine
  feat: Phase D — D5 AI bridge; DoormanOlmo + ClaudeCloud SSE relay; ai.js selection overlay

ef2e7ba3  Tue Jun 16 10:15:19 2026 -0700  Jennifer Woodfine
  feat: Phase B4 — minijinja runtime templates; shell/nav/tab_bar.html
  replace hand-rolled string builder in render/mod.rs

8c540cd4  Wed Jun 17 10:06:31 2026 -0700  Jennifer Woodfine
  style: cargo fmt — pre-Stage-6 fmt gate

0f781dd6  Wed Jul 1 11:59:11 2026 -0700   Jennifer Woodfine
  feat: v0.3.0 DESIGN-BUNDLE directory-mount renderer — /bundles/:name

8f472bd7  Thu Jul 2 19:05:23 2026 -0700   Jennifer Woodfine
  feat: Phase P0 — generalize vault routing to serve components/ (37 previously-
  unreachable components), visual token gallery at /tokens, focus-visible outline

5cd286db  Thu Jul 2 19:18:40 2026 -0700   Peter Woodfine
  feat: Phase P1 — original 3-band footer, 768px breakpoint consolidation,
  drawer.js resize+ARIA fix, touch targets, ai-overlay mobile fix, code-copy,
  tokens bundle mount, token dogfooding, header search, live component preview

7b3d4f7a  Thu Jul 2 19:24:47 2026 -0700   Peter Woodfine
  feat: Phase P2 — dark mode toggle, real landing page (section cards), breadcrumbs,
  skip-link + main landmark + nav aria-labels, forced-colors/print fallbacks,
  favicon, micro-typography bump, fluid clamp() type scale

2fdc98f0  Fri Jul 3 11:03:31 2026 -0700   Peter Woodfine
  fix: DESIGN_TEMPLATES_DIR env var — fix production crash-loop

a249eb06  Fri Jul 3 11:21:19 2026 -0700   Jennifer Woodfine
  fix: DESIGN_STATIC_DIR env var — fix static assets 404 on foundry-prod
```

**Phase framing found in the commits themselves**: `Option B` (initial scaffold naming) → `D1..D8` labels (Phase A/B/C/D map to these: A=D1+D8, B=D4, B4=minijinja templates specifically, C=D3, D=D5) → then a second wave `P0/P1/P2` (polish phases in early July). So there are two distinct phase-naming eras: the June **D-numbered** feature build-out (Phase A/B/C/D), and the July **P-numbered** polish pass (P0/P1/P2), followed by two unnumbered production hot-fixes.

## 4. Earliest commits: structural diff (before feature-accretion)

**Commit `06c60c6d` (Jun 5, "Option B" scaffold)** — `main.rs` is 168 lines, single file, no `templates/`, no `static/`. The entire HTML is inline `format!()` strings. This is the literal starting point:

```rust
async fn index(State(state): State<AppState>) -> Html<String> {
    let mut html = String::from(r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>PointSav Design System</title>
<style>
body { font-family: system-ui, sans-serif; max-width: 800px; margin: 40px auto; padding: 0 20px; }
h1 { font-size: 1.5rem; margin-bottom: 0.25rem; }
.count { color: #666; font-size: 0.875rem; margin-bottom: 1.5rem; }
ul { list-style: none; padding: 0; }
a { color: #0E3A66; text-decoration: none; font-weight: 500; }
</style></head><body><h1>PointSav Design System</h1>
"#);
    // ...then just: <p class="count">N elements</p><ul><li><a href="/elements/{slug}/overview">{slug}</a></li>…</ul>
```

**No header. No sidebar. No footer. No tab bar.** It's literally a single centered column, a title, and a bare `<ul>` of links — closer to a directory listing than a product. This is the "just a clean catalog site" starting point per the operator's framing — though at this stage it's arguably *too* bare (pre-chrome, not yet "site-like").

**Commit `28921bed` (Jun 8, three days later)** — this is where the operator's target reference point actually lives. Commit message literally says *"Carbon light shell: sticky header, 256px sidebar, tab bar, page title."* The hand-rolled `shell()` function in `main.rs` at this commit is:

```rust
fn shell(title: &str, nav_html: &str, tab_bar: &str, page_title: &str, content: &str) -> String {
    // ...
    out.push_str("<header><a href=\"/\">PointSav Design System</a></header>\n");
    out.push_str("<div class=\"layout\">\n<nav class=\"sidebar\">");
    out.push_str(nav_html);
    out.push_str("</nav>\n<div class=\"main\">\n");
    out.push_str(tab_bar);
    if !page_title.is_empty() {
        out.push_str("<h1 class=\"page-title\">"); out.push_str(page_title); out.push_str("</h1>\n");
    }
    out.push_str("<div class=\"content\">"); out.push_str(content); out.push_str("</div>\n</div>\n</div>\n</body>\n</html>");
}
```

with an inline `const CSS: &str` (Carbon palette hard-coded: `#161616` header, `#f4f4f4` sidebar, `#0e3a66` link, `#0050e6` interactive — same hex values later extracted verbatim into `tokens.css`). **No footer, no search, no dark mode, no SSE, no edit overlay, no AI bridge, no breadcrumbs.** Just: header / sidebar / tab-bar / page-title / content. This is the cleanest "just a catalog site" moment in the history — header+sidebar+tabs+content, nothing else, and it's what every subsequent commit builds on top of rather than replaces.

**What got ADDED after this point** (chronologically, per commit messages):
- Jun 15 (Phase A/D1/D8): routes split, inotify file-watcher, in-memory search index (backend only, no UI yet)
- Jun 15 (Phase B/D4): SSE live-reload `<script>EventSource('/sidebar/sse')` + externalized `static/tokens.css` + `static/portal.css` + `drawer.js` (mobile hamburger)
- Jun 15 (Phase C/D3): floating "Edit" button + WYSIWYG textarea overlay + `PUT` save-back (`edit.js`)
- Jun 15 (Phase D/D5): AI assistant overlay, selection-triggered, SSE relay to Doorman/Claude (`ai.js`)
- Jun 16 (Phase B4): moved from hand-rolled Rust strings to actual `templates/shell.html` + `nav.html` + `tab_bar.html` (minijinja) — same DOM, just templated
- Jul 1 (v0.3.0): `/bundles/:name` directory-mount + zip download
- Jul 2 (P0): unlocked 37 previously-unreachable components at `/components/*`, added `/tokens` visual gallery
- Jul 2 (P1): the **3-band footer** (Canonical / Machine surface / family+legal), 768px breakpoint consolidation, header search box + autocomplete, live iframe component previews, code-copy buttons
- Jul 2 (P2): dark-mode toggle, real landing page with section cards (replacing plain link list), breadcrumbs, skip-link, forced-colors/print CSS, favicon, fluid `clamp()` type scale
- Jul 3 ×2: pure infra bugfixes (env-var-configurable template/static dirs — no visual change)

**What has been CONSISTENT since Jun 8 (`28921bed`) without change**: the header-bar / left-sidebar-of-sections / tab-bar-per-item / content-pane four-part skeleton. Every later phase decorates this skeleton (search box in header, footer below, dark-mode toggle in header, breadcrumb inside content) but none of them have ever replaced or restructured it.

Confirming the templating transition, `templates/shell.html` first appears at `ef2e7ba3` (Jun 16) — before that there was no `templates/` directory at all; the three-day-old `28921bed` shell is Rust string concatenation, not a template file.

## 5. Wiki-engine resemblance — does design.pointsav.com's chrome look like a wiki?

**No — the site's own chrome is a distinct "design-system site" idiom, not a wiki layout**, and this distinction is explicit in the codebase's own taxonomy:

The catalog's 37 components are tagged with a `category` field in each `recipe.json`. Cross-checking all of them:

```
badge, breadcrumb, button, checkbox, chip-row, code-block-with-copy,
edit-on-github-link, empty-state-card, input-text, link,
machine-surface-footer, navigation-bar, notification, preview-frame,
select, sidebar-accordion, surface, switch, tab, tab-bar-disclosure   → category: "components"

brand-family-swatch, country-filter-chips, map-side-drawer,
map-stats-panel                                                       → category: "map"

citation-authority-ribbon, freshness-ribbon, home-grid,
research-trail-footer, wiki-article-footer, wiki-article-header,
wiki-badge-tag, wiki-dark-mode-toggle, wiki-drawer-mobile-nav,
wiki-modal-dialog, wiki-pagination, wiki-search-results,
wiki-toc-sidebar                                                      → category: "wiki"
```

So the design system **catalogs** a `"wiki"` category of components (article header/footer, TOC sidebar, pagination, search-results list, badge-tags, dark-mode toggle, mobile drawer nav, citation/freshness/research-trail ribbons) — these are clearly patterns meant for *consumer* products that ARE wiki-shaped (documentation.pointsav.com is the obvious candidate, and this is exactly BIM's current idiom). Example, `wiki-toc-sidebar`'s recipe:

```json
{
  "name": "wiki-toc-sidebar",
  "display_name": "Wiki Table of Contents Sidebar",
  "description": "Sticky right-rail sidebar listing article headings (H2/H3). Active section highlighted via IntersectionObserver.",
  "category": "wiki",
  "html": "<nav class=\"ps-wiki-toc\" aria-label=\"Table of contents\">...</nav>",
  "css": ".ps-wiki-toc { position: sticky; top: 1rem; background: var(--pds-surface-layer-accent); ... }"
}
```

But **design.pointsav.com itself does not use any of these `wiki-*` patterns for its own shell.** Its own chrome is left-sidebar-of-*sections* (not a right-rail TOC of in-page headings), a horizontal tab-bar per catalog item (not article pagination), and a docs-style 3-column footer (not a `wiki-article-footer`). There is no TOC, no "previous/next article" pagination, no article-byline header. The `wiki-*` components exist *in* the catalog as documented patterns — for BIM or a similar sibling to consume — but design.pointsav.com does not eat its own wiki dog food for its own shell.

**Conclusion for the redesign question**: if the operator wants BIM to look like this *sibling product's family* (i.e., the design.pointsav.com site idiom: header + left-nav-of-sections + tab-bar + content pane + docs footer, Carbon-derived palette, `IBM Plex` type), that is a genuinely different visual target than "a wiki." The site itself has never structurally resembled a wiki at any point in its 17-commit history — its earliest post-scaffold shell (`28921bed`, Jun 8) already establishes the sidebar+tabbar+content skeleton that persists unchanged through to today, and it has consistently been a "design-system portal" idiom (Carbon/IBM-inflected internal-tool aesthetic — dark 48px header bar, light gray sidebar, blue accents, IBM Plex fonts) rather than an article/wiki reading-surface idiom. However, this design system's OWN catalog already contains a full `"wiki"` component-category (TOC sidebar, article header/footer, pagination, search results, badge-tags) that would be the more directly relevant reference set if the actual goal is redesigning a wiki-like reading surface rather than restyling BIM into a design-portal shell — those `wiki-*` recipes are a second, more targeted design source worth a follow-up audit.

## Files / paths for reference

- Live shell chrome: `app-privategit-design/templates/shell.html`, `nav.html`, `tab_bar.html`, `footer.html` (HEAD)
- CSS: `app-privategit-design/static/tokens.css`, `app-privategit-design/static/portal.css`
- Pre-template hand-rolled shell (historical): `git show 28921bed:app-privategit-design/src/main.rs` (function `shell()`), `git show ef2e7ba3:app-privategit-design/templates/shell.html` (first templated version)
- Earliest bare scaffold: `git show 06c60c6d:app-privategit-design/src/main.rs`
- Wiki-category component recipes (on disk, served from the vault, not the git repo): `/srv/foundry/deployments/vault-privategit-design-1/components/wiki-*/recipe.json`
- Config/env-var history (source of the two Jul 3 hotfixes): `app-privategit-design/src/config.rs`
