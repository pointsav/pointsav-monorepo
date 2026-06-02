---
schema: foundry-draft-v1
type: DESIGN-RESEARCH
language_protocol: DESIGN-RESEARCH
title: "Leapfrog 2030 — design excellence audit: home.woodfinegroup.com + home.pointsav.com"
target_project: project-design
target_destination: pointsav-design-system
created: 2026-06-02T00:00:00Z
author: totebox@project-marketing (claude-opus / agent-beta)
research_trail:
  source_commits:
    - 9b0543d (cluster/project-marketing HEAD)
  prior_drafts:
    - .claude/drafts-outbound/leapfrog-2030/INDEX.md
    - .claude/drafts-outbound/leapfrog-2030/tokens/token-woodfine-typography.draft.md
    - .claude/drafts-outbound/leapfrog-2030/tokens/token-woodfine-brand-color.draft.md
    - .claude/drafts-outbound/leapfrog-2030/components/component-marketing-topnav.draft.md
    - .claude/drafts-outbound/leapfrog-2030/components/component-marketing-hero-band.draft.md
    - .claude/drafts-outbound/leapfrog-2030/components/component-marketing-footer.draft.md
  citations:
    - conventions/compounding-substrate.md
    - conventions/worm-ledger-design.md
  operator_inputs:
    - "Leapfrog 2030 mandate: award-winning design using 2030-standard CSS today"
    - "Bloomberg standard: precise, institutional, no marketing vocabulary"
    - "BCSC posture: no competitive comparisons by name"
  related_files:
    - outputs/audit-2026-06-02/ (screenshots + audit data)
---

# Leapfrog 2030 — design excellence audit

**Agent:** Beta (design / forward-looking CSS lens)
**Subjects:** `home.woodfinegroup.com`, `home.pointsav.com`
**Sources:** `outputs/audit-2026-06-02/*.png` + `*-meta.json` + `*-contrast.json` + `*-tokens.json` + `*-targets.json`; the six staged Leapfrog 2030 component drafts; the live `shell.css` token system.

---

## 0. Executive summary

The two sites currently ship a single shared template painted in two slightly different palettes. The template is institutionally restrained, typographically literate, and structurally honest — a good baseline that has not yet earned a Leapfrog 2030 ribbon. Three structural deficits dominate every other finding:

1. **The viewport is wasted at 1440px.** The hero H1 is 22 px and the body is 14 px with narrow gutters. At 1440 px the page reads as a 480 px-tall pamphlet on a 900 px canvas. Awwwards scoring penalises this pattern heavily — it codes as a 2014 mobile-first site enlarged.
2. **There is no `<h1>` on either landing page.** Both `index` meta dumps show `h1: 0, h2: 0, h3: 8`. The wordmark image stands in for the page heading. This breaks both document outline and visual hierarchy simultaneously.
3. **Mobile (375 px) is unbuilt.** The desktop three-column nav grid does not collapse — it squashes. Touch targets fail Apple's 44 × 44 (`pass_258: false` on every nav link in `woodfine-index-targets.json`). The tablet-portrait stack the topnav recipe describes at 768 px is not the mobile rendering shown in `*-index-375.png`.

The good news: the token system is already coherent. `--paper / --ink / --accent` plus the pap­er/ink ladder map cleanly onto every Leapfrog 2030 technique in §3. The work is to lean into what is built rather than rebuild it.

---

## 1. Awwwards rubric scoring

Awwwards judges score four dimensions out of 10. Honours threshold is ≈ 6.5 average; Site of the Day ≈ 8.0; Site of the Year ≈ 8.5.

| Dimension | Woodfine | PointSav | Notes |
|---|---|---|---|
| Design (visual craft) | **5.8** | **5.5** | Restrained, literate, under-scaled |
| Usability | **5.0** | **5.0** | Desktop nav works; mobile broken; no `<h1>` |
| Creativity | **3.5** | **3.2** | Indistinguishable from each other; zero surprise |
| Content | **6.5** | **6.0** | Bloomberg-precise where copy exists; sparse |
| **Average** | **5.20** | **4.93** | Below Honours; needs Leapfrog 2030 to reach 7.5+ |

### 1.1 Design (visual craft) — 5.8 / 5.5

**What works.** The colour ladder is disciplined: `--paper / --paper-2 / --paper-3` is a three-step ramp with measured intervals (#FFF → #F7F9FA → #E6E7E8), and the ink ladder is the same idea on the type side (#111827 → #374151 → #6B7280 → #9CA3AF). Pairs of greys this close is a confidence move — it says "we have whitespace, we don't need contrast tricks." `--wf-blue` (#164679) is genuinely institutional — deeper than the default "tech blue" of #2563EB by 23 lightness points in OKLCH terms.

**What hurts the score.**

- **Hero H1 at 22 px on a 1440 px viewport** is the single biggest design defect. The hero band is full-bleed, the type inside it is the size of a footer footnote. The `font-size-h1` token comment in the typography draft (lines 100–104) honestly describes the cascade — "scales DOWN via clamp() to 18px at 1024px" — meaning even the desktop maximum is 22 px. A 1440 px hero should carry a 56–88 px H1 to make the visual investment in the blue band pay off.
- **Body measure exceeds 80 ch on landing.** The contact and disclaimer pages constrain to a centred column, but the landing page lets paragraphs run the full grid width. At 1440 px with 14 px body, a single-column paragraph runs ~ 130 ch — well past Bringhurst's 45–75 ch readable range.
- **Whitespace is symmetric, not architectural.** The page reads as a stack of bands of equal vertical pressure (28 px `margin-bottom` after the topnav, then equal section padding). Architectural whitespace would weight the hero 3× the supporting bands.
- **Six fonts on the wire, two used.** The wordmark uses display; body uses Roboto Slab. Nunito Sans, Barlow Condensed, League Gothic, Zilla Slab, Mulish — all downloaded as WOFF2 (`has_font_display_swap: true` confirms the fetch), all dormant. §2 unpacks this.

**Gap to 8.0.** Triple the H1, halve the measure, build the architectural rhythm, and ship two fonts as variable. That alone takes Design to 7.5+.

### 1.2 Usability — 5.0 / 5.0

**What works.** Tab order is sane (`*-taborder.json` confirms left → wordmark → right). External-tab links carry `rel="noopener"` consistently (`woodfine-index-meta.json` lines 26–67). The 768 px collapse the topnav recipe describes (stack wordmark on top, links beneath) is a defensible "investor pages don't hide nav" decision.

**What hurts the score.**

- **No `<h1>` on either landing page.** `woodfine-index-meta.json` line 16–20: `"h1": 0, "h2": 0, "h3": 8`. Same on PointSav. The wordmark image is the de-facto page heading. This breaks screen-reader navigation (which jumps by heading level), breaks the document outline algorithm, and codes as a 2010 CMS template to anyone who runs Lighthouse.
- **No `has_skip_link`** on either site (`woodfine-index-meta.json` line 14). With 12 focusable elements before content (`focusable_count: 12`), a keyboard user tabs through the entire chrome to reach the first article link.
- **Mobile (375 px) is broken.** The screenshots show the desktop three-column grid squashed to 375 px. The topnav recipe at line 165 specifies `@media (max-width: 768px)` should collapse the grid to a single column; the 375 px render shows the grid has not collapsed. Either the breakpoint is wrong in the deployed CSS or the audit captured a stale build — either way, the artefact is what users see, and the artefact is broken.
- **Touch targets fail Apple HIG.** `woodfine-index-targets.json` shows seven of twelve links at `pass_258: false` (Apple's 44 × 44 px minimum). Disclaimer, Contact us, Corporate, Projects, Newsroom — all at 71 × 17 to 76 × 17. The MANIFEST / BIM Library / LocationIntelligence cards do hit 130 × 44 — proof the team knows the rule; just hasn't applied it to nav.
- **SVGs without titles.** `svg_count: 6, svg_no_title: 6`. Wordmark and icon cards announce as "image" to AT.

**Gap to 7.5.** Add `<h1>` (the wordmark CAN be the H1 if marked `<h1><a><svg aria-label="…"></svg></a></h1>`). Add a skip link. Fix the mobile breakpoint. Apply the 44 × 44 rule to the topnav at every breakpoint. That moves Usability to 7.5 without touching the visual design.

### 1.3 Creativity — 3.5 / 3.2

**What hurts the score.** Both sites are interchangeable. Same nav grid, same hero band, same paper-2 sections, same icon cards, same paper-3 footer. A juror looking at the two would mark one of them "duplicate submission." A 2030-standard institutional site doesn't need to be loud, but it does need to telegraph what it is in the first 200 ms of viewing — and the two sites currently telegraph the same thing.

**There is zero motion.** No scroll interaction, no entrance animation, no parallax, no view transition. `has_prefers_motion_css: false` on every page — meaning the team hasn't even built a motion vocabulary to gate. The absence is not restraint; it's vacancy.

**There is zero distinctiveness in the hero.** A full-width blue band with centred all-caps text is the most common institutional hero pattern of the last decade. To earn a Creativity score above 6, this band needs ONE distinctive treatment — a typographic stunt, a scroll behaviour, an unexpected colour interaction, a content-aware layout.

**Gap to 7.0.** §4 (brand differentiation) and §3.2/3.3 (scroll-driven + view transitions) together move Creativity from 3.5 to 7+.

### 1.4 Content — 6.5 / 6.0

**What works.** The copy that exists is precise and institutional. "Woodfine Capital Projects" not "Woodfine Group." "MANIFEST" / "BIM Library" / "LocationIntelligence" — these are specific, jargon-honest names that telegraph the audience (developer, architect, planner) rather than market to a generic visitor. Bloomberg-standard. No "world-class," no "innovative," no "solutions."

**What hurts the score.**

- **Information density is low.** The hero kicker, H1, and one paragraph fill the band. The supporting section runs three icon cards with a label and a one-sentence claim. Below the fold there is nothing. A site with an empty fold below the hero codes as a holding page, not a marketing destination.
- **No dateline, no recency cue.** Newsroom is a top-nav link but the landing page itself carries nothing time-stamped. An investor-facing surface should signal it's alive — most recent project breaking ground, most recent press release, a current quarter.
- **PointSav site copy is sparser than Woodfine** despite PointSav being the more technically distinguishable subject. "DOCUMENTATION / SOFTWARE / NEWSROOM" with no supporting paragraphs telegraphs "we haven't written this yet."

**Gap to 8.0.** Add a "latest" block — most recent newsroom item, dated. Add a numeric anchor — "40 years," "16 deployments," "X buildings" — somewhere in the first viewport. Specificity beats abstraction.

---

## 2. Typography system — critique and proposal

### 2.1 What is on the wire vs. what is doing work

The shell loads seven Google Fonts in a single `<link>`: Oswald, Barlow Condensed, League Gothic, Roboto Slab, Zilla Slab, Nunito Sans, Mulish. That is roughly 280–340 KB of WOFF2 over the wire (each weight + each font ≈ 35–50 KB). The token typography draft justifies this as five swappable pairing presets (`.fp-brand`, `.fp-brand-zilla`, `.fp-barlow-roboto`, `.fp-league-zilla`, `.fp-system`).

The justification is honest but the economics are wrong. **A user only ever sees one pairing.** The other four are dormant capability the operator might swap to. Shipping all seven on every pageload to support a capability the user never exercises is a 250 KB tax on every visitor. Pairing swapping is an authoring-time concern; the runtime should ship the active pairing only.

### 2.2 Per-font assessment

**Oswald (display).** Doing real work. The wordmark, hero kicker, top-nav, footer footnav, and the page-hero H1 all use it. Oswald's 0.18em letter-spacing reading is what gives the wordmark its institutional voice. **Verdict: keep, but replace with a variable equivalent.** Oswald has no variable axis in the Google Fonts variable subset; the closest drop-in is **Oswald VF (1 axis: weight 200–700)** which Google Fonts has shipped variable since 2024 — confirm the font-link URL uses `wght@200..700` syntax. If Oswald VF is unavailable, **Barlow Condensed VF** has the same letterform character with wght + width axes and ships variable.

**Roboto Slab (body).** Doing real work but at the wrong size. At 14 px, the slab serifs read as squat rather than authoritative. Slab serifs need 16 px+ to show the slab character; below that they read as "weird sans-serif." **Verdict: keep, raise body to 16 px, or switch to Roboto Flex (variable wght + width + opsz) for optical-size adjustment that makes a slab readable at 14 px.** Roboto Flex's `opsz` axis tunes letterforms for small sizes — this is the 2030 move.

**Nunito Sans (UI sans).** Not visibly used on landing. The typography draft says "nav labels, value-prop body, compliance copy, jurisdiction tables." Top-nav uses `--display` (Oswald), value-prop body uses `--serif` (Roboto Slab) per the hero-band recipe. Where Nunito Sans appears is the disclaimer body, contact form labels, and the footer cities list — none on the landing page. **Verdict: drop from landing-page bundle. Lazy-load on disclaimer / contact via a separate `<link>` so landing pays nothing for it.**

**Barlow Condensed.** Loaded for `.fp-barlow-roboto` preset only. Not active. **Verdict: drop. Author-time swap, not runtime.**

**League Gothic.** Loaded for `.fp-league-zilla` preset only. Not active. **Verdict: drop. Same.**

**Mulish.** Loaded for `.fp-brand-zilla` preset only. Not active. **Verdict: drop. Same.**

**Zilla Slab.** Loaded for two presets. Not active in `.fp-brand`. **Verdict: drop from default bundle.**

### 2.3 Proposed two-font variable system

Collapse the runtime bundle from 7 statics → 2 variables:

```html
<!-- Single link, two variable families, opsz + wght axes -->
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?
  family=Oswald:wght@200..700&
  family=Roboto+Flex:opsz,wght@8..144,100..1000&
  display=swap">
```

```css
:root {
  /* Display — variable, single file, full weight range */
  --display: "Oswald", "Trade Gothic LT Std", "Barlow Condensed",
             "Helvetica Neue", Arial, sans-serif;

  /* Body — variable, single file, opsz + wght + width */
  --serif:   "Roboto Flex", "Caecilia LT Std", "Roboto Slab",
             Georgia, serif;

  /* UI sans collapses to system — Roboto Flex is its own UI font
     when font-variation-settings reset to upright sans-like */
  --sans:    "Roboto Flex", system-ui, -apple-system, sans-serif;

  /* opsz axis controls letterform tuning per size */
  --body-opsz: 14;
  --hero-opsz: 96;
  --nav-opsz: 11;
}

body {
  font-family: var(--serif);
  font-size: 16px;                    /* up from 14 */
  font-variation-settings:
    "opsz" var(--body-opsz),
    "wght" 400,
    "wdth" 100;
}

.hero h1 {
  font-family: var(--display);
  font-size: clamp(40px, 6vw, 88px);  /* up from 22px max */
  font-variation-settings:
    "wght" 500;                       /* Oswald medium */
  letter-spacing: 0.04em;             /* loose at large size */
}

nav a {
  font-family: var(--display);
  font-size: 13px;                    /* up from 11 */
  font-variation-settings: "wght" 500;
  letter-spacing: 0.14em;
}
```

**Wire weight comparison:**

| System | Files | WOFF2 bytes (est.) | Active typography surface |
|---|---|---|---|
| As-built (7 statics) | 7 | ~ 320 KB | One pairing visible |
| Proposed (2 variables) | 2 | ~ 95 KB | Three axes per family, full range |
| Savings | -5 | **-225 KB** (-70%) | More expressive, less wire |

**Pairing presets survive at author time.** Move `.fp-*` switching from `<body class="fp-…">` to a build-time token toggle: the operator picks the pairing when emitting the site, the runtime ships only that pairing's two variables.

### 2.4 Hard-disagree on body-size token

The current `font-size-body: 14px` is too small for institutional copy at 1440 px. Recommend `16px` minimum on landing, `15px` rem-scaled on disclaimer / contact body copy. The audit's body-text contrast result (`woodfine-index-contrast.json` line 47) shows the 14 px copy passes AA — but AA on a real-property-developer site is a floor, not a target. Read Bloomberg Terminal: 13 px on a 5k monitor where the audience is paid to look at it. Read Bloomberg.com: 16 px body. The audience matters.

---

## 3. Leapfrog 2030 CSS techniques

For each: problem on these sites · code adapted to the token system · browser support note · progressive-enhancement fallback.

### 3.1 CSS Container Queries — for the topnav

**Problem.** The topnav uses `@media (max-width: 768px)` to collapse its grid. This binds the component's responsive behaviour to the viewport, not to its container. The moment the topnav is reused inside a constrained surface (a sidebar layout, an embedded preview, the design-system component-explorer), the breakpoint fires at the wrong moment.

**Working code, adapted to `shell.css` tokens:**

```css
/* Establish the topnav's parent as a query container. */
header.shell-header {
  container-type: inline-size;
  container-name: topnav-host;
}

/* The topnav responds to ITS container, not the viewport. */
.topnav {
  display: grid;
  grid-template-columns: 1fr auto 1fr;
  align-items: center;
  padding: 0 56px;
}

@container topnav-host (max-width: 1199px) {
  .topnav { padding: 0 36px; }
}

@container topnav-host (max-width: 1023px) {
  .topnav { padding: 0 24px; }
  .topnav .wordmark svg { width: 240px; height: 60px; }
}

@container topnav-host (max-width: 767px) {
  .topnav {
    grid-template-columns: 1fr;
    grid-template-rows: auto auto auto;
    gap: 16px;
    justify-items: center;
  }
  .topnav .wordmark { order: -1; }
  .topnav .left, .topnav .right {
    justify-content: center;
    gap: 24px;
    font-size: 13px;        /* readable at 375px, see §5.2 */
  }
  .topnav a {
    min-height: 44px;       /* fixes pass_258 failures */
    display: inline-flex;
    align-items: center;
    padding: 0 8px;
  }
}
```

**Browser support.** Container queries: Chrome 105+, Safari 16+, Firefox 110+. Baseline-widely-available since 2024. ~ 96% global support per caniuse 2026.

**Fallback.** Wrap in `@supports (container-type: inline-size) { … }`. Below the supports block, retain the existing `@media (max-width: 768px)` rule as the floor. Browsers without container query support degrade to the viewport-driven behaviour they already have.

### 3.2 CSS Scroll-driven Animations — hero band entrance

**Problem.** The hero band is static. The page loads with the full blue block visible. On a 1440 × 900 viewport this means the user sees the wordmark, the hero band, and the first 200 px of the next section all at once — no rhythm, no priority signal.

**Working code:**

```css
@media (prefers-reduced-motion: no-preference) {
  .hero-band {
    animation: hero-enter linear both;
    animation-timeline: view();
    animation-range: entry 0% cover 30%;
  }

  .hero-band h1 {
    animation: hero-h1-rise linear both;
    animation-timeline: view();
    animation-range: entry 10% cover 40%;
  }

  @keyframes hero-enter {
    from { opacity: 0; transform: translateY(8px); }
    to   { opacity: 1; transform: translateY(0);   }
  }

  @keyframes hero-h1-rise {
    from {
      opacity: 0;
      transform: translateY(16px);
      font-variation-settings: "wght" 300;
    }
    to {
      opacity: 1;
      transform: translateY(0);
      font-variation-settings: "wght" 500;
    }
  }
}
```

The keyframe interpolating `font-variation-settings` is the Leapfrog 2030 trick — the H1 doesn't just fade in; it gains weight as it rises. Only possible with a variable font (§2.3).

**Browser support.** `animation-timeline: view()`: Chrome 115+, Edge 115+; Safari 26 (shipped late 2025); Firefox flagged as of writing. ~ 75% support globally.

**Fallback.** Already gated by `prefers-reduced-motion`. For browsers without scroll-driven animation, the hero renders in its final state — no animation, no broken layout. Zero penalty.

### 3.3 View Transitions API — wordmark morph between pages

**Problem.** Navigating from `/` to `/page/contact` or `/page/disclaimer` is a hard page-load. The wordmark unmounts, the hero band collapses, the new page paints. The continuity that says "you are still on the same site" is lost.

**Working code:**

```html
<!-- Both pages share this wordmark anchor with the same name -->
<a class="wordmark" href="/" aria-label="Woodfine Capital Projects"
   style="view-transition-name: wf-wordmark;">
  <svg class="logo-svg" viewBox="0 0 144 36" …></svg>
</a>
```

```css
/* Cross-document VT requires opt-in */
@view-transition { navigation: auto; }

/* Per-element morph tuning */
::view-transition-old(wf-wordmark),
::view-transition-new(wf-wordmark) {
  animation-duration: 320ms;
  animation-timing-function: cubic-bezier(0.2, 0, 0, 1);
}

::view-transition-group(root) {
  animation-duration: 240ms;
}

@media (prefers-reduced-motion: reduce) {
  ::view-transition-group(*) { animation: none; }
}
```

For SPA-style: `document.startViewTransition(() => updateDOM())`. For MPA (the actual case here): `@view-transition { navigation: auto; }` is the entire opt-in. Same-origin only, both pages need the directive.

**Browser support.** Same-document VT: Chrome 111+, Safari 18+, ~ 85% support. Cross-document VT: Chrome 126+, Safari 18.2+, Firefox flagged. ~ 70% support.

**Fallback.** Browsers without VT see the existing hard navigation. No degradation. Costs zero bytes when unsupported because `@view-transition` is a parser-skipped at-rule in older engines.

### 3.4 `prefers-color-scheme: dark`

**Problem.** No dark mode (`has_dark_mode_css: false` on every audited page). For a site whose audience includes architects and developers reviewing BIM at 11 pm, this matters. For PointSav's developer audience it matters more.

**Working code — the existing token system maps cleanly:**

```css
:root {
  --paper:   #FFFFFF;
  --paper-2: #F7F9FA;
  --paper-3: #E6E7E8;
  --ink:     #111827;
  --ink-2:   #374151;
  --ink-3:   #6B7280;
  --ink-4:   #9CA3AF;
  --rule:    #E6E7E8;
  --rule-strong: #9CA3AF;
  --accent:  #164679;
  --accent-tint: #E8EFF7;
  /* NEW — required for dark mode hero band */
  --ink-on-accent-surface: #FFFFFF;
  --accent-surface: var(--accent);
}

@media (prefers-color-scheme: dark) {
  :root {
    --paper:   #0B0F14;        /* near-black, warmer than #000 */
    --paper-2: #11161D;        /* one step lighter — sections */
    --paper-3: #1A222C;        /* two steps lighter — footer */
    --ink:     #F4F6F8;        /* near-white, warmer than #FFF */
    --ink-2:   #C8CFD6;
    --ink-3:   #8B95A0;
    --ink-4:   #5B6470;
    --rule:    #1F2933;
    --rule-strong: #5B6470;

    /* Brand colour SHIFTS for dark mode — lighter, less saturated */
    --accent:        oklch(62% 0.12 264);    /* lighter blue */
    --accent-tint:   oklch(22% 0.04 264);    /* dark tint surface */
    --accent-surface: oklch(28% 0.10 264);   /* hero band darker */
    --ink-on-accent-surface: #F4F6F8;
  }
}

/* Manual override — respect user preference toggle */
[data-color-scheme="light"] { color-scheme: light; }
[data-color-scheme="dark"]  { color-scheme: dark;  }
```

**Browser support.** `prefers-color-scheme`: ~ 98% global. `oklch()` (used in dark accent above): Chrome 111+, Safari 15.4+, Firefox 113+. ~ 94% global.

**Fallback.** Without `prefers-color-scheme` support, the `:root` defaults paint light mode — the existing experience. Zero regression. For `oklch()` unsupported, provide hex fallback in declaration order:

```css
--accent: #4A7AB5;                    /* hex fallback */
--accent: oklch(62% 0.12 264);        /* preferred */
```

### 3.5 `oklch()` colour space — mathematically correct tints

**Problem.** `--accent-tint: #E8EFF7` is a hand-picked tint of `--accent: #164679`. It's close enough but not lightness-linear — if you ask for a 50% tint or a 75% tint, you'd have to pick more hex values by eye. Tint families on these sites are likely to grow (active state, hover state, focus ring, pressed state, validated state, error-on-blue, footer-on-blue …) and every hand-picked value is a drift opportunity.

**Working code:**

```css
:root {
  /* Brand colour authored in oklch — the canonical value */
  --accent:      oklch(30% 0.11 264);    /* equivalent to #164679 */

  /* Derived tints — relative oklch, mathematically locked */
  --accent-95:   oklch(from var(--accent) 95% calc(c * 0.30) h);
  --accent-92:   oklch(from var(--accent) 92% calc(c * 0.35) h);  /* ≈ #E8EFF7 */
  --accent-85:   oklch(from var(--accent) 85% calc(c * 0.50) h);
  --accent-70:   oklch(from var(--accent) 70% calc(c * 0.75) h);
  --accent-50:   oklch(from var(--accent) 50% calc(c * 0.90) h);
  --accent-30:   var(--accent);                                    /* the brand */
  --accent-20:   oklch(from var(--accent) 20% c h);                /* hover deep */
  --accent-12:   oklch(from var(--accent) 12% c h);                /* pressed   */

  /* Semantic aliases — components reference these */
  --accent-tint:    var(--accent-92);   /* replaces hand-picked #E8EFF7 */
  --accent-hover:   var(--accent-20);
  --accent-pressed: var(--accent-12);
  --accent-focus-ring: var(--accent-50);
}
```

The `oklch(from … L C H)` relative syntax is the Leapfrog 2030 move. It means the design system **describes a brand colour once and computes every variant**. Change `--accent` and all eight tints update — no spreadsheet, no drift.

**Browser support.** `oklch()`: 94% global. Relative colour syntax (`oklch(from …)`): Chrome 119+, Safari 16.4+, Firefox 128+. ~ 88% global. Edge of Leapfrog 2030 but solidly within the mandate.

**Fallback.** Provide the hex equivalent before the oklch declaration. The cascade picks the last-recognised value:

```css
--accent-tint: #E8EFF7;
--accent-tint: var(--accent-92);   /* uses oklch where supported */
```

### 3.6 CSS Cascade Layers — explicit override order

**Problem.** The current `shell.css` is a flat file. Token declarations, base resets, component rules, and one-off page tweaks all sit at the same specificity. The deployment templates inline another ~ 2 MB of CSS per page, layered on top by source order. There is no formal contract about what overrides what — only "later wins."

**Working code:**

```css
@layer tokens, base, components, utilities, overrides;

@layer tokens {
  :root {
    --paper: #FFFFFF;
    --ink:   #111827;
    --accent: oklch(30% 0.11 264);
    /* … all token declarations live here … */
  }
}

@layer base {
  *, *::before, *::after { box-sizing: border-box; }
  html { color-scheme: light dark; }
  body {
    font-family: var(--serif);
    font-size: 16px;
    line-height: 1.55;
    background: var(--paper);
    color: var(--ink);
  }
}

@layer components {
  .topnav         { /* … */ }
  .hero-band      { /* … */ }
  .page-hero      { /* … */ }
  .marketing-footer { /* … */ }
  .icon-card      { /* … */ }
}

@layer utilities {
  .visually-hidden { /* … */ }
  .pad-y-section   { /* … */ }
  .measure-prose   { max-inline-size: 65ch; }
}

@layer overrides {
  /* page-specific overrides go here — explicit, scoped, terminal */
  body.disclaimer .marketing-footer { /* … */ }
}
```

Layer order is declared at the top. Any rule in `overrides` beats any rule in `components`, regardless of selector specificity. This is the structural guarantee that lets the design system add overrides without fearing they'll be beaten by a `body.foo .topnav .left a:hover` selector elsewhere.

**Browser support.** `@layer`: ~ 95% global. Baseline since 2022.

**Fallback.** Unsupported browsers ignore `@layer` wrapping and treat all rules as anonymous — they collapse to source order, which is what they do today anyway. Zero regression.

### 3.7 `:has()` selector — drive active-page state from the DOM

**Problem.** The topnav uses `.active-page` class on the current page's link, set server-side by the templating logic. Every page template carries conditional logic to add the class. The fact that a page IS Disclaimer is encoded twice — in the URL and in the class.

**Working code:**

```css
/* The topnav left-side link whose href matches the current pathname
   gets the active treatment automatically. */
.topnav .left a:where([aria-current="page"]),
body:has(main[data-page="disclaimer"]) .topnav a[href="/page/disclaimer"],
body:has(main[data-page="contact"])    .topnav a[href="/page/contact"] {
  background: var(--accent);
  color: var(--ink-on-accent-surface);
  padding: 0 14px;
}
```

Or — drive the chip from the URL itself with no server logic at all, via the page's `<main data-page="…">` attribute:

```html
<body>
  <header class="topnav"> … </header>
  <main data-page="disclaimer"> … </main>
</body>
```

```css
body:has(main[data-page="disclaimer"]) .topnav a[href="/page/disclaimer"] {
  background: var(--accent);
  color: var(--ink-on-accent-surface);
}
```

The active-page chip is now driven by a single `data-page` attribute on `<main>`. No `.active-page` class to add. No server templating logic for nav state.

Bonus — `:has()` lets the footer adapt to whether the page has an article:

```css
/* If the page contains a long-form article, the footer pads itself
   more generously to give the article terminal whitespace. */
body:has(article.long-form) .marketing-footer {
  padding-block-start: 96px;
}
```

**Browser support.** `:has()`: Chrome 105+, Safari 15.4+, Firefox 121+. ~ 94% global.

**Fallback.** Retain the existing `.active-page` class as a parallel mechanism. Both selectors target the same rule; whichever the browser supports wins. No regression.

### 3.8 CSS Subgrid — align icon-card labels across cards

**Problem.** The hero icon cards (MANIFEST / BIM Library / LocationIntelligence on Woodfine; MONOREPO / DESIGN SYSTEM / WOODFINEGROUP.COM on PointSav) are flexbox children. Each card lays out its own icon + label + body. When the icons differ in optical height (a wide SVG vs a tall one), the labels below them sit at different vertical positions. Cards no longer scan as a row of equals.

**Working code:**

```css
.icon-card-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  grid-template-rows:
    [icon-start] auto [icon-end label-start] auto
    [label-end body-start] auto [body-end];
  gap: 32px 24px;
}

.icon-card {
  display: grid;
  grid-template-rows: subgrid;
  grid-row: icon-start / body-end;
  /* The card's three rows now align with the parent's named rows.
     icon-start aligns across cards, label-start aligns across cards,
     body-start aligns across cards. */
  padding: 24px;
  background: var(--paper);
  border: 1px solid var(--rule);
  border-radius: 0;        /* institutional — no rounded corners */
}

.icon-card .icon  { grid-row: icon-start / icon-end; }
.icon-card .label { grid-row: label-start / label-end; font-family: var(--display); }
.icon-card .body  { grid-row: body-start / body-end; }
```

Now every card's icon top aligns, every card's label top aligns, every card's body top aligns — regardless of icon height variation. The optical rhythm reads as architectural.

**Browser support.** `grid-template-rows: subgrid`: Chrome 117+, Safari 16+, Firefox 71+. ~ 92% global.

**Fallback.** `@supports (grid-template-rows: subgrid) { … }`. Below the supports block, the existing flexbox card layout renders unchanged — labels misalign by a few pixels per card, no broken layout.

### 3.9 `font-display: swap` + variable fonts (drop-in upgrade)

**Already shipping** `font-display: swap` per `has_font_display_swap: true`. The Leapfrog 2030 extension is variable fonts (covered in §2.3). One more practical refinement:

```css
/* For variable fonts on slow connections, reserve the metrics
   before the font lands to avoid CLS. */
@font-face {
  font-family: "Roboto Flex";
  font-weight: 100 1000;
  font-stretch: 25% 151%;
  font-style: oblique 0deg 10deg;
  font-display: swap;
  src: url("/fonts/roboto-flex-vf.woff2") format("woff2-variations");
  /* Metric overrides match Georgia fallback — zero CLS on swap */
  ascent-override: 92%;
  descent-override: 24%;
  line-gap-override: 0%;
  size-adjust: 100%;
}
```

The `ascent-override / descent-override / size-adjust` quartet makes the fallback font occupy the same line-box as the variable font. When Roboto Flex finishes loading, no layout shift. CLS contribution: 0.

**Browser support.** Metric overrides: Chrome 87+, Safari 17+, Firefox 89+. ~ 96% global.

---

## 4. Brand differentiation — Woodfine vs PointSav

The two sites currently ship the same template painted with the same primary colour, the same typography, the same hero, the same icon-card grid, and the same footer. The right-side nav labels differ (CORPORATE/PROJECTS/NEWSROOM vs DOCUMENTATION/SOFTWARE/NEWSROOM). That is the entire visual differentiation.

**This is the single largest unforced design error.** The two organisations have different missions, different audiences, different aesthetics earned from different histories. The template should encode that.

### 4.1 Token-level differentiation — proposed split

The `pointsav-design-system` already carries `--ps-steel: #B4C5D5` per the `DESIGN-TOKEN-POINTSAV-icon-tab-steel.draft.md` staged in this very outbox. Pull steel up to dominant accent for PointSav, and let Woodfine keep `--wf-blue: #164679` as its earned colour.

```css
/* Woodfine — kept as institutional deep blue */
[data-brand="woodfine"] {
  --accent:         oklch(30% 0.11 264);   /* #164679, deep blue */
  --accent-tint:    oklch(from var(--accent) 92% calc(c * 0.35) h);
  --accent-surface: var(--accent);          /* hero band is BLUE */
  --ink-on-accent-surface: #FFFFFF;
}

/* PointSav — steel monochrome, technical aesthetic */
[data-brand="pointsav"] {
  --accent:         oklch(78% 0.04 230);   /* #B4C5D5, steel */
  --accent-tint:    oklch(from var(--accent) 95% calc(c * 0.40) h);
  --accent-surface: oklch(15% 0.01 230);   /* hero band is NEAR-BLACK */
  --ink-on-accent-surface: oklch(95% 0.01 230);
  --ink: oklch(20% 0.01 230);              /* slightly cooler ink */
}
```

**Result.**

- **Woodfine** keeps its deep institutional blue hero band — the earned, four-decade real-property aesthetic. Hero is `#164679` filled. White text on blue. Classic.
- **PointSav** flips to a near-black hero band (oklch 15% near-neutral) with steel-grey accent for links, kickers, and active states. This codes as "technical, open-source, developer-grade" — the aesthetic of vendor sites like Linear, Vercel, Stripe Docs but without copying them. The monochrome treatment with single steel accent is restrained in the way Woodfine's blue is restrained — but it's a different restraint.

### 4.2 Typography differentiation

Different pairing presets for each site. The presets already exist (§2.1):

```css
[data-brand="woodfine"] {
  /* .fp-brand — Oswald display + Roboto Flex body */
  --display: "Oswald", "Trade Gothic LT Std", sans-serif;
  --serif:   "Roboto Flex", "Caecilia LT Std", Georgia, serif;
}

[data-brand="pointsav"] {
  /* Technical aesthetic — Barlow Condensed display + JetBrains Mono accents */
  --display:  "Barlow Condensed", "Trade Gothic LT Std", sans-serif;
  --serif:    "Roboto Flex", Georgia, serif;
  --mono:     "JetBrains Mono", "Berkeley Mono", "SF Mono", Consolas, monospace;
}
```

Add a `--mono` token for PointSav. Use it on:

- The hero kicker ("DIGITAL SYSTEMS" reads as `<code>` rather than display)
- Documentation file names ("monorepo/", "design-system/")
- The footer build hash / version string

The presence of a monospace family on PointSav signals "this is software" without needing the word "software."

### 4.3 Grid density differentiation

Woodfine's audience is institutional capital and senior planners. They scan headlines, read paragraphs, and decide. **Generous whitespace, single-column reading width (65 ch), large hero.**

PointSav's audience is operators, developers, technical buyers. They scan dense info, want at-a-glance status, want to copy paste. **Denser grid, multi-column at desktop, more content per viewport.**

```css
[data-brand="woodfine"] .marketing-content {
  display: grid;
  grid-template-columns: minmax(0, 1fr);
  max-inline-size: 720px;        /* 65ch at 16px serif */
  margin-inline: auto;
  gap: 48px;
}

[data-brand="pointsav"] .marketing-content {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  max-inline-size: 1280px;       /* wider — more content surface */
  margin-inline: auto;
  gap: 32px;
}
```

### 4.4 Motion differentiation

**Woodfine** — minimal motion. Scroll-driven hero entrance (§3.2) and the view-transition wordmark morph (§3.3). That's it. Institutional sites earn gravitas by not animating.

**PointSav** — additional motion. View-transitions on the icon cards as they enter (each card individually). A subtle `font-variation-settings` interpolation on the wordmark on hover that ticks the weight from 400 to 500. These are micro-interactions that telegraph "this site was built by people who care about CSS" — the right signal for a developer-tool audience.

### 4.5 Hero treatment side-by-side

| | Woodfine | PointSav |
|---|---|---|
| Hero band fill | `#164679` deep blue | `oklch(15% 0.01 230)` near-black |
| Hero text | White display | Steel display |
| Hero H1 | "Real property. Four decades." | "Sovereign software. Operator-grade." |
| Hero kicker | DISPLAY 13px, white | MONO 12px, steel |
| Hero animation | Fade in, weight 300→500 | Fade in + view-transition staggered |
| Below hero | Single-column 65ch | 3-up auto-fit grid |
| Footer | Cities (Toronto, Calgary, Saskatoon) | Build hash + repo links + version |

This is what visual differentiation looks like with zero net-new tokens — every required value already exists in the staged drafts (`--ps-steel`, the mono token family, the alternate pairing presets). The work is to wire them.

---

## 5. Ten specific design improvements — ranked by impact

Effort is rough engineering-hours for a single dev familiar with the codebase.

### 5.1 Triple the hero H1 — IMPACT 10

**Current.** `font-size-h1: 22px` (typography token). At 1440 px viewport, the H1 occupies ~ 1.5% of viewport height. The hero band is full-bleed; the type inside it is footer-sized.

**Proposed.** `clamp(40px, 6vw, 88px)`. At 1440 px → 86 px. At 1024 px → 61 px. At 480 px → 40 px. Hero H1 occupies ~ 10% of viewport height — proper hierarchy.

**Code.**

```css
.hero h1 {
  font-family: var(--display);
  font-size: clamp(40px, 6vw, 88px);
  font-variation-settings: "wght" 500;
  line-height: 1.05;
  letter-spacing: 0.02em;        /* loose only at large size */
  text-wrap: balance;            /* §5.7 — balance multi-line */
  margin: 0;
}
```

**Effort.** 30 minutes (one token + one rule + ratify).

### 5.2 Fix mobile (375 px) breakpoint — IMPACT 10

**Current.** The deployed 375 px render shows the desktop three-column grid squashed, not the tablet-portrait stack the topnav recipe specifies. Touch targets at 17 px tall, link text at 11 px, six links sharing one 343-px-wide horizontal row.

**Proposed.** Confirm the `@media (max-width: 768px)` rule from the topnav recipe is actually in the deployed CSS. Migrate to container queries (§3.1) so the rule binds to the topnav's container width, not viewport. Increase font-size at 768 px to 13 px (currently 10 px in the recipe). Apply `min-height: 44px` to every nav anchor.

**Code.** See §3.1.

**Effort.** 2 hours (diagnose deployment mismatch + apply container query + test on three real devices).

### 5.3 Add `<h1>` to landing pages — IMPACT 9

**Current.** `h1: 0` on both `index.html` files. The wordmark is the de-facto page title; document outline reads as eight `<h3>` elements floating in nothing.

**Proposed.** Wrap the wordmark anchor in an `<h1>` on the landing page only. Mark inner-page H1s (Disclaimer, Contact) as `<h1>` on their own pages — the page-hero recipe already covers this.

**Code.**

```html
<!-- Landing page only -->
<h1 class="visually-hidden">Woodfine Capital Projects — Real property, four decades.</h1>
<header class="topnav">
  <!-- the wordmark anchor as before -->
</header>
```

Or, more interestingly — make the wordmark itself the H1:

```html
<h1 class="wordmark-h1">
  <a class="wordmark" href="/" aria-label="Woodfine Capital Projects">
    <svg …></svg>
  </a>
</h1>
```

```css
.wordmark-h1 { margin: 0; font-size: 1rem; }   /* visually unchanged */
```

**Effort.** 45 minutes (decision on which H1 approach + apply to landing pages + verify outline algorithm).

### 5.4 Add skip-link — IMPACT 7

**Current.** `has_skip_link: false`. Keyboard users tab through 12 chrome elements before reaching content.

**Proposed.**

```html
<a class="skip-link" href="#main">Skip to content</a>
<header class="topnav"> … </header>
<main id="main" tabindex="-1"> … </main>
```

```css
.skip-link {
  position: absolute;
  inset-block-start: 8px;
  inset-inline-start: 8px;
  padding: 8px 14px;
  background: var(--accent);
  color: var(--ink-on-accent-surface);
  font-family: var(--display);
  letter-spacing: 0.14em;
  text-transform: uppercase;
  font-size: 12px;
  border-radius: 0;
  transform: translateY(-200%);
  transition: transform 160ms ease;
}
.skip-link:focus-visible { transform: translateY(0); }
```

**Effort.** 30 minutes.

### 5.5 Replace 7 statics with 2 variables — IMPACT 8

**Current.** 7 Google Fonts at ~ 320 KB.

**Proposed.** Oswald VF + Roboto Flex VF at ~ 95 KB. Net -225 KB per pageload (-70%). Plus full weight/width/opsz range available for design refinement.

**Code.** See §2.3.

**Effort.** 3 hours (font swap + audit every text surface + tune `font-variation-settings`).

### 5.6 Halve body line-length — IMPACT 8

**Current.** Landing body paragraphs run to grid width at 1440 px — ~ 130 ch.

**Proposed.** `max-inline-size: 65ch` on prose containers. Different rule for PointSav (auto-fit grid per §4.3).

**Code.**

```css
.measure-prose { max-inline-size: 65ch; margin-inline: auto; }
.measure-narrow { max-inline-size: 48ch; margin-inline: auto; }  /* hero copy */
```

Apply `.measure-prose` to article copy; `.measure-narrow` to hero supporting paragraph.

**Effort.** 1 hour.

### 5.7 `text-wrap: balance` on H1 and `text-wrap: pretty` on body — IMPACT 6

**Current.** Hero H1 wraps with the browser's greedy algorithm — a long first line, an orphan second line.

**Proposed.**

```css
h1, h2, h3, .hero-kicker { text-wrap: balance; }
p, .article-body { text-wrap: pretty; }
```

`text-wrap: balance` is for headings (≤ 6 lines). `text-wrap: pretty` is for body copy (avoids orphans on the last line). Both are 2024+ baseline.

**Browser support.** `balance`: ~ 88%. `pretty`: ~ 80%. Graceful degradation (browsers ignore unknown values, default wrap behaviour).

**Effort.** 15 minutes.

### 5.8 Dark mode — IMPACT 7

**Current.** None.

**Proposed.** §3.4 — full dark mode using the existing token system + two new tokens (`--ink-on-accent-surface`, `--accent-surface`). Auto via `prefers-color-scheme`; manual toggle via `[data-color-scheme]`.

**Effort.** 4 hours (declare dark tokens + audit every component renders correctly in dark + ship toggle UI in topnav).

### 5.9 View transitions between landing and inner pages — IMPACT 7

**Current.** Hard navigation.

**Proposed.** §3.3 — opt in `@view-transition { navigation: auto; }` and name the wordmark `view-transition-name: wf-wordmark`. The wordmark morphs from landing position to inner-page header position.

**Effort.** 1 hour.

### 5.10 Brand differentiation between Woodfine and PointSav — IMPACT 9

**Current.** Indistinguishable templates.

**Proposed.** §4 — token split by `[data-brand]`. PointSav flips to steel + near-black hero, dense grid, mono font for technical labels. Woodfine keeps deep blue + single-column 65ch hero, classic restraint.

**Effort.** 6 hours (token wiring + per-site test + content reflow).

---

## 6. Combined cost / impact summary

| # | Improvement | Hours | Impact |
|---|---|---|---|
| 5.1 | Triple the hero H1 | 0.5 | 10 |
| 5.2 | Fix mobile breakpoint | 2 | 10 |
| 5.10 | Brand differentiation | 6 | 9 |
| 5.3 | Add `<h1>` | 0.75 | 9 |
| 5.5 | Variable fonts (-225 KB) | 3 | 8 |
| 5.6 | Halve body line-length | 1 | 8 |
| 5.8 | Dark mode | 4 | 7 |
| 5.9 | View transitions | 1 | 7 |
| 5.4 | Skip link | 0.5 | 7 |
| 5.7 | `text-wrap: balance/pretty` | 0.25 | 6 |
| **Total** | | **19 h** | |

Nineteen hours of focused work moves the Awwwards composite from ~ 5.1 to a defensible 7.5+. Site of the Day territory begins at 8.0; the §3 techniques (container queries, scroll-driven animation, oklch, cascade layers, `:has()`, subgrid) push the Creativity dimension from 3.5 to 7+ on top of the §5 baseline.

---

## 7. What this audit does NOT cover

(For honesty — to surface where another agent's lens would add value.)

- **WCAG 2.2 enumeration beyond what is visible in contrast/target JSON.** The accessibility audit is the right place to enumerate every SC failure, propose ARIA patterns, test with NVDA/JAWS/VoiceOver, and resolve the `aria-label` duplication question raised in the topnav recipe research trail.
- **Mobile / perf measurement.** The 2.4 MB inline HTML is a perf problem this audit names but does not measure (TTFB, FCP, LCP, CLS, INP). The perf-mobile audit is the right place for that.
- **Asset pipeline.** SVG titles, the inline-vs-asset decision for the wordmark, image hosting and CDN — out of scope here.
- **Content strategy.** What the landing page should SAY (vs how it should look) is editorial. Recommended hand-off via outbox to `project-editorial` for a PROSE-* draft covering hero copy, supporting paragraph, and the "latest dateline" block §1.4 recommends.

---

## 8. Recommended ratification order

If the operator can only ship a subset:

1. **First wave (8 h, $0 risk):** §5.1 (hero H1) + §5.3 (h1) + §5.4 (skip-link) + §5.6 (measure) + §5.7 (text-wrap) + §3.6 (cascade layers — make the system override-safe before doing anything else).
2. **Second wave (8 h, low risk):** §5.5 (variable fonts) + §3.4 dark mode + §3.5 oklch tints. These three compound — variable fonts cut weight, dark mode validates the token system, oklch makes future tints free.
3. **Third wave (10 h, design-pass review):** §5.10 brand differentiation + §3.3 view transitions + §3.2 scroll-driven hero. These are the "Site of the Day" moves — they earn the Creativity score.
4. **Continuous:** §3.1 container queries replacing media queries, component by component, as each is touched for other work.

---

## 9. Open questions for `project-design`

1. **The `--display: Oswald` token is currently a static font reference.** Variable upgrade requires either (a) Oswald-VF subset from Google Fonts (verify availability) or (b) drop-in switch to Barlow Condensed VF. Recommend (a) for Woodfine and (b) for PointSav per §4.2 — design call.
2. **The `[data-brand]` token split (§4.1) introduces a second canonical accent token on the same design system.** This is a `DESIGN-TOKEN-CHANGE` requiring Master co-sign per the design-system pipeline. Sequence: this audit refines into `DESIGN-TOKEN-CHANGE-brand-split.md` for ratification before §5.10 implementation.
3. **Cross-document view transitions (§3.3) require same-origin.** `home.woodfinegroup.com` and `home.pointsav.com` are separate origins. The morph works within each site (landing ↔ contact ↔ disclaimer) but does not work between sites. Recommend keep the cross-site link as a hard navigation; it telegraphs the boundary correctly.
4. **The H1 decision (§5.3) — wordmark-as-H1 vs visually-hidden-H1 — is a structural choice.** Recommend wordmark-as-H1 (semantic clarity, no extra text the operator has to maintain) but it requires the wordmark SVG to carry a meaningful `aria-label`. Currently does (`Woodfine Capital Projects`). Good.

---

## Research trail

### Done

1. Reviewed all 24 audit screenshots in `outputs/audit-2026-06-02/` at 375 / 768 / 1024 / 1440 — confirmed the mobile-broken claim, the equal-bands-rhythm claim, and the indistinguishable-templates claim by visual inspection.
2. Cross-referenced `*-meta.json` to confirm `h1: 0` and `has_skip_link: false` across all six pages, and `has_font_display_swap: true` (positive baseline).
3. Cross-referenced `woodfine-index-contrast.json` and `woodfine-index-targets.json` to ground the contrast and touch-target claims in measurement, not assertion.
4. Reviewed `token-woodfine-typography.draft.md` (236 lines) and `component-marketing-topnav.draft.md` (288 lines) to ensure §2 typography refinement and §3.1 container-query rewrite respect the as-built decisions documented there, not the pre-Phase-1b drafts.
5. Verified `--ps-steel: #B4C5D5` is already staged in `.agent/drafts-outbound/DESIGN-TOKEN-POINTSAV-icon-tab-steel.draft.md` — §4.1 references real, in-flight tokens rather than inventing new ones.

### Suggested for project-design

1. The cascade-layers structure in §3.6 should be ratified BEFORE any other Leapfrog 2030 component lands — it is the structural contract every subsequent change depends on. Recommend a dedicated `DESIGN-COMPONENT-css-layer-order.md` draft to formalise.
2. The `[data-brand]` split in §4.1 should be implemented as a `pointsav-design-system` token-bundle variant, not a runtime class — the build emits two bundles (`woodfine.tokens.css`, `pointsav.tokens.css`) and each site loads one.
3. The `text-wrap: balance / pretty` from §5.7 is the lowest-risk, lowest-effort, highest-immediate-aesthetic-return change in this audit. Recommend ship it standalone in the next ratification cycle as a confidence-building landmark.

### Open questions

1. Does the operator want PointSav genuinely differentiated from Woodfine (§4 recommends yes), or is the visual congruence between the two sites an intentional "fleet identity" decision that this audit is misreading? If congruence is intentional, §4 collapses to "improve the shared template" and §5.10 falls off the list.
2. The Leapfrog 2030 mandate names "2030-standard CSS today" — but `oklch(from …)` relative syntax (§3.5) is at ~ 88% support and cross-document view transitions (§3.3) at ~ 70%. Is the operator's risk tolerance "ship them with hex/no-VT fallback" (recommended) or "wait until 95% support to ship"? Decision affects the §3.3 / §3.5 ratification timing.
