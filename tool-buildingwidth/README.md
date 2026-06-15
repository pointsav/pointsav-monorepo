# tool-buildingwidth

**Building Width Calculator** — Woodfine Key Plans V3 (January 2026).

Computes double-loaded floor plate width from Key Plan zone depths.

## Formula

```
full_width = 2 × (Z1 + Z2) + Z3
```

| Zone | Name | Definition |
|---|---|---|
| Z1 | Habitat | Workstation zone — all desks within 6 m of façade (European Lighting Standard) |
| Z2 | Magazine | Storage zone beyond Z1 |
| Z3 | Corridor | Central corridor shared by both sides of the floor plate |

## Zone depths — V3 Jan 2026

| Category | Z1 (m) | Z2 (m) | Z3 (m) | Half-width (m) | Full width (m) |
|---|---|---|---|---|---|
| private-office | 6.0 | 3.8 | 2.0 | 9.8 | 21.6 |
| medical | 7.2 | 4.9 | 2.9 | 12.1 | 27.1 |
| business | 6.0 | 7.3 | 2.7 | 13.3 | 29.3 |
| laboratory | 6.8 | 4.8 | 3.0 | 11.6 | 26.2 |
| academic | 4.7 | 3.0 | 0.0 | 7.7 | 15.4 |
| civic | 6.0 | 7.2 | 3.6 | 13.2 | 30.0 |

Source: `DISCOVERY_MCorp_Sketches_Key_Plans_Summary.pdf` (V3).
Confirmed against `CONSTRUCTION_2026_01_06_Key_Plan_Professional_Office_FFE_FIN.xlsx`.

## Usage

```bash
# Category mode — uses V3 zone table
tool-buildingwidth --category private-office
tool-buildingwidth --category private-office --area 30.19

# Custom mode — explicit zone depths
tool-buildingwidth --z1 6.0 --z2 3.8 --z3 2.0 --area 43.20

# JSON output for pipeline use
tool-buildingwidth --category medical --area 223 --format json
```

## Output example

```
Building Width Calculator — private-office

Zone depths
  Z1 – Habitat    6.00 m  (19.7 ft)
  Z2 – Magazine   3.80 m  (12.5 ft)
  Z3 – Corridor   2.00 m  (6.6 ft)

Floor plate (double-loaded)
  Half-width      9.80 m  (32.2 ft)  [Z1 + Z2, per side]
  Full width     21.60 m  (70.9 ft)  [2 × (Z1 + Z2) + Z3]

Leasable area     30.19 m²  (325.0 SF)
Façade frontage   3.08 m  (10.1 ft)
```

## Build

```bash
cargo build -p tool-buildingwidth
```

Binary: `target/debug/tool-buildingwidth`
