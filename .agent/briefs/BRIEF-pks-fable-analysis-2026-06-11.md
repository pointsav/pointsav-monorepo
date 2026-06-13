---
schema: foundry-brief-v1
artifact: brief
title: PKS Commuter Archetype — Fable Model Analysis
slug: BRIEF-pks-fable-analysis-2026-06-11
status: active
created: 2026-06-11
author: totebox@project-gis (claude-sonnet-4-6 + fable consultation)
---

# PKS Commuter Archetype — Fable Model Analysis

**Session:** 2026-06-11 | **Model consulted:** claude-fable-5

## §1 Problem statement

PKS (Commuter/park-and-transit) archetype had T1=8%/T2=3%/T3=89% tier distribution
on 16,400 features. The semantic goal: identify locations where people drive and leave
their car to board transit (airports, rail stations, bus terminals). The tier breakdown
was useless for map visualization.

## §2 Root causes identified

1. **57% fake bimodal**: ICR (intercity_rail) + CR (commuter_rail) at the same station
   counted as two transit modes. One physical platform with two service levels.
2. **park_ride=0 for US/CA/DE/FR**: ingest-osm-parking.py supports these countries via
   TILE_GRIDS but was never run for them. AT has 1,713 records; US has 0.
3. **No qualification gate**: pure single-mode transit nodes with no parking/rental
   evidence were included — 84.2% of features had zero enrichment signal.
4. **self_storage in PKS pool**: suburban logistics signal, not drive-to-transit evidence.
5. **EU car rental thin**: only FR (europcar-fr.yaml, 1,022 records) and DE (sixt-de.yaml,
   233 records) — no UK, ES, IT, NL, AT, PL.

## §3 Conceptual framework (Fable)

"Drive here, leave car, board transit" has two directional signals:
- **Departure-side** (resident drives in): parking capacity adjacent to platform —
  P+R lots, multi-storey parkades, parking operators
- **Arrival-side** (traveller arrives carless): car rental, limited-service hotels

Current missing signals: parking structures (built/surface adjacency), hotel chains,
EU parking operators (Q-Park, APCOA, NCP, Indigo/Vinci).

## §4 Mode-group collapse

Collapse transit categories into 4 groups (not 5 individual modes):
- AIR: airport
- RAIL: intercity_rail + commuter_rail (same physical station)
- URBAN: metro_subway
- BUS: intercity_bus

Removes fake bimodal from ICR+CR co-location.

## §5 New tier scheme (implemented 2026-06-11)

**Qualification gate**: airport OR ≥2 mode groups OR (any mode group + any enrichment)
→ 11,652 walk-up stops disqualified; 4,934 qualified features.

**Tier logic**:
- T1: (AIR + RENTAL), OR ≥3 groups, OR (≥2 groups + ≥2 enrich), OR (AIR + ≥1 enrich)
- T2: AIR alone, OR (≥2 groups + ≥1 enrich), OR (1 group + ≥2 enrich)
- T3: ≥1 group + ≥1 enrich (not qualifying above)

**Result on current data**: T1=326 (6.6%) / T2=2,219 (45.0%) / T3=2,389 (48.4%)
  Total: 4,934 features — comparable to PRO retail (6,493)

## §6 Projected distribution after overnight ingest

After `ingest-osm-parking.py --countries US CA DE FR IT PL NO IS`:
- Expected +15,000-30,000 park_ride records
- ~2,500-4,000 new T3 features (rail/bus nodes gaining first enrichment signal)
- ~200-400 airports gaining PARK → upgrade T2→T1
- Projected: ~6,500-7,000 total, T1~12-15%, T2~38-42%, T3~44-48%

After EU car rental (generic OSM ingest):
- ~200-400 more EU airports gain RENTAL → T1
- T1 climbs toward ~18-22%

## §7 Data gaps to fill (Fable recommendations)

### Immediate (highest leverage):
1. Re-run `ingest-osm-parking.py` for US, CA, DE, FR, IT, PL, NO, IS
2. Generic `car-rental-osm.yaml` (amenity=car_rental, all 18 countries) — EU fix
3. Expand `sixt-de.yaml` to EU bbox (sixt-eu.yaml)

### Car rental brands to add:
| YAML | Wikidata | Notes |
|---|---|---|
| sixt-eu.yaml | Q705664 | Expand from DE-only to EU bbox |
| budget-us.yaml | Q1004913 | NA airport staple |
| national-us.yaml | verify | Enterprise Holdings airport brand |
| alamo-us.yaml | verify | Enterprise Holdings airport brand |
| avis-eu.yaml | verify | EU footprint of existing NA brand |
| hertz-eu.yaml | verify | EU footprint of existing NA brand |

### Hotel brands (new `hotel` enrichment category):
| YAML | Wikidata | Notes |
|---|---|---|
| ibis-eu.yaml | Q920166 | Incl. Ibis Budget; EU-wide |
| b-and-b-hotels-eu.yaml | Q794939 | Limited-service, airport/ring-road sites |
| premier-inn-gb.yaml | Q2108626 | UK; station districts |
| travelodge-gb.yaml | Q9361374 | UK; station districts |
| motel-one-de.yaml | Q866334 | DE; station districts |
| holiday-inn-express-us.yaml | Q5880423 | NA; airport/highway |
| hampton-us.yaml | Q5646184 | NA; airport/highway |
| courtyard-us.yaml | Q1053170 | NA; airport/highway |

### Parking operators (post-cluster spatial join, NOT in DBSCAN input):
Q-Park (Q2118010), APCOA (Q296108), Indigo/Vinci FR, Interparking EU

### Code change when hotel data arrives:
Add `hotel` to `_enrich_classes()` in build-pks-clusters.py.
Update tier logic: `(AIR + RENTAL_or_HOTEL) → T1`.

## §8 NA/EU asymmetry note (Fable)

Don't write region rules. Make inputs symmetric:
- Airport T1: requires RENTAL (NA has good rental data → airport T1 working)
  Once EU car rental added, EU airports will also qualify T1
- Rail T2: requires PARK+RENTAL (two enrichment classes)
  After park_ride ingest, many EU rail hubs will reach T2
- `use enrichment breadth (n_e), never raw counts` — robust to per-country OSM mapping density

## §9 Known limitation

GTFS service frequency would distinguish regional mainline station from local stopping service
but is a new data source class (Year 2). Current proxy (ICR/CR presence) is correct direction.

## §10 Code changes applied this session

File: `pointsav-monorepo/app-orchestration-gis/build-pks-clusters.py`
(in gitignored subdirectory — unversioned; see NEXT.md for version control debt item)

Changes:
- Added `_mode_groups()` and `_enrich_classes()` helpers
- Updated `qualify_pks()` with new gate
- Updated `tier_pks()` with mode-group logic
- Removed SELF_STORAGE_CHAINS entirely
- Updated `multi_modal` property to use mode-group count (not raw transit count)
- Added n_skipped_no_enrich counter to output stats
