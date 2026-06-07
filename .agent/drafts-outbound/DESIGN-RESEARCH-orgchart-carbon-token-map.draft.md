---
schema: foundry-draft-v1
artifact_type: DESIGN-RESEARCH
draft_id: DESIGN-RESEARCH-orgchart-carbon-token-map
status: staged
created: 2026-06-06
author: totebox@project-orgcharts
gateway: project-design
destination: pointsav-design-system/dtcg-vault/research/orgchart-carbon-token-map.md
master_cosign_required: false
research_done_count: 3
research_suggested_count: 1
open_questions_count: 0
research_provenance: training-data
research_inline: true
notes_for_designer: |
  Carbon v11 values sourced from model training data (2024). Verify against
  https://carbondesignsystem.com before committing to dtcg-bundle.json.
  This is a gap analysis document — use it to decide what to extend vs. reuse.
---

# Org Chart ↔ IBM Carbon v11 Token Mapping

*Carbon v11 values from training data — verify against carbondesignsystem.com before use.*

## Color mapping

| Org chart role | Our token | Our hex | Carbon nearest | Carbon hex | Assessment |
|---|---|---|---|---|---|
| Corporate holding | token-green | `#54924E` | `$support-success` | `#198038` | Same intent (affirming/safe), different hue — ours is lighter sage green; Carbon is darker forest green |
| Investment vehicle | token-blue | `#164679` | `$interactive` | `#0f62fe` | Different semantic register — ours is institutional navy; Carbon is bright interactive blue |
| Broker/Asset Mgr | token-purple | `#7C468C` | — | none | **No Carbon equivalent.** Purple is absent from Carbon's semantic system. Custom primitive required. |
| Equity partner | token-orange | `#F15F22` | `$support-caution-major` | `#ff832b` | Close visual territory, wrong semantic — caution implies warning; equity partner is structural |
| Admin entity | token-grey | `#9CA3AF` | `$border-strong-01` | `#8d8d8d` | Similar lightness; different use — Carbon uses for structural borders, we use for entity fill |
| LP / fund vehicle | token-yellow | `#EAB308` | `$support-warning` | `#f1c21b` | Similar visual territory, wrong semantic — warning implies alert; fund vehicle is structural |
| Legacy corporate | token-magenta | `#9F1853` | — | none | **No Carbon equivalent.** IBM Carbon has no semantic magenta/pink in v11 White theme. Custom primitive required. |
| Legacy asset co | token-teal | `#005D5D` | — | none | Carbon's teal (`$support-info` area) is lighter `#0043ce` (blue-teal). Our `#005D5D` is dark forest teal. Different hue family. Custom primitive required. |

**Finding:** 5 of 9 org chart colors (purple, magenta, teal, and the specific values of blue/green) have no close Carbon equivalent. The entity-role semantic layer requires a dedicated `orgchart.*` namespace — reusing Carbon's `$support-*` tokens would import status semantics (health, alerts, warnings) into corporate structure diagrams.

## Spacing scale comparison

Carbon v11 spacing scale (base unit: 8px):

| Token | Value | Org chart use |
|---|---|---|
| `$spacing-01` | 2px | — |
| `$spacing-02` | 4px | — |
| `$spacing-03` | 8px | Base unit — box padding floor |
| `$spacing-04` | 12px | Compact box padding (`6px 8px`) |
| `$spacing-05` | 16px | — |
| `$spacing-06` | 24px | — |
| `$spacing-07` | 32px | — |
| `$spacing-08` | 40px | Minimum observed row gap |
| `$spacing-09` | 48px | — |
| `$spacing-10` | 64px | — |
| `$spacing-11` | 80px | Short box height |
| `$spacing-12` | 96px | — |
| `$spacing-13` | 160px | — |

**Finding:** Box dimensions (110px standard, 145px tall, 210px wide, 250px pill-wide, 1056×816px canvas) do not derive from the 8px Carbon grid. They are determined by: (a) content legibility at 9–12px type, (b) fit within a US Letter landscape page at 96dpi, and (c) SVG connector geometry. The `component.orgchart.*` token group uses absolute px values directly — do not attempt to derive them from `$spacing-*` aliases, as no clean multiples exist.

## Typography comparison

Carbon type sets use IBM Plex Sans. Org charts use system font stack (`-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif`) — no IBM Plex Sans import.

| Carbon type set | Size / Weight | Org chart class | Size / Weight | Assessment |
|---|---|---|---|---|
| `$label-01` | 12px / 400 | `.t-title` | 12px / 800 | Same size, 2× heavier — weight 800 needed for legibility at small size in print |
| `$body-compact-01` | 14px / 400 | `.t-headline` | 14px / 900 | Same size, much heavier — 900 for chart title prominence |
| `$helper-text-01` | 12px / 400 | `.t-preamble` | 9–10px / 400 | Below Carbon's floor — print-density requirement |
| `$code-01` | 12px / 400 mono | `.t-code` | 9px / 400 | Below Carbon's floor |
| — | no equivalent | `.t-kicker` | 11px / 800 | Sub-floor; no Carbon match |
| — | no equivalent | `.t-node` | 10px / 700 | Sub-floor; no Carbon match |

**Finding:** The entire org chart typography system operates below Carbon's minimum type size of 12px (`$label-01`). The 9–11px range is driven by print density requirements — 9 boxes fit across a 1056px canvas at 210px each, leaving ~30px of whitespace. At those node widths, 9–10px type is the correct choice. Carbon's 12px floor is for interactive web UIs; org chart print has different constraints.

## Carbon component mapping

| Carbon component | Org chart equivalent | Gap |
|---|---|---|
| `Tag` | Node number badge `.t-node` | Carbon Tag sits outside content; our number is inside the box |
| `Tile` (clickable card) | `.token-base` node box | Carbon Tile is fluid-width; our nodes are fixed 210px absolute-positioned |
| `TreeView` | Hierarchy SVG connector structure | Carbon TreeView is a vertical list; ours is a 2D canvas with arbitrary connector routing |
| `StatusIndicator` | Color-coded entity role | Carbon StatusIndicator is a small circle; ours is a full box border + fill |
| `StructuredList` | Legend section | Reasonable match — Carbon StructuredList could render the legend cleanly |

**Finding:** The org chart node and connector are original components with no Carbon precedent. They extend Carbon's visual language (color system, border conventions, typography scale) but cannot inherit Carbon's DOM patterns. Both should be documented as extensions that sit outside the Carbon component hierarchy.
