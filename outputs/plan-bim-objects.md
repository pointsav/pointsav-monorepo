---
schema: foundry-plan-v1
archive: project-bim
topic: bim-objects
status: draft-v1
created: 2026-05-20
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
---

# Plan — BIM Objects: Key Plans, Tiles, Floor Plates

> **First draft — compiled 2026-05-20 from operator notes + 15 source documents.**
> This document is the working specification for the BIM Object Library:
> the complete taxonomy of Key Plans, Tiles, and Floor Plates, plus
> the Rust engine (`tool-buildingwidth`) that computes nesting between them.

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
of BIM objects organized as Key Plans → Tiles → Floor Plates. Architects
deliver Key Plans; the `tool-buildingwidth` Rust engine computes how they
nest into Tiles, and how Tiles nest into Floor Plates.

---

## Four Development Classes

| Class | Floors | Floor Plate (SF) | Notes |
|---|---|---|---|
| Professional Centres | 3–5 | 19,000–25,000 | 3 elevators; offset pulled-back core |
| Suburban Office | 6–9 | 17,000–23,000 | 4 elevators; offset pulled-back core |
| Retail Select | 1 | 4,500 / 6,700 / 7,700 | Strip retail; S/M/L |
| Tech Industrial | 1 | 7,200 / 8,400 | Industrial flex; M/L |

**Offset Pulled-Back Core** (shared by Professional Centres and Suburban Office):
- Building core pushed toward rear of building, not centered
- Enables full-length perimeter corridor without internal tenant corridors
- Supports adaptive reuse conversion (residential, warehouse) without major reconfiguration
- Long unobstructed floor plates ideal for Tiles nesting

---

## Three-Zone Framework

Every Key Plan (with the exception of pure-circulation and infrastructure types)
is organized on the same three-zone cross-section from facade to core:

```
[FACADE]  Zone 1 — Habitat  |  Zone 2 — Magazine  |  Zone 3 — Corridor  [CORE]
```

| Zone | Purpose | Depth (typical range) | Standard |
|---|---|---|---|
| Zone 1 — Habitat | All desks/workstations; natural light required | 4.7m – 7.2m | European Lighting Standard (6m desk-to-window rule) |
| Zone 2 — Magazine | Storage, bookshelves, secondary functions; depth set by the Key Plan | 1.3m – 9.3m | Variable — derived from furniture layout |
| Zone 3 — Corridor | Central circulation running floor length | 2.0m – 3.1m | ~20% oversized from code minimum for wellness |

The Zone 1 depth is the primary driver of building width per Key Plan type.
Zone 3 corridor depth is consistent within a Key Plan category (typically 3.0m
for professional tenants). Zone 2 is the adjustable layer — the `tool-buildingwidth`
engine varies Zone 2 depth to achieve integer-multiple Tile nesting.

---

## Part 1 — Key Plans

### Definition

A Key Plan is the smallest BIM Object unit. It represents a single leaseable
or infrastructure space, defined by:
- Furniture arrangement (specific manufacturer SKUs)
- Zone 1/2/3 dimensions
- Net leasable area (m² and SF)
- Facade frontage requirement
- Accessibility compliance
- Occupancy count
- Building Width Calculator integration (Zone depths feed directly into Rust engine)

Key Plans are authored by architects. Woodfine provides the briefing (zone
requirements, furniture program, accessibility standard); the architect delivers
the final spatial geometry.

**Key Plans that are themselves self-contained (e.g., Lobby Atrium, Emergency
Stairwell) do not need a Tile wrapper — they appear directly in Floor Plates
at Key Plan level.**

---

### Key Plan Registry — 72 entries

#### GENERAL KEY PLANS (25)

**Private Office** — facade-facing; individual lease; Building ID issued per desk

| Code | Name | Area (m²) | Area (SF) | Zone 1 | Zone 2 | Zone 3 |
|---|---|---|---|---|---|---|
| PO-1 | Private Office Small | 30 | 325 | 6.0m | 3.8m | 2.0m |
| PO-2 | Private Office Medium | 43 | 465 | 6.0m | ~5.0m | 2.0m |
| PO-3 | Private Office Large | 64 | 685 | 6.0m | ~7.0m | 2.0m |

Furniture (PO-1 sample): 1 desk + chair, 1 × 3-person round table, 2 side chairs,
1 filing cabinet, 1 credenza, 1 bookshelf, 1 coat rack.

**Corporate Office** — full Tile; no sub-lease; one climate zone per Tile

| Code | Name | Notes |
|---|---|---|
| CO-FF | Corporate Office — Full Floor | Entire floor plate; bespoke |
| CO-1/2 | Corporate Office — 1/2 Floor | |
| CO-1/3 | Corporate Office — 1/3 Floor | |
| CO-1/4 | Corporate Office — 1/4 Floor | |
| CO-1/8 | Corporate Office — 1/8 Floor | Minimum viable Corporate Tile |

Note: Corporate Office is the minimum Tile size for avoiding exposed columns in
smaller tenancies. CO-1/8 is the smallest Corporate Tile.

**Professional Office — Medical**

| Code | Name | Area (SF) | Key features |
|---|---|---|---|
| M-1 | Medical — Chiropractor / Dentist Small | ~2,402 | 2 exam/dental chairs, reception, sterilization, imaging, file room |
| M-2 | Medical — Dentist Medium | ~3,568 | 4 dental chairs, 2-3 sinks, autoclave, imaging, storage |
| M-3 | Medical — General Practitioner Large | ~5,231 | 6 chairs, multiple exam rooms, full support suite |

**Professional Office — Business**

| Code | Name | Area (m²) | Area (SF) | Zone 1 | Zone 2 | Zone 3 |
|---|---|---|---|---|---|---|
| B-1 | Business Small | 311 | 3,350 | 6.0m | 5.1m | 2.75m |
| B-2 | Business Medium | 400 | 4,301 | 6.0m | 5.1m | 2.75m |
| B-3 | Business Large | 486–699 | 5,231–7,524 | 7.2m | 4.9m | 2.9m |

Width range (B-1 to B-3): 25.3m to 32.3m total facade frontage.
Zone 2 provides the width adjustment variable (options A–D for each size).

**Professional Office — Laboratory**

| Code | Name | Area (m²) | Area (SF) | Workstations | Occupancy |
|---|---|---|---|---|---|
| L-1 | Laboratory Small (Medical) | 195 | 2,099 | 20 (9.7m²/ea) | 34 (5.7m²/pp) |
| L-2 | Laboratory Medium (Research) | 316 | 3,401 | 33 (9.5m²/ea) | 49 (6.4m²/pp) |
| L-3 | Laboratory Large | 401 | 4,313 | 45 (8.9m²/ea) | 61 (6.5m²/pp) |

Zone dimensions (all three): Z1=6.7m (22'3"), Z2=4.8m (15'9"), Z3=3.1m (10').
Rooms: reception, 1-2 offices, 3-9 benches, staff room, storage, mechanical, clean room.

**Professional Office — Academic**

| Code | Name | Area (m²) | Area (SF) | Occupancy |
|---|---|---|---|---|
| A-1 | Academic Small | 88–110 | 944–1,189 | 21–25 (4.1–4.4m²/pp) |
| A-2 | Academic Medium | 240 | 2,586 | 63 (3.8m²/pp) |
| A-3 | Academic Large | 327–376 | 3,523–4,050 | 82–103 (3.9–4.6m²/pp) |

Zone dimensions: Z1=4.7m (15'5"), Z2=3.0m (9'10"), Z3=TBD.
Configurations: seminar/podium (A-1), multi-use seminar (A-2), tiered auditorium ~60 cap (A-3).

**Professional Office — Civic**

| Code | Name | Area (m²) | Area (SF) |
|---|---|---|---|
| C-1 | Civic Small | 270 | 2,912 |
| C-2 | Civic Medium | 577 | 6,215 |
| C-3 | Civic Large | 822 | 8,850 |

Layout: offices, clerk desks, conference rooms, staff room, restrooms, communal corridor.
Color zones: Yellow (work areas), Orange (internal corridor), Green (support), Blue (communal).

**Circulation & Utility**

| Code | Name | Notes |
|---|---|---|
| R-1 | Corridor Expanders | Adjustable corridor widths; fills remainder between Tiles |
| V-1 | Meter Room | Utility infrastructure; no Zone 1 requirement |

---

#### PROFESSIONAL CENTRE KEY PLANS (13–14)

Infrastructure Key Plans for the Professional Centre development class.
These appear in Floor Plates directly (not wrapped in leasing Tiles).

| Code | Name |
|---|---|
| N-1 | Tenant Lounge |
| EE-1 | Lobby Atrium |
| O-1 | Building Manager Office |
| P-1 | Mail Room |
| S-1 | Elevator Lobby |
| U-1 | Tenant Restroom |
| X-1 | Loading |
| Y-1 | Recycling |
| Z-1 | Bike Room |
| AA-1 | Workbench |
| BB-1 | Building Staff Lockers |
| CC-1 | Coffee / Bread |
| DD-1 | Public Restrooms |

*Note: source says 14 entries; one entry to be confirmed against the FIN.xlsx index.*

---

#### SUBURBAN OFFICE KEY PLANS (14)

Mirror of Professional Centre set with `-2` suffix; adds Mop Room.

| Code | Name |
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

#### RETAIL SELECT KEY PLANS (3)

| Code | Name |
|---|---|
| RA-1 | Retail Leasehold Small |
| RB-2 | Retail Leasehold Medium |
| RC-3 | Retail Leasehold Large |

---

#### TECH INDUSTRIAL KEY PLANS (3)

| Code | Name |
|---|---|
| TI-1 | Tech Leasehold Small |
| TI-2 | Tech Leasehold Medium |
| TI-3 | Tech Leasehold Large |

---

#### LANDSCAPING KEY PLANS (4)

Eco-region variants — same geometry, different planting palette.

| Code | Name | Eco-region |
|---|---|---|
| LL-1a | Bioswales | Boreal Plains |
| LL-1b | Bioswales | Fescue Grassland |
| LL-1c | Bioswales | Parkland Natural |
| LL-2 | Irrigation Gallery | (all) |

---

#### PARKING KEY PLANS (11)

| Code | Name | Eco-region |
|---|---|---|
| PP-1a | Parking Stalls | Boreal Plains |
| PP-1b | Parking Stalls | Fescue Grassland |
| PP-1c | Parking Stalls | Parkland Natural |
| PP-2a | Accessible Parking | Boreal Plains |
| PP-2b | Accessible Parking | Fescue Grassland |
| PP-2c | Accessible Parking | Parkland Natural |
| PP-3 | Sidewalks | — |
| PP-4 | Snowdrops | — |
| PP-5 | Signage | — |
| PP-6 | Lighting | — |

---

## Part 2 — Tiles

### Definition

A Tile is a BIM Object composed of one or more Key Plans within a single
**Climate Zone**. One Tile = one climate zone = one HVAC control boundary.
Demising walls within a Tile share climate zone control.

Tiles come in two standard sizes:
- **2,700 SF** — base module (single leasing bay)
- **6,000 SF** — end-cap module (corner + stairwell integration)

### Tile Composition Rules

```
T_Basic    = n(PO-1) + p(PO-2) + q(PO-3)
T_Compound = n(PO-1) + p(PO-2) + q(PO-3) + r(A-* | L-* | M-* | B-* | C-*)
T_Special  = n(PO-1) + p(PO-2) + q(PO-3) + r(A-* | L-* | M-* | B-* | C-*)
              [same as Compound but occupies corner/end-cap position in Floor Plate]
```

Where n, p, q, r are non-negative integers (zero allowed).

**Key rule:** A Key Plan that is inherently a full Tile (e.g., Lobby Atrium EE-1,
Emergency Stairwell) does NOT get wrapped in a Tile — it appears directly in the
Floor Plate at Key Plan level.

### Tile Registry

#### 2,700 SF Tiles

| Tile | Type | Composition | Climate Zones | Doors | Service Hookups |
|---|---|---|---|---|---|
| Tile A | Corporate | Corporate Office (full tile) | 1 | 9 | 9 |
| Tile B-1 | Private Mix | PO-1 (300 SF) + PO-3 (500 SF) + PO-2 (450 SF) + B-1 small portion (800 SF) | 1 | TBD | TBD |
| Tile C-1 | Prof Medium | B-2 (2,000 SF) + PO-2 (450 SF) + PO-1 (300 SF) | 1 | TBD | TBD |
| Tile C-2 | Prof Large | B-3 (2,400 SF) + PO-1 (300 SF) | 1 | TBD | TBD |

#### 6,000 SF End-Cap Tiles

| Tile | Type | Composition | Notes |
|---|---|---|---|
| Tile A-1 | Corporate End-Cap | Corporate Office (6,000 SF) | Includes Emergency Stairwell, Corridor E |
| Tile B-2 | Private Mix End-Cap | PO-1 + PO-2 (×2) + PO-3, Corridors D+E | Corner of floor plate |
| Tile C-3 | Prof Medium End-Cap | B-2 (×2) + PO-2, Corridors D+E | |
| Tile C-4 | Prof Large End-Cap | B-3 + PO-3 + PO-2 + PO-1, Corridors D+E | |

### Climate Zone Identity

Climate zone = Tile boundary. This means:
- Two Tiles sharing a demising wall = two separate climate zones
- A tenant occupying multiple Tiles has multiple climate zones (or they consolidate to a Corporate Tile)
- For residential conversion: each Tile maps naturally to a residential suite HVAC zone

---

## Part 3 — Floor Plates

### Definition

A Floor Plate is a BIM Object composed of a combination of Tiles, Key Plans
(infrastructure-level), and the Building Core. Floor Plates define the leasable
configuration of an entire building floor.

### Floor Plate Matrix (9 types)

| ID | Name | Development Class | Size (SF) | Floors Used |
|---|---|---|---|---|
| FP-PC-M | Main Floor — Professional Centres | Professional Centres | 19,000–25,000 | Ground |
| FP-SO-M | Main Floor — Suburban Office | Suburban Office | 17,000–23,000 | Ground |
| FP-PC-TL | Second Floor Tenant Lounge — Professional Centres | Professional Centres | 19,000–25,000 | 2 |
| FP-SO-TL | Second Floor Tenant Lounge — Suburban Office | Suburban Office | 17,000–23,000 | 2 |
| FP-TI-M | Medium Floor Plate — Tech Industrial | Tech Industrial | 7,200 | 1 |
| FP-TI-L | Large Floor Plate — Tech Industrial | Tech Industrial | 8,400 | 1 |
| FP-RS-S | Small Floor Plate — Retail Select | Retail Select | 4,500 | 1 |
| FP-RS-M | Medium Floor Plate — Retail Select | Retail Select | 6,700 | 1 |
| FP-RS-L | Large Floor Plate — Retail Select | Retail Select | 7,700 | 1 |

### Composition Rules

```
Floor Plate = T_Basic + T_Compound + T_Special + Building Core + Infrastructure Key Plans

Where:
  T_Basic    = T_Compound   → Corporate Office equivalent
  T_Basic    = T_Compound = T_Special → Climate Zone = Structural Grid
  Infrastructure Key Plans  → EE-1/2 (Lobby), N-1/2 (Tenant Lounge),
                              S-1/2 (Elevator Lobby), U-1/2 (Restrooms),
                              Emergency Staircases, etc.
```

**Retail Select:**
```
Floor Plate = Left End Cap + Mechanical Room + n(RA-1 | RB-2 | RC-3) + Right End Cap
```

**Tech Industrial:**
```
Floor Plate = Left End Cap + Mechanical Room + n(TI-2 | TI-3) + Right End Cap
```

### The "Ring"

The Floor Plates Methodology documents describe a "ring" — the structural and
circulation annulus around the building core. The ring defines:
- The separation between leasing Tiles and the building core
- The routing path for corridor connections to elevators and stairs
- The minimum Tile depth before the ring begins

The Rust engine must account for ring geometry when computing remainder-free nesting.

### Bi-Directional Solver Requirement

The `tool-buildingwidth` Rust engine must solve two directions:

**Forward (Design):** Given Key Plan dimensions + Tile types → compute valid Floor Plate sizes.

**Reverse (Fit-Out):** Given a fixed Floor Plate size + fixed Key Plan sizes → find Tile
adjustments (Zone 2 depth variation, Corridor Expander R-1 insertion) that eliminate remainder.

If exact fit is impossible with standard Tiles: compute required **Special Tile** dimensions
(a non-standard Tile that fills the gap). Output the Special Tile specification so an
architect can design it.

---

## Part 4 — Building Width Calculator

### Purpose

The Building Width Calculator determines the total building width (facade to core)
required to accommodate a given set of Key Plans in a given Tile configuration.

Inputs:
- Key Plan type(s) and count
- Tile composition
- Floor Plate target size

Outputs:
- Zone 1 depth required (driven by largest Key Plan in Tile)
- Zone 2 depth options (A/B/C/D variants per Key Plan type)
- Zone 3 corridor width
- Total building width
- Whether the combination achieves a remainder-free Floor Plate

### Width Data by Key Plan Type

**Private Office:**
```
PO-1: Total width ~11.8m  (Z1=6.0m, Z2=3.8m, Z3=2.0m)
PO-2: Total width ~varies  (Z1=6.0m, Z2=~5.0m, Z3=2.0m)
PO-3: Total width ~varies  (Z1=6.0m, Z2=~7.0m, Z3=2.0m)
```

**Professional — Laboratory (all three sizes):**
```
Z1 = 6.7m (22'3"), Z2 = 4.8m (15'9"), Z3 = 3.1m (10')
L-1 facade frontage: 55'3" (~16.8m)
L-2 facade frontage: 89'6" (~27.3m)
L-3 facade frontage: 113'6" (~34.6m)
```

**Professional — Academic (all three sizes):**
```
Z1 = 4.7m (15'5"), Z2 = 3.0m (9'10"), Z3 = TBD
```

**Professional — Business (B-1 through B-3):**
```
Width range: 25.3m – 32.3m total
Width options per size:
  Option A: Z1=5.51m, Z2=5.76m, Z3=2.75m
  Option B: higher Z1, adjusted Z2
  Option C: higher still
  Option D: maximum width variant
```

**Professional — Medical:**
```
M-1: ~2,401 SF
M-2: ~3,568 SF
M-3: ~5,231 SF
```
*Note: M-2 and M-3 facade widths appear incorrectly transcribed in source PDFs — confirm
from the FIN.xlsx before these feed into the Rust engine.*

---

## Rust Engine — `tool-buildingwidth`

### Current State

`tool-buildingwidth` exists as a DTCG token file
(`woodfine-bim-library/tokens/bim/building-width-calculator.dtcg.json`).
The operator notes say it needs to be expanded to a **RUST ENGINE**.

The engine will live at:
```
pointsav-monorepo/tool-buildingwidth/
```
(New crate, not yet scaffolded.)

### Engine Responsibilities

1. **Key Plan → Tile nesting:** given a list of Key Plans and their Zone dimensions,
   compute valid Tile compositions that achieve a target Tile size (2,700 SF or
   6,000 SF) without remainder.

2. **Tile → Floor Plate nesting:** given a list of Tiles and a Floor Plate target,
   compute a valid Floor Plate arrangement (accounting for Building Core + ring).

3. **Bi-directional:** accept either (Key Plan sizes fixed, Floor Plate fixed → solve Tiles)
   or (Key Plan sizes fixed, Tile targets fixed → compute Floor Plate sizes).

4. **Special Tile detection:** if no standard Tile composition achieves zero remainder,
   output the required Special Tile dimensions.

5. **Corridor Expander insertion:** insert R-1 Corridor Expanders to absorb small
   remainder values before declaring a Special Tile necessary.

### Proposed Data Model

```rust
struct KeyPlan {
    code: String,            // PO-1, L-1, B-2, EE-1, etc.
    category: KeyPlanCategory,
    area_m2: f64,
    area_sf: f64,
    zone1_depth_m: f64,      // Habitat
    zone2_depth_m: f64,      // Magazine
    zone3_depth_m: f64,      // Corridor
    facade_frontage_m: f64,
    is_self_contained: bool, // true → appears in Floor Plate directly, not in a Tile
}

struct Tile {
    code: String,            // Tile-A, Tile-B1, etc.
    tile_type: TileType,     // Basic | Compound | Special | EndCap
    area_sf: f64,            // 2700 or 6000 standard; Special tiles are variable
    key_plans: Vec<(String, u32)>, // (code, count)
    climate_zones: u32,      // always 1 per standard Tile
    doors: u32,
    service_hookups: u32,
}

struct FloorPlate {
    code: String,            // FP-PC-M, FP-RS-S, etc.
    development_class: DevelopmentClass,
    area_sf_min: f64,
    area_sf_max: f64,
    tiles: Vec<String>,
    infrastructure_key_plans: Vec<String>,
    has_ring: bool,
}
```

### Relationship to DTCG Tokens

The existing `building-width-calculator.dtcg.json` captures the Zone depth
constants per Key Plan type. The Rust engine reads these token values as its
configuration input — the DTCG file is the data source; the crate is the
computation layer.

---

## Deliverables

### Deliverable 1 — BIM Objects: Key Plans (72 entries)

A structured list of all 72 Key Plans with:
- Code, name, category, development class
- Area (m² and SF)
- Zone 1/2/3 depths
- Facade frontage
- Occupancy count
- `is_self_contained` flag

**Verify against:** `CONSTRUCTION_MCorp_2026_01_06_Database_Floor Plans_Key Plans_Index_FIN.xlsx`
This is the master database — spreadsheet wins over PDF extracts on any conflict.

### Deliverable 2 — BIM Objects: Tiles

Structured list of all Tiles with composition rules, standard sizes, end-cap variants.

**Verify against:** `CONSTRUCTION_2026_01_06_Tear Sheet_Floor Plates_Tiles_Alternatives_V2_FIN.xlsx`

### Deliverable 3 — Rust Engine: `tool-buildingwidth`

New crate at `pointsav-monorepo/tool-buildingwidth/`. See engine section above.

---

## Open Questions

1. **Professional Centre Key Plans: 13 or 14?** Directory lists 13 named entries but source
   says 14. What is the 14th? (Possible: Mop Room W-1, a second café type, Parking Lobby.)

2. **Medical width notation:** M-2 and M-3 widths appear incorrectly transcribed in source
   PDFs. Confirm from FIN.xlsx before building Rust data model.

3. **Corporate Office sizing:** CO-1/8 = one 2,700 SF Tile? Or does CO-1/8 have its own
   distinct area? Clarify before the Rust data model is finalized.

4. **Eco-region variants:** Are Landscaping and Parking eco-region variants separate BIM
   Objects (different geometry) or same geometry with a different material tag?

5. **Retail Select and Tech Industrial Tiles:** Do RS and TI use named Tiles, or do their
   Key Plans compose directly into Floor Plates without an intermediate Tile layer?

6. **DTCG → Rust engine linkage:** Does `tool-buildingwidth` read the DTCG token file at
   runtime, or compile token values as constants? Runtime preferred (consistent with
   `app-orchestration-bim` pattern).

---

## Implementation Order

1. Reconcile Key Plans list against FIN.xlsx master database → confirm 72 entries,
   resolve open questions 1, 3, 4.
2. Resolve Medical width notation (question 2).
3. Scaffold `tool-buildingwidth` crate in `pointsav-monorepo` with data types above.
4. Implement Key Plan → Tile nesting (forward direction).
5. Implement Tile → Floor Plate nesting (forward direction).
6. Implement bi-directional solver with Corridor Expander insertion + Special Tile detection.
7. Wire DTCG token file as runtime data source (question 6).
8. Add BIM Object entries to `bim.woodfinegroup.com` library pages.
