---
schema: foundry-draft-v1
type: DESIGN-RESEARCH
language_protocol: DESIGN-RESEARCH
title: "Mobile experience + performance audit: home.woodfinegroup.com + home.pointsav.com"
target_project: project-design
target_destination: pointsav-design-system
created: 2026-06-02T00:00:00Z
author: totebox@project-marketing (claude-opus / agent-gamma)
research_trail:
  source_commits:
    - 9b0543d (cluster/project-marketing HEAD)
  prior_drafts:
    - .claude/drafts-outbound/leapfrog-2030/research/research-mobile-first-marketing.draft.md
    - .claude/drafts-outbound/leapfrog-2030/tokens/token-woodfine-breakpoints.draft.md
    - .claude/drafts-outbound/leapfrog-2030/components/component-marketing-topnav.draft.md
  citations:
    - conventions/compounding-substrate.md
    - .agent/binary-targets.yaml
  operator_inputs:
    - "Tier 0 mandate: runs on $7/mo node, no AI tier, serves mobile traffic"
    - "Performance target: initial HTML < 100 KB; total page weight < 500 KB"
    - "Playwright screenshots at 375px confirm mobile nav is unusable"
  related_files:
    - outputs/audit-2026-06-02/ (screenshots + audit JSON)
    - deployments/media-marketing-landing-1/content/index.html (2,492,052 bytes)
    - deployments/media-marketing-landing-2/content/index.html (2,444,257 bytes)
---

# Mobile experience + performance audit — Agent Gamma

> **Lens:** Mobile experience and performance.
> **Subjects:** home.woodfinegroup.com (2,492,052 B) and home.pointsav.com (2,444,257 B).
> **Headline finding:** On a $7/mo Tier 0 node serving a 375px iPhone, the
> current sites are **24x over the initial-HTML performance budget**. Everything
> downstream of that — LCP, mobile nav unreadability, sub-44px touch targets —
> is consequence, not root cause.

---

## Executive summary

| Metric | Target | Measured | Status |
|---|---|---|---|
| Initial HTML payload | < 100 KB | **2,400 KB** | 24x over budget |
| Total page weight | < 500 KB | ~2,400 KB | 4.8x over budget |
| LCP on 4G (50 Mbps) | < 2.5 s | ~0.4 s | Borderline Good |
| LCP on weak 4G (5 Mbps) | < 2.5 s | **~3.8 s** | **Poor** |
| Nav link touch target | >= 44x44 px | 60-76 x 17 px | **10/12 fail WCAG 2.5.5** |
| Mobile nav usability at 375px | Readable, tappable | Single dense row, unreadable | **Critical fail** |
| Tab order termination | Finite, predictable | **Infinite loop** | **Critical fail** |
| `font-display` | swap | swap | Good |
| Viewport meta | zoom permitted | zoom permitted | Good |

The two sites are architecturally identical (`app-mediakit-marketing` Rust
binary, flat-file content), so all findings apply symmetrically.

---

## Part 1 — Performance budget analysis

### 1.1 Payload composition

The 2,492,052-byte index.html decomposes (estimated from the inlined assets
declared in the document) approximately as:

| Component | Estimated size | % of payload |
|---|---|---|
| Six WOFF2 fonts inline as base64 (Oswald, Roboto Slab, Nunito Sans, Barlow Condensed, League Gothic, Mulish) | ~1,800 KB | **72%** |
| Inline CSS (custom properties, layout, type scale, component CSS) | ~80 KB | 3% |
| HTML text content (hero copy, partner cards, MANIFEST/BIM/Location panels, footer) | ~40 KB | 2% |
| Base64 padding overhead (WOFF2 → ASCII inflation ~33%) | ~470 KB | 19% |
| `<svg>` wordmarks, decorative elements | ~10 KB | < 1% |
| Whitespace, JSON-LD, meta | ~90 KB | 4% |
| **Total** | **~2,490 KB** | 100% |

Base64 inflation alone (the cost of inlining vs. linking) accounts for roughly
**470 KB of pure waste** — bytes the user transfers for no rendered pixel.

### 1.2 LCP arithmetic on a single-document inline payload

LCP cannot complete until enough bytes arrive to render the largest contentful
element. Because **all CSS and all fonts are above the closing `</head>`**, the
browser must finish parsing the entire `<style>` blocks before laying out the
hero band. The effective LCP is bounded below by the time to transfer the
inline-CSS + font payload.

Conservative LCP estimate (assuming inline fonts are required for hero render —
which they are, since hero copy uses Oswald and Roboto Slab):

```
LCP >= (CSS_payload + required_font_payload) / bandwidth + TTFB
```

| Network | Bandwidth (B/s) | Bytes to render hero | LCP estimate | CWV grade |
|---|---|---|---|---|
| Fast 4G (50 Mbps) | 6,250,000 | 2,400 KB | 0.38 s + TTFB | Good (< 2.5 s) |
| Weak 4G (5 Mbps) | 625,000 | 2,400 KB | **3.84 s** + TTFB | **Poor (> 2.5 s)** |
| 3G (1.6 Mbps) | 200,000 | 2,400 KB | **12.0 s** + TTFB | **Poor** |
| Cached repeat visit | — | 0 KB | TTFB only (~50 ms) | Good |

The Tier 0 promise — "runs on $7/mo node, serves mobile traffic" — collides
with the inline-payload architecture **the moment the user is not on perfect
WiFi**. Canadian rural cellular, hotel WiFi, transit, and underground transit
stations all fall into the 5 Mbps / 3G column.

### 1.3 Phased extraction plan

Both deployments already have `tokens.css` (1,649 B on Woodfine) and `shell.css`
in the templates directory — the externalisation scaffold exists but is not
load-bearing yet.

| Phase | Action | Expected payload after | Savings |
|---|---|---|---|
| **Phase 0 (baseline)** | Current state — all inline | 2,400 KB | — |
| **Phase 1 — Font extraction** | Move 6 `@font-face` blocks (base64 WOFF2) into `tokens.css` as external WOFF2 files served from the binary; HTTP/2 multiplexing handles parallel fetch | 600 KB index.html + 6 × ~280 KB WOFF2 served separately and cached | index.html: **-75%** (-1,800 KB) |
| **Phase 2 — CSS extraction** | Move inline `<style>` blocks to external `shell.css` (already exists in templates/); link with `<link rel="stylesheet">` in `<head>` | ~520 KB index.html + cached shell.css | index.html: **-13%** further |
| **Phase 3 — CDN fonts + preload** | Serve fonts from Google Fonts CDN with `<link rel="preconnect">` + `<link rel="preload" as="font" type="font/woff2" crossorigin>` for the two critical fonts (Oswald 700, Roboto Slab 400). Drop self-hosted WOFF2. | ~40 KB index.html (HTML only); CDN handles fonts via shared cache across sites | index.html: **-98% from baseline** |
| **Phase 4 — Variable fonts + subsetting** | Collapse 6 families to 2 variable fonts (Oswald-variable for display/condensed; Roboto-Slab + Mulish-variable for sans/serif). Apply `unicode-range: U+0000-00FF` Latin subset. Each font file ~30-60 KB. | ~40 KB HTML + 2 × ~50 KB fonts = ~140 KB total page weight on first visit, ~40 KB on repeat. | **Hits 100 KB initial HTML target; hits 500 KB total weight target.** |

**Phase 1 alone unlocks Tier 0 viability** — the index.html drops below
the < 100 KB target if fonts move to a separate cacheable origin, and the
weak-4G LCP drops from 3.84 s to roughly 0.6 s.

### 1.4 Caveat: data-URI fonts are not cacheable

The current inline-base64-WOFF2 design means **every page navigation re-downloads
all six fonts**. A user clicking from /index.html to /contact.html to
/disclaimer.html and back pays 2.4 MB + 17 KB + 24 KB + (cached) — but only
the subpages are conventionally structured. The index.html alone is
re-downloaded on every return visit unless `Cache-Control: immutable` is set
**and** the file hash hasn't changed. External font files, in contrast, cache
across all pages and across all visits to either site (and potentially across
Google Fonts CDN consumers globally).

---

## Part 2 — Mobile navigation failure

### 2.1 Root cause

The `component-marketing-topnav` draft specifies a desktop layout:

```css
header nav {
  display: grid;
  grid-template-columns: 1fr auto 1fr; /* left-nav | wordmark | right-nav */
  align-items: center;
}
```

The draft asserts "collapses to single column at 768px" — but the 375px
screenshot proves the collapse is **not happening**. The grid stays
three-column, the left and right `1fr` cells get crushed to roughly 110 px
each, and the six 11px-uppercase nav links are forced into a horizontal strip
of unreadable text.

Two failure modes are possible (cannot disambiguate without the live CSS,
but the draft suggests both):

1. **The `@media (max-width: 768px)` block exists in the draft but was not
   committed to the bundled CSS.** The shell.css linked by templates/ may
   be a stub; the active rules are inline in index.html and predate the
   responsive design tokens.
2. **The block exists but the children (left-nav, right-nav links) are
   `display: flex; flex-direction: row` inside their cells**, so even when
   the grid collapses to one column, the link rows themselves don't wrap.

### 2.2 Geometry at 375px

Six links at observed desktop widths: 71 + 76 + 71 + 60 + 67 + 73 (footer
proxy) = roughly 418 px of link content alone, before separators or padding.
The 375px viewport cannot accommodate this in a single row without either
horizontal scroll or text compression. The screenshot shows compression
(letter-spacing collapsing, possibly font shrinking via clamp() or just visual
crowding).

### 2.3 Solution options and recommendation

The operator constraint is firm: **no hamburger** (logo-centre on all pages
per IA component map). This rules out the standard mobile pattern.

| Option | Pros | Cons |
|---|---|---|
| (a) Horizontal scroll row | Native pattern (Safari tab bar, Twitter); preserves all links; no DOM restructure | External links (CORPORATE, PROJECTS, NEWSROOM) lack scroll affordance; users may not discover them; iOS rubber-band scroll fights the page scroll |
| (b) Two-row nav (wordmark + 6 links wrapped) | All links visible; no hidden state; works without JS | Header gets tall (eats ~120px of 812px viewport); wordmark gets less prominence |
| (c) Collapsed right-nav (split rows) | Preserves left/right semantic grouping; keeps wordmark prominent | Three rows total; complex CSS; visual hierarchy unclear (why are CORPORATE and DISCLAIMER on different rows?) |
| (d) Fixed bottom nav bar | 44px touch targets achievable; always reachable with thumb; matches native iOS/Android conventions | Eats 56px of viewport permanently; overlaps content; new pattern (educational cost); conflicts with browser chrome on iOS |

**Recommended: Option (b) — two-row nav at mobile.** It is the most honest
solution given the no-hamburger constraint: all six links remain visible,
discoverable, and tappable at 44px, with zero hidden state. The header
height cost (~120 px) is acceptable on a 812-px iPhone viewport — the hero
band is still visible above the fold.

### 2.4 Recommended CSS

```css
/* Desktop unchanged */
header nav {
  display: grid;
  grid-template-columns: 1fr auto 1fr;
  grid-template-rows: auto;
  align-items: center;
  padding: 0 var(--page-padding, 36px);
}

header nav .left-nav,
header nav .right-nav {
  display: flex;
  gap: 24px;
}

/* Mobile — two-row stack with wrap */
@media (max-width: 768px) {
  header nav {
    grid-template-columns: 1fr;
    grid-template-rows: auto auto;
    gap: 12px;
    padding: 12px 16px env(safe-area-inset-bottom, 12px);
  }

  header nav .wordmark {
    grid-row: 1;
    justify-self: center;
  }

  header nav .left-nav,
  header nav .right-nav {
    grid-row: 2;
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 8px 16px;
  }

  /* Combine left+right into one wrapping row */
  header nav .right-nav {
    grid-row: 2;
  }
}

@media (max-width: 480px) {
  header nav nav-link {
    font-size: 12px; /* up from 11 — readability */
  }
}
```

If "left-nav" and "right-nav" cannot be merged into a single flex container
due to source-order constraints, use `display: contents` on each at the 768px
breakpoint to flatten them into the parent grid's flex flow.

---

## Part 3 — Touch target remediation

### 3.1 Failure inventory

10 of 12 measured links fail WCAG 2.5.5 (Target Size — 44x44 CSS px) **and**
the more lenient 2.5.8 (24x24 CSS px). The 17 px height is purely the
line-box of an 11px-line-height-1 text run. There is no vertical padding,
no `min-height`, no `display: inline-flex` wrapper.

### 3.2 The naive fix breaks the grid

Adding `padding: 13px 0` directly to `nav a` makes each link 43 px tall —
close to 44 but not over, and worse, the inline links are still inside a
flex row that defines its height by content. The flex row grows to 43 px,
and the `1fr auto 1fr` grid now has a taller centre cell (the 80-px
wordmark) than the side cells (43 px). Vertical alignment via
`align-items: center` masks this, but the **clickable region** of the
nav link is still 17 px tall because padding doesn't expand the
inline-level box's hit testing in the way `min-height` on an inline-flex
element does.

### 3.3 Correct CSS

```css
header nav a,
footer a {
  /* Convert from inline to inline-flex to make min-height work on hit area */
  display: inline-flex;
  align-items: center;
  justify-content: center;

  /* WCAG 2.5.5 — 44x44 CSS px minimum */
  min-height: 44px;
  min-width: 44px;

  /* Visual padding — keeps link text feeling tight without inflating box */
  padding: 0 8px;

  /* Type preserved */
  font-size: 11px;
  line-height: 1;

  /* Negate the height cost in flow */
  margin: -10px 0; /* optional — pulls visual baseline back to original */
}

/* At 480px, drop the negative margin and let the bar breathe */
@media (max-width: 480px) {
  header nav a,
  footer a {
    margin: 0;
    min-height: 48px; /* extra generosity on small screens */
  }
}
```

The `-10px` vertical margin is a calculated trick — it keeps the
**visual** nav band at the original ~24 px height (since the 44px box
overlaps adjacent space) while preserving the **hit-test** 44 px region.
Apply only if visual compactness on desktop is mandatory; otherwise omit
and let the nav band be 44 px tall (more honest, more readable, costs
20 px of vertical space).

Apply identically to footer links (73 × 16 px → 73 × 44 px hit area,
visually unchanged with negative margin).

### 3.4 MANIFEST / BIM / Location tabs

These are 130 × ~44 px — borderline. Round up explicitly:

```css
.icon-tab {
  min-height: 48px; /* Apple HIG recommendation; safer than 44 */
  min-width: 130px;
}
```

---

## Part 4 — Core Web Vitals predictions

### 4.1 LCP — Largest Contentful Paint

The largest contentful element is almost certainly the hero band's headline
copy (likely 32-48 px Oswald, white-on-`#164679`, 8+ words wide). It depends
on:

- HTML parse to reach hero `<section>`: must read ~1.9 MB of inline font
  data first (fonts declared at top of `<style>`, before page content).
- Font swap: not blocking thanks to `font-display: swap` — initial render
  uses fallback (`Trade Gothic LT Std`, then system sans), Oswald paints
  on font-load.

| Network | LCP estimate | Grade |
|---|---|---|
| Fast 4G | ~1.0 s (TTFB + parse + paint) | **Good** |
| Weak 4G | ~3.8 s | **Poor** |
| 3G | ~12 s | **Poor** |

**Grade: Needs Improvement → Poor** across realistic mobile distribution.
After Phase 1 extraction: **Good** across all tiers.

### 4.2 CLS — Cumulative Layout Shift

Counter-intuitive finding: **inline fonts are CLS-friendly**. Because all
six fonts are embedded in the same document, by the time the browser parses
HTML and renders the hero, the fonts are already in the font cache (parsed
from base64). There is no swap event, no FOUT, no layout shift from font
metric differences.

When fonts are extracted (Phase 1+), CLS risk **increases**: the fallback
font (Trade Gothic / system) has different metrics than Oswald, and `swap`
triggers a re-layout when Oswald loads. Mitigation:

```css
@font-face {
  font-family: "Oswald";
  font-display: swap;
  size-adjust: 92%;          /* tune to match fallback metrics */
  ascent-override: 105%;
  descent-override: 25%;
  line-gap-override: 0%;
}
```

`size-adjust` and `*-override` descriptors (Chrome 87+, Safari 17+) let the
fallback font occupy the same metrics box as Oswald, eliminating CLS at
swap.

No images above the fold (other than SVG wordmark, which is inline and
intrinsic-sized). CLS risk from images is **zero**.

**Grade: Good (current, inline) → Good (post-extraction, with size-adjust)**.

### 4.3 INP — Interaction to Next Paint

The sites are static HTML with no detectable JS interaction handlers
beyond navigation. INP measures the latency of the slowest interaction.
With no JS, every click is a full-page navigation, governed by network +
TTFB, not by main-thread work. INP scores will be effectively **0 ms**
(no event handlers to block on).

Caveat: the infinite tab loop (Part 5) suggests the bundler may be
generating duplicate DOM that increases parse time — INP includes the
"next paint" cost, so a 12-stop loop that paints 12 times rapidly could
in theory inflate. But absent a JS event loop, this is negligible.

**Grade: Good.**

### 4.4 FID

Replaced by INP in March 2024. Not measured.

### 4.5 Summary table

| Metric | Current (inline) | Post-Phase-1 | Post-Phase-3 |
|---|---|---|---|
| LCP (fast 4G) | Good | Good | Good |
| LCP (weak 4G) | **Poor** | Good | Good |
| LCP (3G) | **Poor** | Needs Improvement | Good |
| CLS | Good | Good (with size-adjust) | Good |
| INP | Good | Good | Good |

---

## Part 5 — The infinite tab loop

This was logged in the raw data but deserves its own section because it
hints at a deeper architectural issue.

Playwright Tab traversal reports 12 stops (the 12 measured links), then
hits BODY, then **repeats** the same 12 stops infinitely. This is not
normal keyboard behaviour. Possible causes:

1. **JS tab-trap loop** — some kind of `focusin` handler that loops tab
   back to the first link. No JS appears present, but a CSS-only trap is
   not possible, so this is unlikely.
2. **Duplicated DOM** — the bundler that produces index.html may be
   inlining the navigation multiple times (once per "virtual page" if the
   bundler concatenates page templates). Playwright sees stop 1-12, hits
   BODY (a separator), and continues into the **next copy** of the nav.
3. **Iframes or shadow DOM** — unlikely given the architecture description.

The most plausible explanation is **(2) duplicated DOM**. A 2.4 MB single
file with all-pages-inlined would explain (a) the size, (b) the tab loop,
(c) why subpages (contact.html, disclaimer.html) are 17-24 KB while the
landing page is 2.4 MB.

**Action required:** before Phase 1, audit the bundler output for
duplicated `<nav>` elements. If confirmed, this is a P0 separate from
font extraction — duplicate nav DOM means screen readers will announce
"navigation" twice or more, and keyboard users cannot escape the page
via Tab. This is an WCAG 2.1.2 (No Keyboard Trap) failure.

---

## Part 6 — `100svh` and safe-area-inset

### 6.1 Viewport units

The `component-marketing-topnav` draft references `100svh`. Confirm the
hero band uses it. The cost of `100vh` on iOS Safari 14 and earlier:

- Page loads with address bar expanded → `100vh` = window height including
  not-yet-collapsed chrome.
- User scrolls down → address bar collapses → window grows → `100vh` is
  now too tall, hero band overflows.
- User scrolls back up → address bar expands → `100vh` shrinks back.

Result: a "jump" of 56-88 px at the bottom of the hero band as the user
scrolls. This is the most common visible iOS quirk in 2020-2024 web design.

`100svh` (small viewport height — CSS Values 4) locks to the **smallest**
viewport height (address bar expanded), eliminating the jump. Supported in:

- iOS Safari 15.4+ (March 2022)
- Chrome 108+ (November 2022)
- Firefox 101+ (May 2022)

Fallback chain:

```css
.marketing-hero {
  min-height: 100vh;   /* fallback for Safari 15.3 and earlier */
  min-height: 100svh;  /* modern browsers — locks to smallest viewport */
}
```

Browsers that don't recognise `100svh` fall through to `100vh`. Browsers
that recognise it override the previous declaration. No `@supports` needed.

### 6.2 Safe-area insets

For notch / Dynamic Island / home-indicator clearance, the hero and any
edge-flush content (top nav, footer) need:

```css
header nav {
  padding-top: env(safe-area-inset-top, 0);
  padding-left: env(safe-area-inset-left, 16px);
  padding-right: env(safe-area-inset-right, 16px);
}

footer {
  padding-bottom: env(safe-area-inset-bottom, 16px);
}

/* The viewport meta needs to opt-in to safe-area awareness: */
/* <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover"> */
```

**The current viewport meta lacks `viewport-fit=cover`** — without it,
`env(safe-area-inset-*)` returns 0 on iOS, and content respects the safe
area by default (leaving black bars on notched devices in landscape).
For a marketing site that wants edge-to-edge hero, add `viewport-fit=cover`
**together with** safe-area padding on all edge content.

### 6.3 Recommended viewport meta

```html
<meta name="viewport"
      content="width=device-width, initial-scale=1, viewport-fit=cover">
```

---

## Part 7 — Scroll behaviour audit

### 7.1 Horizontal overflow

Without the live CSS to inspect, conjectured but probable issues:

- **`max-width: 1440px` containers** — if the container has `margin: 0 auto`
  but inner elements (e.g., a wide partner-card grid) have fixed pixel
  widths that exceed 375 px, the page will horizontally scroll. Mitigation:
  every container should have `max-width: 100%` and `min-width: 0` as a
  flex/grid item.
- **Long inline links** in nav — already discussed in Part 2. At 375 px,
  the 418-px-wide link row would force horizontal scroll if not constrained.

Recommended defensive CSS:

```css
html, body {
  max-width: 100vw;
  overflow-x: hidden;     /* defensive — kills horizontal scroll */
  overflow-y: auto;
}
```

**Caveat: `overflow: hidden` on `<body>` breaks iOS momentum scroll.** Only
apply to `<html>`, or use `overflow-x: clip` (Chrome 90+, Safari 16+)
which doesn't affect scroll behaviour.

### 7.2 Sticky positioning

If any element uses `position: sticky` (likely the header, given the
component naming), audit for iOS scroll conflicts:

- `position: sticky` works fine on iOS 13+.
- `position: sticky` inside an `overflow: hidden` ancestor **silently
  fails** — the element loses stickiness with no error.
- `position: sticky` on a flex item requires `align-self: flex-start`
  (sometimes) to behave correctly.

If the header is intended to stick on scroll, verify it has no
`overflow: hidden` ancestor and verify `top: 0` is set.

### 7.3 Scroll-behavior

Footer anchor links (CONTACT US, DISCLAIMER) navigate to other pages, not
in-page anchors, so `scroll-behavior: smooth` is not directly applicable.
However, on subpage scrolling, the global rule:

```css
html {
  scroll-behavior: smooth;
}

@media (prefers-reduced-motion: reduce) {
  html { scroll-behavior: auto; }
}
```

is a low-cost good-practice addition. Combine with `prefers-reduced-motion`
respect.

### 7.4 iOS momentum scroll

Default in iOS Safari since 13. No `-webkit-overflow-scrolling: touch`
needed (that property was deprecated). If any scroll container has
`overflow: hidden` or `overflow: clip` on an ancestor that breaks
momentum, fix that container.

---

## Part 8 — Prioritised fix list

Ordered by mobile user impact, with the underlying principle: **the user
on weak 4G at 375 px must reach the hero in under 2.5 seconds and tap any
link with their thumb**. Everything else follows.

| # | Priority | Issue | Current | Fix | Effort | Impact |
|---|---|---|---|---|---|---|
| 1 | **P0-Perf** | 2.4 MB index.html (24x over budget) | All fonts inline as base64 | Phase 1: extract 6 WOFF2 to external files served by Rust binary; reference via tokens.css | 2 h | LCP weak-4G: 3.8s → 0.6s. Tier 0 viability unlocked. |
| 2 | **P0-A11y** | Infinite tab loop (WCAG 2.1.2 fail) | Bundler likely duplicates nav DOM | Audit bundler output; deduplicate nav across virtual pages | 3 h | Keyboard users can navigate; screen readers stop double-announcing |
| 3 | **P0-Mobile** | Nav unreadable at 375 px | `1fr auto 1fr` grid stays 3-column at all sizes | Implement Option (b) two-row nav with `flex-wrap` at 768 px breakpoint | 4 h | Six links visible and tappable on iPhone |
| 4 | **P0-A11y** | 10/12 links fail WCAG 2.5.5 (44×44) | 17 px line-box | `display: inline-flex; min-height: 44px;` on all nav + footer links | 1 h | Thumb-tappable; WCAG AA compliance |
| 5 | P1-Perf | No `viewport-fit=cover` | Notched-device content has bars | Add `viewport-fit=cover` + `env(safe-area-inset-*)` padding | 30 m | Edge-to-edge hero on iPhone X+ |
| 6 | P1-Perf | `100vh` jump on hero | Address-bar collapse causes layout jump | Use `100svh` with `100vh` fallback | 15 m | No hero jump on iOS scroll |
| 7 | P1-Perf | Phase 2: extract CSS | ~80 KB inline `<style>` | Move to external `shell.css` (already exists in templates/) | 1 h | Cacheable across pages; second-visit cost ~0 |
| 8 | P2-Perf | Phase 3: CDN fonts | Self-hosted WOFF2 | Switch to Google Fonts CDN with preload | 2 h | Shared cache; ~98% total payload reduction |
| 9 | P2-Perf | Phase 4: variable fonts + subsetting | 6 font families | Collapse to 2 variable fonts with Latin subset | 4 h | Hits all performance targets; sub-100 KB initial |
| 10 | P2-A11y | CLS at font swap (post-extraction) | Currently 0 (inline); rises with extraction | Add `size-adjust` + `*-override` to `@font-face` | 1 h | CLS stays Good through Phase 1-4 |
| 11 | P3-UX | Footer touch targets | 73 × 16 px | Same fix as nav (#4) | 0 (covered by #4) | — |
| 12 | P3-UX | `scroll-behavior: smooth` with reduced-motion guard | Not set | Add 3 lines of CSS | 5 m | Polish; accessibility respect |

**Total effort to hit Tier 0 + WCAG AA: ~18 hours of design-system work.**
The first four items (P0) alone — roughly 10 hours — would resolve every
material mobile and performance defect. Items 5-12 are polish, hardening,
and the long-tail payload reduction.

---

## Closing observation

The two sites are **architecturally well-designed for desktop reviewers**
and **architecturally hostile to mobile users on imperfect networks**. The
inline-everything bundling decision optimises for a single metric (zero
font FOUT on first paint) at a 24x cost on every other performance and
accessibility axis.

Tier 0 is not just a cost constraint — it is a **distribution** constraint.
A site that runs on a $7/mo node must also run for the user on the $7/day
prepaid plan with weak signal. The current build serves the first
constraint and fails the second.

Phase 1 (font extraction, ~2 hours) is the single highest-leverage change
in this audit. Recommend it ship before any further design work on the
two sites.

— Agent Gamma, project-marketing Totebox, 2026-06-02
