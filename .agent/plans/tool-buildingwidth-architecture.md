# tool-buildingwidth + tool-floorplates — Architecture & Gap Register

> **Status:** Research complete (2026-05-17, Opus agent army).
> **Next:** Create missing token files → scaffold Rust crates.
> **HTML preview:** `/home/jennifer/sandbox/outputs/project-bim/html/`

---

## What the system is

The Key Plans and Tiles system builds floor plate dimensions **backwards from real manufacturer
furniture**:

```
Real furniture SKUs (Steelcase, Midmark, Kewaunee...)
        ↓
Zone depths (Habitat + Magazine + Corridor) per use type
        ↓
Building width = 2×(Z1 + Z2) + Z3
        ↓
Tile width → tile area → floor plate composition
        ↓
Net leasable SF, building length, tenant demising layout
```

Precision is non-negotiable. ±5–10 ft on a floor plate can break tile composition and make
tenant demising impossible. The system must be exact — not approximate.

---

## Two Rust tools

| Tool | Role | Input | Output |
|---|---|---|---|
| `tool-buildingwidth` | Compute building width from use-type tokens | Use type + optional zone overrides | Width (m + ft), validation, provenance |
| `tool-floorplates` | Assemble a valid floor plate from tenant requirements | Tenant mix + target SF + width | Tile composition, length, door count, climate zones |

Both are `lib.rs` + `[[bin]]` — usable as library (future Tauri UI) or CLI.

---

## Workspace layout (under `pointsav-monorepo/crates/`)

```
crates/
  bim-units/            # unit-safe types: Meters, SquareMeters, UseType enum
  bim-tokens/           # DTCG loader + typed model (shared, no business logic)
  bim-furniture/        # furniture catalogue + backwards zone derivation
  tool-buildingwidth/   # zone computation + width calc + bidirectional adjust
  tool-floorplates/     # floor-plate assembly solver (ILP via good_lp)
```

---

## Missing DTCG token files — create these next

### 1. `furniture.dtcg.json` ← **highest priority**

This is the missing foundation. Currently the zone depths are results with no
derivation. This file encodes the manufacturer SKUs that produced each zone depth.

**Contents:**
- `bim.furniture.office` — Steelcase Ology/Think desks (762mm depth = Zone 1 driver),
  Steelcase Coalesse 3-person round table (1,067mm = PO-1 Zone 1 driver),
  credenza (508mm + 600mm door swing = Zone 2 driver),
  bookcase (305mm = Zone 2 driver), filing cabinet (635mm + 750mm pull = Zone 2 driver)
- `bim.furniture.medical` — Midmark 626 exam table (790mm depth, ADA compliant)
- `bim.furniture.laboratory` — Treston TP-915 bench (900mm), fume hood (940mm + 1,500mm egress)
- `bim.furniture.academic` — tablet-arm chair row pitch (965mm)
- `bim.furniture.civic` — Agati judges bench (1,200mm depth)
- `bim.clearance` — ArbStättV workstation clearance (1.0m), ASR A1.8 corridor tiers,
  ADA wheelchair turn (1,525mm), IBC egress minima by occupancy,
  EN 12464-1 max workstation-to-window (6.0m)

**Key derivation to encode (Professional Office Z1 = 6.0m):**
```
3 × Steelcase Ology 762mm desk (perpendicular, in series)   = 2.286m
ArbStättV 1.0m movement clearance behind seat               = 1.000m
ASR A1.8 ≤20-person aisle                                   = 1.000m
EN 17037 daylight buffer (2.4m window head × 2.5)           = 1.714m
                                                             ──────
                                                              6.000m ✓
```

The 0.7m perpendicular-desk addition is the **delta** (1.0m ArbStättV − 0.3m parallel baseline),
NOT a standalone clearance. The current `$description` is misleading — amend it.

### 2. `floor-plate-assembly-rules.dtcg.json`

Machine-readable rules for valid floor plate composition. Currently all rules are narrative
in PDFs. Without this file, `tool-floorplates` cannot validate a composition.

**Rules to encode:**
- `FP-SUM-001` — tiles + special + core + amenities must sum to net leasable ± 100 SF
- `FP-ENDCAP-001` — end-cap tiles at short sides only; must have natural light on both perpendicular axes
- `FP-CORE-001` — Offset Pulled Back Core mandatory; min 18m from short end
- `FP-SNAP-001` — special-tile width snaps to nearest PO key-plan width (tolerance ±50 SF)
- `FP-CLIMATE-001` — one tile = one HVAC zone
- `FP-DOORS-001` — max 10 doors/tile; max 80 doors/floor
- `FP-CORNER-001` — small tile at corner triggers structural-grid review

### 3. `building-grid.dtcg.json`

Structural module and tolerance bands — the ±5ft sensitivity numbers.

**Contents:**
- `bim.grid.structural-module` = 1.524m (5 ft) — the unit of bidirectional adjustment
- `bim.grid.facade-wall-thickness` = 0.4m
- `bim.grid.demising-wall-thickness` = 0.15m
- `bim.grid.corridor-partition-thickness` = 0.12m
- `bim.tolerance.fp-width-green-band` = 0.762m (±2.5 ft — Z2 absorbs silently)
- `bim.tolerance.fp-width-yellow-band` = 1.524m (±5 ft — one zone re-derived, review required)
- `bim.tolerance.fp-width-red-band` = 3.048m (±10 ft — tile family changes, demising affected)
- `bim.tolerance.demising-structural` = 0.6m — the gap between PDF total (21m) and
  arithmetic sum of token zones (21.6m) for Professional Office

### 4. Medium Tile family — add to `tile-system.dtcg.json`

Currently missing entirely. Methodology PDF p.2 shows three tile sizes: Small (2,700 SF),
**Medium (3,500 SF)**, Large (4,900 SF). The Medium family contains:
- Tile D — Corporate Office Medium (3,500 SF)
- Tile E, E-1 (Left End Cap Medium), E-2 (Right End Cap Medium)
- Tile F-medium — Professional Office (3× PO Small + PO Medium)

### 5. Special Tiles — add to `tile-system.dtcg.json`

The residual tiles that fill the area around the building core. Currently absent.
- `tile-sp-a` — core-adjacent filler: 300–2,500 SF options; no natural light; snaps to PO key-plan width
- `tile-sp-b` — secondary core-adjacent: 300–900 SF
- `tile-sp-c` — elevator-lobby front: 4,700 SF; constraint: no direct elevator-door alignment

### 6. `tenant-mix.dtcg.json` (move from floor-plate-standards)

Furniture-availability distribution (currently misplaced in `floor-plate-standards.dtcg.json`):
- Small 80%, Medium 10%, Large 10%
- Use-type distribution: Private 40%, Professional 35%, Corporate 25%

---

## Internal inconsistencies to fix before coding

| # | Issue | Files | Action |
|---|---|---|---|
| 1 | Academic Small area: **105 m² (1,131 SF)** in `building-width-calculator.dtcg.json` vs **87.7 m² (944 SF)** in `professional-office-subtypes.dtcg.json` | Both | Reconcile — cite which source is authoritative (V3 vs V2 Samples) |
| 2 | Medical and Civic missing `key_plan_areas_m2` block | `professional-office-subtypes.dtcg.json` | Backfill from DISCOVERY and Key Plan Samples |
| 3 | `professional-office` key-plan only has `.small` (130 m²); `.medium` and `.large` absent | `building-width-calculator.dtcg.json` | Backfill from Methodology / Samples |
| 4 | Floor plate hard-coded at 20,000 SF; math sheet shows **PC: 19,000–23,000 SF; SU: 17,000–21,000 SF** | `floor-plate-standards.dtcg.json` | Convert to range token with PC and SU variants |
| 5 | 0.7m perpendicular-desk `$description` does not disclose it is a delta (1.0m ArbStättV − 0.3m baseline) | `building-width-calculator.dtcg.json` | Amend description |
| 6 | 21m PDF total ≠ 21.6m token arithmetic for Professional Office | `building-width-calculator.dtcg.json` | Add `bim.tolerance.demising-structural = 0.6m` token |

---

## Rust engine — core types

### `bim-units`
```rust
pub struct Meters(pub f64);         // authoritative; from m-string tokens
pub struct SquareMeters(pub f64);   // area; .to_sf() → f64
pub enum UseType {
    PrivateOffice, ProfessionalOffice, Laboratory, Academic,
    Business, Medical, Civic, CorporateOffice,
}
pub enum TileFamily { Small, Medium, Large, Special, Core, Amenity }
pub enum ToleranceBand { Green, Yellow, Red }
```

### `bim-tokens` — DTCG loader
```rust
// Two-pass load: JSON merge → type coerce.
// Pass 1: deep-merge all *.dtcg.json in tokens/bim/; record provenance per JSON pointer.
// Pass 2: resolve {token.references}; deserialise into typed BimRoot.

pub struct TokenStore { pub root: BimRoot, pub provenance: HashMap<JsonPointer, PathBuf> }
impl TokenStore {
    pub fn load_dir(path: &Path) -> Result<Self, TokenLoadError>;
    pub fn zone(&self, use_type: UseType) -> Result<&ZoneTriple, TokenError>;
}
// DimensionToken parses "6.0m", "5.9944m", "7.2m", "19'8\"" → Meters
```

### `tool-buildingwidth` — key functions
```rust
pub fn compute_width(
    tokens: &TokenStore,
    use_type: UseType,
    zone_overrides: Option<ZoneOverrides>,
) -> Result<BuildingWidthResult, ComputeError>
// Formula: centerline = 2*(Z1+Z2) + Z3; overall = centerline + walls

pub fn adjust(
    tokens: &TokenStore,
    use_type: UseType,
    target_width_m: Meters,
) -> Result<AdjustmentPlan, ComputeError>
// Zone priority: Z1 LOCKED (EN 12464-1) → Z3 LOCKED (IBC) → Z2 yields first
// Bands: ±2.5ft Green, ±5ft Yellow (review), ±10ft Red (tile family change)
```

### `tool-floorplates` — assembly solver
```rust
pub fn fit_floor_plate(
    tokens: &TokenStore,
    request: FloorPlateRequest,
) -> Result<Vec<FloorPlateConfig>, SolverError>
// Algorithm:
// 1. Derive L = net_leasable / width
// 2. Reserve core (Offset Pulled Back Core, ≥18m from short end)
// 3. Place end caps (B-1/E-1 left, B-2/E-2 right; natural light required)
// 4. Compose mid-tiles: bounded ILP via good_lp (≤30 tiles — exhaustive viable)
// 5. Fill residual with special tiles; apply FP-SNAP-001
// 6. Run all FP-* rules; collect ValidationResult
// 7. Return top-K configs sorted by utilisation
```

### Bidirectional adjustment logic (the ±5ft invariant)
```
Zone 1 Habitat:  LOCKED downward by EN 12464-1 (≥6.0m) — never shrinks
Zone 3 Corridor: LOCKED downward by IBC/NBC egress minima per occupancy
Zone 2 Magazine: YIELDS FIRST — only zone with no hard regulatory floor

Green band (≤0.762m / 2.5ft): Z2 absorbs silently; no composition change
Yellow band (≤1.524m / 5ft):  Z2 adjusts; key-plan areas recomputed; tile lengths snap;
                                operator review required
Red band    (≤3.048m / 10ft): Tile family changes Small→Medium or Medium→Large;
                                door count and climate zones shift; tenant demising affected
```

---

## Crate dependency graph

```
bim-units         ← no deps
bim-tokens        ← serde, serde_json, thiserror, bim-units
bim-furniture     ← bim-tokens
tool-buildingwidth ← bim-tokens, bim-furniture, clap
tool-floorplates  ← bim-tokens, bim-furniture, tool-buildingwidth, good_lp, clap
```

---

## CLI examples (target UX)

```bash
# Compute width for a use type
$ tool-buildingwidth compute --use-type medical
  Zone 1 Habitat:    7.200 m  (23'7")
  Zone 2 Magazine:   4.870 m  (15'12")
  Zone 3 Corridor:   2.890 m  (9'6")
  Centerline width: 27.030 m  (88'8")
  Overall width:    27.910 m  (91'7")
  EN 12464-1: ✓  IBC stretcher: ✓  ADA: ✓

# Derive zone from furniture (requires furniture.dtcg.json)
$ tool-buildingwidth compute --use-type professional-office --derive-from-furniture
  steelcase-think (762mm) × 3 + ArbStättV (1,000mm) + ASR A1.8 (1,000mm) + EN pad (1,714mm)
  Derived Z1: 6.000m  Token Z1: 6.000m  Delta: 0mm ✓

# Bidirectional — what happens if the floor plate is 5ft too wide?
$ tool-buildingwidth adjust --use-type medical --target-width 26.39m
  Delta: −1.520m  Band: YELLOW
  Strategy: AbsorbInMagazine  Z2: 4.870m → 4.108m (−0.762m each side)
  Consequence: Medical Small key plan 223m² → 215.7m² (warn)
  Floor length grows: 66.58m → 69.91m (+5.0%)

# Assemble a floor plate
$ tool-floorplates fit \
    --class professional-centre \
    --target-sf 20000 \
    --require "2 medical-medium, 3 private-office-small, 1 corporate-quarter"
  Composition: B-1 | C-2(Med) | C-2(Med) | A | SP-A(450) | Core | SP-B(300) | F | E-2
  Total: 20,050 SF (+0.25%)  Doors: 38/80  Climate zones: 12/floor
```

---

## Research provenance

Manufacturer sources confirmed:
- Steelcase Ology (762mm desk depth) → Zone 1 Professional/Business/Civic
- Steelcase Coalesse round table (1,067mm) → Zone 1 Private Office 5.9944m
- Midmark 626 exam table → Zone 1 Medical 7.2m
- Treston TP-915 lab bench (900mm) → Zone 1 Laboratory 6.78m
- 4× tablet-arm chair rows at 965mm → Zone 1 Academic 4.7m

Regulatory sources confirmed:
- EN 12464-1:2021 + EN 17037 → Z1 ≤ 6.0m cap (2.4m window head × 2.5)
- ArbStättV Anhang §3.1 + ASR A1.8 Tabelle 2 → clearances + corridor tiers
- ADA 2010 §305, CSA-B651 → Medical Zone 1 + corridor
- IBC 2021 §1020/§1029 → Lab 3.048m and Civic 3.6m corridors

---

*Written by totebox@project-bim, 2026-05-17. Based on Opus agent research.*
*Plan file location: `.agent/plans/tool-buildingwidth-architecture.md`*
