---
schema: foundry-draft-v1
artifact_type: DESIGN-TOKEN-CHANGE
draft_id: DESIGN-TOKEN-CHANGE-woodfine-yellow-magenta
status: staged
created: 2026-06-06
author: totebox@project-orgcharts
gateway: project-design
destination: woodfine-media-assets/token-global-color.yaml
target_path: token-global-color.yaml
master_cosign_required: false
research_done_count: 2
research_suggested_count: 0
open_questions_count: 0
research_provenance: direct-observation
research_inline: true
notes_for_designer: |
  Additive only — insert after woodfine-gold-tint in the chart palette section.
  Yellow value matches the 2026-06-06 update applied to all 9 production charts (commit dabe5000).
  Magenta is a legacy color for pre-reorganization Bencal Corporate entities — still in use
  on Bencal Organization charts (JW-series). No master cosign required (brand-specific customer repo).
---

# DESIGN-TOKEN-CHANGE — woodfine-media-assets: yellow + magenta tokens

## Change summary

Adds four entries to the `colors:` block in `token-global-color.yaml` — yellow and magenta border/surface pairs. These complete the Woodfine org chart palette in the YAML token file.

## YAML patch

In `token-global-color.yaml`, locate the chart palette section (currently ends with `woodfine-gold-tint`). Insert immediately after:

```yaml
  woodfine-yellow: "#EAB308"         # Org-chart fund vehicle / LP — dashed pill connector color; updated 2026-06-06 from #F57F17
  woodfine-yellow-tint: "#FFFDE7"    # Soft yellow surface for fund vehicle nodes
  woodfine-magenta: "#9F1853"        # Legacy Bencal Corporate entity — token-magenta nodes
  woodfine-magenta-tint: "#FFD6E8"   # Soft magenta surface for legacy corporate nodes
```

## Context

- `woodfine-yellow` replaces the prior amber `#F57F17` — perceptual distance from orange `#F15F22` was insufficient at small sizes. `#EAB308` (hue ~45°) is clearly distinguishable from orange (hue ~15°).
- `woodfine-magenta` covers Bencal Corporation pre-reorganization charts only. It is not used in current WCP structure charts. It is retained for historical accuracy when rendering legacy diagrams.
- Both colors are currently hardcoded in 9 production HTML chart files. This token registration enables CSS custom property generation and future chart templates to consume canonical values.

## No CSS changes in this artifact

CSS custom property additions (both `theme-woodfine.css` files) are in a separate artifact: `DESIGN-TOKEN-CHANGE-woodfine-chart-css`.
