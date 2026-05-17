# Key Plans Foundation Study

**Date authored:** 2026-05-17
**Author:** totebox@project-bim, claude-opus-4-7
**Status:** Reference synthesis (read-only deliverable; no edits to HTML/tokens/code)

## Source documents read (chronological)

1. **2025-03-03 — Methodology (V12)** —
   `/home/jennifer/sandbox/inputs/project-bim/--- March 03, 2025 -- Collaborators #32 --- /AEC_Floor Plates_Key Plans_Methodology_V12.pdf`
   (1 page; "Spatial Taxonomy — Key Plan Methodology — Professional Centres and Suburban Office")

2. **2025-04-01 — Key Plans and Tiles (V12)** —
   `/home/jennifer/sandbox/inputs/project-bim/--- April 01, 2025 -- Collaborators #11 ---/AEC_Floor Plates_Key Plans and Tiles_V12.pdf`
   (7 pages; covers Professional Centres/Suburban Office, Retail Select, Tech Industrial, and a blank Parking/Landscaping header)

3. **2025-05-06 — Key Plans Index (V2)** —
   `/home/jennifer/sandbox/inputs/project-bim/--- May 06, 2025 -- Collaborators #27 ---/AEC_Floor Plates_Key Plans_Index.pdf`
   (1 page; "Key Plan Directory" — 72 numbered rows)

Cross-referenced (current cluster state):

- `woodfine-bim-library/tokens/bim/professional-office-subtypes.dtcg.json`
- `woodfine-bim-library/tokens/bim/building-width-calculator.dtcg.json`
- `woodfine-bim-library/tokens/bim/tile-system.dtcg.json`
- `woodfine-bim-library/tokens/bim/floor-plate-standards.dtcg.json`
- `woodfine-bim-library/tokens/bim/landscape-parking.dtcg.json` (peeked, entries 51+ context only)
- `preview/building-width-calculator.html` (`BIM_TOKENS` block, lines 884–905)

**Cluster path note:** the operator brief referred to `woodfine-bim-library` as `woodfine-design-bim`. The actual checked-out folder name is `woodfine-bim-library`. Names used in this document follow the on-disk reality.

---

## Executive summary

- **The methodology PDF documents only one fully-drawn key plan** (Small Private Office, PO-1) with named zones (Zone 1 Habitat = 6 m; Zone 2 Magazine = ? m; Zone 3 Corridor = ? m). Every other zone-depth dimension that the cluster's token store currently asserts (7.2 m Medical Habitat, 7.3 m Business Magazine, 3.6 m Civic Corridor, etc.) is **not present in these three PDFs** — those numbers come from other source files cited in the token JSON ($description fields).
- **The Tiles PDF (V12, April) and the Index PDF (V2, May) disagree fundamentally** on the unit of composition. The Tiles PDF builds floor plates from numbered key plans inside tile bands (PO-1/PO-2/PO-3 at 300/450/500 SF; M-1/M-2/M-3 at 1,100/1,400/800 SF; same triadic pattern for B, L, A, C). The Index PDF uses a different naming convention — Private Office is Small/Medium/Large (no PO-N codes); Business is B-1/B-2/B-3; Medical lists specialisations (Chiropractor, Dentist, General Practitioner) not size variants — and introduces a corpus of 72 key plans the Tiles PDF doesn't mention.
- **The current token store and HTML BIM_TOKENS are a partial union of all three sources with gaps and a couple of contradictions.** The PO-1/PO-2/PO-3 codes appear in `tile-system.dtcg.json` tile compositions but never as standalone key plans; M-/B-/L-/A-/C- codes appear nowhere in the token store except as `ffe_codes` arrays inside `professional-office-subtypes.dtcg.json`; ~30 Index-PDF entries (Tenant Lounge N-1, Lobby Atrium EE-1, parking PP-1 through PP-6, etc.) are partially captured in `landscape-parking.dtcg.json` but the bulk of the Professional Centre / Suburban Office / Retail Select / Tech Industrial inventory has no token-level home.

---

## Section 1 — What is a Key Plan?

### The methodology PDF (2025-03-03) defines a key plan visually, not textually

The page is titled **"Building a Key Plan"** and shows two side-by-side diagrams. Both diagrams are labelled **"Small Private Office (PO-1)"**. The left diagram shows the bare floor plan with three colour-coded bands: **Net leasable square feet** (yellow), **Corridor** (green), **Façade frontage** (grey), plus three icons in the leasable area: a square (**Tenant**), a circle (**Accessibility**), and a triangle (**Guest**).

The right diagram is the same Small Private Office *with the Building Width Calculator overlay*. The overlay adds:

- **Zone 1 — Habitat — 6 metres** (annotated; the only metre figure that is fully numeric)
- **Zone 2 — Magazine — ? metres** (annotated; depth shown as a question mark)
- **Zone 3 — Corridor — ? metres** (annotated; depth shown as a question mark)
- **Façade Frontage** band at top (no dimension)
- **Corridor** band at bottom (no dimension)
- Furniture occupants: Desk, Credenza, Round Table, Coat Rack, Bookshelf (the "Actual furniture" legend entry, mustard-yellow)
- A **BUDGET** column referencing Steelcase products: Desk $, Credenza $, Round Table $, Bookshelf $, Coat Rack $, Total $, Square footage? (the dollar values are blank placeholders)

The footer disclaimer reads **"NB — Not drawn to scale — Approximate square footages"** and the title block is **"Spatial Taxonomy — Key Plan Methodology — Professional Centres and Suburban Office / Woodfine Management Corp. / Jennifer M. Woodfine / January 7, 2025 / V12"**.

### Implicit definition (extracted from the diagram)

A **Key Plan** is a furniture-and-circulation-anchored floor plan for a single leasable unit, drawn at the smallest meaningful unit of leasing for a given use type. It carries:

1. A use-type identifier (Private Office, Medical, Business, Laboratory, Academic, Civic, Corporate Office, etc.);
2. A size variant within that use type (Small / Medium / Large, or a numbered code like PO-1);
3. A depth structure of three zones perpendicular to the façade — **Zone 1 Habitat** (desks; European Lighting Standard 6 m); **Zone 2 Magazine** (storage); **Zone 3 Corridor** (shared building corridor);
4. Façade frontage on one or both ends (mirrored floor plate);
5. A specific furniture/equipment list tied to a vendor (Steelcase) for FFE budgeting.

The Tiles PDF (April 2025) reinforces this with its definition headline, repeated on each page:

> **"Key Plans and Tiles"** means a geometric self-similar aperiodic space planning system based on furniture/equipment arrangements and circulation versus modular area per person progressions.

(Tiles PDF, pp. 1, 5, 6, 7.)

For Parking and Landscaping the definition substitutes the subject:

> **"Key Plans and Tiles"** means a geometric self-similar aperiodic space planning system based on private automobiles/pedestrians/cyclists. (Tiles PDF p. 7.)

### Key plan vs. tile vs. floor plate

From the Tiles PDF (pp. 1–4):

| Tier | Unit | Size | Composition |
|---|---|---|---|
| Floor plate | One upper floor | 20,000 SF (1,858 m²) | 4 × 1/4 tile or 8 × 1/8 tile or mix |
| Tile | A standard composable band | 1/8 (2,500 SF), 1/4 (5,000 SF), 1/2 (10,000 SF), full (20,000 SF) | A sequence of key plans + corridor + core |
| Key plan | A single leasable unit | 300–2,500 SF range (Private Office through Corporate end-cap) | Furniture arrangement inside the three zones |

The relationship is **key plan → tile → floor plate → building**. A tile is a *recipe* of key plans arranged across the building width.

### How a key plan relates to a use type

From the Index PDF (May 2025), the **"Development Class"** column groups key plans into seven families:

1. **General** (entries 1–25) — leasable office/retail key plans by use type
2. **Professional Centre** (entries 26–38) — common-area key plans for an office building
3. **Retail Select** (entries 39–41) — retail leasehold key plans
4. **Suburban Office** (entries 42–55) — duplicates Professional Centre common areas with "-2" suffix
5. **Tech Industrial** (entries 56–58)
6. **Landscaping** (entries 59–62)
7. **Parking** (entries 63–72)

A **use type** in the General class (Private Office, Corporate Office, Medical, Business, Laboratory, Academic, Civic, Corridor Expanders, Meter Room) maps to a key plan family. Within each family, **typology** captures the size or specialisation variant (Small/Medium/Large, or named — Chiropractor/Dentist/General Practitioner for Medical, or Full Floor/1-2/1-3/1-4/1-8 for Corporate Office).

A tile (Small Tile A through Tile H per the current token store, or the un-coded Tile A/B/C/D/N tiles per the April Tiles PDF) composes those key plans into a band whose total width equals the building width and whose depth corresponds to one of the standard tile fractions.

---

## Section 2 — The master Key Plans inventory

This section consolidates every key plan that appears in any of the three documents. **Naming is materially different across the documents** — see the discrepancy column.

### 2.1 Inventory from the May 2025 Index PDF (canonical 72-row directory)

Verbatim transcription of the Key Plan Directory rows (Index PDF, p. 1). The "Key Plan ID" is the value the Index PDF puts in the **Typology** column; the directory does not assign size-code suffixes to most entries.

| # | Dev Class | Key Plan family | Typology / ID | File-name slug |
|---:|---|---|---|---|
| 1 | General | Private Office | Small | key plan-general-private office-small |
| 2 | General | Private Office | Medium | key plan-general-private office-medium |
| 3 | General | Private Office | Large | key plan-general-private office-large |
| 4 | General | Corporate Office | Full Floor | key plan-general-corporate office-full floor |
| 5 | General | Corporate Office | 1/2 Floor | key plan-general-corporate office-1-2 floor |
| 6 | General | Corporate Office | 1/3 Floor | key plan-general-corporate office-1-3 floor |
| 7 | General | Corporate Office | 1/4 Floor | key plan-general-corporate office-1-4 floor |
| 8 | General | Corporate Office | 1/8 Floor | key plan-general-corporate office-1-8 floor |
| 9 | General | Medical | Chiropractor | key plan-general-professional office-medical-chiropractor |
| 10 | General | Medical | Dentist | key plan-general-professional office-medical-dentist |
| 11 | General | Medical | General Practitioner | key plan-general-professional office-medical-general practitioner |
| 12 | General | Business | B-1 | key plan-general-business-small |
| 13 | General | Business | B-2 | key plan-general-business-medium |
| 14 | General | Business | B-3 | key plan-general-business-large |
| 15 | General | Laboratory | Medical | key plan-general-professional office-laboratory-medical |
| 16 | General | Laboratory | Research | key plan-general-professional office-laboratory-research |
| 17 | General | Laboratory | L-3 | key plan-general-laboratory-large |
| 18 | General | Academic | A-1 | key plan-general-academic-small |
| 19 | General | Academic | A-2 | key plan-general-academic-medium |
| 20 | General | Academic | A-3 | key plan-general-academic-large |
| 21 | General | Civic | C-1 | key plan-general-civic-small |
| 22 | General | Civic | C-2 | key plan-general-civic-medium |
| 23 | General | Civic | C-3 | key plan-general-civic-large |
| 24 | General | Corridor Expanders | R-1 | key plan-general-corridor expanders-R-1 |
| 25 | General | Meter Room | V-1 | key plan-general-meter room-V-1 |
| 26 | Professional Centre | Tenant Lounge | N-1 | key plan-professional centre-tenant lounge-N-1 |
| 27 | Professional Centre | Lobby Atrium | EE-1 | key plan-professional centre-lobby atrium-EE-1 |
| 28 | Professional Centre | Building Manager | O-1 | key plan-professional centre-building manager-O-1 |
| 29 | Professional Centre | Mail Room | P-1 | key plan-professional centre-mail room-P-1 |
| 30 | Professional Centre | Elevator Lobby | S-1 | key plan-professional centre-elevator lobby-S-1 |
| 31 | Professional Centre | Tenant Restroom | U-1 | key plan-professional centre-tenant restroom-U-1 |
| 32 | Professional Centre | Loading | X-1 | key plan-professional centre-loading-X-1 |
| 33 | Professional Centre | Recycling | Y-1 | key plan-professional centre-recycling-Y-1 |
| 34 | Professional Centre | Bike Room | Z-1 | key plan-professional centre-bike room-Z-1 |
| 35 | Professional Centre | Workbench | AA-1 | key plan-professional centre-workbench-AA-1 |
| 36 | Professional Centre | Building Staff Lockers | BB-1 | key plan-professional centre-building staff lockers-BB-1 |
| 37 | Professional Centre | Coffee/Bread | CC-1 | key plan-professional centre-coffee/bread-CC-1 |
| 38 | Professional Centre | Public Restrooms | DD-1 | key plan-professional centre-public restrooms-DD-1 |
| 39 | Retail Select | Retail Leasehold | RA-1 | key plan-retail select-retail leasehold-RA-1 |
| 40 | Retail Select | Retail Leasehold | RB-2 | key plan-retail select-retail leasehold-RB-2 |
| 41 | Retail Select | Retail Leasehold | RC-3 | key plan-retail select-retail leasehold-RC-3 |
| 42 | Suburban Office | Tenant Lounge | N-2 | key plan-suburban office-tenant lounge-N-2 |
| 43 | Suburban Office | Lobby Atrium | EE-2 | key plan-suburban office-lobby atrium-EE-2 |
| 44 | Suburban Office | Building Manager | O-2 | key plan-suburban office-building manager-O-2 |
| 45 | Suburban Office | Mail Room | P-2 | key plan-suburban office-mail room-P-2 |
| 46 | Suburban Office | Elevator Lobby | S-2 | key plan-suburban office-elevator lobby-S-2 |
| 47 | Suburban Office | Tenant Restroom | U-2 | key plan-suburban office-tenant restroom-U-2 |
| 48 | Suburban Office | Mop Room | W-2 | key plan-suburban office-mop room-W-2 |
| 49 | Suburban Office | Loading | X-2 | key plan-suburban office-loading-X-2 |
| 50 | Suburban Office | Recycling | Y-2 | key plan-suburban office-recycling-Y-2 |
| 51 | Suburban Office | Bike Room | Z-2 | key plan-suburban office-bike room-Z-2 |
| 52 | Suburban Office | Workbench | AA-2 | key plan-suburban office-workbench-AA-2 |
| 53 | Suburban Office | Building Staff Lockers | BB-2 | key plan-suburban office-building staff lockers-BB-2 |
| 54 | Suburban Office | Coffee/Bread | CC-2 | key plan-suburban office-coffee/bread-CC-2 |
| 55 | Suburban Office | Public Restrooms | DD-2 | key plan-suburban office-public restrooms-DD-2 |
| 56 | Tech Industrial | Tech Leasehold | TI-1 | key plan-tech industrial-tech leasehold-TI-1 |
| 57 | Tech Industrial | Tech Leasehold | TI-2 | key plan-tech industrial-tech leasehold-TI-2 |
| 58 | Tech Industrial | Tech Leasehold | TI-3 | key plan-tech industrial-tech leasehold-TI-3 |
| 59 | Landscaping | Bioswales | LL-1 (boreal plains) | key plan-landscaping-bioswales-LL-1-boreal plains |
| 60 | Landscaping | Bioswales | LL-1 (fescue grassland) | key plan-landscaping-bioswales-LL-1-fescue grassland |
| 61 | Landscaping | Bioswales | LL-1 (parkland natural) | key plan-landscaping-bioswales-LL-1-parkland natural |
| 62 | Landscaping | Irrigation Gallery | LL-2 | key plan-landscaping-irrigation gallery-LL-2 |
| 63 | Parking | Parking Stalls | PP-1 (boreal plains) | key plan-parking-parking stalls-PP-1-boreal plains |
| 64 | Parking | Parking Stalls | PP-1 (fescue grassland) | key plan-parking-parking stalls-PP-1-fescue grassland |
| 65 | Parking | Parking Stalls | PP-1 (parkland natural) | key plan-parking-parking stalls-PP-1-parkland natural |
| 66 | Parking | Accessible | PP-2 (boreal plains) | key plan-parking-accessible-PP-2-boreal plains |
| 67 | Parking | Accessible | PP-2 (fescue grassland) | key plan-parking-accessible-PP-2-fescue grassland |
| 68 | Parking | Accessible | PP-2 (parkland natural) | key plan-parking-accessible-PP-2-parkland natural |
| 69 | Parking | Sidewalks | PP-3 | key plan-parking-sidewalks-PP-3 |
| 70 | Parking | Snowdrops | PP-4 | key plan-parking-snowdrops -PP-4 |
| 71 | Parking | Signage | PP-5 | key plan-parking-signage-PP-5 |
| 72 | Parking | Lighting | PP-6 | key plan-parking-lighting-PP-6 |

### 2.2 Inventory from the April 2025 Tiles PDF (PO-N / M-N / B-N / L-N / A-N / C-N codes)

The Tiles PDF uses **a different set of identifiers** with explicit square footages. These codes are the basis for the tile-composition recipes.

**Upper Floors — Private Office (1/8 Floor Tile, 2,500 SF; sample shows 2,150 SF):**

| Code | SF | Furniture (Small / Medium / Large counts) | Tiles PDF page |
|---|---:|---|---|
| PO-1 | 300 | Desk+Chair 1/2/3; Round Table 1/1/1; Filing 1/1/2; Bookshelf 1/1/2; Coat Rack 1/1/2 | p. 1 |
| PO-2 | 450 | (same FFE table, Medium row) | p. 1 |
| PO-3 | 500 | (same FFE table, Large row) | p. 1 |

Tile composition shown (p. 1): **PO-1 / PO-1 / PO-2 / PO-1 / PO-3 / PO-1 → 6 cells totalling 2,150 SF**, with an availability mix of 80% Small / 10% Medium / 10% Large.

**Upper Floors — Professional Office sub-types (1/4 Floor Tile, 5,000 SF; sample shows 5,200 SF) — same triadic pattern across Medical, Business, Laboratory, Academic, Civic:**

| Code | SF | Medical (S/M/L) | Business (S/M/L) | Laboratory (S/M/L) | Academic (S/M/L) | Civic (S/M/L) |
|---|---:|---|---|---|---|---|
| -1 | 1,100 | Reception 1/1/1; Exam Room 2/4/6; Doctor Office 1/1/2; File+Kitchen 1/1/1 | Reception 1/1/1; Private Office 1/2/2; Desk 2/2/4; File+Kitchen 1/1/1 | Private Office 0/1/2; Bench 3/5/7; Storage 1/1/1; Clean Room 0/1/1 | Podium 1/1/1; Desk 6/10/0; Auditorium Seating 0/0/1; Storage 1/1/1 | Judge 0/0/1; Court Clerk Desk 0/0/1; Theater Seating 0/0/28; Storage 1/1/1 |
| -2 | 1,400 | (Medium variant) | (Medium variant) | (Medium variant) | (Medium variant) | (Medium variant) |
| -3 | 800 | (Large variant — note: smallest SF for largest typology because two -3 cells appear in the tile) | (same) | (same) | (same) | (same) |

Tile composition shown (pp. 1–2): for each sub-type, the row is **X-1 / X-1 / X-2 / X-3 / X-3 → 5 cells totalling 5,200 SF**, with availability 40% Small / 30% Medium / 20% Large (the remaining 10% is the implicit "Civic" minority bucket per p. 2).

**Upper Floors — Corporate Office tiles (p. 2):**

| Tile letter | Composition | Total |
|---|---|---:|
| A (1/8 Floor) | A 2,500 + Private Office 2,500 + Private Office 2,500 + Private Office 2,500 + Medical Office 5,000 + Professional Office 5,000 | 20,000 SF |
| B (1/4 Floor) | B 5,000 + Professional Office 5,000 + 4 × Private Office 2,500 | 20,000 SF |
| C (1/2 Floor, p. 3) | C 10,000 + Professional Office 5,000 + Medical Office 5,000 | 20,000 SF |
| D (Full Floor, p. 3) | D 20,000 | 20,000 SF |

Footnote on p. 3: **"*Key Plans J, K, L, and M should each be presented as full-size open plans for medical, business, laboratory, and academic tenants."** No further detail on J/K/L/M in this document — they appear to be planning placeholders.

**Second floor & main floor / common areas (Tiles PDF pp. 3–4):**

| Letter | Name | SF |
|---|---|---:|
| N | Tenant Lounge (2nd floor, 1/2 Floor Tile) | 10,000 |
| O | Building Manager Office | 450 |
| P | Mail Room | 450 |
| R | Corridor Expanders (upper floors) | 50 |
| S | Elevator Lobby (upper floors) | 300 |
| T | Corridor Expanders (upper floors, p. 4 left diagram shows "T 300" — see Discrepancy 2.4 below) | 100 in legend / 300 in diagram |
| U | Tenant Restrooms (upper floors) | 400 |
| V | Meter Room (upper floors) | 150 |
| W | Mop Room (upper floors) | 150 |
| X | Loading (main floor) | 750 |
| Y | Recycling (main floor) | 400 |
| Z | Bike Room (main floor) | 300 |
| AA | Workbench (main floor) | 150 |
| BB | Building Staff Lockers (main floor) | 200 |
| CC | Coffee/Bread (main floor) | 1,200 |
| DD | Public Restrooms (main floor) | 450 |

**Retail Select (Tiles PDF p. 5; Approximate Net Leasable Floor Plate 4,500–7,700 SF):**

| Code | SF | Notes |
|---|---:|---|
| A | 1,400 | End cap, present in all three building sizes |
| B | 1,000 | Middle tile; one in Small, one in Medium, two in Large |
| C | 700 | End cap (paired with M 200) |
| D | 1,100 | Mid-section expansion tile; absent in Small, 2× in Medium and Large |
| E | 1,200 | End cap |
| M | 200 | Vestibule/M-tile (below C, paired) |

Recipes (p. 5):
- **Small (4,500 SF)** = A 1,400 + B 1,000 + C 700 + M 200 + E 1,200
- **Medium (6,700 SF)** = A + B + C 700 + M 200 + D 1,100 + D 1,100 + E 1,200
- **Large (7,700 SF)** = A + B + B 1,000 + C 700 + M 200 + D 1,100 + D 1,100 + E 1,200

Legend note: **"CAN ADD MORE 'B' AND 'D' TO MAKE BUILDING BIGGER"** (p. 5).

**Tech Industrial (Tiles PDF p. 6; Approximate Net Leasable Floor Plate 7,200–9,600 SF):**

| Code | SF |
|---|---:|
| A | 2,200 |
| B | 2,100 |
| C | 1,200 |
| D | 1,500 |
| M | 200 |

Recipes (p. 6):
- **Medium (7,200 SF)** = A + B + M 200 + C + D
- **Large (8,400 SF)** = A + B + M 200 + C + C 1,200 + D

Legend note: **"CAN ADD MORE 'C' TO MAKE BUILDING BIGGER"** (p. 6).

**Parking and Landscaping (Tiles PDF p. 7):**
The page contains only the title block, definition headline, and a "Parking and Landscaping" banner. **No diagrams or tables — the page is otherwise empty.** Index entries 59–72 capture the actual parking/landscaping inventory.

### 2.3 Naming discrepancies across the three documents

This is the single biggest reconciliation problem. The Index PDF and the Tiles PDF describe overlapping inventories but assign different IDs.

| Concept | Methodology PDF (Mar) | Tiles PDF (Apr) | Index PDF (May) |
|---|---|---|---|
| Smallest private office | PO-1 (shown in diagram) | PO-1 (300 SF) | Private Office Small (no PO-N) |
| Medium private office | — | PO-2 (450 SF) | Private Office Medium |
| Large private office | — | PO-3 (500 SF) | Private Office Large |
| Smallest medical | — | M-1 (1,100 SF) | Medical Chiropractor / Dentist / General Practitioner (specialisation, not size) |
| Medium medical | — | M-2 (1,400 SF) | (absent) |
| "Large" medical (smallest of 5-cell row) | — | M-3 (800 SF) | (absent) |
| Smallest business | — | B-1 (1,100 SF) | Business B-1 (size code, no SF) |
| Medium business | — | B-2 (1,400 SF) | Business B-2 |
| Large business | — | B-3 (800 SF) | Business B-3 |
| Laboratory size variants | — | L-1, L-2, L-3 | Laboratory Medical / Research / L-3 (mixed scheme) |
| Academic size variants | — | A-1, A-2, A-3 | Academic A-1 / A-2 / A-3 |
| Civic size variants | — | C-1, C-2, C-3 | Civic C-1 / C-2 / C-3 |
| Tenant Lounge | — | N (10,000 SF, 1/2 floor) | N-1 (Professional Centre) and N-2 (Suburban Office) |
| Building Manager | — | O (450 SF) | O-1 / O-2 |
| Mail Room | — | P (450 SF) | P-1 / P-2 |
| Corridor Expanders | — | R 50 and T 100/300 SF | R-1 (one entry only, no -2 variant) |
| Meter Room | — | V (150 SF) | V-1 |
| Mop Room | — | W (150 SF) | W-2 only (no W-1) — *appears Professional Centre has no Mop Room in the Index* |
| Tenant Restrooms | — | U (400 SF) | U-1 / U-2 |
| Loading | — | X (750 SF) | X-1 / X-2 |
| Recycling | — | Y (400 SF) | Y-1 / Y-2 |
| Bike Room | — | Z (300 SF) | Z-1 / Z-2 |
| Workbench | — | AA (150 SF) | AA-1 / AA-2 |
| Building Staff Lockers | — | BB (200 SF) | BB-1 / BB-2 |
| Coffee/Bread | — | CC (1,200 SF) | CC-1 / CC-2 |
| Public Restrooms | — | DD (450 SF) | DD-1 / DD-2 |
| Lobby Atrium | shown as background block on Tiles p. 3–4 only | (background only, no ID) | EE-1 / EE-2 (full key plan entries) |
| Elevator Lobby | — | S (300 SF) | S-1 / S-2 |

**Implication:** Following the Index, key plans should be denominated `Family-Typology` (e.g., "Private Office Small", "Medical Dentist", "Tenant Lounge N-1"). Following the Tiles PDF, key plans are denominated `Code-Number` (PO-1, M-1, N, etc.) with explicit SF. The two schemes are not mechanically translatable — Medical's Index entries are *specialisations* (Chiropractor / Dentist / GP), while the Tiles PDF's M-1/M-2/M-3 are *sizes*. There is no row in any of the three PDFs that joins these.

### 2.4 Internal inconsistencies inside the Tiles PDF

- **Tile letters reused.** "A" and "B" are used both for **Corporate Office tile letters** (p. 2: 1/8 Floor tile A = 2,500 SF; 1/4 Floor tile B = 5,000 SF) and for **Retail Select key plan letters** (p. 5: A = 1,400 SF end cap; B = 1,000 SF) and again for **Tech Industrial** (p. 6: A = 2,200 SF; B = 2,100 SF). These are not the same A's and B's. The PDF resolves it by context (which page / which use type) but a token model must namespace them.
- **Corridor Expander "T" SF mismatch (p. 4).** The legend reads "Corridor Expanders T 100" with a TOTAL of 450 SF (R 50 + S 300 + T 100). The diagram next to it shows three coloured cells labelled "R 50 / S 300 / T 300". Either the diagram is wrong (T should be 100) or the legend is wrong (T should be 300). The R+S+T = 450 total only holds if T = 100. **Flag for follow-up.**
- **Sample tile totals exceed the headline tile size.** Private Office 1/8 Floor tile is headlined as 2,500 SF (p. 1) but the sample row sums to 2,150 SF. Professional Office 1/4 Floor tile is headlined as 5,000 SF but the sample row sums to 5,200 SF. The PDF treats these as "approximate" (per the NB header) but a calculator using these as authoritative must decide which value wins.
- **"M" character is overloaded.** In Retail Select (p. 5) and Tech Industrial (p. 6), "M" is a 200 SF vestibule/utility tile. In the Index PDF, "M" is the Medical family prefix. The token store today uses "M1/M2/M3" as the Medical FFE codes — no collision yet, but the M-200 utility tile is undocumented in tokens.

---

## Section 3 — Dimensions and zones per key plan

### What the source PDFs actually disclose

Across all three documents, only **one** zone-depth dimension is fully numeric: **Zone 1 Habitat = 6 metres**, on the Small Private Office (PO-1) diagram in the Methodology PDF. The other two zones on that diagram (Magazine and Corridor) are annotated as **"? metres"**.

| Document | Use type | Z1 Habitat | Z2 Magazine | Z3 Corridor | Other dimensions |
|---|---|---|---|---|---|
| Methodology (Mar 2025) | Small Private Office (PO-1) | **6 m** | ? m | ? m | "Approximate square footage?" |
| Methodology (Mar 2025) | All other use types | — | — | — | not shown |
| Tiles (Apr 2025) | Any use type | — | — | — | The Tiles PDF works in SF and tile counts, not metres. No zone depths anywhere. |
| Index (May 2025) | Any use type | — | — | — | The Index is taxonomy + file naming only. No SF, no zones. |

**This is the central finding of Section 3.** The token store's per-use-type metre dimensions (Medical Habitat 7.2 m, Business Magazine 7.3 m, Laboratory Corridor 3.048 m, Civic Corridor 3.6 m, etc.) are **not in these three PDFs**. They originate from sources the token files name explicitly:

- `CONSTRUCTION_2026_01_06_Key Plan_Professional Office_FFE_FIN.xlsx` (Summary_Key Plans tab, V3, 2025-11-29) — cited as the authoritative source in `building-width-calculator.dtcg.json`'s file-level `$description`.
- `building-width-calculator.docx` — cited inline for Medical zone notes.
- `AEC_Floor Plates_Key Plans_Samples_V2.pdf` (Collaborators #32, 2025-03-03) — cited for the Academic key plan areas. This sample PDF lives in the same March 2025 folder as the Methodology PDF but **was not part of the operator's read list**.
- `DISCOVERY_MCorp_Sketches_Key Plans_Summary_Notes.pdf` — cited as the substrate for sub-type taxonomy.

### Token-store dimensions (for cross-reference; sourced from files outside this read list)

From `building-width-calculator.dtcg.json` (lines 4–174):

| Use type | Z1 Habitat | Z2 Magazine | Z3 Corridor | Z3 absent? |
|---|---|---|---|---|
| Professional Office | 6.0 m (19'8") | 3.8 m (12'5") | 2.0 m (6'6") | No |
| Private Office | 5.9944 m (19'8") | 1.3716 m (4'6") | 0.0 m | **Yes — no bilateral corridor** |
| Laboratory | 6.7818 m (22'3") | 4.8006 m (15'9") | 3.048 m (10'0") | No |
| Academic | 4.7 m (15'5") | 3.0 m (9'10") | 0.0 m | **Yes — opens to building corridor** |
| Business | 6.0 m (19'8") | 7.3 m (23'11") | 2.7 m (8'10") | No |
| Medical | 7.2 m (23'10") | 4.87 m (16'0") | 2.89 m (9'5") | No |
| Civic | 6.0 m (19'8") | 7.23 m (23'8") | 3.6 m (12'0") | No |

And a single circulation addition:

- **Perpendicular desk addition: +0.7 m** — added to Zone 1 Habitat when desks are perpendicular to the façade (needed for three desks in series per German Circulation Law).

### Token-store key-plan **areas** (from `building-width-calculator.dtcg.json` lines 182–337)

| Use type | Small (m² / SF) | Medium (m² / SF) | Large (m² / SF) | Source note |
|---|---|---|---|---|
| Private Office | 30.19 / 325 (PO-1) | 43.20 / 465 (PO-2) | 63.64 / 685 (PO-3) | From Summary_Key Plans tab |
| Professional Office | 130.06 / 1,400 (only Small completed) | TBD | TBD | Medium and Large pending |
| Laboratory | 195.00 / 2,099 | 315.96 / 3,401 | 400.69 / 4,313 | Completed samples |
| Academic | 105.00 / 1,131 | 240.00 / 2,583 | 378.00 / 4,070 | Completed samples |
| Business | 311.22 / 3,350 | 399.66 / 4,302 | 669.00 / 7,524 | Completed samples |
| Medical | 223.00 / 2,402 | 331.00 / 3,568 | 486.00 / 5,231 | Completed samples |
| Civic | 270.00 / 2,912 | 577.00 / 6,215 | 822.00 / 8,850 | Completed samples |

**These areas do not match the SF values in the April Tiles PDF.** For example: the Tiles PDF says Medical Small = M-1 = 1,100 SF and the medium tile row sums to 5,200 SF; the token store says Medical Small key plan = 2,402 SF. These are different things — the Tiles PDF M-1 is one **cell** in a tile row, while the token store's "Medical Small" key plan is the **whole leasable unit** (which would be one or more M-cells assembled). This is not currently documented in the token store.

### HTML `BIM_TOKENS` dimensions (mirror of `building-width-calculator.dtcg.json` with drift)

From `preview/building-width-calculator.html` lines 884–905:

| Use type | Z1 Habitat | Z2 Magazine | Z3 Corridor | Matches token JSON? |
|---|---|---|---|---|
| Private Office | 5.9944 | 1.3716 | **3.0** | **No** — JSON says 0.0, HTML says 3.0 (with note "3.0 m shared building corridor (small-tenant traffic)") |
| Professional Office | 6.0 | **3.0** | **3.0** | **No** — JSON says 3.8 / 2.0; HTML says 3.0 / 3.0 (with note "V12 baseline ... Z2 and Z3 are placeholders") |
| Academic | 4.7 | 3.0 | **3.0** | **No** — JSON says 0.0, HTML says 3.0 |
| Business | **5.51** | **9.26** | **2.75** | **No** — JSON says 6.0 / 7.3 / 2.7 |
| Medical | 7.2819 | 4.877 | 2.892 | Approximately (more decimal precision in HTML) |
| Laboratory | 6.7818 | 4.8006 | 3.048 | Yes |
| Civic | 6.0 | 7.23 | 3.6 | Yes |

This is a real divergence — the HTML and the canonical token JSON disagree on **four of seven use types**. The HTML BIM_TOKENS is supposed to be an "inline mirror" of the DTCG files (per its comment on line 881) but is currently not.

### Gaps (zone depths that no document discloses)

The following use types appear in the Index PDF or Tiles PDF but have **no zone-depth data** anywhere in this read list or the token store:

- **Corporate Office** (Full / 1/2 / 1/3 / 1/4 / 1/8) — Index entries 4–8; Tiles PDF tile letters A/B/C/D. The Tiles PDF treats Corporate as raw SF tiles without internal zone structure.
- **Retail Select** (RA-1, RB-2, RC-3 in Index; A/B/C/D/E/M in Tiles p. 5) — no zone depths.
- **Tech Industrial** (TI-1/TI-2/TI-3 in Index; A/B/C/D/M in Tiles p. 6) — no zone depths.
- **All Professional Centre common-area key plans** (Tenant Lounge, Lobby Atrium, Building Manager, Mail Room, Elevator Lobby, Tenant Restroom, Loading, Recycling, Bike Room, Workbench, Building Staff Lockers, Coffee/Bread, Public Restrooms) — no zone depths.
- **Landscaping / Parking** (entries 59–72) — no dimensions in the Tiles PDF (p. 7 is blank).

---

## Section 4 — Tile composition

### Composition recipes the Tiles PDF discloses

**Upper Floors — Private Office (p. 1, 1/8 Floor Tile, headline 2,500 SF; sample 2,150 SF):**

`[PO-1 300] [PO-1 300] [PO-2 450] [PO-1 300] [PO-3 500] [PO-1 300]` → 6 cells.

This is *one* tile row with PO-2 and PO-3 sandwiched between four PO-1 cells. The tile has **no explicit end-cap or middle-tile labels** in this PDF — the Tile B-1 "End Cap" designation comes from the current `tile-system.dtcg.json` token store and is not in any of the three PDFs.

**Upper Floors — Professional Office (pp. 1–2, 1/4 Floor Tile, headline 5,000 SF; sample 5,200 SF):**

For each of Medical, Business, Laboratory, Academic, Civic, the row is:

`[X-1 1,100] [X-1 1,100] [X-2 1,400] [X-3 800] [X-3 800]` → 5 cells.

Same pattern across all five Professional Office sub-types. The X-2 (Medium, 1,400 SF) cell sits in the middle of the row.

**Upper Floors — Corporate Office (pp. 2–3):**

| Fraction | Diagram (left-to-right) | Right-side data table |
|---|---|---|
| 1/8 Floor (p. 2) | Tile A 2,500 ┃ Private Office 2,500 ┃ Private Office 2,500 ┃ Private Office 2,500 ┃ Professional Office 5,000 ┃ Professional Office 5,000 | A 2,500 + PO 2,500 + PO 2,500 + PO 2,500 + Medical Office 5,000 + Professional Office 5,000 = 20,000 |
| 1/4 Floor (p. 2) | Tile B 5,000 ┃ Private Office 2,500 ┃ Private Office 2,500 ┃ Professional Office 5,000 ┃ Private Office 2,500 ┃ Private Office 2,500 | B 5,000 + Professional Office 5,000 + 4 × PO 2,500 = 20,000 |
| 1/2 Floor (p. 3) | Tile C 10,000 ┃ Professional Office 5,000 ┃ Professional Office 5,000 | C 10,000 + Professional Office 5,000 + Medical Office 5,000 = 20,000 |
| Full Floor (p. 3) | Tile D 20,000 (entire row) | D 20,000 |

**Second Floor — Tenant Lounge (p. 3, 1/2 Floor Tile):**

`[Private Office 2,500] [Private Office 2,500] ┃ [N Tenant Lounge 10,000] ┃ [Private Office 2,500] [Private Office 2,500]` → Total 10,000 SF lounge + 10,000 SF private office wrap.

Footnote: **"*Tenant Lounge to be accompanied by Private Office"** (p. 3).

**Main Floor — Building Manager + Mail Room (p. 3):**

`[O 450] / [P 450] ┃ Lobby Atrium` — O above P stacked, with Lobby Atrium to the right (Lobby Atrium has no dimension).

Footnote: **"*Coffee/Bread requires access to the public, Main Lobby, Public Restrooms, and Loading and Recycling"** (p. 4).

**Upper Floors — Corridors (p. 4):** `[R 50] [S 300] [T 300/100]` — see Discrepancy 2.4 about T.

**Upper Floors — Restrooms / Meter / Utility (p. 4):** U (400) on top, V (150) and W (150) below.

**Main Floor — Loading / Recycling / Bike Room (p. 4):**

`[X 750] [Y 400] [Z 300] [AA 150 / BB 200 stacked]` → 1,800 SF total.

**Main Floor — Coffee/Bread + Public Restrooms (p. 4):**

`[CC 1,200] / [DD 450] ┃ Lobby Atrium`.

**Retail Select (p. 5):**

The role of each tile is implicit in the position:
- **A 1,400** is always the leftmost tile (left end cap)
- **B 1,000** sits between A and C (one in Small, two in Large)
- **C 700** with **M 200** below it forms a paired end cap on the right of Small / on the left of D-D pairs in Medium and Large
- **D 1,100** is an interior expansion tile — added in pairs to grow the building
- **E 1,200** is the rightmost tile (right end cap)

**Tech Industrial (p. 6):**

- **A 2,200** left end cap
- **B 2,100** with **M 200** below — paired end cap immediately right of A
- **C 1,200** interior expansion tile (one in Medium, two in Large; "CAN ADD MORE 'C'")
- **D 1,500** right end cap

### Comparison: Tiles PDF vs current `tile-system.dtcg.json`

| In PDF | In token store | Match? |
|---|---|---|
| (no Tile A coded explicitly — Corporate 1/8 floor tile labelled "A") | `tile-a` Corporate Office 2,700 SF | **Partial.** Token's 2,700 SF doesn't match PDF's 2,500 SF (1/8 of 20,000). Token notes "Small Tile" family at 2,700 SF; PDF treats Tile A as 1/8 floor at 2,500 SF. |
| (no Tile B-1 coded; PDF's "B" is the Corporate 1/4 floor tile = 5,000 SF) | `tile-b1` Private Office End Cap 2,700 SF (5 private offices + corridor) | **No match.** The token's Tile B-1 is a Private Office end cap concept that is not in the April PDF. |
| (no Tile C-1 — "C" in PDF is the Corporate 1/2 floor tile = 10,000 SF) | `tile-c1` Professional Office Medium + Small 2,700 SF | **No match.** Token's Tile C-1 is not in the PDF. |
| Same for C-2, C-3, C-4 | Tokens exist | **No match.** |
| (no Tile E coded in PDF for Professional Centre — "E" appears only in Retail Select p. 5 as a 1,200 SF tile) | `tile-e1` Mixed End Cap Left 2,700 SF; `tile-e2` Mixed End Cap Right 2,700 SF | **No match.** Token's Tile E-1/E-2 are not in the PDF; the PDF's E is unrelated (Retail). |
| (no Tile F-G-H in PDF) | `tile-f` 4,900 SF Corporate; `tile-g` 4,900 SF Private Office mix (10 offices); `tile-h` 4,900 SF Professional mix | **Not in PDF.** Large Tile family is a token-store construct sourced from `AEC_Floor Plates_Tiles_Alternatives.pdf` per `tile-system.dtcg.json`'s file-level `$description`. |
| Tile D = Full Floor 20,000 SF | (no "tile-d") | **Missing from tokens.** |

**Implication:** The `tile-system.dtcg.json` Small Tile family (A, B-1, C-1, C-2, C-3, C-4, E-1, E-2 at 2,700 SF each) and Large Tile family (F, G, H at 4,900 SF each) are derived from a different document (`AEC_Floor Plates_Tiles_Alternatives.pdf`, also dated 2025-05-06 per the May folder — but that file was not part of this read list). The April Tiles PDF the operator chose uses a coarser Corporate-driven scheme (A=1/8, B=1/4, C=1/2, D=full). These are different tile vocabularies for the same problem.

The 2,700 SF and 4,900 SF "tile increments" do not divide 20,000 evenly:
- 7 × 2,700 = 18,900 SF (need +1,100 to reach 20,000)
- 4 × 4,900 = 19,600 SF (need +400)
- 2 × 4,900 + 4 × 2,700 = 20,600 SF (over by 600)

The token store's `tile-h` is "approaches 1/4 floor tile at 5,000 SF" — i.e. the 4,900 SF Large Tile is a near-miss of the 1/4 floor tile from the April PDF.

### Climate-zone autonomy (token store concept, not in PDFs)

The token store records a design principle that **"one tile = one HVAC zone"** (`tile-system.dtcg.json` line 198–201). This concept is not stated in the three PDFs but is consistent with the way the PDFs treat each cell as an independently-sized leasable unit.

### Building core (operator's question on this)

**Not explicit in the three PDFs.** Page 3 shows a "Lobby Atrium" block at the centre of the main floor diagrams (adjacent to O/P on the Building Manager diagram and adjacent to CC/DD on the Coffee/Bread diagram). No upper-floor diagram shows the core position. Upper-floor corridors (p. 4) are shown as a row of three cells (R/S/T) with no indication of where they sit relative to the building shell.

Inference: the building core (elevators + egress stairs + risers) appears to be in/near the Lobby Atrium block on the main floor and runs vertically; the upper-floor elevator lobby (S = 300 SF) sits between corridor expanders R (50 SF) and T (300/100 SF). But the three PDFs do not draw the core. See Open Questions §7.

---

## Section 5 — The "we need" gap analysis

### 5.1 Key plans documented in the PDFs but NOT in the current token store

| Key plan | Documented where | Status in tokens |
|---|---|---|
| **PO-1 / PO-2 / PO-3** (Private Office cells: 300 / 450 / 500 SF) | Tiles PDF p. 1 | Used as raw SF values inside `tile-system.dtcg.json` (e.g. `tile-b1` composition `[300, 450, 300, 450, 300]`), but no token defines PO-1/PO-2/PO-3 as named entities. `building-width-calculator.dtcg.json` has `key-plan.private-office.{small,medium,large}` with areas 325 / 465 / 685 SF — these are different (Summary tab key-plan areas, not Tile-PDF cell areas). |
| **M-1 / M-2 / M-3, B-1 / B-2 / B-3, L-1 / L-2 / L-3, A-1 / A-2 / A-3, C-1 / C-2 / C-3** (Professional Office cells: 1,100 / 1,400 / 800 SF) | Tiles PDF pp. 1–2 | Present only as `ffe_codes: ["M1","M2","M3"]` arrays with `ffe_sizes_sf` inside `professional-office-subtypes.dtcg.json`. Not declared as standalone key-plan tokens. The Index PDF doesn't use these codes at all. |
| **Corporate Office tile letters A / B / C / D** (1/8 / 1/4 / 1/2 / Full floor placeholders) | Tiles PDF pp. 2–3 | Not in tokens. `tile-system.dtcg.json` has a `tile-a` but it is the 2,700 SF Small Tile family Corporate variant, not the 2,500 SF 1/8 floor letter A from the PDF. |
| **N — Tenant Lounge (10,000 SF, 2nd floor)** | Tiles PDF p. 3; Index PDF #26 (N-1) & #42 (N-2) | Not in tokens. |
| **R / S / T — Corridors / Elevator Lobby (upper floor, 50 / 300 / 100 SF)** | Tiles PDF p. 4; Index PDF R-1 #24, S-1 #30, S-2 #46 | Not in tokens. |
| **U / V / W — Tenant Restrooms / Meter Room / Mop Room (400 / 150 / 150 SF)** | Tiles PDF p. 4; Index PDF entries 25, 31, 47, 48 | Not in tokens. |
| **X / Y / Z / AA / BB — Loading / Recycling / Bike Room / Workbench / Lockers (750 / 400 / 300 / 150 / 200 SF)** | Tiles PDF p. 4; Index PDF entries 32–36, 49–53 | Not in tokens. |
| **CC / DD — Coffee/Bread / Public Restrooms (1,200 / 450 SF)** | Tiles PDF p. 4; Index PDF entries 37–38, 54–55 | Not in tokens. |
| **EE-1 / EE-2 — Lobby Atrium** | Tiles PDF pp. 3–4 (shown as background block); Index PDF #27, #43 | Not in tokens. |
| **Medical specialisations: Chiropractor / Dentist / General Practitioner** | Index PDF #9–#11 | Partially in tokens (`professional-office-subtype.medical.specialisations: ["Dentist", "General Practitioner"]`) but Chiropractor is **missing** from the array (line 16 of `professional-office-subtypes.dtcg.json`). |
| **Laboratory specialisations: Medical / Research** + **L-3 large** | Index PDF #15–#17 | Partially in tokens (Laboratory specialisations: `["Medical Laboratory", "Research Laboratory"]` — name format differs from Index "Medical" / "Research"). |
| **Retail Select tiles A / B / C / D / E / M** (1,400 / 1,000 / 700 / 1,100 / 1,200 / 200 SF) | Tiles PDF p. 5 | Not in tokens. Only the Index entries RA-1 / RB-2 / RC-3 reference Retail Select at all. |
| **Tech Industrial tiles A / B / C / D / M** (2,200 / 2,100 / 1,200 / 1,500 / 200 SF) | Tiles PDF p. 6 | Not in tokens. Only Index entries TI-1 / TI-2 / TI-3 reference Tech Industrial. |
| **Suburban Office common-area key plans** (N-2, EE-2, O-2, P-2, S-2, U-2, W-2, X-2, Y-2, Z-2, AA-2, BB-2, CC-2, DD-2) | Index PDF #42–#55 | Not in tokens. |
| **Landscaping LL-1 (3 eco regions) + LL-2 Irrigation Gallery** | Index PDF #59–#62 | Partially captured in `landscape-parking.dtcg.json` under different naming. |
| **Parking PP-1 through PP-6 (Parking Stalls × 3 eco regions, Accessible × 3, Sidewalks, Snowdrops, Signage, Lighting)** | Index PDF #63–#72 | Partially in `landscape-parking.dtcg.json`. |
| **Corridor Expanders R-1** | Index PDF #24 | Not in tokens. |
| **Meter Room V-1** | Index PDF #25 | Not in tokens. |
| **Tech Industrial Tech Leasehold TI-1/TI-2/TI-3** | Index PDF #56–#58 | Not in tokens. |
| **Reception, Exam Room, Doctor's Office, File Room and Kitchen** (Medical FFE) | Tiles PDF p. 1 | Not in tokens. |
| **Reception, Private Office, Desk, File Room and Kitchen** (Business FFE) | Tiles PDF p. 1 | Not in tokens. |
| **Private Office, Bench, Storage, Clean Room** (Laboratory FFE) | Tiles PDF p. 2 | Not in tokens. |
| **Podium, Desk, Auditorium Seating, Storage** (Academic FFE) | Tiles PDF p. 2 | Not in tokens. |
| **Judge, Court Clerk Desk, Theater Seating, Storage** (Civic FFE) | Tiles PDF p. 2 | Not in tokens. |

### 5.2 Key plans in the token store but NOT documented in the three PDFs (suspect)

| Token | Where it lives | Source claimed by token | Status |
|---|---|---|---|
| **Small Tile family (2,700 SF): Tile A, B-1, C-1, C-2, C-3, C-4, E-1, E-2** | `tile-system.dtcg.json` | Cites `AEC_Floor Plates_Tiles_Alternatives.pdf` (not in this read list) | Out-of-scope source — needs separate confirmation. The 2,700 SF tile increment is not in the April Tiles PDF or the Index PDF. |
| **Large Tile family (4,900 SF): Tile F, G, H** | `tile-system.dtcg.json` | Same source above | Out-of-scope source — needs separate confirmation. |
| **`tile-system.design-principles` — climate-zone-autonomy, rolling-efficiency, corner-handling, professional-to-corporate-jump** | `tile-system.dtcg.json` lines 196–217 | No source cited | Synthesised principles; not in the three PDFs. |
| **`professional-office-subtype.medical.specialisations: ["Dentist", "General Practitioner"]` (missing Chiropractor)** | `professional-office-subtypes.dtcg.json` line 16 | Index PDF #9–#11 | Index PDF lists three: Chiropractor, Dentist, General Practitioner. Chiropractor is missing in tokens. |
| **`professional-office-subtype.business.specialisations: []`** | `professional-office-subtypes.dtcg.json` line 32 | — | Index lists Business as B-1/B-2/B-3 (sizes, no specialisations). Token alignment correct in that respect, but Index does not have B-1/B-2/B-3 as B-Small/B-Medium/B-Large in the filename — see naming discrepancy. |
| **`professional-office-subtype.academic.specialisations: ["Classroom","Seminar Room","Lecture Hall"]`** | `professional-office-subtypes.dtcg.json` line 74 | — | Not in the Index PDF. Index lists Academic as A-1/A-2/A-3 only. Specialisations may come from a different source — should be tagged with a source citation. |
| **`professional-office-subtype.civic.specialisations: ["Courtroom","Municipal Office","Cultural Space","Civic Assembly"]`** | `professional-office-subtypes.dtcg.json` line 100 | — | Same — not in the Index PDF; Index lists Civic only as C-1/C-2/C-3. |
| **Civic Theater Seating count 28** | Tiles PDF p. 2 (so this is in the PDF) | — | Not in tokens — captured here for completeness. |

### 5.3 Dimensions documented in the PDFs but NOT in tokens (need to add)

| Dimension | PDF source | Current token status |
|---|---|---|
| PO-1 = 300 SF (cell) | Tiles PDF p. 1 | Token has Private Office Small = 325 SF (different — Summary tab). PDF cell value not stored. |
| PO-2 = 450 SF (cell) | Tiles PDF p. 1 | Token has Private Office Medium = 465 SF. Cell value not stored. |
| PO-3 = 500 SF (cell) | Tiles PDF p. 1 | Token has Private Office Large = 685 SF. Cell value not stored. |
| M-1 / M-2 / M-3 cell SF (1,100 / 1,400 / 800) | Tiles PDF pp. 1–2 | Stored as `ffe_sizes_sf` in `professional-office-subtypes.dtcg.json` lines 11–14. Already present. |
| Same for B-, L-, A-, C- | Tiles PDF pp. 1–2 | Same — already in `ffe_sizes_sf`. |
| 1/8 Floor headline = 2,500 SF | Tiles PDF p. 1 | Token has `floor-plate.tile-fraction.one-eighth = 232.26 m² / 2,500 SF`. Matches. |
| 1/4 Floor headline = 5,000 SF | Tiles PDF p. 1 | Matches `one-quarter`. |
| 1/2 Floor = 10,000 SF | Tiles PDF p. 3 | Matches `one-half`. |
| Full Floor = 20,000 SF | Tiles PDF p. 3 | Matches `full`. |
| 1/3 Floor (~6,667 SF) | Index PDF #6 | **Not in tokens.** Index lists Corporate 1/3 Floor; no corresponding `tile-fraction.one-third`. |
| Building Manager 450 SF + Mail Room 450 SF = 900 SF main floor | Tiles PDF p. 3 | Token `main-floor.building-manager-mail-room = 83.61 m² / 900 SF`. Matches. |
| Loading 750 + Recycling 400 + Bike Room 300 + Workbench 150 + Lockers 200 = 1,800 SF | Tiles PDF p. 4 | Token `main-floor.loading-recycling = 167.23 m² / 1,800 SF`. Matches **sum** but individual SF values not stored. |
| Coffee/Bread 1,200 + Public Restrooms 450 = 1,650 SF | Tiles PDF p. 4 | Token `main-floor.coffee-bread-public-restrooms = 153.29 m² / 1,650 SF`. Matches sum; individuals not stored. |
| Corridor totals upper floor: R 50 + S 300 + T 100 = 450 SF | Tiles PDF p. 4 | Token `upper-floor-common.corridors = 41.81 m² / 450 SF`. Matches sum (assuming T=100); individuals not stored. |
| U 400 + V 150 + W 150 = 700 SF upper-floor restrooms/utility | Tiles PDF p. 4 | Token `upper-floor-common.restrooms-meters = 65.03 m² / 700 SF`. Matches sum; individuals not stored. |
| Tenant Lounge 10,000 SF (1/2 floor) | Tiles PDF p. 3 | Token `second-floor.tenant-lounge = 929.03 m² / 10,000 SF`. Matches. |
| Retail Select Small / Medium / Large building sizes 4,500 / 6,700 / 7,700 SF | Tiles PDF p. 5 | Not in tokens. |
| Tech Industrial Medium / Large building sizes 7,200 / 8,400 SF | Tiles PDF p. 6 | Not in tokens. |
| **Z1 Habitat = 6 m for Small Private Office (PO-1)** | Methodology PDF | Token `private-office.habitat = 5.9944 m` (marginally under). PDF says exactly 6 m. The 5.9944 m value comes from "Key Plan sample geometry" per token note. |

### 5.4 Conflicts (PDFs vs PDFs, and PDFs vs current state)

**PDF vs PDF conflicts:**

| Conflict | Source 1 | Source 2 |
|---|---|---|
| Private Office naming: PO-1/PO-2/PO-3 vs Small/Medium/Large | Tiles PDF (PO-1/PO-2/PO-3) | Index PDF (Small/Medium/Large with no PO-N) |
| Medical sub-divisions: M-1/M-2/M-3 sizes vs Chiropractor/Dentist/GP specialisations | Tiles PDF (sizes) | Index PDF (specialisations) |
| Laboratory sub-divisions: L-1/L-2/L-3 vs Medical/Research/L-3 | Tiles PDF (L-1/L-2/L-3 all sizes) | Index PDF (Medical, Research = specialisations; L-3 = size — mixed scheme) |
| Tile letter "A" — Corporate 1/8 floor tile (2,500 SF) vs Retail end cap (1,400 SF) vs Tech end cap (2,200 SF) | Tiles PDF p. 2 vs p. 5 vs p. 6 | (same PDF, different sections — namespacing issue) |
| Corridor Expander T = 100 SF (legend) vs T = 300 SF (diagram) | Tiles PDF p. 4 legend | Tiles PDF p. 4 diagram |
| Mop Room — exists only as W-2 in Suburban Office (Index #48) | Index PDF | The Tiles PDF shows W = 150 SF in the Professional Centre Restrooms/Meter/Utility group (p. 4) without -1 / -2 distinction. |
| Civic specialisations — Index lists C-1/C-2/C-3 only | Index PDF #21–#23 | Token store lists Courtroom / Municipal Office / Cultural Space / Civic Assembly — source unknown. |
| Sample tile rows exceed headline tile size by ±5–10% | Tiles PDF (PO row = 2,150 ≠ 2,500; Professional row = 5,200 ≠ 5,000) | Same PDF — "Approximate" disclaimer absorbs the discrepancy. |

**PDF vs current state conflicts:**

| Conflict | PDFs say | Tokens / HTML say |
|---|---|---|
| Private Office Z1 Habitat | 6 m exactly (Methodology) | Tokens 5.9944 m; HTML 5.9944 m |
| Private Office Z3 Corridor | PDFs don't disclose | Tokens 0.0 m (no corridor); HTML 3.0 m ("shared building corridor") |
| Professional Office Z2 / Z3 | PDFs don't disclose | Tokens 3.8 / 2.0 m; HTML 3.0 / 3.0 m (labelled "placeholders") |
| Academic Z3 Corridor | PDFs don't disclose | Tokens 0.0 m; HTML 3.0 m |
| Business Z1 / Z2 / Z3 | PDFs don't disclose | Tokens 6.0 / 7.3 / 2.7 m; HTML 5.51 / 9.26 / 2.75 m |
| Tile vocabulary (small tile / large tile family) | PDFs use A/B/C/D Corporate tile letters | Tokens have A, B-1, C-1, C-2, C-3, C-4, E-1, E-2 (2,700 SF) + F, G, H (4,900 SF). Different scheme. |
| Number of Small Tile B variants | PDF has no B-1 (B is Corporate 1/4 tile) | Tokens have B-1 (Private Office End Cap). |
| Medical specialisations | Index lists 3: Chiropractor, Dentist, GP | Tokens list 2: Dentist, GP (Chiropractor missing) |
| 1/3 Floor Corporate fraction | Index entry #6 | Not in tokens (no `tile-fraction.one-third`) |
| Civic Theater Seating count | PDF Civic Large = 28 seats | Not in tokens |
| Tenant Lounge as upper-floor 2nd-floor 10,000 SF unit | Tiles PDF p. 3 | Tokens have `second-floor.tenant-lounge = 10,000 SF` (matches); not a tile, no use-type zone profile. |

---

## Section 6 — The path forward (recommendation, not a plan to execute)

If the goal is to align the cluster's token store and HTML with the three PDFs as authoritative substrate, the operator could consider the following moves. **This is a recommendation list, not an instruction to act.**

### 6.1 Resolve the naming-scheme question first

The single most impactful decision: which **key-plan identifier scheme** is canonical?

- **(a) Tiles PDF scheme** — PO-1/PO-2/PO-3, M-1/M-2/M-3, etc. Compact, drawn from Tiles PDF, but loses Medical specialisations (Chiropractor/Dentist/GP) and Laboratory specialisations (Medical/Research).
- **(b) Index PDF scheme** — Family + Typology (Private Office Small, Medical Dentist, Tenant Lounge N-1, etc.). Comprehensive (72 entries) and matches the file-naming slugs that will end up on disk, but does not match the size-driven tile composition the Tiles PDF uses.
- **(c) Composite scheme** — Key plan = Family + Typology (Index) with size-cell codes (PO-1, M-1, etc.) as a *secondary* attribute used in tile composition. This is the most expressive but also the most expensive to maintain.

Until this is decided, every other recommendation below is conditional.

### 6.2 Stabilise the HTML vs JSON drift

`preview/building-width-calculator.html` lines 884–905 declares values for Private Office Z3, Professional Office Z2/Z3, Academic Z3, and Business Z1/Z2/Z3 that disagree with `building-width-calculator.dtcg.json`. The HTML claims to be a "mirror" of the JSON but isn't. Either the HTML is being kept on a divergent footing intentionally (placeholders / V12 baseline pending furniture lock) or the JSON has been updated since the HTML was authored. **The operator should decide which is authoritative and the other should follow.**

### 6.3 Cite the actual sources behind `building-width-calculator.dtcg.json` zone depths

The current $description fields name `CONSTRUCTION_2026_01_06_Key Plan_Professional Office_FFE_FIN.xlsx` and `building-width-calculator.docx` as the sources for the per-use-type zone depths. None of the three PDFs in this read list discloses these dimensions. **If those source files are themselves derived from the Methodology PDF + the Samples PDF, a clear chain-of-citation in the token files would let the calculator be defended publicly.** Right now a reader cannot reach the metre values by reading only the three PDFs the operator named.

### 6.4 Add tokens for the Professional Centre common-area key plans

13 common-area key plans (N, O, P, R, S, T, U, V, W, X, Y, Z, AA, BB, CC, DD, EE) appear in the April Tiles PDF with explicit SF and are duplicated for Suburban Office in the Index PDF as -1 / -2 variants. None are in the token store. They would naturally fit a new `bim.key-plan.common-area.{name}` namespace with `area_sf`, `tile_letter`, `floor` (main / upper / second), and `index_entry`.

### 6.5 Decide on the tile vocabulary

The token store's Small Tile (2,700 SF) and Large Tile (4,900 SF) families come from `AEC_Floor Plates_Tiles_Alternatives.pdf` — a sister document in the May 2025 folder. The April Tiles PDF uses a different scheme (Corporate-driven A/B/C/D letters tied to 1/8, 1/4, 1/2, full floor fractions). The two schemes are not mutually exclusive — Alternatives may be a refinement of the April PDF for finer tile granularity — but the relationship is undocumented. **A note in `tile-system.dtcg.json` that says "Tile A in this file refers to the 2,700 SF Small Tile family Corporate variant; the April Tiles PDF's 'A' refers to the 2,500 SF 1/8 floor Corporate tile" would prevent future confusion.**

### 6.6 Add the missing Corporate Office 1/3 fraction

The Index PDF (entry #6) lists Corporate Office 1/3 Floor. The token store's `floor-plate.tile-fraction` has 1/8, 1/4, 1/2, 3/4, full — no 1/3. Either the Index includes 1/3 erroneously or the token store should add it. **Note that 20,000 / 3 = 6,666.67 SF, which doesn't divide cleanly by 2,700 or 4,900 either.**

### 6.7 Fix `professional-office-subtype.medical.specialisations`

Index lists Chiropractor, Dentist, General Practitioner. Token store lists only Dentist and General Practitioner. Add Chiropractor.

### 6.8 Document FFE inventories per Professional Office sub-type

Each sub-type has a Small/Medium/Large furniture table in the Tiles PDF (Reception counts, Exam Room counts, Bench counts, Auditorium seats, etc.). The token store has the *zone depths* and *cell sizes* but not the *furniture roster per typology*. Adding these would make the calculator drive directly from the source.

### 6.9 Resolve the Corridor Expander T discrepancy

Tiles PDF p. 4: legend says T = 100 SF, diagram says T = 300 SF, total of R+S+T = 450 SF only works if T = 100. **Pick one — the legend totals are arithmetically consistent and should win.**

### 6.10 Note the Retail Select and Tech Industrial structures

If Retail Select and Tech Industrial are in scope for the cluster, the Tiles PDF gives complete tile vocabularies (A/B/C/D/E/M for Retail, A/B/C/D/M for Tech) with composition recipes. These could be added as `bim.tile.retail-select.{a..e,m}` and `bim.tile.tech-industrial.{a..d,m}` namespaces. The Index PDF references them at a higher level (RA-1, RB-2, RC-3, TI-1, TI-2, TI-3) — the relationship between the Tiles cells (A, B, C, D, E, M) and the Index entries (RA/RB/RC) is not documented.

---

## Section 7 — Open questions for the operator

### 7.1 Which naming scheme is canonical?

PO-1/PO-2/PO-3 (Tiles PDF) or Private Office Small/Medium/Large (Index PDF)? See §6.1.

### 7.2 What is the relationship between a "key plan" (the Index PDF's 72 entries) and a "tile cell" (the Tiles PDF's PO-1, M-1, etc.)?

Are tile cells *primitive* and key plans *composed* (e.g., "Private Office Small" = one PO-1 cell at 300 SF, but a real leasehold called "Private Office Small" in the token store is 325 SF and includes its own Habitat + Magazine zones)? Or are key plans the primitives and tile cells just the SF cuts shown for diagramming?

### 7.3 Where does the Medical specialisation list (Chiropractor / Dentist / GP) connect to the M-1/M-2/M-3 size triplet?

Is a Dentist always an M-1, M-2, or M-3 depending on practice size? Or are the size codes and the specialisation codes independent dimensions (so e.g., a Dentist-M-2 is valid)?

### 7.4 What is the source of the Academic specialisations (Classroom / Seminar Room / Lecture Hall) and Civic specialisations (Courtroom / Municipal Office / Cultural Space / Civic Assembly) in the token store?

These appear in `professional-office-subtypes.dtcg.json` but are not in any of the three PDFs. The token file cites a generic basket of sources; a specific source per specialisation list would let it be defended.

### 7.5 Is the Methodology PDF's Z1 = 6 m the authoritative Private Office Habitat?

Or is the token store's 5.9944 m (from Key Plan sample geometry, marginally under the European Lighting Standard) the authoritative value? The token note says **"flag for review on final Key Plan sign-off."**

### 7.6 Where is the building core drawn?

None of the three PDFs shows the elevator + stair + riser core in plan. The April Tiles PDF shows "Lobby Atrium" on the main floor adjacent to building services. Should the core be specified as a key plan / token? Today it appears to be implicit space within `floor-plate.tile-fraction.full` (20,000 SF net leasable — gross floor area not stored).

### 7.7 Is the building-width direction the long axis or the short axis?

The Methodology PDF's diagram orients the floor plate with Zone 1 at the top, Zone 3 at the bottom, and Façade Frontage at top and bottom. This implies Zone 1/2/3 stack across the building **depth** (short axis), and a tile composition lays cells across the building **width** (long axis). The April Tiles PDF's strip diagrams (e.g. p. 1's PO-1 / PO-1 / PO-2 / PO-1 / PO-3 / PO-1) appear consistent with this — cells laid horizontally across width. **Confirm this is the intended geometry before fixing dimensions.**

### 7.8 What does "Snowdrops" mean (Index entry #70, Parking PP-4)?

The Index lists Parking Snowdrops PP-4. In a Canadian site context this could mean snow-storage piles, but the term is non-standard in AEC practice. Worth a definition.

### 7.9 What is the J / K / L / M footnote on Tiles PDF p. 3?

> "*Key Plans J, K, L, and M should each be presented as full-size open plans for medical, business, laboratory, and academic tenants"

This implies a planned set of Key Plans J/K/L/M that are not described elsewhere in any of the three documents — placeholders for full-size open-plan variants. Are they superseded by the M-/B-/L-/A- codes in the same PDF? Or are they a parallel concept the cluster should track?

### 7.10 Tile sample row sums vs headline tile sizes

PO sample row (p. 1) sums to 2,150 SF, headlined 2,500. Professional Office sample row (pp. 1–2) sums to 5,200 SF, headlined 5,000. Which one is authoritative when the calculator tiles a floor plate?

---

*End of foundation study.*
