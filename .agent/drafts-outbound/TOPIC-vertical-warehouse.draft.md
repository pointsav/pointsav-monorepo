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

## Current proxy data (test results — 2026-06-01)

The production VWH detection pipeline is not yet complete. The current test uses a proxy:
clusters where **hardware chains are present but no grocery hypermarket is present**,
located 5–80 km from a major metro centre.

**360 proxy candidates** identified globally across 17 display countries:

| Country | Candidates | Representative examples |
|---|---|---|
| US | 99 | Colorado Springs (Lowe's, no grocery, 7.5 km from metro centre) |
| DE | 77 | Munich suburbs (Hagebaumarkt + MediaMarkt clusters, 5–6 km from centre) |
| MX | 56 | Monterrey fringe (Home Depot + Sam's Club, no grocery) |
| FR | 44 | Rennes fringe (Leroy Merlin + Decathlon, no grocery) |
| IT | 28 | Turin fringe (Leroy Merlin + Decathlon) |
| NL | 28 | Eindhoven (Praxis + Gamma + IKEA retail park, no grocery) |
| CA | 13 | Hamilton ON (Home Depot, no grocery, 6.6 km from Hamilton) |

Metro distance distribution: peak at 10–19 km (96 sites), remainder 5–9 km (76) and 20–79 km (188).

---

## Data collection plan

### Already ingested (YAML scaffolded, OSM query pending)

| chain_id | Chain | Count |
|---|---|---|
| `autozone-us` | AutoZone | ~6,300 US |
| `oreilley-auto-us` | O'Reilly Auto Parts | ~6,100 US |
| `napa-us` | NAPA Auto Parts | ~6,000 US |
| `sherwin-williams-us` | Sherwin-Williams | ~4,900 US |
| `halfords-uk` | Halfords | ~400 UK |

### Priority additions (Tier A — definitive VWH, never in grocery parks)

| Chain | Wikidata | Count | Why it matters |
|---|---|---|---|
| Floor & Decor | Q22350998 | ~240 US | Warehouse-format contractor flooring; same footprint as Home Depot |
| Topps Tiles | Q7825827 | ~300 UK | Contractor tile retail in industrial estates |
| United Rentals | Q7889284 | ~1,400 NA | Deliberately co-locates next to hardware anchors |
| Sunbelt Rentals | Q7645154 | ~1,100 NA | Same strategy; never in grocery parks |
| Loxam | Q6692217 | ~1,100 EU | EU tool rental; industrial estates |
| Kiloutou | Q3197034 | ~600 FR | FR tool rental |
| **Würth** | Q183759 | ~1,500 EU | Biggest single EU gap — MRO in every EU industrial park |
| Fastenal | Q1394323 | ~3,400 NA | Industrial MRO; always industrial-zoned |
| Grainger | Q904633 | ~600 NA | Industrial MRO |
| Hilti | Q565285 | ~600 EU | Precision tools; Hilti Centers in industrial parks |
| 84 Lumber | Q4641204 | ~310 US | Lumber yards are definitionally industrial fringe |
| Builders FirstSource | Q4934620 | ~570 US | Lumber/building materials B2B |
| Kent Building Supplies | Q6383907 | ~45 CA | Canadian lumber/building supply |

### Tier B (plumbing, electrical, welding — industrial zones, lower OSM coverage)

| Chain | Wikidata | Market | Notes |
|---|---|---|---|
| Ferguson | Q5442877 | NA | Plumbing/HVAC supply; trade counters |
| Wolseley UK | Q832040 | UK | ~600 plumbing branches |
| Rexel | Q1758780 | EU | ~2,000 electrical branches |
| CEF (City Electrical Factors) | Q5012183 | UK | ~400 electrical trade counters |
| BOC UK | Q4844095 | UK | ~60 welding/gas branches |

### Taxonomy categories needed (non-tier-gating additions to taxonomy.py)

`flooring`, `tool_rental`, `mro_industrial`, `lumber`, `plumbing`, `electrical` — none of
these gate T1/T2/T3 tier logic. They appear as VWH enrichment signals in cluster member arrays.

---

## Open questions

1. **Würth branch format**: Würth operates both trade-counter branches (~200 sqm) and larger
   Würth Haus locations (~2,000 sqm). Both are industrial-park located. Does OSM distinguish
   between them via sub-type tags? Check before ingest.
2. **Lumber OSM coverage**: 84 Lumber and Builders FirstSource are both B2B trade counters
   with very sparse OSM coverage. Name-query fallback (`name_query: "84 Lumber"`) is needed.
   Confirm before ingesting that name-query returns meaningful results in US country bbox.
