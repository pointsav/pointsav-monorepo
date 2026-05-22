# WIKIPEDIA-PARITY-RESEARCH-LOG.md — Muscle Memory Audit

> Detailed log of MediaWiki Vector 2022 characteristics to be ported to `app-mediakit-knowledge`.
> Audience: Rust engineer implementing each gap. Reference for `src/server.rs`,
> `src/render.rs`, `static/style.css`, `static/wiki.js`.

---

## 2026-05-15 — Comprehensive Gap Audit: Sticky Header, More Menu, Special Pages, Watch/Move/Print

### 1. Current Shipped State (Live Verification 2026-05-15)

Verified by curling the running binary at `http://localhost:9090`. The
production deployment serves `documentation.pointsav.com` from the same
binary on `cluster/project-knowledge`.

| Feature | State | Notes |
|---|---|---|
| Three-column layout (`#mw-panel` / `.mw-body` / right rail) | Shipped | Confirmed in `/wiki/{*slug}` HTML |
| Site header (`.mw-header`) with `.site-title` + `header-search` | Shipped | Single-form search bar with autocomplete dropdown |
| `vector-main-menu` left rail | Shipped | 7 links: Main page / Random / Wanted / All pages / Recent changes / Statistics / Search |
| `vector-toc` collapsible left-rail TOC with `pin` button | Shipped | `toc-pin-btn` exists |
| Sticky header (`.wiki-sticky-header`) | **Partial** | **Only logo + title.** No tabs, no search, no "More", no watch |
| Article/Talk tabs (`.wiki-page-tabs`) | Shipped | `/wiki/{slug}` vs `/talk/{slug}` |
| Read / View source / View history tabs (`#p-views`) | Shipped | Edit/View-source gated by Phase 5 auth |
| Page-tools right rail (`#p-tb` equivalent) | **Missing** | No "What links here / Permanent link / Page info / Cite this page" portlet |
| "More" / actions dropdown | **Missing** | No equivalent surface anywhere |
| Watch star (`.mw-watchlink`) | **Missing** | No watchlist concept |
| Move (rename) | **Missing** | No `/move/{slug}` route |
| Protect | **Missing** | Auth gate exists; no per-page protection layer |
| `/special/recent-changes` | Shipped (basic) | 50-row table, no filters |
| `/special/all-pages` | Shipped | Alphabetical jump bar + 302 articles |
| `/special/statistics` | Shipped (assumed) | Route exists, not audited this pass |
| `/special/login` + session auth | Shipped | Phase 5 |
| `Special:Search` (`/search`) | Shipped | Tantivy BM25 |
| `Special:Categories` index page | **Missing** | `/category/{name}` exists for one category; no top-level alphabetical index |
| Printable version (`/print/{slug}` or `?printable=yes`) | **Missing** | No print-optimised route or `@media print` CSS |
| Talk pages (`/talk/{slug}`) | Shipped | Sprint C4 |
| Mobile hamburger drawer | Shipped | `.mobile-nav-drawer` + `.mobile-toc-drawer` |
| IVC band + density toggle (placeholders) | Shipped | Phase 7 surfaces |
| Language switcher (`.wiki-lang-switcher`) | Shipped | EN ⇄ ES |
| Hatnote, infobox, navbox rendering | Shipped | Per render.rs |

Counted gaps: **9** (sticky-header completion, More menu, p-tb tools portlet,
Watch, Move, Protect, RecentChanges filter UI, Special:Categories index, Printable).

---

### 2. Group A — Sticky Header Completeness

**Wikipedia Vector 2022 DOM (verified at `/wiki/Rust_(programming_language)`):**

```html
<div class="vector-sticky-header">
  <div class="vector-sticky-header-start">
    <a class="vector-sticky-header-icon-end" href="/wiki/Main_Page">…logo…</a>
    <span class="vector-sticky-header-context-bar">
      <h1 class="vector-sticky-header-context-bar-primary">Rust (programming language)</h1>
    </span>
  </div>
  <div class="vector-sticky-header-end">
    <!-- search button (collapsed input that expands on click) -->
    <button class="vector-sticky-header-search-toggle">…</button>
    <!-- page action tabs (Read / Edit / View history) -->
    <div class="vector-sticky-header-buttons">
      <a class="mw-page-action-read" …>Read</a>
      <a class="mw-page-action-edit" …>Edit</a>
      <a class="mw-page-action-history" …>View history</a>
    </div>
    <!-- "More" dropdown -->
    <div class="vector-page-actions-more">…⋯…</div>
    <!-- Language selector -->
    <button class="vector-sticky-header-language-button">…</button>
  </div>
</div>
```

**Current Leapfrog sticky header (`src/server.rs` near sticky-title block):**

```html
<div class="wiki-sticky-header" id="wiki-sticky-header" aria-hidden="true">
  <div class="sticky-inner">
    <a class="sticky-logo" href="/">PointSav Documentation Wiki</a>
    <span class="sticky-title" id="sticky-title">…</span>
  </div>
</div>
```

**Gap and implementation plan:**

1. Add a `.sticky-actions` right-side container parallel to Wikipedia's
   `vector-sticky-header-end`. Inside it:
   - Compact search button that expands the existing `#header-search-q`
     input (or duplicates it, hidden until activated).
   - `Read | View source | View history` tab strip — the same three
     `<a>`-elements as `#p-views`, rendered as smaller chiclets.
   - The "More" dropdown (Group B below) anchored here.
2. The sticky-header logic in `static/wiki.js` already toggles
   `aria-hidden` based on scroll-past-title. Extend the function to also
   keep the action tabs in sync with the main `#p-views` active state
   (highlight Read when on the article, etc.).
3. CSS: add `.wiki-sticky-header .sticky-actions { display:flex; gap:0.5rem; }`
   and reuse `.wiki-tab` styling at a smaller scale.

Wikipedia's center slot (between start and end) is also occupied by a
secondary section title that updates as the reader scrolls past each
`<h2>`. Defer this; it is muscle memory but optional for v1.

**Files to touch:** `src/server.rs` (sticky-header `html!` macro block),
`static/wiki.js` (sticky toggle), `static/style.css` (sticky-actions
styling).

---

### 3. Group B — "More" Menu / Actions Dropdown

Wikipedia's "More" dropdown (`vector-page-actions-more`) is a `<details>`
or a JS-toggled `<div>` listing utility actions. Verified items present
for an article:

| Wikipedia label | URL pattern | Class |
|---|---|---|
| Move | `/w/index.php?title=…&action=move` | `#ca-move` |
| Watch | (toggles) `/w/api.php?action=watch` | `.mw-watchlink` |
| Print this page | `/w/index.php?title=Special:DownloadAsPdf&page=…` | `.mw-page-action-print` |
| Permanent link | `/w/index.php?title=…&oldid=<REV>` | `.mw-page-action-permalink` |
| Page information | `/w/index.php?title=…&action=info` | `.mw-page-action-info` |
| Cite this page | `/w/index.php?title=Special:CiteThisPage&page=…` | `.mw-page-action-cite` |
| Get shortened URL | `/w/index.php?title=Special:UrlShortener&url=…` | (link) |
| Download QR code | (modal) | (link) |
| Wikidata item | `https://www.wikidata.org/wiki/Q575650` | (link) |

**Implementation spec for Leapfrog:**

Render a single `<nav class="wiki-page-actions-more">` inside the
sticky-header right slot AND inside the main `#p-views` row (two
render points; same template fragment). DOM:

```html
<nav class="wiki-page-actions-more" aria-label="More actions">
  <button class="more-toggle" aria-haspopup="true" aria-expanded="false"
          aria-controls="more-menu-popup">⋯</button>
  <ul class="more-menu" id="more-menu-popup" hidden>
    <li><a href="/move/{slug}" class="mw-page-action-move">Move</a></li>
    <li><a href="/watch/{slug}" class="mw-watchlink mw-watchlink-unwatched">Watch</a></li>
    <li><a href="/print/{slug}" class="mw-page-action-print">Printable version</a></li>
    <li><a href="/history/{slug}#rev-{sha}" class="mw-page-action-permalink">Permanent link</a></li>
    <li><a href="/info/{slug}" class="mw-page-action-info">Page information</a></li>
    <li><a href="/cite/{slug}" class="mw-page-action-cite">Cite this page</a></li>
    <li><a href="/git/{slug}" class="mw-page-action-source">View raw source</a></li>
  </ul>
</nav>
```

**Per-item routing decisions for a flat-file/git backend:**

- **Move** — implementable as `/move/{slug}` POST handler that does a
  `git mv` and rewrites internal `[[wikilinks]]` pointing at the old
  slug. Auth-gated. Defer until Group F decision below.
- **Watch** — implementable per-user via `state_dir/watchlists/<user>.txt`
  containing slugs. See Group F.
- **Printable version** — implementable as a route (`/print/{slug}`)
  that re-uses the wiki template with a `.printable` body class. See
  Group E.
- **Permanent link** — already implementable today: the latest commit
  SHA for the slug is known from `history.rs`. Link to
  `/history/{slug}` with the SHA visible.
- **Page information** — new lightweight route showing: byte count,
  word count, last edit SHA + author + date, contributor list,
  category, languages available, redirects pointing here (Phase 4
  link-graph already exists in redb per `cleanup-log.md` 2026-04-27).
- **Cite this page** — render a static block showing APA / MLA /
  Chicago / BibTeX citations templated from frontmatter (`title`,
  `last_edited`, slug, base URL).
- **View raw source** — already exists at `/git/{slug}`; surface it
  here.
- **Wikidata item / QR code / shortened URL** — out of scope for v1.

**Files to touch:** new partial in `src/server.rs` (`render_more_menu`),
new handlers `info_page` and `cite_page`, `static/wiki.js` (toggle on
click + Esc + outside-click close), `static/style.css`.

---

### 4. Group C — Special:RecentChanges Gap Analysis

**Current implementation** (curled `/special/recent-changes`):
flat 50-row `<table class="wiki-special-table">` of git log entries.
No filters, no namespace dropdown, no toggles, no pagination.

**Wikipedia's filter URL parameters (verified):**

| Parameter | Default | Meaning | Git-backend feasibility |
|---|---|---|---|
| `days=N` | 1 | Cutoff in days | **Feasible** — `git log --since="N days ago"` |
| `limit=N` | 50 | Max rows | **Feasible** — `git log -n N` |
| `from=<ts>` | none | Pagination cursor | **Feasible** — `git log --until=<ts>` |
| `namespace=N` | 0 | NS filter | **Adaptable** — map to directory prefix (`architecture/`, `services/`, etc.); namespace 0 = "all root TOPICs"; "Talk" = `/talk/` directory once Talk gets git-backed storage |
| `hideminor=1` | off | Hide minor edits | **Not feasible without convention** — git has no `minor` flag. Could parse commit messages for `[minor]` prefix |
| `hidebots=0` | bots hidden | Hide bots | **Adaptable** — known bot identities (`ps-administrator`, `mcorp-administrator` for governance; anything else = human) |
| `hideliu=1` | off | Hide logged-in users | **Not meaningful** — git commits don't carry session identity, only the configured author |
| `hideanons=1` | off | Hide anons | **Not meaningful** — same reason; flat-file edits go through Phase 5 auth so all writes are by identified accounts |
| `hidemyself=1` | off | Hide my edits | **Feasible post-Phase-5** — compare commit author email to session user |
| `hidecategorization=0` | off (hidden) | Show categorization edits | **Adaptable** — detect frontmatter-only diffs (only `category:` changed) via `git diff --name-only` + parse |
| `hideWikibase` | off | Wikidata | **Not applicable** |
| `hidenondamaging=1` | off | Patrol filter | **Not applicable** — no patrol queue |
| `tagfilter=…` | none | Edit tags | **Adaptable** — Conventional Commits prefixes (`fix:`, `feat:`, `docs:`) function as tags |
| **Group changes by page** | off | "Enhanced" mode | **Feasible** — `GROUP BY slug` over the same git-log rows |

**Concrete filter UI to ship in Iteration 1 (smallest useful set):**

1. Time-range pills: `1 | 3 | 7 | 14 | 30 days`
2. Limit pills: `50 | 100 | 250 | 500`
3. Namespace dropdown: `All | Architecture | Services | Systems |
   Applications | Governance | Infrastructure | Reference | Design-system`
   (the nine ratified categories as a proxy for Wikipedia's NS list)
4. "Hide bots" toggle (default ON — bot commits like the propagation
   tool clutter the human reading view)
5. "Group changes by page" toggle (collapses multiple edits to one row)
6. "From timestamp" pagination cursor (`?from=20260515T012233Z`)

Defer: tag filter, hidemyself, hidecategorization, live-updates.

**Row rendering — match Wikipedia order:**

```
[diff] [hist] <Article title> · <timestamp> · <±bytes> · <author> · "summary" · [tags]
```

Current row order is `date / author / sha-summary`. Reorder to put
article title first, add `[diff]` and `[hist]` mini-links, surface
byte delta from `git log --numstat`. CSS classes: `.rc-line`,
`.rc-diff`, `.rc-hist`, `.rc-title`, `.rc-time`, `.rc-bytes` (with
`.rc-bytes-pos` / `.rc-bytes-neg`), `.rc-author`, `.rc-summary`,
`.rc-tags`.

**Files to touch:** `src/server.rs` `recent_changes` handler (likely in
a `special.rs` module — verify), new helpers to derive byte deltas
from `git log --numstat` (gix can do this), `static/style.css`,
`static/wiki.js` for filter-form submission and "From" auto-update.

---

### 5. Group D — Special:Categories Index

**Wikipedia surface:**
A top-level alphabetical index of all categories. Each row:
`<Category name> (N members)`. Letter jump bar (A–Z + 0–9 + two-letter
combos like `Aa`, `Ab`). Pagination via `?offset=Aa`. Click-through to
`/wiki/Category:Foo` which lists subcategories + member pages.

**Current Leapfrog surface:**
- `/category/{name}` route exists (`category_page` in `server.rs` line
  975) and shows pages within one category.
- **No top-level `/special/categories` route exists.**
- Categories come from the singular `category:` frontmatter field
  (not Wikipedia's list-valued `[[Category:Foo]]` markup), and from
  the directory prefix in slugs (`architecture/leapfrog-2030-architecture`
  → category `architecture`).

**Implementation spec:**

1. Add route `/special/categories` → handler `special_categories`.
2. Reuse `bucket_topics_by_category` to get `BTreeMap<String, Vec<TopicSummary>>`.
3. Render an alphabetical list. Because category names today are the
   nine ratified categories plus arbitrary user-added values, render:
   - Top section: the nine ratified categories in render order with
     prominent presentation.
   - Below: all-categories-alphabetical with member counts.
4. Each category row: `<a href="/category/{slug}">{Title}</a> (<N> members)`.
5. Letter jump bar — feasible but probably overkill with under ~30
   distinct categories. Defer until the catalogue exceeds 100 distinct
   categories.

**DOM:**

```html
<main class="mw-body wiki-special">
  <h1>Categories</h1>
  <p class="wiki-special-intro">N categories across M articles.</p>
  <h2>Primary categories</h2>
  <ul class="mw-category-group wiki-cat-primary">
    <li class="mw-category"><a href="/category/architecture">Architecture</a> <span class="mw-cat-count">(42 members)</span></li>
    …
  </ul>
  <h2>All categories</h2>
  <ul class="mw-category-group wiki-cat-all">…</ul>
</main>
```

**Category page (`/category/{name}`) — already exists.** Audit
separately: add member count to header, surface "Pages in category"
H2 explicitly, render percent-encoded names safely.

**Files to touch:** `src/server.rs` (new `special_categories` handler,
add route in builder around line 120 next to existing `category_page`),
`static/style.css` (re-use existing `.wiki-special-table` shell).

---

### 6. Group E — Printable Version

**Wikipedia approach:** `?printable=yes` query param adds
`mw-printable` body class; `@media print` stylesheet hides chrome.
Wikipedia itself deprecates the separate printable URL and encourages
the browser's native print (`Ctrl+P`).

**Recommended Leapfrog approach: CSS-first + opt-in route.**

1. **Add `@media print` block to `static/style.css`.** Hide:
   - `.mw-header`, `.wiki-sticky-header`, `#mw-panel`,
     `.vector-main-menu`, `.vector-toc`, `.mw-page-actions-more`,
     `#p-views`, `.wiki-ivc-band`, `.wiki-density-toggle`,
     `.mobile-nav-drawer`, `.mobile-toc-drawer`, `.site-footer`
     (or replace footer with a print-only short URL stamp).
   - Re-flow `.wiki-layout` to single column.
   - Use serif body font, 11pt, black-on-white.
   - Expand inline references — `.wiki-references` already a numbered
     list, but ensure `<a href>` targets render the URL in print
     (CSS: `.wiki-references a::after { content: " (" attr(href) ")"; }`).
   - Preserve infoboxes; rescale images to `max-width:60%`.
2. **Add `/print/{slug}` route** that serves the same HTML with a
   `printable` body class. This gives a stable URL to share that
   forces the print stylesheet even on screen — useful for QA and
   "save as PDF" via a headless renderer down the line.
3. **Link back** — at top of printable view, a small banner:
   *"Printable view — return to article"* with link to `/wiki/{slug}`.

**Citations and references in print:**
- Footnote markers `[1]`, `[2]` preserved as superscripts.
- References section ALREADY at bottom of every article (per existing
  rendering). No change needed.
- Add `@page` rule with margins (`@page { margin: 2cm; }`).

**PDF generation deferred.** A future enhancement could pipe
`/print/{slug}` HTML through `headless-chrome` or `paged.js` for
server-side PDF, but the browser's "Save as PDF" via Ctrl+P over the
print stylesheet is the v1 answer (Wikipedia's own current stance).

**Files to touch:** `src/server.rs` (`print_page` handler, add route),
`static/style.css` (`@media print` block, ~50 lines).

---

### 7. Group F — Watch / Move / Protect Decision Matrix

| Action | Wikipedia surface | Leapfrog feasibility | Recommendation |
|---|---|---|---|
| **Watch** (star icon) | Toggles `mw-watchlink-watched` ⇄ `unwatched`; user's `Special:Watchlist` shows recent changes filtered to watched pages | Per-user file `state_dir/watchlists/<user>.txt`. Watchlist page = filtered Recent changes. | **Implement** — natural feature for an internal docs wiki; lets contributors get notified |
| **Move** (rename slug) | `?action=move` form; updates page + creates redirect | `git mv`; rewrite `[[wikilinks]]` graph (redb already tracks links per Phase 4 plan); leave a redirect stub at the old slug | **Implement** — high-value for the slug-rename churn we already see in cleanup-log |
| **Protect** | Per-page edit lock; admin sets levels (semi-protected, fully protected) | Per-page YAML in `state_dir/protection/<slug>.yaml` listing required role; checked in edit handler before commit | **Defer** — current auth gate (logged-in = can edit) is sufficient until we have a multi-tier role model |

**Watch — concrete spec:**

- Storage: `state_dir/watchlists/<user_id>.json` — array of slugs.
- New routes:
  - `POST /watch/{slug}` → toggle for authenticated user; redirect back.
  - `GET /special/watchlist` → personalised Recent changes filtered
    to slugs in user's watchlist.
- Star button DOM (in `#p-views` + sticky header + More menu):
  ```html
  <form method="post" action="/watch/{slug}" class="wiki-watch-form">
    <button type="submit"
            class="mw-watchlink mw-watchlink-unwatched"
            aria-pressed="false"
            aria-label="Add this page to your watchlist">☆</button>
  </form>
  ```
  Filled `★` and `aria-pressed="true"` plus class `mw-watchlink-watched`
  when on the list.
- Unauthenticated visitors: button is rendered but links to
  `/special/login?return=/wiki/{slug}`.

**Move — concrete spec:**

- Route: `GET /move/{slug}` shows form (new slug + reason).
- `POST /move/{slug}` performs:
  1. Validate new slug shape (lowercase, hyphens, no special chars).
  2. Atomic `git mv content/<old>.md content/<new>.md` via gix.
  3. Walk redb link-graph (Phase 4); for every page that
     `[[old-slug]]`, rewrite to `[[new-slug]]` on disk in same commit.
  4. Write a redirect stub at the old path: a 6-line markdown file
     with frontmatter `redirect: <new-slug>`. Server's `wiki_page`
     handler honours `redirect:` with HTTP 302.
  5. Single commit with message `move: <old> → <new> — <reason>`.
- Tantivy reindex hook already exists (`edit-triggers-reindex`);
  trigger it for both the old (now-redirect) and new slugs.

**Protect — deferred.** When implemented, follow same pattern as Watch
with per-slug YAML in `state_dir/protection/`. Reads cheap; writes only
on operator action.

**Files to touch (Watch + Move only for v1):**
- `src/server.rs` — new handlers `watch_toggle`, `move_get`,
  `move_post`, `watchlist_page`. Mount routes.
- `src/render.rs` — propagate `redirect:` frontmatter field.
- `static/style.css` — `.mw-watchlink` star styling, filled/unfilled
  states, focus ring.
- `static/wiki.js` — optional progressive-enhancement to make the
  watch toggle AJAX instead of a full reload.

---

### 8. Tools Right-Rail Portlet (related to Group B; surfaces orthogonally)

Wikipedia renders an `#p-tb` (`vector-page-tools`) portlet in the right
rail with: *What links here*, *Related changes*, *Upload file*,
*Permanent link*, *Page information*, *Cite this page*, *Get shortened
URL*. Several of these (Permanent link, Page info, Cite, Shortened
URL) overlap with the More menu — Wikipedia surfaces them in BOTH
places. The right-rail portlet provides discoverability for desktop
readers; the More menu surfaces them in the sticky header.

**Recommendation:** ship a parallel right-rail portlet that mirrors
the More menu:

```html
<aside class="vector-page-tools" id="p-tb">
  <h3 class="vector-menu-heading">Tools</h3>
  <ul class="vector-menu-content-list">
    <li><a href="/links-here/{slug}">What links here</a></li>
    <li><a href="/related-changes/{slug}">Related changes</a></li>
    <li><a href="/info/{slug}">Page information</a></li>
    <li><a href="/cite/{slug}">Cite this page</a></li>
    <li><a href="/git/{slug}">View source</a></li>
    <li><a href="/print/{slug}">Printable version</a></li>
  </ul>
</aside>
```

`links-here` and `related-changes` rely on the redb link-graph (Phase 4
already specified; verify whether the index is being kept current on
edit). If not yet wired, ship the portlet with these two links pointing
at a placeholder page until the backlinks API lands.

---

### 9. Suggested Implementation Order

Cheapest-to-highest-impact first:

1. **Iteration A (chrome-only, no new routes):**
   - Sticky-header completion (Group A): action tabs + search button +
     More-menu trigger inside `.sticky-actions`.
   - More-menu dropdown DOM + JS toggle (Group B), with each item
     pointing at routes that may not exist yet (graceful 404 for now).
   - `@media print` block (Group E half-1).
2. **Iteration B (new read-only routes):**
   - `/print/{slug}` (Group E full).
   - `/info/{slug}` and `/cite/{slug}` (Group B handlers).
   - `/special/categories` index (Group D).
   - Right-rail tools portlet (Group 8).
3. **Iteration C (auth-gated state writes):**
   - Watchlist storage + `/watch/{slug}` + `/special/watchlist`
     (Group F watch).
   - Move handler `/move/{slug}` + link-graph rewrite (Group F move).
4. **Iteration D (filter UI on existing list views):**
   - RecentChanges filter pills + namespace dropdown + "Hide bots"
     + "Group by page" (Group C).
5. **Iteration E (deferred):**
   - Protect-page surface (Group F protect).
   - Live-update streaming on RecentChanges.
   - Server-side PDF generation.
   - Tag filter on RecentChanges (requires commit-message convention
     ratified).

---

### 10. Next Research Steps

- [ ] Audit `app-mediakit-knowledge/src/history.rs` for byte-delta
      extraction — confirm whether `git log --numstat` is already
      parsed (needed for RecentChanges row format).
- [ ] Confirm whether the Phase 4 redb link-graph index is being
      updated on edit (Group 8 portlet's "What links here" depends on
      it being current).
- [ ] Spec out the `/info/{slug}` page content list — minimum:
      contributor list, byte count, word count, link-counts in/out,
      creation date, last-edit date, language pairs.
- [ ] Validate that `gix` (already a dep) supports `git mv`-equivalent
      operations cleanly, or whether shelling out to `git` is more
      robust for Move.
- [ ] Survey the existing `/talk/{slug}` Sprint C4 implementation — is
      a Talk-namespace filter in RecentChanges (Group C namespace
      dropdown) realisable, or are Talk pages stored outside the git
      history that RecentChanges reads?
- [ ] Decide commit-message convention for "minor edit" before adding
      `hideminor` (Group C). Options: `[minor]` prefix, `Minor:`
      conventional-commits scope, or `--allow-empty` trailer line.
- [ ] Quick check of `Special:Statistics` current output completeness
      against Wikipedia's surface (page count, edit count, view count
      proxy, contributor count) — defer to a separate audit pass.

---

## 2026-05-11 — Initial Audit (MediaWiki Vector 2022)

### DOM Naming Convention (Standard MW)
| Component | MediaWiki ID/Class | Current app-mediakit-knowledge |
|---|---|---|
| Page Body Wrapper | `.mw-body` | `.wiki-main` |
| Content Wrapper | `#mw-content-text` | `.wiki-article` |
| Site Header | `.mw-header` | `.site-header` |
| Navigation Menu | `.vector-main-menu` | `.wiki-nav-portlet` |
| TOC Wrapper | `.vector-toc` | `.wiki-toc` |
| Sidebar (Left) | `#mw-panel` | `.wiki-left-rail` |
| Page Actions (Top) | `#p-views` | `.wiki-action-tabs` |

### Visual Tokens (Colors & Typography)
- **Wikipedia Blue:** `#36c` (Links), `#3366cc` (Tabs).
- **Wikipedia Red:** `#ba0000` (Redlinks).
- **Background Gray:** `#f8f9fa` (Chrome/TOC).
- **Border Gray:** `#a2a9b1`.
- **Typography:** 
  - Article Body: `serif` (Georgia, "Times New Roman").
  - Headers/Chrome: `sans-serif` (-apple-system, BlinkMacSystemFont, "Segoe UI").
  - Content Font Size: `0.875rem` (14px) baseline.

### Interaction Behaviors
1. **Keyboard Shortcuts:**
   - `/`: Focus search input.
   - `Alt+Shift+F`: Focus search input.
   - `Alt+Shift+E`: Edit page.
   - `Alt+Shift+H`: View history.
2. **TOC "Pinning":**
   - In Vector 2022, the TOC can be "pinned" to the left rail.
   - When pinned, the main content container shifts to the right to avoid overlapping.
   - When unpinned, it collapses into a menu.
3. **Sticky Header Behavior:**
   - Appears after scrolling past the main title.
   - Contains the article title, search, and edit tools.
   - Implementation in `wiki.js` (Sprint H) is a good start but needs visual refinement.

### Rendering Components
- **Infoboxes:** Must be `table.infobox` with specific styles for headers (`th.infobox-above`), images (`td.infobox-image`), and data rows (`th.infobox-label` / `td.infobox-data`).
- **Navboxes:** Must be `div.navbox` with `table.nowraplinks` and collapsible logic (`initNavboxes` in `wiki.js`).
- **Hatnotes:** Must be `div.hatnote` with 1.6em indentation and italics.

---

## Next Research Steps (2026-05-11 baseline)
- [ ] Audit `src/render.rs` Infobox parser.
- [ ] Compare search result page with Wikipedia's Special:Search.
- [ ] Analyze "Talk" page threading (MediaWiki's new Discussion Tools vs our Sprint C4 implementation).
