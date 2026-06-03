---
artifact: brief
schema: foundry-brief-v1
name: BRIEF-location-intelligence-archetypes-2026-06-01
language_protocol: CODE-RESEARCH
status: active
created: 2026-06-01
updated: 2026-06-03
author: totebox@project-gis
---

# Location Intelligence Archetypes — Research Brief

Three canonical co-location archetypes under the **Location Intelligence** product concept.
Three-letter codes ratified 2026-06-01.

---

## 1. The Three Archetypes

| Code | Name | Status | Anchor type |
|---|---|---|---|
| **PRO** | Retail Centres | Live — T1/T2/T3 rings + dedicated map mode | Grocery hypermarket + hardware ± price club / lifestyle / electronics |
| **VWH** | Urban Fringe | Live — dedicated map mode, **7,028 sites** (Retail-density, 2026-06-03) | Light-industrial-fringe trade-supply co-location OR any lone STRONG/BROAD trade store |
| **PKS** | Commuter | Live — dedicated map mode, **5,977 sites** (geometric airport-led, 2026-06-03) | Regional airport (park-and-fly) OR outer commuter-rail-belt station (drive-in park-and-ride) |

---

## 1b. Current production models (updated 2026-06-03)

The §3–§4 detail below records the original research framing. The **live build models** are:

**VWH — Urban Fringe (Retail-density, 7,028 features).** `qualify_vwh(cats)` in
`build-vwh-clusters.py` admits a cluster when it has **≥2 distinct trade-supply categories**
(a genuine co-location) **OR any single STRONG/BROAD trade-supply store** (lone hardware /
mro_industrial / tool_rental / electrical / plumbing / lumber / flooring / welding) — a single
trade store still marks the light-industrial fringe. Only lone WEAK stores (auto_parts-only,
paint-only) are dropped. Tier: `tier_vwh(cats, n)` composition score
`|cats| + 2·|cats∩STRONG| + (hardware?1) + min(n,8)` → T1 ≥10 / T2 ≥5 / T3 <5
(T1 747 / T2 2,732 / T3 3,549; the T3 mass is single-store fringe markers).

**PKS — Commuter (geometric park-and-ride, airport-led, 5,977 features).** Purely geometric,
no metadata tags. A candidate is **either** a sized regional airport (park-and-fly node, within
~600 km of a metro reference) **or** an outer commuter-rail-belt station (15–110 km ring,
connected toward the metro core, ≤4 stops from the line end). **Airports lead deliberately —
they are geographically distributed and fill the map where rail does not.** This fixed a spread
problem: rail-only coverage was 96 North-America map cells; airport-led is **957** (≈10×). Tier:
`tier_pks_geo(metro_d, inward, iso_km, outward, is_airport)` — regional airports in the 25–120 km
drive-in band → T1; rail scored by ring proximity (peak ~45 km) + line connectivity + catchment
isolation + terminus proximity (T1 1,317 / T2 3,183 / T3 1,477).

**Both target ≈ Retail density (~6,500 bubbles)** so all three archetype maps read at the same
fullness. A **simulation harness** (`tools/sim_spread.py`) clusters once and evaluates candidate
qualify/tier rules instantly — used to tune both models.

**Planned next (June 4 overnight ingest, scheduled in crontab):** four new VWH categories
(builders' merchants, self-storage, trade counters, parcel depots → genuine co-locations) and a
parking layer (`ingest-osm-parking.py`) that labels each PKS candidate **BUILT / PARTIAL /
GREENFIELD** — the "no parkade yet" filter that turns the spread map into a ranked
park-and-ride development-opportunity map.

---

## 2. PRO — Retail Centres (existing system)

The current T1/T2/T3 pipeline. Grocery-anchored retail co-location at three scales:
- **T1 Regional**: hypermarket ∧ hardware ∧ (price club ∨ lifestyle ∨ electronics) — or ≥4 anchor categories — or ≥3 anchors in tight (≤1km) cluster
- **T2 District**: hypermarket ∧ hardware, span ≤ 2.5km
- **T3 Local**: remaining anchor pairs

6,493 clusters live as of Phase 23+Change B rebuild (2026-05-28):
T1=1,746 / T2=2,726 / T3=2,021. 17 display countries (NA + EU).

---

## 3. VWH — Urban Fringe

### Definition

A 3–6 story multi-storey warehouse building for:
- Light manufacturing (electronics assembly, robotics, paint/coatings formulation)
- Just-in-time delivery and last-mile logistics
- Mixed industrial tenants in an urban or near-urban location

NOT a retail zone. NO grocery anchor. Trades/contractor and industrial supply orientation.

### Co-location signals (site selection)

**Essential:**
| Signal | Why | Available |
|---|---|---|
| Highway interchange ≤2km | Truck ingress/egress | Derivable from Overture road network |
| Industrial landuse neighbours | Zoning compatibility | OSM `landuse=industrial` — not yet ingested |
| Population 300k+ / 30-min drive | Manufacturing + logistics labour | ✅ Kontur (13 countries) |
| Freight rail ≤2km | JIT component delivery | OSM `railway=rail` + `usage=freight` — not ingested |

**Significant:**
| Signal | Why | Available |
|---|---|---|
| Air cargo airport ≤20km | Electronics/components, rapid replenishment | ✅ Overture airports (20,841 records) |
| FedEx/UPS/DHL hub ≤5km | Last-mile network node | ❌ Not in chain taxonomy |
| Transit corridor ≤500m | Workforce access | ❌ Railway stations not ingested |
| Power substation ≤2km | Heavy electrical load for robotics | OSM `power=substation` — not ingested |

**Disqualifying:** Dense residential immediately adjacent; flood plain; heritage zone; inside PRO cluster.

### Test proxy (current data)

Hardware chain(s) present in cluster members AND hypermarket absent — strong industrial-commercial
fringe signal. Metro distance 5–80km.

### Test results (2026-06-01 run)

Script: `app-orchestration-gis/test-cluster-archetypes.py`

| Country | Candidates | Notes |
|---|---|---|
| US | 99 | Lowe's, Home Depot alone or with Price Club/Electronics |
| DE | 77 | OBI, Hornbach, Hagebaumarkt + MediaMarkt/Saturn |
| MX | 56 | Home Depot + Price Club (Costco/Sam's) |
| FR | 44 | Castorama/Leroy Merlin + Decathlon/Electronics |
| IT | 28 | Leroy Merlin + Electronics/Sport |
| NL | 28 | Praxis/Gamma + IKEA (no grocery) |
| CA | 13 | |
| Other EU | 17 | PL, AT, ES, SE, DK, IS, FI, GR, PT |
| **Total** | **360** | |

Metro-distance distribution: 0–9km=76, 10–19km=96 (peak), 20–29km=61, 30–79km=127.

**Spot-checks:**
- Colorado Springs, CO: Lowe's 7.5km from metro centre, no grocery anchor — textbook VWH
- DE (OBI, Hornbach at 5–6km from Munich/Cologne/Frankfurt city centres): inner-suburban industrial fringe
- NL (Eindhoven: Praxis + Gamma + 3× IKEA, no grocery): retail park without food anchor

### Production data gaps

To make VWH production-grade, add:
1. Auto-parts chains: AutoZone (Q2241044), O'Reilly (Q1783118), NAPA (Q1349140) — NA; Halfords (Q3773366) — UK
2. Paint: Sherwin-Williams (Q380484) — NA; Dulux paint centres — EU
3. OSM `landuse=industrial` polygon layer via new `ingest-osm-industrial.py`
4. Freight rail: OSM `railway=rail` + `usage=freight` via new `ingest-osm-freight-rail.py`
5. Logistics hubs: FedEx (Q376941), UPS (Q193597), DHL (Q489815), Amazon (Q3884) distribution centres

---

## 4. PKS — Commuter

### Definition

A 3–9 story car parking structure at a **transit node** where commuters park and ride into
the metro core. Function: residents of a Regional Market (PRO cluster) park and
fly/train/metro to a Metro Market. Three transit sub-types:

| Sub-type | Metro distance | OSM class | Examples |
|---|---|---|---|
| Airport | 35–150 km | `airport` | Regional airports (not major hubs) |
| Intercity rail | 35–150 km | `intercity` | Amtrak, VIA Rail, DB, SNCF, Renfe |
| Suburban/commuter rail | 10–80 km | `suburban` | NJ Transit, LIRR, Metro-North, Metra, Transilien, S-Bahn |
| Metro/subway last stops | 5–35 km | `subway` / `light_rail` | End-of-line and outer-ring fringe stations |

"Last stops" on metro/subway lines are key PKS sites: residents drive to the terminus,
park, and ride the rapid-transit line into the core. Geographic fringe (5–35 km from
metro centroid) identifies these stations without needing route-relation parsing.

### Co-location signals (site selection)

**Essential:**
| Signal | Why | Available |
|---|---|---|
| Regional transit anchor ≤3km | Airport or intercity train station with direct metro service | ✅ Airports (Overture 20,841); ❌ Train stations not ingested |
| Metro isolation 15–150km | Defines "regional" relationship | ✅ Computable from NA/EU metro lists |
| T1 or T2 cluster ≤10km | Same population generates parking demand | ✅ Existing cluster data |
| Multi-lane arterial ≤1km | Traffic volume for parking inflow/outflow | Derivable |
| Regional population ≥150k | Minimum demand for structure viability | ✅ Kontur data |

**Significant:**
| Signal | Why | Available |
|---|---|---|
| Car rental ≤1km | Arriving travellers need transport | ❌ Enterprise/Hertz/Avis not ingested |
| Hotel cluster ≤500m | Business travel / multi-day parking | ❌ Hotel chains not ingested |
| Second transit mode ≤5km | Multi-modal integration = highest value | Partial (airports only) |
| No major hub ≤30km | Competing hub kills park-and-fly demand | ✅ Computable |

**Disqualifying:** Major hub within 15km; population under 100k; no direct metro service.

### Major hub filter (proxy, current data)

Airports with a T1 cluster within 5km are excluded as likely major hubs (major airports have T1
retail directly adjacent; regional airports typically do not).

### Test results (2026-06-01 run)

After 5km deduplication of Overture airport records (17,878 → 11,490 in display countries),
then filtering (15–150km metro distance; no T1 within 5km):

| Country | Candidates | Integrated (≤10km) | Integrated % |
|---|---|---|---|
| US | 3,678 | 1,071 | 29% |
| DE | 547 | 216 | 39% |
| CA | 421 | 133 | 32% |
| FR | 405 | 97 | 24% |
| GB | 338 | 129 | 38% |
| IT | 245 | 41 | 17% |
| MX | 214 | 28 | 13% |
| ES | 189 | 27 | 14% |
| PL | 143 | 24 | 17% |
| Other | 460 | 37 | |
| **Total** | **6,640** | **1,803** | **27%** |

"Integrated" = PKS candidate with a T1/T2 cluster within 10km — the Regional Market is already
co-located with the transit node. These are the highest-value Parking Structure sites.

**Spot-checks:**
- Krefeld, DE: 22.2km from Düsseldorf; T1/T2 cluster 5.9km away; score=1.74 — confirms our
  Rank 5 EU Regional Market is also a PKS site
- Colorado Springs (Pueblo airport): 66km from Colorado Springs metro; cluster 4.1km — valid
- Florida Gulf Coast (Sarasota, Venice, Tarpon Springs): all pass as PKS feeding Tampa metro
- Major hubs excluded correctly: LAX, JFK, LHR, CDG all filtered (T1 retail within 5km)

### Production data gaps

1. **Railway stations** (highest priority): OSM `railway=station` + `usage IN (main,branch)`
   — intercity stations across 17 display countries (~5,000–15,000 expected)
   — new script: `ingest-osm-railway.py`
2. **Car rental**: Enterprise (Q2283517), Hertz (Q379425), Avis (Q849144), Sixt (Q704156)
3. **Airport passenger volume**: CAPA, OAG, or IATA commercial data (Overture has no pax counts)
   — would replace the T1-adjacency proxy for hub/regional classification
4. **Multi-storey parking**: OSM `amenity=parking` + `parking=multi-storey` — confirms existing
   infrastructure; identifies where PKS has already been built

---

## 5. GeoJSON outputs (current test data)

| File | Features | Description |
|---|---|---|
| `work/archetype-vwh-candidates.geojson` | 360 | VWH proxy candidates; orange dots |
| `work/archetype-pks-candidates.geojson` | 6,640 | PKS candidates; teal=integrated, grey=standalone |
| `work/archetype-test-results.json` | — | Full results JSON with by-country breakdowns |

Deployed to map:
| File | Description |
|---|---|
| `www/data/archetype-vwh.geojson` | Copy of work file; lazy-loaded by `toggleVwhLayer()` |
| `www/data/archetype-pks.geojson` | Copy of work file; lazy-loaded by `togglePsLayer()` |

---

## 6. Pipeline scripts

| Script | Purpose | Status |
|---|---|---|
| `test-cluster-archetypes.py` | Produces VWH and PKS GeoJSON; per-class metro distance ranges | ✅ Updated 2026-06-02 — loads commuter JSONL; per-class ranges |
| `ingest-osm-airports.py` | IATA-filtered commercial airport ingest | ✅ Done — 4,024 airports |
| `ingest-osm-railway.py` | OSM intercity railway station ingest | ✅ Done — 18,107 intercity stations, 16 countries |
| `ingest-osm-railway-commuter.py` | Commuter/suburban rail + metro/subway station ingest | ✅ Written 2026-06-02 — **NOT YET RUN** |
| `ingest-osm-industrial.py` | OSM `landuse=industrial` polygon ingest | ❌ Planned |
| Chain YAMLs (auto-parts, paint, car rental) | VWH Tier A/B chains; PKS car rental | ✅ Done — 19,242 records / 32 chains |

**Current PKS GeoJSON (gateway):** 2,396 features (intercity rail + airports only; commuter/metro
not yet included). Re-run `test-cluster-archetypes.py` after commuter ingest to expand.

---

## 7. UI integration (updated 2026-06-02)

**All three modes are now first-class dedicated map modes.** Toggling Urban Fringe or Commuter
gives a 100% dedicated view — Retail cluster nodes are fully hidden, not ghosted.

### Mode architecture

`activeArchetype` (null | `'VWH'` | `'PKS'`) drives all layer dispatch in `applyView()`.

| Mode | `activeArchetype` | Overview (zoom < 9) | Drill-in (zoom ≥ 9) |
|---|---|---|---|
| Retail | `null` | Retail cluster bubbles (T1/T2/T3 colors) | Retail rings + individual store dots |
| Urban Fringe | `'VWH'` | VWH dots (T1/T2/T3 colors, no Retail nodes) | VWH rings (same proximity-* layers) |
| Commuter | `'PKS'` | PKS dots (T1/T2/T3 colors, no Retail nodes) | PKS rings (2 km fixed radius) |

All three modes are mutually exclusive — `deactivateOverlayGroup()` enforces this; switching
resets `activeArchetype` to null and restores Retail nodes before entering the new mode.

### Ring colors — all three modes

T1 = `#164679` (blue) / T2 = `#54924E` (green) / T3 = `#EAB308` (gold). Same across PRO, VWH, PKS.

### Ring radius

- PRO: `Math.max(1.0, span_km / 2 * 1.15)` km — matches cluster physical footprint
- VWH: same formula (VWH candidates carry `span_km` from their source cluster)
- PKS: fixed 2.0 km — transit nodes are single points with no cluster span

### Ring data sources

`buildProdRings()` dispatches to the right builder based on `activeArchetype`:
- `_buildRetailRings()` — reads `clusterMeta` (loaded from `data/clusters-meta.json`)
- `buildVwhRings()` — reads `vwhData` (fetched from `data/archetype-vwh.geojson`)
- `buildPksRings()` — reads `pksData` (fetched from `data/archetype-pks.geojson`)

### State variables (commit 438b37d6)

```javascript
let activeArchetype   = null;  // null | 'VWH' | 'PKS'
let vwhData = null;  // GeoJSON FeatureCollection, fetched at startup
let pksData = null;  // GeoJSON FeatureCollection, fetched at startup
let urbanFringeActive = false;
let commuterActive    = false;
```

### Layer control buttons

Toggle buttons sit in the layer control panel; same `lc-active` class pattern as Retail.
Labels: "Urban Fringe" / "Commuter". Switching deactivates the other mode automatically.

### Individual store dots at drill-in — Phase 1 scope

VWH and PKS drill-in shows rings but NOT individual chain dots (no VWH/PKS-specific PMTiles
built yet). `all-locations` (Retail store dots from `layer1-locations.pmtiles`) is suppressed
in archetype mode. Phase 2: build `layer-vwh-locations.pmtiles` and `layer-pks-locations.pmtiles`
via `build-tiles.py` extension.

---

## 8. Research addendum (2026-06-01) — Airport, Rail, VWH taxonomy, PKS commercial pattern

### Airport classification
- Current Overture data: 20,841 records, ALL tagged `category_id: airport`, NO type metadata
- `location_name` is always just "airport" — no IATA code, no aerodrome:type, no passenger count
- OSM filters for commercial airports: `aerodrome:type IN (public, international, regional, domestic)` OR `iata=*` tag present
- IATA tag = ~100% commercial; aerodrome:type coverage ~70-80% in EU/NA, <40% in sparse regions
- Exclude: `aerodrome:type IN (private, military, glider)`, `aeroway IN (heliport, airstrip)`
- Expected reduction: 20,841 → ~5,000–8,000 commercial-grade airports after IATA/type filter
- New script needed: `ingest-osm-airports.py` (pattern from ingest-osm-civic.py)

### Railway station classification
- OSM has NO `station=regional` or `station=intercity` — service type is on ROUTE RELATIONS, not station nodes
- Reliable EXCLUDE (on station node): `station IN (subway, light_rail, tram, monorail)`
- Reliable INCLUDE (via route relations): `service IN (long_distance, high_speed, regional)`
- Key national intercity operators to filter for:
  - NA: Amtrak (US), VIA Rail (CA); MX has no intercity passenger rail → airport-only
  - EU: SNCF (FR), DB (DE), Renfe (ES), Trenitalia (IT), ÖBB (AT), NS (NL), SJ (SE), DSB (DK), Vy (NO), VR Group (FI), CP (PT), PKP Intercity (PL)
  - IS: No passenger rail → airport-only
- The commuter exclusion problem: stations like Reading (UK) or Tarrytown (NY) serve both intercity
  and commuter. Accept mixed-service stations — they are valid PKS sites regardless.
- **Architecture decision**: railway stations → `service-places/cleansed-civic-railway.jsonl`
  (same tier as airports and hospitals, not service-business or service-parking)

### VWH taxonomy additions (full priority table)

Tier A — definitive VWH signals (never in grocery retail parks):
| Category | Chain | Wikidata | Count | OSM |
|---|---|---|---|---|
| Flooring/tile | Floor & Decor | Q22350998 | ~240 US | Moderate |
| Flooring/tile | Topps Tiles | Q7825827 | ~300 UK | Moderate |
| Tool rental | United Rentals | Q7889284 | ~1,400 NA | Moderate |
| Tool rental | Sunbelt Rentals | Q7645154 | ~1,100 NA | Moderate |
| Tool rental | Loxam | Q6692217 | ~1,100 EU | Sparse |
| Tool rental | Kiloutou | Q3197034 | ~600 FR | Moderate |
| Industrial MRO | **Würth** | Q183759 | ~1,500 EU | **Moderate** — highest-value EU gap |
| Industrial MRO | Fastenal | Q1394323 | ~3,400 NA | Sparse |
| Industrial MRO | Grainger | Q904633 | ~600 NA | Sparse |
| Industrial MRO | Hilti | Q565285 | ~600 EU | Moderate |
| Lumber (NA) | 84 Lumber | Q4641204 | ~310 US | Very sparse |
| Lumber (NA) | Builders FirstSource | Q4934620 | ~570 US | Very sparse |
| Lumber (CA) | Kent Building Supplies | Q6383907 | ~45 CA | Moderate |

Tier B — probable VWH (mixed suburban + industrial):
| Category | Chain | Wikidata | Notes |
|---|---|---|---|
| Plumbing/HVAC | Ferguson | Q5442877 | Trade counter only; sparse OSM |
| Plumbing/HVAC | Wolseley UK | Q832040 | ~600 UK branches; moderate OSM |
| Electrical | Rexel | Q1758780 | ~2,000 EU; most tractable electrical EU |
| Electrical | CEF UK | Q5012183 | ~400 UK industrial estates; moderate OSM |
| Welding/gas | BOC UK | Q4844095 | ~60 UK; moderate OSM |

Excluded — NOT VWH signals:
- Auto services (Jiffy Lube, Midas, Euromaster) — suburban arterial, not industrial
- Self-storage — residential fringe coincidence, not functional VWH signal
- Consumer glass/glazing (Safelite/Carglass) — suburban strip
- Printing/signage (Fastsigns) — too mixed (60% suburban)

### PKS commercial pattern (from Overpass queries on 4 test sites)

| Signal | Toluca MX | Delicias MX | Largo FL |
|---|---|---|---|
| Car rental | **4** (Hertz + locals) | 1 | 0 |
| Auto parts | **7** (Valeo OEM!) | **9** (AutoZone) | 1 (Advance Auto) |
| Fuel stations | 0 | **6** | 1 |
| Fast food | 1 | **24** | **8** |
| Convenience | 5 | **33** (OXXO×3) | 1 |
| Car wash | 0 | **7** | 1 |
| Multi-storey parking | 0 (unmapped in OSM) | 0 | 0 |
| Hotel | 0 (unmapped in MX) | 0 | 0 |

Key findings:
- Car rental is the DEFINING PKS commercial signal — appears at airport transit nodes
- Auto parts at Toluca = Valeo industrial OEM (manufacturing-belt airport, VWH/PKS overlap zone)
- Fuel + convenience + QSR = consistent secondary cluster
- Multi-storey parking universally absent from OSM — building type is unmapped
- Hotels absent from OSM for MX (exist in reality; data gap)

PKS car rental taxonomy (new `car_rental` category, non-tier-gating):
- Enterprise (Q2283517), Hertz (Q379425), Avis (Q849144), Sixt (Q704156), Europcar (Q466704)

### service-parking architecture decision
- Already planned in cluster-totebox-personnel-1 MANIFEST as "moonshot tier"
- OSM `amenity=parking` + `parking=multi-storey` reliable for CONFIRMATION, not DISCOVERY
- Parking operators: Q-Park (Q1127798), APCOA (Q296108), NCP (Q6971273), Indigo/Vinci (Q3559970), SP+ (Q7598289)
- Phase 1: OSM multi-storey structures near confirmed transit nodes
- Phase 2: Operator chain directory + API integration (Q-Park, APCOA have APIs)
- NOT blocking current VWH/PKS test — deferred to future session

---

## 9. Full data collection inventory (2026-06-01)

### VWH — chains already scaffolded (YAML exists, ingest not run)

| chain_id | Chain | Wikidata | Count | Category |
|---|---|---|---|---|
| `autozone-us` | AutoZone | Q2241044 | ~6,300 US | auto_parts |
| `oreilley-auto-us` | O'Reilly Auto Parts | Q1783118 | ~6,100 US | auto_parts |
| `napa-us` | NAPA Auto Parts | Q1349140 | ~6,000 US | auto_parts |
| `sherwin-williams-us` | Sherwin-Williams | Q380484 | ~4,900 US | paint |
| `halfords-uk` | Halfords | Q3773366 | ~400 UK | auto_parts |

Run: `python3 ingest-osm.py --chain autozone-us oreilley-auto-us napa-us sherwin-williams-us halfords-uk`

### VWH — Tier A chains still to add (YAML + taxonomy needed)

New taxonomy categories needed: `flooring`, `tool_rental`, `mro_industrial`, `lumber` (all non-tier-gating).

| chain_id | Chain | Wikidata | Count | Category | OSM |
|---|---|---|---|---|---|
| `floor-decor-us` | Floor & Decor | Q22350998 | ~240 US | flooring | Moderate |
| `topps-tiles-uk` | Topps Tiles | Q7825827 | ~300 UK | flooring | Moderate |
| `united-rentals-us` | United Rentals | Q7889284 | ~1,400 NA | tool_rental | Moderate |
| `sunbelt-rentals-us` | Sunbelt Rentals | Q7645154 | ~1,100 NA | tool_rental | Moderate |
| `loxam-fr` | Loxam | Q6692217 | ~1,100 EU | tool_rental | Sparse |
| `kiloutou-fr` | Kiloutou | Q3197034 | ~600 FR | tool_rental | Moderate |
| `wurth-de` | **Würth** | Q183759 | ~1,500 EU | mro_industrial | **Moderate** |
| `fastenal-us` | Fastenal | Q1394323 | ~3,400 NA | mro_industrial | Sparse |
| `grainger-us` | Grainger | Q904633 | ~600 NA | mro_industrial | Sparse |
| `hilti-ch` | Hilti | Q565285 | ~600 EU | mro_industrial | Moderate |
| `84-lumber-us` | 84 Lumber | Q4641204 | ~310 US | lumber | Very sparse |
| `builders-firstsource-us` | Builders FirstSource | Q4934620 | ~570 US | lumber | Very sparse |
| `kent-building-supplies-ca` | Kent Building Supplies | Q6383907 | ~45 CA | lumber | Moderate |

### VWH — Tier B chains (plumbing/electrical/welding)

| chain_id | Chain | Wikidata | Count | Category |
|---|---|---|---|---|
| `ferguson-us` | Ferguson | Q5442877 | ~1,700 NA | plumbing |
| `wolseley-uk` | Wolseley UK | Q832040 | ~600 UK | plumbing |
| `rexel-fr` | Rexel | Q1758780 | ~2,000 EU | electrical |
| `cef-uk` | CEF (City Electrical Factors) | Q5012183 | ~400 UK | electrical |
| `boc-uk` | BOC UK | Q4844095 | ~60 UK | welding |

### PKS — transit infrastructure (service-places, not service-business)

| Data | Script | Output | Status |
|---|---|---|---|
| Commercial airports (IATA-filtered) | `ingest-osm-airports.py` | `service-places/cleansed-civic-airports.jsonl` | ❌ Planned |
| Intercity rail stations | `ingest-osm-railway.py` | `service-places/cleansed-civic-railway.jsonl` | ❌ Planned |

Airport filter: `aerodrome:type IN (public, international, regional, domestic)` OR `iata=*` present.
Expected: 20,841 Overture records → ~5,000–8,000 commercial airports.

Rail station filter: `railway=station` AND `station NOT IN (subway, light_rail, tram, monorail)`.
Intercity operators by country: Amtrak (US), VIA Rail (CA), SNCF (FR), DB (DE), Renfe (ES),
Trenitalia (IT), ÖBB (AT), NS (NL), SJ (SE), DSB (DK), Vy (NO), VR Group (FI), CP (PT), PKP Intercity (PL).
MX and IS: airport-only (no intercity passenger rail).

### PKS — car rental chains (new `car_rental` category, non-tier-gating)

| chain_id | Chain | Wikidata | Count | Notes |
|---|---|---|---|---|
| `enterprise-us` | Enterprise | Q2283517 | ~8,500 NA | Defining PKS signal |
| `hertz-us` | Hertz | Q379425 | ~3,500 NA | Found at Toluca airport in Overpass |
| `avis-us` | Avis | Q849144 | ~2,500 NA | |
| `sixt-de` | Sixt | Q704156 | ~700 EU | Strong EU airport presence |
| `europcar-fr` | Europcar | Q466704 | ~2,000 EU | Major EU airport car rental |

### PKS — parking operators (service-parking, future moonshot)

| Operator | Wikidata | Countries | Status |
|---|---|---|---|
| Q-Park | Q1127798 | NL/DE/BE/FR/UK/IE/DK | Phase 2 — API available |
| APCOA | Q296108 | 13 EU countries | Phase 2 — API available |
| NCP (National Car Parks) | Q6971273 | UK | Phase 2 |
| Indigo/Vinci Park | Q3559970 | FR/EU | Phase 2 |
| SP+ | Q7598289 | US | Phase 2 |

### Ingestion sequencing (remaining work)

1. **Run `ingest-osm-railway-commuter.py --all`** — after 22:00 Vancouver. Commuter/suburban/metro.
   Output: `service-places/cleansed-civic-railway-commuter.jsonl`
2. **Re-run `test-cluster-archetypes.py`** — picks up commuter JSONL automatically; updates
   `work/archetype-pks.geojson` and `work/archetype-vwh.geojson`
3. **Copy `archetype-pks.geojson` to gateway** —
   `deployments/gateway-orchestration-gis-1/www/data/archetype-pks.geojson`
4. **Stage 6** — promote commit `438b37d6` to canonical monorepo (Command Session)

**Items already completed (2026-06-01 and earlier):**
- YAML chain ingests (auto-parts, paint, car rental, tool rental, MRO, flooring, lumber)
- `ingest-osm-airports.py` (4,024 IATA airports)
- `ingest-osm-railway.py` (18,107 intercity stations, 16 countries)
- VWH rings + PKS rings live on gis.woodfinegroup.com (commit `438b37d6`)

---

## 10. Session state — 2026-06-02 (updated)

**Committed this session:**
- `2e70000e` — feat(map-ui): VWH + PKS dots fade through z9-z12 (zoom continuity)
- `0fab2667` — feat(vwh): coord-based ISO fix + Nordic enrichment chains

**What shipped (this session):**
- Map UI: VWH/PKS archetype dots now visible at LOCAL zoom (z9–z12) via paint opacity
  fade expression. Previously disappeared abruptly at z9.
- ISO fix: `ingest-osm.py` now uses Shapely point-in-polygon for multi-country chains
  when OSM addr:country tag absent. Würth DE-tagged Finnish/Italian/Polish records
  now correctly attributed. Rewrote wurth-de, rexel-fr, loxam-fr, kiloutou-fr.
- New Nordic enrichment chains: cramo-fi (53 records FI/SE) + ahlsell-se (48 records SE/FI)
  + ramirent-fi/hss-hire-uk/speedy-hire-uk wired into _VWH_INDUSTRIAL_CHAINS.
- VWH re-run: 2,590 total (T1=471 T2=1933 T3=186). Coverage improvements:
  IT 35→91, FI 3→81, NL 39→69, PL 6→50, AT 16→48, SE 4→33, PT 3→28, ES 2→18.
  DE 771→459 (correct — were misattributed Würth branches outside Germany).
- GeoJSON synced: archetype-vwh.geojson + archetype-pks.geojson → gateway www/data/.

**Previously shipped (6e84a3a4):**
- VWH Pass 2: enrichment-chain-led detection. 440 → 2,587 → 2,590 after ISO fix.

**Previously shipped (438b37d6):**
- VWH and PKS dedicated ring modes. `ingest-osm-railway-commuter.py` written (not run).

**Outstanding (see NEXT.md):**
- Commuter ingest run (overnight, after 22:00 Vancouver) — script at `/tmp/aec-commuter-ingest.sh`
  or run manually: `python3 ingest-osm-railway-commuter.py --all`
- Re-run archetypes + sync PKS GeoJSON after commuter ingest completes
- Stage 6 promotion (Command Session)

**PKS GeoJSON on gateway:** 2,396 features (intercity+airport only, no commuter/metro yet).
PKS rings are live on the map but show intercity stations only until the ingest runs.
