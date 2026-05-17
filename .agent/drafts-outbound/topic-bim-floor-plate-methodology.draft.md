---
schema: foundry-draft-v1
version: "1.0"
draft_id: topic-bim-floor-plate-methodology-2026-05-17
language_protocol: PROSE-TOPIC
state: ready-for-sweep
originating_cluster: project-bim
target_repo: vendor/content-wiki-projects
target_path: topics/bim/floor-plate-methodology.md
audience: operator
bcsc_class: vendor-internal
authored: 2026-05-17T22:00:00Z
title: "Floor Plate Methodology — Key Plans and Tiles"
authored_by: totebox@project-bim
authored_with: claude-opus-4-7
research_done_count: 6
research_suggested_count: 4
open_questions_count: 5
research_provenance:
  - "AEC_Floor Plates_Methodology_V12.pdf (May 06 2025, Collaborators #27) — 13 pages — primary methodology source"
  - "AEC_Floor Plates_Key Plans_Methodology_V12.pdf (March 03 2025, Collaborators #32) — Key Plan construction primer"
  - "PROJECTS_MCorp_Tear Sheet_Floor Plates_Key Plans and Tiles.pdf (January 06 2026, V12) — tile/key-plan composition tear sheet"
  - "PROJECTS_MCorp_Tear Sheet_Floor Plates_Tiles_Combinations.pdf (January 06 2026, V1) — formal algebra + leasing-efficiency chart"
  - "AEC_Floor Plates_Tiles_Alternatives.pdf (May 06 2025, Collaborators #27) + tear-sheet alternates — End Cap tile catalogue"
  - "woodfine-bim-library/tokens/bim/tile-system.dtcg.json, floor-plate-standards.dtcg.json"
research_inline: true
notes_for_editor: |
  Living document — designed to receive additional sections as new
  research lands. Primary audience: Woodfine operators, AEC collaborators,
  and downstream tool builders (tool-floorplates, tool-buildingwidth).
  EN only at draft stage. The slide deck preview/floor-plate-methodology.html
  is the visual companion — keep the prose and the deck synchronised when
  source documents are revised.
---

# Floor Plate Methodology — Key Plans and Tiles

## Working definition

> "Key Plans and Tiles" means a geometric self-similar aperiodic
> space-planning system based on furniture/equipment arrangements
> and circulation versus modular area-per-person progressions.

Source: V12 Methodology (May 2025) and V12 Tear Sheet (January 2026).
The definition is invariant across both versions.

A **Key Plan** is the smallest unit: a labelled rectangle around one
tenant's actual furniture inventory, sized to enclose desks, storage,
circulation, and accessibility. A **Tile** is a composable group of
Key Plans. A **Floor Plate** is the assembly of Tiles plus a Building
Core plus Special Tiles. Nothing on the floor is left unrepresented.

Three properties qualify the system:

- **Aperiodic** — Tiles do not repeat on a fixed grid.
- **Self-similar** — each Tile is exactly one HVAC climate zone,
  regardless of size.
- **Geometric** — all dimensions resolve to real metric measurements
  driven by real furniture SKUs (Steelcase, Midmark, Treston, Agati).

## The three tile families

| Family | Area | m² | Role |
|---|---|---|---|
| Small | 2,700 SF | 250.84 | Most flexible; smallest unit of climate-zone autonomy |
| Medium | 3,500 SF | 325.16 | Bridge family; absorbs short-side geometry |
| Large | 4,900 SF | 455.22 | Corporate anchor; approaches 1/4 floor tile |

Source: V12 Methodology p. 2. The Medium family is documented in the
May 2025 V12 Methodology PDF but is absent from the current
`tile-system.dtcg.json` — flagged as a token-store gap.

## Tile composition rules

These rules govern how Tiles compose into a valid Floor Plate.
They are encoded as `FP-*` validators in the planned
`tool-floorplates` Rust crate. The narrative source is the
V12 Methodology PDF; the formal algebra is the V1 Combinations PDF.

### FP-SUM — every square foot is represented

The aggregate of Basic Tiles + Compound Tiles + Special Tiles + the
Building Core sums to the full Net Leasable Area within a ±100 SF
tolerance. No "free space" exists without a corresponding Key Plan.

### FP-ENDCAP — short sides receive natural light

The two ends of the building use End Cap tiles:

- Small-family End Caps: Tile B-1 (Private 2,700 SF), Tile E-1
  (Mixed Left 2,700 SF), Tile E-2 (Mixed Right 2,700 SF).
- 6,000 SF paired End Caps: Tile A-1 (Corporate), Tile B-2 (Private),
  Tile C-3 (Professional Medium), Tile C-4 (Professional Large).
- Medium-family End Caps: Tile E-1 (Private Office Medium variant)
  and Tile E-2 (Professional Office Medium variant), each ≈3,500–
  5,500 SF.

End Caps must receive natural light on both perpendicular axes; an
End Cap with no window on one axis is "not 100% efficient" and
triggers a window-bay revision (V12 Methodology p. 8, Sample #3/#4).

### FP-CORE — Offset Pulled Back Core

The Building Core (Elevators, Service Stairs, Emergency Stairs,
Restrooms, Meter Room, Mop Room) is positioned at least 18 m from
the short end of the building. This leaves enough plate length for
two End Cap tiles plus at least one Basic Tile on each side of the
core.

### FP-SNAP — Special Tile width matches Key-Plan width

Special Tiles SP-A, SP-B, SP-C fill the residual area surrounding
the Core. Their width is snapped to the nearest Professional Office
Key Plan width (e.g. if a PO is 450 SF and SP-A is 400 SF, SP-A is
increased by 50 SF to match). This preserves demising-wall
continuity. SP-C, in front of the Core, must avoid direct alignment
of any door with the elevator opening (V12 Methodology p. 9).

### FP-CLIMATE — one tile, one HVAC zone

Each Tile is an independent climate zone. Tenants who combine Tiles
combine climate-control authority; the leasing agreement prices this
trade-off explicitly. Smaller tiles increase tenant autonomy
(more thermostats per square foot); larger tiles reduce zone count
in favour of contiguous space.

### FP-DOORS — door count per tile / per floor

Each tile may carry up to 10 doors; each floor may carry up to 80
doors. The door count is one of the cost drivers in the leasing
economics — every door is also a Service Hookup (electric, data,
HVAC tap), enumerated alongside the climate-zone count.

### FP-CORNER — small tiles at corners trigger structural review

When a Small-family tile lands at a structural corner, the column
grid is reviewed before the tile is finalised. Small tiles may
"box out" awkwardly against the structural module; the core
position is adjusted if the short-side tiles conflict
(V12 Methodology p. 2, Inventory of Tiles note).

## Floor-plate dimensional range

The 20,000 SF reference plate is a midpoint, not a constraint.

| Class | Net leasable per floor | Floor count |
|---|---|---|
| Professional Centres | 19,000 – 23,000 SF | 3 – 5 floors |
| Suburban Office | 17,000 – 21,000 SF | 6 – 9 floors |
| Retail Select | 4,500 / 6,700 / 7,700 SF | varies |
| Tech Industrial | 7,200 / 8,400 SF | varies |

Source: V1 Combinations PDF p. 2 (formal algebra). The fleet
deployment templates published in `customer/woodfine-fleet-deployment/`
must reflect these ranges, not the singular 20,000 SF figure
currently encoded in `floor-plate-standards.dtcg.json`.

## Why "geometric self-similar aperiodic" matters

The phrase is unfamiliar; the consequences are concrete.

- **Geometric** — every dimension cites a furniture SKU + a
  regulatory clearance, not a developer rule of thumb. The
  Building Width Calculator derives the plate width from
  Zone 1 Habitat + Zone 2 Magazine + Zone 3 Corridor depths,
  each rooted in real manufacturer dimensions.
- **Self-similar** — a Tile at 2,700 SF is the same kind of object
  as a Tile at 4,900 SF: one climate zone, one leasable instrument.
  Tenants experience tiles consistently regardless of which family
  they occupy.
- **Aperiodic** — tiles do not repeat on a fixed grid. A floor can
  carry a different tile sequence on every level; a Mixed
  Professional Centre next to a Private-Office-heavy plate next to
  a Corporate floor are all valid compositions of the same
  catalogue.

The architectural payoff is that the building does not look
"engineered". Repeating tile grids produce monotone facades and
predictable interiors; an aperiodic composition with consistent
self-similar units produces variation without losing legibility.

## Floor-plate composition example (Sample Small Tile)

From V12 Methodology p. 4:

- Tile A (Corporate Office 2,700) ×2 left of left-emergency-stairs
- Tile A (Corporate Office 2,700) ×2 between emergency stairs
- Special Tile A (400 SF) — core-adjacent left
- Building Core (Core + Service Stairs)
- Special Tile B (300 SF) — core-adjacent right
- Tile A (Corporate Office 2,700) — right of core
- Tile A ×2 (5,400 SF combined) at right end

Basic Tiles: 18,900 SF. Special Tiles: 1,100 SF. Total: 20,000 SF.

## Open research

- **Q1.** The token store currently lists E-1 and E-2 as 2,700 SF
  Small End Caps. The V12 Methodology shows them as Medium End Caps
  at ~3,500–5,500 SF. Which convention is authoritative? Both?
- **Q2.** SP-C is documented at 4,700 SF in front of the Core.
  How does it interact with the Elevator Lobby for compositions
  using Medium tiles where the lobby occupies the SP-C footprint?
- **Q3.** What is the exact derivation of the 18 m minimum
  core-to-short-end distance? Inferred from end-cap + first-tile
  width, but no PDF cites the figure directly.
- **Q4.** The Tile B-1 token records 1,800 SF office + 900 SF
  corridor. The May Methodology shows 5 PO modules summing to 1,850
  SF + 850 SF residual. Which composition is current?
- **Q5.** The 100 SF tolerance in FP-SUM is operator policy, not
  source-document text. Confirm the tolerance band with the
  Methodology author before encoding.

## Future research

- Encode FP-* rules as `floor-plate-assembly-rules.dtcg.json` so
  `tool-floorplates` can validate compositions programmatically.
- Backfill `tile-system.dtcg.json` with the Medium family
  (Tile D / Tile E / Tile F-medium) and the Special Tile series
  (SP-A / SP-B / SP-C).
- Reconcile the 2,700 SF E-1/E-2 vs. 3,500/5,500 SF E-1/E-2 with
  the methodology author; pick a canonical convention.
- Cross-link this topic with the Building Width Calculator topic
  and the Climate Zone topic when those drafts ratify.

## Dated entries

### 2026-05-17 — initial draft

First-pass draft synthesising V12 Methodology (May 2025), V12
Tear Sheet (January 2026), V1 Combinations PDF (January 2026), and
the existing token files. Surfaced six tile-composition rules and
the PC 19–23 / SU 17–21 floor-plate range. Five open questions
recorded above. Slide deck `preview/floor-plate-methodology.html`
updated in parallel.
