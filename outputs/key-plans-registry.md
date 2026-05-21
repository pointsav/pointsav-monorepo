---
schema: woodfine-bim-registry-v1
type: key-plans-registry
status: draft-v1
updated: 2026-05-21
source: CONSTRUCTION_2026_01_06_Key Plan_Professional Office_FFE_FIN.xlsx — tab Summary_Key Plans (V3, 2025-11-29)
note: "NB — Not drawn to scale — Approximate square footages (per source spreadsheet)"
---

# Key Plans Registry

Authoritative catalog of all BIM Object Key Plans for Woodfine Management Corp.
Source of record for `bim.woodfinegroup.com` library pages and the
`tool-buildingwidth` Rust engine.

A **Key Plan** is the smallest BIM Object unit — a spatial program defined by
real furniture placement, three-zone cross-section, net leasable area, and
accessibility compliance.

---

## Three-Zone Framework

Every Key Plan has this cross-section from facade to building core:

```
[FACADE]  Zone 1 — Habitat  |  Zone 2 — Magazine  |  Zone 3 — Corridor  [CORE]
```

| Zone | Purpose | Depth range |
|---|---|---|
| Zone 1 — Habitat | All desks and workstations; natural light required | 4.7 m – 7.2 m |
| Zone 2 — Magazine | Storage and secondary functions; adjustable by type | 1.3 m – 9.3 m |
| Zone 3 — Corridor | Central circulation running the length of the floor | 2.0 m – 3.1 m |

---

## 1. Private Office

Three sizes. Facade-facing individual leasehold. Building ID issued per desk.
Zone 2 depth is constant across all three sizes; frontage width increases with size.

| Code | Display name | m² | SF | Z1 (m) | Z2 (m) | Z3 (m) |
|---|---|---|---|---|---|---|
| PO-1 | Private Office — Small | 30.19 | 325 | 6.0 | 3.8 | 2.0 |
| PO-2 | Private Office — Medium | 43.20 | 465 | 6.0 | 3.8 | 2.0 |
| PO-3 | Private Office — Large | 63.64 | 685 | 6.0 | 3.8 | 2.0 |

**Furniture (PO-1):** desk + chair, 3-person round table, 2 side chairs,
filing cabinet, credenza, bookshelf, coat rack.
Desks placed perpendicular to facade require +0.7 m in Zone 1 for three desks in series.

---

## 2. Corporate Office

Five sizes. Full Tile; no sub-lease; one climate zone per Tile.
Dimensions are sized as a proportion of the Floor Plate — independent
dimensions are not defined at this stage.

| Code | Display name | m² | SF |
|---|---|---|---|
| CO-FF | Corporate Office — Full Floor | TBD | TBD |
| CO-1/2 | Corporate Office — Half Floor | TBD | TBD |
| CO-1/3 | Corporate Office — Third Floor | TBD | TBD |
| CO-1/4 | Corporate Office — Quarter Floor | TBD | TBD |
| CO-1/8 | Corporate Office — Eighth Floor | TBD | TBD |

Internal tile codes (type-prefixed): CO-A, CO-B, CO-C, CO-D, CO-E.

---

## 3. Professional Office — Medical

Three sizes. Wider Zone 1 (7.2 m) than other sub-types to accommodate
exam-table depth and patient + clinician circulation.

| Code | Display name | m² | SF | Z1 (m) | Z2 (m) | Z3 (m) |
|---|---|---|---|---|---|---|
| M-1 | Medical — Chiropractor | 223 | 2,401 | 7.20 | 4.87 | TBD |
| M-2 | Medical — Dentist | 331 | 3,568 | 7.20 | 4.87 | TBD |
| M-3 | Medical — General Practitioner | 486 | 5,231 | 7.20 | 4.87 | TBD |

**Key rooms:** reception, exam / dental chairs (2 / 4 / 6), file room,
autoclave, sterilization, imaging, storage, washroom.

---

## 4. Professional Office — Business

Three sizes. Zone 2 adjustable up to 7.30 m max; width options A–D available
per size. Total facade frontage range: 25.3 m – 32.3 m.

| Code | Display name | m² | SF | Z1 (m) | Z2 (m) | Z3 (m) |
|---|---|---|---|---|---|---|
| B-1 | Business — Small | 311.22 | 3,350 | 6.00 | 7.30 | 2.75 |
| B-2 | Business — Medium | 399.66 | 4,301 | 6.00 | 7.30 | 2.75 |
| B-3 | Business — Large | 669.00 | 7,201 | 6.00 | 7.30 | 2.90 |

---

## 5. Professional Office — Laboratory

Three sizes. Benched laboratory layout; mechanical and clean room included.

| Code | Display name | m² | SF | Z1 (m) | Z2 (m) | Z3 (m) | Frontage |
|---|---|---|---|---|---|---|---|
| L-1 | Laboratory — Medical | 195.00 | 2,099 | 6.78 | 4.80 | 3.10 | ~16.8 m |
| L-2 | Laboratory — Research | 315.96 | 3,401 | 6.78 | 4.80 | 3.10 | ~27.3 m |
| L-3 | Laboratory — Large | 400.69 | 4,313 | 6.78 | 4.80 | 3.10 | ~34.6 m |

**Key rooms:** reception, 1–2 offices, 3–9 benches, staff room, storage,
mechanical, clean room.
**Occupancy:** L-1 = 34 (5.7 m²/pp), L-2 = 49 (6.4 m²/pp), L-3 = 61 (6.5 m²/pp).

---

## 6. Professional Office — Academic

Three sizes. Lecture / seminar configurations.

| Code | Display name | m² | SF | Z1 (m) | Z2 (m) | Z3 (m) |
|---|---|---|---|---|---|---|
| A-1 | Academic — Small | 105 | 1,131 | 4.70 | 3.00 | TBD |
| A-2 | Academic — Medium | 240 | 2,583 | 4.70 | 3.00 | TBD |
| A-3 | Academic — Large | 378 | 4,070 | 4.70 | 3.00 | TBD |

**Configurations:** seminar / podium (A-1), multi-use seminar (A-2),
tiered auditorium ~60 capacity (A-3).
**Occupancy:** A-1 = 21–25, A-2 = 63, A-3 = 82–103.

---

## 7. Professional Office — Civic

Three sizes. Government services, community programming, and social services.

| Code | Display name | m² | SF | Z1 (m) | Z2 (m) | Z3 (m) |
|---|---|---|---|---|---|---|
| C-1 | Civic — Small | 270 | 2,912 | 6.00 | 7.23 | TBD |
| C-2 | Civic — Medium | 577 | 6,215 | 6.00 | 7.23 | TBD |
| C-3 | Civic — Large | 822 | 8,850 | 6.00 | 7.23 | TBD |

**Key rooms:** offices, clerk desks, conference rooms, staff room,
restrooms, communal corridor.

---

## 8. Circulation and Utility

| Code | Display name | Area | Notes |
|---|---|---|---|
| R-1 | Corridor Expanders | variable | Fills remainder gaps between Tiles; width adjustable |
| T | Corridor Expander T | 300 SF | Fixed unit; inserted before declaring a Special Tile |
| V-1 | Meter Room | TBD | Utility; no Zone 1 requirement |

---

## 9. Professional Centre Infrastructure

Sizes TBD — architect delivers Key Plan drawings from the equipment programs
below. These Key Plans appear directly in Floor Plates (not wrapped in Tiles).
Confirmed count: 13.

| Code | Display name | Sizes | Equipment program |
|---|---|---|---|
| N-1 | Tenant Lounge | S / M / L | Seating, kitchenette |
| EE-1 | Lobby Atrium | S / M / L | Entry, reception |
| O-1 | Building Manager Office | M / L | Manager desk + table, asst. manager desk, mail supervisor desk, night watch desk, 2–4 mail charts, 6–12 shelves, first aid |
| P-1 | Mail Room | (combined with O-1) | One Key Plan tab in source; architect to confirm split or merge |
| S-1 | Elevator Lobby | M / L | 3 (M) / 4 (L) elevator cabs, service staircase |
| U-1 | Tenant Restroom | M / L | 2 shared sinks, 2 stalls each gender, mop room (sink, drain, shelves) |
| X-1 | Loading | M / L | 1–2 grade doors, 1–2 service doors, 1–2 refuse bins, 1–2 compactors |
| Y-1 | Recycling | M / L | Paper / plastic / glass bins (1–2 each) |
| Z-1 | Bike Room | M / L | 12 (M) / 24 (L) bike racks, exterior door |
| AA-1 | Workbench | M / L | 1 (M) / 2 (L) workbench units |
| BB-1 | Building Staff Lockers | M / L | 5 (M) / 10 (L) lockers, 2–4 showers |
| CC-1 | Coffee / Bread | S / M / L | Kitchenette |
| DD-1 | Public Restrooms | M / L | 2–3 shared sinks, 2–4 stalls each gender, 1 family / maternal / accessible room |

---

## 10. Suburban Office Infrastructure

Mirror of the Professional Centre set with `-2` suffix. Adds Mop Room (W-2).
Sizes TBD from architect drawings using the same equipment programs.

Per source: "Suburban Office doesn't need additional washroom stalls — although
the building is taller, the Floor Plates are smaller; each floor may have the
same occupancy or less."

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

## 11. Retail Select

Three leasehold sizes. Key Plans compose directly into Floor Plates — no Tile layer.
Floor Plate composition: Left End Cap + Mechanical Room + n(RA / RB / RC) + Right End Cap.

| Floor Plate | Target SF |
|---|---|
| Retail Select — Small | 4,500 |
| Retail Select — Medium | 6,700 |
| Retail Select — Large | 7,700 |

Zone depths: Z1 = 6.0 m, Z2 = 3.8 m, Z3 = 2.0 m.

| Code | Display name | m² | SF |
|---|---|---|---|
| RA-1 | Retail Leasehold — Small | TBD | TBD |
| RB-2 | Retail Leasehold — Medium | TBD | TBD |
| RC-3 | Retail Leasehold — Large | TBD | TBD |

**Also in RS Floor Plates (non-leasehold):** End Cap S / M, Mechanical Room,
Spacer M / L — sizes TBD.
Internal tile codes (type-prefixed): RS-A through RS-M.

---

## 12. Tech Industrial

Three leasehold sizes. Key Plans compose directly into Floor Plates — no Tile layer.
Floor Plate composition: Left End Cap + Mechanical Room + n(TI-2 / TI-3) + Right End Cap.

| Floor Plate | Target SF |
|---|---|
| Tech Industrial — Medium | 7,200 |
| Tech Industrial — Large | 8,400 |

Zone depths: Z1 = 6.0 m, Z2 = 3.8 m, Z3 = 2.0 m.

| Code | Display name | m² | SF |
|---|---|---|---|
| TI-1 | Tech Leasehold — Small | TBD | TBD |
| TI-2 | Tech Leasehold — Medium | TBD | TBD |
| TI-3 | Tech Leasehold — Large | TBD | TBD |

**Also in TI Floor Plates (non-leasehold):** End Cap M / L, Mechanical Room,
Spacers M / L — sizes TBD.
Internal tile codes (type-prefixed): TI-A through TI-M.

---

## 13. Landscaping

Two active Key Plans. Eco-region variants (Boreal Plains, Fescue Grassland,
Parkland Natural) deferred to a later iteration.

| Code | Display name | m² |
|---|---|---|
| LL-1 | Bioswales | TBD |
| LL-2 | Irrigation Gallery | TBD |

---

## 14. Parking

Six active Key Plans. Eco-region variants for PP-1 and PP-2 deferred.

| Code | Display name | m² |
|---|---|---|
| PP-1 | Parking Stalls | TBD |
| PP-2 | Accessible Parking | TBD |
| PP-3 | Sidewalks | TBD |
| PP-4 | Snowdrops | TBD |
| PP-5 | Signage | TBD |
| PP-6 | Lighting | TBD |

---

## Count summary

| Category | Count | Sizes confirmed | Sizes TBD |
|---|---|---|---|
| Private Office | 3 | 3 | — |
| Corporate Office | 5 | — | 5 (floor-plate dependent) |
| Medical | 3 | 3 | — |
| Business | 3 | 3 | — |
| Laboratory | 3 | 3 | — |
| Academic | 3 | 3 | — |
| Civic | 3 | 3 | — |
| Circulation and Utility | 3 | — | 3 |
| Professional Centre Infrastructure | 13 | — | 13 (architect drawings) |
| Suburban Office Infrastructure | 14 | — | 14 (architect drawings) |
| Retail Select | 3 + components | — | 6 |
| Tech Industrial | 3 + components | — | 6 |
| Landscaping | 2 | — | 2 |
| Parking | 6 | — | 6 |
| **Total active** | **66** | **15** | **51** |

9 eco-region variants deferred (LL-1 ×3, PP-1 ×3, PP-2 ×3);
total will reach 75 when those variants are added.
