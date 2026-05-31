---
schema: foundry-draft-v1
version: "1.0"
draft_id: design-component-bim-properties-panel-2026-05-06
language_protocol: DESIGN-COMPONENT
state: ready-for-sweep
created: 2026-05-06T22:00:00Z
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: "pointsav-design-system/components/bim-properties-panel/{recipe.html,recipe.css,aria.md}; IFC 4.3 Pset_WallCommon + Qto_WallBaseQuantities schema; sub-agent A report 2026-04-28"
research_inline: false
route_to: project-design
target_path: pointsav-design-system/components/bim-properties-panel/
---

# bim-properties-panel — component recipe

## Identity

| Field | Value |
|---|---|
| Component name | `bim-properties-panel` |
| IFC anchor | `IfcPropertySet` (Pset_*) + `IfcQuantitySet` (Qto_*) |
| Uniclass 2015 | FI_60 (Information management) |
| Surface scope | Universal AEC — workplace + console |
| Mode prop | `data-mode="workplace"` \| `data-mode="console"` |
| Container element | `<aside class="bim-properties-panel">` |
| ARIA role | Landmark `<aside>` with `aria-label="Element properties"` |

## Purpose

Renders all Pset_* (property sets) and Qto_* (quantity sets) attached
to the currently selected IFC element. Surfaces the IFC class name and
GUID in the header for identification. In workplace mode, all values
are editable inline with the appropriate input type (text, number, or
dropdown for enumerated values). In console mode, all values are
read-only key/value rows.

## Visual anatomy

```
.bim-properties-panel (aside, aria-label="Element properties")
  .bim-properties-panel__header
    h3 "Properties"
    .bim-properties-panel__subject
      .bim-properties-panel__class  "IfcWall"
      .bim-properties-panel__guid   "2N1NMOV9z7$8Ww2DVqvxPB"

  .bim-properties-panel__section
    h4 "Identification"
    dl
      dt "Name"   dd "WAL-EXT-001"
      dt "Tag"    dd "EXT-A1-G-005"
      dt "Classification (Uniclass)"  dd "EF_25_10_30"

  .bim-properties-panel__section
    h4 "Pset_WallCommon"
    dl
      dt "FireRating"               dd "RW 60"
      dt "ThermalTransmittance (U)" dd "0.18 W/m²K"
      dt "IsExternal"               dd "Yes"
      dt "LoadBearing"              dd "No"

  .bim-properties-panel__section
    h4 "Qto_WallBaseQuantities"
    dl
      dt "Length"    dd "4.250 m"
      dt "Height"    dd "2.700 m"
      dt "NetVolume" dd "3.443 m³"
```

## Mode-prop behaviour

| Behaviour | `workplace` | `console` |
|---|---|---|
| Values | Editable `<input>` or `<select>` per property type | Read-only `<dd>` text |
| Save trigger | Blur or Enter on each field | N/A |
| Add property set | "+" button on section header | Hidden |
| Delete property | Right-click → Remove | Disabled |
| Copy GUID | Click → clipboard | Click → clipboard (both modes) |

## Property rendering rules

- `BOOLEAN` → checkbox in workplace; "Yes" / "No" text in console
- `IfcLabel`, `IfcText` → `<input type="text">` / text
- `IfcReal`, `IfcInteger`, measurement types → `<input type="number">` with unit suffix
- Enumerated `IfcLabel` (e.g. FireRating values) → `<select>` in workplace
- `IfcClassificationReference` → read-only monospace `<code>` with bSDD link

## ARIA contract

- `<dl>` for property key/value pairs throughout
- `<dt>` = property name, `<dd>` = value
- Section headings: `<h4>` with unique id; section `aria-labelledby` that id
- Edit inputs: `aria-label` = the corresponding `<dt>` text
- GUID span: `aria-label="Element GUID"`, `title` = full GUID

## CSS token dependencies

- `--bim-font-mono` — GUID and IFC class display
- `--bim-text-sm` — key/value row text size
- `--bim-bg-surface` — panel background
- `--bim-border` — section dividers
- `--bim-accent` — active input border (workplace)

## Not part of this component

- Loading / fetching properties from service-buildings: host app concern
- Climate zone performance values: read from climate-zones.dtcg and shown
  as a separate informational row in the Regulation + Climate Zone tabs
  of the token detail page (app-orchestration-bim), not inline here
- BCF issue creation: handled by bim-annotation-layer
