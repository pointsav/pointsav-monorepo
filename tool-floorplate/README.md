# tool-floorplate

**Floor Plate Composition** — Woodfine Key Plan Tile algebra (January 2026).

Computes floor plate breakdown from development class and gross area:
building core, net leasable area, tile count, CO Key Plan fractions, and
building totals.

## Development classes

| Class | Floors | Gross floor plate |
|---|---|---|
| `professional-centre` | 3–5 | 19,000–23,000 SF |
| `suburban-office` | 6–9 | 17,000–21,000 SF |
| `retail-select` | 1 | Tile = Floor Plate (S/M/L) |
| `tech-industrial` | 1 | Tile = Floor Plate (M/L) |

For Professional Centre and Suburban Office:
- Building core: 13% of gross area
- Net leasable: 87% of gross area
- 1 Tile (CO-1/8) = 2,500 SF leasable
- CO-FF = 20,000 SF (full floor, 8 tiles)

## CO Key Plan fractions

| Label | Leasable | Tiles |
|---|---|---|
| CO-FF | 20,000 SF | 8 tiles — full leasable floor |
| CO-1/2 | 10,000 SF | 4 tiles |
| CO-1/3 | 6,667 SF | 2–3 tiles |
| CO-1/4 | 5,000 SF | 2 tiles |
| CO-1/8 | 2,500 SF | 1 tile — smallest unit |

## Tile algebra

```
T_Basic    = n × PO-S (325 SF) + p × PO-M (465 SF) + q × PO-L (685 SF)
T_Compound = T_Basic + amenity Key Plans
T_Special  = T_Basic + corner / elevator lobby Key Plans
Floor Plate = T_Basic + T_Compound + T_Special + Building Core
```

Source: `inputs/CONSTRUCTION_MCorp_2026_01_06_Tiles_Leasing Plan Efficiencies_FIN.docx`.

## Usage

```bash
# Class midpoint (default area)
tool-floorplate --class professional-centre

# Specific gross area
tool-floorplate --class professional-centre --area 21000

# With floor count — appends building totals
tool-floorplate --class suburban-office --area 19500 --floors 7

# Retail Select tile sizes
tool-floorplate --class retail-select

# JSON output for pipeline use
tool-floorplate --class professional-centre --area 21000 --floors 4 --format json
```

## Build

```bash
cargo build -p tool-floorplate
```
