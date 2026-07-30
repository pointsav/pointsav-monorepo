---
schema: foundry-draft-v1
artifact_type: DESIGN-TOKEN-CHANGE
draft_id: DESIGN-TOKEN-CHANGE-woodfine-chart-css
status: staged
created: 2026-06-06
author: totebox@project-orgcharts
gateway: project-design
destination: woodfine-media-assets/css/theme-woodfine.css
target_path_secondary: woodfine-media-assets/css/theme-woodfine-light.css
master_cosign_required: false
research_done_count: 3
research_suggested_count: 0
open_questions_count: 1
research_provenance: direct-observation
research_inline: true
notes_for_designer: |
  Apply identical block to BOTH theme-woodfine.css AND theme-woodfine-light.css.
  --wf-chart-* prefix avoids collision with --wf-safe/--wf-warning AEC semantic tokens
  which use the same base hues with different meanings (green=IDS pass vs green=holding co).
  Green and blue already have CSS vars in both files; included here with --wf-chart-* names
  for completeness so chart authors have a single namespace to reference.
---

# DESIGN-TOKEN-CHANGE — woodfine-media-assets: org chart CSS custom properties

## Change summary

Adds a new `/* Org-chart palette */` block to `:root` in both `theme-woodfine.css` and `theme-woodfine-light.css`. Provides CSS custom properties for all 9 org chart colors (border + surface) plus all layout spacer dimensions and typography tokens.

## CSS patch — add to `:root` in both theme files

```css
  /* ── Org-chart palette — all 9 entity-role colors ────────────────── */
  --wf-chart-green: #54924E;          /* corporate holding company */
  --wf-chart-green-tint: #EEF6EC;
  --wf-chart-blue: #164679;           /* investment vehicle / investor unit */
  --wf-chart-blue-tint: #E8EFF7;
  --wf-chart-purple: #7C468C;         /* broker-dealer / asset manager */
  --wf-chart-purple-tint: #EEE6F1;
  --wf-chart-orange: #F15F22;         /* equity partner / named individual */
  --wf-chart-orange-tint: #FDE8DD;
  --wf-chart-grey: #9CA3AF;           /* admin entity / titleco */
  --wf-chart-grey-tint: #E6E7E8;
  --wf-chart-grey-light: #F7F9FA;     /* dashed placeholder / service provider */
  --wf-chart-grey-dark: #374151;      /* strong border variant */
  --wf-chart-yellow: #EAB308;         /* fund vehicle / LP (dashed pill) */
  --wf-chart-yellow-tint: #FFFDE7;
  --wf-chart-magenta: #9F1853;        /* legacy corporate (Bencal pre-reorg) */
  --wf-chart-magenta-tint: #FFD6E8;
  --wf-chart-teal: #005D5D;           /* legacy asset company (Bencal pre-reorg) */
  --wf-chart-teal-tint: #9EF0F0;

  /* ── Org-chart typography tokens ───────────────────────────────────── */
  --wf-chart-font: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
  --wf-chart-font-mono: ui-monospace, SFMono-Regular, monospace;
  --wf-chart-t-headline-size: 14px;     /* chart/page title */
  --wf-chart-t-headline-weight: 900;
  --wf-chart-t-headline-tracking: -0.02em;
  --wf-chart-t-title-size: 12px;        /* node entity name */
  --wf-chart-t-title-weight: 800;
  --wf-chart-t-title-tracking: -0.01em;
  --wf-chart-t-title-leading: 1.2;
  --wf-chart-t-kicker-size: 11px;       /* section label / zone header */
  --wf-chart-t-kicker-weight: 800;
  --wf-chart-t-kicker-tracking: 0.06em;
  --wf-chart-t-body-size: 10px;         /* node badge / alias / country */
  --wf-chart-t-body-weight: 700;
  --wf-chart-t-legal-size: 10px;        /* legal / disclaimer text */
  --wf-chart-t-legal-leading: 1.2;
  --wf-chart-t-preamble-size: 10px;     /* explanatory sub-text */
  --wf-chart-t-preamble-leading: 1.4;
  --wf-chart-t-code-size: 9px;          /* reference codes — monospace */

  /* ── Org-chart layout spacers ───────────────────────────────────────── */
  --wf-chart-canvas-w: 1056px;          /* US Letter landscape at 96dpi */
  --wf-chart-canvas-h: 816px;
  --wf-chart-node-w: 210px;             /* standard rectangle */
  --wf-chart-node-h: 110px;
  --wf-chart-node-h-tall: 145px;        /* board / managing entity */
  --wf-chart-node-h-short: 80px;        /* asset / subsidiary */
  --wf-chart-node-h-ellipse: 90px;      /* cross-border flow-through */
  --wf-chart-node-w-compact: 160px;     /* service provider / secondary */
  --wf-chart-node-h-compact: 60px;
  --wf-chart-pill-w: 250px;             /* fund vehicle / LP pill */
  --wf-chart-node-pad: 10px;            /* standard box padding */
  --wf-chart-pill-pad: 12px;            /* pill box padding */
  --wf-chart-border: 2px;               /* all border widths */
  --wf-chart-connector-stroke: 2px;     /* SVG path stroke */
  --wf-chart-connector-pad: 30px;       /* hit-detection proximity threshold */
```

## Open questions

1. Should `--wf-chart-green` and `--wf-chart-blue` alias to the existing `--wf-safe` and `--wf-accent` values? Currently they carry the same hex values but different semantic meanings (entity-role vs. AEC status/interactive). Aliasing would reduce maintenance but would document a semantic collision that doesn't currently exist.
