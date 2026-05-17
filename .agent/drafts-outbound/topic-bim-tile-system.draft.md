---
schema: foundry-draft-v1
version: "1.0"
draft_id: topic-bim-tile-system-2026-05-17
language_protocol: PROSE-TOPIC
state: ready-for-sweep
originating_cluster: project-bim
target_repo: vendor/content-wiki-projects
target_path: topics/bim/tile-system.md
audience: operator
bcsc_class: vendor-internal
authored: 2026-05-17T22:00:00Z
title: "Tile System — Small, Medium, Large, and Special"
authored_by: totebox@project-bim
authored_with: claude-opus-4-7
research_done_count: 5
research_suggested_count: 3
open_questions_count: 4
research_provenance:
  - "AEC_Floor Plates_Methodology_V12.pdf (May 06 2025, Collaborators #27) — 13 pages — tile catalogue source"
  - "AEC_Floor Plates_Tiles_Alternatives.pdf (May 06 2025, Collaborators #27) — Tile A, B-1, C-1..C-4, A-1, B-2, C-3, C-4"
  - "PROJECTS_MCorp_Tear Sheet_Floor Plates_Tiles_Alternatives.pdf (January 06 2026, V12) — alternates tear sheet"
  - "PROJECTS_MCorp_Tear Sheet_Floor Plates_Key Plans and Tiles.pdf (January 06 2026, V12) — full Key Plan inventory"
  - "woodfine-bim-library/tokens/bim/tile-system.dtcg.json"
research_inline: true
notes_for_editor: |
  Living document — the canonical tile catalogue. The DTCG token file
  is the machine-readable mirror; this topic is the human-readable
  narrative. When the token file is amended, this topic is updated
  in the same commit.
---

# Tile System — Small, Medium, Large, and Special

A Tile is a rectangular composable unit of the floor plate. Each
Tile is exactly one HVAC climate zone. Tiles compose to fill the
Net Leasable Area without remainder.

There are three regular tile families (Small, Medium, Large) and
one residual family (Special Tiles).

## The three regular families

### Small Tiles — 2,700 SF (250.84 m²)

| Code | Name | Composition | Climate zones | Role |
|---|---|---|---|---|
| Tile A | Corporate Office | Single 2,700 SF unit | 1 | Mid-plate Corporate anchor |
| Tile B-1 | Private Office | 5 PO (300 + 500 + 450 + 300 + 300) + Professional Small 800 ≈ 2,700 SF | 5 (PO) + 1 (Pro Small) = 6 | Private-office heavy interior |
| Tile C-1 | Professional Office (Med + Med + Small) | PO-M 2,000 + PO-M 450 + PO-S 300 = 2,750 | 3 | Professional anchor (Medical, Business) |
| Tile C-2 | Professional Office (Large + Small) | PO-L 2,400 + PO-S 300 = 2,700 | 2 | Larger-anchor professional |
| Tile C-3 | (Alternatives) | Variant composition | 2 | See Alternatives PDF |
| Tile C-4 | (Alternatives) | Variant composition | 1 | Civic / Academic anchor |
| Tile E-1 | Mixed End Cap (Left) | Mixed PO + Professional Small + corridor | 3 | Left short side |
| Tile E-2 | Mixed End Cap (Right) | Mirror of E-1 | 3 | Right short side |

**Note.** The V12 Methodology PDF page 2 also documents a
Small "Tile B" with composition 300 + 500 + 300 + 450 + 450 + 300
= 2,300 SF of PO + 400 SF residual. The Alternatives PDF expands
Tile B into Tile B-1 with a Professional Small unit added.
Convention: Tile B-1 is the live token; Tile B is the methodology
shorthand for the same role.

### Medium Tiles — 3,500 SF (325.16 m²)

This family is documented in the V12 Methodology PDF but is currently
absent from `tile-system.dtcg.json` — flagged for backfill.

| Code | Name | Composition | Climate zones |
|---|---|---|---|
| Tile D | Corporate Office Medium | Single 3,500 SF Corporate unit | 1 |
| Tile E | Private Office Medium | 8 PO (300 / 500 / 300 / 450 / 450 / 300 / 300 / 300 ≈ 2,900) + corridor | 8 |
| Tile F (Medium) | Professional Office Medium | PO Small 800 + PO Small 800 + PO Medium 1,100 + PO Small 800 = 3,500 | 4 |
| Tile E-1 (Medium) | Left End Cap (Private) | Mixed PO with end-cap corridor; ≈3,500–5,500 SF including end-cap geometry | 8 |
| Tile E-2 (Medium) | Right End Cap (Professional) | Mirror of Medium E-1 with Professional Office Key Plans | 4–6 |

**Naming collision.** "Tile F" appears at both 3,500 SF (Medium
Professional) and 4,900 SF (Large Corporate) in the V12 Methodology.
The disambiguator is the family label. Tooling should normalise to
`tile-f-medium` and `tile-f-large` to avoid ambiguity in the token
store.

### Large Tiles — 4,900 SF (455.22 m²)

| Code | Name | Composition | Climate zones |
|---|---|---|---|
| Tile F | Large Corporate Office | Single 4,900 SF Corporate unit | 1 |
| Tile G | Large Private Office Mix | 10 PO (300–500 SF each) + corridor | 10 |
| Tile H | Large Professional Office | 5 PO units (800 + 800 + 1,100 + 800 + 1,400 = 4,900) | 5 |

Large tiles approach the 1/4 floor tile (5,000 SF) in scale. They
are useful for Corporate anchors, high-density Private-Office
floors, and Professional Centres that need contiguous Professional
Office sub-types (Medical, Business, Laboratory, Academic, Civic).

### 6,000 SF Paired End Caps

A second End Cap series exists for buildings where the short side
spans two paired Small tiles plus an Emergency Stairwell:

| Code | Name | Use |
|---|---|---|
| Tile A-1 | Corporate Office End Cap | Pair of Corporate Office units + Emergency Stairwell + Internal Corridor |
| Tile B-2 | Private Office End Cap | Pair of Private Office groups + Emergency Stairwell |
| Tile C-3 | Professional Office Medium End Cap | Two PO-Medium units + Emergency Stairwell |
| Tile C-4 | Professional Office Large End Cap | PO-Large 2,400 + PO mix + Emergency Stairwell |

Source: Alternatives PDF p. 2 (May 2025 and January 2026 variants).

## Special Tiles

Special Tiles fill the residual area around the Building Core and
the Elevator Lobby. They are not part of the regular family
catalogue; their sizes are variable and snap to the nearest
Professional Office Key Plan width.

| Code | Approximate sizes | Role |
|---|---|---|
| SP-A | 400 / 1,350 / 2,000 / 2,100 SF | Core-adjacent filler (left and right of Core) |
| SP-B | 300 / 800 / 900 SF | Secondary core-adjacent filler |
| SP-C | 4,700 SF | Elevator Lobby front; constraint: no direct door-to-elevator alignment |

Source: V12 Methodology p. 3 (Sample Small Tile), p. 4 (Sample
Medium Tile), p. 5 (Sample Large Tile).

The Tile algebra (from V1 Combinations PDF) treats Special Tiles as
a distinct composition class:

```
T_Basic     = n × P_Private_Small + p × P_Private_Medium + q × P_Private_Large
T_Compound  = T_Basic + r × P_(Academic | Laboratory | Medical | Business | Civic)
T_Special   = n × PO_Small + p × PO_Medium + q × PO_Large
              + r × P_(Academic | Laboratory | Medical | Business | Civic)
Floor Plate = T_Basic + T_Compound + T_Special + Building Core
```

The equivalence `T_Basic = T_Compound = Corporate Office` is asserted
in the V1 Combinations PDF: a Corporate Office leasehold may be
composed of Basic Tiles, Compound Tiles, or both, summing to the
desired Corporate area.

## Key Plan dimensions

The Key Plans that fill each tile are sourced from the Tear Sheet
"Key Plans and Tiles" (January 2026 V12):

| Class | Small | Medium | Large |
|---|---|---|---|
| Private Office (PO-1 / PO-2 / PO-3) | 300 SF (PO-1) | 450 SF (PO-2) | 500 SF (PO-3) |
| Professional Office — Medical | 1,100 SF (M-1) | 1,400 SF (M-2) | 800 SF (M-3) [Note: M-3 is "Small"] |
| Professional Office — Business | 1,100 SF (B-1) | 1,400 SF (B-2) | 800 SF (B-3) |
| Professional Office — Laboratory | 1,100 SF (L-1) | 1,400 SF (L-2) | 800 SF (L-3) |
| Professional Office — Academic | 1,100 SF (A-1) | 1,400 SF (A-2) | 800 SF (A-3) |
| Professional Office — Civic | 1,100 SF (C-1) | 1,400 SF (C-2) | 800 SF (C-3) |

The 1/4 floor tile (5,000 SF) for any Professional Office sub-type
totals ≈5,200 SF (two M-1/B-1/L-1/A-1/C-1 + one M-2/B-2/L-2/A-2/C-2
+ two M-3/B-3/L-3/A-3/C-3). The 200 SF surplus over the 5,000 SF
reference is absorbed by the floor-plate dimensional range.

**Availability distribution** (Tear Sheet January 2026):

- Private Office: Small 80% / Medium 10% / Large 10%
- Professional Office sub-types: Small 40% / Medium 30% / Large 20%
  (10% other / mixed)

## Open research

- **Q1.** The Tear Sheet "% of availability" for Medical/Business/
  Laboratory/Academic/Civic sums to 90%, not 100%. Confirm the
  missing 10% category (open-plan? unallocated?).
- **Q2.** Medical / Business sub-types have 1,100/1,100/1,400/800/
  800 mix totalling 5,200 SF, not 5,000. Reconcile with the 1/4
  floor tile reference of exactly 5,000 SF.
- **Q3.** Tile B-1 in the token file records "5 private offices
  + 900 SF corridor" but the Alternatives PDF shows 5 PO + a
  Professional Office Small (800 SF). Pick the canonical version.
- **Q4.** Are the 6,000 SF End Caps (A-1, B-2, C-3, C-4) used only
  when the floor-plate length crosses a specific threshold?
  No source document states the trigger condition.

## Future research

- Backfill the Medium family in `tile-system.dtcg.json`.
- Add the Special Tile series (SP-A, SP-B, SP-C) with width-snap
  rules.
- Disambiguate the `tile-f` token into `tile-f-medium` and
  `tile-f-large`.
- Encode the Key Plan availability distributions in a separate
  `tenant-mix.dtcg.json` (currently misfiled in
  `floor-plate-standards.dtcg.json`).

## Dated entries

### 2026-05-17 — initial draft

First-pass draft synthesising the V12 Methodology, V12 Tear Sheet,
and V1 Combinations PDF. Surfaced the Medium family gap in the
current token store. Catalogued the Special Tile series and the
6,000 SF Paired End Cap series. Identified the `tile-f` naming
collision.
