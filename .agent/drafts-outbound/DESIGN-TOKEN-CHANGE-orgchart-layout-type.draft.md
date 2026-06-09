---
schema: foundry-draft-v1
artifact_type: DESIGN-TOKEN-CHANGE
draft_id: DESIGN-TOKEN-CHANGE-orgchart-layout-type
status: staged
created: 2026-06-06
author: totebox@project-orgcharts
gateway: project-design
destination: pointsav-design-system/tokens/dtcg-bundle.json
target_path: tokens/dtcg-bundle.json
master_cosign: "command@claude-code 2026-06-09T16:35:12Z"
master_cosign_required: true
research_done_count: 3
research_suggested_count: 1
open_questions_count: 0
research_provenance: direct-observation
research_inline: true
notes_for_designer: |
  BLOCKED on master_cosign. Additive only — adds component.orgchart group.
  Values measured from production HTML charts (9 files, 15 JW iterations).
  Canvas dimensions are US Letter landscape at 96dpi (11" × 8.5" = 1056×816px).
  Box dimensions do NOT derive from Carbon $spacing-* scale — see carbon-token-map research.
---

# DESIGN-TOKEN-CHANGE — Org Chart Layout + Typography Tokens

## Change summary

Adds `component.orgchart` group to `tokens/dtcg-bundle.json` covering: canvas dimensions, all node size variants, connector geometry, and typography weight/size tokens. These formalize the constants that are currently hardcoded across 9 production HTML charts.

## DTCG JSON patch

Insert into `dtcg-bundle.json` under `"component"`:

```json
"orgchart": {
  "$description": "Layout, sizing, connector, and typography tokens for Woodfine org charts. Canvas = US Letter landscape at 96dpi. Box dimensions derived from content legibility at 9-12px type, not from Carbon $spacing-* scale.",

  "canvas": {
    "width":  { "$type": "dimension", "$value": "1056px", "$description": "US Letter landscape width at 96dpi (11 inches)" },
    "height": { "$type": "dimension", "$value": "816px",  "$description": "US Letter landscape height at 96dpi (8.5 inches)" }
  },

  "node": {
    "width":          { "$type": "dimension", "$value": "210px",  "$description": "Standard rectangle width — token-base" },
    "height":         { "$type": "dimension", "$value": "110px",  "$description": "Standard rectangle height" },
    "height-tall":    { "$type": "dimension", "$value": "145px",  "$description": "Tall variant — Board-level or managing entity" },
    "height-short":   { "$type": "dimension", "$value": "80px",   "$description": "Short variant — asset or subsidiary" },
    "height-ellipse": { "$type": "dimension", "$value": "90px",   "$description": "Ellipse height — cross-border flow-through" },
    "width-compact":  { "$type": "dimension", "$value": "160px",  "$description": "Compact variant width — service provider / secondary role" },
    "height-compact": { "$type": "dimension", "$value": "60px",   "$description": "Compact variant height" },
    "width-pill":     { "$type": "dimension", "$value": "250px",  "$description": "Pill width — fund vehicle / LP" },
    "padding":        { "$type": "dimension", "$value": "10px",   "$description": "Standard box internal padding" },
    "padding-pill":   { "$type": "dimension", "$value": "12px",   "$description": "Pill box internal padding" },
    "padding-compact":{ "$type": "dimension", "$value": "6px 8px","$description": "Compact box padding (vertical horizontal)" },
    "border-width":   { "$type": "dimension", "$value": "2px",    "$description": "All border widths — solid, dashed, and dotted variants" }
  },

  "connector": {
    "stroke-width":   { "$type": "dimension", "$value": "2px",  "$description": "SVG path stroke width" },
    "hit-padding":    { "$type": "dimension", "$value": "30px", "$description": "PAD constant — proximity threshold for connector click-detection in interactive editors" },
    "svg-offset-top": { "$type": "dimension", "$value": "1px",  "$description": "SVG overlay top offset relative to print-canvas" },
    "svg-offset-left":{ "$type": "dimension", "$value": "3px",  "$description": "SVG overlay left offset relative to print-canvas" },
    "marker-view-size":{ "$type": "number",   "$value": 10,     "$description": "SVG marker viewBox dimension (0 0 10 10)" },
    "marker-ref":     { "$type": "number",    "$value": 10,     "$description": "SVG marker refX — arrowhead tip at end of path" },
    "marker-width":   { "$type": "number",    "$value": 6,      "$description": "SVG markerWidth" },
    "marker-height":  { "$type": "number",    "$value": 6,      "$description": "SVG markerHeight" }
  },

  "type": {
    "headline": {
      "size":    { "$type": "dimension", "$value": "14px", "$description": "Chart / page title" },
      "weight":  { "$type": "fontWeight", "$value": 900 },
      "tracking":{ "$type": "dimension", "$value": "-0.02em" }
    },
    "title": {
      "size":    { "$type": "dimension", "$value": "12px", "$description": "Node entity name — primary label inside box" },
      "weight":  { "$type": "fontWeight", "$value": 800 },
      "tracking":{ "$type": "dimension", "$value": "-0.01em" },
      "leading": { "$type": "number", "$value": 1.2 }
    },
    "kicker": {
      "size":    { "$type": "dimension", "$value": "11px", "$description": "Section label / zone header / boundary text" },
      "weight":  { "$type": "fontWeight", "$value": 800 },
      "tracking":{ "$type": "dimension", "$value": "0.06em" }
    },
    "node-badge": {
      "size":    { "$type": "dimension", "$value": "10px", "$description": "Node number badge — .t-node inside box" },
      "weight":  { "$type": "fontWeight", "$value": 700 }
    },
    "alias": {
      "size":    { "$type": "dimension", "$value": "10px", "$description": "Short alias / alternate name — italic" },
      "weight":  { "$type": "fontWeight", "$value": 400 },
      "style":   { "$type": "string",     "$value": "italic" }
    },
    "preamble": {
      "size":    { "$type": "dimension", "$value": "10px", "$description": "Explanatory sub-text below title" },
      "weight":  { "$type": "fontWeight", "$value": 400 },
      "leading": { "$type": "number", "$value": 1.4 }
    },
    "legal": {
      "size":    { "$type": "dimension", "$value": "10px", "$description": "Legal / disclaimer text — footer area" },
      "weight":  { "$type": "fontWeight", "$value": 400 },
      "leading": { "$type": "number", "$value": 1.2 }
    },
    "code": {
      "size":    { "$type": "dimension", "$value": "9px", "$description": "Reference codes — monospace" },
      "weight":  { "$type": "fontWeight", "$value": 400 }
    }
  },

  "print": {
    "page-size":   { "$type": "string", "$value": "landscape",  "$description": "@page size value for US Letter landscape" },
    "page-margin": { "$type": "dimension", "$value": "0px",     "$description": "@page margin — zero; canvas fills the page" },
    "color-adjust":{ "$type": "string",  "$value": "exact",     "$description": "print-color-adjust and -webkit-print-color-adjust value — required for background fills in Chrome" }
  },

  "font": {
    "sans": { "$type": "fontFamily", "$value": "-apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Arial, sans-serif", "$description": "System font stack — no external import required; renders in print at all tested sizes" },
    "mono": { "$type": "fontFamily", "$value": "ui-monospace, SFMono-Regular, monospace", "$description": "System monospace — used for .t-code reference strings" }
  }
}
```

## Downstream impact

Additive only — no existing tokens modified. These constants are currently duplicated across 9 HTML chart files. Centralizing them enables future chart authoring to pull from a single source via CSS custom properties generated from this token group.
