---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC-ARCHITECTURE
status: draft
created: 2026-06-01
author: totebox@project-gis
destination: project-editorial
target_repo: media-knowledge-documentation
bcsc_class: no-disclosure-implication
research_done_count: 4
research_suggested_count: 6
open_questions_count: 2
research_provenance: |
  OSM wiki (aerodrome tagging, railway tagging), Overpass API spot checks at 4 PKS test sites,
  multi-agent taxonomy research (VWH retailer categories, EU MRO equivalents), Wikidata chain
  lookup, existing GIS pipeline analysis (taxonomy.py, build-clusters.py, ingest-osm.py).
research_inline: |
  Tier A/B chain classification derived from agent research on industrial-park co-location
  patterns. Würth branch-count (~1,500 EU) from Würth Group annual report citations in agent
  research. Tool rental adjacency strategy (United Rentals/Sunbelt deliberate hardware co-location)
  from industry coverage. Floor & Decor contractor-facing model from company investor materials.
paired_with: TOPIC-vertical-warehouse.es.draft.md
---

# Vertical Warehouse (VWH)

A **Vertical Warehouse** is a 3–6 story multi-storey building used for light manufacturing,
just-in-time logistics, and last-mile delivery in urban or near-urban locations. The building
type stacks functions that would traditionally spread horizontally across a ground-level
industrial precinct: fabrication, component storage, assembly, and outbound dispatch.

Three-letter code: **VWH**. One of three Location Intelligence archetypes alongside
Professional Centres (PRO) and Parking Structures (PKS).

---

## What a Vertical Warehouse does

Typical tenant mix:

- **Light manufacturing** — electronics assembly, robotics integration, paint and coatings
  formulation, precision fabrication
- **Just-in-time delivery** — regional distribution hub for e-commerce and manufacturing
  supply chains; goods arrive by truck, are processed, and depart same-day or next-day
- **Last-mile logistics** — final sort and dispatch for urban delivery zones; proximity to
  dense population required

The vertical form factor is driven by urban land cost. Where a horizontal warehouse would
require 4–8 acres at the urban fringe, a Vertical Warehouse achieves equivalent floor area
on 1–2 acres by going up. Ground-floor truck access and dock levellers are retained; upper
floors are served by freight elevators rated for forklift loads.

---

## Where Vertical Warehouses locate

**Spatial signature:** 5–25 km from the nearest major metro centre, in the
industrial-to-suburban transition zone. Located along multi-lane arterial roads or highway
interchanges with direct truck access. Zoned industrial or light industrial.

**What is nearby:**
- Hardware anchors (Home Depot, Leroy Merlin, OBI, Bauhaus class) — building trades supply
- Auto-parts retailers (AutoZone, O'Reilly, Halfords) — vehicle maintenance for the fleet
- Tool rental branches (United Rentals, Sunbelt, Loxam, Kiloutou) — equipment for fit-outs
- Industrial MRO distributors (Würth, Fastenal, Grainger, Hilti) — fasteners, tools, consumables
- Flooring and tile supply (Floor & Decor, Topps Tiles) — finishing materials, contractor supply
- Lumber and building materials (84 Lumber, Builders FirstSource) — structural supply

**What is NOT nearby:** grocery hypermarkets, lifestyle anchors (IKEA), price clubs
(Costco/Sam's Club in their consumer role). A VWH zone is defined partly by the *absence*
of grocery-anchored retail.

---

## Site selection signals

**Essential — required for a viable VWH site:**

| Signal | Threshold | Rationale |
|---|---|---|
| Highway interchange | ≤2 km | Trucks cannot use local residential roads for daily bulk delivery |
| Industrial landuse neighbours | Adjacent or within 500 m | Zoning compatibility; existing logistics ecosystem |
| Labour catchment | 300,000+ population / 30-min drive | Manufacturing and logistics roles require accessible workforce |
| Freight rail access | ≤2 km (where available) | Just-in-time component delivery; bulk raw material |

**Significant — value-add:**

| Signal | Threshold | Rationale |
|---|---|---|
| Air cargo airport | ≤20 km | Electronics components, rapid replenishment |
| Logistics hub (FedEx/UPS/DHL/Amazon) | ≤5 km | Shared last-mile infrastructure |
| Transit corridor | ≤500 m | Workers without vehicles need bus/rail access |
| Power substation | ≤2 km | Robotics and electronics manufacturing: heavy electrical load |

**Disqualifying:**
- Dense residential immediately adjacent (truck traffic conflict, planning restrictions)
- Flood plain (capital investment at risk; insurance prohibitive)
- Heritage or environmentally protected area (height and access restrictions)
- Inside an existing Professional Centres (PRO) cluster (wrong land use; grocery retail adjacent)

---

## Production data (2026-06-11)

The VWH detection pipeline is production-grade. Hardware stores (10,338 locations across 45
chains) were profiled as proxy anchors; DBSCAN was run on trade-supply POIs without the
hardware anchor (held-out validation); tier rules were calibrated using group-collapse logic
and validated against hardware co-location (73.4% of T1+T2 clusters have a hardware store
within 3 km — above the 55% acceptance threshold).

**6,368 clusters** identified globally across 17 display countries:

| Country | Clusters | Notes |
|---|---|---|
| US | 3,167 | Home Depot / Lowe's / Menards + MRO + auto parts |
| DE | 648 | OBI / Hornbach / Hagebaumarkt + Würth + Bauking |
| GB | 543 | B&Q / Wickes + Screwfix / Toolstation + Travis Perkins |
| CA | 506 | Home Depot / Home Hardware + Fastenal + Princess Auto |
| FR | 420 | Castorama / Leroy Merlin + Loxam / Kiloutou + Point-P |
| NL | 240 | Praxis / Gamma + Boels Rental |
| IT | 226 | Leroy Merlin + Rexel |
| PL | 171 | Leroy Merlin / Castorama + Fastenal |

Tier distribution: T1 (Full Trade Hub) = 852 (13.4%), T2 (Established) = 1,327 (20.8%),
T3 (Emerging / Thin) = 4,189 (65.8%). T3-heavy is expected: most hardware store locations
are thin VWH sites; full trade hubs (MRO + tool rental + builders merchant + auto parts)
are legitimately rare.

**`retail_contamination` flag:** 3,048 clusters (47.9%) have a grocery hypermarket within 1 km.
These are dual-use commercial parks — hardware and grocery adjacent. The flag is informational
and does not exclude clusters from the dataset.

---

## Chain taxonomy

All chains below are ingested and active in the production pipeline (2026-06-11).

### TRADE chains (gate T1/T2 tier logic via group-collapse rules)

| chain_id | Chain | Category | Market |
|---|---|---|---|
| `wurth-de` | Würth | `mro_industrial` | EU-wide |
| `fastenal-us`, `fastenal-ca` | Fastenal | `mro_industrial` | NA |
| `grainger-us` | Grainger | `mro_industrial` | NA |
| `hilti-ch` | Hilti | `mro_industrial` | EU |
| `princess-auto-ca` | Princess Auto | `mro_industrial` | CA |
| `floor-decor-us` | Floor & Decor | `flooring` | US |
| `topps-tiles-uk` | Topps Tiles | `flooring` | UK |
| `united-rentals-us` | United Rentals | `tool_rental` | NA |
| `sunbelt-rentals-us` | Sunbelt Rentals | `tool_rental` | NA |
| `loxam-fr` | Loxam | `tool_rental` | EU |
| `kiloutou-fr` | Kiloutou | `tool_rental` | FR |
| `boels-rental-nl` | Boels Rental | `tool_rental` | NL/EU |
| `hss-hire-uk`, `speedy-hire-uk` | HSS Hire / Speedy | `tool_rental` | UK |
| `travis-perkins-uk`, `jewson-uk`, `selco-uk` | Travis Perkins / Jewson / Selco | `builders_merchant` | UK |
| `point-p-fr`, `bigmat-fr` | Point-P / BigMat | `builders_merchant` | FR |
| `bauking-de`, `raab-karcher-de` | Bauking / Raab Karcher | `builders_merchant` | DE |
| `screwfix-uk`, `toolstation-uk` | Screwfix / Toolstation | `trade_counter` | UK |
| `cef-uk` | City Electrical Factors | `electrical` | UK |
| `rexel-fr` | Rexel | `electrical` | EU |
| `ahlsell-se` | Ahlsell | `electrical` | SE |
| `ferguson-us` | Ferguson | `plumbing` | NA |
| `wolseley-uk` | Wolseley | `plumbing` | UK |

### AUTO chains (combine with TRADE for T2; alone = T3)

| chain_id | Chain | Category | Market |
|---|---|---|---|
| `autozone-us`, `oreilley-auto-us`, `napa-us` | AutoZone / O'Reilly / NAPA | `auto_parts` | NA |
| `atu-de` | ATU | `auto_parts` | DE |
| `norauto-fr` | Norauto | `auto_parts` | FR |
| `halfords-uk` | Halfords | `auto_parts` | UK |

### SUPPORT chains (informational — never gate tier)

| chain_id | Chain | Category | Market |
|---|---|---|---|
| `sherwin-williams-us` | Sherwin-Williams | `paint` | NA |
| `comex-mx` | Comex | `paint` | MX |
| Various self-storage | U-Haul, Public Storage, Shurgard, Big Yellow, etc. | `self_storage` | NA/EU |

### Not ingested (low OSM coverage; excluded from calibration)

| Chain | Category | Reason |
|---|---|---|
| 84 Lumber, Builders FirstSource, Kent Building Supplies | `lumber` | 0.9% co-occurrence with hardware anchors — below structural signal threshold |
| BOC UK | `welding` | 0.3% co-occurrence — 12 POIs only; insufficient signal |

---

## Open questions

1. **Lumber OSM coverage**: 84 Lumber and Builders FirstSource have sparse OSM coverage.
   The `lumber` category contributed only 0.9% co-occurrence with hardware anchors in
   calibration — below structural signal threshold. Excluded from current production build.
   Revisit if OSM coverage improves or if dedicated B2B lumber data source becomes available.
