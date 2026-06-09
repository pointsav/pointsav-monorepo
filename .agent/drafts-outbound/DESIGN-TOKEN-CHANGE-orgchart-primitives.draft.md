---
schema: foundry-draft-v1
artifact_type: DESIGN-TOKEN-CHANGE
draft_id: DESIGN-TOKEN-CHANGE-orgchart-primitives
status: staged
created: 2026-06-06
author: totebox@project-orgcharts
gateway: project-design
destination: pointsav-design-system/tokens/dtcg-bundle.json
target_path: tokens/dtcg-bundle.json
master_cosign: "command@claude-code 2026-06-09T16:35:12Z"
master_cosign_required: true
research_done_count: 4
research_suggested_count: 1
open_questions_count: 0
research_provenance: direct-observation
research_inline: true
paired_research: DESIGN-RESEARCH-orgchart-token-system
notes_for_designer: |
  BLOCKED on master_cosign. Operator must add timestamp + identity to master_cosign
  field before this artifact can be committed to pointsav-design-system.
  Research file: dtcg-vault/research/orgchart-token-system.md (see paired draft).
  Adds two new groups: primitive.color.orgchart (18 values) and semantic.orgchart (8 roles).
---

# DESIGN-TOKEN-CHANGE — Org Chart Color Primitives + Semantic Namespace

## Change summary

Adds `primitive.color.orgchart` (18 hex primitives) and `semantic.orgchart` (8 entity-role semantic aliases) to `tokens/dtcg-bundle.json`. New `orgchart.*` semantic namespace is parallel to the existing `wiki.*` namespace (ratified 2026-04-30).

## DTCG JSON patch

Insert into `dtcg-bundle.json` under `"primitive" > "color"`:

```json
"orgchart": {
  "$description": "Woodfine org-chart semantic palette — 9 entity-role colors × border/bg pair. Source: project-orgcharts 9-chart production set. Yellow updated 2026-06-06 from #F57F17.",
  "green":         { "$value": "#54924E", "$type": "color", "$description": "Corporate holding company border" },
  "green-bg":      { "$value": "#EEF6EC", "$type": "color", "$description": "Corporate holding company surface" },
  "blue":          { "$value": "#164679", "$type": "color", "$description": "Investment vehicle / investor unit border" },
  "blue-bg":       { "$value": "#E8EFF7", "$type": "color", "$description": "Investment vehicle surface" },
  "purple":        { "$value": "#7C468C", "$type": "color", "$description": "Broker-dealer / asset manager border" },
  "purple-bg":     { "$value": "#EEE6F1", "$type": "color", "$description": "Broker-dealer surface" },
  "orange":        { "$value": "#F15F22", "$type": "color", "$description": "Equity partner / named individual border" },
  "orange-bg":     { "$value": "#FDE8DD", "$type": "color", "$description": "Equity partner surface" },
  "grey":          { "$value": "#9CA3AF", "$type": "color", "$description": "Admin entity / titleco border" },
  "grey-bg":       { "$value": "#E6E7E8", "$type": "color", "$description": "Admin entity surface" },
  "grey-light-bg": { "$value": "#F7F9FA", "$type": "color", "$description": "Dashed placeholder / service provider surface" },
  "grey-dark":     { "$value": "#374151", "$type": "color", "$description": "Strong border variant (grey-dark token class)" },
  "yellow":        { "$value": "#EAB308", "$type": "color", "$description": "Fund vehicle / LP / limited partnership border — dashed pill. Updated 2026-06-06 from #F57F17 for perceptual distance from orange #F15F22." },
  "yellow-bg":     { "$value": "#FFFDE7", "$type": "color", "$description": "Fund vehicle surface" },
  "magenta":       { "$value": "#9F1853", "$type": "color", "$description": "Legacy corporate (Bencal pre-reorganization) border" },
  "magenta-bg":    { "$value": "#FFD6E8", "$type": "color", "$description": "Legacy corporate surface" },
  "teal":          { "$value": "#005D5D", "$type": "color", "$description": "Legacy asset company (Bencal Real Assets pre-reorganization) border" },
  "teal-bg":       { "$value": "#9EF0F0", "$type": "color", "$description": "Legacy asset company surface" }
}
```

Insert into `dtcg-bundle.json` under `"semantic"`:

```json
"orgchart": {
  "$description": "orgchart.* semantic namespace — entity-role colors for Woodfine corporate visualization. Parallel to wiki.* namespace (ratified 2026-04-30). Entity-role semantics must not be conflated with $support-* status semantics.",
  "corporate-holding": {
    "$description": "Primary group entity; top-level holding company",
    "border":  { "$type": "color", "$value": "{primitive.color.orgchart.green}" },
    "surface": { "$type": "color", "$value": "{primitive.color.orgchart.green-bg}" }
  },
  "investment-vehicle": {
    "$description": "Investment unit; investor box; capital receiver",
    "border":  { "$type": "color", "$value": "{primitive.color.orgchart.blue}" },
    "surface": { "$type": "color", "$value": "{primitive.color.orgchart.blue-bg}" }
  },
  "broker-dealer": {
    "$description": "Registered broker-dealer, asset manager, or regulatory intermediary",
    "border":  { "$type": "color", "$value": "{primitive.color.orgchart.purple}" },
    "surface": { "$type": "color", "$value": "{primitive.color.orgchart.purple-bg}" }
  },
  "equity-partner": {
    "$description": "Named equity partner, individual principal, or Bencal-group operating entity",
    "border":  { "$type": "color", "$value": "{primitive.color.orgchart.orange}" },
    "surface": { "$type": "color", "$value": "{primitive.color.orgchart.orange-bg}" }
  },
  "admin-entity": {
    "$description": "Administrative entity — titleco, support company, service vehicle",
    "border":  { "$type": "color", "$value": "{primitive.color.orgchart.grey}" },
    "surface": { "$type": "color", "$value": "{primitive.color.orgchart.grey-bg}" }
  },
  "fund-vehicle": {
    "$description": "Limited partnership, fund LP, fideicomiso — always rendered as dashed pill",
    "border":  { "$type": "color", "$value": "{primitive.color.orgchart.yellow}" },
    "surface": { "$type": "color", "$value": "{primitive.color.orgchart.yellow-bg}" }
  },
  "legacy-corporate": {
    "$description": "Pre-reorganization Bencal Corporation entity — legacy token-magenta nodes",
    "border":  { "$type": "color", "$value": "{primitive.color.orgchart.magenta}" },
    "surface": { "$type": "color", "$value": "{primitive.color.orgchart.magenta-bg}" }
  },
  "legacy-asset": {
    "$description": "Pre-reorganization Bencal Real Assets entity — legacy token-teal nodes",
    "border":  { "$type": "color", "$value": "{primitive.color.orgchart.teal}" },
    "surface": { "$type": "color", "$value": "{primitive.color.orgchart.teal-bg}" }
  }
}
```

## Downstream impact

- No existing tokens modified — additive only.
- `semantic.orgchart.*` aliases reference `primitive.color.orgchart.*` — no cross-namespace aliasing.
- No CSS variables generated automatically; CSS custom properties live in `woodfine-media-assets` (see DESIGN-TOKEN-CHANGE-woodfine-chart-css).
