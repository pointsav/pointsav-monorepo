---
artifact: brief
schema: foundry-brief-v1
archive: project-bim
topic: tool-keyplan
status: active
created: 2026-05-22
updated: 2026-07-06
---

# Brief — tool-keyplan (BIM Key Plan DTCG Compiler)

## What this is

`tool-keyplan` is a Rust CLI that implements the AEC Key Plan methodology:

> 1. Place a dot. 2. Lay furniture around it. 3. Move furniture to satisfy
> German Circulation Law + European Lighting Standard. 4. Use real
> manufacturer SKUs. 5. Check wheelchair accessibility.
> 6. Ensure sufficient facade frontage. 7. Draw bounding box → compute area.

**Input:** TOML config (furniture placements + zone geometry)
**Input:** `interior.dtcg.json` (furniture BIM Objects with Steelcase specs)
**Output:** DTCG JSON — validated key plan entry with structured `furniture_refs`,
computed bounding box, and compliance record

## Files

| File | Status | Description |
|---|---|---|
| `pointsav-monorepo/tool-keyplan/Cargo.toml` | committed | Crate manifest (v0.0.1) |
| `pointsav-monorepo/tool-keyplan/src/main.rs` | committed | Engine implementation |
| `pointsav-monorepo/tool-keyplan/configs/po-1.toml` | committed | PO-1 input config |
| `pointsav-monorepo/Cargo.toml` | committed | tool-keyplan added to members |
| `woodfine-bim-library/tokens/bim/interior.dtcg.json` | committed | Furniture + circulation BIM Objects (canonical location) |
| `woodfine-bim-library/tokens/bim/key-plans.dtcg.json` | committed (PO-1 structured furniture_refs applied 2026-05-23) | Canonical location |

**Update 2026-07-06:** the `pointsav-design-system` misrouted copies noted
below are moot — that repo no longer exists as a clone in this archive.
`woodfine-bim-library` has been canonical since the 2026-05-24 migration
(`cleanup-log.md`).

## Furniture BIM Objects — PO-1 (interior.dtcg.json)

All Steelcase product data retrieved from steelcase.com / coalesse.com on 2026-05-22.

| Token | Model | W mm | D mm | H mm | Weight |
|---|---|---|---|---|---|
| `bim.interior.furniture.task-chair.steelcase-leap-v2` | Leap V2 (462 Series) | 686 | 629 | 978–1099 | null (33 kg gross) |
| `bim.interior.furniture.desk.steelcase-migration-se-58x29` | Migration SE 58"×29" | 1473 | 737 | 574–1237 | 46.7 kg |
| `bim.interior.furniture.table.steelcase-groupwork-36` | Groupwork 36" Round | 914 | 914 | 724 | null |
| `bim.interior.furniture.storage.steelcase-ts-mobile-pedestal` | TS Series Mobile Pedestal | 387 | 559 | 533 | null |
| `bim.interior.furniture.storage.steelcase-currency-bookcase-36` | Currency Enhanced Bookcase 36" | 914 | 381 | 1846 | null |
| `bim.interior.furniture.lounge-chair.coalesse-wing-ch445` | Wing Chair CH445 | 899 | 896 | 1031 | 36 kg |
| `bim.interior.furniture.utility.generic-coat-rack` | Generic Coat Rack | 400 | 300 | 1800 | null |

Lounge Chair CH445 is in interior.dtcg.json but NOT in po-1.toml (not in FFE spreadsheet for PO-1).

## Circulation constraint token

`bim.interior.circulation.standard-private-office` (`$type: bim.circulation-constraint`)

| Constraint | Value | Code |
|---|---|---|
| Min room area / person | 8.0 m² | ASR A1.2 §4.1 |
| Min aisle (single) | 875 mm | ArbStättV §12 |
| Min aisle (two-way) | 1200 mm | ArbStättV §12 |
| Wheelchair turning radius | 750 mm (diameter 1500 mm) | DIN 18040 |
| Desk-to-window max | 6.0 m | EN 12464-1 |

## CLI usage

```bash
# Build
CARGO_TARGET_DIR=/srv/foundry/cargo-target/jennifer cargo build -p tool-keyplan

# Validate only (no output file)
./target/release/tool-keyplan \
  --interior woodfine-bim-library/tokens/bim/interior.dtcg.json \
  --config tool-keyplan/configs/po-1.toml \
  --validate-only

# Compile full DTCG output
./target/release/tool-keyplan \
  --interior woodfine-bim-library/tokens/bim/interior.dtcg.json \
  --config tool-keyplan/configs/po-1.toml \
  --output /tmp/po-1-out.dtcg.json
```

## PO-1 constraint check (expected output)

```
Loaded 6 furniture tokens from interior.dtcg.json
Compiling PO-1 — Private Office — Small
  ASR A1.2 ✓ — 30.19 m² / 1 = 30.19 m²/person ≥ 8.0 m² minimum
  European Lighting ✓ — all Zone 1 furniture within 6.0 m of facade
  Wheelchair ✓ — plan width 2559 mm ≥ 1500 mm (2 × 750 mm radius)
PO-1: ALL CONSTRAINTS SATISFIED
  Bounding box: 2559 mm × 11800 mm = 30.19 m²
Written → /tmp/po-1-out.dtcg.json
```

## Pending for this crate

- [ ] Refine po-1.toml positions from `DISCOVERY_MCorp_Sketches_Key Plans_Private Office.pdf`
- [ ] Extend to PO-2 (po-2.toml) and PO-3 (po-3.toml) using same furniture tokens
- [x] Update `key-plans.dtcg.json` PO-1 entry to use structured `furniture_refs` — done 2026-05-23
- [ ] Aisle clearance check (875 mm between furniture bounding boxes) — v0.0.2
- [ ] Extend to Medical, Laboratory, Business, Academic, Civic interior.dtcg.json tokens

**Update 2026-07-06:** functionally unchanged since v0.0.1 — only formatting/
clippy/licensing housekeeping commits since 2026-05-22
(`96225980`, `add53f64`, `78384aae`, `ee23105b`). None of the "Pending" items
below have been picked up.

The 3 early commits on this crate (`8ce0b9ba`, `a4ba3e96`, `1608fa26`) that
were flagged 2026-07-03 as needing "a dedicated reconciliation session" against
canonical were verified 2026-07-06 to be a non-issue: their content is already
on `origin/main` under the housekeeping commits listed above. No reconciliation
work is actually needed — see NEXT.md.

## History

| Version | What changed |
|---|---|
| 0.0.1 | Initial scaffold — engine process, ASR/lighting/wheelchair validation, DTCG output |
