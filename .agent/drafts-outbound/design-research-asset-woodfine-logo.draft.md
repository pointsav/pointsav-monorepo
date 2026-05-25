---
schema: foundry-draft-v1
draft_id: design-research-asset-woodfine-logo
language_protocol: DESIGN-RESEARCH
state: ready-for-sweep
target_path: vendor/pointsav-design-system/research/bim-woodfine-logo-asset.md
created: 2026-05-05T00:00:00Z
revised: 2026-05-06T17:45:00Z
author: task@project-bim
cites: [woodfine-media-assets]
research_done_count: 1
research_suggested_count: 0
open_questions_count: 1
research_provenance: |
  Originated: .agent/artifacts/ASSET-WOODFINE-LOGO-FIX.md (2026-05-05)
  Canonical source verified: customer/woodfine-media-assets/assets/logo/wf-logo_V1.svg
  Spec source: customer/woodfine-media-assets/tokens/design/wf-logo-spec_V1.yaml
research_inline: true
---

# Woodfine Logo Asset — Branding Spec Notes

## Issue

The inline SVG Woodfine wordmark in `app-orchestration-bim/src/render.rs` function
`woodfine_wordmark_svg()` was authored from memory without reference to the canonical asset.
The aspect ratio, typography sizing, and hash placement may differ from the canonical spec.

## Canonical Asset Location

- **SVG:** `customer/woodfine-media-assets/assets/logo/wf-logo_V1.svg`
- **Spec:** `customer/woodfine-media-assets/tokens/design/wf-logo-spec_V1.yaml`
- **Signet only:** `customer/woodfine-media-assets/assets/logo/wf-signet_V1.svg`
- **Full wordmark (root level):** `customer/woodfine-media-assets/ASSET-WORDMARK-WOODFINE.svg`

## Technical Spec (from wf-logo-spec_V1.yaml)

- **ViewBox:** `0 0 144 36`
- **Primary colour:** `#111827` (Woodfine Slate, dark context default)
- **Wordmark:** "WOODFINE" in Geist Bold 11px
- **Descriptor:** "CAPITAL PROJECTS" in Geist Medium 6.5px
- **Structural hashes:** 10.164 × 1.2702 px at specified coordinates
- **Aspect ratio:** 4:1

## Deployment Target

The canonical SVG should be placed at:
```
pointsav-monorepo/apps/app-orchestration-bim/static/images/woodfine-logo.svg
```

And the `woodfine_wordmark_svg()` function in `render.rs` should be replaced with a
`<img>` tag referencing `/static/images/woodfine-logo.svg`, eliminating the inline SVG
string that must be maintained manually.

## Open Questions

1. Master extraction of the SVG data from `wf-logo_V1.svg` to the target path above is
   pending. This is tracked in the project-bim outbox (2026-05-06 message to master).
