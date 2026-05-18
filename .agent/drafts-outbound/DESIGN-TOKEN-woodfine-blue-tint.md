---
schema: foundry-draft-v1
artifact_type: DESIGN-TOKEN-CHANGE
draft_id: DESIGN-TOKEN-woodfine-blue-tint
status: staged
created: 2026-05-16
author: task@project-marketing
gateway: project-design
destination: customer/woodfine-media-assets/token-global-color.yaml
master_cosign_required: false
research_trail:
  source_file: sandbox/inputs/project-marketing/CONSTRUCTION_MCorp_2026-04-25_Chart_Venn_Diagram_JW2.html
  motivation: Token-alignment pass — Venn diagram chart pulled to registered palette. woodfine-blue-tint is the sole color in the chart that has no registered equivalent and cannot be approximated by an existing token without losing the blue-tint semantic for Venn bubble fills.
  prior_art: woodfine-blue (#164679) is registered; this is its surface/tint derivation at low opacity. Analogous pattern: woodfine-green (#54924E) + woodfine-green-bg (#ECFDF3); woodfine-amber (#B54708) + woodfine-amber-bg (#FFF8ED).
  conflicts: none — no existing token is close enough to cause ambiguity.
  alternatives_considered: woodfine-canvas (#F7F9FA) — too neutral/warm; woodfine-grey-light (#e9ecef) — no blue character. No viable substitute.
---

# DESIGN-TOKEN-CHANGE: woodfine-blue-tint

Add one color token to `customer/woodfine-media-assets/token-global-color.yaml`.

## Addition

```yaml
  woodfine-blue-tint: "#E8EFF7"        # Woodfine Blue surface tint — Venn/chart bubble fill
```

Insert after `woodfine-blue: "#164679"` in the core palette block (before the AEC semantic additions).

## Rationale

`#E8EFF7` is the light-tinted surface derivation of Woodfine Blue (`#164679`). It is used as the fill for the Signature Features (middle) Venn bubble in the CONSTRUCTION chart series. The pattern mirrors the existing tint pairs already in the token file:

| Base | Tint | Semantic |
|---|---|---|
| `woodfine-green #54924E` | `woodfine-green-bg #ECFDF3` | AEC pass/verified |
| `woodfine-amber #B54708` | `woodfine-amber-bg #FFF8ED` | AEC warning |
| `woodfine-blue #164679` | **`woodfine-blue-tint #E8EFF7`** ← new | Chart/Venn accent surface |

## Commit instructions (project-design / mcorp-administrator)

File: `customer/woodfine-media-assets/token-global-color.yaml`
Line to add (after `woodfine-blue: "#164679"`):
```
  woodfine-blue-tint: "#E8EFF7"        # Woodfine Blue surface tint — Venn/chart bubble fill
```
