---
artifact: brief
status: active
---

# Plan: Three-site wiki redesign — institutional chrome sprint

## Execution status (as of 2026-05-17)

| Phase | Status | Commit |
|---|---|---|
| B1–B6 — CSS redesign (fonts, tokens, header, dark-mode removal, footer, link colours) | **COMPLETE** | `57c7dfe2` on `readme-fixes-2026-05-16` |
| C1–C7 — Rust rebuild (categories, header chrome, dark-mode JS, emojis, tabs, footer, sticky) | **COMPLETE** | `37fe2a49` on `readme-fixes-2026-05-16` |
| D — Per-site theme verification (SVG wordmarks, `--wf-claret`, `--wf-slate`) | **COMPLETE** | `ada53ef8` on `readme-fixes-2026-05-16` |
| E2 — Stub suppression in home grid | **COMPLETE** | `ada53ef8` on `readme-fixes-2026-05-16` |
| E1, E3, E4 — Quality gates (wanted, category counts, title QA) | **BLOCKED — awaiting build** | post-Stage 6 + project-knowledge build |
| Stage 6 promotion | **PENDING** | Command Session scope; 3 commits ahead of `origin/main`; outbox message sent |

**Branch:** `pointsav-monorepo` sub-clone at `/srv/foundry/clones/project-editorial/pointsav-monorepo/`, branch `readme-fixes-2026-05-16`.

**Next session entry point:** Phase D. Check running service first:
```bash
curl -s http://localhost:9090/ | grep "shell-header"
curl -s http://localhost:9090/ | grep "company\|help"
```
Then inline SVG wordmarks and add `--wf-claret`/`--wf-slate` per-theme CSS overrides in `style.css`. Phase E quality gates follow.

**Font files:** 18 WOFF2 files committed to `static/fonts/` (~728 kB). URL path is `/static/fonts/filename.woff2` (rust-embed serves `static/` at `/static/` prefix).

---

## Context

The operator has flagged that `documentation.pointsav.com`, `corporate.woodfinegroup.com`, and
`projects.woodfinegroup.com` do not look institutional. The target audience is Goldman Sachs and
Google. Problems identified: cluttered header (7 controls, no logo, emoji glyphs), dark mode
toggle (inappropriate for institutional audience), inconsistent title capitalisation, broken
wikilinks, retired categories still rendering, and the site title rendered in a monospace
programming font instead of a wordmark.

The wireframe reference (`/srv/foundry/clones/project-knowledge/wireframe-home-header-v2c.html`)
establishes the design pattern: three-row header (utility | brand | nav), centred SVG wordmark,
clean `--ds-*` token system, Oswald + Roboto Slab + Nunito Sans typography, footer with
cities | copyright | trademark.

**Architecture shortcut:** All three sites run the same Rust binary
(`vendor/pointsav-monorepo/app-mediakit-knowledge/`) via three separate systemd service instances.
A `brand_theme` flag already exists in `server.rs:1773` for per-site theming. One rebuild
covers all three sites.

---

## Decisions required before execution (pending agent research)

1. **Typography — Google Fonts vs self-hosted:** The wireframe loads Oswald + Roboto Slab +
   Nunito Sans from `fonts.googleapis.com`. The design system's trust badge says
   `[ ∅ ZERO COOKIES ]`. Resolution: self-host the three font families in
   `app-mediakit-knowledge/static/fonts/` and serve locally. No Google CDN dependency.

2. **Nav links per site:**
   - `documentation.pointsav.com`: Left = `Disclaimer · Contact` | Right = `pointsav.com · GitHub`
   - `corporate.woodfinegroup.com`: Left = `Disclaimer · Contact` | Right = `Projects · Newsroom`
   - `projects.woodfinegroup.com`: Left = `Disclaimer · Contact` | Right = `Corporate · Newsroom`
   (To be confirmed from Goldman Sachs + Wikipedia research agents)

3. **Search placement:** Wireframe says "search is wiki-only, no search slot in nav row."
   For wiki pages, search moves to a dedicated fourth row below the nav row, or into the
   left-rail panel. NOT in the main header alongside the wordmark.

4. **Cities in footer:** `documentation.pointsav.com` → PointSav entity → "Vancouver · New York"
   or suppress entirely. Woodfine sites → "Vancouver | New York" (per wireframe).

5. **Feature gating:** Remove from all three sites: dark mode toggle, "Aa" appearance button,
   "Edit" button (Phase 5 auth not shipped), "View source" replaces it for public users.
   Retain: search, history, TOC, language switcher.

---

## Critical file map

| File | Purpose | Change type |
|---|---|---|
| `vendor/pointsav-monorepo/app-mediakit-knowledge/src/server.rs` | Live renderer | Rust (rebuild required) |
| `vendor/pointsav-monorepo/app-mediakit-knowledge/static/style.css` | Chrome stylesheet | CSS only |
| `vendor/pointsav-monorepo/app-mediakit-knowledge/static/` | Static assets | Add font files + SVGs |
| `vendor/pointsav-media-assets/ASSET-SIGNET-MASTER.svg` | PointSav signet | Copy to static/ |
| `vendor/pointsav-media-assets/ASSET-WORDMARK-POINTSAV.svg` | PointSav wordmark | Embed in server.rs |
| `clones/project-editorial/content-wiki-documentation/` | EN+ES wiki articles | Content sweep |

---

## Sprint structure — ordered by dependency

### Phase A — Content sweep (no rebuild needed)

These changes make the corpus clean before the renderer updates. All content-only, no Rust.

**A1 — Remove retired categories from renderer** *(actually requires Rust — do in Phase B)*
- `server.rs:593-606`: delete `"company"` and `"help"` from `RATIFIED_CATEGORIES`

**A2 — Batch title normalisation** (content-wiki-documentation, ~262 EN+ES articles)
- Policy: sentence-case per `tokens/design/wikipedia-layout.yaml` `heading_capitalization: "sentence-case"`
- Strip raw-slug titles (e.g. `"service-egress"` → `"Egress service"`)
- Strip prefix-em-dash patterns (e.g. `"os-console — The Command Ledger"` → `"Command ledger"`)
- Retain proper nouns: BCSC, WORM, GIS, WCAG, seL4, PointSav, etc.
- Bilingual ES pairs: same pass in lockstep
- Scope: one editorial session

**A3 — Fix malformed wikilinks** (~12 trailing-backslash artefacts)
- `grep -r "\[\[[^]]*\\" content-wiki-documentation/` to find all instances
- Search-and-replace: `[[doorman-protocol\` → `[[doorman-protocol]]` etc.

**A4 — Stub aliases for moved design-system slugs** (30 slugs from the split)
- Add 5-line stub articles in `design-system/` OR add `aliases:` frontmatter to moved articles
- Resolves red links from body wikilinks to `[[design-color]]`, `[[design-typography]]` etc.
- redirects.yaml already has 30 URL redirects — this fixes the wikilink resolver (different path)

---

### Phase B — CSS redesign (no rebuild — reloaded on next request)

**B1 — Replace font stack**

Typography decision (incorporates Wikipedia agent + Goldman Sachs/Bloomberg agent findings):

Goldman Sachs agent finding: *"Oswald is the weakest link — reads as free-template Squarespace to
a Goldman analyst."* Wikipedia agent: Georgia serif H1 ~34px / sans body 14.4px / 1.6 line-height.

**Resolved stack:**
- **Display/nav labels** — retain Oswald (400/600/700) at 10-11px uppercase in utility and nav rows only. At this scale and usage, Oswald reads as precise nav labelling, not Squarespace. The Goldman concern applies to Oswald as a headline font, not as a compressed nav label.
- **Serif (article H1/H2/H3)** — replace Roboto Slab with **Source Serif 4** (400/700, WOFF2). Source Serif 4 is OFL licensed, carries an editorial weight that matches the institutional read, and performs at Bloomberg/FT levels. Roboto Slab is a reasonable fallback in the `font-face` stack.
- **Body sans** — replace Nunito Sans with **Inter** (400/500/600, WOFF2). Inter is the neutral institutional benchmark; Nunito Sans reads as friendly/consumer. Inter's 14px / 1.6 line-height matches Wikipedia's body rhythm exactly.

CSS variables:
```css
--serif: "Source Serif 4", "Roboto Slab", Georgia, serif;
--sans:  "Inter", -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, sans-serif;
--display: "Oswald", "Helvetica Neue", Arial, sans-serif;  /* nav labels only */
```

Self-host all three families in `static/fonts/` WOFF2. Approximately 1.4 MB total; zero Google CDN
dependency. Preserves `[ ∅ ZERO COOKIES ]` posture.

Files to download (WOFF2 subsets):
- Oswald: 400, 600, 700
- Source Serif 4: 400, 700 (+ italic 400)
- Inter: 400, 500, 600

**B2 — Adopt wireframe token system**
- Add `--ds-paper-1/2/3`, `--ds-ink-1/2/3/4`, `--ds-sp-1..12`, `--ds-ls-*`, `--ds-fw-*` to `:root`
- Map existing variables: `--bg → --ds-paper-2`, `--fg → --ds-ink-1`, `--border → --ds-rule-hairline`
- PointSav accent: `--ps-blue: #869FB9` (replaces `--accent: #869FB9`)
- Woodfine accent: `--wf-blue: #164679` (wireframe default); `--wf-claret: #8B1A2F` / `--wf-slate: #3D5A73` (Phase D, pending brand ratification)
- Link colour: `--link: #1B5DC8` (replaces `#3366CC` — see B6)
- Add `--radius: 2px` — Goldman principle #3 (sharp institutional edges)
- Add `--measure: 68ch` — Goldman principle #4 (reading measure for article body)

**B3 — Rewrite `.mw-header` CSS**
- Replace single-row flexbox with three-row flex-column `.shell-header` pattern
- `.utility-row` — 10px Oswald uppercase, paper-3 background, language toggle right
- `.brand-row` — centred wordmark, generous padding, paper-2 background
- `.nav-row` — 11px Oswald uppercase, three-column grid (left | divider | right)
- Search: new `.search-row` row BELOW nav (wiki pages only); hidden on home page
- Remove all dark-mode `:root[data-theme="dark"]` overrides

**B4 — Remove dark mode entirely**
- Delete `[data-theme="dark"]` CSS block (entire section)
- Delete localStorage theme persistence script from `server.rs` head section
- Hard-code light mode: `html { background: var(--ds-paper-2); color: var(--ds-ink-1); }`

**B5 — Footer CSS**
- Adopt wireframe footer pattern: `.shell-footer` with grid `1fr auto`
- `.cities` — Source Serif 4, 14px, ink-1
- `.footnav` — Oswald uppercase, 10px, ink-3, right-aligned
- `.copyright` — 10px, ink-4, below the footer row
- `.trademark` — 9px, ink-4, fine print at very bottom

**B6 — Wikipedia link colour modernisation**

Wikipedia agent finding: Wikipedia's `#3366CC` is the canonical muscle-memory blue but reads as
slightly faded at modern display densities. Goldman Sachs/FT agent: no external-link arrows, no
visited-purple. Resolution:

- Link colour: `--link: #1B5DC8` — denser than Wikipedia's `#3366CC`, readable on paper-2 (`#F7F9FA`),
  WCAG AA against white. Retains the "this is a link" recognition without the 1990s faded blue.
- Visited colour: **remove entirely**. Delete `--link-visited: #6b4ba1` from `:root` and any
  `a:visited` selector. Wikipedia keeps visited-purple because editors track what they've read;
  our reader audience does not need this distinction.
- External-link arrows: **remove** the `a[href^="http"]::after { content: "↗" }` pattern
  (currently in style.css or server.rs). External links get no visual decoration beyond colour.
- Red links (unresolved wikilinks): retain as `color: var(--ds-ink-3)` underline-dotted — visible
  to contributors but not alarming to casual readers. The Wikipedia agent confirmed this is the
  right discipline for a wiki that has a `/wanted` endpoint.

---

### Phase C — Rust template rebuild (requires `cargo build --release`)

All changes to `server.rs`. One build covers all three sites.

**C1 — Remove `company` and `help` from `RATIFIED_CATEGORIES`** (line 593-606)
```rust
const RATIFIED_CATEGORIES: &[&str] = &[
    "architecture", "substrate", "patterns", "services",
    "systems", "applications", "governance", "infrastructure",
    "reference", "design-system",
];
```

**C2 — Replace header chrome** (replaces `header.mw-header` block at lines 1830-1880)
Three-row pattern:
```
div.shell-header {
  div.utility-row {
    button.lang-toggle aria-label="Language" { "EN ▾" }  // text, no 🌐 emoji
  }
  div.brand-row {
    a.wordmark href="/" {
      // Per-theme: PointSav SVG wordmark OR Woodfine SVG wordmark
      // Inline SVG from ASSET-WORDMARK-POINTSAV.svg or wireframe paths
    }
  }
  nav.nav-row {
    ul.nav-list.left {
      li { a href="/page/disclaimer" { "Disclaimer" } }
      li { a href="/page/contact" { "Contact" } }
    }
    span.nav-divider aria-hidden="true" {}
    ul.nav-list.right {
      // Per-theme nav items (documentation vs corporate vs projects)
    }
  }
  // Wiki-only: search row (not shown on home/marketing pages)
  @if !is_home_page {
    div.search-row {
      form.header-search action="/search" method="get" { ... }
    }
  }
}
```

**C3 — Remove dark mode / appearance controls**
- Delete `wiki-appearance-wrap` / `wiki-appearance-btn` / `wiki-appearance-menu` block (~lines 1840-1863)
- Delete Anti-FOUT localStorage script (~line 1806)
- Delete `data-theme` attribute from `<html>`

**C4 — Replace emoji glyphs with text or SVG**
- `"☰"` → `"Menu"` or inline SVG hamburger icon
- `"§"` → `"Contents"` text label
- `"✕"` → `"Close"` text label
- Focus outline on buttons: keep `:focus-visible` ring

**C5 — Feature gate: show only what's ready**
- Remove "Edit" link entirely (Phase 5 auth not shipped; creating false expectations)
- Remove "View source" from header (keep it as a /git/{slug} link in the article footer instead)
- Keep: Read, History, TOC, Language, Search
- Per-phase approach: Phase 5 ships auth → "Edit" returns; Phase 4 is done → History stays

**C6 — Footer chrome rebuild**
Replace `site-footer` with wireframe footer pattern:
```
footer.shell-footer {
  div.footer-row {
    div.cities { "Vancouver" span.sep { "|" } "New York" }
    nav.footnav {
      a href="/page/contact" { "Contact" }
      a href="/page/disclaimer" { "Disclaimer" }
    }
  }
}
div.copyright { "© 2026 Woodfine Capital Projects Inc. All rights reserved." }
div.trademark { /* canonical trademark text */ }
```

**C7 — Sticky header update**
The sticky header (`.wiki-sticky-header`) appears on scroll — it should show:
`[signet 32px] [article title] [Search]` — minimal, no controls

---

### Phase D — Per-site theming

The `brand_theme` flag in `server.rs:1773` already supports per-theme logic.
Extend it for the three-row header.

Goldman Sachs/Bloomberg agent finding: a single flat colour palette across all three sites
undercuts authority differentiation. Each site has a distinct institutional identity:

| Site | Theme flag | Wordmark | Paper | Accent | Right nav |
|---|---|---|---|---|---|
| `documentation.pointsav.com` | `None` (default) | PointSav SVG | `--ds-paper-1: #FFF` / `--ds-paper-2: #F7F9FA` | `--ps-blue: #869FB9` (PointSav steel) | `pointsav.com · GitHub` |
| `corporate.woodfinegroup.com` | `Some("woodfine")` | Woodfine SVG (wireframe paths) | `--ds-paper-2: #FAF8F5` (warm) | `--wf-claret: #8B1A2F` | `Projects · Newsroom` |
| `projects.woodfinegroup.com` | `Some("woodfine-projects")` | Woodfine SVG | `--ds-paper-2: #FAF8F5` (warm) | `--wf-slate: #3D5A73` | `Corporate · Newsroom` |

**Note on claret and slate:** These are editorial-direction values from the institutional research
agent. They should be validated by a session at `pointsav-design-system` or `woodfine-media-assets`
before hard-coding. The `--wf-blue: #164679` from the wireframe remains available as the fallback for
both Woodfine sites if the brand team does not ratify the differentiated palette.

**Implementation note:** The per-site accent variables live in a `[data-theme="woodfine"]` and
`[data-theme="woodfine-projects"]` CSS block in `style.css`, emitted by the server via the existing
`brand_theme` branch. Paper warm (`#FAF8F5`) signals "private equity" to the Goldman audience;
pure white + steel signals "technology platform." Both are correct for their contexts.

---

### Phase E — Quality gates (post-rebuild)

**E1 — Broken link audit:** Run the live `/wanted` endpoint after rebuild; address the top 20 by inbound-count. Strategy: stub articles for planned content, not red links.

**E2 — Stub suppression:** Articles with `status: stub` should render with a banner but NOT appear in the home-page category grid article list. Add logic to `home_chrome()` to exclude `status: stub` from category article listings.

**E3 — Category counts verification:** After removing `company` + `help`, verify all 10 remaining categories have at least 5 articles in the home grid.

**E4 — Title QA sweep:** After A2, spot-check 20 articles across 5 categories for sentence-case compliance and no raw-slug titles.

---

## Execution order

```
A2 (title normalisation) → A3 (wikilink fix) → A4 (stub aliases)  [content, no rebuild]
   ↓
B1+B2+B3+B4+B5+B6 (CSS redesign)  [CSS only, live on next request]
   ↓
C1+C2+C3+C4+C5+C6+C7 (Rust rebuild)  [one cargo build covers all three sites]
   ↓
D (per-site theme verification)
   ↓
E1+E2+E3+E4 (quality gates)
```

**Note on font self-hosting (B1):** Download Oswald (400,600,700), Source Serif 4 (400,700 + italic 400),
and Inter (400,500,600) as WOFF2. Place in `static/fonts/`. Approximately 1.4 MB total; zero Google
CDN dependency. Preserves `[ ∅ ZERO COOKIES ]` posture. All three families are OFL licensed.

---

## Verification

After Phase C build:
1. `curl -s http://localhost:<port>/ | grep "shell-header"` — new three-row structure present
2. `curl -s http://localhost:<port>/wiki/doorman-protocol | grep "dark"` — no dark-mode controls
3. `curl -s http://localhost:<port>/ | grep "company\|help"` — not in category grid
4. Load in browser: header shows SVG wordmark, not mono text
5. Check all three service instances respond with correct wordmark/accent per theme
6. Run `/wanted` endpoint — count red links; target <15 unique missing slugs

---

## Research findings — incorporated

### Wikipedia muscle memory (7 essential patterns to preserve)

From Wikipedia agent research pass (Main Page + 4 content articles):

1. **White page, dark text** — `#FFFFFF` body, `#202122` text. Our `--ds-paper-2: #F7F9FA` is
   close enough; pure white (`--ds-paper-1`) for article body area.
2. **Centred wide search** — search is the primary nav action, prominent on every article page.
   Our Phase C search-row (below nav row) satisfies this; search must be wide and centred.
3. **Title → rule → lead sequence** — H1 article title, a horizontal rule below it, then the
   lead paragraph before any TOC or infobox. The current renderer must enforce this order.
4. **Right-floated infobox** — key facts panel at article top-right. Not in scope for this
   sprint but noted as the article-anatomy sequence to preserve when introduced.
5. **H2 with full-width rule** — each section break uses an `<hr>`-equivalent visual rule. CSS
   must add `border-bottom: 1px solid var(--ds-rule-hairline)` on `h2` elements.
6. **Single blue link colour** — one link colour, full stop. Resolved as `#1B5DC8` (see B6).
7. **Category strip + last-edited footer** — article bottom shows category membership and edit
   timestamp. These already exist in the renderer; must survive the footer rebuild.

**From Wikipedia: 8 things NOT to copy (modernise instead):**
- Linux Libertine / Georgia as article H1 font → replace with Source Serif 4
- `#3366CC` faded link blue → use `#1B5DC8`
- Visited-purple (`a:visited`) → remove
- External-link arrow glyph → remove
- Red-link alarming red → soften to `--ds-ink-3` dotted-underline
- Edit / History tabs in article header → remove (our Phase C5 does this)
- Maintenance banners in reader view → suppress; show only in contributor view
- Fixed left-margin TOC at all viewports → make collapsible/hidden on mobile

### Goldman Sachs / Bloomberg / FT institutional principles (5 rules)

From Goldman Sachs/Bloomberg/FT agent research:

1. **7-custom-property palette cap.** Institutional sites use ≤7 named colours at the design
   level. Our `--ds-*` token system with `--ds-paper-1/2/3`, `--ds-ink-1/2/3/4`, and one accent
   per site satisfies this exactly. Do not add colour variables during implementation.
2. **Type does the work.** No gradient backgrounds, no hero images, no decorative borders.
   Header chrome gets visual weight from typography and whitespace, not from graphic elements.
3. **`--radius: 2px` sharp edges.** Current radius is unspecified (browser default). Add
   `--radius: 2px` to `:root`; apply to buttons, search input, code blocks. Sharp ≠ harsh at 2px.
4. **68ch reading measure.** Article body text column capped at `max-width: 68ch`. Current
   renderer has no max-width on article body; add to the article content div in CSS.
5. **Legal-identity footer.** The footer must carry: entity name, city, copyright year, trademark
   notice. Woodfine: "Woodfine Capital Projects Inc." PointSav: "PointSav Digital Systems."
   Phase C6 footer rebuild satisfies this.

**Goldman finding on Oswald:** Resolved in B1 — retained for nav labels only at 10-11px uppercase.
At that scale the Goldman concern (Squarespace-ish) does not apply.

**Goldman finding on information density:** "Lead with the number, then explain." Not a layout
change for this sprint, but an editorial discipline for article titles — enforced by Phase A2
sentence-case normalisation.
