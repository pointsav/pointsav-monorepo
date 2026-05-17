---
schema: foundry-draft-v1
version: "1.0"
draft_id: topic-bim-building-width-method-2026-05-17
language_protocol: TOPIC
state: ready-for-sweep
originating_cluster: project-bim
target_repo: vendor/content-wiki-projects
target_path: topics/bim/building-width-method.md
audience: operator
bcsc_class: vendor-internal
authored: 2026-05-17T22:00:00Z
authored_by: totebox@project-bim
authored_with: claude-opus-4-7
title: "The Backwards Method — Sizing Buildings from Furniture"
research_done_count: 6
research_suggested_count: 4
open_questions_count: 5
research_provenance:
  - "/home/jennifer/sandbox/inputs/project-bim/--- April 01, 2025 -- Collaborators #11 ---/AEC_Floor Plates_Building Width Calculator_V12.pdf (V12, 2025-01-07, J. Woodfine — the canonical mirror diagram)"
  - "/home/jennifer/sandbox/inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Medical_Notes copy.pdf (Summary V2, 2025-05-13 — building-width-calculator Z-values per use type)"
  - "/home/jennifer/sandbox/inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Business_Notes.pdf (Notes V3, M. Woodfine — 21 enumerated width options)"
  - "/home/jennifer/sandbox/inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Private Office_Notes copy.pdf (PO-1/PO-2/PO-3 dimensioned floor plans)"
  - "/srv/foundry/clones/project-bim/woodfine-bim-library/tokens/bim/building-width-calculator.dtcg.json"
  - "/srv/foundry/clones/project-bim/.agent/plans/tool-buildingwidth-architecture.md"
research_inline: true
notes_for_editor: |
  Living document. Section headers and the "Future research" list at the
  bottom are designed to absorb additional research as it lands
  (regulatory citations, manufacturer SKU dimensions, climate-zone
  overlays). Do not collapse the sections — leave them as scaffolding.
  Slide deck preview that renders this same data:
  preview/building-width-calculator.html.
---

# The Backwards Method — Sizing Buildings from Furniture

> **Status:** Living document. Updated 2026-05-17.

## 1. The inversion

Conventional office buildings begin with a building width — sixty feet
for daylight is the rule of thumb in North American Class-A speculative
office — and the tenant fit-out is then made to fit. Woodfine inverts
the sequence. The Building Width Calculator starts with the tenant's
furniture (desk, exam table, lab bench, courtroom bench) and the
building width is whatever the arrangement requires.

Source: V12, Jan 7 2025, *Spatial Taxonomy — Building Width Calculator*
(J. M. Woodfine).

## 2. The mirror cross-section

The cross-section is a seven-row strip:

```
Façade
Zone 1 — Habitat   (desks/workstations)            6.0 m
Zone 2 — Magazine  (storage/bookshelves, etc.)     3.0 m  ← TBD per use type
Zone 3 — Corridor  (single, centreline)            3.0 m  ← TBD per use type
Zone 2 — Magazine  (mirrored)                      3.0 m
Zone 1 — Habitat   (mirrored)                      6.0 m
Façade
─────────────────────────────────────────────────────────
TOTAL                                              21 m / 69 ft
```

V12 establishes Professional Office at **21 m / 69 ft** total as the
baseline. Z2 Magazine and Z3 Corridor are marked "TBD" — they are
shaped per use type once the furniture is selected.

The crucial geometric fact: **Zone 3 Corridor is a single centreline
row, not mirrored**. Habitat and Magazine appear on both sides of the
corridor; the corridor appears once.

## 3. The formula

```
Building Width = 2 × (Zone 1 Habitat + Zone 2 Magazine) + Zone 3 Corridor
```

This is confirmed by two independent V3 sources:

- V12 Professional Office: 2 × (6 + 3) + 3 = **21 m** ✓
- Business Notes V3 Option A/A: 2 × (5.51 + 9.26) + 2.75 = **32.29 m**
  — the document explicitly tabulates this total

A common error (corrected in the project-bim HTML preview on 2026-05-17)
is to double the corridor as well. Doubling the corridor yields an
overstated building width (Professional Office becomes 23.6 m instead
of 21 m; Civic becomes 33.66 m instead of 30.06 m). Verify any third-party
implementation against the formula above.

## 4. Why three zones

### 4.1 Zone 1 — Habitat

The 6 m daylight depth is the design intent for **workstation-occupied**
space. It approximates the European Lighting Standard guidance for
natural light at a workstation (cited in the project-bim research
trail as EN 12464-1 / EN 17037, pending precise citation in the
regulatory token file).

The 6 m figure is not universal. Three of the seven established
use types diverge:

- **Academic** compresses Habitat to **4.7 m** because seating faces
  forward (toward a podium/board) rather than toward the façade —
  the lighting requirement applies to the workstation, not the
  back of the room.
- **Medical** widens Habitat to **7.28 m** (286 5/8" measured on the
  M3 sketch) to accommodate exam-table depth plus patient and
  clinician circulation per German Circulation Law-style clearances.
- **Laboratory** widens Habitat to **6.78 m** to accommodate bench
  depth and fume-hood clearance.

### 4.2 Zone 2 — Magazine

Magazine is the slack variable. It holds storage, bookshelves,
filing, server rooms, staff rooms. The width is set by what the
tenant needs to balance the plate — V12 directs that the Z2 depth
"can be used to balance out the dimensions of the floor plate to
not end up with simply a long, narrow rectangle."

In practice Z2 is the widest zone for Business (9.26 m, Option A/A)
and Civic (7.23 m), and the narrowest for Private Office (1.37 m,
4'6") because individual offices carry minimal shared storage.

### 4.3 Zone 3 — Corridor

The corridor is a single centreline. Its width is set by egress and
accessibility — IBC stretcher minima for Medical, IBC high-hazard
egress for Laboratory, public assembly egress for Civic. V12 directs
that corridors be **overdesigned by 20%** to promote well-being and
absorb the "in and out" traffic of smaller tenants.

Two use types — Private Office and Academic — have **no Zone 3**.
Their leaseholds open directly onto the shared building corridor
(part of the building core, outside the leasehold), so they carry
no corridor inside the tenant envelope.

## 5. The 0.7 m perpendicular-desk addition

V2 Summary (2025-05-13) records a perpendicular-desk supplement:

> With desks placed perpendicular to the façade, be careful, as an
> extra 0.7 metres is required for proper circulation for three
> desks in series in Zone 1—Habitat. Without accounting for the
> circulation, 6 metres exactly to the façade results in only two
> desks perpendicular to the façade in Zone 1—Habitat rather
> than three desks.

Token: `bim.circulation-addition.perpendicular-desk = 0.7 m`.

This is a Zone 1 supplement, not a separate zone. It changes the
effective Habitat depth from 6.0 m to 6.7 m when perpendicular desk
arrangements are required to seat three desks in series.

## 6. Why this method matters

The backwards method makes three claims that conventional speculative
office practice does not:

1. **Furniture drives geometry.** A landlord can't reduce a tenant's
   exam-table depth by tightening the floor plate; the building must
   accommodate the use, not vice versa.
2. **Each tenant type has a known width.** Once the use type is
   selected, the building width is computed, not negotiated.
3. **Mixed-tenant floors must reconcile widths at design time.** A
   floor with a Medical anchor (27.2 m) and a Private Office cluster
   (14.7 m) has to absorb 12.5 m of demising tolerance — the floor
   plate width must match the widest tenant on that floor, with the
   narrower tenants accepting the surplus (typically absorbed as
   wider Zone 2 magazines).

## 7. Status and ratification

| Component | Source | Status |
|---|---|---|
| Mirror cross-section diagram | V12 (Jan 2025) | Ratified |
| Z1 = 6 m default | V12 + EN 12464-1 (cited) | Ratified |
| Z2 = TBD per use type | V12 | Ratified |
| Z3 = centreline, single | V12 (visual); Business V3 totals | Ratified |
| Formula `2(Z1+Z2) + Z3` | V12; Business V3 enumeration | Ratified |
| 0.7 m perpendicular-desk add | Summary V2 (May 2025) | Ratified |
| Professional Office Z2/Z3 | V12 ("TBD", placeholder 3 m / 3 m) | **Pending** — Z2 and Z3 specifics deferred |
| Business Z1/Z2/Z3 | Notes V3 (21 options enumerated) | **Pending operator selection** from the enumerated grid |
| Civic Z1/Z2/Z3 | No completed DISCOVERY sketch | **Pending** sketch |

## 8. Future research

- [ ] Add precise regulatory citation for the 6 m Habitat (EN 12464-1
      clause, ArbStättV cross-reference)
- [ ] Document the German Circulation Law clearance referenced in
      Medical and Laboratory ($\geq$ 1 m? — confirm)
- [ ] Confirm whether Z3 corridor is always single, or whether a
      large floor plate ever introduces a secondary corridor parallel
      to the centreline
- [ ] Resolve the Professional Office Z2/Z3 placeholders — V12 leaves
      them at 3 m / 3 m TBD; the current HTML preview uses those
      values but the actual constructed value will come from the
      Professional Office Key Plan when it is completed
- [ ] Decide which of the 21 Business width options is canonical for
      the token store (current proposal: Option A/A at 32.29 m, the
      widest of the enumerated symmetric options)
- [ ] Verify the 0.7 m perpendicular-desk addition is a Habitat
      *supplement* (effective Z1 = 6.7 m) versus a separate zone
      contribution — the architecture plan flags this as
      `bim.circulation-addition.perpendicular-desk` with description
      ambiguity (see plans/tool-buildingwidth-architecture.md, item 5)

## 9. Related documents

- `topic-bim-zone-depths-per-use-type.md` — the per-use-type table
  with sources and rationale for each zone depth
- `plans/tool-buildingwidth-architecture.md` — the Rust engine
  architecture that consumes these tokens
- `preview/building-width-calculator.html` — slide deck preview
  rendering this data live from the BIM_TOKENS object

## Appendix A — Source-document timeline

| Date | Document | Contribution |
|---|---|---|
| 2025-01-07 | V12 — Building Width Calculator | Canonical mirror cross-section; 21 m baseline; "TBD" placeholders for Z2/Z3 |
| 2025-03-03 | Collaborators #32 — Key Plans Samples V2 | Per-use-type key plan area schedules |
| 2025-05-12 | Cappelletti-Sestito Medical sketches | M1/M2/M3 dimensioned plans (286 5/8" + 192" + 113 7/8") |
| 2025-05-13 | Summary V2 — Key Plans + BWC | Z1/Z2/Z3 tabulation per use type |
| 2025-11-29 | V3 Summary_Key Plans tab (xlsx) | Token-source spreadsheet (DTCG inputs locked) |
| 2026-01-06 | CONSTRUCTION FFE_FIN.xlsx | FFE-anchored construction handoff |
| 2026-05-17 | preview/building-width-calculator.html | This document's slide-deck preview |
