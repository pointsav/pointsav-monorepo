---
schema: foundry-draft-v1
version: "1.0"
draft_id: design-component-bim-audit-log-2026-05-06
language_protocol: DESIGN-COMPONENT
state: ready-for-sweep
created: 2026-05-06T22:00:00Z
research_done_count: 2
research_suggested_count: 0
open_questions_count: 0
research_provenance: "bim-token-taxonomy.md component catalog (console-unique #2); sub-agent A report 2026-04-28; IFC 4.3 IfcOwnerHistory schema (IfcPersonAndOrganization + IfcApplication)"
research_inline: false
route_to: project-design
target_path: pointsav-design-system/components/bim-audit-log/
---

# bim-audit-log — component recipe

## Identity

| Field | Value |
|---|---|
| Component name | `bim-audit-log` |
| IFC anchor | `IfcOwnerHistory` |
| Uniclass 2015 | FI_60_60 |
| Surface scope | Console-unique (read-only operations) |
| Mode | Read-only — no mode prop needed |
| Container element | `<section class="bim-audit-log">` |
| ARIA role | `<section>` with `aria-label="Element change history"` |

## Purpose

Renders the change history for an IFC element by reading its
`IfcOwnerHistory` chain. Each `IfcOwnerHistory` record carries:
who changed it (`IfcPersonAndOrganization`), when (timestamp),
which application made the change (`IfcApplication`), and
what the change type was (ADDED, MODIFIED, DELETED, NOTDEFINED).

In the Totebox Archive, element YAML sidecars extend this with
field-level diff data (old value → new value per property). This
component surfaces both the IFC-native history and the sidecar diff
when available.

## Visual anatomy

```
.bim-audit-log (section, aria-label="Element change history")
  .bim-audit-log__header
    h3 "History"
    .bim-audit-log__element-id
      code "2N1NMOV9z7$8Ww2DVqvxPB"   ← IfcRoot.GlobalId
      span "IfcWall"

  ol.bim-audit-log__entries (reversed — newest first)
    li.bim-audit-log__entry[data-change-type="MODIFIED"]
      time.bim-audit-log__time (datetime="2026-04-28T14:23:00Z") "28 Apr 2026 14:23"
      .bim-audit-log__who "Jennifer Woodfine · Revit 2024 (Autodesk)"
      .bim-audit-log__change-type "MODIFIED"
      details.bim-audit-log__diff
        summary "2 properties changed"
        table
          thead
            tr: th "Property" + th "Before" + th "After"
          tbody
            tr: td "ThermalTransmittance" + td "0.22" + td "0.18"
            tr: td "FireRating" + td "RW 45"  + td "RW 60"

    li.bim-audit-log__entry[data-change-type="ADDED"]
      time "15 Mar 2026 09:41"
      .bim-audit-log__who "Peter Woodfine · ArchiCAD 27 (Graphisoft)"
      .bim-audit-log__change-type "ADDED"
```

## Interaction model

- Entries load paginated (20 per page); "Load earlier" at bottom
- Click on an entry: restores viewport camera to the BCF viewpoint
  captured at that change (if available in the sidecar YAML)
- Diff table: collapsed by default; expand with `<details>`; keyboard Enter
  on summary expands
- Export: "Export CSV" button → `{guid}-history.csv`

## ARIA contract

- `<ol reversed>` for chronological list (newest at top, semantic order is reverse-chronological)
- Each entry: `<li>` with `aria-label="Change on [date]: [change-type]"`
- `<time datetime="...">` for machine-readable timestamps
- Diff details: `<details>/<summary>` with `aria-expanded`
- Change type badge: `aria-label="Change type: [value]"` on the badge span

## CSS token dependencies

- `--bim-font-mono` — GUID display, property values in diff table
- `--bim-accent` — ADDED entries border
- `--bim-fg-muted` — NOTDEFINED entries
- `color: var(--bim-error)` — DELETED entries (red variant from base tokens)

## Service dependency

Reads from Totebox Archive vault: `vault/elements/{guid}.yaml` for
sidecar diff data, combined with `IfcOwnerHistory` from the IFC file
at `vault/ifc/{model}.ifc`. Falls back to IFC-only history when no
sidecar exists (new installations before YAML sidecar generation runs).
