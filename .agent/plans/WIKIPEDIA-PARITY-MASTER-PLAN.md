# WIKIPEDIA-PARITY-MASTER-PLAN.md — Leapfrog 2030 Wiki Engine

> **Updated:** 2026-05-15
> **Binary:** 21,782,968 bytes, built 2026-05-15 00:43 UTC
> **Tests:** 170 passing
> **Source of truth at write time:** `pointsav-monorepo/app-mediakit-knowledge/`
> running at `documentation.pointsav.com` (port 9090) and
> `projects.woodfinegroup.com` (port 9093).
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

### 2.5 Home page (Phase 3 Wave 3 + leapfrog-iteration-2)

| Feature | Notes |
|---|---|
| Lede with site title | Top of home; replaceable via `index.md`. |
| Featured TOPIC pin | `featured-topic.yaml` (slug + optional `since` + `note`); silent suppress on missing/unresolvable. |
| Recent additions | Top 5 by `last_edited:` desc; git shell-out + mtime fallbacks. |
| 9-category 3×3 grid | architecture, services, systems, applications, governance, infrastructure, company, reference, help. "In preparation" placeholder for empty categories. |
| Leapfrog facts panel | `leapfrog-facts.yaml` (8 facts with path-qualified link slugs). |
| Home stats banner | "N articles across N categories — last updated YYYY-MM-DD." |
| Two guide-dir surface | Single binary serves multiple wikis; `--guide-dir-2` mounts a second TOPIC source under the same router. |

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
| **Citation authority ribbon** | DESIGN-COMPONENT drafted; engine implementation pending | `drafts-outbound/component-citation-authority-ribbon.md` | Will sit below H1, mirroring Phase 7's IVC verification surface. Green/amber/red badge tied to `cites:` frontmatter resolved against `citations.yaml`. |
| **Freshness ribbon** | DESIGN-COMPONENT drafted; engine implementation pending | `drafts-outbound/component-freshness-ribbon.md` | Computes age from `last_edited:`; renders current / recent / aging / stale badge in footer. |
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
| DOM contract | Number of canonical Vector 2022 selectors emitted (mw-header, mw-body, mw-content-text, vector-toc, vector-main-menu, mw-panel, p-views, infobox, navbox, reflist, hatnote, p-cactions, vector-appearance) | 13 / 13 | 12 / 13 (missing `#p-cactions` — §3.3) |
| Visual contract | Number of hardcoded hex literals in `style.css` | 0 outside the variable declarations block | Verified clean as of Phase 2A |
| Visual contract | Working dark mode on the article shell | Y | **N** (§3.1 not yet shipped) |
| Visual contract | 76em content discipline | Y, toggleable | Y (toggle absent — §3.1) |
| Interaction contract | `?` keyboard help overlay | Y | Y |
| Interaction contract | TOC pin + localStorage | Y | Y |
| Interaction contract | Hover-card preview latency (perceived) | < 250ms | Y (200ms delay + LRU cache) |
| Interaction contract | Sticky header carries action tabs | Y | **N** (§3.2 — logo+title only) |
| Interaction contract | Mobile hamburger + § TOC drawer | Y | Y |
| Reader actions | Article / Talk / Read / Edit / History / View source surfaced | Y | Y |
| Reader actions | "More" actions dropdown | Y | **N** (§3.3) |
| Reader actions | Watch / Unwatch | Y | **N** (§3.7) |
| Reader actions | Printable version | Y | **N** (§3.5) |
| Special pages | RecentChanges / AllPages / Statistics / Search / WhatLinksHere / Cite / PageInfo | All 7 | All 7 |
| Special pages | RecentChanges filter panel | Filters live | **Partial** (no filters yet — §3.6) |
| Special pages | Special:Categories | Y | **N** (§3.4) |
| Performance | P50 article page render time | < 50ms | Y (Rust + flat-file) |
| Performance | P50 search query time (49 TOPICs) | < 30ms | Y (Tantivy) |
| Substrate | Markdown source-of-truth (no schema migrations) | Y | Y |
| Substrate | `git clone` of the wiki returns the full content tree | Y | Y |
| Doctrine | Research-trail footer when frontmatter declares it | Y | Y |
| Doctrine | BCSC-compliant disclosure (FLI notice, planned/intended SDF) | Enforced | Enforced by render-time linting |
| Tests | Integration + unit tests passing | 100% green | 170 / 170 |

The five **N** rows above are the gap between today's 78–82%
realised parity and the 97% target. §6 schedules them.

---

## 6. Next Sprint Proposal

The two highest-leverage parity gaps for the next implementation
session are **§3.1 (appearance menu / dark mode)** and **§3.2
(sticky header completeness)**. They unblock §3.7 (watch star,
which needs both auth-bound state *and* a place in the sticky
header to live) and §3.3 (the "More" dropdown sits next to the
expanded view tabs).

### Recommended next sprint — "Appearance + Sticky"

**Scope:** §3.1 + §3.2 + §3.5 (printable). Three items because
they share both file touchpoints (`wiki_chrome`, `style.css`,
`wiki.js`) and one CSS infrastructure pass (the dark-mode variable
inversion). Bundling them avoids three separate full-binary
rebuilds.

**Sequencing:**
1. **§3.1.a — Variable inversion pass.** Audit `style.css` for any
   non-variable colour reference; fix any survivors from Phase 2A.
   Add the `[data-theme="dark"]` block. Add `[data-width="wide"]`
   block.
2. **§3.1.b — Appearance menu DOM.** Add the button + popover in
   `wiki_chrome()` next to the language switcher. Wire JS state.
3. **§3.1.c — FOUC-prevention script.** Inline a small `<script>`
   in `<head>` (before stylesheet load) that reads localStorage
   and sets `<html data-theme=…>` synchronously.
4. **§3.2 — Sticky header expansion.** Extend the existing
   `wiki-sticky-header` to carry the search form and `#p-views-
   sticky` action tabs.
5. **§3.5 — Printable.** Add the `printable` flag + body class +
   CSS hides. Smallest of the three.
6. **Tests** — new tests covering: theme attribute round-trip
   under each toggle; sticky header DOM contains action tabs;
   printable mode strips chrome.
7. **Stage-6 promotion ask** — outbox to Master once binary
   rebuilt and integration tests green.

**Out of scope for this sprint:**
- §3.3 (More menu) — leave for the sprint after, paired with
  §3.4 (Special:Categories) and §3.6 (RecentChanges filters).
- §3.7 / §3.8 — gated on BP5 watchlist schema decision.
- Phase 7 IVC machinery — separate workstream entirely
  (depends on `local-fs-anchoring` and citation registry).

**Estimated effort:** 1.5–2 working days for a single Task Claude
session, including tests and a Stage-6-ready commit. Dark-mode is
the longest single piece (the variable inversion pass is
mechanical but exhaustive).

**Branch:** continue on `cluster/project-knowledge`. Single commit
acceptable since the three items are tightly coupled; alternatively
three commits in sequence (one per §3.x) for cleaner Stage-6
history.

### Sprint after that — "More + Categories + RC Filters"

- §3.3 More actions dropdown
- §3.4 Special:Categories
- §3.6 RecentChanges short-term filter set (days / limit /
  hidemin only — namespace and tags deferred)
- Watch star DOM hook (button only — backend gated on BP5)

### Sprint after that — Phase 7 prep + Leapfrog originals

Citation-authority ribbon and freshness ribbon (DESIGN-COMPONENT
drafts already staged at `drafts-outbound/`). These need the
project-design refinement pass to land DESIGN tokens into
`pointsav-design-system` first; the engine then consumes the
ratified token bundle.

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
