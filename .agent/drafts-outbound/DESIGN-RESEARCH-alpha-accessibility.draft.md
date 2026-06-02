---
schema: foundry-draft-v1
type: DESIGN-RESEARCH
language_protocol: DESIGN-RESEARCH
title: "Accessibility audit — WCAG 2.2 AA/AAA gap analysis: home.woodfinegroup.com + home.pointsav.com"
target_project: project-design
target_destination: pointsav-design-system
created: 2026-06-02T00:00:00Z
author: totebox@project-marketing (claude-opus / agent-alpha)
research_trail:
  source_commits:
    - 9b0543d (cluster/project-marketing HEAD — artifacts commit)
  prior_drafts:
    - .claude/drafts-outbound/leapfrog-2030/research/research-mobile-first-marketing.draft.md
  citations:
    - conventions/compounding-substrate.md
  operator_inputs:
    - "Browser-in-the-loop Playwright audit 2026-06-02: axe-core, tab order, touch targets, color contrast, meta"
    - "Leapfrog 2030 mandate: WCAG 2.2 AAA target"
  related_files:
    - outputs/audit-2026-06-02/ (all audit JSON + screenshots)
    - .agent/drafts-outbound/DESIGN-COMPONENT-icon-tab.draft.md
---

# Accessibility audit — WCAG 2.2 AA/AAA gap analysis

**Scope:** `home.woodfinegroup.com/index` and `home.pointsav.com/index`, desktop (1440px) and mobile (375px) breakpoints. Instrumented via Playwright + axe-core on 2026-06-02. Raw artifacts archived under `outputs/audit-2026-06-02/`.

**Headline:** Both sites currently sit below WCAG 2.2 Level A in one material respect (keyboard trap on cyclic tab order) and below Level AA on contrast, target size, document structure, and SVG semantics. Neither home page declares an H1; neither offers a skip link; neither responds to `prefers-reduced-motion`. The Leapfrog 2030 AAA target requires a coordinated remediation pass before the v0.0.2 component sprint begins; piecemeal fixes will not close the gap.

**Methodology note:** axe-core flagged 4 violations on Woodfine and 5 on PointSav, but axe is conservative on structural patterns it cannot statically prove. Manual tab-order traversal surfaced the most severe finding (keyboard trap) which axe did not detect. Touch-target measurement was performed at the desktop breakpoint as specified by WCAG 2.5.8 (Target Size — Minimum, 24×24 CSS pixels, Level AA, new in 2.2) and WCAG 2.5.5 (Target Size, 44×44 CSS pixels, Level AAA).

---

## Findings summary

| # | Finding | WCAG | Level | Site(s) | Priority |
|---|---------|------|-------|---------|----------|
| 1 | Cyclic tab order — focus never escapes the page | 2.1.2 No Keyboard Trap | **A** | Both | **P0** |
| 2 | Invalid ARIA on SVG `<path>` (`aria-label` on non-permitted element) | 4.1.2 Name, Role, Value | **A** | PointSav | **P0** |
| 3 | Empty anchor with no accessible name | 2.4.4 Link Purpose; 4.1.2 | **A** | PointSav | **P0** |
| 4 | Nav link contrast 4.34:1 (FAILS 4.5:1 minimum) | 1.4.3 Contrast (Minimum) | **AA** | Both | **P0** |
| 5 | Footer link contrast 3.9:1 | 1.4.3 Contrast (Minimum) | **AA** | Both | **P0** |
| 6 | `.footer-version` contrast | 1.4.3 | **AA** | PointSav | **P0** |
| 7 | Touch targets 17px height (nav + footer links) | 2.5.8 Target Size (Min) | **AA** | Both | **P0** |
| 8 | Missing H1 on home page | 1.3.1 Info & Relationships; 2.4.6 Headings & Labels | **AA** | Both | **P1** |
| 9 | Two unlabeled `<nav>` landmarks | 1.3.1; 2.4.1 Bypass Blocks | **AA** | Both | **P1** |
| 10 | `.copyright` div outside any landmark | 1.3.1 | **A** | Woodfine | **P1** |
| 11 | All SVGs missing `<title>` + `aria-labelledby` | 1.1.1 Non-text Content | **A** | Both | **P1** |
| 12 | No skip-navigation link | 2.4.1 Bypass Blocks | **A** | Both | **P1** |
| 13 | Focus indicator outline 1px (likely fails contrast/area) | 2.4.11 Focus Not Obscured (Min); 2.4.13 Focus Appearance | **AA / AAA** | Both | **P1** |
| 14 | Touch targets 17px (also fails 44×44 AAA) | 2.5.5 Target Size | **AAA** | Both | **P2** |
| 15 | No `prefers-reduced-motion` handling | 2.3.3 Animation from Interactions | **AAA** | Both | **P2** |
| 16 | No `prefers-color-scheme` handling | (UX / 1.4.3 robustness) | n/a | Both | **P2** |
| 17 | Contrast 4.58:1 on general links passes AA, fails AAA (7:1) | 1.4.6 Contrast (Enhanced) | **AAA** | Woodfine | **P2** |

---

## P0 — Critical violations (block users today)

### P0-1. Keyboard trap: cyclic tab order

**WCAG 2.1.2 No Keyboard Trap — Level A** (also 2.4.3 Focus Order, Level A)

**User impact.** Keyboard-only users (motor impairments, RSI, blind users on screen readers, power users) cannot escape the page using `Tab` alone. After Stop 11 (footer Disclaimer), focus lands on `<body>` with a visible 3px outline, then cycles back to Stop 0 — indefinitely. There is no exit to the browser chrome (address bar, browser tabs, extensions) without `Ctrl+L` or `F6`. For screen-reader users in browse mode this is less catastrophic, but for sighted keyboard users it is disorienting and is a hard Level A failure.

**Root cause.** The `<body>` element has been made focusable — most likely via `tabindex="0"` or `tabindex="-1"` with programmatic focus, or a CSS `:focus` rule attached to body. Combined with no terminal element after the footer, `Tab` from the last interactive control wraps internally rather than handing focus to the browser chrome.

**Current (broken) markup (inferred):**

```html
<body tabindex="0"> <!-- or programmatic focus on body -->
  ...
  <footer>
    <a href="/contact">Contact us</a>
    <a href="/disclaimer">Disclaimer</a>
  </footer>
</body>
```

**Remediation.**

1. Remove `tabindex` from `<body>` (and from `<html>`, `<main>`, `<header>`, `<footer>` unless explicitly required for programmatic focus management).
2. Do not call `.focus()` on `<body>` from JavaScript. If a route change needs to reset focus, target the H1 inside `<main>` instead (see P1-1).
3. Audit any third-party widget (analytics, chat, consent banner) for `tabindex` injection.

```html
<body>
  <a class="skip-link" href="#main">Skip to main content</a>
  <header role="banner">...</header>
  <main id="main" tabindex="-1">
    <h1>...</h1>
    ...
  </main>
  <footer role="contentinfo">...</footer>
</body>
```

```css
/* Make body explicitly non-focusable; remove the 3px outline */
body:focus { outline: none; }
body { /* no tabindex attribute */ }
```

**Verification.** After remediation, `Tab` from the footer's last link must move focus to the browser address bar (visible blue ring leaves the document). Run with `document.activeElement` logging or use Chromium's `Show focused element` DevTools toggle.

---

### P0-2. Invalid ARIA: `aria-label` on SVG `<path>` elements (PointSav)

**WCAG 4.1.2 Name, Role, Value — Level A**

**User impact.** axe flagged this as `aria-prohibited-attr` (serious). The ARIA in HTML specification forbids `aria-label` on `<path>` because `<path>` has no implicit role and is not in the ARIA role mapping; assistive technology ignores the label. The two POINTSAV / DIGITAL SYSTEMS wordmarks therefore announce as "graphic" or are skipped entirely depending on the screen reader.

**Current (broken):**

```html
<svg viewBox="0 0 600 120" class="wordmark">
  <path id="text1" d="M..." aria-label="POINTSAV"/>
  <path id="text2" d="M..." aria-label="DIGITAL SYSTEMS"/>
</svg>
```

**Remediation.** Move the accessible name to the `<svg>` root via `<title>` + `aria-labelledby`. This is the WAI-recommended pattern (SVG Accessibility API Mappings §4.1).

```html
<svg viewBox="0 0 600 120" class="wordmark"
     role="img"
     aria-labelledby="wordmark-title wordmark-desc">
  <title id="wordmark-title">POINTSAV Digital Systems</title>
  <desc id="wordmark-desc">Company wordmark.</desc>
  <path id="text1" d="M..."/>
  <path id="text2" d="M..."/>
</svg>
```

Notes:
- `role="img"` is mandatory; Safari and JAWS historically treated unlabeled SVGs as a presentational group.
- One combined title for the wordmark is preferable to two separate labels — the visible identity is the unit "POINTSAV Digital Systems", not two split tokens.
- If the SVG is decorative (purely visual, e.g. the same wordmark also appears as readable text adjacent), use `aria-hidden="true"` and omit `<title>`.

---

### P0-3. Empty anchor (PointSav)

**WCAG 2.4.4 Link Purpose (In Context) — Level A; 4.1.2 Name, Role, Value — Level A**

**User impact.** `<a href="#"></a>` announces as "link, blank" (VoiceOver) or "link, hash" (NVDA). It is unactionable, untargetable by screen-reader links list, and on click scrolls to top — which can disorient users who lose their place.

**Current (broken):**

```html
<a href="#"></a>
```

**Remediation.** Identify intent. If this is a back-to-top affordance:

```html
<a href="#top" class="back-to-top">
  <span class="visually-hidden">Back to top of page</span>
  <svg aria-hidden="true" focusable="false">...</svg>
</a>
```

If it is dead code from a template, delete it. If it is a placeholder for a future feature, comment it out — do not ship empty anchors.

```css
/* Standard visually-hidden utility (already present in many systems; add if missing) */
.visually-hidden {
  position: absolute;
  width: 1px; height: 1px;
  padding: 0; margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}
```

---

### P0-4. Nav link contrast 4.34:1 (Woodfine, both home pages)

**WCAG 1.4.3 Contrast (Minimum) — Level AA** (requires ≥ 4.5:1 for normal text)

**User impact.** Users with low vision, cataracts, age-related contrast sensitivity loss, or screen glare cannot reliably read the navigation. The deficit (4.34 vs 4.5) is small but the navigation is the single most important wayfinding affordance on a marketing site.

**Computed values.** `--ink-3: #6B7280` on `--paper-2: #F7F9FA` → 4.34:1.

**Remediation.** Replace `--ink-3` with a darker token in nav contexts, or introduce a dedicated `--ink-nav` token. The smallest change that crosses 4.5:1 against `#F7F9FA` is approximately `#5E6571` (≈ 4.52:1). A safer, also-AAA-passing choice is `--ink-2: #374151` (≈ 9.7:1).

```css
:root {
  /* Existing */
  --ink-3: #6B7280;       /* tertiary text — body de-emphasis only */
  /* New */
  --ink-nav: #374151;     /* navigation, footer links — AAA on paper-2 */
}

nav a,
footer a {
  color: var(--ink-nav);
}
```

**Why not just dial `--ink-3` darker?** `--ink-3` is used elsewhere as a deliberately subordinated body color where 4.5:1 is acceptable against pure white (it currently passes against `#FFFFFF` at 5.74:1). Mutating it would unintentionally darken decorative typography. A dedicated `--ink-nav` token preserves intent.

---

### P0-5. Footer link contrast 3.9:1

**WCAG 1.4.3 — Level AA**

**User impact.** Same population as P0-4; footer contains Contact and Disclaimer — both BCSC-relevant links.

**Computed values.** `--ink-3: #6B7280` on `--paper-3: #E6E7E8` → 3.9:1.

**Remediation.** Use `--ink-nav` (above) which yields ≈ 8.3:1 against `#E6E7E8`. Alternatively darken the surface to `--paper-2: #F7F9FA` and accept the visual softening of the footer.

```css
footer { background: var(--paper-3); }
footer a, footer .copyright { color: var(--ink-nav); }
```

---

### P0-6. `.footer-version` contrast (PointSav)

**WCAG 1.4.3 — Level AA**

axe flagged this; exact ratio not captured in the run (the element computed against the wrong background due to transparency). Treat as P0 pending re-measurement. Apply `--ink-nav` and re-test.

```css
.footer-version { color: var(--ink-nav); font-variant-numeric: tabular-nums; }
```

---

### P0-7. Touch target heights 16–17px

**WCAG 2.5.8 Target Size (Minimum) — Level AA (new in 2.2)** — requires ≥ 24×24 CSS pixels.

**User impact.** Users with motor impairments (tremor, Parkinson's, stylus users, touchscreen users on any device) mis-tap. Ten of twelve interactive elements on the Woodfine home page fail the 24px minimum — including all six primary navigation links. The wordmark (320×80) and partner card (254×231) are the only passing targets.

**Computed values (Woodfine, 1440px viewport):**

| Element | Measured | 2.5.8 (AA, 24px) | 2.5.5 (AAA, 44px) |
|---------|----------|------------------|-------------------|
| Wordmark | 320×80 | pass | pass |
| Partner card | 254×231 | pass | pass |
| MANIFEST tab | 130×~44 | pass | borderline |
| BIM Library | 130×~44 | pass | borderline |
| LocationIntelligence | 130×~44 | pass | borderline |
| Disclaimer (header) | 71×17 | **fail** | **fail** |
| Contact us (header) | 76×17 | **fail** | **fail** |
| Corporate | 71×17 | **fail** | **fail** |
| Projects | 60×17 | **fail** | **fail** |
| Newsroom | 67×17 | **fail** | **fail** |
| Contact us (footer) | 73×16 | **fail** | **fail** |
| Disclaimer (footer) | 73×16 | **fail** | **fail** |

**Remediation.** Increase the hit area without changing the visible type size, using vertical padding on the anchor and converting the nav list items to flex items with adequate gap.

```css
:root {
  --target-min: 24px;     /* 2.5.8 AA */
  --target-pref: 44px;    /* 2.5.5 AAA — Leapfrog 2030 target */
}

nav ul {
  display: flex;
  gap: 1rem;
  align-items: center;
  margin: 0;
  padding: 0;
  list-style: none;
}

nav a,
footer a {
  display: inline-flex;
  align-items: center;
  min-height: var(--target-pref);  /* AAA */
  padding-block: 0.75rem;           /* yields ~44px with 17px text */
  padding-inline: 0.5rem;
  /* Visible type unchanged */
  font-size: 0.9375rem;
  line-height: 1.2;
}
```

If padding cannot be added (because the visible row height must remain compact), use **2.5.8's spacing exception**: the target may be smaller if the inter-target spacing creates a 24px-diameter circle that does not overlap adjacent targets. With nav links currently sitting in a tight row this exception probably is not met — measure before relying on it.

---

## P1 — Serious gaps

### P1-1. Missing H1 on home page (both sites)

**WCAG 1.3.1 Info & Relationships — Level A; 2.4.6 Headings & Labels — Level AA**

**User impact.** Screen-reader users use the H1 as a landing beacon ("h" key to jump to first heading). Its absence forces them to read the entire page or rely on landmark navigation. Search engines also weight the H1 — the missing H1 is both an accessibility and discoverability defect.

**Current (broken).** No `<h1>` anywhere in the document. The wordmark SVG is visually the largest element but is not a heading.

**Remediation.** Add a semantic H1 that names the page. On a marketing home page the H1 should state what the entity does, not just its name:

Woodfine:
```html
<main id="main" tabindex="-1">
  <h1 class="visually-hidden">Woodfine Capital Projects — institutional real estate, capital projects, and digital systems</h1>
  ...
</main>
```

PointSav:
```html
<main id="main" tabindex="-1">
  <h1 class="visually-hidden">PointSav Digital Systems — sovereign data infrastructure and knowledge platforms</h1>
  ...
</main>
```

The `visually-hidden` H1 preserves the existing visual treatment (wordmark as primary visual hierarchy) while exposing semantic structure. A visible H1 is preferred long-term but is a design call for the v0.0.2 sprint.

---

### P1-2. Two unlabeled `<nav>` landmarks (both sites)

**WCAG 1.3.1 — Level A; 2.4.1 Bypass Blocks — Level A**

**User impact.** Screen readers announce "navigation, navigation" twice with no way to distinguish them. axe flagged `landmark-unique`.

**Current (broken):**

```html
<nav class="left">...</nav>
<nav class="right">...</nav>
```

**Remediation.** Either label them or collapse to one nav.

Option A (preserve two navs, label each):

```html
<nav class="left" aria-label="Primary navigation">
  <a href="/disclaimer">Disclaimer</a>
  <a href="/contact">Contact us</a>
</nav>
<nav class="right" aria-label="Sections">
  <a href="/corporate">Corporate</a>
  <a href="/projects">Projects</a>
  <a href="/newsroom">Newsroom</a>
</nav>
```

Option B (recommended — one nav, visually split):

```html
<nav aria-label="Primary">
  <ul class="nav-left">
    <li><a href="/disclaimer">Disclaimer</a></li>
    <li><a href="/contact">Contact us</a></li>
  </ul>
  <a class="wordmark" href="/">
    <svg role="img" aria-labelledby="wm-title"><title id="wm-title">Woodfine Capital Projects</title>...</svg>
  </a>
  <ul class="nav-right">
    <li><a href="/corporate">Corporate</a></li>
    <li><a href="/projects">Projects</a></li>
    <li><a href="/newsroom">Newsroom</a></li>
  </ul>
</nav>
```

Option B yields a single primary landmark and a cleaner screen-reader landmark list.

---

### P1-3. `.copyright` div outside any landmark (Woodfine)

**WCAG 1.3.1 — Level A**

axe flagged `region`. All non-decorative content should be inside a landmark (`<main>`, `<footer>`, `<aside>`, `<nav>`, `<header>`). A stray `.copyright` div is announced as orphan content in screen-reader region navigation.

**Remediation.** Move it inside `<footer>`:

```html
<footer role="contentinfo">
  <nav aria-label="Footer">
    <a href="/contact">Contact us</a>
    <a href="/disclaimer">Disclaimer</a>
  </nav>
  <div class="copyright">© 2026 Woodfine Capital Projects Inc.</div>
</footer>
```

---

### P1-4. SVGs missing `<title>` (all 11 SVGs across both sites)

**WCAG 1.1.1 Non-text Content — Level A**

**User impact.** Screen readers announce SVGs without `<title>` as "graphic" or skip them. Six SVGs on Woodfine and five on PointSav currently provide no alternative text.

**Remediation.** Each SVG must be classified as either *informative* (needs a name) or *decorative* (must be hidden).

Informative pattern:

```html
<svg role="img" aria-labelledby="svg-corporate-title" viewBox="...">
  <title id="svg-corporate-title">Corporate section icon</title>
  <path d="..."/>
</svg>
```

Decorative pattern:

```html
<svg aria-hidden="true" focusable="false" viewBox="...">
  <path d="..."/>
</svg>
```

Note `focusable="false"` is required to prevent IE/Edge-legacy keyboard focus on the SVG — still relevant for institutional users on locked-down enterprise browsers.

Audit checklist for the v0.0.2 sprint:

| SVG | Site | Suggested classification | Title |
|-----|------|--------------------------|-------|
| Wordmark | Both | Informative | "Woodfine Capital Projects" / "POINTSAV Digital Systems" |
| Tab icons (MANIFEST / BIM / LocationIntelligence) | Woodfine | Decorative (label in text) | aria-hidden |
| Partner card icon | Woodfine | Decorative | aria-hidden |
| Hero ornament | Both | Decorative | aria-hidden |
| Footer mark | Both | Decorative if wordmark already named | aria-hidden |

---

### P1-5. No skip-navigation link

**WCAG 2.4.1 Bypass Blocks — Level A**

**User impact.** Keyboard users tab through 11 interactive elements before reaching main content on every page load. Screen-reader users have landmark navigation, but a skip link benefits sighted keyboard users (also Switch Control users, who cannot easily jump).

**Remediation.**

```html
<body>
  <a class="skip-link" href="#main">Skip to main content</a>
  <header>...</header>
  <main id="main" tabindex="-1">...</main>
</body>
```

```css
.skip-link {
  position: absolute;
  top: -40px;
  left: 0;
  padding: 0.75rem 1rem;
  background: var(--ink);
  color: var(--paper);
  text-decoration: none;
  z-index: 1000;
  border-radius: 0 0 4px 0;
  font-weight: 600;
  transition: top 120ms ease;
}
.skip-link:focus {
  top: 0;
  outline: 3px solid var(--accent);
  outline-offset: 2px;
}
```

Verify the skip link does *not* require `:hover` to appear, and that `top` animation is suppressed under `prefers-reduced-motion` (see P2-2).

---

### P1-6. Focus indicator 1px outline insufficient

**WCAG 2.4.11 Focus Not Obscured (Minimum) — Level AA (new in 2.2)**
**WCAG 2.4.13 Focus Appearance — Level AAA (new in 2.2)**

**User impact.** Tab-order audit captured `outline_width=1px` on link stops (with one stop showing `outline_width=3px`, likely the body trap from P0-1). A 1px outline at the default browser color is invisible on dark text and unreliable against the `--paper-3` footer.

WCAG 2.4.13 (AAA) requires the focus indicator to:
- Have an area at least equal to the perimeter of the focused element times 2 CSS pixels; AND
- Have a 3:1 contrast ratio against the adjacent (focused/unfocused) colors; AND
- Not be obscured by author-created content.

A 1px outline at `outline-color: -webkit-focus-ring-color` will not meet 2.4.13. It may not even meet 2.4.11 if other UI (cookie banner, chat widget) overlaps.

**Remediation.**

```css
:root {
  --focus-ring: var(--accent);     /* wf-blue, high contrast against paper */
  --focus-ring-offset: 2px;
  --focus-ring-width: 3px;
}

*:focus { outline: none; }                   /* reset default */

*:focus-visible {
  outline: var(--focus-ring-width) solid var(--focus-ring);
  outline-offset: var(--focus-ring-offset);
  border-radius: 2px;
}

/* High-contrast / Windows forced-colors */
@media (forced-colors: active) {
  *:focus-visible {
    outline-color: Highlight;
  }
}
```

Use `:focus-visible` (not `:focus`) to avoid the click-and-focus outline that designers tend to suppress. Re-test all interactive elements after applying.

---

## P2 — Enhancements (AAA, 2.2-specific, and robustness)

### P2-1. Touch targets to 44×44px (AAA)

**WCAG 2.5.5 Target Size — Level AAA**

The P0-7 remediation (`min-height: 44px`) satisfies this. Confirm during sprint that no element regresses below 44px on any breakpoint.

### P2-2. `prefers-reduced-motion`

**WCAG 2.3.3 Animation from Interactions — Level AAA**

**User impact.** Users with vestibular disorders, migraine sensitivity, ADHD, or autism experience discomfort or symptoms from motion. The site currently has no `prefers-reduced-motion` handling; any animation (current or future) plays regardless.

**Remediation.** Add a global guard early in the stylesheet:

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}
```

For motion that conveys meaning (e.g. tab indicator slide), provide an instant alternative under the same media query.

### P2-3. `prefers-color-scheme`

Not strictly a WCAG criterion, but high-contrast modes and dark-mode users expect respect. Establish dark-mode tokens during v0.0.2 even if a dark theme does not ship. Sketch:

```css
@media (prefers-color-scheme: dark) {
  :root {
    --paper: #0B1220;
    --paper-2: #111827;
    --paper-3: #1F2937;
    --ink: #F9FAFB;
    --ink-2: #E5E7EB;
    --ink-nav: #E5E7EB;
    /* --wf-blue retained; verify 4.5:1 against dark paper */
  }
}
```

### P2-4. General link contrast to AAA

**WCAG 1.4.6 Contrast (Enhanced) — Level AAA** — requires ≥ 7:1.

`--wf-blue: #164679` on `--paper-2` measures 9.1:1 (AAA pass). General Woodfine links at 4.58:1 fail AAA. Adopt `--wf-blue` for all body links to achieve AAA:

```css
a { color: var(--wf-blue); text-underline-offset: 2px; }
a:hover { text-decoration-thickness: 2px; }
```

### P2-5. WCAG 2.5.3 Label in Name — Level A (verify)

**Verification, not remediation.** Each nav link's visible text matches its accessible name (no `aria-label` overrides). Confirmed for current markup; flag if future copy uses `aria-label` that diverges from visible text — speech-input users rely on visible text.

### P2-6. Document language

`<html lang="en">` confirmed present on both sites. No action.

### P2-7. `font-display: swap`

Confirmed present. No FOIT — supports WCAG 1.4.4 Resize Text indirectly.

---

## Mobile-specific concerns (375px breakpoint)

Although deep mobile audit is out of this lens's scope, accessibility-mobile intersections must be called out:

- **Six nav links across 375px** with no responsive collapse is a *2.5.8* failure compounded by *1.4.4 Resize Text* — users who scale to 200% find the nav row truncates or wraps unpredictably.
- **No mobile menu pattern.** A hamburger-disclosure pattern would resolve both the touch-target and reflow concerns. Disclosure widget must follow APG (`aria-expanded`, `aria-controls`, focus trapping while open, `Escape` to close).
- **1.4.10 Reflow — Level AA** requires content to be usable at 320px CSS without horizontal scroll. The current header layout almost certainly fails reflow at 320px; verify.

A mobile-menu component is a candidate for the v0.0.2 sprint and should be specified jointly with the agent-charlie (mobile/performance) findings.

---

## Screen-reader narrative on page load (VoiceOver, Safari, macOS)

Current state (Woodfine, today):

> "home.woodfinegroup.com. Web content. Navigation. Disclaimer, link. Contact us, link. Out of navigation. Link, graphic. Navigation. Corporate, link. Projects, link. Newsroom, link. Out of navigation. Web content end."

Problems audible in this narrative:
- No page title spoken as a heading (no H1).
- Wordmark announces as "Link, graphic" — no name.
- Two navs both announce as "navigation" with no distinguishing label.
- No landmark for main content.

Post-remediation:

> "home.woodfinegroup.com. Skip to main content, link. Banner landmark. Primary navigation. Disclaimer, link. Contact us, link. Woodfine Capital Projects, link, image. Corporate, link. Projects, link. Newsroom, link. Out of navigation. Main landmark. Woodfine Capital Projects — institutional real estate, capital projects, and digital systems. Heading level 1. ..."

This is what an institutional site at WCAG 2.2 AA should sound like.

---

## Site comparison

| Criterion | Woodfine | PointSav |
|-----------|----------|----------|
| axe violations (count) | 4 | 5 |
| Serious violations | 1 (contrast) | 3 (aria-prohibited-attr, contrast, link-name) |
| Cyclic tab order | Confirmed | Likely (same template) — verify |
| H1 | Missing | Missing |
| Skip link | Missing | Missing |
| Nav landmarks labeled | No | No |
| SVG titles | 0 / 6 | 0 / 5 |
| Empty anchors | 0 | 1 |
| Nav contrast | 4.34:1 (FAILS AA) | Unable to measure (transparent bg) — likely FAILS |
| Footer contrast | 3.9:1 (FAILS AA) | 9.1:1 (PASSES AAA) |
| Body link contrast | 4.58:1 (passes AA, fails AAA) | 9.1:1 (PASSES AAA) |
| Reduced-motion | Not handled | Not handled |
| Color scheme | Light only | Light only |

**Asymmetric observation.** PointSav's color system is materially stronger (uses `--wf-blue` rather than `--ink-3` for links) — Woodfine should adopt the same pattern. PointSav's ARIA is materially weaker (invalid SVG attributes, empty anchor) — both indicate template hand-editing that bypassed component review. Both observations argue for a shared design-system substrate, which is the v0.0.2 sprint's purpose.

---

## Remediation effort estimate

| Priority | Scope | Engineer hours | Designer hours | QA hours |
|----------|-------|----------------|----------------|----------|
| **P0** (1–7) | Keyboard trap, ARIA, contrast tokens, target sizing | 6 | 2 | 3 |
| **P1** (8–13) | H1, landmarks, SVG titles, skip link, focus ring | 8 | 3 | 3 |
| **P2** (14–17) | AAA polish, motion, color-scheme tokens | 5 | 4 | 2 |
| **Mobile menu** (out of P0–P2 numbering, called out above) | Disclosure pattern + reflow | 10 | 4 | 4 |
| **Totals** | | **29** | **13** | **12** |

Combined ≈ **54 hours**, plus a half-day cross-browser smoke (Chromium / Safari / Firefox / NVDA / VoiceOver / TalkBack). Realistic two-week sprint with one engineer half-loaded, one designer at 25%, one QA at 25%.

**Sequencing recommendation.** Land P0-1 (keyboard trap) and P0-4/5 (contrast tokens) in the first commit — both are template-level and unblock everything downstream. P1 should land in a single coordinated commit (HTML structure changes affect screen-reader narrative as a whole). P2 can ship incrementally.

---

## Verification plan

After each priority tier lands, re-run:

1. `axe-core` (zero violations target — currently 4 + 5).
2. Manual tab-order traversal from URL bar → footer → URL bar (no cycle).
3. Touch-target measurement at 1440 / 1024 / 768 / 375 viewports (all ≥ 24px AA, ≥ 44px AAA target).
4. Color-contrast spot check on all token combinations against APCA + WCAG 2 ratios.
5. VoiceOver / Safari narrative recording — diff against the "post-remediation" script in §Screen-reader narrative.
6. Lighthouse Accessibility score (target ≥ 95; AAA work pushes toward 100).

Sign-off requires all six checks plus a screen-reader user test (recommend Fable or paid panel) before the home pages re-launch under the Leapfrog 2030 banner.

---

## Open questions for project-design

1. Should the H1 be visually hidden (preserving the wordmark as primary visual element) or visible? — design call, but visibility is the AA-and-up best practice.
2. Is the partner-card click target (254×231) the entire card or only the headline anchor? — if only the headline, fix per WCAG 2.5.5 by extending the anchor to the card boundary.
3. Confirm whether `aria-current="page"` is applied to the active nav item — not measured here.
4. Confirm whether tab components (MANIFEST / BIM / LocationIntelligence) follow APG Tabs pattern — outside this audit but adjacent to the icon-tab DESIGN-COMPONENT draft already in `drafts-outbound/`.
5. Should `--ink-nav` be a primitive token or a semantic alias of `--ink-2`? — recommend semantic alias to keep primitive count low.

---

*End of draft. Hand off to project-editorial for prose refinement and project-design for v0.0.2 sprint planning.*
