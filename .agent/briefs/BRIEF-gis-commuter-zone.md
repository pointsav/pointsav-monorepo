---
artifact: brief
schema: foundry-brief-v1
brief-id: gis-commuter-zone
title: Commuter Zone — Zone Classification + Co-location Categories
status: active
owner: project-gis
created: 2026-06-30
updated: 2026-06-30
parent: gis-intercity-fringe
---

## Context

Companion to [[gis-intercity-fringe]]. Where Intercity Fringe captures trade/industrial
corridor zones, Commuter Zone captures transportation-gateway areas: places where people
transition between modes of movement. Two distinct sub-types identified from Calgary
reference areas:

1. **Transit Terminus** — last stop on a metro/LRT line, where commuters park, transfer
   to bus, or walk to adjacent residential. Reference: WB 69 Street CTrain Station,
   Calgary AB (last westbound stop; lat=51.0376, lon=-114.1885).

2. **Regional Aviation** — general aviation / charter airport serving a metro region,
   not a major hub. Different POI profile from transit terminus: aviation services,
   light industrial, FBO, hangar-adjacent commercial. Reference: Springbank Airport
   (CYBW), Calgary AB (lat=51.1028, lon=-114.3741).

The open question driving this BRIEF: are these one zone type ("Commuter") with
sub-types, or two fundamentally separate classifications?

## Scope

1. POI-profile both reference zones via Overpass (agent dispatched 2026-06-30).
2. Compare against Tuscany NW suburban baseline (prior session).
3. Determine: single "Commuter" type with sub-types, or two independent zone types.
4. Define co-location categories (target: 6–10 per type, or 8–12 if unified).
5. Establish detection threshold (minimum category count).
6. Validate against 2–3 additional reference zones before writing build script.

## Decisions locked

- No weighted ranking — simple co-location presence/absence or count gates.
- 69 Street CTrain (last stop) and Springbank Airport are canonical reference samples.
- Categories must be resolvable from OSM tags; new data sources not required for MVP.
- Sister classification to Intercity Fringe — same architectural pattern (OSM tag
  sweeps, not chain brand:wikidata YAML files).

## Decisions locked (from POI analysis 2026-06-30)

- **Split into two sub-types** — Transit Terminus and Aviation Fringe are
  fundamentally different. Shared trait: infrastructure-defined land use (not
  retail demand). Detection method, NAICS cluster, and site-selection use case
  all differ. Umbrella name: **"Threshold Zone"** (placeholder pending naming decision).
- Transit Terminus: `transit_anchor` + `park_and_ride` are the kernel anchors.
  Commercial density is BELOW suburban baseline — commuters pass through, don't linger.
- Aviation Fringe: `aerodrome_anchor` + `hangar_cluster` (≥5 hangars) define the zone.
  Near-zero standard retail; all commercial is aviation-trade.
- Both score zero on T1/T2/T3 retail taxonomy — complementary, not competing.

## Decisions open

- [ ] "Threshold Zone" umbrella name — or keep Transit Terminus / Aviation Fringe as
      peer classifications to Intercity Fringe?
- [ ] Map display: separate toggle / layer per sub-type, or one "Threshold Zone" layer
      with sub-type icons?
- [ ] Minimum hangar count for Aviation Fringe (proposed: ≥5 `aeroway=hangar` ways).
- [ ] `residential_anchor` category in Transit Terminus as a higher-order sub-class
      (walk-shed terminus vs pure park-and-ride endpoint) — worth the distinction?

## Co-location categories

### Sub-type 1: Transit Terminus

Detection anchor: `transit_anchor` + `park_and_ride`. POI density is LOW — use
presence/absence, not count thresholds.

| Category | OSM signal | NA examples | EU equivalents |
|---|---|---|---|
| `transit_anchor` | `railway=station`, `public_transport=station`, `amenity=bus_station` | CTrain 69 St, BART Fremont, TTC Kipling | Métro terminus, U-Bahn Endstation |
| `park_and_ride` | `amenity=parking` (adjacent to station), `park_ride=yes` | GO Transit surface lots, Sound Transit P&R | Park+Ride Anlage, P+R |
| `drive_through_food` | `amenity=fast_food` + `drive_through=yes`, `amenity=cafe` | Tim Hortons, McDonald's, Starbucks | McDrive, Costa Coffee drive-through |
| `fuel_convenience` | `amenity=fuel`, `shop=convenience` | Shell, Petro-Canada, Circle K | BP, Total, REWE To Go |
| `bank_branch` | `amenity=bank`, `amenity=atm` | TD, RBC, Scotiabank | Sparkasse, BNP Paribas |
| `residential_anchor` | `shop=supermarket`, `amenity=pharmacy` | Safeway, Shoppers Drug Mart | Lidl, Aldi, Apotheke |

**Detection threshold:** `transit_anchor` + `park_and_ride` + any 2 of `drive_through_food`,
`fuel_convenience`, `bank_branch` → Transit Terminus zone.
`residential_anchor` present = higher-order terminus (strong walk-shed); absent = pure
park-and-ride endpoint.

### Sub-type 2: Aviation Fringe

Detection anchor: `aerodrome_anchor` + `hangar_cluster` ≥5.

| Category | OSM signal | NA examples | EU equivalents |
|---|---|---|---|
| `aerodrome_anchor` | `aeroway=aerodrome`, `aeroway=runway` | CYBW Springbank, KFCM Flying Cloud MN | EDFE Egelsbach DE, LFPN Toussus FR |
| `hangar_cluster` | `aeroway=hangar` count ≥5 | Springbank ×55, Centennial CO | Cannes-Mandelieu, Dübendorf CH |
| `flight_ops` | `aeroway=terminal` (per-operator), aviation `office=company` | Calgary Flying Club, Genesis Helicopter | Luftfahrtschule, ULM Club |
| `aircraft_mro` | `shop=electronics` (avionics), craft=* (aircraft), MRO `office=company` | Foster Aircraft Maintenance, AOG Friday | Ruag Aviation, Marshall Aerospace |
| `airtanker_base` | Named tanker base `aeroway=aerodrome` | Springbank Air Tanker Base, BC Tanker Bases | Sécurité Civile base (FR) |
| `aviation_industrial` | `landuse=industrial` adjacent to `aeroway=*` | Fringe warehousing at CYBW | Gewerbegebiet Flughafen-Umfeld |

**Detection threshold:** `aerodrome_anchor` + `hangar_cluster` (≥5 hangars) → Aviation Fringe.
`airtanker_base` = specialty sub-type (wildfire/emergency), flag separately.

## Planning literature anchors

**Transit Terminus:**
- APA/CNU TOD research: terminus/gateway stations = park-and-ride dominant; retail viability
  LOW unless specifically designed as regional destination. Mid-line stations = pedestrian TOD.
- MRSC/FHWA: terminus TODs work for office/employment park, not retail — workers arrive by
  train, don't need parking. Confirmed by 69 St profile (thin retail, heavy parking).
- POI character: fast food + gas + bank on arterial approach; grocery if residential walk-shed.

**Aviation Fringe:**
- FAA classification: Springbank = Regional GA (NPIAS). Compatible uses per FAA AC 150/5070:
  aviation MRO, flight schools (NAICS 611512), helicopter charters (481211).
- "Airport Influence Area" (AIA) concept: buffer restricts height + noise-sensitive uses →
  remaining commercial = industrial, aviation-trade, low-density agricultural.
- NAICS clusters: 481211 (charter air), 488190 (FBO/hangar), 488111 (ATC), 336411 (MRO).
- EU: EASA GA airports same land compatibility → Ruag Aviation, Marshall Aerospace type clusters.

## Work log

- 2026-06-30 — BRIEF created. Agent queried Overpass for 69 St CTrain
  (778 elements; dominant signal: 131 public_transport platforms, 33 parking) and
  Springbank Airport (478 elements; dominant: 115 aeroway=taxiway, 55 aeroway=hangar,
  20 named aviation operators). Confirmed split into two sub-types. Categories and
  detection thresholds established.

## Carry-forward

- Validate Transit Terminus threshold against: Sheppard-Yonge TTC (Toronto),
  King George SkyTrain (Surrey BC), BART Fremont (CA).
- Validate Aviation Fringe against: YKF Waterloo Regional (ON), KFCM Flying Cloud
  (Minneapolis MN), LFPN Toussus-le-Noble (Paris FR).
- Naming decision: "Threshold Zone" umbrella vs standalone zone types.
- Build scripts: build-transit-terminus.py + build-aviation-fringe.py (OSM tag sweep
  pattern, same architecture as planned build-intercity-fringe.py).
