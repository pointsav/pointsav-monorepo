---
schema: foundry-draft-v1
artifact_type: DESIGN-COMPONENT
draft_id: DESIGN-COMPONENT-orgchart-node
status: staged
created: 2026-06-06
author: totebox@project-orgcharts
gateway: project-design
destination: pointsav-design-system/components/orgchart-node/guide.md
master_cosign_required: false
component_metadata:
  component_name: orgchart-node
  component_kind: data-visualization
  carbon_baseline: none
  accessibility_targets: [wcag-2-2-aa, aria-group-labelling]
research_done_count: 4
research_suggested_count: 1
open_questions_count: 1
research_provenance: direct-observation
research_inline: true
notes_for_designer: |
  No Carbon baseline — this component has no Carbon equivalent (see carbon-token-map research).
  CSS extracted verbatim from 9 production charts. Verify against current chart files before publishing.
  ARIA recommendation is additive — charts currently use no ARIA attributes on nodes.
---

# Component Guide — Org Chart Node

An absolutely-positioned content box representing a corporate entity in an ownership hierarchy diagram. Renders at fixed pixel dimensions on a 1056×816px canvas (US Letter landscape). Three shape families: rectangle (operating entities), pill (fund vehicles), ellipse (cross-border flow-throughs).

## HTML Recipe

```html
<!-- Standard rectangle node -->
<div class="token-base token-green" style="left: 423px; top: 240px;"
     role="group" aria-label="Woodfine Capital Projects Inc. node 1">
  <div class="t-title">Woodfine Capital Projects Inc.</div>
  <div class="t-alias">WCP</div>
  <p class="t-node">1</p>
</div>

<!-- Tall variant -->
<div class="token-base token-blue tall" style="left: 776px; top: 464px;"
     role="group" aria-label="Investment Units node 12">
  <div class="t-title">Investment Units</div>
  <div class="t-alias">Investor pool</div>
  <p class="t-node">12</p>
</div>

<!-- Pill node (fund vehicle — always dashed) -->
<div class="token-yellow" style="left: 403px; top: 464px;"
     role="group" aria-label="Professional Centres LP node 17">
  <div class="t-title">Professional Centres LP</div>
  <p class="t-node">17</p>
</div>

<!-- Ellipse (cross-border flow-through — always dashed) -->
<div class="token-orange-ellipse-dashed" style="left: 565px; top: 500px;"
     role="group" aria-label="Cross-Border Flow-Through node 50">
  <div class="t-title">Cross-Border Flow-Through</div>
  <p class="t-node">50</p>
</div>
```

## CSS Recipe

```css
/* ── Layout foundation ── */
.token-base {
  position: absolute;
  display: flex;
  flex-direction: column;
  width: 210px;
  height: 110px;
  padding: 10px;
  box-sizing: border-box;
  text-align: center;
  justify-content: center;
  z-index: 2;
}

/* ── Size modifiers ── */
.token-base.tall  { height: 145px; }
.token-base.short { height: 80px; }
.token-base.compact { width: 160px; height: 60px; padding: 6px 8px; }

/* ── Color modifiers (apply to .token-base) ── */
.token-green  { background-color: #EEF6EC; border: 2px solid #54924E; border-radius: 0; }
.token-blue   { background-color: #E8EFF7; border: 2px solid #164679; border-radius: 0; }
.token-purple { background-color: #EEE6F1; border: 2px solid #7C468C; border-radius: 0; }
.token-orange { background-color: #FDE8DD; border: 2px solid #F15F22; border-radius: 0; }
.token-grey   { background-color: #E6E7E8; border: 2px solid #9CA3AF; border-radius: 0; }
.token-grey-solid { background-color: #E6E7E8; border: 2px solid #9CA3AF; border-radius: 4px; }
.token-gray-light { background-color: #E6E7E8; border: 2px solid #9CA3AF; border-radius: 12px; }
.token-gray-dark  { background-color: #E6E7E8; border: 2px solid #374151; border-radius: 0; }
.token-magenta    { background-color: #FFD6E8; border: 2px solid #9F1853; border-radius: 0; }
.token-teal       { background-color: #9EF0F0; border: 2px solid #005D5D; border-radius: 0; }

/* ── Pill shapes — fund vehicles (standalone, not .token-base) ── */
.token-yellow         { position: absolute; display: flex; flex-direction: column;
                        width: 250px; height: 110px; padding: 12px; box-sizing: border-box;
                        text-align: center; justify-content: center; z-index: 2;
                        background-color: #FFFDE7; border: 2px dashed #EAB308;
                        border-radius: 110px; }
.token-blue-dashed    { position: absolute; display: flex; flex-direction: column;
                        width: 250px; height: 110px; padding: 12px; box-sizing: border-box;
                        text-align: center; justify-content: center; z-index: 2;
                        background-color: #E8EFF7; border: 2px dashed #164679;
                        border-radius: 110px; }
.token-orange-pill-dashed { position: absolute; display: flex; flex-direction: column;
                            width: 250px; height: 110px; padding: 12px; box-sizing: border-box;
                            text-align: center; justify-content: center; z-index: 2;
                            background-color: #FDE8DD; border: 2px dashed #F15F22;
                            border-radius: 110px; }
.token-slate-pill-dashed  { position: absolute; display: flex; flex-direction: column;
                            width: 250px; height: 110px; padding: 12px; box-sizing: border-box;
                            text-align: center; justify-content: center; z-index: 2;
                            background-color: #F7F9FA; border: 2px dashed #9CA3AF;
                            border-radius: 110px; }

/* ── Ellipse shapes — cross-border flow-throughs ── */
.token-orange-ellipse-dashed  { position: absolute; display: flex; flex-direction: column;
                                width: 210px; height: 90px; padding: 10px; box-sizing: border-box;
                                text-align: center; justify-content: center; z-index: 2;
                                background-color: #FDE8DD; border: 2px dashed #F15F22;
                                border-radius: 110px; }
.token-purple-ellipse-dotted  { position: absolute; display: flex; flex-direction: column;
                                width: 210px; height: 90px; padding: 10px; box-sizing: border-box;
                                text-align: center; justify-content: center; z-index: 2;
                                background-color: #EEE6F1; border: 2px dotted #7C468C;
                                border-radius: 110px; }

/* ── Dashed placeholder rectangle ── */
.token-grey-dashed { position: absolute; display: flex; flex-direction: column;
                     width: 210px; height: 110px; padding: 10px; box-sizing: border-box;
                     text-align: center; justify-content: center; z-index: 2;
                     background-color: #F7F9FA; border: 2px dashed #9CA3AF;
                     border-radius: 0; }

/* ── Node typography ── */
.t-title   { font-size: 12px; font-weight: 800; letter-spacing: -0.01em; line-height: 1.2; margin: 0; }
.t-alias   { font-size: 10px; font-style: italic; color: #475569; margin: 2px 0 0; }
.t-node    { font-size: 10px; font-weight: 700; color: #6B7280; margin: 4px 0 0; }
.t-glyph   { font-size: 12px; font-weight: 700; margin: 0; }
.t-country { font-size: 10px; color: #64748B; margin: 2px 0 0; }
.t-legal   { font-size: 10px; line-height: 1.2; color: #374151; margin: 0; }
.t-code    { font-size: 9px; font-family: ui-monospace, SFMono-Regular, monospace; color: #6B7280; margin: 0; }
```

## Class composition table

| Visual appearance | Class pattern |
|---|---|
| Green rectangle (standard) | `token-base token-green` |
| Blue rectangle (tall) | `token-base token-blue tall` |
| Grey rectangle (short, rounded) | `token-base token-grey-solid short` |
| Orange rectangle | `token-base token-orange` |
| Purple rectangle | `token-base token-purple` |
| Yellow fund LP (pill, dashed) | `token-yellow` |
| Blue holding LP (pill, dashed) | `token-blue-dashed` |
| Orange SPV (pill, dashed) | `token-orange-pill-dashed` |
| Orange ICAV (ellipse, dashed) | `token-orange-ellipse-dashed` |
| Purple external SPV (ellipse, dotted) | `token-purple-ellipse-dotted` |
| Service provider (rect, dashed) | `token-grey-dashed` |
| Legacy Bencal Corp | `token-base token-magenta` |
| Legacy Bencal Assets | `token-base token-teal` |

## Accessibility

Org chart canvases are print-first documents. Apply `role="group"` and `aria-label="[Entity name] node [N]"` to each box div. The SVG connector layer uses `pointer-events: none` and requires no ARIA annotation.

## Open questions

1. Should `.token-grey-solid` (radius 4px) and `.token-gray-light` (radius 12px) be consolidated? Both represent administrative entities — the distinction currently reflects different subcategories (titleco vs. buildings) but is not systematically applied.
