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
research_done_count: 5
research_suggested_count: 4
open_questions_count: 3
research_provenance: |
  OSM wiki (aerodrome tagging, railway station tagging), Overpass API spot checks at 4 PKS
  test sites (Toluca MX, Delicias MX, Largo FL, Haines City FL), multi-agent research on
  parking operators and railway station classification, existing Overture airport data
  analysis (20,841 records), GIS test-cluster-archetypes.py results.
research_inline: |
  Car rental as defining PKS signal derived from Overpass query results: 4 car rental
  operations found at Toluca MX airport zone; AutoZone + Valeo OEM at Delicias confirms
  VWH/PKS spatial overlap in manufacturing-belt regions. Railway station operator list
  compiled from national rail operator OSM wiki pages. Airport filter strategy from
  aerodrome tagging documentation. Expected airport reduction (20,841 → 5,000–8,000) from
  aerodrome:type coverage statistics (~70-80% in EU/NA).
paired_with: TOPIC-parking-structures.es.draft.md
---

# Parking Structures (PKS)

A **Parking Structure** is a 3–9 story multi-level car park located at a regional airport
or intercity train station. Its function is to serve residents of a Regional Market who
drive to the transit node, park their car, and travel onward by plane or train to a
Metro Market.

The defining relationship: a **Regional Market feeds a Metro Market** either by plane or
by train. The parking structure is the physical infrastructure that makes this journey
possible at scale.

Three-letter code: **PKS**. One of three Location Intelligence archetypes alongside
Professional Centres (PRO) and Vertical Warehouse (VWH).

---

## The regional-to-metro relationship

A PKS site sits at the hinge between two market types:

- **Regional Market** — the city or suburb where the parking structure's users live and
  shop (captured by the existing T1/T2/T3 PRO cluster system)
- **Metro Market** — the major city that those users travel TO by plane or train

The PKS parking structure exists because the journey between them is long enough that
driving to the transit node and parking beats driving all the way to the metro. This
threshold is approximately 15–150 km:

| Distance to major metro | PKS viability |
|---|---|
| ≤15 km | Too close — suburb; driving to the metro is faster than parking and transiting |
| 15–100 km | Prime PKS zone — 1–2 hours driving; transit saves meaningful time |
| 100–150 km | Viable if transit is fast (high-speed rail, direct flight) |
| >150 km | Standalone market; may have its own metro relationship or be too remote |

---

## Transit anchor types

### Regional airports

A regional airport is distinguished from a major hub by:
- Serving primarily domestic routes or short-haul international destinations
- Passenger volume typically 500,000 – 5,000,000 annually
- Located 15–150 km from a major metro centre
- No T1 Professional Centres cluster immediately adjacent (major hubs have T1 retail
  within 5 km; regional airports typically do not)

OSM identification: `aeroway=aerodrome` + `aerodrome:type IN (public, regional, domestic)` OR
`iata=*` tag present. Exclude: `aerodrome:type IN (private, military, glider)`,
`aeroway=heliport`, `aeroway=airstrip`.

Current Overture data has 20,841 airport records with no type metadata. After IATA/type
filtering via `ingest-osm-airports.py`, approximately 5,000–8,000 commercial airports
are expected across 17 display countries.

**Country exceptions:**
- Mexico: No national intercity passenger rail — PKS is airport-only
- Iceland: No passenger rail — airport-only

### Intercity train stations

An intercity train station serves trains that travel 30–150 km to a major metro centre.
This is distinct from:
- Metro/subway stations (urban underground; typically within city limits)
- Commuter rail platforms (short-distance suburban service)
- Tram and light rail stops (street-running; no parking demand)

OSM identification: `railway=station` AND `station NOT IN (subway, light_rail, tram, monorail)`.
Stations are further filtered by membership in route relations with `service IN (long_distance,
high_speed, regional)` operated by national intercity rail operators.

**National intercity operators by country:**

| Country | Operator | Notes |
|---|---|---|
| US | Amtrak | Only intercity passenger rail in NA |
| CA | VIA Rail Canada | |
| FR | SNCF | TGV, Intercités, TER regional |
| DE | Deutsche Bahn (DB) | ICE, IC, EC, RE, RB |
| ES | Renfe | AVE, Alvia, Regional |
| IT | Trenitalia, Italo | Frecciarossa, Intercity |
| AT | ÖBB | Railjet, Intercity |
| NL | NS | All intercity service |
| SE | SJ | Long-distance; regional operators |
| DK | DSB | |
| NO | Vy (formerly NSB) | |
| FI | VR Group | |
| PT | CP (Comboios de Portugal) | |
| PL | PKP Intercity, RegioJet | |

Mixed-service stations (serving both intercity and commuter trains, e.g. Reading UK or
Tarrytown NY) are retained — they generate equivalent parking demand regardless of the
mixed service pattern.

---

## Co-location signals for site selection

**Essential:**

| Signal | Threshold | Rationale |
|---|---|---|
| Regional transit anchor | ≤3 km | Airport or intercity station with direct metro service |
| Metro isolation | 15–150 km | Defines the regional relationship |
| T1 or T2 PRO cluster | ≤10 km | The Regional Market whose population generates parking demand |
| Multi-lane road access | ≤1 km | Parking structure inflow/outflow requires arterial capacity |
| Regional population | ≥150,000 | Minimum demand for structure viability |

**Significant:**

| Signal | Threshold | Rationale |
|---|---|---|
| Car rental within 1 km | — | Arriving travellers need transport; highest-confidence PKS commercial signal |
| Hotel cluster | ≤500 m | Business travel; multi-day parking demand |
| Second transit mode | ≤5 km | Airport + train station = multi-modal integration; highest value sites |
| No major hub | ≥30 km | Competing major airport kills park-and-fly demand to regional |

**Disqualifying:**
- Major international airport within 15 km (travellers bypass regional for the hub)
- Population under 100,000 (insufficient demand)
- No direct service to a major metro (the transit relationship collapses)

---

## Commercial co-location pattern

From Overpass API queries against 4 confirmed PKS test sites (Toluca MX, Delicias MX,
Largo FL, Haines City FL):

| Commercial use | Signal strength | Notes |
|---|---|---|
| Car rental | **Defining** | Found at every well-mapped airport zone; Hertz at Toluca |
| Auto parts | Strong | AutoZone at Delicias; Valeo OEM at Toluca (manufacturing-belt overlap with VWH) |
| Fuel / petrol | Strong | Pre-departure fill-up; 6 stations at Delicias |
| Convenience retail | Strong | OXXO × 3 at Delicias; perimeter concessions at Toluca |
| Quick-service food | Moderate | 24 at Delicias, 8 at Largo FL |
| Car wash | Moderate | 7 at Delicias |
| Multi-storey parking | Not in OSM | Building type is unmapped universally — exists in reality |
| Hotels | Not in OSM for MX | Exist in reality; data gap in OSM coverage |

---

## Production data (2026-06-11)

The PKS detection pipeline is production-grade. Parking structures (114,835 built + 25,366
park-and-ride = 140,201 total) were profiled as proxy anchors; DBSCAN was run on transit
and commercial enrichment categories; tier rules were calibrated using mode-group collapse
logic (collapsing intercity_rail + commuter_rail into the RAIL group to prevent artificial
bimodality inflation).

**6,953 clusters** across 17 display countries:

| Tier | Count | % | Definition |
|---|---|---|---|
| T1 (Regional Hub) | 691 | 9.9% | Multi-modal + full commercial ecosystem (rental + hotel) |
| T2 (Transit Interchange) | 2,658 | 38.2% | Transit anchor + at least one commercial enrichment signal |
| T3 (Transit Node) | 3,604 | 51.9% | Transit present; commercial development opportunity |

Park-and-ride records (23,117) serve as the discrete geographic anchor — actual car→transit
transition points, not interpolated from rail network geometry.

**Commercial enrichment signals:**

| Signal | Source | Coverage |
|---|---|---|
| Car rental | hertz-eu (687), avis-eu (741), europcar-eu (1,021), sixt-eu (246), budget-eu (130), enterprise-us, hertz-us + others | All 17 countries |
| Hotel | ibis-eu (708), premier-inn-gb (817), travelodge-gb (580), holiday-inn-express-us (2,021), courtyard-us (1,020), hampton-us (240), b-and-b-hotels-eu (797), motel-one-de (24) | All 17 countries |

**Note on airport filter:** `ingest-osm-airports.py` applies the IATA/`aerodrome:type` filter
to commercial airports; railway station data comes from `ingest-osm-railway.py` (intercity)
and `ingest-osm-railway-commuter.py` (commuter/metro). Both scripts are in production.

---

## Chain taxonomy (production — 2026-06-11)

### Transit infrastructure (service-places, ingested)

| Script | Output | Records |
|---|---|---|
| `ingest-osm-airports.py` | `cleansed-civic-airports.jsonl` | Commercial airports; IATA/aerodrome:type filter applied |
| `ingest-osm-railway.py` | `cleansed-civic-railway.jsonl` | Intercity rail stations |
| `ingest-osm-railway-commuter.py` | `cleansed-civic-railway-commuter.jsonl` | Commuter rail + metro/subway |
| `ingest-osm-bus-terminal.py` | `cleansed-civic-bus-terminal.jsonl` | Intercity bus terminals |
| Park-and-ride | `cleansed-civic-parking.jsonl` (park_ride filter) | 23,117 records — primary geographic anchor |

### Car rental chains (ingested)

| chain_id | Chain | Records |
|---|---|---|
| `hertz-eu` | Hertz EU | 687 |
| `avis-eu` | Avis EU | 741 |
| `budget-eu` | Budget EU | 130 |
| `europcar-eu` | Europcar EU | 1,021 |
| `sixt-eu` | Sixt EU | 246 |
| `budget-us` | Budget US | 278 |
| `alamo-us` | Alamo US | 110 |
| `enterprise-us`, `hertz-us`, `avis-us` | Enterprise / Hertz / Avis US | Existing |

### Hotel chains (ingested)

| chain_id | Chain | Records |
|---|---|---|
| `ibis-eu` | Ibis EU | 708 |
| `b-and-b-hotels-eu` | B&B Hotels EU | 797 |
| `premier-inn-gb` | Premier Inn UK | 817 |
| `travelodge-gb` | Travelodge UK | 580 |
| `motel-one-de` | Motel One DE | 24 |
| `holiday-inn-express-us` | Holiday Inn Express US | 2,021 |
| `hampton-us` | Hampton Inn US | 240 |
| `courtyard-us` | Courtyard US | 1,020 |

### Parking operators (service-parking — future moonshot)

| Operator | Wikidata | Countries |
|---|---|---|
| Q-Park | Q1127798 | NL/DE/BE/FR/UK/IE/DK |
| APCOA Parking | Q296108 | 13 EU countries |
| NCP (National Car Parks) | Q6971273 | UK |
| Indigo (Vinci Park) | Q3559970 | FR/EU |
| SP+ | Q7598289 | US |

---

## Open questions

1. **Airport filter threshold**: Is 5 km the right T1-adjacency radius for hub exclusion?
   LHR has T1 retail at Heathrow Village (~4 km) — does this correctly exclude it? Spot-check
   against 3–5 known major hubs and 3–5 known regional airports before finalising.

2. **Rail station commuter boundary**: Stations like Reading (UK, 64 km from London) and
   Princeton Junction (NJ, 75 km from NYC) serve heavy long-distance commuters, not just
   intercity travellers. Do these generate parking demand consistent with PKS? Or does the
   commuter pattern suppress parking (season-ticket holders often walk/cycle)?

3. **Multi-modal premium**: Is there a quantifiable premium on PKS sites that have BOTH
   an airport AND an intercity station within 5 km? How many such sites exist in the
   current dataset? (Both transit types within 5 km would be the highest-confidence
   Parking Structure development signal.)
