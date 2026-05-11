---
schema: foundry-draft-v1
version: "1.0"
draft_id: design-component-bim-spatial-tree-2026-05-06
language_protocol: DESIGN-COMPONENT
state: ready-for-sweep
created: 2026-05-06T22:00:00Z
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: "pointsav-design-system/components/bim-spatial-tree/{recipe.html,recipe.css,aria.md}; sub-agent A report 2026-04-28 decision BB.4 (Bonsai SpatialTree pattern); bim-token-taxonomy.md"
research_inline: false
route_to: project-design
target_path: pointsav-design-system/components/bim-spatial-tree/
---

# bim-spatial-tree — component recipe

## Identity

| Field | Value |
|---|---|
| Component name | `bim-spatial-tree` |
| IFC anchor | `IfcSpatialElement` |
| Uniclass 2015 | SL (Spaces/Locations) |
| Surface scope | Universal AEC — workplace + console |
| Mode prop | `data-mode="workplace"` \| `data-mode="console"` |
| Container element | `<aside class="bim-spatial-tree">` |
| ARIA role | Landmark `<aside>` with `aria-label="Spatial hierarchy"` |

## Purpose

The spatial tree renders the IfcSpatialElement hierarchy
(IfcSite → IfcBuilding → IfcBuildingStorey → IfcSpace) as an
accessible collapsible tree. It is the primary navigation instrument
for orienting in a BIM file. Default expansion is storey-level
(per decision BB.4 — Bonsai research): all buildings expanded to
show storeys; spaces within a storey collapsed by default unless
the user has expanded them. This default reflects construction-site
usage: engineers navigate by level, not by room name.

## Visual anatomy

```
.bim-spatial-tree (aside, aria-label="Spatial hierarchy")
  .bim-spatial-tree__header
    h3 "Spatial"
    input.bim-spatial-tree__search (type="search", aria-label="Search spaces")
  ul.bim-spatial-tree__list (role="tree")
    li[role="treeitem"][aria-expanded] (data-ifc-class="IfcSite")
      .bim-spatial-tree__icon (⌧)
      .bim-spatial-tree__label "Site"
      ul[role="group"]
        li[role="treeitem"][aria-expanded] (data-ifc-class="IfcBuilding")
          .bim-spatial-tree__icon (▣)
          .bim-spatial-tree__label "Building A"
          ul[role="group"]
            li[role="treeitem"][aria-expanded="true"] (data-ifc-class="IfcBuildingStorey")
              .bim-spatial-tree__icon (═)
              .bim-spatial-tree__label "Ground Floor"
              ul[role="group"][hidden]  ← spaces collapsed by default
```

## Mode-prop behaviour

| Behaviour | `workplace` | `console` |
|---|---|---|
| Selection | Click → highlight + emit GUID to store | Click → highlight + emit GUID to store |
| Drag-to-reorder | Permitted (IfcSite scoping) | Not permitted |
| Right-click context menu | Yes — Move / Rename / Set active level | No |
| Keyboard: Space | Expand/collapse | Expand/collapse |
| Keyboard: Enter | Select element | Select element |
| Keyboard: Delete | Remove spatial unit (with confirmation) | Disabled |

## ARIA contract

- Tree root: `role="tree"` on `<ul>`, `aria-label="Spatial hierarchy"`
- Tree items: `role="treeitem"`, `aria-expanded` on expandable items, `aria-selected` on selected
- Icon spans: `aria-hidden="true"`
- Search input: `aria-label="Search spaces"`, `aria-controls="[list-id]"`
- Keyboard: Arrow keys navigate; Space expands/collapses; Enter selects

## CSS token dependencies

- `--bim-font-mono` — GUID display in tooltip
- `--bim-accent` — selected state border
- `--bim-bg-surface` — panel background
- `--bim-border` — tree item separator

## Design decisions (BB.4)

BB.4 research (2026-04-28): Bonsai BIM uses a scene-graph tree
repurposed as spatial navigation. This component is purpose-built for
the IfcSpatialElement hierarchy — it does NOT render geometry nodes,
only the four spatial tiers. Default expansion is storey-level
because construction crews navigate by level; opening all spaces
would create a list too long to scan on tablet.

## Not part of this component

- 3D selection sync: wired by the host application between
  bim-spatial-tree and bim-viewport-3d via the BIM_STORE signal
- Search backend: the search `<input>` filters the already-loaded
  tree client-side; server queries are the host app's concern
- Section planes: handled by bim-section-plane, not this component
