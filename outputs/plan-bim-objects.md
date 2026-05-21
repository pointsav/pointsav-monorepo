---
schema: foundry-plan-v1
archive: project-bim
topic: bim-objects
status: draft-v2
created: 2026-05-20
updated: 2026-05-21
sources: |
  inputs/plan-bim-objects (operator notes)
  inputs/DISCOVERY_MCorp_Sketches_Key Plans_Summary.pdf
  inputs/DISCOVERY_MCorp_Sketches_Key Plans_Summary_Notes.pdf
  inputs/DISCOVERY_MCorp_Sketches_Key Plans_Business_Notes.pdf
  inputs/PROJECTS_MCorp_Design Slides_Key Plans_Sample.pdf
  inputs/PROJECTS_MCorp_Database_Floor Plates_Key Plans_Index.pdf
  inputs/CONSTRUCTION_MCorp_2026_01_06_Database_Floor Plans_Key Plans_Index_FIN.xlsx
  inputs/PROJECTS_MCorp_Tear Sheet_Floor Plates_Tiles_Combinations.pdf
  inputs/PROJECTS_MCorp_Tear Sheet_Floor Plates_Tiles_Alternatives.pdf
  inputs/CONSTRUCTION_2026_01_06_Tear Sheet_Floor Plates_Tiles_Alternatives_V2_FIN.xlsx
  inputs/CONSTRUCTION_MCorp_2026_01_06_Tiles_Leasing Plan Efficiencies_FIN.docx
  inputs/CONSTRUCTION_2026_01_06_Tear Sheet_Floor Plans_Key Plans_Methodology_FIN.xlsx
  inputs/PROJECTS_MCorp_Tear Sheet_Floor Plates_Key Plans_Methodology.pdf
  inputs/building-width-calculator.docx
  inputs/CONSTRUCTION_2025_10_31_Design Slides_Openstudio_Woodfine Response.docx
  inputs/--- April 01, 2025 -- Collaborators #11 ---/ (4 files)
  inputs/--- March 03, 2025 -- Collaborators #32 ---/ (3 files)
  inputs/--- May 06, 2025 -- Collaborators #27 ---/ (5 files)
  CONSTRUCTION_2026_01_06_Key Plan_Professional Office_FFE_FIN.xlsx
    tabs: Summary_Key Plans, Summary_V3, Summary_Retail and Tech Industr,
          Summary_Common Areas, Summary_Amenities, Loading and Recycling,
          Upper Floors_Building Core, Public Lobby Washroom,
          Building Manager + Mail Room
operator_answers:
  Q1: "Professional Centre Key Plans = 13 (confirmed)"
  Q2: "Sizes from FIN.xlsx Summary_Key Plans tab — authoritative"
  Q3: "Corporate Office = no dimensions yet; sized against Floor Plate at the end"
  Q4: "Landscaping eco-region variants deferred"
  Q5: "Retail Select and Tech Industrial — Key Plans compose directly into Floor Plates (no Tile layer)"
  Q6: "DTCG-to-Rust linkage = open; decide at Rust engine scaffold time"
---

# Plan — BIM Objects: Key Plans, Tiles, Floor Plates

> **Draft v2 — updated 2026-05-21.** Authoritative sizes from FIN.xlsx folded in.
> Q1–Q6 resolved. Command Decisions 1–4 incorporated.
> This document is the working specification for the BIM Object Library.

---

## Context

Woodfine Management Corp. designs and builds commercial real estate across
regional markets in Western Canada (car-dependent, adjacent to Power Centres).
The building portfolio spans four development classes: Professional Centres,
Suburban Office, Retail Select, and Tech Industrial.

The core design philosophy inverts conventional space planning. Rather than
starting from a per-person square footage target, each space is designed from
the occupant outward: real manufacturer furniture (Steelcase for offices,
Midmark for medical) is placed first, circulation compliance (German
Circulation Law, European Lighting Standards, wheelchair accessibility) is
enforced second, and the building dimensions follow from the resulting spatial
requirements. This produces a **non-modular** system — Private Office Small
is not 50% of Private Office Medium — with a consistent three-zone structure
across every Key Plan.

The digital artifact of this system is the **BIM Object Library**: a catalog
of BIM Objects organized as Key Plans → Tiles → Floor Plates. Architects
deliver Key Plans; the `tool-buildingwidth` Rust engine computes how they
nest into Tiles, and how Tiles nest into Floor Plates.

---

## Command Decisions (received 2026-05-20)

Four decisions received via inbox `command-20260520-bim-foundation-decisions`.
These apply to all work in this archive going forward.

**Decision 1 — Naming convention**
Use **descriptive display names** (Index PDF style) as labels on `bim.woodfinegroup.com`.
Codes (PO-1, M-1, B-1, etc.) are internal-only keys in DTCG JSON — not shown to users.
- Private Office: size IS the descriptor → "Private Office — Small / Medium / Large"
- Medical: specialisation IS the descriptor → "Medical — Chiropractor / Dentist / GP"
- Business and others: follow the Index PDF descriptive pattern
- Throughout all user-facing copy: **BIM Objects** (not "tokens")

**Decision 2 — HTML BIM_TOKENS block**
**Delete** the inline `BIM_TOKENS` block from `building-width-calculator.html`.
The page must fetch values directly from DTCG JSON files at render time.
Single source of truth — no manual sync.

**Decision 3 — Scope**
All three building types are in scope now:
- Professional Centre (offices + Medical + Business + common areas)
- Retail Select (6 tiles: A-RS through M-RS)
- Tech Industrial (5 tiles: A-TI through M-TI)
- All 12 Professional Centre common-area Key Plans

**Decision 4 — Tile disambiguation**
- Type-prefixed tile codes as internal keys: Corporate Office → CO-A/B/C; Retail Select → RS-A/B/C; Tech Industrial → TI-A/B/C
- Corridor Expander T = **300 SF** (diagram value is operative)
- Arithmetic gaps between tile row sums and target sizes are **intentional by design** — `tool-buildingwidth` manages Tile/Key-Plan/Floor-Plate size negotiation at build time; add `$description` note to DTCG entries
- J/K/L/M tiles: create **stub DTCG entries** with `status: reserved` and note source not yet located

---

## Four Development Classes

| Class | Floors | Floor Plate (SF) | Notes |
|---|---|---|---|
| Professional Centres | 3–5 | 19,000–25,000 | 3 elevators; offset pulled-back core |
| Suburban Office | 6–9 | 17,000–23,000 | 4 elevators; offset pulled-back core |
| Retail Select | 1 | 4,500 / 6,700 / 7,700 | Strip retail; S/M/L; Key Plans → Floor Plates directly |
| Tech Industrial | 1 | 7,200 / 8,400 | Industrial flex; M/L; Key Plans → Floor Plates directly |

**Offset Pulled-Back Core** (Professional Centres + Suburban Office):
- Building core pushed toward rear, not centered
- Enables full-length perimeter corridor without internal tenant corridors
- Supports adaptive reuse (residential, warehouse) without major reconfiguration

---

## Three-Zone Framework

Every Key Plan (except pure-circulation and infrastructure types) has this
cross-section from facade to core:

```
[FACADE]  Zone 1 — Habitat  |  Zone 2 — Magazine  |  Zone 3 — Corridor  [CORE]
```

| Zone | Purpose | Depth range | Standard |
|---|---|---|---|
| Zone 1 — Habitat | All desks/workstations; natural light required | 4.7m – 7.2m | European Lighting Standard (6m desk-to-window rule) |
| Zone 2 — Magazine | Storage, secondary functions; adjustable | 1.3m – 9.3m | Derived from furniture layout; `tool-buildingwidth` varies this to hit Tile targets |
| Zone 3 — Corridor | Central circulation running floor length | 2.0m – 3.1m | ~20% oversized from code minimum for wellness |

Zone 1 depth drives building width per Key Plan type. Zone 2 is the adjustable
layer. Zone 3 is consistent within a category.

---

## Part 1 — Key Plans

### Definition

A Key Plan is the smallest BIM Object unit. Defined by:
- Furniture arrangement (specific manufacturer SKUs)
- Zone 1/2/3 dimensions
- Net leasable area (m² and SF)
- Facade frontage
- Accessibility compliance
- Occupancy count

Key Plans are authored by architects from Woodfine's equipment programs and
zone requirements. Infrastructure Key Plans (Lobby Atrium, Emergency Stairwell,
etc.) are self-contained — they appear directly in Floor Plates without a
Tile wrapper.

All sizes below are from `CONSTRUCTION_2026_01_06_Key Plan_Professional
Office_FFE_FIN.xlsx` Tab `Summary_Key Plans` (V3, 2025-11-29), the
operator-designated authoritative source. Spreadsheet note: "NB — Not drawn
to scale — Approximate square footages." Zone depth columns in raw extraction
were offset by one; corrected by cross-reference with PDF samples.

---

### Key Plan Registry

#### 1. Private Office (3)

Facade-facing; individual lease; Building ID issued per desk.

| Code | Display name | m² | SF | Z1 (m) | Z2 (m) | Z3 (m) |
|---|---|---|---|---|---|---|
| PO-1 | Private Office — Small | 30.19 | 325 | 6.0 | 3.8 | 2.0 |
| PO-2 | Private Office — Medium | 43.20 | 465 | 6.0 | 3.8 | 2.0 |
| PO-3 | Private Office — Large | 63.64 | 685 | 6.0 | 3.8 | 2.0 |

Furniture (PO-1): desk + chair, 3-person round table, 2 side chairs, filing
cabinet, credenza, bookshelf, coat rack.
Note: Zone 2 = 3.8m across all sizes; frontage width increases, not depth.
Desks perpendicular to facade require +0.7m in Zone 1 for three desks in series.

---

#### 2. Corporate Office (5)

Full Tile; no sub-lease; one climate zone per Tile. **Dimensions TBD** —
sized as a proportion of the Floor Plate (Q3 resolution). Corporate Office
is the minimum Tile size for avoiding exposed columns in smaller tenancies.

| Code | Display name | m² | SF |
|---|---|---|---|
| CO-FF | Corporate Office — Full Floor | TBD | TBD |
| CO-1/2 | Corporate Office — Half Floor | TBD | TBD |
| CO-1/3 | Corporate Office — Third Floor | TBD | TBD |
| CO-1/4 | Corporate Office — Quarter Floor | TBD | TBD |
| CO-1/8 | Corporate Office — Eighth Floor | TBD | TBD |

Tile codes (Decision 4): CO-A, CO-B, CO-C, CO-D, CO-E (type-prefixed).

---

#### 3. Professional Office — Medical (3)

| Code | Display name | m² | SF | Z1 (m) | Z2 (m) | Z3 (m) |
|---|---|---|---|---|---|---|
| M-1 | Medical — Chiropractor | 223 | 2,401 | 7.20 | 4.87 | TBD |
| M-2 | Medical — Dentist | 331 | 3,568 | 7.20 | 4.87 | TBD |
| M-3 | Medical — General Practitioner | 486 | 5,231 | 7.20 | 4.87 | TBD |

Key rooms: reception, exam/dental chairs (2/4/6), file room, autoclave,
sterilization, imaging, storage, washroom.

---

#### 4. Professional Office — Business (3)

| Code | Display name | m² | SF | Z1 (m) | Z2 (m) | Z3 (m) |
|---|---|---|---|---|---|---|
| B-1 | Business — Small | 311.22 | 3,350 | 6.00 | 7.30 | 2.75 |
| B-2 | Business — Medium | 399.66 | 4,301 | 6.00 | 7.30 | 2.75 |
| B-3 | Business — Large | 669.00 | 7,201 | 6.00 | 7.30 | 2.90 |

Width options A–D available per size; Zone 2 adjustable up to 7.30m max.
Total facade frontage range: 25.3m – 32.3m.

---

#### 5. Professional Office — Laboratory (3)

| Code | Display name | m² | SF | Z1 (m) | Z2 (m) | Z3 (m) | Frontage |
|---|---|---|---|---|---|---|---|
| L-1 | Laboratory — Medical | 195.00 | 2,099 | 6.78 | 4.80 | 3.10 | ~16.8m |
| L-2 | Laboratory — Research | 315.96 | 3,401 | 6.78 | 4.80 | 3.10 | ~27.3m |
| L-3 | Laboratory — Large | 400.69 | 4,313 | 6.78 | 4.80 | 3.10 | ~34.6m |

Key rooms: reception, 1–2 offices, 3–9 benches, staff room, storage, mechanical, clean room.
Occupancy: L-1=34 (5.7m²/pp), L-2=49 (6.4m²/pp), L-3=61 (6.5m²/pp).

---

#### 6. Professional Office — Academic (3)

| Code | Display name | m² | SF | Z1 (m) | Z2 (m) | Z3 (m) |
|---|---|---|---|---|---|---|
| A-1 | Academic — Small | 105 | 1,131 | 4.70 | 3.00 | TBD |
| A-2 | Academic — Medium | 240 | 2,583 | 4.70 | 3.00 | TBD |
| A-3 | Academic — Large | 378 | 4,070 | 4.70 | 3.00 | TBD |

Configurations: seminar/podium (A-1), multi-use seminar (A-2), tiered
auditorium ~60 capacity (A-3).
Occupancy: A-1=21–25, A-2=63, A-3=82–103.

---

#### 7. Professional Office — Civic (3)

| Code | Display name | m² | SF | Z1 (m) | Z2 (m) | Z3 (m) |
|---|---|---|---|---|---|---|
| C-1 | Civic — Small | 270 | 2,912 | 6.00 | 7.23 | TBD |
| C-2 | Civic — Medium | 577 | 6,215 | 6.00 | 7.23 | TBD |
| C-3 | Civic — Large | 822 | 8,850 | 6.00 | 7.23 | TBD |

Key rooms: offices, clerk desks, conference rooms, staff room, restrooms, communal corridor.

---

#### 8. Circulation & Utility (2)

| Code | Display name | m² | Notes |
|---|---|---|---|
| R-1 | Corridor Expanders | variable | Fills remainder gaps; width adjustable |
| T | Corridor Expander T | 300 SF | Decision 4: 300 SF is operative value |
| V-1 | Meter Room | TBD | Utility; no Zone 1 requirement |

---

#### 9. Professional Centre Infrastructure (13)

Sizes TBD — architect delivers drawings from the equipment programs below.
These Key Plans appear directly in Floor Plates (not wrapped in Tiles).

| Code | Display name | Sizes | Equipment program |
|---|---|---|---|
| N-1 | Tenant Lounge | S/M/L | Seating, kitchenette |
| EE-1 | Lobby Atrium | S/M/L | Entry, reception |
| O-1 | Building Manager Office | M/L | Manager desk+table, asst. manager desk, mail supervisor desk, night watch desk, 2–4 mail charts, 6–12 shelves, first aid |
| P-1 | Mail Room | (part of O-1) | Combined Key Plan tab in FIN.xlsx; architect to confirm split or merge |
| S-1 | Elevator Lobby | M/L | 3 (M) / 4 (L) elevator cabs, service staircase |
| U-1 | Tenant Restroom | M/L | 2 shared sinks, 2 stalls each gender, mop room |
| X-1 | Loading | M/L | 1–2 grad doors, 1–2 service doors, 1–2 refuse bins + compactors |
| Y-1 | Recycling | M/L | Paper / plastic / glass bins (1–2 each) |
| Z-1 | Bike Room | M/L | 12 (M) / 24 (L) bike racks, exterior door |
| AA-1 | Workbench | M/L | 1 (M) / 2 (L) workbench units |
| BB-1 | Building Staff Lockers | M/L | 5 (M) / 10 (L) lockers, 2–4 showers |
| CC-1 | Coffee / Bread | S/M/L | Kitchenette |
| DD-1 | Public Restrooms | M/L | 2–3 shared sinks, 2–4 stalls each gender, 1 family/maternal/accessible room |

*Q1 resolved: 13 entries confirmed.*

---

#### 10. Suburban Office Infrastructure (14)

Mirror of Professional Centre set with `-2` suffix. Adds Mop Room (W-2).
FIN.xlsx note: "Suburban Office doesn't need additional washroom stalls —
the Floor Plates are smaller, so each floor may have the same occupancy or less."

| Code | Display name |
|---|---|
| N-2 | Tenant Lounge |
| EE-2 | Lobby Atrium |
| O-2 | Building Manager Office |
| P-2 | Mail Room |
| S-2 | Elevator Lobby |
| U-2 | Tenant Restroom |
| W-2 | Mop Room |
| X-2 | Loading |
| Y-2 | Recycling |
| Z-2 | Bike Room |
| AA-2 | Workbench |
| BB-2 | Building Staff Lockers |
| CC-2 | Coffee / Bread |
| DD-2 | Public Restrooms |

---

#### 11. Retail Select Key Plans (3)

**Q5 resolved:** RS Key Plans compose directly into Floor Plates — no Tile layer.

Floor Plate composition: Left End Cap + Mechanical Room + n(RA/RB/RC) + Right End Cap.
Floor Plate targets: Small=4,500 SF, Medium=6,700 SF, Large=7,700 SF.
Zone depths (RS): Z1=6.0m, Z2=3.8m, Z3=2.0m.

| Code | Display name | m² | SF |
|---|---|---|---|
| RA-1 | Retail Leasehold — Small | TBD | TBD |
| RB-2 | Retail Leasehold — Medium | TBD | TBD |
| RC-3 | Retail Leasehold — Large | TBD | TBD |

Also in RS Floor Plates (non-leasehold Key Plans): End Cap S/M, Mechanical Room, Spacer M/L — sizes TBD.
Tile codes (Decision 4): RS-A through RS-M (type-prefixed).

---

#### 12. Tech Industrial Key Plans (3)

**Q5 resolved:** TI Key Plans compose directly into Floor Plates — no Tile layer.

Floor Plate composition: Left End Cap + Mechanical Room + n(TI-2/TI-3) + Right End Cap.
Floor Plate targets: Medium=7,200 SF, Large=8,400 SF.
Zone depths (TI): Z1=6.0m, Z2=3.8m, Z3=2.0m.

| Code | Display name | m² | SF |
|---|---|---|---|
| TI-1 | Tech Leasehold — Small | TBD | TBD |
| TI-2 | Tech Leasehold — Medium | TBD | TBD |
| TI-3 | Tech Leasehold — Large | TBD | TBD |

Also in TI Floor Plates: End Cap M/L, Mechanical Room, Spacers M/L — sizes TBD.
Tile codes (Decision 4): TI-A through TI-M (type-prefixed).

---

#### 13. Landscaping (2 active — eco-region variants deferred)

*Q4 resolved: Boreal Plains / Fescue Grassland / Parkland Natural variants deferred to later iteration.*

| Code | Display name | m² |
|---|---|---|
| LL-1 | Bioswales | TBD |
| LL-2 | Irrigation Gallery | TBD |

---

#### 14. Parking (6 active — eco-region variants deferred)

*Q4 resolved: eco-region variants for PP-1 + PP-2 deferred.*

| Code | Display name | m² |
|---|---|---|
| PP-1 | Parking Stalls | TBD |
| PP-2 | Accessible Parking | TBD |
| PP-3 | Sidewalks | TBD |
| PP-4 | Snowdrops | TBD |
| PP-5 | Signage | TBD |
| PP-6 | Lighting | TBD |

---

### Key Plan count summary

| Category | Count | Sizes confirmed | Sizes TBD |
|---|---|---|---|
| Private Office | 3 | 3 | — |
| Corporate Office | 5 | — | 5 (floor-plate dependent) |
| Medical | 3 | 3 | — |
| Business | 3 | 3 | — |
| Laboratory | 3 | 3 | — |
| Academic | 3 | 3 | — |
| Civic | 3 | 3 | — |
| Circulation & Utility | 3 | — | 3 |
| Professional Centre Infrastructure | 13 | — | 13 (architect drawings) |
| Suburban Office Infrastructure | 14 | — | 14 (architect drawings) |
| Retail Select | 3 (+3 components) | — | 6 |
| Tech Industrial | 3 (+3 components) | — | 6 |
| Landscaping | 2 | — | 2 |
| Parking | 6 | — | 6 |
| **Total active** | **66** | **15** | **51** |

*9 eco-region variants deferred (LL-1 ×3, PP-1 ×3, PP-2 ×3); will bring total to 75 when added.*

---

## Part 2 — Tiles

### Definition

A Tile is a BIM Object composed of one or more Key Plans within a single
**Climate Zone**. One Tile = one climate zone = one HVAC control boundary.

Standard sizes: **2,700 SF** (base module) and **6,000 SF** (end-cap module).

**Decision 4 note:** arithmetic gaps between tile row sums and target SF are
intentional — `tool-buildingwidth` negotiates the trade-off at build time.
DTCG entries carry a `$description` noting this.

### Tile Composition Rules

```
T_Basic    = n(PO-1) + p(PO-2) + q(PO-3)
T_Compound = n(PO-1) + p(PO-2) + q(PO-3) + r(A-* | L-* | M-* | B-* | C-*)
T_Special  = same as Compound, occupies corner / end-cap position in Floor Plate
```

Self-contained Key Plans (Lobby Atrium EE-1, Emergency Stairwell) appear
directly in the Floor Plate — not wrapped in a Tile.

### Tile Registry

#### 2,700 SF Tiles (Professional Centre + Suburban Office)

| Internal code | Display name | Composition | Climate zones |
|---|---|---|---|
| CO-A | Corporate Office | Corporate Office (full tile) | 1 |
| B-1 | Private Mix | PO-1 + PO-2 + PO-3 + partial B-* | 1 |
| C-1 | Professional Medium | B-2 + PO-2 + PO-1 | 1 |
| C-2 | Professional Large | B-3 + PO-1 | 1 |

#### 6,000 SF End-Cap Tiles

| Internal code | Display name | Composition | Notes |
|---|---|---|---|
| CO-A-EC | Corporate End-Cap | Corporate Office (6,000 SF) | Emergency Stairwell + Corridor E |
| B-2-EC | Private Mix End-Cap | PO-1 + PO-2 ×2 + PO-3 | Corner position |
| C-3-EC | Prof Medium End-Cap | B-2 ×2 + PO-2 | Corridors D+E |
| C-4-EC | Prof Large End-Cap | B-3 + PO-3 + PO-2 + PO-1 | Corridors D+E |

#### Stub Tiles (Decision 4 — reserved)

| Internal code | Status | Note |
|---|---|---|
| J | reserved | Source document referenced in Tiles PDF p.3 footnote not yet located |
| K | reserved | Same — values withheld pending V13 or source confirmation |
| L | reserved | Same |
| M | reserved | Same |

#### Retail Select Tiles — RS-A through RS-M (Decision 4)
#### Tech Industrial Tiles — TI-A through TI-M (Decision 4)

Detailed composition TBD when RS/TI Key Plan sizes are confirmed.

### Climate Zone Identity

One Tile = one climate zone = one HVAC boundary. For residential conversion,
each Tile maps naturally to a residential suite HVAC zone.

---

## Part 3 — Floor Plates

### Floor Plate Matrix (9 types)

| ID | Name | Development class | Target SF | Floors |
|---|---|---|---|---|
| FP-PC-M | Main Floor — Professional Centres | Professional Centres | 19,000–25,000 | Ground |
| FP-SO-M | Main Floor — Suburban Office | Suburban Office | 17,000–23,000 | Ground |
| FP-PC-TL | Tenant Lounge Floor — Professional Centres | Professional Centres | 19,000–25,000 | 2 |
| FP-SO-TL | Tenant Lounge Floor — Suburban Office | Suburban Office | 17,000–23,000 | 2 |
| FP-TI-M | Medium Floor Plate — Tech Industrial | Tech Industrial | 7,200 | 1 |
| FP-TI-L | Large Floor Plate — Tech Industrial | Tech Industrial | 8,400 | 1 |
| FP-RS-S | Small Floor Plate — Retail Select | Retail Select | 4,500 | 1 |
| FP-RS-M | Medium Floor Plate — Retail Select | Retail Select | 6,700 | 1 |
| FP-RS-L | Large Floor Plate — Retail Select | Retail Select | 7,700 | 1 |

### Composition Rules

**Professional Centres + Suburban Office:**
```
Floor Plate = T_Basic + T_Compound + T_Special
            + Building Core + Infrastructure Key Plans (EE, N, S, U, staircases…)
```

**Retail Select:**
```
Floor Plate = Left End Cap + Mechanical Room + n(RA | RB | RC) + Right End Cap
```

**Tech Industrial:**
```
Floor Plate = Left End Cap + Mechanical Room + n(TI-2 | TI-3) + Right End Cap
```

### The "Ring"

The "ring" is the structural and circulation annulus around the building core.
It defines the separation between leasing Tiles and the core, the corridor
routing to elevators and stairs, and the minimum Tile depth. The Rust engine
accounts for ring geometry when computing remainder-free nesting.

### Bi-Directional Solver

`tool-buildingwidth` solves two directions:

**Forward:** Key Plan dimensions + Tile types → valid Floor Plate sizes.

**Reverse:** Fixed Floor Plate + fixed Key Plan sizes → Tile adjustments
(Zone 2 depth variation + R-1/T Corridor Expander insertion) that eliminate
remainder. If no standard Tile achieves zero remainder: output required
Special Tile dimensions for architect.

---

## Part 4 — Building Width Calculator

### Purpose

Determines total building width (facade to core) for a given Key Plan / Tile combination.

**Inputs:** Key Plan type(s) + count, Tile composition, Floor Plate target SF
**Outputs:** Z1 required, Z2 options (A/B/C/D), Z3 width, total building width, remainder

### Width data by Key Plan type

| Type | Z1 (m) | Z2 (m) | Z3 (m) | Notes |
|---|---|---|---|---|
| Private Office (all) | 6.0 | 3.8 | 2.0 | Z2 same across PO-1/2/3; frontage changes |
| Medical (all) | 7.20 | 4.87 | TBD | |
| Business S/M | 6.00 | 7.30 | 2.75 | Width options A–D; total 25.3–32.3m |
| Business L | 6.00 | 7.30 | 2.90 | |
| Laboratory (all) | 6.78 | 4.80 | 3.10 | Frontage: L-1=16.8m, L-2=27.3m, L-3=34.6m |
| Academic (all) | 4.70 | 3.00 | TBD | |
| Civic (all) | 6.00 | 7.23 | TBD | |
| RS + TI | 6.0 | 3.8 | 2.0 | Same as Private Office standard |

---

## Rust Engine — `tool-buildingwidth`

New crate at `pointsav-monorepo/tool-buildingwidth/` (not yet scaffolded).

### Responsibilities

1. Key Plan → Tile nesting (compute valid 2,700 SF / 6,000 SF compositions)
2. Tile → Floor Plate nesting (accounting for Building Core + ring)
3. Bi-directional solver (forward + reverse as above)
4. Special Tile detection (when no standard Tile achieves zero remainder)
5. Corridor Expander insertion (R-1 + T = 300 SF) before declaring Special Tile

### Proposed data model

```rust
struct KeyPlan {
    code: String,
    display_name: String,        // Decision 1: descriptive; codes are internal-only
    category: KeyPlanCategory,
    area_m2: f64,
    area_sf: f64,
    zone1_depth_m: f64,
    zone2_depth_m: f64,
    zone3_depth_m: f64,
    facade_frontage_m: f64,
    is_self_contained: bool,     // true → direct Floor Plate placement, no Tile
}

struct Tile {
    code: String,                // CO-A, RS-B, TI-C, etc. (Decision 4: type-prefixed)
    tile_type: TileType,         // Basic | Compound | Special | EndCap | Reserved
    area_sf: f64,
    key_plans: Vec<(String, u32)>,
    climate_zones: u32,
    doors: u32,
    service_hookups: u32,
    arithmetic_gap_note: Option<String>, // Decision 4: intentional gap explanation
}

struct FloorPlate {
    code: String,
    development_class: DevelopmentClass,
    area_sf_min: f64,
    area_sf_max: f64,
    tiles: Vec<String>,
    infrastructure_key_plans: Vec<String>,
    has_ring: bool,
}
```

### DTCG relationship

`building-width-calculator.dtcg.json` holds Zone depth constants per Key Plan type.
The Rust engine reads these at runtime as its configuration input (Q6: linkage pattern
decided at scaffold time — runtime reading preferred per `app-orchestration-bim` pattern).

---

## Deliverables

### Deliverable 1 — Key Plans Registry ← READY TO IMPLEMENT

**Output:** `woodfine-bim-library/key-plans/key-plans-registry.md`
**Format:** Markdown table (human-verifiable; consistent with SYS-ADR-07)
**Content:** Full Key Plan registry from this document's Part 1 section
**Also copy to:** `outputs/plan-bim-objects.md` → accessible via `fpull bim outputs/`
**Commit:** `woodfine-bim-library` sub-clone via `commit-as-next.sh`

Verification:
- Private Office sizes: 30.19 / 43.20 / 63.64 m² (FIN.xlsx Summary_Key Plans)
- Medical sizes: 223 / 331 / 486 m² (Q2 resolved)
- Laboratory Z1=6.78m, Z2=4.80m, Z3=3.10m
- Corporate Office rows show TBD with floor-plate-dependent explanation
- RS/TI rows note "composes directly into Floor Plates — no Tile layer"

### Deliverable 2 — Tiles Registry

**Output:** `woodfine-bim-library/tiles/tiles-registry.md`
Structured list of all Tiles with Decision 4 type-prefixed codes, composition rules,
stub entries for J/K/L/M with `status: reserved`.
**Verify against:** `CONSTRUCTION_2026_01_06_Tear Sheet_Floor Plates_Tiles_Alternatives_V2_FIN.xlsx`

### Deliverable 3 — Decision 2: Delete BIM_TOKENS block

**File:** `preview/building-width-calculator.html`
Delete the inline BIM_TOKENS block; wire DTCG JSON fetch at render time.

### Deliverable 4 — Rust Engine: `tool-buildingwidth`

New crate at `pointsav-monorepo/tool-buildingwidth/`.
Depends on: Deliverable 1 + 2 complete; DTCG files up to date.

---

## Resolved questions

| # | Question | Resolution |
|---|---|---|
| Q1 | Professional Centre count | **13** confirmed |
| Q2 | Medical width notation in PDFs | Use FIN.xlsx sizes: 223 / 331 / 486 m² |
| Q3 | Corporate Office sizing | **TBD** — sized against Floor Plate at the end |
| Q4 | Eco-region variants (landscaping/parking) | **Deferred** to later iteration |
| Q5 | RS/TI Tile layer | **No Tile layer** — Key Plans compose directly into Floor Plates |
| Q6 | DTCG-to-Rust linkage | **Open** — decide at Rust engine scaffold time |
