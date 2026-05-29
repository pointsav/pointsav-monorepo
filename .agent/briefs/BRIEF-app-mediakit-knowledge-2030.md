---
artifact: brief
status: active
topic: app-mediakit-knowledge — Leapfrog 2030 vision + implementation state
archive: project-knowledge
created: 2026-05-28
supersedes: BRIEF-knowledge-platform.md
owner: totebox@project-knowledge
research_sources:
  - agent-1-home-page-ux-internet-research-2026-05-28
  - agent-2-article-surface-internet-research-2026-05-28
  - agent-3-codebase-synthesis-2026-05-28
---

# BRIEF — app-mediakit-knowledge: Leapfrog 2030

## 1. Product identity

`app-mediakit-knowledge` is a sovereign-data, Wikipedia-pattern HTTP wiki engine.
Single Rust binary. Git-native content store. No PHP, no Node.js runtime, no
MediaWiki, no Hugo.

**Live instances:**

| URL | Service unit | Port | Content repo |
|---|---|---|---|
| `documentation.pointsav.com` | `local-knowledge-documentation.service` | 9090 | `media-knowledge-documentation` |
| `projects.woodfinegroup.com` | `local-knowledge-projects.service` | 9093 | `media-knowledge-projects` |
| `corporate.woodfinegroup.com` | `local-knowledge-corporate.service` | 9095 | `media-knowledge-corporate` |

**One-sentence positioning:** A knowledge platform where every article is Git-committed,
every claim is citable and planned for machine-queryable verification, and the entire
stack runs on infrastructure owned and operated by the customer — not a third-party cloud.

**"21st-century Wikipedia" design mandate:**
The reading surface must look like a great magazine article, not a CMS admin panel.

| Reader state | Visible UI |
|---|---|
| Anonymous reader | Clean title + lead + body. Zero edit controls. Status badge = coloured dot (expandable). Citation ribbon at bottom, collapsed. |
| Logged-in contributor | Edit pencil on section hover. Full tools in overflow, not dominant. |
| Mobile | All toolbars behind `…` overflow or bottom action bar. |

Talk/Discussion tabs are never visible to anonymous readers. History is accessible via
`Tools ▾`, not a prominent tab.

**Why not MediaWiki:** PHP runtime, MySQL, not customer-rooted, Wikipedia-branded not
product-branded, no claim-layer, no modern tokenized CSS.

**Why not Hugo:** Static — no search-as-you-type, no auth-gated content, no edit
workflow, no revision history UI, no claim verification, no MCP API.

**Market peer:** Q4 Inc. (Toronto; TSXV: QFOR) serves public-company investor relations.
Their gap: no customer-rooted claim layer, no bilingual structured content, no edit review
queue for regulatory disclosure text. Our differentiation: claim-layer citation
verification, tamper-evident Git-native audit trail, no vendor lock-in on the content store.

---

## 2. What's locked (non-negotiable decisions)

These are decided. Do not revisit within a session without operator confirmation.

| # | Decision | Rationale |
|---|---|---|
| L1 | Single Rust binary (`cargo build --release -p app-mediakit-knowledge`) | Customer-rooted; no runtime dependency |
| L2 | Git-native content store (flat `.md` files + `git2` library) | Markdown + Git = 50-year readable, diffable, auditable |
| L3 | DTCG token pipeline (`scripts/dtcg-bundle.json` → `dtcg-to-css.py` → `static/tokens.css`) | Single token vault, design-system aligned |
| L4 | Bilingual routing (`.es.md` sibling articles, single canonical slug) | All public content ships in EN + ES |
| L5 | Self-hosted WOFF2 fonts — no CDN | GDPR Art. 44; `784ceea7` removed all Google Fonts CDN links |
| L6 | Wikipedia Vector 2022 DOM conventions (`aside.toc`, `nav #p-views`, `div #mw-content-text`, etc.) | MediaWiki tooling compatibility; established muscle memory |
| L7 | Canonical footer trademark text verbatim (see §9) | Legal; sourced from `wireframe-home-header-v2c.html` |
| L8 | Font stack: Oswald (display h1/h2) + Nunito Sans (body/UI) + Roboto Slab (blockquotes) | Round-2 jury winner; `70259d32`; `784ceea7` |
| L9 | `--navy: #164679`; `--bg: #F7F9FA`; `--link: var(--navy)` | Core brand token triad; WCAG AA verified |
| L10 | MCP JSON-RPC 2.0 native implementation (`src/mcp.rs`, ~330 lines) | Doctrine claim #54 ("We Own It"); no `rmcp` vendor SDK |
| L11 | Claim-layer HTML comment markup (`<!--claim id=... confidence=... cites=[]-->`) | Already in production content; foundation for §11 |
| L12 | SYS-ADR-07: no structured data through AI | IFC, citation records, article AST — deterministic pipelines only |
| L13 | SYS-ADR-10: F12 mandatory; human commits only | Edit review queue enforces; no auto-publish |
| L14 | SYS-ADR-19: no automated AI publishing to verified ledgers | AI marginalia is ephemeral overlay, never committed |
| L15 | Apache 2.0 licence | Matches monorepo licence |
| L16 | Commit identity: `jwoodfine`/`pwoodfine` only; `commit-as-next.sh` only | Pre-commit gate enforces; no direct `git commit` |

---

## 3. Current implementation state (Phases 1–7H shipped + deployed, 2026-05-29)

All commits on `pointsav-monorepo` staging branch. Phase 6A/6B/6C promoted to canonical
by Command (2026-05-28). Phases UX-A through 7H committed 2026-05-29; binary deployed to
all three live instances (2026-05-29 ~05:15 UTC). Stage 6 promotion of 15 commits pending
Command Session (`bin/promote.sh`).

| Phase | Status | Key commit(s) |
|---|---|---|
| 1 — render + chrome | Shipped | Route `/wiki/{slug}`, TOC, hatnote, tabs |
| 1.1 — Wikipedia chrome | Shipped | Article/Talk/History tabs, language switcher, footer |
| 2 — edit (Steps 1–7) | Shipped | JSON-LD, atomic edit, CodeMirror 6, SAA squiggles, citation autocomplete |
| 3 — search + feeds | Shipped | Tantivy BM25, `/feed.atom`, `/sitemap.xml`, `/llms.txt` |
| 4 — git sync + MCP | Shipped | git2, redb, blake3, MCP JSON-RPC 2.0, git smart-HTTP, OpenAPI 3.1 |
| 4 DTCG | Shipped | `dtcg-bundle.json` → `dtcg-to-css.py` → `tokens.css` (157 tokens, all colors in `oklch()`) |
| 5 core — auth + edit review | Shipped | Cookie sessions, argon2id, edit review queue; `auth.rs` + `pending.rs` + `users.rs` |
| 5.1+ ACLs/OIDC/webhooks | Deferred | Gated on BP5 clearance |
| 6A/6B/6C | Shipped + promoted `afa67bfa` | AJAX nav fix; home page caps; topnav 80px header |
| 7A | Committed + deployed `168314a1` | TOC buttons + topnav search restored |
| UX-A | Committed + deployed `0dfe1647` | Typography tokens wired; dark-mode contrast; auto dark mode; appearance toggle hidden |
| UX-B | Committed + deployed `2a19c626` | Appearance dropdown removed; home standfirst; `shell_footer()`; footer convergence; CC BY 4.0 gate; provenance ribbon |
| 7B | Committed + deployed `bbb339b5` | `nav.article-tabs`; Tools▾ dropdown; anchor-share `¶`; auth-gated tabs |
| 7C | Committed + deployed `d649f051` | Reading mode toggle; CSS body-class; localStorage |
| 7X | Pre-existing | Home search hero; YAML-based featured article + DYK |
| 7D | Committed + deployed | Citation hover preview; freshness dot; CITATIONS redb table |
| 7E | Committed + deployed `bbb339b5` | Mobile bottom bar; article-tabs hidden on mobile; table/code overflow |
| 7F | Committed + deployed `c240837b` | Tufte sidenotes for `layout: journal`; `inject_sidenotes()` post-processor |
| 7G+7H | Committed + deployed `c240837b` | Auto-numbered sections for `[data-instance="woodfine-corporate"]` (CSS) |
| **8 — history surface** | **Next** | Revision list at `/history/{slug}`; diff UI; `article-integrity-bar` (blake3 SHA) |

**Test state:** `cargo test` exits 0 (107 tests: 106 lib + 1 journal integration; `collab_test` +
`doorman_test` pre-existing failures on removed `AppState.collab` field — unrelated to current phases).

**Binary state:** All three services live on binary deployed 2026-05-29 ~05:15 UTC (includes
Phases UX-A through 7H). 15 commits pending Stage 6 promotion by Command.

**UX-B.7 — BLOCKED:** `WORDMARK_WOODFINE` constant still Unicode `■ Woodfine`. Operator must
provide Woodfine Management Corp. SVG wordmark file before this can ship.

**Post-6C cleanup (now done):** Legacy `.shell-header` CSS block removed in Phase 7B.

**Still open:**
- ES bilingual pairs for four governance stub articles
- `.agent/manifest.md` wrong `cluster_name` (says `project-bim`) — Command correction needed

---

## 4. Three-instance differentiation

The engine serves three distinct editorial brands from a single binary.

**Instance flags (AppState):** `brand_theme: BrandTheme` + `brand_instance: BrandInstance`
**HTML attribute:** `data-instance` on `<html>` root (enables per-instance CSS scoping)

| Instance | Domain | Brand | Token file | `data-instance` | `brand_theme` |
|---|---|---|---|---|---|
| documentation | `documentation.pointsav.com` | PointSav | `tokens.css` | `pointsav` | `PointSav` |
| projects | `projects.woodfinegroup.com` | Woodfine | `tokens-woodfine.css` | `woodfine-projects` | `Woodfine` |
| corporate | `corporate.woodfinegroup.com` | Woodfine | `tokens-woodfine.css` | `woodfine-corporate` | `Woodfine` |

**Per-instance branding approach (2030 target):** Three CSS custom properties per
subdomain — `--brand-accent`, `--brand-wordmark`, `--brand-surface` — sufficient to
differentiate without separate stylesheets. Each instance has a distinct home page hero
section; article chrome is shared. Introduced via `data-instance` attribute selector.

**Phase 6 deployment split (gated — three conditions):**
1. Operator renames six GitHub repos: `{j,p}woodfine/content-wiki-{documentation,projects,corporate}` → `media-knowledge-{documentation,projects,corporate}` (GitHub UI)
2. Command Session applies MASTER Doctrine amendment: source-of-truth inversion for `media-knowledge-*` repos (Totebox clone = canonical; GitHub = downstream mirror)
3. Command updates service unit `WIKI_CONTENT_DIR` for projects + corporate instances

Nothing in this Totebox archive to do until Command confirms gates 1+2 clear.

---

## 5. Home page — vision + current state + 2030 targets

### Current state (post-6B, pending rebuild)

Three sections rendered by `home_chrome()`:
- Recent articles (capped at 8, fetch by last-edited date)
- Guides (capped at 6, alphabetical from content root)
- Category grid (12 ratified categories: architecture, substrate, governance, services, reference, company, help, …)

Uncategorised articles block removed (6B). No search-hero. No featured article. No curated reading paths.

### 2030 vision

**Recommended pattern: Hybrid A5 — search-hero + curated category grid + recent.**

Highest-leverage additions in priority order:

1. **Search hero** — centered search input above the fold (below the 80px topnav), 480px
   max-width, with instant autocomplete against the Tantivy index (infrastructure already
   present in `/api/complete`). First contentful element every reader sees is an invitation
   to find something. This single change has the highest conversion from "home page visit"
   to "article read".

2. **Featured article** — editorially curated by frontmatter flag `featured: true` in the
   topic file. Rotates weekly (operator-controlled). Large card above the category grid with
   article title, lede paragraph, and a quality badge. Mirrors Wikipedia's "Featured Article"
   column — the human-editorial signal that the wiki has a point of view about its own content.

3. **Reading paths** — YAML-configured article sequences (`reading-path: governance-primer`)
   stored in the content repo. Displayed as horizontal cards on the home page: "Start here →
   Governance overview → Constitutional charter → §IV compliance". Progress tracked per user in
   `localStorage`. For logged-in users, synced to server. This is the highest-leverage new
   feature for institutional finance readers who need a guided curriculum, not a file tree.

4. **"Did you know?"** — a rotating factoid pulled from articles tagged `dyk_candidate: true`
   in frontmatter. Single sentence, citation marker, link to the source article. Appears below
   the featured article. Mirrors Wikipedia's DYK box; maintains the encyclopedic editorial voice.

5. **Per-instance home page hero** — `documentation.pointsav.com` shows a PointSav-branded
   wordmark + tagline in the hero area; `projects.woodfinegroup.com` shows project-specific
   navigation; `corporate.woodfinegroup.com` shows the corporate charter lede.

### Content corpus (2026-05-28)

~220 English topics across 10 categories in `content-wiki-documentation`:
- Architecture: 54 topics
- Substrate: 33 topics
- Reference: 32 topics
- Services: 21 topics
- Governance: 15 topics
- (remaining categories: infrastructure, applications, company, help, other)

Spanish sibling files (`.es.md`) present for a subset; full bilingual coverage is a
standing open item.

---

## 6. Header + navigation — vision + current state + 2030 targets

### Current state (post-6C + 7A, pending rebuild)

Single-row `header.topnav` (80px, `1fr/auto/1fr` grid):
- Left `div.left`: SVG wordmark (PointSav 320×80px inline SVG, navy on white), Oswald 11px uppercase nav links
- Center `div.center`: empty (reserved for future headline element)
- Right `div.right`: `div.topnav-search-wrap` (form + `#search-autocomplete-dropdown`) + language toggle + user menu

Token: `--header-h: 80px`. Defined in `src/server.rs` at `home_chrome()`, `wiki_chrome()`, `chrome()` — three separate emits of `header.topnav`.

### 2030 vision

**For `wiki_chrome` only: two-row header (topnav 80px + article-tabs row 40px).**

The `home_chrome` and `chrome` functions keep the single 80px topnav. Only `wiki_chrome`
needs a second row because only the article surface has article-specific actions.

Proposed article-tabs row (hidden on `home_chrome`/`chrome`):
```
[Article] [Talk]  ·············  [Read] [Edit*] [History] [Tools ▾]
                                        *hidden for anon
```

`Tools ▾` dropdown contains:
- Cite this page
- Permanent link (exact version URL)
- Printable version
- Download as PDF
- Page information
- Special pages

**Per-section affordances (all readers):**
- Anchor-share glyph (`¶`) appears on `h2:hover` (desktop) or always at 50% opacity (mobile). Click copies `#section-slug` to clipboard. 1.5s toast confirmation.

**Per-section edit pencil (contributors only):**
- `div.section-edit` injected next to each `h2` in `wiki_chrome`. Hidden via `body.is-anon .section-edit { display: none; }`. On click: jumps to the section editor (Phase 2 CodeMirror, already implemented).

**Language switcher:**
- Top-right of the topnav `div.right`, before the user menu. `lang="en"` / `lang="es"` toggle. For wiki pages, resolves to the `.es.md` sibling (or shows "no Spanish version available" state).
- Uses `prefers_spanish()` logic already present in `src/server.rs` (Phase 5 bilingual).

---

## 7. Article reading surface — vision + current state + 2030 targets

### Current state (post-7A, pending rebuild)

**Layout (from `wiki_chrome()`):**
```
header.topnav (80px)
nav.crumb
div.shell
  nav.sidebar (left)
  main.article-wrap
    h1.article__title
    p.article__lede
    dl.article__meta
    aside.toc
      div.toc__header
        span.toc__title
        button.toc-toggle #toc-toggle
        button.toc-pin-btn #toc-pin-btn
      ol #toc-list
    div.prose
  (right metadata rail — placeholder)
```

**Active-section TOC tracking:** `initActiveTocTracking()` in `wiki.js` using
`IntersectionObserver`. Targets `.prose h2[id], .prose h3[id]`. Working post-6A fix.

**Font stack:**
- Display (h1, h2): Oswald, self-hosted WOFF2
- Body + UI (h3+, body, TOC, metadata): Nunito Sans, self-hosted WOFF2
- Serif accent (blockquotes, lede): Roboto Slab, self-hosted WOFF2
- Mono (code, pre): `ui-monospace, "SF Mono", Menlo, Consolas`

### 2030 typography targets

Research confirms the current font pair is correct. Gaps to close:

**Body text:**
```css
.page-body {
  font-size: 17px;                     /* target: 17px (between 16px min and 18px research opt.) */
  line-height: 1.6;                    /* generous for sustained reading */
  max-width: 68ch;                     /* clamped measure; highest-impact change */
  font-feature-settings: "kern", "liga", "calt";
  font-variant-numeric: oldstyle-nums proportional-nums;  /* institutional finance look */
  text-rendering: optimizeLegibility;
}
```

**Tables (tabular numerics):**
```css
.page-body table {
  font-variant-numeric: lining-nums tabular-nums;
}
```

**Vertical rhythm:** Every block element's `margin-block-end` should be a multiple of `0.75rem`.
Headings start on a baseline-grid row: `h2 { margin-top: calc(1.6 * 2 * 1rem); }`.

**Bilingual note:** Nunito Sans covers full Latin Extended-A — no per-language font swap.
Spanish averages 15–20% longer than English. All flex children need `min-width: 0;` to
prevent overflow. Add `hyphens: auto` on `.page-body[lang]` for both languages.

**Reading mode (2030 target):**
A "Reading Mode" button in the article toolbar. CSS-only:
- Hide topnav, tabs, TOC, metadata rail
- `max-width: 64ch` on body
- `font-size: 19px`
- Background `#fbf7ec` (warm paper), foreground `#222`
- Floating "Exit" pill bottom-right
- Persist `localStorage["wiki-reading-mode"]` across sessions
This is approximately 50 lines of CSS and ~20 lines of JS.

**Density control (2030 target, logged-in only):**
Three-state `--density: comfortable | compact | spacious` toggle in user preferences.
Changes only line-height, paragraph spacing, and chrome heights — never font-size.
Default `comfortable` for all readers; toggle exposed in user preferences only.

**Tufte sidenotes for JOURNAL-class articles (2030 target):**
Articles with frontmatter `layout: journal` activate marginal note rendering at ≥1280px
viewport. Footnotes and inline citations render as Tufte-style sidenotes in the right
margin column adjacent to their cited paragraph; collapse to inline expanders below
1280px. This serves the J1–J6 academic paper programme (see `.agent/rules/journal-artifact-discipline.md`)
directly without affecting standard wiki articles. Does not require changes to other
article types — gate on frontmatter flag only.

**Per-instance body type (2030 direction, not current):**
The current Oswald/Nunito Sans/Roboto Slab stack (locked as L8) is the editorial
platform-document register. Research suggests the **corporate instance** specifically
benefits from a heavier serif body (18–19px, higher line-height) aligned with the
institutional finance / legal document register. This is a future per-instance
typography split to evaluate in a dedicated design sprint, not a change to the locked
stack. Candidate: Source Serif 4 (full Latin Extended-A, open licence) for `[data-instance="woodfine-corporate"] .page-body`.

---

## 8. Article toolbars — vision + current state + 2030 targets

### Current state

`nav #p-views` (Wikipedia Vector 2022 convention) renders Article/Talk/Read/Edit/History tabs.
Anonymous users see Edit (but it prompts login). History tab visible to all.

### 2030 targets

**Toolbar state machine:**

| User state | Visible tabs | Edit pencil | Tools dropdown |
|---|---|---|---|
| Anonymous | Article, Talk (hidden), Read | Hidden | Cite, Permanent link, Printable version |
| Logged-in contributor | Article, Talk, Read, Edit, History | Visible on `h2:hover` | Full set (above + Download PDF, Page information) |
| Admin | Same as contributor | Same | Same + Move, Protect (future) |

**Progressive disclosure rule:** Only Read and Article are visible by default for anonymous.
Everything else goes into `Tools ▾` or is promoted to the tab row only for contributors.

**Per-section toolbar on article h2 hover:**
```
[Section title]                               [¶] [✏️ Edit section*]
```
- `¶` (anchor-share): always visible, all reader states. Touch: 50% opacity always.
- `✏️ Edit section`: contributors only; `body.is-anon .section-edit { display: none; }`.

**"View source" in Tools (not "View on GitHub"):**
Per the "We Own It" principle, GitHub is a downstream mirror. The canonical source
view is a `GET /wiki/{slug}?action=raw` endpoint returning the markdown source from
the canonical git server. The toolbar entry reads "View source", not "View on GitHub".

**Print / export:**
- "Printable version" in Tools → server renders `chrome()` with only the article body,
  no topnav, no TOC sidebar. Browser print-to-PDF from there.
- `@media print` stylesheet already in scope — needs a pass to hide `header.topnav`,
  `nav.crumb`, `aside.toc`, `nav.sidebar`, article tabs. Single-column at 1in margins.

**History surface (2030 target):**
Reverse-chronological list: date | author | size delta (green `+` / red `−`) | edit summary.
Radio buttons for arbitrary diff selection. Side-by-side diff with line-level highlight.
Infrastructure: `record_hash()` in `src/links.rs` + git2 log — already wired (Phase 4).

---

## 9. Footer — canonical text + 2030 targets

### Canonical footer text (verbatim — do not modify)

Sourced from `wireframe-home-header-v2c.html`. All three instances use this text:

```
© 2026 Woodfine Capital Projects Inc. All rights reserved.
Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™,
Totebox Orchestration™, and Totebox Archive™ are trademarks of Woodfine Capital
Projects Inc. used in Canada, the United States, Latin America, and Europe. All other
trademarks are the property of their respective owners.
```

Year field updates annually. The `PointSav is a trademark of PointSav Digital Systems`
line from Design D prototype is **not** canonical — it was rejected by the jury.

### Current state

Footer emits the trademark text inside a `footer.site-footer` element. Present in all
three chrome functions.

### 2030 targets

**Citation ribbon** (collapsed by default): a thin bar at the bottom of each article page
below the footer. Displays: "Cite this article as: [auto-generated citation]". Expands to
a panel with APA, MLA, BibTeX formats. Clicking any format copies to clipboard. The
citation generation uses the article's frontmatter (title, author, date) from the Git
commit log.

**Corporate instance — `effective_date:` / `supersedes:` frontmatter block:**
Any article on `corporate.woodfinegroup.com` with frontmatter `effective_date:` and
optionally `supersedes:` renders a disclosure block immediately under `h1.article__title`:
```
Version 1.4 · Effective 2026-04-01 · Supersedes Version 1.3 (2025-11-12)
```
This is the working-paper / SEC-filing convention; it is also what the
`foundry-journal-v1` schema already specifies for JOURNAL manuscripts (see
`.agent/rules/journal-artifact-discipline.md` "Public posting requirements"). Extend
the same pattern to wiki articles on the corporate instance. Gate on `brand_instance`.

**Corporate instance — auto-numbered sections:**
For corporate articles, auto-generate `1.` / `1.1` / `1.1.1` section numbers from the
heading hierarchy at render time. Implemented as a CSS `counter-reset` + `counter-increment`
block scoped to `[data-instance="woodfine-corporate"] .page-body`. Operators may opt out
per-article with frontmatter `numbered_sections: false`. Suppressed on documentation and
projects instances.

**Corporate instance — suppress feedback widget:**
The "Was this helpful?" widget is appropriate for documentation. It is wrong-register for
the corporate instance (regulatory documents are not rated by helpfulness). Gate via
`brand_instance != BrandInstance::Corporate` in the chrome emit.

**"Last edited" + integrity fingerprint:**
```
Last edited by jwoodfine · 2026-05-28 · SHA-256: 7a9b...2c4f
```
The blake3 hash from `src/links.rs` `record_hash()` is already stored in `links.redb`.
Surface it in a `div.article-integrity-bar` below the article body, above the footer.
Clicking the hash copies the full 64-char hex.

---

## 10. Mobile — current state + 2030 targets

### Current state

`@media (max-width: 768px)` rules hide:
- `a:not(.lang-toggle)` in the topnav (nav links hidden; wordmark + lang toggle stay)
- `.wiki-appearance-wrap`
- Topnav search input narrows to 100px (from 170px)

TOC remains in the article flow at 768px. No dedicated mobile chrome pattern.

### 2030 targets

**Breakpoint strategy:**

| Breakpoint | Layout |
|---|---|
| 0–767px (mobile) | Single column. No TOC rail. Sticky top bar 56px. Sticky bottom action bar 56px. |
| 768–1023px (tablet) | Single column. TOC collapses to a button in sticky header. No right metadata rail. |
| 1024–1279px (desktop) | Left TOC visible. No right metadata rail. |
| 1280px+ (wide) | Left TOC + right metadata rail (claim-rail, §11). |

**Top bar (56px, sticky, mobile only):**
```
[☰ menu]   [PointSav wordmark]   [🔍 search]
```

**Bottom action bar (56px, sticky, mobile only):**
```
[📑 TOC]  [⭐ save]  [🔗 share]  [✏️ edit*]  [⋯ more]
```
*Edit only for logged-in users. All touch targets: 44×44px minimum (WCAG 2.5.5 / Apple HIG).
Bar slides down on scroll-down, slides up on scroll-up.

**Mobile TOC:** Tap `📑 TOC` → bottom sheet, 80% viewport height, IntersectionObserver-driven
active highlight. Tap heading → sheet dismisses + `scroll-margin-top: 72px` scroll.

**Code blocks on mobile:**
```css
@media (max-width: 767px) {
  .page-body pre { font-size: 13px; overflow-x: auto; -webkit-overflow-scrolling: touch; }
  .page-body pre::after { /* right-edge fade indicating horizontal scroll */ }
}
```
Do not word-wrap code — indentation-sensitive languages (Python, YAML, Rust) break.

**Tables on mobile:** `div.table-scroll` wrapper with `overflow-x: auto; tabindex="0"`.
Same right-edge fade pattern. Do not reflow tables to cards — institutional finance data
tables must remain scannable.

---

## 11. 2030 differentiation

### Already built (22 features in production or committed)

From the Phase 4 + 5 + 6 + 7A implementation:
- Native MCP JSON-RPC 2.0 server (`src/mcp.rs`) — AI agents are first-class readers
- Claim-layer HTML comment markup in production content
- redb wikilink graph with backlinks (`src/links.rs`) — `GET /special/whatlinkshere/{slug}`
- blake3 content hashes in `links.redb` — federation baseline, integrity fingerprint
- Tantivy BM25 full-text search with `/api/complete` autocomplete
- git smart-HTTP read-only remote (`src/git_protocol.rs`)
- OpenAPI 3.1 spec + `/llms.txt` — machine-readable entry points
- Bilingual `/es/` routing with `Accept-Language` negotiation
- Edit review queue (Phase 5) — human F12 approval before content goes live
- SAA squiggles in CodeMirror 6 — style/terminology guidance at edit time
- Citation autocomplete in the editor
- Revision history + diff surface (`/history/{slug}`, `/diff/{slug}/{a}/{b}`)
- QR code in `app-console-keys` (separate but Kitty/Sixel infrastructure)
- DTCG token pipeline — design-system aligned, zero hardcoded hex in templates
- Self-hosted fonts — no external CDN, GDPR compliant
- `data-auth` + `data-instance` on `<html>` — CSS state machine for all reader states
- Breadcrumb navigation (`nav.crumb`)
- `GET /feed.atom` + `GET /feed.json` + `GET /sitemap.xml` + `GET /robots.txt`
- argon2id password auth + cookie sessions (Phase 5)
- Quality badge infrastructure (coloured dot expandable; Phase 5 UI)
- Citation ribbon placeholder (Phase 5 footer)
- IVC band placeholder in article chrome

### The three planned 2030 differentiators

These three features, implemented together, create a platform that no current documentation
or wiki system offers:

**Differentiator A — Claim-rail freshness sidebar**

A second optional right rail (visible only at ≥1280px viewport) that maps every paragraph
to its citation freshness state. As the reader scrolls, the rail shows colored ticks
aligned to each cited paragraph: green = verified within freshness window, amber = stale,
red = broken URL.

Implementation plan:
- Server: at render time, walk the article AST, emit `<aside class="claim-rail">` containing
  one `<a>` per citation with `data-cite-id`, `data-status`, `data-checked-date`, and
  `data-paragraph-anchor` attributes.
- Client: ~80 lines of JS using `IntersectionObserver` on paragraphs to highlight the
  corresponding rail tick.
- Storage: extend `links.redb` with a `citations` table — `(cite_id, source_url, last_checked, status)`.
  A nightly job re-validates URLs.
- Visual: 4px wide rail. Ticks sized to citation density per paragraph. Negative space dominates.

Infrastructure required: extend `citations` redb table; nightly URL-validation job;
`claim-rail` emit in `wiki_chrome()` server-side.

**Differentiator B — AI marginalia as opt-in ephemeral overlay**

A "Summarize section ▾" button in the article toolbar (contributor-state and above).
On click: an aside slides in from the right (or up from bottom on mobile) with a 3-sentence
section summary. The summary is rendered into the *overlay only* — never into canonical
article HTML (SYS-ADR-19). A clear `service-slm/local — NOT AUTHORITATIVE` label is present.
The summary surface uses `font-family: var(--font-mono)` and a warm paper background to
prevent visual confusion with article text. A cite-back ribbon at the bottom: "Based on
§3.2, §3.4, §5.1" with clickable section jumps.

Gated by: SYS-ADR-07/10/19 review by operator; `service-slm` local integration.

**Differentiator C — Cross-session reading state (no vendor cloud)**

Reading position stored in `localStorage["wiki-read-state"]` keyed by article slug:
`{ slug, scrollPct, lastReadAt, completed }`. A 3px progress bar fills from 0–100% as the
reader scrolls; persists across visits — returning to a 47%-read article restores position.
"Continue reading" strip on the home page for logged-in users: top 5 unfinished articles
by `lastReadAt`, with percent-complete ring.

Optional cross-device sync: when logged in, `wiki-read-state` is POSTed to the server
on `pagehide` and pulled on login. No third-party service required.

### The product framing

"Wikipedia where every sentence is Git-committed, every claim is planned for machine-queryable
verification, and AI agents are first-class readers but never the author of record."

The `query_claims(topic, asof)` MCP API (intended Phase 11) lets AI agents verify whether
a disclosure claim has changed since a given date — a capability no current documentation
platform offers. This is the planned commercial moat for regulated-industry customers.

---

## 12. Phase 7+ implementation backlog

In priority order. Each phase is gated on operator clearance after prior phase ships.

| Phase | Scope | Dependencies |
|---|---|---|
| **7B** — article toolbar split | `wiki_chrome` two-row header; article tabs row 40px; Tools dropdown; per-section anchor-share `¶` | Phase 7A live (binary rebuild) |
| **7C** — reading mode | "Reading Mode" button; CSS-only; `localStorage` persistence; ~70 lines total | 7B ships first |
| **7D** — citation hover preview | DOM-clone reference cards on `[N]` hover; freshness dot on each citation marker; `citations` redb table scaffold | Phase 4 `record_hash` already present |
| **7E** — mobile chrome | Sticky 56px top bar + sticky 56px bottom action bar; TOC bottom sheet; code-block overflow handling; `div.table-scroll` wrappers | 7B ships first; requires viewport breakpoint pass |
| **8** — history surface | Wikipedia-convention revision list; side-by-side diff with line highlight; radio buttons for arbitrary diff; `article-integrity-bar` SHA fingerprint | Phase 4 git2 + blake3 + redb already present |
| **9** — claim-rail freshness sidebar | Right rail at ≥1280px; `citations` redb table + nightly URL validator; `<aside class="claim-rail">` server emit | Phase 7D (citations table) ships first |
| **10** — reading state | Progress bar 3px; `localStorage` position; "Continue reading" home strip (logged-in); optional server-side sync | 7B ships first |
| **11** — `query_claims(topic, asof)` | MCP API extension: given a claim ID and a date, return the resolved claim state at that point in the blake3 hash chain | Phase 9 (citations table + hash chain) ships first |
| **7F** — Tufte sidenotes (JOURNAL) | Marginal note rendering for `layout: journal` articles at ≥1280px; collapse to inline expanders below; serves J1–J6 programme | 7B ships first |
| **7G** — corporate frontmatter blocks | `effective_date:` / `supersedes:` block under `h1`; cite format expanders; suppress feedback widget on corporate | 7B ships first |
| **7H** — corporate auto-numbered sections | CSS `counter-reset/increment` for `h2`/`h3` on `[data-instance="woodfine-corporate"]`; opt-out via `numbered_sections: false` frontmatter | 7G ships first |
| **12** — AI marginalia | Opt-in "Summarize section ▾"; `service-slm` integration; ephemeral overlay; SYS-ADR-07/10/19 compliant | Gated on BP5 + SYS-ADR review |

**Gated items (not yet sequenced):**
- Phase 5.1+ ACLs / OIDC SSO / webhooks — gated on BP5 clearance
- Phase 6 three-instance deployment split — gated on GitHub renames + Doctrine amendment (see §4)
- Phase 6B DID identity (`did:web:` + WebFinger) — needs BP6 design decision

---

## 13. Key files

| File | Role |
|---|---|
| `app-mediakit-knowledge/src/server.rs` | Main HTTP handler, routing, AppState (5,121 lines) |
| `app-mediakit-knowledge/src/links.rs` | redb wikilink graph, blake3 hashes, `LinkGraph` |
| `app-mediakit-knowledge/src/mcp.rs` | MCP JSON-RPC 2.0 server (native, ~330 lines) |
| `app-mediakit-knowledge/src/auth.rs` | Phase 5: cookie sessions, auth extractors (428 lines) |
| `app-mediakit-knowledge/src/pending.rs` | Phase 5: edit review queue (505 lines) |
| `app-mediakit-knowledge/src/claim.rs` | Claim model + extractor |
| `app-mediakit-knowledge/static/style.css` | Main stylesheet (2,554 lines post-7A) |
| `app-mediakit-knowledge/static/wiki.js` | Client JS — AJAX nav, TOC, search autocomplete, hover cards |
| `app-mediakit-knowledge/scripts/dtcg-bundle.json` | DTCG canonical token vault |
| `app-mediakit-knowledge/scripts/dtcg-to-css.py` | Token → CSS generator |
| `app-mediakit-knowledge/static/tokens.css` | Generated token CSS (157 tokens, all `oklch()`) |
| `app-mediakit-knowledge/static/tokens-woodfine.css` | Woodfine brand override tokens |
| `app-mediakit-knowledge/ARCHITECTURE.md` | Phase plan, ADRs, status snapshot |
| `app-mediakit-knowledge/NEXT.md` | Open items (last updated 2026-05-28) |
| `app-mediakit-knowledge/openapi.yaml` | OpenAPI 3.1 spec (1027 lines) |
| `.agent/briefs/BRIEF-knowledge-platform.md` | Archived predecessor to this brief |
| `.agent/drafts-outbound/DESIGN-WIKI-REDESIGN-SPEC.draft.md` | Round-2 jury spec (416 lines) |
| `wireframe-home-header-v2c.html` | Ratified header/footer pattern (canonical footer text source) |

**Content repos (live, reading from Totebox clone paths pending Phase 6 gates):**
- `content-wiki-documentation/` — ~220 English topics, 10 categories
- `content-wiki-projects/` — projects instance content
- `content-wiki-corporate/` — corporate instance content

---

*This brief supersedes `BRIEF-knowledge-platform.md` (status: archived, 2026-05-28).*
*Consolidated from: BRIEF-knowledge-platform.md + BRIEF-award-winning-wiki-overhaul.md +*
*BRIEF-MASTER_STRATEGY_AWARD_WINNING_WIKI.md + BRIEF-overhaul-documentation-pointsav-com.md +*
*BRIEF-institutional-chrome-sprint.md + three-agent internet research swarm (2026-05-28).*
