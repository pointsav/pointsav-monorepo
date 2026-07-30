---
schema: foundry-draft-v1
artifact_type: DESIGN-COMPONENT
draft_id: DESIGN-COMPONENT-orgchart-canvas
status: staged
created: 2026-06-06
author: totebox@project-orgcharts
gateway: project-design
destination: pointsav-design-system/components/orgchart-canvas/guide.md
master_cosign_required: false
component_metadata:
  component_name: orgchart-canvas
  component_kind: layout
  carbon_baseline: none
  accessibility_targets: [wcag-2-2-aa, print-color-exact]
research_done_count: 4
research_suggested_count: 0
open_questions_count: 0
research_provenance: direct-observation
research_inline: true
notes_for_designer: |
  The 4-rule @media print block is the consensus pattern across all 9 production charts.
  Do not add transform:scale() — Chrome uses layout dimensions (not visual size) for page breaks.
  Canvas MUST be ≤816px in layout height for content to fit on one landscape page.
  The SVG render-quality block is mandatory for print sharpness.
---

# Component Guide — Org Chart Canvas

The fixed-dimension print canvas that hosts all org chart node boxes and SVG connector overlays. Sized exactly to US Letter landscape at 96dpi (11" × 8.5" = 1056×816px). Renders as a white card on screen; fills the page edge-to-edge when printed from Chrome.

## HTML Recipe

```html
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>Chart — Entity Name</title>
<style>
  /* Screen body — grey wrapper shows the canvas as a card */
  body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
    background: #E2E8F0;
    padding: 40px;
    margin: 0;
  }

  /* Canvas — fixed US Letter landscape dimensions */
  .print-canvas {
    position: relative;
    width: 1056px;
    height: 816px;
    background-color: #FFFFFF;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }

  /* ── PRINT MEDIA BLOCK — consensus 4-rule pattern ── */
  @media print {
    @page { size: landscape; margin: 0; }
    body  { padding: 0; margin: 0; background: #FFFFFF;
            -webkit-print-color-adjust: exact; print-color-adjust: exact; }
    .print-canvas { box-shadow: none; width: 1056px; height: 816px;
                    page-break-after: avoid; }
  }

  /* ── SVG render quality — critical for print sharpness ── */
  svg         { shape-rendering: geometricPrecision; }
  path        { shape-rendering: crispEdges; }
  marker path { shape-rendering: geometricPrecision; }
</style>
</head>
<body>
  <div class="print-canvas">
    <!-- Node boxes and SVG connector overlay go here -->
  </div>
</body>
</html>
```

## Signature stamp recipe

Place the signature stamp as an absolutely-positioned div inside `.print-canvas`. Standard position: `bottom: 30px; right: 40px`. If the canvas is close to the 816px height limit, place alongside the lowest box instead (see Bencal JW15 pattern: `top: 762px; right: 40px`).

```html
<div style="position: absolute; bottom: 30px; right: 40px;
            font-size: 10px; color: #374151; text-align: right; line-height: 1.4;">
  <strong>CONFIDENTIAL</strong><br>
  Woodfine Capital Projects Inc.<br>
  Prepared: June 8, 2026<br>
  Counter-party: [Name]
</div>
```

## Print rules — rationale and constraints

**Why `margin: 0` on `@page`?** The canvas is exactly 1056×816px — the full US Letter landscape page at 96dpi. Any page margin would push content off the edge. Zero margin means the white canvas fills the page exactly.

**Why no `transform: scale()`?** Chrome (and all Chromium browsers) computes page breaks using the element's **layout dimensions**, not its visual size after CSS transforms. A 880px canvas with `transform: scale(0.9)` still occupies 880px in the layout — the bottom 64px gets cut off onto page 2. The only reliable fix is to keep canvas layout height ≤ 816px.

**Why `print-color-adjust: exact`?** Without this, Chrome strips background-color fills from divs when printing. All token background colors would disappear, leaving only borders.

**Why `page-break-after: avoid`?** Prevents Chrome from inserting a blank second page after the canvas when the document has any trailing whitespace.

**Why `box-shadow: none` in print?** Print drivers render box-shadow as a filled area that clips against page edges. Removing it in print keeps the canvas clean.

## SVG render quality

The three SVG render hints are mandatory for print output:

```css
svg         { shape-rendering: geometricPrecision; }
path        { shape-rendering: crispEdges; }
marker path { shape-rendering: geometricPrecision; }
```

`crispEdges` on connector paths renders horizontal/vertical lines as crisp 1–2px strokes at any print resolution. `geometricPrecision` on markers and the SVG element enables sub-pixel arrowhead rendering. Without these, printed connectors appear blurry or anti-aliased.

## Height constraint

The canvas height of 816px is a hard layout constraint. Node boxes + connectors + signature stamp must all fit within this height. If authoring a chart that approaches the limit:
1. Move all elements up (decrement all `top` values equally)
2. Reduce the canvas height in both screen CSS and SVG `viewBox` to match the new content height
3. Move the signature stamp alongside the lowest box rather than at `bottom: 30px`
