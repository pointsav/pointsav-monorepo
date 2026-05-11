---
schema: foundry-draft-v1
version: "1.0"
draft_id: design-component-bim-view-navigator-2026-05-06
language_protocol: DESIGN-COMPONENT
state: ready-for-sweep
created: 2026-05-06T22:00:00Z
research_done_count: 2
research_suggested_count: 1
open_questions_count: 0
research_provenance: "bim-token-taxonomy.md component catalog (universal AEC #4); sub-agent A report 2026-04-28; IFC 4.3 IfcViewDefinition schema"
research_inline: false
route_to: project-design
target_path: pointsav-design-system/components/bim-view-navigator/
---

# bim-view-navigator — component recipe

## Identity

| Field | Value |
|---|---|
| Component name | `bim-view-navigator` |
| IFC anchor | `IfcViewDefinition` (via MVD) |
| Uniclass 2015 | FI_60 |
| Surface scope | Universal AEC — workplace + console |
| Mode prop | `data-mode="workplace"` \| `data-mode="console"` |
| Container element | `<nav class="bim-view-navigator">` |
| ARIA role | `<nav>` with `aria-label="Saved views"` |

## Purpose

Manages named saved viewpoints (camera position + section plane state
+ selected element set) stored as `IfcViewDefinition` references in
the IFC file. Provides a list of saved views the user can jump between.
Analogous to the "Views" panel in Revit or the "Views" tab in Navisworks.

In workplace mode, the user can save new views, rename, reorder, and
delete. In console mode the list is read-only — the user can activate
any saved view but cannot modify or create.

## Visual anatomy

```
.bim-view-navigator (nav, aria-label="Saved views")
  .bim-view-navigator__header
    h3 "Views"
    button.bim-view-navigator__add (workplace only, aria-label="Save current view")
  ul.bim-view-navigator__list (role="listbox", aria-label="Saved views")
    li.bim-view-navigator__item (role="option", aria-selected)
      .bim-view-navigator__thumbnail (aria-hidden="true")  ← 48×32 px preview
      .bim-view-navigator__label "Ground floor plan"
      .bim-view-navigator__meta "IfcViewDefinition · 2026-04-28"
      .bim-view-navigator__actions (workplace only)
        button "Rename"
        button "Delete"
```

## Mode-prop behaviour

| Behaviour | `workplace` | `console` |
|---|---|---|
| Activate view | Click → restore camera + section planes + selection | Click → restore camera + selection |
| Save current view | "+" button → name prompt → append to list | Hidden |
| Rename | Inline double-click rename | Disabled |
| Delete | Trash icon on hover | Hidden |
| Reorder | Drag handle (grip icon) | Disabled |

## View data model

Each view item carries:
- `data-view-id` — internal UUID
- `data-ifc-guid` — IfcViewDefinition GlobalId (when synced to IFC)
- `data-camera` — JSON: `{eye, target, up, projection}`
- `data-section-planes` — JSON array of active section plane states
- `data-selection` — comma-separated GUIDs of selected elements at capture time

## ARIA contract

- `role="listbox"` on `<ul>` with `aria-label="Saved views"`
- `role="option"` on each item with `aria-selected`
- Thumbnail: `aria-hidden="true"` (decorative preview)
- Add button: `aria-label="Save current view"`, `aria-keyshortcuts="Ctrl+S"` (workplace)
- Keyboard: Up/Down arrows navigate; Enter activates; Delete removes (workplace)

## CSS token dependencies

- `--bim-bg-surface` — panel background
- `--bim-border` — item separator
- `--bim-accent` — active view border
- `--bim-font-mono` — metadata line (IFC GUID, date)

## Not part of this component

- Camera state management: host application + xeokit sync
- IfcViewDefinition write-back to IFC file: service-buildings endpoint
- Section plane state management: bim-section-plane component
