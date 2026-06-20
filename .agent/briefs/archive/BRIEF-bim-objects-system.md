---
artifact: brief
status: active
archive: project-bim
updated: 2026-06-09
---

# BRIEF — BIM Objects System

4-part BIM Objects architecture for Woodfine Management Corp.
Delivers IFC4 Key Plan compositions, Tile algebra, Floor Plate composition,
and a Rust building-width calculator.

---

## 4-part architecture

```
Key Plans
  ↓ compose into
Tiles (T_Basic / T_Compound / T_Special)
  ↓ compose into
Floor Plates (Tile columns + Building Core)
  ↓ sized by
Building Width Calculator (Rust engine, tool-buildingwidth)
```

### What each layer does

| Layer | Unit | Source |
|---|---|---|
| Key Plan | Smallest BIM Object: one leasable program with furniture | `woodfine-bim-library/key-plans/*.ifc` |
| Tile | Column of Key Plans on one floor | `woodfine-bim-library/tiles/tiles-registry.md` |
| Floor Plate | Full leasable floor: Tiles + Building Core | Deliverable 3 — Rust engine pending |
| Building Width Calculator | Rust tool: given area + zone depths, computes width | Deliverable 4 — Rust engine pending |

---

## Development classes

| Class | Floors | Floor plate | Tile algebra | Key Plans = Climate Zone? |
|---|---|---|---|---|
| Professional Centres | 3–5 | 19,000–23,000 SF | T_Basic / T_Compound / T_Special | No |
| Suburban Office | 6–9 | 17,000–21,000 SF | T_Basic / T_Compound / T_Special | No |
| Retail Select | — | = Tile area | — | Yes |
| Tech Industrial | — | = Tile area | — | Yes |

---

## What is done

### Deliverable 1 — Key Plans

- [x] `key-plans-registry.md` — 66 entries; 18 with confirmed dimensions (2026-05-21)
- [x] PO-1/2/3 IFC4 compositions (2026-06-04)
- [x] M-1/2/3, B-1/2/3, L-1/2/3, A-1/2/3, C-1/2/3 IFC4 compositions (2026-06-09)
- [x] Nightly pipeline: `foundry-bim-furniture.timer` → `run-furniture-pipeline.sh` (active)
- [x] `scripts/generate-key-plans.py` extended to 18 Key Plans with category-specific layout functions

### Deliverable 2 — Tiles

- [x] `tiles/tiles-registry.md` — Tile algebra, Floor Plate composition rule, availability percentages (2026-06-09)

### Infrastructure

- [x] `bim.woodfinegroup.com` — app-orchestration-bim serving `/furniture`, `/key-plans`
- [x] `woodfine-bim-library` origin on GitHub (`woodfine/woodfine-bim-library`)

---

## What is pending

### Key Plans

| Item | Blocker |
|---|---|
| Corporate Office Key Plans (CO-FF/1-2/1-3/1-4/1-8) | Architect dimensions TBD |
| Infrastructure Key Plans (N-1/2, EE-1/2, etc.) | Architect dimensions TBD |
| Retail Select + Tech Industrial Key Plans | Architect dimensions TBD |
| Tiles IFC generation (tile-*.ifc files) | Deferred until tiles-registry ratified |

### Deliverable 3 — Floor Plates

- Floor Plate composition matrix — Rust engine; Phase 4
- Depends on Corporate Office Key Plan sizes from architect

### Deliverable 4 — Building Width Calculator

- Rust CLI tool: `tool-buildingwidth`
- Input: floor area (SF or m²), zone depths (Z1/Z2/Z3), category
- Output: frontage width recommendation
- Phase 4; blocked on Deliverable 3 architecture decisions

---

## Zone dimensions (V3 Jan 2026)

| Category | Z1 (m) | Z2 (m) | Z3 (m) | Small (m²) | Medium (m²) | Large (m²) |
|---|---|---|---|---|---|---|
| Private Office | 6.0 | 3.8 | 2.0 | 30.19 | 43.20 | 63.64 |
| Medical | 7.2 | 4.9 | 2.9 | 223 | 331 | 486 |
| Business | 6.0 | 7.3 | 2.7 | 311 | 400 | 669 |
| Laboratory | 6.8 | 4.8 | 3.0 | 195 | 316 | 401 |
| Academic | 4.7 | 3.0 | 0.0 | 105 | 240 | 378 |
| Civic | 6.0 | 7.2 | 3.6 | 270 | 577 | 822 |

Source: `inputs/DISCOVERY_MCorp_Sketches_Key Plans_Summary.pdf` (V3, Jan 6 2026).

---

## Tile availability per slot (within T_Compound / T_Special)

| Category | Small | Medium | Large |
|---|---|---|---|
| Medical | 40% | 40% | 20% |
| Business | 20% | 60% | 20% |
| Laboratory | 40% | 40% | 20% |
| Academic | 20% | 60% | 20% |
| Civic | 20% | 60% | 20% |

---

## WELL compliance

All Key Plans sized to support tenant WELL Building Standard compliance.
Zone 1 daylighting depths are authoritative; no Tile composition may reduce them.
Source: handwritten annotation on sketch PDFs (Medical, Business, Academic, Civic, V10).

---

## Source files consumed this session (2026-06-09)

| File | Content |
|---|---|
| `inputs/DISCOVERY_MCorp_Sketches_Key Plans_Summary.pdf` | Master zone dimensions — all 6 categories |
| `inputs/Sketches/DISCOVERY_MCorp_Sketches_Key Plans_Medical.pdf` | Medical room program, exam table dimensions |
| `inputs/Sketches/DISCOVERY_MCorp_Sketches_Key Plans_Business.pdf` | Business room program, furniture counts |
| `inputs/Sketches/DISCOVERY_MCorp_Sketches_Key Plans_Academic.pdf` | Academic room program, seminar layout |
| `inputs/Sketches/DISCOVERY_MCorp_Sketches_Key Plans_Laboratory.pdf` | Laboratory room program, bench layout |
| `inputs/Sketches/DISCOVERY_MCorp_Sketches_Key Plans_Civic.pdf` | Civic room program, court room |
| `inputs/PROJECTS_MCorp_Tear Sheet_Floor Plates_Tiles_Combinations.pdf` | Tile algebra, Floor Plate sizes, availability |

Not yet processed (XLSX/DOCX — not readable directly):
- `inputs/CONSTRUCTION_2026_01_06_Key Plan_Professional Office_FFE_FIN.xlsx`
- `inputs/CONSTRUCTION_2026_01_06_Tear Sheet_Floor Plans_Key Plans_Methodology_FIN.xlsx`
- `inputs/CONSTRUCTION_2026_01_06_Tear Sheet_Floor Plates_Tiles_Alternatives_V2_FIN.xlsx`
- `inputs/CONSTRUCTION_MCorp_2026_01_06_Database_Floor Plans_Key Plans_Index_FIN.xlsx`
- `inputs/CONSTRUCTION_MCorp_2026_01_06_Tiles_Leasing Plan Efficiencies_FIN.docx`
- `inputs/CONSTRUCTION_2025_10_31_Design Slides_Openstudio_Woodfine Response.docx`
- `inputs/building-width-calculator.docx`

---

## Committed this session (2026-06-09)

| SHA | Scope | What |
|---|---|---|
| 97747c2e | project-bim archive | ops(cleanup): remove archive contamination |
| 868be90 | woodfine-bim-library | feat: extend key-plan IFC generator (18 files) |
| c050103 | woodfine-bim-library | feat: tiles-registry.md — Deliverable 2 |

Push to origin still pending operator auth (woodfine-bim-library 3 commits ahead).

---

## App-orchestration-bim pending

Two commits on `app-orchestration-bim` still need Stage 6 promotion (Command scope):
- `d412d9f8` — feat: DWG/RFA external links + Cargo.lock fix
- `5acbab54` — feat: /key-plans route + furniture page IFC-first
