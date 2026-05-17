---
schema: foundry-draft-v1
version: "1.0"
draft_id: topic-bim-zone-depths-per-use-type-2026-05-17
language_protocol: TOPIC
state: ready-for-sweep
originating_cluster: project-bim
target_repo: vendor/content-wiki-projects
target_path: topics/bim/zone-depths-per-use-type.md
audience: operator
bcsc_class: vendor-internal
authored: 2026-05-17T22:00:00Z
authored_by: totebox@project-bim
authored_with: claude-opus-4-7
title: "Zone Depths per Use Type — Sources, Values, Rationale"
research_done_count: 7
research_suggested_count: 6
open_questions_count: 7
research_provenance:
  - "/home/jennifer/sandbox/inputs/project-bim/--- April 01, 2025 -- Collaborators #11 ---/AEC_Floor Plates_Building Width Calculator_V12.pdf (V12, 2025-01-07)"
  - "/home/jennifer/sandbox/inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Medical_Notes copy.pdf (Summary V2, 2025-05-13; M1/M2/M3 dimensioned sketches)"
  - "/home/jennifer/sandbox/inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Business_Notes.pdf (Notes V3, sketches 1-4 + 21 options table)"
  - "/home/jennifer/sandbox/inputs/project-bim/DISCOVERY_MCorp_Sketches_Key Plans_Private Office_Notes copy.pdf (PO-1/PO-2/PO-3 dimensioned sketches)"
  - "/srv/foundry/clones/project-bim/woodfine-bim-library/tokens/bim/building-width-calculator.dtcg.json"
  - "/srv/foundry/clones/project-bim/woodfine-bim-library/tokens/bim/professional-office-subtypes.dtcg.json"
  - "/srv/foundry/clones/project-bim/.agent/plans/tool-buildingwidth-architecture.md"
research_inline: true
notes_for_editor: |
  Living document. The "Per use type" sections are designed to grow as
  source documents land — manufacturer SKU dimensions (Steelcase Ology
  desks, Midmark exam tables, Treston lab benches), regulatory citations
  (EN 12464-1, ArbStättV, IBC §1020), and DISCOVERY sketches for the
  use types not yet covered. Do not collapse the per-use-type sections.
---

# Zone Depths per Use Type — Sources, Values, Rationale

> **Status:** Living document. Updated 2026-05-17.
> **Companion:** `topic-bim-building-width-method.md` for the formula
> and the mirror cross-section.

## 1. Summary table — canonical values

The Z1/Z2/Z3 values below are the **per-side** depths (Z1 and Z2 are
mirrored, Z3 is centreline single). All values trace to the cited
source.

| Use Type | Z1 Habitat | Z2 Magazine | Z3 Corridor | Building Width | Source |
|---|---|---|---|---|---|
| Private Office | 5.9944 m / 19'8" | 1.3716 m / 4'6" | — | **14.73 m / 48'4"** | PO-1/PO-2/PO-3 sketches |
| Academic | 4.7 m / 15'5" | 3.0 m / 9'10" | — | **15.40 m / 50'6"** | Summary V2, 2025-05-13 |
| Professional Office (V12 baseline) | 6.0 m / 19'8" | 3.0 m / 9'10" | 3.0 m / 9'10" | **21.00 m / 68'11"** | V12, 2025-01-07 (Z2/Z3 marked TBD) |
| Laboratory | 6.7818 m / 22'3" | 4.8006 m / 15'9" | 3.048 m / 10'0" | **26.21 m / 86'0"** | Summary V2; IBC high-hazard egress |
| Medical | 7.2819 m / 23'10" | 4.877 m / 16'0" | 2.892 m / 9'5" | **27.20 m / 89'3"** | M3/M1/M2 sketches (286 5/8" + 192" + 113 7/8") |
| Business | 5.51 m / 18'1" | 9.26 m / 30'5" | 2.75 m / 9'0" | **29.30 m / 96'2"** | Notes V3 Option A/A (widest enumerated) |
| Civic | 6.0 m / 19'8" | 7.23 m / 23'9" | 3.6 m / 11'10" | **30.06 m / 98'8"** | No DISCOVERY sketch — synthesised values |

Formula: `Width = 2 × (Z1 + Z2) + Z3`. See
`topic-bim-building-width-method.md` for the derivation.

## 2. Per use type

### 2.1 Private Office (PO-1 / PO-2 / PO-3)

| Zone | Value | Sketch evidence |
|---|---|---|
| Z1 Habitat | **5.9944 m / 19'8"** | "19'-8"" on each of PO-1, PO-2, PO-3 |
| Z2 Magazine | **1.3716 m / 4'6"** | "4'-6"" on each sketch (shallow storage zone, below the dashed centreline) |
| Z3 Corridor | **— (none)** | Each office opens to the shared building corridor (outside the leasehold) |
| Key plan SF | PO-1 = 325, PO-2 = 465, PO-3 = 685 | Sketch labels |
| Key plan m² | 30.19, 43.20, 63.64 | Calculated from SF |
| Building width | **14.73 m / 48'4"** | 2 × (5.9944 + 1.3716) |

Private Office is the narrowest of the use types. The 5.9944 m
Habitat is **marginally under the 6.0 m European Lighting Standard
threshold** — flagged for review at Key Plan sign-off.

Combinations: PO-1 + PO-1 = 60.4 m² (650 SF); PO-1 + PO-2 = 73.4 m²
(790 SF); PO-1 + PO-3 = 93.8 m² (1,010 SF); PO-2 + PO-3 = 106.8 m²
(1,150 SF). These fill the gap between Private Office Large and
Professional Office Small.

### 2.2 Academic (A1 / A2 / A3)

| Zone | Value | Source |
|---|---|---|
| Z1 Habitat | **4.7 m / 15'5"** | Summary V2 |
| Z2 Magazine | **3.0 m / 9'10"** | Summary V2 (NB: the source spreadsheet shows "3'7"" in feet, which is a transposition; 3.0 m = 9'10" is correct) |
| Z3 Corridor | **— (none)** | Suites open to building corridor |
| Key plan m² | Small 87.7 / Medium 240.3 / Large 376.3 | Key Plans Samples V2 |
| Key plan SF | Small 944 / Medium 2,586 / Large 4,050 | (see inconsistency note below) |
| Building width | **15.40 m / 50'6"** | 2 × (4.7 + 3.0) |

Academic compresses Habitat below 6 m because **seating faces forward**
(toward a podium/board) rather than toward the façade. The European
Lighting Standard applies to workstations; auditorium-style seating
is not workstation use.

**Inconsistency:** the building-width-calculator.dtcg.json file
records Academic Small as **105 m² / 1,131 SF**, while the
professional-office-subtypes.dtcg.json file records it as
**87.7 m² / 944 SF**. The Key Plans Samples V2 (Collaborators #32,
2025-03-03) is the older source for the 87.7 figure; the Summary V3
spreadsheet (2025-11-29) is the source for 105 m². Architecture plan
inconsistency #1 — pending operator decision on which is canonical.

### 2.3 Professional Office (baseline)

| Zone | Value | Source |
|---|---|---|
| Z1 Habitat | **6.0 m / 19'8"** | V12 (Jan 2025) |
| Z2 Magazine | **3.0 m / 9'10"** | V12 — *marked "TBD"; placeholder* |
| Z3 Corridor | **3.0 m / 9'10"** | V12 — *marked "TBD"; placeholder* |
| Key plan m² | Small 130.06 (only) | DTCG token; Medium/Large pending |
| Key plan SF | Small 1,400 (only) | DTCG token |
| Building width | **21.00 m / 68'11"** | V12 stated total ("21 metres / 69 feet") |

Professional Office is the **baseline** use type — the type from
which the other Professional Centre sub-types (Medical, Business,
Laboratory, Academic, Civic) are derived by varying Z1, Z2, and Z3.

V12 explicitly marks Z2 and Z3 as **"TBD"** with values of 3 m as
placeholders. The token store reflects this. When the Professional
Office Key Plan is completed, the operator will lock Z2 and Z3 to
match the furnished layout.

### 2.4 Laboratory (L1 / L2 / L3)

| Zone | Value | Source |
|---|---|---|
| Z1 Habitat | **6.7818 m / 22'3"** | Summary V2 |
| Z2 Magazine | **4.8006 m / 15'9"** | Summary V2 |
| Z3 Corridor | **3.048 m / 10'0"** | Summary V2 (IBC high-hazard occupancy minimum) |
| Key plan m² | Small 195.0 / Medium 316.0 / Large 400.7 | Samples V2 |
| Key plan SF | Small 2,099 / Medium 3,401 / Large 4,313 | Samples V2 |
| Building width | **26.21 m / 86'0"** | 2 × (6.7818 + 4.8006) + 3.048 |

Laboratory's wider Z1 accommodates bench depth (Treston TP-915
≈ 900 mm) and fume-hood clearance (≈ 940 mm + 1,500 mm egress in
front of the hood). The 10'0" corridor is the IBC high-hazard
occupancy egress minimum.

Specialisations: Medical Laboratory, Research Laboratory.

### 2.5 Medical (M1 / M2 / M3)

| Zone | Value | Sketch evidence |
|---|---|---|
| Z1 Habitat | **7.2819 m / 23'10"** | "286 5/8"" on M1/M2/M3 (= 7.282 m); Summary V2 shows 7.2 m as rounded display |
| Z2 Magazine | **4.877 m / 16'0"** | "192"" on M1/M2/M3 (= 4.877 m); Summary V2 shows 4.9 m / "16" (typo for 16'0") |
| Z3 Corridor | **2.892 m / 9'5"** | "113 7/8"" on M1/M2/M3 (= 2.892 m); Summary V2 shows 2.9 m |
| M3 (Small, 2 dental chairs) | **2,401.5 SF** / 223 m² | Sketch label |
| M1 (Medium, 4 dental chairs) | **3,567.7 SF** / 331 m² | Sketch label |
| M2 (Large, 6 dental chairs) | **5,231.4 SF** / 486 m² | Sketch label |
| Building width | **27.20 m / 89'3"** | 2 × (7.2819 + 4.877) + 2.892 |

Medical has the widest Habitat across the use types. The depth
accommodates exam-table depth (Midmark 626 ≈ 790 mm) plus patient
seating, plus clinician circulation, plus a reception window facing
the façade. The 2.892 m corridor is the ADA/CSA-B651 stretcher-and-
wheelchair clearance.

Hand-annotated operator notes on the M3 sketch flag:

- "Add second door for emergencies" — egress secondary
- "Line of sight — safety — reception needs to see the door"
- "Extend office into Zone #2" — operator commentary on layout flex
- "Move seats down out of Zone #1" — Habitat is desk-only

Specialisations: Dentist, General Practitioner.

**Furniture references on sketch:** "KaVo uniQa 96 1/2" × 114 7/8""
— dental treatment unit; cited as the Habitat driver for Medical
Z1 sizing. Specific equipment list from operator notes: mechanical
room, workbench with 2-3 seats and sink, autoclave, imaging room,
file room, storage room, inventory.

### 2.6 Business (B1 / B2 / B3)

The Business use type has **21 enumerated width options** per Notes
V3 (Sketches 1-4 plus Options A/B/C/D table). The token store records
the widest symmetric option as the default:

| Zone | Value | Source |
|---|---|---|
| Z1 Habitat | **5.51 m / 18'1"** | Notes V3 Option A |
| Z2 Magazine | **9.26 m / 30'5"** | Notes V3 Option A (widest enumerated) |
| Z3 Corridor | **2.75 m / 9'0"** | Notes V3 (fixed across all 21 options) |
| Key plan SF | Small 3,350 / Medium 4,302 / Large 7,524 | Sketches 2-4 (3,349.9 / 4,301.90 / 7,523.97 SF) |
| Building width | **29.30 m / 96'2"** | 2 × (5.51 + 9.26) + 2.75 (Option A/A in Notes V3: 32.29 m — see below) |

**Important caveat:** the Notes V3 *Building Width Options* table
enumerates 21 combinations (Option A/A through D/D), where the two
letters denote the left-side and right-side Z1/Z2 choices. The
narrowest Business option is **25.29 m (D/D)** and the widest is
**32.29 m (A/A)**. The asymmetric options (e.g. A/B with Z1_left
= 7.39 m and Z1_right = 5.51 m) are also enumerated but generally
disfavoured for tile composition because demising walls then have
to absorb the asymmetry.

The current token uses Option A's per-side dimensions (5.51 / 9.26 /
2.75) computed symmetrically: 2 × 5.51 + 2 × 9.26 + 2.75 = 32.29 m.
The HTML preview rounds intermediate display.

**Operator note from Notes V3:** "Each of the Key Plans for Small,
Medium, and Large for all of the Use Cases we are examining are too
large — Business, Academic, Laboratory, Medical, and Civic." The
key-plan areas are intentionally generous; scaling down is deferred.

### 2.7 Civic (C1 / C2 / C3)

| Zone | Value | Source |
|---|---|---|
| Z1 Habitat | **6.0 m / 19'8"** | Synthesised (no completed sketch) |
| Z2 Magazine | **7.23 m / 23'9"** | Synthesised (token store value) |
| Z3 Corridor | **3.6 m / 11'10"** | Synthesised — public assembly egress |
| Key plan m² | Small 270 / Medium 577 / Large 822 | Token store (samples-derived) |
| Key plan SF | Small 2,912 / Medium 6,215 / Large 8,850 | Token store |
| Building width | **30.06 m / 98'8"** | 2 × (6.0 + 7.23) + 3.6 |

**Civic does not yet have a DISCOVERY sketch.** Z1/Z2/Z3 values in
the token store are synthesised from the design intent (public
records storage; widest corridor for ceremonial circulation and
public assembly egress). The Civic key-plan areas are derived from
Samples V2 but the zone depths are not field-verified.

Specialisations: Courtroom, Municipal Office, Cultural Space,
Civic Assembly.

## 3. The 0.7 m perpendicular-desk addition

Applies to **any Z1 Habitat** that arranges desks perpendicular to
the façade (rather than parallel). Summary V2 directs:

> An extra 0.7 metres is required for proper circulation for three
> desks in series in Zone 1—Habitat. Without accounting for the
> circulation, 6 metres exactly to the façade results in only two
> desks perpendicular to the façade in Zone 1—Habitat rather than
> three desks.

Token: `bim.circulation-addition.perpendicular-desk = 0.7 m`.

Effective Z1 = 6.0 + 0.7 = **6.7 m** when perpendicular desks are
required to seat three in series.

The architecture plan (item 5) flags that the token's `$description`
does not currently disclose this is a **delta** (additional depth)
rather than a standalone clearance. To be amended.

## 4. Floor-plate target ranges

From Notes V3, MW commentary on Option D:

> The overall area of the Fixed Floor Plates [is] Professional
> Centres at approximately **19,000 SF to 23,000 SF** and Suburban
> Office at approximately **17,000 SF to 21,000 SF**.

The current token store hard-codes 20,000 SF as `floorPlate.netLeasableSF`.
Architecture plan inconsistency #4: convert to a range with PC and SU
variants.

## 5. Future research

- [ ] Confirm Academic Small key plan area: **87.7 m² (944 SF)** per
      Samples V2 vs. **105 m² (1,131 SF)** per DTCG token store
      (architecture plan inconsistency #1)
- [ ] Complete Civic DISCOVERY sketch to anchor Z1/Z2/Z3 values to
      a real furniture arrangement (courtroom, witness stand, jury
      box, ceremonial circulation)
- [ ] Confirm operator preference for Business: Option A/A (widest,
      32.29 m, currently in token) versus a balanced option such as
      C/C (27.27 m) or B/B (~28-29 m)
- [ ] Capture manufacturer SKU dimensions in a `furniture.dtcg.json`
      file (Steelcase Ology 762 mm, Midmark 626 ≈ 790 mm, KaVo uniQa,
      Treston TP-915 ≈ 900 mm, Agati judges bench ≈ 1,200 mm) — see
      architecture plan §"Missing DTCG token files"
- [ ] Complete Professional Office Z2/Z3 (currently V12 placeholders
      at 3 m / 3 m TBD) — values will follow from the Professional
      Office Key Plan once finalised
- [ ] Document the 0.7 m perpendicular-desk supplement as an
      effective-Habitat modifier rather than a separate token, OR
      amend the token description to disclose it is a delta
- [ ] Verify whether the 6.0 m Habitat tracks a specific clause in
      EN 12464-1 (2.4 m window-head × 2.5 daylight factor pad) — the
      research trail cites this but the precise clause reference is
      not yet in the token file

## 6. Source-document inconsistencies (open)

| # | Issue | Files | Action |
|---|---|---|---|
| 1 | Academic Small area: 105 m² (DTCG) vs 87.7 m² (subtypes DTCG / Samples V2) | building-width-calculator.dtcg.json, professional-office-subtypes.dtcg.json | Operator decision needed |
| 2 | Medical Z1 token = 7.2 m, sketch = 286 5/8" = 7.28 m, Summary display = 23'10" | building-width-calculator.dtcg.json | HTML preview updated to 7.2819 m (2026-05-17); token file pending update |
| 3 | Professional Office Z2 = 3.8 m and Z3 = 2.0 m in HTML preview (prior to 2026-05-17 fix); V12 says 3 m / 3 m TBD | preview HTML now reverted to V12 baseline 3.0/3.0 | Token file values to confirm |
| 4 | Floor-plate hard-coded 20,000 SF; Notes V3 directs 19,000–23,000 SF (PC) and 17,000–21,000 SF (SU) | floor-plate-standards.dtcg.json | Convert to range token |
| 5 | 21 m PDF total (V12) vs 21.6 m token arithmetic for Professional Office (with prior 3.8/2.0 zones) | both | Reconciled 2026-05-17 by restoring V12 values |

## 7. Related documents

- `topic-bim-building-width-method.md` — the formula and the
  mirror cross-section
- `plans/tool-buildingwidth-architecture.md` — Rust engine
  architecture and DTCG file gaps
- `preview/building-width-calculator.html` — slide deck preview

## Appendix A — Source-table verbatim (Summary V2, 2025-05-13)

Per the Summary V2 table on page 1 of `DISCOVERY_MCorp_Sketches_Key
Plans_Medical_Notes copy.pdf`:

```
Building Width Calculator (m / ft)              | KP Lab    | KP Acad   | KP Bus | KP Med    | KP Civ
Zone 1 — Habitat **                            | 6.8 22'3" | 4.7 15'5" | —      | 7.2 23'10"| —
Zone 2 — Magazine                              | 4.8 15'9" | 3.0 3'7"* | —      | 4.9 16"** | —
Zone 3 — Corridor                              | 3.0 10'0" | —         | —      | 2.9 9'5"  | —
```

*\* 3'7" is a feet-display typo; 3.0 m = 9'10".*
*\*\* 16" is a display truncation of 16'0".*
*\*\* Note (Habitat): With desks placed perpendicular to the façade, be
careful, as an extra 0.7 metres is required for proper circulation for
three desks in series in Zone 1—Habitat. Without accounting for the
circulation, 6 metres exactly to the façade results in only two desks
perpendicular to the façade in Zone 1—Habitat rather than three desks.*
