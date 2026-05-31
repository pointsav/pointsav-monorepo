---
schema: foundry-draft-v1
title: "Research: Wikipedia Vector 2022 Toolbar + Mobile — Engine Gap Analysis"
slug: research-wikipedia-toolbar-mobile
state: draft-pending-design-pass
originating_cluster: project-knowledge
target_repo: pointsav-design-system
target_path: research/
target_filename: wikipedia-toolbar-mobile.md
audience: vendor-internal
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-RESEARCH
authored: 2026-05-07T00:00:00Z
authored_by: task-project-knowledge (Opus parent + Sonnet sub-agent research)
authored_with: claude-sonnet-4-6
references:
  - external:mediawiki.org/wiki/Skin:Vector/2022
  - external:mediawiki.org/wiki/Skin:Vector/2022/Design_documentation
  - external:mediawiki.org/wiki/Skin:MinervaNeue
  - external:doc.wikimedia.org/mediawiki-extensions-MobileFrontend/master/
  - clones/project-knowledge/.agent/drafts-outbound/research-wikipedia-leapfrog-2030.draft.md
notes_for_designer: |
  This research draft provides the engine-gap analysis that grounds Sprint G through
  Sprint K of the Wikipedia muscle-memory implementation roadmap. It pairs with the
  broader research-wikipedia-leapfrog-2030 draft (which covers homepage, token vocabulary,
  and competitive landscape). This draft is scoped specifically to toolbar navigation,
  responsive collapse, and the mobile UX patterns the engine needs next.
---

# Research: Wikipedia Vector 2022 Toolbar + Mobile — Engine Gap Analysis

## Research trail

- **Method:** Sonnet sub-agent research into MediaWiki Vector 2022 and MinervaNeue source
  documentation; codebase audit of `static/style.css`, `static/wiki.js`, and
  `src/server.rs` to cross-reference research findings against current engine state;
  route-table audit to identify stub vs. implemented handlers. 2026-05-07.
- **Depth:** Vector 2022 JS (`movePinnableElement`, `IntersectionObserver` sticky header,
  `pinnable-element` portlet system); MinervaNeue collapsible sections and bottom toolbar;
  full breakpoint inventory; engine CSS/JS/route audit against Wikipedia patterns.
- **Confidence:** High — Vector 2022 behavior derived from MediaWiki design documentation
  and source code; engine audit is direct first-party inspection.
- **Limitations:** No live Wikipedia DOM inspection or browser devtools session. Patterns
  derived from documented MediaWiki source and design documentation.

---

## §1 — Engine baseline (current state)

The engine already has more responsive infrastructure than a cold audit might suggest.
The following items are shipped and working:

### Navigation

| Element | Implementation | Status |
|---|---|---|
| Hamburger nav button (`nav-toggle-btn`) | Rendered in header at all widths | Present |
| Mobile nav drawer (`mobile-nav-drawer`) | Slide-in from left; contains Nav links | Working |
| Nav drawer JS toggle (`initMobileNav`) | Open/close with overlay + Esc key | Working |
| Left rail Navigation portlet | All 7 nav links (`/`, `/random`, `/wanted`, etc.) | Present |
| Talk tab | `href="/talk/{slug}"` — real handler, reads talk-files | Working |
| Edit tab | `href="/edit/{slug}"` for logged-in users; "View source" for anonymous | Working |
| View history tab | `href="/history/{slug}"` — real handler via `history_page` | Working |

The file header comment at `src/server.rs` line 6 states "Edit + View-history are href='#'
placeholders" — this is stale documentation from Phase 1.1. Both routes have been properly
wired since Phase 2 and Phase 4 respectively.

### Contents / ToC

| Element | Implementation | Status |
|---|---|---|
| Left-rail ToC | Rendered from `numbered_headings` iterator | Present |
| TOC collapse toggle | JS `initToc()` with localStorage persistence | Working |
| Active section tracking | Not implemented | **Missing** |
| ToC in mobile drawer | Not implemented | **Missing** |

### Responsive layout

| Breakpoint | What happens | Status |
|---|---|---|
| `≤768px` | Grid collapses to 1 column; left rail `order: 2` (below article) | Present |
| `≤799px` | Mobile nav drawer `display: flex` when `body[data-nav-open]` | Present |
| `≤600px` | Various narrower adjustments | Present |
| Left rail hidden at mobile | **Not implemented** — left rail still renders (below article) | **Missing** |
| Sticky header on scroll | **Not implemented** | **Missing** |
| Icon buttons (ToC, Tools) in titlebar | **Not implemented** | **Missing** |

### Route coverage

All navigable routes are implemented. There are no `href="#"` stubs in the rendered
article HTML. The only stub endpoints are infrastructure placeholders:

- `/api/doorman/complete`, `/api/doorman/instruct` — 501 until Phase 4 Doorman wiring
- WebSocket `/ws/collab/{slug}` — shipped but disabled by default (`--enable-collab` flag)
- IVC masthead band — placeholder text "Verification not yet available — Phase 7"
- Citation density toggle — localStorage state only; no Phase 7 machinery yet

---

## §2 — Vector 2022 responsive behavior

Wikipedia's Vector 2022 skin uses a three-tier responsive system.

### Breakpoints

| Width | Layout | Left rail |
|---|---|---|
| `>1200px` | Wide desktop | Pinned left rail + right rail |
| `1000–1200px` | Desktop | Pinned left rail only |
| `<1000px` | Tablet / mobile | Left rail disappears; icon buttons in titlebar |

Max content width: 948px prose column, 1596px outer container.

### Left rail collapse mechanism (complex)

Wikipedia's approach is driven by the `pinnable-element` JavaScript module:

1. Each portlet (Navigation, Contents, Tools) is a `.vector-pinnable-element`.
2. At <1000px, `movePinnableElement()` transplants each portlet's DOM node from the
   `.vector-pinned-container` (left rail) into a corresponding titlebar dropdown trigger.
3. State (pinned vs. unpinned) is saved per-portlet via `clientPrefs` (cookie for anonymous
   users; server preference for logged-in users).
4. Each unpinned portlet renders as a dropdown panel anchored below its titlebar icon button.

This is 2,000+ lines of JavaScript and a complex preference system. It is not worth
replicating at this scale.

### Sticky header

Wikipedia's sticky header uses `IntersectionObserver` on the main `#mw-header` element:

1. A second, hidden `<div id="vector-sticky-header">` lives just after `<body>`.
2. When `#mw-header` scrolls off-screen, the observer adds `vector-sticky-header-visible`
   to the sticky header element.
3. The sticky header contains: site logo, current page title, current section (updated by
   a second IntersectionObserver on each `<h2>`), and an Edit button.
4. `scroll-margin-top: 75px` is applied to all heading elements so anchored jumps clear
   the sticky bar.

### Active section tracking

An IntersectionObserver watches each `<h2>` and `<h3>` in the article body. When a
heading enters the viewport, the ToC entry for that section receives the
`vector-toc-list-item-active` class (styled as bold + left blue bar in Vector 2022).

---

## §3 — MinervaNeue (Wikipedia mobile skin)

MinervaNeue is a separate skin served to mobile user agents — not just Vector 2022 at
narrow widths. It is a fundamentally different layout.

### Key patterns

| Pattern | Detail |
|---|---|
| Hamburger drawer | Left slide-in; no overlay grid |
| Collapsible sections | Each `<h2>` section (and its content up to the next `<h2>`) wraps in a `<section>` element; JS toggle collapse/expand; localStorage persistence per section |
| No persistent left rail | Navigation is only in the hamburger drawer; ToC is in a floating button at bottom-right |
| Bottom fixed toolbar | Search icon, Talk icon, History icon — fixed to bottom of viewport |
| Lead section always visible | Only the lead (before first `<h2>`) is always expanded; all later sections default to collapsed on narrow screens |
| In-place footnote drawers | Tapping a footnote reference `[1]` opens a bottom-sheet overlay showing the footnote text; no anchor jump to footnotes section |

### Leapfrog opportunity

MinervaNeue collapsible sections significantly reduce scroll depth on mobile — a user
can scan section headings and expand only what they need. This directly addresses the
SMB use case of "quick look-up, not deep reading." The implementation is straightforward
(no need for the full Minerva skin framework — just the section-collapse pattern).

---

## §4 — Leapfrog approach for this engine

The goal is Wikipedia muscle memory at faster implementation speed than Wikipedia's
own complex machinery. CSS-first, JS-minimal.

### Breakpoint selection

Use **960px** as the collapse threshold (between Wikipedia's 1000px and the current 768px).
This catches tablets (iPad Air = 820px, iPad Pro = 1024px in portrait) more reliably than
768px, which is too narrow (misses most tablets in landscape orientation).

### Left-rail collapse (avoids Wikipedia's DOM transplant)

Instead of transplanting portlet DOM nodes like Wikipedia does, always render the left
rail in the DOM but hide it at <960px via CSS. Always render three icon buttons in the
header at all widths; at ≥960px, hide the icon buttons via CSS.

```
Desktop (≥960px): left rail visible; icon buttons display: none
Mobile  (<960px): left rail display: none; icon buttons display: flex
```

The ToC drawer and Nav drawer are independent `<div>` elements rendered once in the HTML,
toggled by JS (mirroring the existing `mobile-nav-drawer` pattern). This means:

- Zero DOM transplant
- Works without JS for basic reading (left rail hidden is still readable)
- Two short `initTocDrawer()` and `initToolsDrawer()` functions mirroring `initMobileNav()`

### Sticky header

A `<div id="wiki-sticky-header">` is added just after the opening `<body>` tag, hidden by
default (`display: none`). `IntersectionObserver` on `#wiki-site-header` (the main logo bar
at the top of every page) toggles `wiki-sticky-visible` class. The sticky header contains:
site logo + page title + an Edit shortcut button.

`scroll-margin-top: 75px` needs to be added to all `h2`, `h3` elements in the article CSS
so anchor-jump links clear the sticky bar.

### Active ToC tracking

`IntersectionObserver` on each heading (`h2`, `h3`) in `.wiki-body`. When a heading enters
the viewport (with a top-margin offset to account for the sticky header), find its `id`,
locate `a[href="#<id>"]` in `#toc-list`, and add `toc-section-active` to its parent `<li>`.
Remove the class from all other items when a new heading becomes active.

---

## §5 — Priority implementation order

Ranked by impact on the SMB "wow factor" — the feeling of using a well-engineered,
fast, trustworthy wiki that feels as familiar as Wikipedia but built to a higher standard.

### Priority 1 — Responsive collapse (CRITICAL / BREAKING)

**Why critical:** At <768px the left rail renders below the article. On a phone, a user
lands on an article and must scroll past the entire Navigation portlet + ToC before
reaching the article body. This is broken UX, not just suboptimal UX.

**What to do:**
- Change the mobile breakpoint from 768px to 960px for left-rail collapse
- Change `order: 2` to `display: none` for `.wiki-left-rail` at <960px
- Add Contents icon button to the header titlebar (next to the existing hamburger)
- New `<div id="wiki-toc-drawer">` mirrors the existing `mobile-nav-drawer` structure
- `initTocDrawer()` in wiki.js (≈50 lines, mirrors `initMobileNav()`)
- CSS for the new drawer and icon button (≈120 lines)

**Effort:** ~1 session. No server-side changes except HTML for the icon button + drawer.

### Priority 2 — Sticky header

**Why important:** Long articles (>3 screens) lose the logo and navigation context.
Wikipedia's sticky header is one of the highest-recognition muscle memory elements —
users expect to see the page title and Edit button as they scroll.

**What to do:**
- Add `<div id="wiki-sticky-header">` to `server.rs` (just after `<body>`)
- `IntersectionObserver` on `#wiki-site-header` → toggle `wiki-sticky-visible`
- CSS: `position: fixed; top: 0; height: 50px; z-index: 100` when visible
- Add `scroll-margin-top: 75px` to article `h2`, `h3` in style.css

**Effort:** ~0.5 session.

### Priority 3 — Active ToC section tracking

**Why important:** Gives the reader location awareness in long articles. One of the most
praised Vector 2022 features. Technically simple.

**What to do:**
- `IntersectionObserver` on each `h2`/`h3` in `.wiki-body`
- Update `toc-section-active` class on the matching ToC `<li>`
- CSS: left blue bar + bold text on active item (matches Vector 2022 visual pattern)

**Effort:** ~0.5 session (60 lines JS, 10 lines CSS).

### Priority 4 — Mobile collapsible sections

**Why important:** Reduces scroll depth on mobile. Matches MinervaNeue muscle memory for
users who access Wikipedia on phones. Critical for SMB field workers checking articles
on phones.

**What to do:**
- At <960px: wrap each `<h2>` + following content in a `<section>` via JS
- Click on `<h2>` toggles `section-collapsed` class
- localStorage persists open/closed state per slug + section

**Effort:** ~1 session (80 lines JS, 30 lines CSS).

### Priority 5 — ToC in mobile drawer

**Why important:** Currently on mobile, there is no way to navigate to a specific section
without scrolling through the entire article. The hamburger drawer shows Navigation links
but not the article ToC.

**What to do:**
- Add a ToC list to the top of `mobile-nav-drawer` (same `numbered_headings` data
  already rendered in the left-rail ToC)
- Add a "Contents" heading and separator above the "Navigation" heading in the drawer

**Effort:** ~0.5 session (40 lines server.rs, 20 lines CSS).

---

## §6 — Implementation sprint summary

| Sprint | Goal | Priority | Effort |
|---|---|---|---|
| G | Responsive collapse (<960px left-rail hide + ToC drawer) | CRITICAL | 1 session |
| H | Sticky header (IntersectionObserver + 50px bar) | High | 0.5 session |
| I | Active ToC tracking (IntersectionObserver per heading) | High | 0.5 session |
| J | Mobile collapsible h2 sections | Medium | 1 session |
| K | ToC content in mobile hamburger drawer | Medium | 0.5 session |

Sprints G + H + I bring the engine from ~78% to ~90% Wikipedia muscle memory on mobile.
Sprints J + K close the remaining gap for phone users.
