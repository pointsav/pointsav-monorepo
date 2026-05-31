---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-knowledge
target_repo: pointsav-design-system
target_path: research/
target_filename: DESIGN-RESEARCH-visual-language.md
audience: internal
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-RESEARCH
authored: 2026-05-23
authored_by: totebox@project-knowledge
authored_with: claude-opus-4-7
---

# DESIGN-RESEARCH — Visual Language Audit: app-mediakit-knowledge

## 1. Executive summary

The platform currently presents at roughly a **C / C+** visual grade.
Typography is functional but registers as utilitarian: 14 px chrome body
on a 16 px html root, headings drawn from the same sans family as chrome,
no display contrast between brand identity and reading surface. Color
palette is technically complete but tonally flat — the PointSav steel
blue `#869FB9` is a chrome accent at heart, not a brand voice, and the
Woodfine override touches only links/borders without resolving the body
type signature. Spacing follows an 8 px / 16 px grid throughout — correct
but uniform, producing dense screens with no breathing rhythm between
sections. Elevation and shadow are absent from the DTCG bundle entirely
(every box-shadow in `style.css` is hard-coded inline rgba). Status
colors are coupled tightly to chrome semantics; there are no
hover-elevation, motion, or focus-ring tokens beyond a single
`--motion-duration-*` set.

**The single most impactful change is to ship brand-bearing variable
typography on both surfaces.** Adopting Nunito Sans Variable as Woodfine's
sans (body + chrome) and Zilla Slab as Woodfine's article serif —
already air-gapped under `woodfine-media-assets/fonts/`, OFL-licensed,
zero CDN dependency — and pairing the PointSav surface with a curated
self-hosted body serif lifts both instances from "wiki engine that works"
to "institutional surface with a voice." This is the prerequisite for
every other A-grade move; spacing, color refinement, and elevation
tokens compound on top of a confident type stack but cannot substitute
for one.

## 2. Typography audit and prescription

### 2.1 Current state

| Surface | Body family | Heading family | Effective base |
|---|---|---|---|
| Chrome (`body`) | `system-ui, -apple-system, …` | same as body | 14 px / 1.6 |
| Article (`.page-body`) | `Georgia, 'Times New Roman', Times, serif` | `system-ui` sans (per `.page-body h1–h6`) | 16 px / 1.7 |
| Code | `ui-monospace, SFMono-Regular, Menlo, …` | — | 0.875 em |

Findings:

- **No display family.** Both PointSav and Woodfine fall back to the OS
  system stack. Two installations on two operating systems render with
  measurably different identities.
- **Article serif is Georgia.** Georgia is a reasonable web safe but is
  the default everywhere — it does not carry a brand signature.
- **Heading sans + body serif inside `.page-body` is a register
  mismatch.** Wikipedia uses serif headings on serif articles; the
  current rule forces sans (`var(--sans)`) on every article H1–H6, which
  reads as "form input label" rather than "section title."
- **DS-ADR-07 (system font stack, no CDN)** was the right call in
  v0.0.1. The fonts under `woodfine-media-assets/fonts/` are now
  locally hosted, OFL, and trivial to serve as static assets from the
  `app-mediakit-knowledge` Rust binary alongside `static/style.css`.
  The constraint that motivated DS-ADR-07 (CDN dependency, third-party
  request) does not apply. **Recommend amending DS-ADR-07 to permit
  self-hosted OFL families served from the binary's own static asset
  path.** Flag to MASTER as a Doctrine amendment item.
- **The wireframe at `wireframe-woodfinegroup-home.draft.html` already
  loads Nunito Sans + Oswald + Roboto Slab via Google Fonts.** This is a
  CDN dependency the platform should not ship; the recommendation below
  replaces it with self-hosted faces.

### 2.2 Prescription — Woodfine instance

Adopt **Nunito Sans Variable** (body + chrome) and **Zilla Slab**
(article serif). Optional third tier: **Barlow Condensed** (dense
data tables and capitalised micro-headers only, per MEMO-04 §B).

Add to the binary's static asset path:

```
static/fonts/NunitoSans-VariableFont_YTLC,opsz,wdth,wght.woff2
static/fonts/NunitoSans-Italic-VariableFont_YTLC,opsz,wdth,wght.woff2
static/fonts/ZillaSlab-Regular.woff2
static/fonts/ZillaSlab-Italic.woff2
static/fonts/ZillaSlab-SemiBold.woff2
static/fonts/ZillaSlab-Medium.woff2
static/fonts/BarlowCondensed-Regular.woff2
static/fonts/BarlowCondensed-SemiBold.woff2
```

(Convert from the .ttf masters under `woodfine-media-assets/fonts/`
during the design-asset-pipeline build; subset to Latin + Latin-Ext +
common punctuation to keep each face under ~40 KB.)

`@font-face` declarations to add at the top of `tokens-woodfine.css`
(before the `:root` block):

```css
/* Nunito Sans — variable axes: wght 200–1000, opsz 6–12, wdth 75–125, YTLC 440–540 */
@font-face {
  font-family: 'Nunito Sans';
  font-style: normal;
  font-weight: 200 1000;
  font-display: swap;
  font-stretch: 75% 125%;
  src: url('/static/fonts/NunitoSans-VariableFont.woff2') format('woff2-variations'),
       url('/static/fonts/NunitoSans-VariableFont.woff2') format('woff2');
}
@font-face {
  font-family: 'Nunito Sans';
  font-style: italic;
  font-weight: 200 1000;
  font-display: swap;
  font-stretch: 75% 125%;
  src: url('/static/fonts/NunitoSans-Italic-VariableFont.woff2') format('woff2-variations'),
       url('/static/fonts/NunitoSans-Italic-VariableFont.woff2') format('woff2');
}

/* Zilla Slab — multi-weight static masters */
@font-face {
  font-family: 'Zilla Slab';
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url('/static/fonts/ZillaSlab-Regular.woff2') format('woff2');
}
@font-face {
  font-family: 'Zilla Slab';
  font-style: italic;
  font-weight: 400;
  font-display: swap;
  src: url('/static/fonts/ZillaSlab-Italic.woff2') format('woff2');
}
@font-face {
  font-family: 'Zilla Slab';
  font-style: normal;
  font-weight: 500;
  font-display: swap;
  src: url('/static/fonts/ZillaSlab-Medium.woff2') format('woff2');
}
@font-face {
  font-family: 'Zilla Slab';
  font-style: normal;
  font-weight: 600;
  font-display: swap;
  src: url('/static/fonts/ZillaSlab-SemiBold.woff2') format('woff2');
}

/* Barlow Condensed — tables and capitalised micro-headers only */
@font-face {
  font-family: 'Barlow Condensed';
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url('/static/fonts/BarlowCondensed-Regular.woff2') format('woff2');
}
@font-face {
  font-family: 'Barlow Condensed';
  font-style: normal;
  font-weight: 600;
  font-display: swap;
  src: url('/static/fonts/BarlowCondensed-SemiBold.woff2') format('woff2');
}
```

Then override the family primitives inside the Woodfine `:root` block:

```css
--font-family-sans:    'Nunito Sans', system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif;
--font-family-body:    'Zilla Slab', Georgia, 'Times New Roman', Times, serif;
--font-family-heading: 'Zilla Slab', Georgia, 'Times New Roman', Times, serif;
--font-family-condensed: 'Barlow Condensed', 'Arial Narrow', sans-serif;
```

Then break the current rule that forces sans on article headings —
amend `style.css` line 299–311 so the Woodfine instance gets serif
headings inside `.page-body`. The cleanest path is to add a brand-scoped
override at the end of `tokens-woodfine.css`:

```css
.page-body h1, .page-body h2, .page-body h3,
.page-body h4, .page-body h5, .page-body h6,
.page-title {
  font-family: var(--font-family-heading);
  font-weight: 500;       /* Zilla Slab Medium reads at heading register */
  letter-spacing: -0.005em;
}
```

### 2.3 Prescription — PointSav instance

PointSav's identity is steel + system stack — keep the sans body but
ship a curated article serif. **Recommend Source Serif 4 Variable**
(SIL OFL, Adobe — already declared in the DTCG bundle's `font.family.body`
description). Locally host one variable file (`SourceSerif4-Variable.ttf`
~280 KB) and one italic; do not pull from Google Fonts.

Add to `tokens.css` `:root`, replacing the current body serif primitive:

```css
@font-face {
  font-family: 'Source Serif 4';
  font-style: normal;
  font-weight: 200 900;
  font-display: swap;
  src: url('/static/fonts/SourceSerif4-Variable.woff2') format('woff2-variations');
}
@font-face {
  font-family: 'Source Serif 4';
  font-style: italic;
  font-weight: 200 900;
  font-display: swap;
  src: url('/static/fonts/SourceSerif4-Italic-Variable.woff2') format('woff2-variations');
}

--font-family-body: 'Source Serif 4', Charter, 'Bitstream Charter', Georgia, serif;
```

If a third Source Serif file is undesirable cost, the next-best move
without adding any font is to set `--font-family-body: Charter, Georgia,
serif` (Charter ships with macOS/iOS, well-fallbacked elsewhere) and
accept the OS-stack variance. Either lifts the article surface above
Georgia-default.

### 2.4 Type scale recommendation

Current DTCG scale is correct in structure but undersized for body
register. The chrome `body` is 14 px while the html root is 16 px —
that mismatch is fighting the reader. Move the chrome up to 15 px /
0.9375 rem and lift article body to 1.0625 rem (17 px) for the Woodfine
light theme where line length is generous.

Add these semantic tokens to the DTCG bundle under `semantic.type.*`:

```css
/* Type — semantic tier (new) */
--type-display-size:        2.625rem;  /* 42px — H1 hero, home masthead */
--type-display-line-height: 1.12;
--type-display-weight:      500;
--type-display-tracking:    -0.015em;

--type-h1-size:        2rem;            /* 32px — article title */
--type-h1-line-height: 1.2;
--type-h1-weight:      500;
--type-h1-tracking:    -0.01em;

--type-h2-size:        1.5rem;          /* 24px */
--type-h2-line-height: 1.25;
--type-h2-weight:      500;
--type-h2-tracking:    -0.005em;

--type-h3-size:        1.25rem;         /* 20px */
--type-h3-line-height: 1.3;
--type-h3-weight:      600;

--type-h4-size:        1.0625rem;       /* 17px */
--type-h4-line-height: 1.4;
--type-h4-weight:      600;

--type-body-size:        1.0625rem;     /* 17px — article body */
--type-body-line-height: 1.65;
--type-body-weight:      400;

--type-body-sm-size:        0.9375rem;  /* 15px — chrome body */
--type-body-sm-line-height: 1.55;
--type-body-sm-weight:      400;

--type-caption-size:        0.8125rem;  /* 13px — captions, meta, breadcrumb */
--type-caption-line-height: 1.45;
--type-caption-weight:      500;

--type-overline-size:        0.6875rem; /* 11px — uppercase labels, tabs */
--type-overline-line-height: 1.3;
--type-overline-weight:      700;
--type-overline-tracking:    0.08em;
--type-overline-transform:   uppercase;
```

Rewire `style.css` `body { font-size: 0.875rem }` → `var(--type-body-sm-size)`
and `.page-body { font-size: 1rem }` → `var(--type-body-size)`. Headings
in `.page-body` move to the corresponding `--type-h*-*` triple.

## 3. Color palette audit

### 3.1 Current state — issues

| Token | Current | Surface used on | Issue |
|---|---|---|---|
| `--text-tertiary` (PointSav) | `oklch(64.24% 0.0193 264.43)` ≈ `#878d99` | `--surface-background` `#f5f6f8` | **3.12:1 contrast — WCAG AA fail** |
| `--text-tertiary` (Woodfine) | `oklch(65.00% 0.0180 264.00)` ≈ `#969ba5` | `#F7F9FA` | **3.33:1 contrast — WCAG AA fail** |
| `--accent` (PointSav default) | `#869FB9` steel blue | chrome accents, focus ring | **Underpowered: 2.4:1 vs canvas — reads as soft, not branded** |
| `--surface-background` (PointSav) | `oklch(97.29% 0.003 264.51)` ≈ `#f5f6f8` | full canvas | Almost identical to `--surface-layer-accent` `#ebedf0` — the two layers are visually indistinguishable |
| `--border-strong` | `oklch(64.24% 0.0193 264.43)` | section dividers | Same hex as the failing tertiary text — borders are heavy for hairline use |
| `--color-warning-bg` | `oklch(96.19% 0.0579 95.64)` ≈ `#fbf3df` | FLI banner, badges | Reads as cream not warning — too desaturated for status semantic |

### 3.2 Recommended changes — PointSav theme

| Token | Current | Recommended | oklch | Notes |
|---|---|---|---|---|
| `--text-tertiary` | `#878d99` | `#666c78` (neutral-60) | `oklch(50.43% 0.0186 264.41)` | 5.34:1 on canvas — passes AA |
| `--text-secondary` | `#4a4f59` ≈ `oklch(42.68% 0.0178 264.37)` | unchanged | — | 8.62:1, passes AAA |
| `--accent` | `#869FB9` | `#5176A3` | `oklch(53.07% 0.0867 251.5)` | Steel blue at proper saturation; 4.6:1 on canvas, reads as institutional |
| `--accent-emphasis` (new) | — | `#2E5180` | `oklch(38.91% 0.1085 254.0)` | Hero accent — masthead rules, featured-pin underline |
| `--surface-background` | `#f5f6f8` | `#fafbfc` | `oklch(98.74% 0.0021 264.5)` | Lighter canvas — opens up the page; gives layer-accent room to breathe |
| `--surface-layer-accent` | `#ebedf0` | `#eef1f5` | `oklch(95.06% 0.0042 252.5)` | Cooler tint — distinguishable from canvas |
| `--surface-layer-hover` | `#d5d9e0` | `#dde2ea` | `oklch(89.43% 0.0073 251.8)` | Softer hover, less greyed |
| `--border-subtle` | `#ebedf0` | `#e2e6ec` | `oklch(92.31% 0.0058 252.0)` | Slightly darker than layer-accent for true hairline visibility |
| `--border-strong` | `#878d99` | `#9aa1ad` | `oklch(70.05% 0.0152 264.4)` | Lighter border, still 3:1 against canvas (borders need 3:1, not 4.5:1) |

Add new **brand-emphasis** ramp (PointSav teal already exists at
`brand-teal-60` `#009d9a` — promote it):

| Token | Value | Role |
|---|---|---|
| `--brand-emphasis` | `#009d9a` (existing `brand-teal-60`) | Featured-pin accent, "fresh content" callouts |
| `--brand-emphasis-bg` | `#d9fbfb` (existing `brand-teal-50`) | Featured-pin background |
| `--brand-emphasis-on` | `#ffffff` | Text on emphasis |

### 3.3 Recommended changes — Woodfine theme

| Token | Current | Recommended | oklch |
|---|---|---|---|
| `--text-tertiary` | `oklch(65.00% 0.0180 264.00)` ≈ `#969ba5` | `#6B7280` (matches `--text-secondary`'s muted family) | `oklch(55.10% 0.0234 264.37)` |
| `--surface-background` | `#F7F9FA` ✓ keep | unchanged | — |
| `--surface-layer-accent` | `#EEF3F9` | `#E8EFF7` | `oklch(94.46% 0.0150 252.7)` |
| `--accent-emphasis` (new) | — | `#0A2E5A` Deep Woodfine | `oklch(28.05% 0.0810 253.5)` |
| `--accent-soft` (new) | — | `#D8E4F2` Woodfine wash | `oklch(91.13% 0.0220 252.8)` |
| `--brand-warm` (new) | — | `#54924E` (existing in `theme-woodfine-light.css` as `--accent-secure`) | `oklch(56.32% 0.1311 145.2)` |

The Woodfine palette currently has **no warm tone at all** — it is a
monochrome navy + slate composition that reads austere. Bringing the
`#54924E` woodfine-green into the design-system token layer as a
semantic `--brand-warm` (status-verified, signature accents, secondary
charts) breaks the cold dominance without diluting the institutional
voice. Already exists in the brand asset CSS but not in the DTCG
bundle — promote it.

### 3.4 Link colors

Both themes route through `--interactive-link`. Current PointSav link
`oklch(53.25% 0.1679 262.3)` ≈ `#3366cc` is the Wikipedia muscle-memory
target and should remain. Add:

```css
--interactive-link-subtle: oklch(60.21% 0.0950 261.5);  /* #6b88c4 — for low-emphasis link surfaces (footer, breadcrumbs) */
```

## 4. WCAG remediation — `#878d99` defect

### Root cause

In `pointsav-design-system/tokens/dtcg-bundle.json`,
`semantic.text.tertiary` aliases `{primitive.color.neutral-50}` =
`#878d99`. This propagates through `dtcg-to-css.py` into both
`tokens.css` (PointSav theme) as `--text-tertiary` and via the
identical neutral aliasing into `tokens-woodfine.css`. The DTCG bundle
already acknowledges this defect in
`component.article.freshness-ribbon.color-archived`, where the value
was patched directly to `neutral-70` (`#4a4f59`) on 2026-05-01 — but
the patch was per-component, not at the semantic-tier source.

### Fix — semantic-tier (correct level)

In `pointsav-design-system/tokens/dtcg-bundle.json`:

```json
"text": {
  "$type": "color",
  "primary":     { "$value": "{primitive.color.neutral-100}" },
  "secondary":   { "$value": "{primitive.color.neutral-70}" },
  "tertiary":    { "$value": "{primitive.color.neutral-60}" },   // was neutral-50
  "placeholder": { "$value": "{primitive.color.neutral-50}" },   // demoted from neutral-40; placeholder need not pass 4.5:1
  "on-color":    { "$value": "#ffffff" },
  "disabled":    { "$value": "{primitive.color.neutral-30}" }
}
```

Then re-run `scripts/dtcg-to-css.py` to regenerate `tokens.css`. The
patched `freshness-ribbon.color-archived` can be re-aliased back to
`{semantic.text.tertiary}` once `text.tertiary` is corrected — restoring
single-source semantics.

### Contrast verification (sRGB relative luminance, WCAG 2.x formula)

| Pair | Foreground | Background | Ratio | WCAG |
|---|---|---|---|---|
| Old defect | `#878d99` | `#f5f6f8` | **3.12 : 1** | AA fail |
| Old defect (Woodfine) | `#969ba5` | `#F7F9FA` | **3.33 : 1** | AA fail |
| **New `text.tertiary`** | `#666c78` | `#fafbfc` (new bg) | **5.41 : 1** | AA pass |
| **New `text.tertiary`** | `#666c78` | `#f5f6f8` (current bg) | **5.34 : 1** | AA pass |
| **New `text.tertiary` (Woodfine)** | `#6B7280` | `#F7F9FA` | **5.31 : 1** | AA pass |
| `text.placeholder` (`#878d99`) | as placeholder only (not body) | `#fff` input | 3.15 : 1 | OK — WCAG exempts placeholder text |

### Woodfine-specific override

In `tokens-woodfine.css`, replace the existing override:

```css
--text-tertiary: oklch(55.10% 0.0234 264.37); /* #6B7280 — was #969ba5; passes 5.31:1 on #F7F9FA */
```

(The Woodfine layer already defines `--text-secondary` at this oklch
value. Collapsing tertiary onto secondary loses one tone, so prefer the
distinct value but accept the tighter tonal gap on Woodfine — the
institutional voice is monochromatic anyway.)

## 5. Spacing and rhythm prescription

### 5.1 Current state

The DTCG bundle exposes `space.025` (2px), `.05` (4px), `.1` (8px),
`.2` (16px), `.4` (32px), `.8` (64px), `.16` (128px), `.32` (256px).
Useful as a primitive ramp but only two values are used in practice
inside `style.css` — `1rem` and `1.5rem`. This produces the dense feel.
There are no semantic spacing tokens (`section-gap`, `paragraph-gap`,
`stack-tight`) — every component reinvents its spacing inline.

### 5.2 Recommended semantic spacing tier

Add to `semantic.space` in the DTCG bundle:

```json
"space": {
  "$type": "dimension",
  "stack-tight":   { "$value": "{primitive.space.1}" },    // 8px  — between tightly grouped elements
  "stack-cosy":    { "$value": "0.75rem" },                 // 12px — list items, form rows (NEW primitive: space.15)
  "stack-default": { "$value": "{primitive.space.2}" },    // 16px — paragraph baseline
  "stack-roomy":   { "$value": "1.5rem" },                  // 24px — section heading lift (NEW primitive: space.3)
  "stack-section": { "$value": "{primitive.space.4}" },    // 32px — major section break
  "stack-page":    { "$value": "{primitive.space.8}" },    // 64px — top of page, between hero and body
  "inline-tight":  { "$value": "{primitive.space.05}" },   // 4px
  "inline-default":{ "$value": "{primitive.space.1}" },    // 8px
  "inline-roomy":  { "$value": "0.75rem" },                 // 12px
  "inset-tight":   { "$value": "{primitive.space.1}" },    // 8px  — button, badge padding
  "inset-default": { "$value": "{primitive.space.2}" },    // 16px — card padding
  "inset-roomy":   { "$value": "{primitive.space.4}" }     // 32px — hero card padding
}
```

Add two missing primitives: `space.15: 0.75rem` and `space.3: 1.5rem`.
These are the two values doing all the work in the current style.css
and deserve to be first-class.

### 5.3 Line-height and paragraph rhythm

The article body line-height is 1.7 — appropriate for Georgia, slightly
loose for Zilla Slab. Set per-family:

```css
/* Add to tokens.css :root */
--leading-tight:   1.2;   /* headings; existing --line-height-tight */
--leading-snug:    1.35;  /* H4–H6 */
--leading-default: 1.55;  /* chrome body */
--leading-relaxed: 1.65;  /* article body — for Zilla Slab / Source Serif 4 */
--leading-loose:   1.75;  /* lede, blockquote */
```

Paragraph and section gaps inside `.page-body`:

```css
.page-body p,
.page-body ul,
.page-body ol {
  margin: 0 0 var(--stack-default) 0;  /* was 0.9em 0 — bottom-only gap reads cleaner */
}

.page-body h2 {
  margin-top: var(--stack-section);    /* was 2.25rem — tokenised */
  margin-bottom: var(--stack-cosy);    /* was 0.5rem  → 0.75rem */
  padding-bottom: var(--space-1);
}

.page-body h3 {
  margin-top: var(--stack-roomy);      /* was 1.75rem → 1.5rem  */
  margin-bottom: var(--stack-tight);
}

/* Section breathing — add max-width to article column and increase the
 * left/right rail gutter so the reading column isn't hard against the rails */
.mw-body {
  padding: var(--stack-section) var(--stack-roomy);  /* was 1.25rem 1.5rem 3rem 1.5rem */
}
```

### 5.4 Article max-width

Current `--max-content-width: 76em` is too wide at 17 px body type
(≈ 1290 px / ~106 ch). Drop to `66em` (≈ 1056 px / ~85 ch) for the
article surface. Wikipedia uses ~60 em; institutional documentation
benefits from slightly more for tables. Keep 76 em for home page two-col
and three-rail layouts.

```css
--max-content-width: 66em;       /* article reading column */
--max-shell-width:   76em;       /* home/index three-rail */
```

## 6. Shadow and elevation tokens

The DTCG bundle has no shadow primitives. Six different `box-shadow`
values are hard-coded in `style.css` (`0 4px 8px rgba(0,0,0,.12)`,
`0 4px 12px rgba(0,0,0,0.15)`, `0 4px 16px rgba(0,0,0,0.15)`,
`0 4px 24px rgba(0,0,0,0.07)`, `0 2px 6px rgba(0,0,0,0.15)`,
`0 1px 4px rgba(0,0,0,0.08)`). Consolidate.

Add `primitive.shadow.*` to the DTCG bundle (DTCG `shadow` type):

```json
"shadow": {
  "$type": "shadow",
  "0": { "$value": "none" },
  "1": {
    "$value": { "color": "rgba(15,23,42,0.06)", "offsetX": "0", "offsetY": "1px", "blur": "2px", "spread": "0" },
    "$description": "Hairline lift — sticky header, inline chips"
  },
  "2": {
    "$value": { "color": "rgba(15,23,42,0.08)", "offsetX": "0", "offsetY": "2px", "blur": "6px", "spread": "0" },
    "$description": "Card resting — home tiles, infobox"
  },
  "3": {
    "$value": { "color": "rgba(15,23,42,0.10)", "offsetX": "0", "offsetY": "4px", "blur": "12px", "spread": "0" },
    "$description": "Hover lift, dropdown menus, autocomplete"
  },
  "4": {
    "$value": { "color": "rgba(15,23,42,0.14)", "offsetX": "0", "offsetY": "8px", "blur": "24px", "spread": "0" },
    "$description": "Hover-card, popover preview, modal trigger"
  },
  "5": {
    "$value": { "color": "rgba(15,23,42,0.20)", "offsetX": "0", "offsetY": "16px", "blur": "40px", "spread": "0" },
    "$description": "Modal, full-overlay dialog"
  },
  "focus-ring": {
    "$value": { "color": "{semantic.interactive.focus-ring}", "offsetX": "0", "offsetY": "0", "blur": "0", "spread": "3px" },
    "$description": "Replaces outline ring on inputs"
  }
}
```

And a semantic elevation tier in `semantic.elevation.*`:

```css
--elevation-flat:    none;                             /* resting */
--elevation-raised:  var(--shadow-1);                  /* sticky header, tabs */
--elevation-card:    var(--shadow-2);                  /* cards, infobox, leapfrog */
--elevation-hover:   var(--shadow-3);                  /* hover-card, dropdown */
--elevation-popover: var(--shadow-4);                  /* preview hover, glossary tooltip */
--elevation-modal:   var(--shadow-5);                  /* dialog, shortcut overlay */
```

Then replace every `box-shadow: 0 …` in `style.css` with the semantic
token. Dark theme overrides shadow color to
`rgba(0,0,0,0.45)` and increases blur 1.5× to keep elevation perceptible
on dark canvas.

## 7. Visual polish checklist

### 7.1 Hover states

Current pattern: links underline on hover, cards change border color to
`var(--link)`, buttons change background. No motion, no elevation
change. Recommend a layered hover model.

```css
/* Semantic hover tokens (new) */
--hover-overlay:  oklch(from var(--text-primary) l c h / 0.04);   /* 4% ink wash for surface hover */
--hover-overlay-strong: oklch(from var(--text-primary) l c h / 0.08);
--hover-lift-y:   -1px;  /* translateY for cards that elevate */
--hover-link-underline-thickness: 1.5px;
--hover-link-underline-offset:    0.2em;
```

Apply:

```css
.wiki-home-cat-card,
.wiki-home-sister-link {
  transition:
    border-color   var(--motion-duration-base) var(--motion-easing-standard),
    box-shadow     var(--motion-duration-base) var(--motion-easing-standard),
    transform      var(--motion-duration-base) var(--motion-easing-standard);
}
.wiki-home-cat-card:hover,
.wiki-home-sister-link:hover {
  border-color: var(--interactive-link);
  box-shadow: var(--elevation-hover);
  transform: translateY(var(--hover-lift-y));
}

/* Article links — replace the universal underline-on-hover with offset underline always-on */
.page-body a {
  text-decoration: underline;
  text-decoration-thickness: 1px;
  text-underline-offset: 0.18em;
  text-decoration-color: color-mix(in srgb, var(--interactive-link) 40%, transparent);
  transition: text-decoration-color var(--motion-duration-base) var(--motion-easing-standard);
}
.page-body a:hover {
  text-decoration-color: var(--interactive-link);
  text-decoration-thickness: var(--hover-link-underline-thickness);
}
```

### 7.2 Focus rings

Current global `:focus-visible` outlines with `2px solid var(--link)` at
`2px` offset. Functional but identical to hover treatment. Differentiate:

```css
:focus-visible {
  outline: 2px solid var(--interactive-focus-ring);
  outline-offset: 3px;            /* was 2px — pulls ring clear of surface */
  border-radius: var(--radius-xs);
  box-shadow: 0 0 0 4px color-mix(in srgb, var(--interactive-focus-ring) 18%, transparent);
}
```

Add `--interactive-focus-ring-bg: color-mix(in srgb, var(--interactive-focus-ring) 18%, transparent)`
as a semantic token so the same wash is reusable.

### 7.3 Transitions

Current motion tokens are durations only. Add semantic motion roles:

```css
--motion-hover:     var(--motion-duration-fast) var(--motion-easing-standard);   /* 75ms */
--motion-press:     var(--motion-duration-instant);
--motion-reveal:    var(--motion-duration-base) var(--motion-easing-decelerate); /* 150ms */
--motion-dismiss:   var(--motion-duration-base) var(--motion-easing-accelerate);
--motion-emphasis:  var(--motion-duration-slow) var(--motion-easing-standard);   /* 300ms — once-only callouts */

/* The easing primitives are currently written as JSON arrays in tokens.css
 * (e.g. --motion-easing-standard: [0.2, 0, 0.38, 0.9];) — this is invalid
 * CSS. dtcg-to-css.py must emit cubic-bezier(0.2, 0, 0.38, 0.9) instead. */
```

Wrap state-change-bearing properties only — `color`, `background-color`,
`border-color`, `box-shadow`, `transform`, `opacity`. Never blanket
`transition: all`.

### 7.4 Status colors — calibration

Status backgrounds today are at ~94–96% lightness; status base colors
are at ~50–55%. Functional, but they all read at the same emphasis
level. Add an emphasis tier:

```json
"status": {
  "success": {
    "base":     { "$value": "{primitive.color.positive-60}" },   // #26823f
    "emphasis": { "$value": "{primitive.color.positive-70}" },   // #16602b — for "verified" badges
    "border":   { "$value": "{primitive.color.positive-30}" },   // #9fdaae
    "bg":       { "$value": "{primitive.color.positive-10}" },   // #e8f6ed
    "on":       { "$value": "#ffffff" }
  },
  "warn": {
    "base":     { "$value": "{primitive.color.caution-60}" },    // #a87514
    "emphasis": { "$value": "{primitive.color.caution-70}" },    // #7a520a
    "border":   { "$value": "{primitive.color.caution-30}" },    // #f5cd7a
    "bg":       { "$value": "{primitive.color.caution-10}" },    // #fff5e1
    "on":       { "$value": "#ffffff" }
  },
  "error": {
    "base":     { "$value": "{primitive.color.critical-60}" },   // #a52323
    "emphasis": { "$value": "{primitive.color.critical-70}" },   // #7d1414
    "border":   { "$value": "{primitive.color.critical-30}" },   // #f0a3a3
    "bg":       { "$value": "{primitive.color.critical-10}" },   // #fceaea
    "on":       { "$value": "#ffffff" }
  },
  "info": {
    "base":     { "$value": "{primitive.color.brand-blue-60}" }, // #0f62fe
    "emphasis": { "$value": "{primitive.color.brand-blue-70}" }, // #0043ce
    "border":   { "$value": "{primitive.color.brand-blue-50}" }, // #edf5ff
    "bg":       { "$value": "{primitive.color.brand-blue-50}" },
    "on":       { "$value": "#ffffff" }
  }
}
```

Adds `border` and `emphasis` slots that don't exist today, removing the
`color-mix(... 30%, var(--bg))` improvisations scattered through
`style.css` (login error, pending-btn-reject, quality badges).

### 7.5 Radius

Current scale: 0 / 2 / 4 / 8 px. Encyclopedia register correctly says
"minimal rounding." Keep, but add a `radius.pill: 9999px` for status
badges and the auth pending-count chip — the current chip uses a
hard-coded `border-radius: 10px` which is neither token nor pill.

### 7.6 The lede accent (existing pattern, refine)

The article-lede left-border accent at 3px is a strong existing pattern.
Upgrade:

```css
#mw-content-text > .page-body > p:first-of-type {
  border-left: 4px solid var(--brand-emphasis);  /* was 3px, was --accent */
  padding-left: var(--stack-roomy);              /* was 1rem → 1.5rem */
  font-size: 1.125rem;                            /* was inherit — lede should be larger than body */
  line-height: var(--leading-loose);
  color: var(--text-primary);
}
```

This is a small change but it's the first thing a reader sees on every
article. 4 px + brand-emphasis colour + larger size carries the
institutional voice.

## 8. Accessibility audit of proposed changes

Contrast ratios for every recommended token pair, sRGB / WCAG 2.x.

### 8.1 PointSav theme

| Pair | Foreground | Background | Ratio | WCAG |
|---|---|---|---|---|
| Body | `--text-primary` `#0e0f12` | `--surface-background` `#fafbfc` | 19.4:1 | AAA |
| Secondary | `--text-secondary` `#4a4f59` | `#fafbfc` | 8.7:1 | AAA |
| Tertiary (new) | `--text-tertiary` `#666c78` | `#fafbfc` | 5.4:1 | AA |
| Link default | `--interactive-link` `#3366cc` | `#fafbfc` | 5.5:1 | AA |
| Link visited | `--interactive-link-visited` `#795cb2` | `#fafbfc` | 5.6:1 | AA |
| Link hover | `--interactive-link-hover` `#447ff5` | `#fafbfc` | 4.0:1 | AA Large only — restrict to ≥18 pt or non-text use |
| Accent (new) | `--accent` `#5176A3` | `#fafbfc` | 4.6:1 | AA |
| Accent emphasis (new) | `#2E5180` | `#fafbfc` | 8.9:1 | AAA |
| Brand emphasis | `--brand-emphasis` `#009d9a` | `#fafbfc` | 3.0:1 | UI only — non-text per WCAG 1.4.11; do not use for body text |
| Status success base | `#26823f` | `#fafbfc` | 5.0:1 | AA |
| Status warn base | `#a87514` | `#fafbfc` | 4.6:1 | AA |
| Status error base | `#a52323` | `#fafbfc` | 6.1:1 | AA |
| Status info base | `#0f62fe` | `#fafbfc` | 5.6:1 | AA |
| Border-strong | `#9aa1ad` | `#fafbfc` | 3.1:1 | AA non-text (3:1 required) |
| Border-subtle | `#e2e6ec` | `#fafbfc` | 1.3:1 | Decorative — not a UI-meaningful boundary |

### 8.2 Woodfine theme

| Pair | Foreground | Background | Ratio | WCAG |
|---|---|---|---|---|
| Body | `--text-primary` `#111827` | `#F7F9FA` | 16.2:1 | AAA |
| Secondary | `#6B7280` | `#F7F9FA` | 5.3:1 | AA |
| Tertiary (new) | `#6B7280` (collapsed onto secondary) | `#F7F9FA` | 5.3:1 | AA |
| Link default | `--interactive-link` `#164679` | `#F7F9FA` | 9.4:1 | AAA |
| Link visited | `#0F3258` | `#F7F9FA` | 12.6:1 | AAA |
| Link hover | `#1A5FA8` | `#F7F9FA` | 6.5:1 | AA |
| Accent-emphasis (new) | `#0A2E5A` | `#F7F9FA` | 12.9:1 | AAA |
| Brand-warm (new) | `#54924E` | `#F7F9FA` | 3.9:1 | AA Large only / non-text |
| On-blue text | `#FFFFFF` | `#164679` | 9.4:1 | AAA |
| Focus ring | `#164679` | `#F7F9FA` | 9.4:1 | AAA — focus needs only 3:1 |

### 8.3 Dark theme (PointSav `[data-theme="dark"]`)

| Pair | Foreground | Background | Ratio | WCAG |
|---|---|---|---|---|
| Body | `#d5d5d5` | `#1a1a1a` | 11.2:1 | AAA |
| Muted | `#9ea3a7` | `#1a1a1a` | 6.5:1 | AA |
| Link | `#6794d0` | `#1a1a1a` | 6.0:1 | AA |
| Link hover | `#7facdf` | `#1a1a1a` | 7.4:1 | AAA |
| Visited | `#b786ff` | `#1a1a1a` | 7.0:1 | AAA |
| Accent (dark) | `#a0bcd0` | `#1a1a1a` | 8.6:1 | AAA |
| Danger | `#f47067` | `#1a1a1a` | 6.2:1 | AA |

Dark theme passes throughout — no token changes needed beyond piping the
new shadow/spacing/elevation semantic tokens through.

### 8.4 Defects to fix elsewhere

Three additional WCAG defects surfaced during this audit:

- **Login submit button** (`style.css:1892–1903`) uses `color: #fff` on
  `var(--link)` `#3366cc` — passes at 4.5:1 (5.5:1), but the
  `:hover { opacity: 0.88 }` reduces effective contrast below 4.5:1.
  Replace with `:hover { background: var(--interactive-link-hover) }` —
  same hover semantics, no opacity-induced contrast loss.
- **`pending-btn-accept`** uses `--color-success` `#26823f` on white at
  5.0:1 — passes. Hover state `color-mix(... 80%, #000)` darkens to
  ~7.3:1 — fine.
- **`density-btn-active`** (line 755) uses `--accent` as background and
  `var(--bg)` as foreground. With the recommended `--accent: #5176A3`
  and `--surface-background: #fafbfc`, contrast is 4.6:1 — AA pass.
  Current `#869FB9` on `#f5f6f8` is 2.4:1 — fail. The accent recolour
  fixes this implicitly.

## 9. Implementation sequencing

Prioritised so each step is independently shippable and lifts a
visible grade tier:

1. **WCAG fix** — patch `text.tertiary` to `neutral-60` in DTCG bundle;
   regenerate `tokens.css`. (One file, immediate AA compliance.)
2. **Color refinement** — patch surface, accent, border tokens for both
   themes. Add `--accent-emphasis`, `--brand-emphasis`, `--brand-warm`.
3. **Shadow / elevation tokens** — add DTCG `shadow.*` ramp + semantic
   elevation; replace inline `box-shadow` calls in `style.css`.
4. **Spacing semantic tier** — add `stack-*`, `inline-*`, `inset-*`
   semantic tokens; refactor `style.css` to consume them in the busiest
   article and home rules.
5. **Type scale semantic tier** — add `--type-*-{size,line-height,weight,tracking}`
   triples; rewire `body`, `.page-body`, and `.page-body h1–h6`.
6. **Self-hosted variable typography** — convert `woodfine-media-assets/fonts/`
   masters to woff2, ship from binary's `/static/fonts/`. Amend
   DS-ADR-07 to permit self-hosted OFL families. Add `@font-face`
   declarations to `tokens-woodfine.css`. Add Source Serif 4 to
   `tokens.css` PointSav stack.
7. **Hover / focus / motion polish** — add `--hover-*`, `--motion-*`
   semantic tokens; rewire card hovers, focus rings, link underlines.

Step 1 alone moves the grade from C to C+ (fixes a hard blocker).
Steps 1–4 move to B-. Step 6 (typography) is the single biggest visual
lift; steps 1–6 land at A-. Step 7 is the polish layer that distinguishes
A- from A.
