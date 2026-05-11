---
schema: foundry-draft-v1
state: draft
language_protocol: DESIGN-RESEARCH
originating_cluster: project-gis
target_repo: vendor/pointsav-design-system
target_path: research/
target_filename: design-gis-chain-search-bento-2026-05-06.md
audience: internal-design
bcsc_class: internal
authored: 2026-05-06
authored_by: project-gis Task Claude
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 2
open_questions_count: 2
research_provenance: |
  MapLibre GL JS docs: setPaintProperty circle-opacity, addLayer filter expressions.
  Implemented and deployed in commits d6d29c2 (chain search) and prior UX sprint.
  Bento box reorder based on operator brief 2026-05-06.
research_inline: true
notes_for_editor: |
  Route to project-design. Two components to extract (chain-search-input, bento-info-card).
  One pattern to document: highlight-and-dim map layer interaction.
  Open question 1: should chain search also filter the bento count display?
  Open question 2: should sel-el block support click-to-navigate (Google Maps–style)?
---

# GIS UI Design Research — Chain Search + Bento Box (May 2026)

Two interface features shipped in this sprint: a retailer chain search/highlight and a restructured bento box information hierarchy. Both address the same operator need: making the map legible as a lookup tool, not just an exploration surface.

---

## 1. Chain Search (Light-Up Filter)

### Problem

The map shows 4,237 clusters as undifferentiated dots. An operator wanting to understand Costco's coverage cannot isolate it without clicking every cluster. The overview zoom conveys density but not brand-level pattern.

### Solution

A typeahead input in the panel header filters by brand name. Matching clusters glow amber with a ring; all others dim to 10% opacity. The interaction is:

1. Type "Costco" → dropdown shows canonical brand names containing that string
2. Select → `activeChainFilter` set, `reapplyChainHighlight()` fires
3. All clusters containing a Costco location (in `hw_list` or `wh_list`) receive full opacity + amber stroke ring
4. Non-matching clusters dim to 10% opacity
5. `×` button clears filter, restores uniform display

### Implementation pattern

The filter uses MapLibre's `['in', chainId, ['get', 'hw_list']]` expression, which does substring match against the JSON-encoded array stored in the tile property. This is efficient: no tile reload, no API call, purely paint-layer manipulation.

A separate `nodes-highlight` layer (circle, amber stroke, no fill) is toggled visible/hidden. This avoids modifying the base `nodes` layer's color logic.

**Key data structure:**
- `CHAIN_DISPLAY_NAMES` maps chain IDs to display names (e.g. `"home-depot-ca"` → `"Home Depot"`)
- `BRAND_CHAINS` reverses this: `"Home Depot"` → `["home-depot-us", "home-depot-ca", "home-depot-mx"]`
- The filter expression checks all chain IDs for a brand in a single `['any', ...]` expression

### Design tokens needed

- `--color-highlight-amber`: `#F59E0B` (used for ring stroke)
- `--opacity-dim`: `0.10` (non-matching clusters)
- `--opacity-active`: `0.88` (matching clusters)
- Typeahead dropdown: inherits panel surface token, 180px max-height, scroll

### Open question 1

Should clearing the chain filter (×) also restore the bento box to overview mode, or should the bento remain on the last-clicked cluster? Current behavior: bento unchanged, filter clears independently.

---

## 2. Bento Box Information Hierarchy

### Problem

The original bento box led with raw data (score, tier) before establishing geographic context. An operator looking at an unfamiliar cluster had no immediate sense of where they were before seeing a numeric grade.

### Operator brief

> "Region Market first, then Market Grade and Co-location Score, then Anchor Location, then Nearby Services, then Retailer Info at the bottom."

### Revised hierarchy

```
1. Market Region          — geographic anchor (bold, brand-primary navy)
2. Market Grade · Score   — tier badge + score_final / 1000 (combined flex row)
3. Anchor Location        — primary retail node (pill + sub-entities)
4. Co-Tenants             — HW + WH chain pills
5. Nearby Services        — medical / academic counts
6. sel-el block           — cycling selected-element detail (updates on dot click)
```

The sel-el block at the bottom inverts the usual "inspector leads with the clicked item" pattern. Instead, the cluster's grade and anchor are always the primary read — the individual element detail is secondary context. This reflects the operator's workflow: grade and anchor first, then drill into any dot.

### Component extraction candidates

**`cluster-grade-marker`** — the tier badge (T3/T2/T1) + score display. Currently inline HTML in `showClusterDetail()`. Should be a reusable component usable on list views, export tables, and print layouts.

**`location-info-card`** — the bento box panel itself. Currently a `<div id="inspector">` with inline innerHTML. Should be a structured component with named slots: region, grade, anchor, co-tenants, services, element-detail.

### Open question 2

The `sel-el` block shows the most recently clicked dot. On mobile, this competes for vertical space with the cluster-level content. Consider a tab bar (Cluster / Selected) rather than a stacked layout for mobile.
