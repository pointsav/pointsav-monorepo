---
schema: foundry-draft-v1
version: "1.0"
draft_id: design-component-bim-regulation-rs1-2026-05-06
language_protocol: DESIGN-COMPONENT
state: ready-for-sweep
created: 2026-05-06T22:00:00Z
research_done_count: 3
research_suggested_count: 0
open_questions_count: 1
research_provenance: "sub-agent B report 2026-04-28 (City Code as Composable Geometry); IDS 1.0 buildingSMART spec; service-codes scaffold at port 9103; bim-design-philosophy.md"
research_inline: false
route_to: project-design
target_path: pointsav-design-system/components/bim-regulation-rs1/
open_question_1: "Should the regulation overlay panel be a standalone component in the design-system, or is it exclusively rendered by the token detail page in app-orchestration-bim? Decision affects whether it needs recipe.html/css or is a render.rs sub-template only."
---

# bim-regulation-rs1 — component recipe

## Identity

| Field | Value |
|---|---|
| Component name | `bim-regulation-rs1` |
| IFC anchor | `IfcConstraint` |
| Uniclass 2015 | Co_25_50 |
| Surface scope | App-orchestration-bim token detail page (standalone overlay viewer) |
| Mode | Read-only display — no mode prop |
| Container element | `<section class="bim-regulation-overlay">` |
| ARIA role | `<section>` with `aria-label="Regulatory overlay — [jurisdiction]"` |

## Purpose

Renders a jurisdictional regulatory overlay as a static lookup table.
The first planned overlay is Woodfine BC RS-1 (BC Building Code 2018 +
NBC 2020 Part 11, RS-1 residential zoning). The component is NOT an
interactive constraint engine — it is a reference surface showing the
human-readable constraint table that corresponds to the machine-readable
IDS 1.0 file.

This design directly implements Doctrine claim #41 (City Code as
Composable Geometry). The component name `bim-regulation-rs1` uses the
specific overlay identifier; the general shell is `bim-regulation-overlay`.

## Visual anatomy

```
.bim-regulation-overlay (section, aria-label="Regulatory overlay — BC RS-1")
  .bim-regulation-overlay__header
    .bim-regulation-overlay__jurisdiction "BC, Canada — RS-1 Residential"
    .bim-regulation-overlay__standard "BC Building Code 2018 + NBC 2020 Pt.11"
    .bim-regulation-overlay__status-chip "In development — v0.0.3"

  .bim-table-wrap
  table.bim-regulation-overlay__table
    thead
      tr: th "Element" + th "Constraint" + th "Required value" + th "Unit" + th "Source"
    tbody
      tr: td "IfcWall (IsExternal=true)" + td "ThermalTransmittance" + td "≤ 0.210" + td "W/m²K" + td "NBC 2020 §9.36"
      tr: td "IfcSlab (IsExternal=true)" + td "ThermalTransmittance" + td "≤ 0.183" + td "W/m²K" + td "NBC 2020 §9.36"
      tr: td "IfcWindow" + td "ThermalTransmittance" + td "≤ 1.40" + td "W/m²K" + td "NBC 2020 §9.36"
      tr: td "IfcDoor (IsExternal=true)" + td "ThermalTransmittance" + td "≤ 1.40" + td "W/m²K" + td "NBC 2020 §9.36"
      tr: td "IfcWall (IsExternal=true)" + td "Min. height from grade" + td "150 mm" + td "mm" + td "BC Building Code §9.13"

  .bim-regulation-overlay__three-file
    details
      summary "Three-file bundle"
      ul
        li: code "woodfine-bc-rs1.ids" — IDS 1.0 constraint specification
        li: code "woodfine-bc-rs1.ifc" — IFC exclusion-zone fragments
        li: code "bsdd-woodfine-bc.json" — bSDD dictionary
```

## Three-layer composition rule (displayed in component)

```
Priority: municipal (1) → provincial (2) → federal (3) → accessibility (4)
Numeric: effective_value = max(regulation_min, climate_zone_min)
Geometric: exclusion-zone IFC fragments take unconditional precedence
```

This rule is shown as an informational callout in the component
(not interactive — read-only reference).

## ARIA contract

- `<section>` with descriptive `aria-label`
- Status chip: `aria-label="Status: [value]"`
- Table: `<caption>` "Regulatory constraints for [jurisdiction]"
- `<th scope="col">` for column headers
- Source references: `<abbr title="[full citation]">` for standard abbreviations

## CSS token dependencies

- `--bim-font-mono` — element class names, constraint values, standard citations
- `--bim-accent` — active/registered overlay header border
- `--bim-bg-surface` — panel background
- `--bim-border` — table cell dividers

## How it connects to service-codes

`service-codes` (port 9103) serves registered overlays as:
```
GET /overlays/{jurisdiction-id}  → { constraints: [...], ids_url: "...", ifc_url: "..." }
```
The component renders the static constraint table from this response.
At v0.0.1 `service-codes` is scaffold-coded; the overlay data is
hardcoded in the recipe until the endpoint ships at v0.0.3.
