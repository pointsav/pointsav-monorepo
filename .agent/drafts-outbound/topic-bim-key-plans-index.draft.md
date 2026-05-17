---
schema: foundry-draft-v1
version: "1.0"
draft_id: topic-bim-key-plans-index-2026-05-17
language_protocol: TOPIC
state: ready-for-sweep
originating_cluster: project-bim
target_repo: vendor/content-wiki-projects
target_path: topics/bim/key-plans-index.md
audience: operator
bcsc_class: vendor-internal
authored: 2026-05-17T22:00:00Z
authored_by: totebox@project-bim
authored_with: claude-opus-4-7
research_done_count: 6
research_suggested_count: 4
open_questions_count: 3
research_provenance:
  - Key Plan Directory V2, January 7, 2025 — `inputs/project-bim/--- May 06, 2025 -- Collaborators #27 ---/AEC_Floor Plates_Key Plans_Index.pdf` (72-row master index)
  - Key Plan Directory V2, January 6, 2026 — `inputs/project-bim/PROJECTS_MCorp_Database_Floor Plates_Key Plans_Index.pdf` (identical content; redated)
  - V3 Master Summary, January 6, 2026 — `inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Summary_Notes copy.pdf` page 1 (authoritative for per-use-type zone depths and key plan areas)
  - V2 Summary, February 19, 2025 — `inputs/project-bim/--- March 03, 2025 -- Collaborators #32 --- /AEC_Floor Plates_Key Plans_Summary.pdf` (earlier iteration with Academic Small at 110.5 m²; superseded)
  - Key Plans Notes V2, January 7, 2025 — `inputs/project-bim/--- March 03, 2025 -- Collaborators #32 --- /AEC_Floor Plates_Key Plans_Notes.pdf` (definition of Key Plans and Tiles; zone derivation methodology)
  - DTCG token files — `woodfine-bim-library/tokens/bim/building-width-calculator.dtcg.json` + `professional-office-subtypes.dtcg.json`
research_inline: true
notes_for_editor: |
  Living document — master index of all 72 key plans across nine
  development classes. Each per-use-type sub-section will receive
  its own TOPIC as additional DISCOVERY work lands. This file is the
  cross-reference table.
---

# Key Plans Index — Master Reference

The Key Plan Directory inventories every distinct floor-plate sub-area
that the Building Information Model (BIM) recognises. Each entry pairs
a Development Class with a Typology and (where relevant) an Eco Region,
yielding a file-name slug that is referenced from every downstream BIM
artifact (DTCG tokens, IFC entity classifications, Rust solvers, slide
decks, regulation overlays).

The current Directory (V2, dated both 2025-01-07 and re-dated
2026-01-06 — identical content) contains **72 key plans across nine
Development Classes**.

## Entry 2026-05-17 — V2 inventory

### Development Classes and Key Plan counts

| # | Development Class | Key Plan count | Index range |
|---|---|---:|---|
| 1 | General | 25 | 1–25 |
| 2 | Professional Centre | 13 | 26–38 |
| 3 | Retail Select | 3 | 39–41 |
| 4 | Suburban Office | 14 | 42–55 |
| 5 | Tech Industrial | 3 | 56–58 |
| 6 | Landscaping | 4 | 59–62 |
| 7 | Parking | 10 | 63–72 |
| | **Total** | **72** | |

### General class — Index 1–25 (the leasable + circulation core)

| Index | Typology | ID | Notes |
|---:|---|---|---|
| 1 | Private Office | Small | PO-1 sample; 325 SF |
| 2 | Private Office | Medium | PO-2 sample; 465 SF |
| 3 | Private Office | Large | PO-3 sample; 685 SF |
| 4 | Corporate Office | Full Floor | one tile per floor |
| 5 | Corporate Office | 1/2 Floor | |
| 6 | Corporate Office | 1/3 Floor | |
| 7 | Corporate Office | 1/4 Floor | one quarter-tile |
| 8 | Corporate Office | 1/8 Floor | smallest Corporate increment |
| 9 | Medical | Chiropractor | specialisation, no completed sample |
| 10 | Medical | Dentist | M3 sample 2,402 SF |
| 11 | Medical | General Practitioner | specialisation |
| 12 | Business | B-1 | small business sample 3,350 SF |
| 13 | Business | B-2 | medium 4,302 SF |
| 14 | Business | B-3 | large 7,524 SF |
| 15 | Laboratory | Medical | specialisation of Lab |
| 16 | Laboratory | Research | specialisation of Lab |
| 17 | Laboratory | L-3 | large-lab key plan |
| 18 | Academic | A-1 | small 1,131 SF |
| 19 | Academic | A-2 | medium 2,583 SF |
| 20 | Academic | A-3 | large 4,070 SF |
| 21 | Civic | C-1 | small 2,912 SF |
| 22 | Civic | C-2 | medium 6,215 SF |
| 23 | Civic | C-3 | large 8,850 SF |
| 24 | Corridor Expanders | R-1 | special tile |
| 25 | Meter Room | V-1 | special tile |

### Professional Centre — Index 26–38 (building amenities and core)

Tenant Lounge (N-1), Lobby Atrium (EE-1), Building Manager (O-1),
Mail Room (P-1), Elevator Lobby (S-1), Tenant Restroom (U-1), Loading
(X-1), Recycling (Y-1), Bike Room (Z-1), Workbench (AA-1), Building
Staff Lockers (BB-1), Coffee/Bread (CC-1), Public Restrooms (DD-1).

### Suburban Office — Index 42–55

Same amenity vocabulary as Professional Centre with "-2" suffix, plus
Mop Room (W-2), which is unique to Suburban Office.

### Retail Select — Index 39–41

Three Retail Leasehold variants: RA-1, RB-2, RC-3.

### Tech Industrial — Index 56–58

Three Tech Leasehold sizes: TI-1, TI-2, TI-3.

### Landscaping — Index 59–62

Bioswales (LL-1) appear three times — one per Eco Region (boreal plains,
fescue grassland, parkland natural) — and one Irrigation Gallery (LL-2).

### Parking — Index 63–72

Parking Stalls (PP-1, three Eco Region variants), Accessible (PP-2,
three Eco Region variants), Sidewalks (PP-3), Snowdrops (PP-4), Signage
(PP-5), Lighting (PP-6).

## Entry 2026-05-17 — V3 Master Summary reconciliation

The V3 Master Summary table (January 6, 2026; one page, originating in
`DISCOVERY_MCorp_Sketches_Key Plans_Summary_Notes`) supersedes earlier
iterations for the General-class leasable typologies. Authoritative
numbers:

| Use type | Z1 Habitat | Z2 Magazine | Z3 Corridor | KP-S (m² / SF) | KP-M | KP-L |
|---|---:|---:|---:|---|---|---|
| Private Office | 6.0 m / 19'8" | 1.4 m / 4'6" | — | 30 / 325 | 43 / 465 | 64 / 685 |
| Laboratory | 6.8 m / 22'3" | 4.8 m / 15'9" | 3.0 m / 10' | 195 / 2,099 | 316 / 3,401 | 401 / 4,313 |
| Academic | 4.7 m / 15'5" | 3.0 m / 3'7" | — | **105 / 1,131** | 240 / 2,583 | 378 / 4,070 |
| Business | 6.0 m / 19'8" | 7.3 m / 23'11" | 2.7 m | 311 / 3,350 | 400 / 4,302 | 669 / 7,524 |
| Medical | 7.2 m / 23'10" | 4.9 m / 16" | 2.9 m / 9'5" | 223 / 2,402 | 331 / 3,568 | 486 / 5,231 |
| Civic | 6.0 m / 19'8" | 7.2 m / 23'8" | 3.6 m / 12' | 270 / 2,912 | 577 / 6,215 | 822 / 8,850 |
| Professional Office (Initial Design baseline) | 6.0 m / 19'8" | 3.8 m / 12'5" | 2.0 m / 6'6" | 130 / 1,400 | — | — |

### Academic Small — reconciliation

The Academic Small key plan area appears at three different values
across the iteration history:

- **87.7 m² / 944 SF** — Samples_V2 "smaller" option ("Consider removing one office?")
- **110.5 m² / 1,189 SF** — Samples_V2 full option (V2 Summary, February 2025)
- **105 m² / 1,131 SF** — V3 Master Summary (January 6, 2026) — **authoritative**

The DTCG token in `building-width-calculator.dtcg.json` correctly
carries 105 m² / 1,131 SF. The token in `professional-office-subtypes.dtcg.json`
incorrectly carries the 87.7 m² value and must be updated to match V3.

## Entry 2026-05-17 — Naming convention

Key plans use the convention:

```
key plan-{development-class}-{typology}-{id}[-{eco-region}]
```

Examples:
- `key plan-general-private office-small`
- `key plan-general-business-medium` (Note: `Key Plan ID` column says
  "Business" but `ID Typology` column says "B-2"; downstream slug uses
  the size label `medium`)
- `key plan-landscaping-bioswales-LL-1-boreal plains` (Eco Region suffix)

The Directory mixes two naming registers — short IDs (B-2, M-3, PP-1)
and size labels (small / medium / large). Downstream code should
accept both; the canonical slug uses size labels.

## Entry 2026-05-17 — FFE tile codes vs key plan footprints

Each Professional Office sub-type also carries FFE (Furniture, Fixtures
and Equipment) tile codes representing 1/4-floor sub-allocations:

- `-1` = 1,100 SF
- `-2` = 1,400 SF
- `-3` = 800 SF

These are **not** the key plan footprints. A Medical M2 FFE tile code
(1,400 SF) is distinct from the Medical Medium key plan (3,568 SF),
which is the completed-sample full-suite area. The HTML slide deck
makes this distinction explicit on Slide 2's lede.

## Future research

The following per-use-type TOPIC drafts are planned (each will mature
from DISCOVERY sketch material into a full reference page):

- `topic-bim-private-office-key-plans.md` — PO-1/PO-2/PO-3 detail
  (drafted 2026-05-17)
- `topic-bim-medical-key-plans.md` — M1/M2/M3 dental-chair-anchored
  detail (drafted 2026-05-17)
- `topic-bim-business-key-plans.md` — B-1/B-2/B-3 with magazine-driven
  width options A/B/C/D (drafted 2026-05-17)
- `topic-bim-academic-key-plans.md` — A-1/A-2/A-3, podium/desks/
  auditorium variants
- `topic-bim-laboratory-key-plans.md` — L-1/L-2/L-3, fume hood +
  bench depth driving Z1
- `topic-bim-civic-key-plans.md` — C-1/C-2/C-3, public-assembly
  corridor at 3.6 m
- `topic-bim-corporate-office-key-plans.md` — Full / 1/2 / 1/3 /
  1/4 / 1/8 floor variants
- `topic-bim-amenity-key-plans.md` — Professional Centre + Suburban
  Office amenity vocabulary (N, EE, O, P, S, U, W, X, Y, Z, AA, BB,
  CC, DD)
- `topic-bim-landscape-parking-key-plans.md` — Eco Region variants

## Open questions

1. **Corporate Office key plan areas (Index 4–8)** are still TBD in
   every source document examined. The Tile column on the V3 Summary
   is empty. When will Corporate Office samples land?
2. **Chiropractor and General Practitioner Medical specialisations**
   (Index 9, 11) appear in the Directory but have no completed
   samples — only the Dentist sample exists. Are these intended as
   layout variants of the M1/M2/M3 footprints, or separate sample
   exercises?
3. **Eco Region variants for Bioswales and Parking** (Index 59–68)
   need climate-zone-aware key plan content — currently the
   Directory rows exist but no per-region sample sketches have
   surfaced.
