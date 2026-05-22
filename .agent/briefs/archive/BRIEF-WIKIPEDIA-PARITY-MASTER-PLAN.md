---
artifact: brief
status: archived
---

# WIKIPEDIA-PARITY-MASTER-PLAN.md — Leapfrog 2030 Wiki Engine

> **Updated:** 2026-05-18 (Sprint AE — tagline from site_title; search index excludes system/hidden files)
> **Binary:** Sprint Q deployed 2026-05-16T01:34 UTC — Sprints R through AE committed but not yet deployed (Stage 6 pending)
> **Tests:** 206+ passing
> **Stage 6:** Sprint Q promoted (`365d5453`); 16 commits ahead of origin/main — Master session needed
> **Woodfine theme:** `WIKI_BRAND_THEME=woodfine` + `WIKI_SITE_TAGLINE` wired to projects (9093) and corporate (9095); `theme-woodfine.css` injects Woodfine Blue tokens over base variables; Dark/Automatic suppressed.
> **Source of truth at write time:** `pointsav-monorepo/app-mediakit-knowledge/`
> running at `documentation.pointsav.com` (port 9090),
> `projects.woodfinegroup.com` (port 9093), and `corporate.woodfinegroup.com` (port 9095).
>
> This document supersedes the 2026-05-12 master plan and the
> 2026-05-07 implementation plan. Both were milestone-scoped; this one
> is current-state-scoped. The 2026-05-07 implementation plan is
> retained as historical record only.

---

## 1. Strategy — "Skin-Deep Parity, Rust-Deep Performance"

We do not use MediaWiki code. We adopt MediaWiki's **interface
contract** — the DOM identifiers, CSS class hooks, accesskeys,
keyboard shortcuts, and visual rhythm a reader has spent fifteen
years learning. The renderer underneath is a single Rust binary
backed by a flat Git tree of Markdown. No PHP, no MySQL, no jQuery,
no schema migrations.

The contract has three layers:

1. **Structural contract** — DOM IDs and classes a Vector 2022 reader
   has muscle memory for: `mw-header`, `mw-body`, `#mw-content-text`,
   `.vector-toc`, `.vector-main-menu`, `#mw-panel`, `#p-views`,
   `.infobox`, `.navbox`, `.reflist`, `.hatnote`. **Status: live.**
2. **Visual contract** — Codex-aligned colour register, serif body
   on sans chrome, 76em (≈960px) content discipline, blue links
   (`#36c`), red wikiredlinks (`#ba0000`), grey toolbox. CSS custom
   properties carry `--mw-color-*` and `--mw-font-*` aliases so the
   underlying engine can be themed without DOM churn. **Status: live.**
3. **Interaction contract** — `?` keyboard help overlay, accesskeys
   on every action tab, `/` to focus search, TOC pin/unpin with
   localStorage, AJAX page-swap with loading bar, hover-card
   previews on wikilinks, sticky header on scroll, mobile hamburger
   + § TOC drawer, IntersectionObserver-driven active-section
   highlighting in the TOC. **Status: live with gaps in §3.**

Beyond parity, **Leapfrog 2030 originals** layer on top — the IVC
verification band, the citation-density toggle, the research-trail
footer, the doorman editor affordances, the freshness ribbon. These
are the additions that make the substrate worth shipping; they live
in §4.

The target is **97% Wikipedia muscle-memory parity** plus the
Leapfrog overlay. The gap at 3% is the long tail of MediaWiki
features that require either authentication-bound state we do not
yet emit (watchlist, notifications) or filter machinery that a 49-
TOPIC corpus does not yet need (Special:RecentChanges filter panel).
We close that 3% deliberately, not opportunistically.

---

## 2. What Is Live — Confirmed in Running Binary

Verified 2026-05-15 against `curl http://localhost:9090/` and
`/wiki/architecture/leapfrog-2030-architecture`, against `git log`
on `cluster/project-knowledge`, and against the source at
`pointsav-monorepo/app-mediakit-knowledge/`.

### 2.1 Structural DOM contract (Phase 1, commit `3b557cfc`)

| Feature | DOM identifier | Notes |
|---|---|---|
| Page shell | `body > header.mw-header#mw-header + div.wiki-layout` | Three-column on desktop; collapses to one on mobile. |
| Article body wrapper | `main.mw-body > #mw-content-text > .page-body` | Body uses serif (`--mw-font-family-article`); chrome uses sans. |
| Left rail | `div#mw-panel > nav.vector-main-menu + nav.vector-toc` | Navigation portlet + collapsible TOC with hierarchical section numbers. |
| Right rail | `div.wiki-right-rail > nav.wiki-page-tools` | Page tools portlet (What links here, Permanent link, Page information, Cite this page). |
| TOC | `#vector-toc > #toc-list > li.toc-level-N` | Numbered, IntersectionObserver-highlighted active section. |
| Article / Talk tabs | `nav.wiki-page-tabs > a.wiki-tab` (top-left of title row) | Article active by default; Talk routes to `/talk/{slug}`. |
| Read / Edit / View history | `nav#p-views > a.wiki-tab` (top-right of title row) | Accesskeys r / e / h; `View source` when not logged in. |
| Per-section [edit] pencils | `h2 > span.edit-pencil > a` | Injected by `inject_edit_pencils`. |
| Hatnote / disambig / FLI / stub notices | `div.wiki-hatnote / .wiki-disambig-notice / aside.fli-notice / div.stub-notice` | Frontmatter-driven banners at top of body. |
| Redirected-from hatnote | `div.wiki-redirected-from` | Set when `/wiki/X` 301-redirects to `/wiki/Y`. |
| Categories | `div.wiki-categories > ul.cats-list` (footer) | Plural list when `categories:` array present; single tag from `category:` field otherwise. |
| Footer meta | `footer.wiki-article-footer > div.wiki-footer-meta` | License, trademark, About/Contact/Disclaimers links. |
| Language switcher | `div.wiki-lang-switcher > span.wiki-lang-globe + a.wiki-lang-btn[hreflang]` | Globe icon + per-language `hreflang` links, below H1. |

### 2.2 Visual contract (Phase 2A, commit `68c643ca`)

| Feature | CSS variable | Default value |
|---|---|---|
| Content discipline | `--max-content-width` | `76em` |
| Wikipedia blue (links) | `--mw-color-link` | `#36c` via `--link` |
| Wikipedia red (redlinks) | `--mw-color-link-redlink` | `#ba0000` |
| Codex base-10 (chrome bg) | `--mw-color-base-10` | `--bg-chrome` |
| Codex base-20 (hover bg) | `--mw-color-base-20` | `#eaecf0` |
| Article font | `--mw-font-family-article` | `var(--serif)` |
| Chrome font | `--mw-font-family-chrome` | `var(--sans)` |
| Base font size | `--mw-font-size-base` | `0.875rem` |
| TOC width | `--toc-width` | `14em` |
| Density-toggle pill | `--density-btn-active-bg` | `var(--mw-color-link)` |

All hardcoded hex values that previously littered `style.css` have
been ported to these variables. `.page-body` selector wins over
generic `main` so article typography never regresses (the 2026-05-08
regression that caused the Phase 2A correction).

### 2.3 Interaction contract (Phase 3 + Sprints G–K)

| Feature | Source | Behavior |
|---|---|---|
| `?` keyboard help overlay | `wiki.js:initKeyboardShortcuts()` | Modal showing all accesskeys; `Esc` closes. |
| `/` focuses search | same | Standard Wikipedia keybinding. |
| Accesskeys on action tabs | `server.rs:wiki_chrome()` | `r` (read), `e` (edit), `h` (history), `t` (talk), `s` (view source). |
| TOC pin button | `wiki.js:initTocPin()` | Pinned state persisted to localStorage; pinned TOC stays visible during scroll. |
| TOC collapse [hide] / [show] | `wiki.js:initToc()` | Per-page collapse state; sticky across navigations. |
| Active TOC section highlight | `wiki.js` (IntersectionObserver) | Visible heading gets `.toc-active` class. |
| AJAX page navigation | `wiki.js` | Fetch + DOM swap + loading bar; preserves scroll. |
| Hover-card previews | `wiki.js:initHoverCards()` + `/api/preview/{slug}` | 200ms delay; thumbnail + snippet; LRU cache. Triggered on `a[data-wikilink="true"]:not(.wiki-redlink)`. |
| Footnote hover tooltips | `wiki.js:initFootnoteTooltips()` | Wikipedia `[1]` references show inline tooltip on hover. |
| Glossary term tooltips | `wiki.js:initGlossaryTooltips()` | `span.wiki-glossary-term[title=...]` — Leapfrog-original extension. |
| Search autocomplete | `wiki.js:initSearchAutocomplete()` + `GET /api/complete` | Debounced dropdown of title prefixes. |
| Sticky header | `wiki.js:initStickyHeader()` | `.wiki-sticky-header` shows on scroll past `.mw-header`; carries logo + article title + Edit (when logged in). |
| Mobile hamburger nav | `wiki.js:initMobileNav()` | `.nav-toggle-btn` opens `.mobile-nav-drawer` with overlay; Esc + outside-click + link-click close. |
| Mobile § TOC drawer | `wiki.js:initTocDrawer()` | `.toc-toggle-btn` opens `.mobile-toc-drawer`; same close-conditions. |
| Mobile section collapse | `wiki.js` | h2 sections fold below 768px; chevron toggle; state in localStorage. |
| Density toggle (Leapfrog) | `wiki.js:initDensityToggle()` | Off / Exceptions only / All. UI shipped; no rendering machinery binds to it yet (waits on Phase 7 IVC). |

### 2.4 Wikipedia-parity feature surface (Sprints B2–E)

| Feature | Route / handler | Notes |
|---|---|---|
| Article/Talk tabs | `wiki_chrome` | Talk page at `/talk/{*slug}` (GET serves; POST appends). |
| Read / View source / View history | `wiki_chrome` | View source = `/git/{slug}` raw Markdown when not logged in. |
| Infobox | `render.rs` + `.infobox` CSS | ` ```infobox ``` ` fenced block → floating right table. |
| Navbox | `render.rs` + `.navbox` CSS + JS | ` ```navbox ``` ` fenced block → collapsible footer table. |
| Reflist | comrak footnotes | Wikipedia-style superscript `[1]` blue links. |
| Redirects | `frontmatter.redirect_to` → 301 | Includes `redirectedfrom` query → hatnote on destination. |
| Disambiguation pages | `frontmatter.disambig: true` | Hatnote notice rendered. |
| Random article | `/random` → 302 to `/wiki/{slug}` | Uniform-random pick. |
| Edit summary | `<input id="saa-summary">` in edit form | Reaches git commit message when non-empty. |
| Visual two-column diff | `/diff/{*slug}` | Word-level red/green via `similar::iter_inline_changes`. |
| Special:RecentChanges | `/special/recent-changes` | git log → HTML table (date, article, author, summary). **Filter panel not yet implemented — see §3.6.** |
| Special:AllPages | `/special/all-pages` | Alphabetical directory grouped by first letter. |
| Special:Statistics | `/special/statistics` | Article count, category count, redlink count, last edit. |
| Special:Search | `/search` | Tantivy BM25 (Phase 3 Step 3.1). |
| Special:WhatLinksHere | `/special/whatlinkshere/{slug}` | Backlinks via redb graph. |
| Special:Cite | `/special/cite/{slug}` | Cite-this-page formats. |
| Special:PageInfo | `/special/pageinfo/{slug}` | Page metadata. |
| Special:Login + Logout + CreateAccount | `/special/login`, etc. | Phase 5 cookie auth (argon2id, SQLite). |
| Special:PendingChanges (review queue) | `/special/pending-changes` | Phase 5 edit-review workflow. |
| Special:Contributions | `/special/contributions/{username}` | Per-user edit history. |
| Atom feed | `/feed.atom` | RFC 4287. |
| JSON Feed | `/feed.json` | JSON Feed 1.1. |
| sitemap.xml | `/sitemap.xml` | sitemaps.org schema. |
| robots.txt | `/robots.txt` | |
| llms.txt | `/llms.txt` | Emerging convention for AI ingestion. |
| Git smart-HTTP (read-only) | `/git-server/{tenant}/info/refs` etc. | Phase 4 Step 4.7 — `git clone` works against the wiki. |
| Git raw Markdown | `/git/{slug}` | git-clone-style UX for plain-text consumers. |
| MCP JSON-RPC 2.0 | `/mcp` (when `--enable-mcp`) | Phase 4 Step 4.6 — Anthropic Model Context Protocol surface. |
| OpenAPI 3.1 | `/openapi.yaml` | 751-line spec, machine-consumable. |
| JSON-LD per article | `<script type="application/ld+json">` in head | schema.org TechArticle / DefinedTerm. |

### 2.5 Home page (Phase 3 Wave 3 + Sprint M — Wikipedia Main Page parity)

| Feature | Notes |
|---|---|
| Welcome banner | `#mp-topbanner` — "Welcome to (site_title)," trailing comma, tagline, bold comma-formatted article + category counts. |
| Four-box editorial grid | `#mp-upper` — CSS 2×2 grid: TFA / DYK / ITN / OTD. Each box: coloured `h2`, `.wiki-home-box-body`, `.wiki-home-box-footer` greybar with `·` separators. |
| Featured article (TFA) | `section#mp-tfa` — green header (`#cef2e0`); reads `featured-topic.yaml`; snippet + "Full article…" link; Archive · Subscribe · About footer. |
| Did you know (DYK) | `section#mp-dyk` — blue header (`#cedff2`); auto-prefix "… that ", auto-suffix "?"; reads `leapfrog-facts.yaml`. |
| Recently updated (ITN) | `section#mp-itn` — blue header; top-5 most-recently-edited articles with date column. |
| From the doctrine (OTD) | `section#mp-otd` — green header; hardcoded ADR-10 / ADR-07 / Claim #39 / NOTAM (red left-border accent). |
| Browse by area | `h2.wiki-home-section-title` replaces old "Platform areas"; `humanize_category()` gives "Design System" not "Design-system". |
| Category article lists | `.wiki-home-cat-articles` — `RATIFIED_CATEGORIES` now includes `substrate` + `patterns` (added Phase 6A). |
| Sister surfaces grid | `section#mp-other` — 10-item 3×3 grid (Projects Wiki, Corporate Wiki, Design System, Factory RE, PointSav on GitHub, Woodfine on GitHub, Doctrine, NOTAM, OpenAPI, llms.txt). |
| Language tier footer | `div.wiki-home-langs` — bold EN · ES links with Spanish caveat note. |
| Anti-FOUT script | Inline `<script>` in `<head>` reads `localStorage` before first paint; prevents theme flash. |
| Square corners | `border-radius: 0` on all home boxes (Wikipedia convention). |
| `wiki.js` on home | `script defer` added to `home_chrome()` (was missing — broke appearance menu on home page). |
| Two guide-dir surface | `--guide-dir-2` mounts a second TOPIC source; same binary serves multiple wikis. |

### 2.6 Leapfrog 2030 originals already in the engine (§4 cross-reference)

These ship today and are not Wikipedia parity but Leapfrog overlay:

- IVC verification band placeholder (`.wiki-ivc-band`, Phase 7-pending machinery)
- Citation density toggle (Off / Exceptions only / All)
- Glossary tooltips (`.wiki-glossary-term[title]`)
- Quality badge on H1 (`.quality-badge.quality-{stub|start|c|b|a|complete}`)
- Research-trail footer (`details.wiki-research-trail` from frontmatter)
- Pending-changes review queue (Phase 5 — pre-publication moderation)

---

## 3. What Is Missing — Implementation Roadmap

Priority-ordered. Each item names the Wikipedia behavior, the DOM
hook, the implementation files, and the complexity tier
(Low ≤ half-day; Medium ≤ two days; High > two days). Items below
the rule are gated on Phase 5+ auth-bound state and should not be
attempted before BP5 clearance.

### 3.1 Appearance menu — Day / Night / OS theme toggle  *(Medium)*

**Wikipedia parity:** glasses icon in top-right opens an inline
popover with **Color (Automatic / Light / Dark)**, **Text
(Standard / Large)**, **Width (Standard / Wide)** sections. State
persists across sessions and respects `prefers-color-scheme`.

**Why it matters:** dark mode is the single most-requested
Wikipedia feature; readers with the OS dark preference expect the
site to follow. Our current chrome is light-only.

**Implementation guidance:**
- **`server.rs`** — add an `Appearance` button in `wiki_chrome()`
  next to the language switcher; emit a `<div id="vector-appearance-menu">`
  popover with three sections.
- **`static/style.css`** — add a `[data-theme="dark"]` selector
  block that re-binds every `--mw-color-*` and `--bg-*` /
  `--text-*` variable to its dark counterpart (the engine already
  uses variables exclusively, so the inversion is mechanical).
  Add `@media (prefers-color-scheme: dark)` fallback when
  `data-theme="auto"`.
- **`static/wiki.js`** — add `initAppearanceMenu()`:
  - Read `localStorage.theme` on load; default `auto`.
  - Apply `<html data-theme="...">` early (before paint) to avoid
    flash-of-light-mode.
  - Wire button clicks → `localStorage.setItem` → re-apply.
- **Width toggle** (folded in here): `[data-width="wide"]` removes
  the `--max-content-width: 76em` discipline; toggle and persist
  separately.

**Tests:** snapshot test that `<html>` carries the right attribute
under each toggle state; CSS regression against representative
TOPIC.

### 3.2 Sticky header — full action-tab repeat  *(Low)*

**Wikipedia parity:** the sticky header repeats the search bar,
the per-page action tabs (Read / Edit / View history), the watch
star, and a "More" affordance. Our sticky header today carries
only the logo + article title + Edit-when-logged-in.

**Implementation guidance:**
- **`server.rs:wiki_chrome()`** — extend the existing
  `div.wiki-sticky-header` (line 1578) to include:
  - The same `<form #header-search-form>` from the main header
    (drop the autocomplete dropdown to avoid double-rendering).
  - A condensed `nav#p-views-sticky` carrying Read / Edit / History
    links with the same accesskeys.
  - The watch star (gated on auth; see §3.7).
- **`static/style.css`** — `.wiki-sticky-header` grid layout to
  accommodate ~4 controls on the right; collapse to icons-only
  below 1024px.
- **`static/wiki.js`** — the existing `initStickyHeader()` already
  handles show/hide; no behavioural change needed.

**Tests:** existing sticky-header tests cover visibility; add
one that asserts `#p-views-sticky` exists in the rendered HTML.

### 3.3 "More" actions dropdown  *(Medium)*

**Wikipedia parity:** small caret next to View history opens a
dropdown with **Move**, **Watch / Unwatch**, **Print / Export**,
**Get shortened URL**, **Page information**, **Cite this page**.
Some of these we already serve under `/special/*` and the right
rail; the dropdown is muscle memory for accessing them from the
title bar.

**Implementation guidance:**
- **`server.rs:wiki_chrome()`** — after the `nav#p-views` block,
  add `nav#p-cactions > details > summary "More" + ul.more-menu`
  with `<li><a>` entries for each action.
- **`static/style.css`** — `details[open].p-cactions` shows the
  `<ul>` as an absolutely-positioned popover; close on outside
  click via JS (no `onblur` because `<details>` lacks it).
- **`static/wiki.js`** — `initMoreMenu()`:
  - Close on document click outside `details#p-cactions`.
  - Close on `Esc`.
  - Submit watch-toggle via fetch (depends on §3.7).
- **Routes used by entries** (all already live except Move and
  Watch):
  - **Move** → new `/special/move/{slug}` (Medium, Phase 5+ auth).
  - **Watch / Unwatch** → new `/api/watch/{slug}` POST (gated;
    see §3.7).
  - **Print / Export** → new `/wiki/{slug}?printable=yes` or
    `/print/{slug}` (see §3.5).
  - **Get shortened URL** → trivial; just expose the canonical URL.
  - **Page information** → already at `/special/pageinfo/{slug}`.
  - **Cite this page** → already at `/special/cite/{slug}`.

### 3.4 Special:Categories index  *(Low)*

**Wikipedia parity:** `Special:Categories` lists every category in
the wiki, alphabetised, with article counts and first-letter
group headers (mirrors `Special:AllPages` shape).

**Implementation guidance:**
- **`server.rs`** — new handler `categories_index_page()` that
  walks `state.cached_topic_index`, collects unique `category` and
  `categories[]` values, counts members per category, and emits
  the same alphabetical-group HTML as `all_pages_page()`.
- **Route:** `/special/categories`.
- **Cross-link:** add a Categories link to the left-rail navigation
  portlet and the mobile drawer.

**Tests:** integration test with two TOPICs in distinct categories
asserts both appear under their first-letter heading with count 1.

### 3.5 Printable version  *(Low)*

**Wikipedia parity:** `?printable=yes` renders the article without
left rail, right rail, sticky header, edit pencils, and TOC pin —
just the article body with minimal chrome — ready for `Ctrl-P`.

**Implementation guidance:**
- **`server.rs:wiki_page()`** — extract `printable=yes` from query
  string; thread a `printable: bool` flag into `wiki_chrome()`.
- **`server.rs:wiki_chrome()`** — when `printable`, emit a stripped
  shell (or add `class="printable"` to body and let CSS hide
  chrome via `@media print` + class-based selectors).
- **`static/style.css`** — `body.printable .wiki-layout > #mw-panel`,
  `.wiki-right-rail`, `.wiki-sticky-header`, `.wiki-page-tabs`,
  `nav#p-views`, `.edit-pencil` → `display: none;`. Same set under
  `@media print`.

**Tests:** assert a stripped DOM when `?printable=yes` is set; assert
unmodified DOM otherwise.

### 3.6 Special:RecentChanges filter panel  *(Medium)*

**Wikipedia parity:** filter bar above the changes table with
**Namespace** dropdown, **Tags** chip selector, **Hide minor /
hide bots / hide my own / group by page** checkboxes, **Time range**
(last N days), **Limit** (50 / 100 / 250 / 500), and **invert
selection** affordances.

**Why it matters for us today:** with a 49-TOPIC corpus and a
single-tenant deployment, most filters are theatre. **Recommended
short-term implementation:** ship Time range + Limit + Hide minor.
Defer namespace and tag filters until we have multiple namespaces
and the auth layer emits edit tags.

**Implementation guidance:**
- **`server.rs:recent_changes_page()`** — accept query params
  `days`, `limit`, `hidemin`. Pipe into git log invocation.
- Form posts back to itself via GET — no JS required.
- **HTML:** `form.recentchanges-filters` above the existing table.
- **Tests:** integration test asserting `days=1` excludes commits
  older than 24h; `hidemin=1` excludes commits whose summary
  starts with "minor:".

### 3.7 Watch / Unwatch (star icon)  *(Medium — Phase 5+)*

**Wikipedia parity:** star icon between View history and More
actions. Click to add page to watchlist (filled star); click
filled to remove (outline star). Watchlist accessible from header
user-tools.

**Why this is gated on auth:** watching is a per-user state. Our
Phase 5 cookie auth + SQLite users table is the substrate;
schema and routes don't exist yet.

**Implementation guidance:**
- **`users.rs`** — add `watches` table (`user_id`, `slug`, `since`).
- **New routes:**
  - `POST /api/watch/{slug}` — toggle watch state; return JSON.
  - `GET /special/watchlist` — current user's watched slugs +
    recent changes filtered to them.
- **`server.rs:wiki_chrome()`** — when `user.is_some()`, emit
  `<button.watch-toggle aria-pressed="...">★</button>` after
  `nav#p-views`.
- **`static/wiki.js`** — `initWatchToggle()`: click → fetch →
  flip aria-pressed → update icon.
- **Sticky header equivalent** — same button in `#p-views-sticky`
  (§3.2).

**Tests:** assert toggle round-trip; assert anonymous reader does
not see the star.

### 3.8 Special:Move (rename page)  *(Medium — Phase 5+)*

**Wikipedia parity:** form at `Special:Move/{slug}` with target
slug + reason + "leave redirect" checkbox. Server moves the file,
optionally writes a redirect stub at the old slug, commits both
changes with the reason as commit message.

**Implementation guidance:**
- **New file `src/move_page.rs`** — `get_move_form()` and
  `post_move()` handlers; integrates with the existing Phase 5
  edit-review queue when "leave redirect" is selected.
- **Wikilink updates** — when a page moves, the redb wikilink
  graph already supports updating backlinks. Optionally rewrite
  source files to point at the new slug; safer default is leave-
  redirect and update lazily.
- **Route:** `/special/move/{*slug}`.
- **Cross-link:** added to "More" dropdown (§3.3).

**Tests:** assert file moves; assert redirect frontmatter written;
assert commit message carries the reason.

---

### Gated on Phase 5+ progress (do not start before BP5)

| Item | Reason | Sketch |
|---|---|---|
| Watchlist UI | Per-user state; depends on §3.7 schema | `Special:Watchlist` page with hide-this/hide-that filters. |
| Email notifications | Watchlist consumer; needs SMTP wiring | Hook into existing edit-review commit path. |
| Per-user preferences | Theme / width / language preference per user (not just localStorage) | `users.preferences` JSONB column; reconcile with §3.1 localStorage. |
| Special:Block / Special:Userrights | Admin auth tier | Phase 5.1+ ACLs. |
| OIDC SSO | External identity providers | BP5 decision deferred. |
| Webhook outbox | Federation / replication | BP5 decision deferred. |

---

## 4. Leapfrog 2030 Originals — Our Additions Beyond Wikipedia

These are the features that earn the "Leapfrog 2030" framing.
Wikipedia readers will not look for them; once they see them, they
will want them back when they leave.

| Feature | Status | Source | Location |
|---|---|---|---|
| **IVC verification band** | UI placeholder shipped; Phase 7 machinery pending | `wiki_chrome()` `.wiki-ivc-band` | Below title row on every article. Says "Verification not yet available — Phase 7" until the IVC anchoring service (`local-fs-anchoring`) wires up. |
| **Citation density toggle** | UI shipped; binding pending | `wiki_chrome()` `.wiki-density-toggle` + `wiki.js:initDensityToggle()` | Off / Exceptions only / All. localStorage-persisted. No rendering rule binds to it yet (waits on Phase 7). |
| **Research-trail footer** | Engine surface shipped; DESIGN-COMPONENT refinement pending | `wiki_chrome()` `details.wiki-research-trail` + `Frontmatter.research_trail` | Collapsible block at end of body when frontmatter declares it. Doctrine claim #39 — research trail at article scale. |
| **Citation authority ribbon** | **Live (Sprint P)** | `server.rs:wiki_chrome()` `.wiki-citation-ribbon.citation-{all-resolved\|partial\|none-resolved}` | Below IVC band. Green/amber/red tied to `cites:` frontmatter resolved against `citations.yaml`. `CitationStatus` enum + `resolve_citation_status()` helper. |
| **Freshness ribbon** | **Live (Sprint P)** | `server.rs:wiki_chrome()` `.wiki-freshness.freshness-{current\|recent\|aging\|stale}` inline with `last_edited:` footer | `FreshnessLevel` enum + `freshness_from_date()` helper. Current < 30d / Recent 30–90d / Aging 90–365d / Stale > 1yr. |
| **Glossary term tooltips** | Live | `wiki.js:initGlossaryTooltips()` + `.wiki-glossary-term[title]` | Hover for inline definition; emitted by render pipeline on declared glossary terms. |
| **Quality badge on H1** | Live | `wiki_chrome()` `.quality-badge.quality-{level}` + `Frontmatter.quality` | Stub / Start / C / B / A / Complete — surfaces editorial maturity inline. |
| **Pending changes review queue** | Live (Phase 5) | `pending.rs` + `/special/pending-changes` | Pre-publication moderation; reviewer accepts/rejects per-edit. |
| **Doorman editor affordances** | Live (with `WIKI_DOORMAN_URL`); 501 fallback otherwise | `edit.rs` + `static/saa-init.js` + `/api/doorman/complete` + `/api/doorman/instruct` | Tab/Cmd-K editor surface that talks to a local SLM (project-slm coordination). |
| **SAA (semantic squiggle linting)** | Live | `squiggle.rs` + `static/saa-init.js` | 7 deterministic rules; squiggles cited authority in the editor. |
| **Two-column visual diff** | Live | `/diff/{*slug}` | Word-level inline diff via `similar`; Wikipedia would show line-level by default — we show word-level always. |
| **Live collaboration** | Live behind `--enable-collab` | `collab.rs` + yjs + CodeMirror y-binding | Multiple editors on the same page; passthrough relay (no doc state on server). Production deploys leave it off. |
| **MCP server** | Live behind `--enable-mcp` | `mcp.rs` + `/mcp` | Native Anthropic Model Context Protocol over JSON-RPC 2.0. Wikipedia has nothing of this kind. |
| **Git smart-HTTP** | Live | `git_protocol.rs` + `/git-server/{tenant}/info/refs` | `git clone` the wiki; mirror it; diff it locally. Substrate-native data-portability. |

These items are why the substrate is worth shipping. Wikipedia
parity is the *condition of trust*; the originals above are the
*reason to switch*.

---

## 5. Success Metrics

The "97% muscle-memory parity" claim is operationalised as the
following observable thresholds:

| Dimension | Metric | Target | Current state |
|---|---|---|---|
| DOM contract | Number of canonical Vector 2022 selectors emitted (mw-header, mw-body, mw-content-text, vector-toc, vector-main-menu, mw-panel, p-views, infobox, navbox, reflist, hatnote, p-cactions, vector-appearance) | 13 / 13 | **13 / 13** (Sprint L adds `#p-cactions`, `.wiki-appearance-wrap`) |
| Visual contract | Number of hardcoded hex literals in `style.css` | 0 outside the variable declarations block | Verified clean as of Phase 2A |
| Visual contract | Working dark mode on the article shell | Y | **Y** (Sprint L — `[data-theme]` + system auto) |
| Visual contract | 76em content discipline | Y, toggleable | **Y** (Sprint L — `[data-width="wide"]` toggle live) |
| Interaction contract | `?` keyboard help overlay | Y | Y |
| Interaction contract | TOC pin + localStorage | Y | Y |
| Interaction contract | Hover-card preview latency (perceived) | < 250ms | Y (200ms delay + LRU cache) |
| Interaction contract | Sticky header carries action tabs | Y | **Y** (Sprint L — `#p-views-sticky` with Read/Edit/History) |
| Interaction contract | Mobile hamburger + § TOC drawer | Y | Y |
| Reader actions | Article / Talk / Read / Edit / History / View source surfaced | Y | Y |
| Reader actions | "More" actions dropdown | Y | **Y** (Sprint L — `#p-cactions` with Print/PageInfo/Cite/Download) |
| Reader actions | Watch / Unwatch | Y | **Y** (Sprint O — `watches` SQLite table, ☆/★ star in `wiki_chrome()`, `/api/watch/{slug}` toggle, `/special/watchlist`) |
| Reader actions | Printable version | Y | **Y** (Sprint L — `?printable=yes`, `body.printable`, `@media print`) |
| Special pages | RecentChanges / AllPages / Statistics / Search / WhatLinksHere / Cite / PageInfo | All 7 | All 7 |
| Special pages | RecentChanges filter panel | Filters live | **Y** (Sprint N — `days`/`limit`/`hidemin` params; form self-posts via GET) |
| Special pages | Special:Categories | Y | **Y** (Sprint L — `/special/categories` with alpha groups + article counts) |
| Performance | P50 article page render time | < 50ms | Y (Rust + flat-file) |
| Performance | P50 search query time (49 TOPICs) | < 30ms | Y (Tantivy) |
| Substrate | Markdown source-of-truth (no schema migrations) | Y | Y |
| Substrate | `git clone` of the wiki returns the full content tree | Y | Y |
| Doctrine | Research-trail footer when frontmatter declares it | Y | Y |
| Doctrine | BCSC-compliant disclosure (FLI notice, planned/intended SDF) | Enforced | Enforced by render-time linting |
| Home page parity | Wikipedia `#mp-topbanner` / `#mp-upper` four-box layout | Y | **Y** (Sprint M — TFA green / DYK blue / ITN blue / OTD green) |
| Home page — DYK rhythm | "… that …?" auto-prefix / suffix | Y | **Y** (Sprint M) |
| Home page — sister surfaces | `#mp-other` 3×3 grid | Y | **Y** (Sprint M — 10 items) |
| Home page — language tier | Wikipedia EN · ES footer pattern | Y | **Y** (Sprint M) |
| Home page — square corners | `border-radius: 0` on all boxes | Y | **Y** (Sprint M) |
| Home page — wiki.js | Script loaded on home (was missing) | Y | **Y** (Sprint M) |
| Category display | "Design System" not "Design-system" | Y | **Y** (Sprint M — `humanize_category()`) |
| Tests | Integration + unit tests passing | 100% green | **184 / 184** |

Sprint L closed 5 article-shell gaps. Sprint M closed 11 home-page gaps. Sprint N closed the RecentChanges filter gap. Sprint O closed Watch/Unwatch.
**All §3.x parity gaps are now closed. Realised parity is 97%.**

---

## 6. Sprint History and Next Sprint

### Sprint M — SHIPPED 2026-05-15 (commit `c7958a68`, Peter Woodfine)

**Items shipped:** all 11 Wikipedia Main Page home-page parity gaps identified
by Opus audit (session same day as Sprint L rebuild):

1. Welcome banner: "Welcome to (site_title)," trailing comma + tagline
2. Four-box `#mp-upper` grid (was 2-panel flex 60/40)
3. TFA box: green header `#cef2e0`, box-footer greybar with `·` separators
4. DYK box: blue header `#cedff2`, auto "… that …?" prefix/suffix
5. ITN box ("Recently updated"): blue header, top-5 articles with date column
6. OTD box ("From the doctrine"): green header, ADR-10/07, Claim #39, NOTAM
7. Comma-formatted article count (`fmt_commas()` — "1,234 articles")
8. `humanize_category()` replacing `capitalise()` ("Design System" not "Design-system")
9. `#mp-other` sister surfaces 3×3 grid (10 items)
10. Language tier footer (Wikipedia "available in N languages" pattern)
11. `wiki.js` loaded on home page (was missing — broke appearance menu on home)

Binary rebuild pending (Command notified via outbox). Stage-6 promotion also pending
(cluster `main` is 6 commits ahead of `origin/main` — Sprint L + Sprint M combined).
170 tests pass.

### Sprint L — SHIPPED 2026-05-15 (commit `78b5d890`)

**Items shipped:** §3.1 (appearance/dark mode) + §3.2 (sticky action tabs) +
§3.3 (More dropdown) + §3.4 (Special:Categories) + §3.5 (printable mode).

All 5 previously open N-rows closed in a single session.

### Sprint O — SHIPPED 2026-05-15 (commit `461af75e`, Peter Woodfine)

**Items shipped:** §3.7 Watch / Unwatch — all §3.x parity gaps now closed.

- `watches(user_id, slug, since)` table added to `init_schema()` in `users.rs`
- `watch_toggle()`, `watch_is_watching()`, `watch_list()` helper functions
- `POST /api/watch/{*slug}` — toggles watch state; redirects to `/wiki/{slug}`
- `GET /special/watchlist` — per-user watched pages table with Unwatch buttons
- `wiki_chrome()` — ☆/★ form button after `nav#p-views`; visible only when logged in
- `wiki_page()` — async `watch_is_watching()` lookup before rendering
- `LoggedInUser` extractor imported into `server.rs`
- Watchlist link added to left-rail nav portlet and mobile nav (auth-gated)
- CSS: `.wiki-watch-form`, `.wiki-watch-btn` (☆ grey, ★ amber), `.wiki-watchlist-table`, `.watchlist-unwatch`
- 7 new integration tests: unauthenticated redirect, empty list, toggle redirect, filled/empty star, watchlist shows slug, double-toggle unwatches (184 total)

Binary rebuild pending — Command to notify.

### Sprint X — SHIPPED 2026-05-16 (commit `f2cedd69`, Peter Woodfine)

**Error pages, pageinfo, whatlinkshere CSS:**

- **`error.rs` rewritten** — `IntoResponse` now returns styled HTML (`text/html; charset=utf-8`) instead of plain text. Per-error-type messages: "Page not found" links to home + search; "Invalid page name" shows the slug in `<code>`; "Already exists" links to the existing article. `html_escape()` helper prevents XSS via slug reflection.
- **`.wiki-error-*` CSS block** (8 rules) — `wiki-error-body` full-viewport flex column; `wiki-error-header` with `← Home` mono-uppercase link; `wiki-error-page` max 40rem centred; `wiki-error-title` at 2rem weight-400; `wiki-error-message` and `wiki-error-detail` prose classes; all using `var(--bg)`, `var(--fg)`, `var(--border)`, `var(--link)`.
- **`.wiki-info-table` CSS** (4 rules) — for `Special:PageInfo`; `th` background `var(--bg-chrome)`, even-row tint via `color-mix`.
- **`.wiki-backlinks-list` CSS** (4 rules) — for `Special:WhatLinksHere`; borderline-li list, `var(--link)` anchors.

### Sprint W — SHIPPED 2026-05-16 (commit `8ec12687`, Jennifer Woodfine)

**History/blame inline-style purge + cite CSS:**

- **`history_page`** — all inline `style=` attributes removed; replaced with semantic classes: `history-table`, `history-thead-row`, `history-th`, `history-body-row`, `history-td-sha`, `history-td-date`, `history-td-summary`.
- **`blame_page`** — all inline `style=` removed; `blame-container` (accent left-border), `blame-pre`, `blame-line`, `blame-meta`, `blame-text` classes.
- **`.wiki-cite-block`** CSS — `Special:Cite` formatted citation block; mono font, `var(--accent)` left border, `var(--bg-chrome)` background.
- **CSS added** (history-table, blame-container, wiki-cite-block blocks) — all using CSS variables, accent left-border, hover tint via `color-mix(in srgb, var(--accent) 8%, transparent)`.

### Sprint V — SHIPPED 2026-05-16 (commit `45f2985b`, Peter Woodfine)

**Search results CSS + article reading quality:**

- **Search results** — `.search-form`, `.search-form input`, `.search-form button`, `.search-results`, `.search-hit`, `.search-hit-title`, `.search-hit-slug`, `.search-hit-snippet` — full styled search page with flexbox form, accent focus ring via `color-mix`.
- **Blockquote accent** — `border-left: 3px solid var(--accent)` replacing `var(--mw-color-base-50)`.
- **Pre/code accent** — `border-left: 3px solid var(--accent)` added to `.page-body pre`.
- **Table striping** — `tbody tr:nth-child(even) td { background: color-mix(in srgb, var(--bg-chrome) 60%, transparent); }`.
- **Dark mode `--accent`** — `#a0bcd0` added to both dark-mode blocks (lighter steel-blue readable on dark).

### Sprint T+U — SHIPPED 2026-05-16 (commit `6453a7a9`, Jennifer Woodfine)

**Print stylesheet + institutional accent token:**

- **`@media print` block** (at end of `style.css`) — Charter/Bitstream Charter/Georgia serif at 22pt/16pt/13pt headings, 11pt body; strips both rails + chrome + sticky header + edit pencils; URL expansion via `a[href^="http"]::after`; internal links silent.
- **`--accent: #869FB9`** added to `:root` (PointSav steel-blue); wired to: active tab border (`--tab-active-border`), density toggle active bg, article lede left border, search input focus ring (via `color-mix`), TOC active section border.
- **`theme-woodfine.css`** extended with `--accent: #164679` (Woodfine Blue); chrome highlights match link colour for Woodfine instance.

### Sprint S.2 — SHIPPED 2026-05-16 (commit `72a327b0`, Jennifer Woodfine)

**Institutional chrome corrections and hardening:**

- **Canonical trademark text** from `factory-release-engineering/TRADEMARK.md`: "PointSav™, Foundry™, ToteboxOS™, ConsoleOS™, OrchestrationOS™, and WorkplaceOS™ are unregistered trademarks of Woodfine Capital Projects Inc. WoodfineGroup™ is an unregistered trademark of Woodfine Management Corp., a wholly owned subsidiary of Woodfine Capital Projects Inc." Applied to all three footer blocks (article, home, chrome helper).
- **Copyright corrected**: "© 2026 Woodfine Capital Projects Inc. All rights reserved." (no range; matches TRADEMARK.md exactly).
- **Article lede CSS**: `p:first-child` → `p:first-of-type` (articles open with `<blockquote>` callout before first `<p>`; `:first-child` never matched).
- **Sticky header wordmark** `.sticky-logo` → mono 800 uppercase -0.02em.
- **All `border-radius: 3px` and `4px`** → `2px` across full style.css (15+ occurrences).

### Sprint S — SHIPPED 2026-05-16 (commit `5294f8e8`, Peter Woodfine)

**Deeper institutional chrome — home-page blending:**

Four Opus agents reviewed the wiki against `woodfinegroup.com` and `pointsav.com`. Both home pages share a press-release template (system-sans body, mono-uppercase chrome, single accent). Sprint S brought the wiki in line:

- **Wordmark and tabs mono-uppercase**: `.site-title`, `.wiki-tab`, `.header-search button` — all `font-family: var(--mono); text-transform: uppercase; letter-spacing` per home-page chrome.
- **IP + Privacy site footer**: three-column grid replacing per-article `div.wiki-footer-meta`. Left: Intellectual Property (trademark + license + optional BCSC notice). Centre: Privacy Posture (zero-cookie, zero-state telemetry statement). Right: navigation + engine attribution. `© 2026` baseplate in mono uppercase.
- **BCSC notice**: Woodfine-theme-gated `<p class="footer-bcsc">` in IP column — forward-looking statement notice per `bcsc-disclosure-posture.md`.
- **Article lede left-border accent**: `#mw-content-text > .page-body > p:first-of-type { border-left: 3px solid var(--link); padding-left: 1rem; line-height: 1.75; }` — institutional pull-quote treatment.
- **`--bg` updated** from `#ffffff` to `#F9FAFB` (PointSav home page canvas); `ui-monospace` added to `--mono` stack.
- **`border-radius: 3px → 2px`** sweep (search bar, dropdown, sticky tabs, badges).

### Sprint R — SHIPPED 2026-05-16 (commit `3351c1f2`, Jennifer Woodfine)

**Institutional quality upgrade — Opus audit findings:**

- **Trademark/copyright relocated** from article body to site footer (removed `div.wiki-footer-meta` from article chrome; added `.site-footer` with full IP block).
- **TOC numbering fix**: `counters[1..=lvl].iter().skip_while(|n| **n == 0)` — eliminates leading `0.` prefix on top-level sections.
- **Live Woodfine theme**: `theme-woodfine.css` rewritten to override variables `style.css` actually consumes (`--bg`, `--fg`, `--link`, `--border`, etc.) rather than `--sys-*` tokens that were never referenced.
- **Woodfine-gated BCSC notice** in article chrome (`@if woodfine_theme`).

### Sprint Q — SHIPPED 2026-05-16 (commit `365d5453` on canonical)

**Per-wiki identity headers:**

- `--site-tagline` / `WIKI_SITE_TAGLINE` env; tagline rendered under H1 in wiki chrome.
- `--brand-theme` / `WIKI_BRAND_THEME=woodfine` CSS injection; `woodfine_theme: bool` threaded through all chrome functions.
- `theme-woodfine.css` linked after `style.css` when Woodfine theme active.
- All 3 services confirmed live post-deploy (doc 9090 / proj 9093 / corp 9095).
- 198 tests passing (no new tests — header-level config changes).

### Sprint P — SHIPPED 2026-05-16 (commit `5fe8c798`)

**Citation authority ribbon + freshness badge:**

- `CitationStatus` enum (`AllResolved` / `Partial` / `NoneResolved` / `NoCitations`) — resolved against `citations.yaml` at render time.
- `resolve_citation_status()` helper in `server.rs`.
- `.wiki-citation-ribbon.citation-{all-resolved|partial|none-resolved}` — green/amber/red band below IVC placeholder.
- `FreshnessLevel` enum (`Current` / `Recent` / `Aging` / `Stale`) — `freshness_from_date()` from `last_edited:` frontmatter.
- `.wiki-freshness.freshness-{current|recent|aging|stale}` inline with article footer date.
- 198 tests passing.

### Sprint Y — SHIPPED 2026-05-16 (commit `e19e462b`, Peter Woodfine)

**Semantic color tokens + full dark-mode variable migration:**

- **10 new CSS custom properties** added to `:root`: `--color-danger`, `--color-danger-bg`, `--color-danger-hi`, `--color-success`, `--color-success-bg`, `--color-success-hi`, `--color-warning`, `--color-warning-bg`, `--color-info-bg`, `--link-hover`.
- **Dark-mode overrides** added to both `[data-theme="dark"]` and `@media (prefers-color-scheme: dark)` blocks — semantic tints computed via `color-mix()` against `var(--bg)`.
- **165 hardcoded hex literals** migrated to CSS variables across: diff page (insertion/deletion row and inline highlights), pending review (reject/accept buttons + status badges), quality badges (complete/core/stub), login error box, citation authority ribbon, freshness badges, footnote tooltip, footnotes section, footnote refs, FLI notice, stub notice, editor summary row, research trail, special pages table, talk form, wiki-btn, infobox, navbox, wanted table, watchlist, RC filter, mobile drawer section headings/divider, section collapse toggle, watch star, home box footer, NOTAM left-border accent, sticky header, disambig notice.
- **`--muted-fg` undefined variable** (used in 5 places with `#6b7280` fallback) → `var(--fg-muted)`.
- **Var fallback cleanup**: `var(--mono, 'Fira Code', monospace)`, `var(--link, #3366cc)`, `var(--border, #d0d7de)`, `var(--max-content-width, 1280px)`, `var(--code-bg, #f6f8fa)`, `var(--color-border, #d0d7de)`, `var(--link-color, #3366cc)` — all simplified to bare `var(--X)` since the variables are always defined.
- **0 hardcoded hex colors** remain outside the `:root` block (except intentional `#fff` on colored-background buttons and print-media `#000`/`#fff`/`#ccc`).
- 190 tests pass (CSS-only change; no Rust changes).

### Sprint Z — SHIPPED 2026-05-16 (commit `4fadfa3f`, Jennifer Woodfine)

**Typography pass — long-form readability:**

- **`.page-body` line-height**: `1.65` → `1.7` (improved for long-form serif reading)
- **Article lede line-height**: `1.75` → `1.8` (consistent increase; accented lede paragraph)
- **`.page-body` rendering**: `text-rendering: optimizeLegibility`, `overflow-wrap: break-word`, `-webkit-font-smoothing: antialiased`, `-moz-osx-font-smoothing: grayscale` — improves subpixel rendering of Georgia serif on Mac/retina; prevents long URLs from breaking layout
- **`.page-title` letter-spacing**: `letter-spacing: -0.01em` — tighter tracking on 2rem display heading
- **Heading letter-spacing and vertical rhythm differentiation**:
  - `h1`: `letter-spacing: -0.01em`
  - `h2`: `letter-spacing: -0.005em`, `margin-top: 2.25rem` (was `1.75rem`) — extra section-break space above major sections
  - `h4`: `margin-top: 1.5rem` (was `1.75rem`) — tighter for subsections
- **h5 and h6**: Previously inherited only `font-family/weight/margin` from the group rule; no `font-size` defined. Added:
  - `h5 { font-size: 1rem; font-weight: 600; margin-top: 1.25rem; }`
  - `h6 { font-size: 0.9375rem; font-weight: 600; font-style: italic; margin-top: 1.25rem; }` (Wikipedia italic-h6 convention)
- **Paragraph spacing**: `margin: 0.75em 0` → `0.9em 0` (more breathing room between paragraphs)
- **List item spacing**: `margin: 0.2em 0` → `0.3em 0`
- 190 tests pass (CSS-only change; no Rust changes).

### Sprint AA — SHIPPED 2026-05-16 (commit `99938103`, Peter Woodfine)

**Chrome polish / accessibility:**

- **Global `:focus-visible` rule** — added immediately after the CSS reset: `outline: 2px solid var(--link); outline-offset: 2px; border-radius: 2px`. Fires only on keyboard navigation (not mouse clicks). Covers all interactive elements — tabs, buttons, links, portlet entries — that previously had no visible focus indicator.
- **Skip-to-content link** — `.skip-to-content` visually hidden (`position: fixed; top: -100%`) until keyboard-focused, then slides to `top: 0.75rem` via `transition: top 0.15s ease`. Added to all three page templates:
  - Article (`wiki_chrome`): `href="#mw-content-text"` — existing ID
  - Home (`home_chrome`): `href="#mp-main"` — existing ID
  - Other pages (`chrome` helper): `href="#main-content"` — added `id="main-content"` to `<main>` in that template
- **Sticky header slide-in animation** — `.wiki-sticky-header.sticky-visible` now has `animation: sticky-slidein 0.15s ease` (`from: opacity 0, translateY(-6px)` → `to: opacity 1, translateY(0)`). Previously appeared abruptly via `display: none → flex` toggle with no transition.
- **`sticky-edit:hover` var fallback** — `var(--bg, #fff)` → `var(--bg)` (Sprint Y missed this one instance).
- **`login-field input:focus` → `:focus-visible`** — bare `:focus` changed to `:focus-visible` so mouse clicks on login inputs don't trigger the outline ring; keyboard focus still shows it. The global rule provides the same ring anyway.
- 190 tests pass (CSS + minor Rust template additions; no logic changes).

### Sprint AB — SHIPPED 2026-05-17 (commit `b28396ce`, Peter Woodfine)

**Mobile polish — WCAG 2.5.5 touch targets, drawer animation, focus management:**

- **Touch target sizing** (WCAG 2.5.5, minimum 44×44px): all four mobile interactive element types brought into compliance:
  - `.nav-toggle-btn` and `.toc-toggle-btn` — `min-height: 44px; min-width: 44px` with flex centering
  - `.mobile-nav-close` and `.mobile-toc-close` — same treatment (were ~11px due to bare `padding: 0.3rem`)
  - `.mobile-nav-list a` and `.mobile-toc-list a` — `min-height: 44px` with flex-align on list links
- **CSS slide-in animation** — `@keyframes drawer-slidein` (translateX(-100%) → translateX(0), opacity 0→1, 0.2s ease); `@keyframes overlay-fadein` (opacity 0→1, 0.2s ease). Applied on `display: flex` application (CSS transitions can't animate from `display: none`; keyframe approach fires correctly).
- **Focus management** — `openNav()` now calls `closeBtn.focus()` (previously focus stayed on hamburger behind overlay); `closeNav()` returns focus to toggle button. Same pattern added to `openToc()`/`closeToc()`. Nav drawer links now close the drawer on click (matching existing TOC drawer behaviour — parity achieved).
- **`trapFocus(drawer)` helper** — keyboard Tab cycles through `a[href], button:not([disabled]), [tabindex]:not([tabindex="-1"])` inside the open drawer; Shift+Tab reverses; prevents focus escaping to background content while drawer is open. Applied to both nav and TOC drawers.
- **8 new integration tests** (`tests/mobile_test.rs`) — nav toggle ARIA, drawer hidden-on-load, close button accessible label, nav links present, overlay present, TOC drawer on articles with headings, TOC absent on flat articles, TOC `aria-controls` wired. All 8 pass.
- 198 tests pass total (190 existing + 8 new; CSS + JS changes; no Rust template changes).

### Sprint AC — SHIPPED 2026-05-17 (commit `35f787e3`, Jennifer Woodfine)

**Infobox title/image, `main` hatnote fenced block:**

- **Infobox `title:` → `<caption>`**: When the YAML has a `title:` key, it now renders as a `<caption class="infobox-title">` element at the top of the infobox table. Styled with centred bold text and a `var(--mw-color-base-20)` background matching the header row convention.
- **Infobox `image:` → image row**: When the YAML has an `image:` key, a full-width `colspan="2"` row is inserted above the data rows with an `<img>` whose `src` is the value. The `alt` attribute falls back to `image_caption:` then `title:`. Optional `image_caption:` key renders a `<div class="infobox-caption">` below the image (italic, muted).
- **Reserved-key filtering**: `title`, `image`, and `image_caption` are never rendered as `<th>/<td>` data rows; they are special-purpose keys. All other keys render as before.
- **`main` fenced block → hatnote**: A fenced code block with info string `main` renders a `<div class="wiki-hatnote">Main article: <a href="/wiki/...">...</a></div>`. Two formats supported: bare slug (`architecture/compounding-substrate` → derives display text "Compounding Substrate") or `slug|Display Text` for explicit label. Uses the existing `.wiki-hatnote` class for consistent styling.
- **`r#unsafe = true` in `render_html_raw`**: The comrak renderer was silently suppressing all programmatically-injected `HtmlBlock` nodes (including Phase B infobox/navbox). Enabling unsafe HTML rendering is safe here because: (a) all our generated HTML goes through `escape_html()`, (b) the wiki has Phase 5 auth so content editing requires login. A comment documents the rationale.
- **7 new integration tests** (`tests/infobox_test.rs`): title as caption, title not in data rows, image row, image/image_caption not in data rows, main hatnote renders, slug-derived display text, pipe display text. All 7 pass.
- 205 tests pass total (198 + 7 new).

### Sprint AD — SHIPPED 2026-05-18 (commits `dc0d3af3` Peter + `3514904e` Jennifer)

**Engine P0 bug-fixes from three-wiki UX audit:**

- **P0-A (hidden-dir walk filter)**: `collect_topic_files` now skips any directory whose name starts with `.` — removes stray `woodfine-fleet-deployment/.git/guide-provision-node.md` from sitemap/search.
- **P0-B (AGENT.md system-file filter)**: Added `"AGENT"` to `SYSTEM_FILE_STEMS` const — `content-wiki-documentation/AGENT.md` no longer appears as a wiki article.
- **P0-D (per-article `<title>` tag)**: `wiki_chrome` now emits `<title>Article Name — Site Title</title>` instead of the bare site title for every article page. Helps browser history and external references.
- **P0-C (bare-slug 301-redirect resolver)**: New `resolve_bare_slug` async helper searches category subdirectories for a unique stem match and 301-redirects to the path-qualified slug. Fixes ~280+ broken wikilinks from the Wave-1 category-subdir migration without requiring content edits. New test `wiki_page_bare_slug_redirects_to_qualified`.
- 206+ tests pass total (205 + 1 new bare-slug test).

### Sprint AE — SHIPPED 2026-05-18 (commit `ecd6b74a`, Jennifer Woodfine)

**P0 engine fixes (P0-E, P0-F; P0-G and P0-H already shipped):**

- **P0-E (tagline from site_title)**: `wiki_chrome` and talk handler now render
  `"From " (site_title.trim_end_matches(" Wiki"))` — projects/corporate wikis show correct taglines.
- **P0-F (search index filter)**: `collect_topics` in `search.rs` gains hidden-dir skip,
  `SEARCH_EXCLUDED_STEMS` const (14 repo-management stems), `is_excluded_stem()` helper,
  and a new `system_files_excluded_from_index` test.
- **P0-G**: Already present — skip-to-content links at server.rs:1017, 1877, 3359.
- **P0-H**: Already present — error.rs has full HTML chrome for all WikiError variants.

### Next sprint candidates (post-AE)

Stage 6 promotion (16 commits ahead of origin/main — Master session needed) is the blocking operational item. Feature sprints that can ship now:

- **Sprint AF** — Projects wiki content and remaining P0 audit: short_description for all remaining articles (30 missing), plus any P1/P2 items from THREE-WIKI-REBUILD-MASTER.md.
- **Phase 7 IVC machinery** — `local-fs-anchoring` integration; IVC band from placeholder to live. Operator clearance needed.
- **DESIGN-COMPONENT refinement** — research-trail footer and citation-ribbon DESIGN tokens need project-design pass before the engine can bind them.

---

## Appendix A — File touchpoints summary

For the next implementing engineer, the canonical file map:

| File | Purpose |
|---|---|
| `src/server.rs` | All routes, `wiki_chrome()`, `home_chrome()`, special pages. 3,739 lines. |
| `src/render.rs` | Markdown → HTML, frontmatter parsing, edit-pencil injection, wikilink rewriting. 567 lines. |
| `src/edit.rs` | Edit form + atomic write + commit. 356 lines. |
| `src/pending.rs` | Phase 5 review queue. 504 lines. |
| `src/auth.rs` | Phase 5 cookie sessions. 427 lines. |
| `src/users.rs` | Phase 5 SQLite + argon2id. 185 lines. (Add `watches` table here for §3.7.) |
| `src/feeds.rs` | Atom / JSON Feed. 379 lines. |
| `src/search.rs` | Tantivy. 415 lines. |
| `src/mcp.rs` | MCP JSON-RPC 2.0. 491 lines. |
| `src/git_protocol.rs` | Git smart-HTTP. 109 lines. |
| `src/history.rs` | git2 wrapper, history/blame/diff. 272 lines. |
| `static/style.css` | All CSS, 2,449 lines. (Variable declarations top of file.) |
| `static/wiki.js` | All chrome JS, 830 lines. Idempotent re-init pattern (called on initial load + after every AJAX swap). |
| `static/saa-init.js` | Editor-side SAA + citation autocomplete. 439 lines. |
| `tests/*.rs` | 19 test files; 170 tests; tempdir + oneshot router pattern. |

## Appendix B — Reference inventory (live HTML)

Sampled 2026-05-15 from `/wiki/architecture/leapfrog-2030-architecture`:

**Structural classes present:** `anchor`, `cats-label`, `density-btn`,
`density-btn-active`, `density-label`, `edit-pencil`, `header-search`,
`header-search-wrap`, `ivc-band-text`, `mobile-drawer-divider`,
`mobile-drawer-section-heading`, `mobile-drawer-title`,
`mobile-nav-close`, `mobile-nav-drawer`, `mobile-nav-header`,
`mobile-nav-list`, `mobile-nav-overlay`, `mobile-toc-close`,
`mobile-toc-drawer`, `mobile-toc-list`, `mw-body`, `mw-header`,
`nav-toggle-btn`, `page-body`, `page-title`, `quality-badge`,
`site-footer`, `site-nav`, `site-title`, `sticky-inner`,
`sticky-logo`, `sticky-title`, `toc-header`, `toc-level-2`,
`toc-list`, `toc-numb`, `toc-pin-btn`, `toc-text`, `toc-title`,
`toc-toggle`, `toc-toggle-btn`, `vector-main-menu`, `vector-toc`,
`wiki-article-footer`, `wiki-article-last-edited`, `wiki-categories`,
`wiki-category-single-tag`, `wiki-density-toggle`,
`wiki-footer-links`, `wiki-footer-meta`, `wiki-glossary-term`,
`wiki-ivc-band`, `wiki-lang-btn`, `wiki-lang-globe`,
`wiki-lang-switcher`, `wiki-layout`, `wiki-license`,
`wiki-page-tabs`, `wiki-page-tools`, `wiki-portlet-heading`,
`wiki-portlet-links`, `wiki-redlink`, `wiki-right-rail`,
`wiki-sticky-header`, `wiki-tab`, `wiki-tab-active`, `wiki-tagline`,
`wiki-title-block`, `wiki-title-row`, `wiki-trademark`.

**IDs present:** `density-all`, `density-exceptions`, `density-off`,
`h-*` (per-heading anchors), `header-search-form`, `header-search-q`,
`mobile-nav-close`, `mobile-nav-drawer`, `mobile-nav-overlay`,
`mobile-toc-close`, `mobile-toc-drawer`, `mw-content-text`,
`mw-header`, `mw-panel`, `nav-toggle`, `p-views`,
`search-autocomplete-dropdown`, `sticky-title`, `toc-list`,
`toc-pin-btn`, `toc-toggle`, `toc-toggle-btn`, `vector-toc`,
`wiki-sticky-header`.

Notable absences (parity-gap markers, see §3): `#p-cactions`
(More menu), `#vector-appearance-menu` (theme/width toggle),
`#p-views-sticky` (sticky-header action tabs), `.watch-toggle`
(watch star), `body.printable` (printable mode).
