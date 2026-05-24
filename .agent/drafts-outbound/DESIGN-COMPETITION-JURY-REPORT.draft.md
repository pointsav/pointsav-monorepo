---
artifact: design-research
status: draft
type: DESIGN-COMPETITION-JURY-REPORT
created: 2026-05-24
---

# Design Competition Jury Report — Wiki UI/UX

## Executive summary

No single entry wins outright. Design D (Brand Continuity) achieves the most
magazine-quality reading surface and the tightest integration with the ratified
home wireframe, but its article TOC is a floating right-rail pill that disappears
below 1280 px and its home view is editorial rather than encyclopaedic. Design A
(Stripe Precision) delivers the cleanest encyclopaedic structure, the most
complete dark mode, and the most faithful three-row chrome interpretation. The
recommendation is a hybrid: adopt Design D's token layer, two-column article
layout, editorial home hero, and footer structure; graft Design A's three-row
shell, category-band home layout, left-rail sticky TOC, and citation ribbon; and
borrow Design B's numbered TOC hierarchy and hatnote/breadcrumb article
preamble. Design C's audience-filter concept belongs in a future iteration, not
in v1.

---

## Scoring table

| Dimension | A | B | C | D | Max |
|---|---|---|---|---|---|
| 1. Brand authority (three-row header, correct tokens) | 9 | 7 | 6 | 10 | 10 |
| 2. Typography (variable fonts, scale coherence, Zilla Slab) | 8 | 8 | 6 | 9 | 10 |
| 3. Navigation clarity (<10 s article discovery) | 8 | 8 | 7 | 7 | 10 |
| 4. Home→wiki continuity | 7 | 6 | 5 | 10 | 10 |
| 5. Dark/light mode (WCAG AA, genuinely designed) | 9 | 7 | 7 | 8 | 10 |
| 6. Article reading surface (TOC, spacing, lead, citation) | 8 | 9 | 6 | 9 | 10 |
| 7. Mobile 480 px (header collapse, article readability) | 8 | 7 | 6 | 7 | 10 |
| 8. Brand parameterisation (one shell, three skins) | 8 | 6 | 7 | 9 | 10 |
| 9. 21st-century Wikipedia (clean reader mode, progressive disclosure) | 8 | 9 | 4 | 9 | 10 |
| **Total** | **73** | **67** | **54** | **78** | **90** |

---

## Per-design narrative

### Design A — Stripe Precision

Design A is the tightest engineering execution in the field. Every CSS variable is
named, semantic, and documented; the dark mode is genuinely recalibrated
(lifted brand blue, adjusted shadow opacities) rather than simply inverted. The
three-row shell maps precisely to the ratified pattern: utility stripe with
instance pill, brand row with wordmark and instance label, nav row with dominant
search and sister-site links. The article surface is clean for anonymous readers:
the status dot collapses to 6 px, edit pencils are hover-only on headings, and
the editor pill appears only for authenticated users. The citation ribbon is
correctly placed at article bottom and collapsed by default. Responsiveness is
thorough — the mobile-overflow FAB is a good substitute for the editor pill.
Where it loses points: the brand row uses a pseudo-mark rather than the actual
vector wordmark SVG from the ratified wireframe, so continuity from
home.woodfinegroup.com is partial rather than complete. The home view's hero
is understated compared to the editorial ambition the content deserves.
Typography is correct (Barlow Condensed, Nunito Sans, Zilla Slab all loaded),
but article body size is 16 px/1.7 — slightly cramped versus Design D's
19 px lead.

### Design B — Wikipedia Evolved

Design B is the most Wikipedia-faithful interpretation. Numbered TOC entries with
sub-levels (2.1, 2.2, 2.3), inline citation superscripts with bracket notation,
hatnote ribbons for bilingual cross-references, and a `<details>` element for
references are all correctly conceived. The edit pencil is shown as a text label
("Edit") appearing on section heading hover for any user — this needs to be
auth-gated. The "gH" / "gA" Wikipedia keyboard shortcuts and "/" search focus
are thoughtful additions. The sticky-header collapse behaviour (rows 1 and 2
animate away on scroll) is well-executed. Dark mode suffers from a purple-slate
palette (`#1A1A2E`) that diverges from the canonical `#09090B` dark canvas and
`#869FB9` PointSav steel blue. The wordmark is text-only at 44 px, which loses
brand equity on the brand row — no SVG rendering, no structural tie to the home
wireframe. The visited-link colour (`#5B2A86`) is novel but creates a purple
accent that is not in the brand token set.

### Design C — Enterprise Learn

Design C introduces audience segmentation (For auditors / For engineers / For
operators) as a first-class home page concept. This is a meaningful product
insight — the knowledge platform does serve distinct reader profiles — but the
execution places audience-filter buttons in the brand row, which disrupts the
three-row chrome pattern. The home view is a dashboard of titled sections by
audience rather than an encyclopaedic portal; it reads like a corporate intranet
and scores well for task navigation but poorly for reading-surface quality. The
article view uses a sidebar with a hierarchical tree nav plus a numbered TOC list
— two navigation structures in the same panel is redundant. The article meta
block (audience badges, "Applies to" pills, status pill) adds significant chrome
above the article lead, working against the 21st-century-Wikipedia requirement
of a clean reading surface. The citation accordion is the only well-executed
element borrowed directly from enterprise documentation conventions. Dark mode is
correct in tone but the canvas is `#0F172A` (Tailwind slate-900) rather than
`#09090B`. No Zilla Slab serif — body text at 15 px in Nunito Sans throughout
article, reducing reading-surface authority. Audience segmentation belongs in a
Phase 2 feature behind a filter toggle, not in the main chrome.

### Design D — Brand Continuity

Design D is the only entry that inherits the token layer verbatim from the
ratified home wireframe (`wireframe-home-header-v2c.html`): identical CSS
variable names (`--ds-*`, `--wf-*`), identical Oswald/Nunito Sans/Zilla Slab
stack, identical SVG wordmark, identical footer structure with city names, footnav,
copyright, and trademark in the canonical "variant C" ordering. The home view
employs a full-bleed editorial hero with a `clamp(48px, 6vw, 92px)` headline,
italic Zilla Slab emphasis, and a right-column data-stat aside — this is
magazine-grade and gives the wiki the same visual weight as the corporate home.
The article reading surface at 19 px/1.75 in Zilla Slab lead plus 17 px/1.7
body is the most comfortable in the field. The two-column layout at wide
viewports (`column-count: 2`) with a column rule is a sophisticated typographic
choice for long articles. The floating right-rail TOC (visible only at ≥1280 px)
is minimal and non-intrusive. The floating action bar for logged-in users is
well-proportioned. Where it loses points: the TOC is invisible below 1280 px
with no fallback — a significant gap for typical laptop widths (1280–1440). The
home view editorial grid shows three articles in a wide/mid/narrow column
layout that does not degrade gracefully to a single-column for the 30-article
catalogue. Category navigation is a horizontal pill strip rather than a browsable
panel, making it harder to build a mental model of the content structure.
Dark mode is correctly `#09090B` canvas.

---

## Footer/trademark finding

| Entry | Footer verdict |
|---|---|
| A | Incorrect — copyright line only ("© 2026 Woodfine Capital Projects Inc."), truncated footer, no trademark text at all. |
| B | Absent — no footer block rendered in the prototype. |
| C | Incorrect — copyright is condensed ("© 2026 Woodfine Capital Projects Inc. Content licensed source-by-source") and trademark text is absent. |
| D | Correct — copyright "© 2026 Woodfine Capital Projects Inc. All rights reserved." is present, followed by the full canonical trademark paragraph ("Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™, Totebox Orchestration™ and Totebox Archive™ are trademarks of Woodfine Capital Projects Inc. used in Canada, the United States, Latin America, and Europe.") with minor non-material variance from the canonical text (drops the Oxford comma before "and Europe" — acceptable). Only Design D passes. |

**Requirement for all implementations:** the footer must carry both lines verbatim:
1. `© 2026 Woodfine Capital Projects Inc. All rights reserved.`
2. The full trademark paragraph as specified in the brief.

---

## 21st-century Wikipedia verdict

**Design A:** Passes the reader-mode test. Anonymous visitors see a clean title,
lead, and body with no status banners and no editing chrome. Status is a 6 px
dot in the top-right corner. Edit pencils appear only on heading hover. The editor
pill is completely absent for anonymous users. Feels like a well-maintained
reference site, not a CMS.

**Design B:** Mostly passes. The edit pencil is visible on heading hover for all
users regardless of auth state (CSS rule `h2:hover .edit-pencil` applies always,
not only under `[data-auth="auth"]`). This is a minor but fixable bug. The
hatnote and bilingual notice add friendly Wikipedia-style context without CMS
clutter. The references block defaults to open (`<details open>`), which means
citations are visible by default — should default to collapsed for anonymous
readers.

**Design C:** Fails. The article meta block above the lead contains audience
badges, "Applies to" pills, and a "Featured" status pill — all rendered in full
colour in the default anonymous view. The overflow menu with Edit/Talk options
is only conditionally shown when logged in, which is correct, but the metadata
banner above the article itself creates the CMS-panel feeling the challenge
explicitly prohibits.

**Design D:** Passes cleanly. The article opens to title, chip (category label in
a small rounded pill), and lead paragraph. Status is a single gold dot in the
article meta line. The section-edit button is hidden until CSS `html:not([data-auth="anon"]) .article-body h2:hover` applies — meaning it only appears for users who are
not anonymous. The floating action bar is `display: none` by default and shown
only under `html[data-auth="user"] body.article-active .action-bar`. Feels like
reading a magazine article on a branded platform.

---

## Recommendation

**Hybrid: D shell + A/B article structure.** Build from Design D's token layer
and home editorial grid as the foundation, replace its right-rail floating TOC
with Design A's left-rail sticky TOC (collapsed to chip-nav on mobile), add
Design B's numbered TOC hierarchy and hatnote conventions, and apply Design A's
three-row chrome interpretation (with the SVG wordmark from D, not A's
pseudo-mark). Footer is Design D exactly.

---

## Hybrid specification

### Layout skeleton

```css
/* ── Inherited from wireframe-home-header-v2c.html (Design D) ── */
/* All --ds-* and --wf-* tokens carried verbatim. */

/* ── Editorial layer extensions ── */
:root {
  /* Reading surface */
  --ed-canvas:        var(--ds-paper-2);       /* #F7F9FA light, #09090B dark */
  --ed-surface:       var(--ds-paper-1);
  --ed-quote-rule:    var(--wf-blue);
  --ed-chip-bg:       var(--wf-blue-tint);
  --ed-chip-ink:      var(--wf-blue);
  --ed-toc-active:    var(--wf-blue);
  --ed-hover-tint:    rgba(22, 70, 121, 0.06);

  /* Article typography */
  --ed-reading-fs:    19px;
  --ed-reading-lh:    1.75;
  --ed-body-fs:       17px;
  --ed-body-lh:       1.7;

  /* TOC */
  --toc-width:        240px;

  /* Article column */
  --article-max:      720px;
  --shell-max:        1360px;

  /* Elevation */
  --ed-shadow-float:  0 8px 32px rgba(17,24,39,0.12), 0 2px 8px rgba(17,24,39,0.08);
  --ed-shadow-card:   0 1px 2px rgba(17,24,39,0.04);

  /* Status semantics */
  --status-featured:  #10B981;
  --status-stub:      #F59E0B;
  --status-draft:     #6B7280;
}

html[data-theme="dark"] {
  /* Carries Design D's dark values: #09090B canvas, #5C8FCE brand-blue */
  --ed-canvas:   #09090B;
  --ed-surface:  #111317;
  --ed-shadow-float: 0 8px 32px rgba(0,0,0,0.6), 0 2px 8px rgba(0,0,0,0.4);
}
```

**Home page layout:**
```
┌───────────────────────────────────────────────────┐
│ HERO (from D): 1.2fr / 1fr grid; large editorial  │
│ headline; right aside with fact numbers            │
├───────────────────────────────────────────────────┤
│ EDITORIAL GRID (from D): wide / mid / narrow cols  │
│ with eyebrow, kicker, card-title, card-lede        │
├───────────────────────────────────────────────────┤
│ CATEGORY BANDS (from A): vertical list not pills;  │
│ icon + name + desc + article count                 │
├───────────────────────────────────────────────────┤
│ STATS BAR (from D): centered, small caps           │
└───────────────────────────────────────────────────┘
```

**Article page layout:**
```
┌──────────────────────────────────────────────────────┐
│ BREADCRUMB (from B): Home › Category › Article        │
├──────────┬───────────────────────────────────────────┤
│ TOC      │ ARTICLE BODY                               │
│ 240px    │ max-width 720px                            │
│ sticky   │                                            │
│ left rail│ chip · title · meta line · status dot      │
│ (from A) │ lead (Zilla Slab 19px/1.75) (from D)       │
│          │ sections with section-edit on hover        │
│ numbered │ (from D auth gate: not-anon only)          │
│ (from B) │ two-col body at ≥1024px (from D)           │
│          │ citation ribbon collapsed (from A)         │
├──────────┴───────────────────────────────────────────┤
│ FLOATING ACTION BAR (from D): visible only logged-in  │
│ primary "Edit", History, overflow (Talk buried here)  │
└──────────────────────────────────────────────────────┘
```

### Token list

The following tokens should be extracted into `dtcg-bundle.json`:

```json
{
  "$schema": "https://tr.designtokens.org/format/",
  "wf": {
    "blue":       { "$value": "#164679", "$type": "color", "$description": "Woodfine primary — light mode" },
    "blue-dark":  { "$value": "#5C8FCE", "$type": "color", "$description": "Woodfine primary lifted — dark mode" },
    "blue-on":    { "$value": "#FFFFFF", "$type": "color" },
    "blue-tint":  { "$value": "#E8EFF7", "$type": "color" },
    "gold":       { "$value": "#EAB308", "$type": "color", "$description": "Accent — use sparingly" }
  },
  "ps": {
    "steel":      { "$value": "#869FB9", "$type": "color", "$description": "PointSav primary" }
  },
  "canvas": {
    "light":      { "$value": "#F7F9FA", "$type": "color" },
    "dark":       { "$value": "#09090B", "$type": "color" }
  },
  "surface": {
    "light-1":    { "$value": "#FFFFFF", "$type": "color" },
    "light-2":    { "$value": "#F7F9FA", "$type": "color" },
    "light-3":    { "$value": "#E6E7E8", "$type": "color" },
    "dark-1":     { "$value": "#111317", "$type": "color" },
    "dark-2":     { "$value": "#09090B", "$type": "color" },
    "dark-3":     { "$value": "#1C1F26", "$type": "color" }
  },
  "ink": {
    "1": { "$value": "#111827", "$type": "color" },
    "2": { "$value": "#374151", "$type": "color" },
    "3": { "$value": "#6B7280", "$type": "color" },
    "4": { "$value": "#9CA3AF", "$type": "color" }
  },
  "font": {
    "display":  { "$value": "\"Barlow Condensed\", \"Trade Gothic LT Std\", Arial, sans-serif", "$type": "fontFamily" },
    "body":     { "$value": "\"Nunito Sans\", -apple-system, \"Segoe UI\", sans-serif", "$type": "fontFamily" },
    "serif":    { "$value": "\"Zilla Slab\", Georgia, serif", "$type": "fontFamily" }
  },
  "reading": {
    "lead-size":   { "$value": "19px", "$type": "dimension" },
    "lead-lh":     { "$value": "1.75", "$type": "number" },
    "body-size":   { "$value": "17px", "$type": "dimension" },
    "body-lh":     { "$value": "1.70", "$type": "number" },
    "article-max": { "$value": "720px", "$type": "dimension" },
    "toc-width":   { "$value": "240px", "$type": "dimension" }
  },
  "status": {
    "featured":  { "$value": "#10B981", "$type": "color" },
    "stub":      { "$value": "#F59E0B", "$type": "color" },
    "draft":     { "$value": "#6B7280", "$type": "color" }
  }
}
```

### Component decisions

| Component | Source | Notes |
|---|---|---|
| Shell header (three-row structure) | Design A | Precise layout; correct semantic markup with `<header role="banner">` |
| SVG wordmark | Design D / wireframe-home-header-v2c | Verbatim SVG from ratified wireframe |
| Utility row: instance pill + language + theme + auth | Design A | Pill style + button semantics |
| Brand row: wordmark centred | Both A and D | D's SVG + A's semantic markup |
| Nav row: left links + search + right sister-site links | Design A | Grid layout; prominent search |
| Search: pill shape, ⌘K shortcut | Design A | Rounded search, kbd shortcut hint |
| Home hero: editorial, large-type, fact aside | Design D | `clamp()` heading, Zilla Slab italic em, data stats aside |
| Home content grid: editorial top + category bands below | D hero + A bands | D's editorial grid for featured content; A's category-band list for browse |
| Category display | Design A (bands, not pills) | Icon + title + description + count; vertically scannable |
| Article TOC: left-rail sticky, numbered | A structure + B numbering | 240 px sticky left rail; numbered items from B; chip-pill on mobile from A |
| Article breadcrumb | Design B | Home › Category › Article; small-caps Barlow Condensed |
| Article preamble: chip + title + meta dot + lead | Design D | chip (category), h1 (editorial weight), gold dot status, Zilla Slab lead |
| Section headings | Design D | Barlow Condensed 30 px; blue left-rule `::before` at ≥1024 px |
| Section edit button | Design D | CSS `html:not([data-auth="anon"]) h2:hover` — never visible to anonymous |
| Body text: two-column at ≥1024 px | Design D | `column-count: 2; column-rule: 1px solid var(--ds-rule-hairline)` |
| Citation: collapsed ribbon at bottom | Design A | `aria-expanded="false"` default; no inline citations for anonymous readers |
| Inline citations (logged-in/contributor view) | Design B | `<sup>[n]</sup>` superscript bracket notation |
| Hatnote / bilingual ribbon | Design B | Italic serif, left-indent, with link to Spanish article |
| Editor floating action bar | Design D | Bottom-centre pill; primary "Edit" button; overflow for Talk/Discuss |
| Mobile: editor tools | Design A | `display: none !important` for editor pill ≤767 px; "..." FAB instead |
| Dark mode | Design A (values) | Canonical `#09090B` canvas; D's `#5C8FCE` lifted brand-blue |
| Footer structure | Design D | Cities line / footnav / copyright / trademark (variant C, all four present) |
| Footer trademark text | Design D | Full canonical text; correct two-line structure |

---

## Implementation path

1. **Start from wireframe-home-header-v2c.html** as the CSS token base. Copy
   `--ds-*` and `--wf-*` blocks verbatim. Do not rename tokens.

2. **Add editorial layer tokens** (`--ed-*`) as defined in the token list above.
   Append them after the base tokens in a clearly labelled section.

3. **Add dark mode overrides** in `html[data-theme="dark"]`. Use Design D's
   values for `--ds-paper-*`, `--ds-ink-*`, `--ds-rule-*`, and `--wf-blue`.
   Use Design A's values for shadow opacities.

4. **Build the three-row shell** following Design A's HTML structure and CSS class
   names (`shell-utility`, `shell-brand`, `shell-nav`), substituting Design D's
   SVG wordmark into the brand row.

5. **Build the home view** in two zones:
   - Zone 1 (hero + editorial cards): Design D's `.hero` and `.editorial-grid`
     CSS and HTML, adjusted to a 1360 px max-width.
   - Zone 2 (category browse): Design A's `.category-bands` component; place
     below the editorial grid at full width.
   - Zone 3 (stats bar): Design D's `.stats-bar`.

6. **Build the article view** with:
   - `.article-shell` as a two-column grid: `var(--toc-width) minmax(0, 1fr)`
   - Left TOC rail: Design A's structure; add Design B's `<span class="toc-num">`
     numbering inside each `<a>` element.
   - Article body: Design D's `.article-header`, `.article-lede`, `.article-body`
     CSS. Apply Design D's two-column layout class to body sections.
   - Section edit pencil: Design D's CSS selector
     `html:not([data-auth="anon"]) .article-body h2:hover .section-edit`.
   - Citation ribbon: Design A's collapsed-by-default pattern at article bottom.

7. **Build the floating action bar** from Design D exactly. Auth gate with
   `html[data-auth="user"] body.article-active .action-bar { display: flex; }`.
   Talk/Discussion must stay in the overflow menu, never on the reading surface.

8. **Build the footer** from Design D exactly. Verify trademark text matches the
   canonical wording in the brief line-for-line before shipping.

9. **Responsive breakpoints**: apply Design A's breakpoints (`1023 px`, `767 px`)
   with Design D's padding-reduction pattern. At 767 px:
   - Collapse TOC to chip-pill strip (Design A's approach).
   - Hide editor pill; show "..." FAB (Design A).
   - Stack nav row to single column (Design D's approach for ≤768 px).

10. **Brand parameterisation** (three skins): implement a `data-instance`
    attribute on `<html>` with values `projects`, `documentation`, `corporate`.
    Override only `--wf-blue`, `--wf-blue-tint`, `--wf-blue-on`, and the
    instance pill label. PointSav/documentation skin: `--wf-blue: #869FB9`
    (steel), dark-first canvas (`--ed-canvas: #09090B` by default). Woodfine
    Projects and Corporate: light-first canvas. Token override should be fewer
    than 8 lines per skin.

11. **Verify WCAG AA** on all four colour combinations before closing:
    `#164679` on `#FFFFFF` (≥4.5:1), `#5C8FCE` on `#09090B` (verify, may need
    lift to `#6B9FD4`), `#374151` on `#F7F9FA` (body text), accent `#EAB308`
    only used on dark backgrounds or icon-only contexts without text.

12. **Keyboard shortcuts to implement**: `/` focuses search (Design B); `⌘K`
    also focuses search (Design A); `g h` goes home, `g a` goes to article
    (Design B Wikipedia-style).
