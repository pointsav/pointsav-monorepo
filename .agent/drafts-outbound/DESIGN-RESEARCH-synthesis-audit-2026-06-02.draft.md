---
schema: foundry-draft-v1
type: DESIGN-RESEARCH
language_protocol: DESIGN-RESEARCH
title: "Leapfrog 2030 audit synthesis — browser-in-loop: home.woodfinegroup.com + home.pointsav.com"
target_project: project-design
target_destination: pointsav-design-system
created: 2026-06-02T00:00:00Z
author: totebox@project-marketing (claude-code / synthesis)
research_trail:
  source_commits:
    - 9b0543d (cluster/project-marketing HEAD — prior artifacts commit)
  prior_drafts:
    - .agent/drafts-outbound/DESIGN-RESEARCH-alpha-accessibility.draft.md
    - .agent/drafts-outbound/DESIGN-RESEARCH-beta-leapfrog2030.draft.md
    - .agent/drafts-outbound/DESIGN-RESEARCH-gamma-mobile-performance.draft.md
  citations:
    - conventions/compounding-substrate.md
    - conventions/worm-ledger-design.md
  operator_inputs:
    - "Browser-in-the-loop Playwright audit 2026-06-02"
    - "Three competing Opus agents: Alpha (accessibility), Beta (design), Gamma (mobile/perf)"
    - "Leapfrog 2030 award-winning target + mobile accessibility"
  related_files:
    - outputs/audit-2026-06-02/ (24 screenshots, 36 JSON audit files)
    - scripts/audit-2026-06-02.py
    - scripts/audit-2026-06-02-part2.py
---

# Leapfrog 2030 audit — synthesis

Three competing Opus agents audited `home.woodfinegroup.com` and `home.pointsav.com` independently on 2026-06-02, each from a distinct lens: accessibility and WCAG 2.2 compliance (Alpha), design excellence and forward-looking CSS (Beta), mobile experience and performance (Gamma). This document synthesises the three outputs into a unified, prioritised backlog for project-design and the v0.0.2 sprint.

Findings where two or more agents independently converge carry the highest confidence. Findings unique to one agent are still actionable but flagged.

---

## Audit method

- Playwright 1.60.0 (Python); Chromium headless
- Sites tested at localhost (ports 9101/9102) — avoids CDN/caching effects
- Viewports: 375×812, 768×1024, 1024×768, 1440×900
- axe-core: 4 violations (Woodfine), 5 violations (PointSav) — supplemented by manual inspection
- Instruments: tab-order traversal (35 stops), touch-target sizing, CSS token extraction, contrast computation, meta/ARIA inventory
- Output: 24 PNG screenshots + 36 JSON files in `outputs/audit-2026-06-02/`

---

## Where all three agents agree (highest confidence)

| Finding | Alpha | Beta | Gamma |
|---|:---:|:---:|:---:|
| Keyboard / tab order loops infinitely | ✅ P0 | ✅ noted | ✅ P0 |
| Mobile nav at 375px is unusable | ✅ | ✅ | ✅ P0 |
| Nav + footer link touch targets 17px — fail WCAG 2.5.5 | ✅ P0 | ✅ noted | ✅ P0 |
| 2.4 MB index.html — performance crisis | ✅ noted | ✅ noted | ✅ P0 |
| Missing H1 on both home pages | ✅ P1 | ✅ | — |
| All SVGs missing `<title>` | ✅ P1 | ✅ | — |
| No skip navigation link | ✅ P1 | — | — |

The tab loop, mobile nav failure, undersized touch targets, and 2.4 MB bundle are agreed P0 findings. These four items define the minimum v0.0.2 scope.

---

## Master improvement backlog

### P0 — Fix before next release (blocks users or violates WCAG 2.x Level AA/A)

**P0-1. Keyboard trap — cyclic tab order** *(Alpha P0-1, Gamma §6)*
WCAG 2.1.2 No Keyboard Trap — Level **A**. After the 12 focusable elements, focus lands on `<body>` and cycles indefinitely. Keyboard-only users cannot exit to the browser chrome.

Root cause theory (Gamma): the bundler is likely duplicating the nav DOM across multiple virtual "pages" in the 2.4 MB HTML file — the same navigation appears multiple times and Tab cycles through all copies. Extracting fonts and CSS (P0-4) may resolve this as a side-effect; if not, inspect the bundler output for repeated `<nav>` elements and remove duplicates.

Remediation: remove `tabindex` from `<body>`; ensure the document has exactly one copy of each nav; verify that after the last focusable element `Tab` hands control to the browser chrome.

Effort: 1h diagnosis + 2h fix.

---

**P0-2. Mobile nav — 375px unusable** *(Gamma P0-2, Beta §4, Alpha mobile section)*
The `1fr auto 1fr` CSS grid with 6 navigation links does not collapse correctly at 375px. The 6 links (3 left + 3 right of the centred wordmark) compress into a single unreadable horizontal strip — effective touch targets are below 20px in both dimensions at device pixel ratio 3×.

Gamma recommends **two-row nav** (no hamburger, per IA component map constraint): top row = wordmark + primary links (DISCLAIMER, CONTACT US); bottom row = secondary/external links (CORPORATE, PROJECTS, NEWSROOM). Full CSS implementation in Gamma §3.

```css
@media (max-width: 480px) {
  .topnav {
    display: grid;
    grid-template-areas:
      "logo logo"
      "left right";
    grid-template-columns: 1fr 1fr;
  }
  .topnav-wordmark { grid-area: logo; justify-self: center; }
  .topnav-left     { grid-area: left; }
  .topnav-right    { grid-area: right; text-align: right; }
  nav a {
    display: block;
    min-height: 44px;
    display: flex;
    align-items: center;
    padding: 0 8px;
  }
}
```

Effort: 3–4h.

---

**P0-3. Touch targets — nav and footer links at 17px** *(Alpha P0-7, Gamma P0-3)*
WCAG 2.5.8 Target Size (Minimum) — Level **AA** (new in 2.2). Nav links: 60–76 × 17px. Footer links: 73 × 16px. Minimum: 24 × 24px (AA) or 44 × 44px (AAA). 10 of 12 interactive elements fail.

Gamma's inline-flex fix avoids breaking the desktop grid visual:

```css
nav a, footer a {
  display: inline-flex;
  align-items: center;
  min-height: 44px;
  padding: 0 8px;
}
```

Effort: 1h.

---

**P0-4. Nav link contrast 4.34:1 — fails WCAG 1.4.3 AA** *(Alpha P0-4)*
Nav links render `--ink-3` (#6B7280) against a transparent background resolving to `--paper-2` (#F7F9FA). Computed ratio: 4.34:1. AA minimum: 4.5:1.

Alpha recommends introducing `--ink-nav: #5A6270` (darkened 5% from ink-3) or switching nav links to `--ink-2` (#374151, ratio 8.6:1). The token already exists; the nav simply uses the wrong one.

```css
nav a { color: var(--ink-2); }       /* #374151 — ratio 8.6:1, passes AAA */
footer a { color: var(--ink-2); }    /* same fix for footer 3.9:1 violation */
```

Effort: 30min.

---

**P0-5. PointSav: `aria-label` on SVG `<path>` elements** *(Alpha P0-2)*
WCAG 4.1.2 Name, Role, Value — Level **A**. axe `aria-prohibited-attr` violation. The `<path>` element is not allowed to carry `aria-label`. Move the label to the `<svg>` root:

```html
<!-- Current (invalid) -->
<path id="text1" aria-label="POINTSAV">

<!-- Remediation -->
<svg role="img" aria-labelledby="svg-title-pointsav">
  <title id="svg-title-pointsav">PointSav Digital Systems</title>
  <path id="text1">
</svg>
```

Effort: 30min.

---

**P0-6. PointSav: empty anchor `<a href="#"></a>`** *(Alpha P0-3)*
WCAG 2.4.4 Link Purpose — Level **A**. axe `link-name` violation. Remove the element or add `aria-label` + visible content.

Effort: 15min.

---

**P0-7. Performance — 2.4 MB inline bundle** *(Gamma P0-1, Beta §2, Alpha context)*
Both index.html files are ~2.4 MB. Gamma's payload breakdown: ~72% inline WOFF2 font data, ~19% base64 inflation overhead, ~9% HTML/CSS/content. At 5 Mbps (weak 4G): LCP ≈ 3.84s — "Poor" on Core Web Vitals. At 50 Mbps (fast 4G): ~0.38s — barely "Good."

Phase 1 fix (highest leverage, Gamma): extract all `@font-face` declarations to `tokens.css` (already exists at 1,649 bytes on Woodfine; expand it). Link externally with `<link rel="preload">`. Expected savings: ~1.8 MB from index.html, dropping weak-4G LCP to ~0.6s.

**CLS warning (Gamma):** Extracting inline fonts introduces a font-swap event where none currently exists. Mitigate with `size-adjust`, `ascent-override`, `descent-override` on fallback fonts to minimise layout shift.

Effort: 2h extraction + 1h CLS mitigation.

---

### P1 — Fix in v0.0.2 sprint (significant UX/accessibility gap)

**P1-1. Missing H1 on both home pages** *(Alpha P1-8, Beta §1)*
WCAG 2.4.6 Headings and Labels — Level **AA**. Neither home page has an `<h1>`. The wordmark SVG acts as the de-facto page identity but carries no heading semantics. axe flags `page-has-heading-one` as moderate violation.

Alpha recommends a visually hidden H1 that matches the page `<title>`:
```html
<h1 class="sr-only">Woodfine Capital Projects — Real Property Development</h1>
```

Beta recommends making the H1 visible: at 1440px, a 36px display heading in the hero band would improve visual hierarchy. Both are valid; project-design decides.

Effort: 30min.

---

**P1-2. All SVGs missing `<title>`** *(Alpha P1-11)*
WCAG 1.1.1 Non-text Content — Level **A**. Woodfine: 6 SVGs missing `<title>`. PointSav: 5 SVGs missing `<title>`. Inline SVG wordmarks need `role="img"` + `<title>` + `aria-labelledby`. Decorative SVGs (icons with adjacent visible text) need `aria-hidden="true"`.

Alpha provides a classification table and per-SVG remediation in the full draft.

Effort: 2h.

---

**P1-3. Missing skip navigation link** *(Alpha P1-12)*
WCAG 2.4.1 Bypass Blocks — Level **A**. No skip link exists on either site. Keyboard users Tab through all 6 nav links before reaching content on every page.

```html
<a href="#main-content" class="skip-link">Skip to main content</a>
```

```css
.skip-link {
  position: absolute; top: -100%; left: 0;
  background: var(--accent); color: #fff;
  padding: 12px 20px; font-size: 14px;
  z-index: 1000;
}
.skip-link:focus { top: 0; }
```

Effort: 1h.

---

**P1-4. Unlabeled nav landmarks** *(Alpha P1-9, axe `landmark-unique`)*
Two `<nav>` elements with no `aria-label` are indistinguishable to screen readers. The left nav and right nav need labels:

```html
<nav aria-label="Site">     <!-- DISCLAIMER, CONTACT US -->
<nav aria-label="External"> <!-- CORPORATE, PROJECTS, NEWSROOM -->
```

Effort: 15min.

---

**P1-5. Focus indicator — 1px outline insufficient** *(Alpha P1-13)*
WCAG 2.4.11 Focus Not Obscured (Minimum) — Level **AA**; WCAG 2.4.13 Focus Appearance — Level **AAA** (both new in 2.2). Tab order shows `outline_width: 1px` on all focusable elements. WCAG 2.4.13 requires focus indicator area ≥ perimeter of unfocused element × CSS pixel, and contrast ratio ≥ 3:1 against adjacent colours. A 1px solid outline almost certainly fails the area requirement.

Alpha recommends:
```css
:focus-visible {
  outline: 3px solid var(--accent);
  outline-offset: 3px;
  border-radius: 2px;
}
:focus:not(:focus-visible) { outline: none; }
```

Effort: 1h.

---

**P1-6. `prefers-reduced-motion` — no CSS handling** *(Alpha P2-15, Beta §3.2, Gamma §5)*
All three agents independently flagged this. When Leapfrog 2030 CSS animations land (scroll-driven, View Transitions), `prefers-reduced-motion` must gate them. Add baseline now:

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}
```

Effort: 30min.

---

### P2 — Leapfrog 2030 enhancements (raises to award-winning; v0.0.2–v0.0.3)

Beta's full draft contains working code for each. Summarised here by priority:

**P2-1. `prefers-color-scheme: dark`** — the existing `--paper`/`--ink`/`--accent` token system maps cleanly. Only two new tokens needed (`--ink-on-accent-surface` for hero text in dark mode). Beta provides complete implementation. *(Beta §3.4)*

**P2-2. CSS Container Queries** — replace topnav `@media` queries with `@container` so the component is portable. Beta shows the rewrite. *(Beta §3.1)*

**P2-3. Variable fonts — collapse 6 to 2** — current 6-font stack costs ~225 KB net of variable-font savings. Beta proposes Oswald Variable (not available) → Barlow (variable condensed axis) + Roboto Flex VF. Gamma's font extraction (P0-7) is a prerequisite. *(Beta §2, Gamma §7)*

**P2-4. `oklch()` colour space for brand tints** — `--accent-tint: #E8EFF7` is hand-picked. In oklch: `oklch(from var(--accent) 92% c h)` computes the tint mathematically. *(Beta §3.5)*

**P2-5. CSS Scroll-driven Animations** — hero band fade-in on `animation-timeline: view()`. Gate with `@media (prefers-reduced-motion: no-preference)`. *(Beta §3.2)*

**P2-6. View Transitions API** — wordmark morph on page navigation. Cross-document `@view-transition` with `view-transition-name` on the wordmark. *(Beta §3.3)*

**P2-7. Brand differentiation: Woodfine vs PointSav** — the two sites are near-identical templates. Beta proposes: PointSav flips to `--ps-steel` (#B4C5D5) dominant accent, near-black hero (#111827 on #F7F9FA), `--mono` font slot (JetBrains Mono for code blocks), denser grid. Woodfine keeps institutional deep blue. Token split via `[data-brand="woodfine"]` / `[data-brand="pointsav"]` on `<html>`. *(Beta §4)*

**P2-8. CSS Cascade Layers** — organise existing CSS as `@layer tokens, components, utilities`. *(Beta §3.6)*

**P2-9. Subgrid for property-type icon cards** — the 4 icons (Professional Centres, Tech Industrial, Retail Select, Suburban Office) sit in a flat flex container. Subgrid aligns labels across variable icon heights. *(Beta §3.8)*

**P2-10. `100svh` + safe-area-inset** — hero band should use `min-height: 100svh` (with `100vh` fallback) and `padding: env(safe-area-inset-top) env(safe-area-inset-right) ...` for notch/Dynamic Island devices. *(Gamma §5)*

**P2-11. AAA colour contrast via `--ink-2`** — switching nav and footer links from `--ink-3` (#6B7280, ratio 4.34) to `--ink-2` (#374151, ratio 8.6:1) achieves WCAG 1.4.6 AAA. Fixes P0-4 and P2 simultaneously. *(Alpha P2-17)*

---

### P3 — Aspirational (v0.1.0 and beyond)

- **Bilingual (English/Spanish)** — planned in website-congruence-plan.md; no drafts yet
- **`:has()` state-driven styling** — replace server-injected `class="active"` with CSS parent selectors *(Beta §3.7)*
- **BCSC disclosure footer language** — slot reserved in footer component draft; content authored separately
- **Newsroom server** — two-binary split (`service-rss` + `app-mediakit-marketing`); requires Master permission *(website-congruence-plan.md Phase 4)*
- **Contact form → DataGraph mutations** — forms as graph entity creation *(manifest v0.1.0)*

---

## v0.0.2 sprint scope recommendation

Based on effort estimates across all three agents:

| Item | Effort | Agent |
|---|---|---|
| P0-4: Nav contrast fix (`--ink-2`) | 30min | Alpha |
| P0-6: Empty anchor removal | 15min | Alpha |
| P0-5: PointSav SVG ARIA fix | 30min | Alpha |
| P0-3: Touch target min-height | 1h | Gamma |
| P1-4: Nav landmark labels | 15min | Alpha |
| P1-3: Skip navigation link | 1h | Alpha |
| P1-6: prefers-reduced-motion baseline | 30min | All |
| P1-5: Focus indicator upgrade | 1h | Alpha |
| P1-1: H1 (hidden) | 30min | Alpha |
| P1-2: SVG title elements | 2h | Alpha |
| P0-2: Mobile nav two-row layout | 4h | Gamma |
| P0-7: Font extraction to tokens.css | 3h | Gamma |
| P0-1: Keyboard trap diagnosis + fix | 3h | Alpha/Gamma |

**Total P0+P1 remediation: ~17h.** Eliminates all WCAG Level A and AA violations and closes the critical mobile nav failure.

Leapfrog 2030 enhancements (P2) are a separate sprint, planned after P0+P1 are clean and the token system is promoted to canonical via Stage 6.

---

## Site comparison at audit date (2026-06-02)

| Criterion | Woodfine | PointSav |
|---|---|---|
| axe violations | 4 | 5 |
| ARIA attributes | 6 | 14 |
| SVGs without `<title>` | 6 | 5 |
| Nav contrast | 4.34:1 ❌ | 2.19:1 ❌ |
| Body/general contrast | 9.1:1 ✅ | 9.1:1 ✅ |
| Invalid ARIA patterns | None | `aria-label` on `<path>` ❌ |
| Empty anchor | None | `<a href="#"></a>` ❌ |
| Skip link | No | No |
| H1 | No | No |
| Dark mode CSS | No | No |
| prefers-reduced-motion | No | No |
| font-display: swap | Yes | Yes |
| Touch targets passing 2.5.5 | 2/12 | 2/12 |

PointSav has more ARIA investment but also more ARIA errors — the `aria-label`-on-path pattern and empty anchor suggest hand-authoring without validation rather than systematic design-system derivation. This reinforces the case for shared design-system components over per-site authoring.

---

## Unique findings per agent (not in synthesis above)

**Alpha unique:** Screen reader narrative (VoiceOver announcement sequence before and after remediation); WCAG 2.5.3 Label in Name verification; `aria-current="page"` for active nav items; APG Tabs conformance question for MANIFEST/BIM/Location tab row.

**Beta unique:** Awwwards composite scores (Woodfine 5.2/10, PointSav 4.93/10); per-font contribution audit (Barlow Condensed, League Gothic, Mulish are effectively dormant); `opsz` optical size axis as Leapfrog 2030 typography move; metric-override CLS prevention for variable font swap.

**Gamma unique:** Bundler-duplication hypothesis for the tab loop and the 99× size differential between index.html and subpages; inline-font CLS advantage that extraction removes (mitigated by `size-adjust`); `viewport-fit=cover` needed alongside `safe-area-inset`; iOS momentum scroll `overflow: hidden` trap warning.

---

## Open questions for project-design

1. **H1 visibility**: Hidden H1 (SR-only) vs visible display heading? If visible, what typographic treatment in the hero band?
2. **Mobile nav approach**: Two-row (recommended) vs scroll-row vs bottom bar?
3. **Font extraction timing**: Should P0-7 (font extraction) happen before or simultaneously with P2-3 (variable font consolidation)? Doing both in one pass avoids a double CLS event.
4. **Brand differentiation**: Does operator want PointSav to diverge visually in v0.0.2 or later?
5. **Tab loop**: Is the cause bundler-side (duplicated nav DOM) or template-side (`tabindex` on body)? Requires inspection of the bundler output.

---

*Drafts ready for sweep: 4 DESIGN-RESEARCH files in `.agent/drafts-outbound/`. Gateway: project-design.*
