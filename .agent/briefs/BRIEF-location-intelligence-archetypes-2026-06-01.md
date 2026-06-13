---
artifact: brief
schema: foundry-brief-v1
name: BRIEF-location-intelligence-archetypes-2026-06-01
language_protocol: CODE-RESEARCH
status: active
created: 2026-06-01
author: totebox@project-gis
---

# Location Intelligence Archetypes — Research Brief

Three canonical co-location archetypes under the **Location Intelligence** product concept.
Three-letter codes ratified 2026-06-01.

---

## 1. The Three Archetypes

| Code | Name | Status | Anchor type |
|---|---|---|---|
| **PRO** | Retail Centres | Live — T1/T2/T3 clusters | Grocery hypermarket + hardware ± price club / lifestyle / electronics |
| **VWH** | Urban Fringe | Live — 6,368 clusters (T1=852 / T2=1,327 / T3=4,189) | Hardware + trade-supply ecosystem (MRO, tool rental, builders merchant, auto parts); group-collapse tiers; `retail_contamination` flag |
| **PKS** | Commuter | Live — 6,953 clusters (T1=691 / T2=2,658 / T3=3,604) | Park-and-ride anchor; transit modes + car rental + hotel enrichment; mode-group collapse tiers |

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

### Production state (2026-06-11)

Calibrated via two-step profile → simulate → calibrate (same methodology as PKS). Full methodology in §11.

**Calibration anchors:** 10,338 hardware store locations (45 chains) across NA/EU.
**Sim validation:** 73.4% of T1+T2 clusters have a hardware store within 3km (target ≥55%) — PASS.

Key design decisions:
- Hypermarket excluded from DBSCAN inputs (73.9% co-occurrence — retail park noise); checked
  post-cluster as `retail_contamination` flag instead
- CBD proximity filter NOT applied (73.6% of hardware stores are >30km from major metro refs;
  METRO_REFS covers only major cities, not mid-size cities where hardware operates)
- Group-collapse tier rules replace additive score (§11.3)
- EPS_LOOSE=3.0km (not 2.5km — discrete parcels, no rail-network-collapse risk)

**Production results (`build-vwh-clusters.py`, 2026-06-11):**

| Metric | Value |
|---|---|
| Total clusters | 6,368 |
| T1 (Full Trade Hub) | 852 (13.4%) |
| T2 (Established Trade) | 1,327 (20.8%) |
| T3 (Emerging / Thin) | 4,189 (65.8%) |
| `retail_contamination` | 3,048 (47.9%) — hypermarket <1km; informational flag |

Country distribution: US=3,167 / DE=648 / GB=543 / CA=506 / FR=420 / NL=240 / IT=226 / PL=171.

T3-heavy is expected: hardware-alone = thin T3; full trade hubs (MRO + tool rental + builders
merchant + auto) are legitimately rare. Mirrors PKS T3=51.9%.

Deployed: `gateway-orchestration-gis-1/www/data/archetype-vwh.geojson` (3.0MB, 6,368 features)

---

## 4. PKS — Commuter

### Definition

A 3–9 story car parking structure at a **regional** airport or intercity train station.
Function: residents of a Regional Market (PRO cluster) park and fly/train to a Metro Market.
The Regional Market **feeds** the Metro Market either by plane or by train.

"Regional" = 15–150km from the major metro centre.
- ≤15km = suburb; the plane or train saves little over driving
- 15–150km = sweet spot; 1–2 hours saved justifies park-and-transit behaviour
- \>150km = standalone market; likely has its own metro without a feeder relationship

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
| `test-cluster-archetypes.py` | Produces VWH and PKS GeoJSON from existing data | ✅ Done; runs in ~60s |
| `ingest-osm-airports.py` | IATA-filtered commercial airport ingest (replaces Overture 20,841) | ✅ Done — 4,024 airports (3,774 w/ IATA); US tile gaps remain |
| `ingest-osm-railway.py` | OSM intercity railway station ingest → service-places | ✅ Done — 18,107 intercity stations, 16 countries |
| `ingest-osm-industrial.py` | OSM `landuse=industrial` polygon ingest | ❌ Planned |
| Chain YAMLs (see §3 production gaps) | Auto-parts, paint, car rental, logistics | ✅ Done — 19,242 records / 32 chains |

**Final PKS run (2026-06-01 08:10Z):** with real transit anchors (4,024 IATA airports +
18,107 intercity rail), test produced **14,332 PKS candidates** (1,744 airport + 12,588 rail),
**3,904 integrated** with a T1/T2 Regional Market (637 airport + 3,267 rail). Railway dominates —
EU park-and-train pattern. Replaces the earlier 6,640-candidate Overture-airport-only proxy.

---

## 7. UI integration (updated 2026-06-01)

VWH and PKS toggle buttons placed under **★ Regional Markets** section in `index.html` layer control
(not a separate Location Intelligence group — that concept was retired). PRO Retail Centres
is the base map product and is NOT a toggle overlay alongside VWH/PKS.

State variables: `vwhActive`, `psActive`. Functions: `toggleVwhLayer(btn)`, `togglePsLayer(btn)`.

**Fade behaviour**: when VWH or PKS toggled on, cluster bubbles ghost to `circle-opacity: 0.10`
(identical to rm-stars Top 400 fade behaviour — copies `applyRmStarsStyle()` pattern exactly).
`applyLiOverlayStyle()` function handles this; restore branch guards against all active overlay
states (`rmStarsActive`, `koppenActive`, `ecoregionsActive`, `vwhActive`, `psActive`).

Layers persist across all view transitions (BentoBox, Retail View) — same behaviour as Köppen.
BentoBox inspector is NOT modified — VWH/PKS remain map overlays only.

**`retail_contamination` badge — IMPLEMENTED 2026-06-13:**
`showArchetypeDetail()` now renders a `rm-badge`-styled "Mixed-use site — hypermarket within 1 km"
warning for VWH clusters where `p.retail_contamination === true` (3,048 / 6,368 clusters).
Implementation: `contamBadge` variable rendered between the tier cell and composition chips.
Reuses existing `.rm-badge` CSS class (line 113 of index.html). No CSS changes needed.

Future: BentoBox cluster inspector shows nearby VWH/PKS as a "Location Intelligence context"
panel — separate scope, touches `showClusterDetail()`.

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

### Ingestion sequencing (recommended order)

1. **Run existing YAMLs** — `ingest-osm.py --chain autozone-us oreilley-auto-us napa-us sherwin-williams-us halfords-uk`
2. **Add Würth** — single biggest EU gap; YAML + `mro_industrial` category in taxonomy.py
3. **Add Floor & Decor + United Rentals + Sunbelt** — clearest new retail/rental VWH signals
4. **Write `ingest-osm-airports.py`** — fix 20,841 noise before adding car rental
5. **Add car rental YAMLs** — Enterprise/Hertz/Avis/Sixt/Europcar once airports are clean
6. **Write `ingest-osm-railway.py`** — EU intercity stations to service-places
7. **Re-run `test-cluster-archetypes.py`** — updated VWH and PKS candidate counts

Editorial artifacts produced this session:
- `BRIEF-location-intelligence-archetypes-2026-06-01.md` (this file) — active
- `TOPIC-vertical-warehouse.draft.md` — dispatched to project-editorial
- `TOPIC-parking-structures.draft.md` — dispatched to project-editorial
- `GUIDE-location-intelligence-data-collection.draft.md` — dispatched to project-editorial

---

## 10. PKS Production System — Build State (updated 2026-06-11)

The §4/§6/§8/§9 sections above describe the test-data research phase (2026-06-01).
PKS moved into production in June 2026 with a new DBSCAN-based build script.
This section is the authoritative state record from that point forward.

### 10.1 Current build script

File: `pointsav-monorepo/app-orchestration-gis/build-pks-clusters.py`
(In gitignored subdirectory — unversioned. See §10.11 for version control debt item.)

**No metro-distance ring gate.** The §4 ring (15–150km) has been removed.
Category selection + enrichment gate drives location naturally.

Transit sources ingested:
- `cleansed-civic-airports.jsonl` — commercial airports (is_significant_airport filter)
- `cleansed-civic-railway.jsonl` — intercity rail stations
- `cleansed-civic-railway-commuter.jsonl` — commuter rail + metro/subway
- `cleansed-civic-bus-terminal.jsonl` — intercity bus terminals

Commercial enrichment:
- Car rental chains: enterprise-us, hertz-us, avis-us, enterprise-ca, hertz-mx, europcar-fr, sixt-de
- Park-and-ride: `cleansed-civic-parking.jsonl` (parking_class=park_ride filter)

Clustering: two-pass DBSCAN (tight=1.0km, loose=2.5km). Max span=8km.

### 10.2 Schema — output properties

GeoJSON output: `work/archetype-pks.geojson` → deployed `www/data/archetype-pks.geojson`

| Property | Type | Description |
|---|---|---|
| `commuter_tier` | int 1/2/3 | Tier assignment (front-end reads this) |
| `transit_categories` | array | Transit modes present (airport, intercity_rail, commuter_rail, metro_subway, intercity_bus) |
| `multi_modal` | bool | ≥2 distinct mode GROUPS present (not raw transit mode count) |
| `car_rental` | bool | Car rental co-located in cluster |
| `pks_signal` | array | All category signals in cluster (transit + enrichment) |
| `span_km` | float | Cluster diameter |
| `metro_dist_km` | float | Distance to nearest metro reference point |
| `node_count` | int | POI count in cluster |
| `iso` | str | ISO-2 country code |

### 10.3 Tier logic — mode-group collapse (implemented 2026-06-11)

**Problem discovered:** ICR (intercity_rail) + CR (commuter_rail) at the same physical station
were being counted as two distinct transit modes. 57.3% of clusters were ICR+CR "bimodal"
due to this — completely artificial inflation.

**Fix:** Collapse transit categories into four mode GROUPS before tier calculation:
- **AIR**: airport
- **RAIL**: intercity_rail OR commuter_rail (same physical platform, two service levels)
- **URBAN**: metro_subway
- **BUS**: intercity_bus

Only distinct GROUPS count toward multimodality. A station that appears in both
`cleansed-civic-railway.jsonl` and `cleansed-civic-railway-commuter.jsonl` is RAIL (one group).

**Enrichment classes** (parallel to mode groups):
- **RENTAL**: car_rental
- **PARK**: park_ride
- **HOTEL**: hotel (added 2026-06-11 — see §10.10)

### 10.4 Qualification gate (implemented 2026-06-11)

Not all transit clusters are PKS. Pure walk-up urban stops are excluded.

A cluster qualifies as PKS if ANY of:
- `AIR` mode group present (airports are inherently drive-to)
- ≥2 mode groups present (genuine multimodal interchange)
- ≥1 enrichment class present (parking/rental evidence of drive-to behaviour)

Effect: 11,652 of 19,653 raw DBSCAN clusters disqualified as walk-up stops.

### 10.5 Tier criteria

| Tier | Criteria | Semantics |
|---|---|---|
| T1 | (AIR + RENTAL or HOTEL) OR ≥3 groups OR (≥2 groups + ≥2 enrich) OR (AIR + ≥1 enrich) | Confirmed commercial hub: drive-to + traveller commerce |
| T2 | AIR alone OR (≥2 groups + ≥1 enrich) OR (1 group + ≥2 enrich) | Drive-to hub; enrichment evidence but not full T1 |
| T3 | ≥1 group + ≥1 enrich (qualifying, not T1/T2) | Functional transit stop with single drive-evidence signal |

### 10.6 Current deployment state (updated 2026-06-11 — with car rental + hotel)

**Live file:** `/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/archetype-pks.geojson`

| Metric | Value |
|---|---|
| Total features | **6,953** |
| T1 | 691 (9.9%) |
| T2 | 2,658 (38.2%) |
| T3 | 3,604 (51.9%) |
| Disqualified (walk-up stops) | 9,051 |

Country totals: US=2,226 / DE=1,403 / FR=908 / GB=591 / CA=318 / AT=267 / PL=239 / ES=228 / NL=156 / SE=153 / IT=99 / NO=79 / MX=75 / FI=72 / PT=59 / DK=40 / GR=26 / IS=14

Previous deployment (before car rental + hotel, 2026-06-11 morning): 4,934 features T1=326/T2=2,219/T3=2,389.

Distribution vs target (retail-derived): T1 ✓ (5–10%), T2 slightly over (38% vs 25–35% target),
T3 slightly under (52% vs 55–65%). Target derived from PRO retail — transit hubs naturally
cluster more multi-modally than retail chains, so higher T2% is expected and defensible.

### 10.7 Root cause of T3 heaviness — park_ride ingest gap

`ingest-osm-parking.py` runs Overpass queries for `parking_class=park_ride` records.
The script covers US, CA, DE, FR via TILE_GRIDS but was never run for those countries.

Current park_ride coverage (parking_class=park_ride, DISPLAY_ISO countries):
- AT=1,713 / NL=1,125 / GB=639 / ES=535 / SE=474 / FI=361 / MX=261 / DK=135 / PT=113 / GR=40
- **US=0 / CA=0 / DE=0 / FR=0 / IT=0 / PL=0 / NO=0 / IS=0** ← ingest never run

Effect: rail/bus clusters in US/CA/DE/FR have no PARK enrichment → T3 or DISQ instead of T2.
The 6,953 deployment figure reflects this gap — park_ride ingest for large countries is pending.

**Action (run after 05:00 UTC / 10pm Vancouver):**
```bash
cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis
nohup python3 ingest-osm-parking.py --countries US CA DE FR IT PL NO IS >> ingest.log 2>&1 &
```
After ingest completes: re-run `python3 build-pks-clusters.py`.
Expected result: ~7,500–9,000 features, with US/CA/DE/FR gaining more T2 clusters from PARK enrichment.

### 10.8 Crontab fix required (one-time operator action)

Current crontab (wrong — runs old ring-filtered script from project-orgcharts):
```
0 5 * * * cd /srv/foundry/clones/project-orgcharts/... && bash nightly-rebuild.sh
0 5 * * 1 cd /srv/foundry/clones/project-orgcharts/... && bash build-aec-global.sh
```

Fix (run as mathew — auto-classifier blocks AI from running this):
```bash
crontab -l | sed 's|project-orgcharts|project-gis|g' | crontab -
```
Changes entries 1 (nightly-rebuild.sh) and 3 (build-aec-global.sh) to project-gis.
Entry 2 (run-overnight-ingests.sh, June 4) is a past date — harmless, leave it.

### 10.9 EU car rental data — COMPLETED 2026-06-11

Car rental ingests completed this session. Coverage now 7,297 records across all display countries.

| YAML | Records | Notes |
|---|---|---|
| `hertz-eu.jsonl` | 687 | name_query fallback |
| `avis-eu.jsonl` | 741 | name_query fallback |
| `budget-eu.jsonl` | 130 | name_query fallback |
| `europcar-eu.jsonl` | 1,021 | name_query fallback |
| `sixt-eu.jsonl` | 246 | wikidata Q704156 |
| `budget-us.jsonl` | 278 | NA; polygon-filtered |
| `alamo-us.jsonl` | 110 | NA; polygon-filtered |
| `national-us.jsonl` | 2 | sparse OSM |

Remaining gap: generic `amenity=car_rental` OSM query (brand-agnostic, high EU leverage) — planned
for next data phase.

### 10.10 Hotel chains — COMPLETED 2026-06-11

Hotel added as a third enrichment class (HOTEL). T1 criteria updated to include `AIR + HOTEL`.
6,207 hotel records ingested; 6,124 loaded after dedup.

| YAML | Records | Notes |
|---|---|---|
| `ibis-eu.jsonl` | 708 | Q920166; EU-wide |
| `b-and-b-hotels-eu.jsonl` | 797 | Q794939 |
| `premier-inn-gb.jsonl` | 817 | Q2108626; GB only |
| `travelodge-gb.jsonl` | 580 | Q9361374; GB only |
| `motel-one-de.jsonl` | 24 | Q866334; name_query |
| `holiday-inn-express-us.jsonl` | 2,021 | Q5880423; US |
| `hampton-us.jsonl` | 240 | name_query "Hampton Inn" |
| `courtyard-us.jsonl` | 1,020 | Q1053170; US |

Effect of adding hotel: +1,229 clusters vs car-rental-only build (4,934 → 6,953).

### 10.11 Version control debt

`build-pks-clusters.py` and all operational scripts are in the gitignored
`pointsav-monorepo/app-orchestration-gis/` subdirectory — unversioned.
Changes made 2026-06-11 are live but not in git history.

Resolution options (deferred):
1. `git init` the `pointsav-monorepo/` subdirectory as a standalone repo
2. Promote key scripts to tracked `app-orchestration-gis/` at repo root (preferred)

---

### 10.12 Simulation calibration methodology (2026-06-11)

**Research question:** What POI categories, clustering thresholds, and tier rules produce a PKS
co-location dataset that is calibrated to retail PRO scale and defensibly distributed across T1/T2/T3?

**Step 1 — Profile existing parkades** (`analyze-parkade-colocation.py`)

Profiled 140,201 existing parking structures (114,835 built + 25,366 park_ride) from
`cleansed-civic-parking.jsonl` against all POI categories within 3km. Key findings:

| Category | Built % | Park-ride % |
|---|---|---|
| commuter_rail | 64.7% | 50.8% |
| hypermarket | 50.1% | 32.8% (noise — retail parking garages) |
| intercity_rail | 47.0% | 41.9% |
| car_rental | 44.7% | 17.2% |
| hotel | 36.2% | 12.6% |
| airport | 4.1% | 2.7% |

Interpretation:
- commuter_rail dominates — PKS is overwhelmingly a rail archetype, not an airport archetype
- hypermarket at 50.1% = retail parking garages; excluded as commercial noise
- car_rental (44.7%) and hotel (36.2%) are the clean transit-commercial signals
- airport (4.1%) is real but minority — confirms rail-first design

Output: `work/parkade-colocation-profile.json`

**Step 2 — Simulation iterations** (`sim-pks-colocation.py`)

Ran five iterations to find the right combination of categories and clustering parameters:

| Iteration | Approach | Total | T1% | T2% | T3% | Outcome |
|---|---|---|---|---|---|---|
| 1 (default) | Transit-first, EPS 3.0km | 1,695 | 10.8% | 79.7% | 9.5% | T0 gate excludes 93% — branded chain data too thin |
| 2 (lenient) | Transit-first, 3.0km, T3 allowed alone | 22,846 | 1.3% | 57.3% | 41.5% | Rail network collapse at 3km |
| 3 (no commuter) | Intercity+airport only, 3.0km | 17,703 | 1.4% | 6.6% | 92.0% | 13,467 intercity-rail-alone T3 singletons |
| 4 (commercial-first) | Cluster car_rental+hotel, check transit | 2,635 | 11.4% | 81.6% | 7.0% | Commercial signal too sparse at retail scale |
| 5 (tight EPS) | Transit-first, 2.5km | 1,695 | 10.8% | 79.7% | 9.5% | Same T0 problem — EPS doesn't fix chain sparsity |

**Step 3 — Root cause and production resolution**

The sim confirmed that the production `build-pks-clusters.py` approach is correct for three reasons:

1. **Park-and-ride as anchor**: 23,117 discrete park-and-ride records provide a geographically
   distributed anchor set — neither as continuous as rail networks nor as sparse as branded chains.
   These are actual car→transit transition points, exactly the co-location target.

2. **EPS_LOOSE = 2.5km** (not 3.0km): prevents rail network collapse while still grouping the
   physically meaningful 1–2km station catchment.

3. **Enrichment gate (not input)**: transit modes are the anchor; car_rental/hotel/park_ride are
   enrichment evidence. Removing the parkade from the anchor set (as the sim attempted) leaves
   only branded chains — too thin to reproduce the right cluster count.

**Calibration verdict**: The production build (6,953 clusters; T1=9.9%, T2=38.2%, T3=51.9%)
is the calibrated PKS dataset. T2 is slightly higher than the PRO retail target (38% vs 25–35%)
because transit nodes naturally co-locate multi-modally more than retail chains do — this is
expected and defensible.

**Next step — opportunity scoring**:
Add `opportunity_class` as a second-pass property per cluster:
- **DEVELOP**: T3 cluster, single enrichment signal → parkade absent or underbuilt; highest ROI
- **EXPAND**: T2 cluster, rental+hotel present → existing capacity likely needs expansion
- **SATURATED**: T1 hub, full commercial ecosystem → supply likely meets demand
Implement in `build-pks-clusters.py` after tier assignment.

---

## 11. VWH Calibration Methodology (2026-06-11)

Same two-step approach as PKS (§10.12): profile hardware stores as proxy anchor → simulate
DBSCAN without anchor → calibrate tier rules → run production build with anchor included.

### 11.1 Hardware store profile (`analyze-vwh-colocation.py`)

Profiled 10,338 hardware store locations (45 chains; HARDWARE_CHAINS union across NA/EU) against
all VWH trade categories within 3km. Key findings:

| Category | Co-location % | Notes |
|---|---|---|
| hypermarket | 73.9% | Retail contamination — hardware frequently in retail parks |
| auto_parts | 51.2% | Strong but ubiquitous (AutoZone on every arterial strip) |
| parcel_depot | 31.3% | Generic — not VWH-specific |
| self_storage | 20.3% | Residential fringe — not VWH-specific |
| trade_counter | 13.5% | UK-diagnostic (Screwfix/Toolstation) |
| builders_merchant | 11.4% | EU-diagnostic (Travis Perkins, Point-P, Bauking) |
| mro_industrial | 10.4% | Globally diagnostic but sparse (Würth, Fastenal, Grainger) |
| tool_rental | 7.8% | Diagnostic (United Rentals, Sunbelt) |
| flooring | 7.8% | Sparse (Floor & Decor US, Topps Tiles UK) |
| electrical | 5.0% | Very sparse (CEF UK, Rexel FR) |
| plumbing | 4.7% | Very sparse (Ferguson US, Wolseley UK) |
| lumber | 0.9% | Insufficient (84 Lumber, Builders FirstSource — US only) |
| welding | 0.3% | Insufficient (BOC UK only, 12 POIs) |

Metro distance distribution: 73.6% of hardware stores are >30km from the nearest METRO_REFS
reference point. CBD proximity filter is NOT useful — METRO_REFS covers only major metros, not
the mid-size cities where hardware operates. Confirmed: do NOT gate VWH clusters by metro distance.

Output: `work/hardware-colocation-profile.json`

**Key distinction from PKS:** Hypermarket at 73.9% is the dominant contamination signal —
hardware stores co-locate heavily with grocery retail parks, not just industrial land.
The `retail_contamination` flag (hypermarket within 1km) addresses this.

### 11.2 Simulation (`sim-vwh-colocation.py`)

Ran one iteration to validate category selection and T0 gate design.

**Configuration:**
- DBSCAN input: TRADE + AUTO + SUPPORT categories (hardware and hypermarket EXCLUDED)
- Hardware EXCLUDED: held out as out-of-sample validation anchor
- Hypermarket EXCLUDED: would dominate clustering and produce retail-park clusters, not VWH
- DBSCAN: EPS_TIGHT=1.0km, EPS_LOOSE=3.0km (NOT 2.5km — VWH POIs are discrete parcels, no rail network collapse risk), span max 8km
- T0 gate: ≥2 categories AND ≥1 TRADE category
- Tier: group-collapse (see §11.3)

| Metric | Result | Target | Status |
|---|---|---|---|
| Total clusters | 1,555 | — (sim without anchor; production scale different) | — |
| Hardware validation | 73.4% of T1+T2 within 3km hardware | ≥55% | PASS ✓ |
| T1% | 58.3% | — | — |
| T2% | 41.7% | — | — |
| T3% | 0% | — | Expected (T3 space requires hardware in input; see note) |

Note: T3=0% in sim is expected. Without hardware as a category, every qualifying cluster
has ≥1 TRADE category → meets T1 (≥2 TRADE) or T2 (TRADE+AUTO or TRADE+SUPPORT). T3 space
emerges in the production build where hardware-alone clusters (thin VWH sites) are admitted
as T3.

**Root cause of PKS vs VWH sim difference:**
PKS sim attempted 5 iterations because EPS and anchor type required exploration. VWH converged
in 1 iteration because (a) EPS is pre-validated (3.0km safe — no linear network risk), and (b)
the hardware proxy is more geographically distributed than transit infrastructure.

### 11.3 Group-collapse tier rules (`build-vwh-clusters.py` update 2026-06-11)

Replaced additive score (line 378-399 old) with group-collapse:

```
TRADE_CATS = {mro_industrial, tool_rental, lumber, builders_merchant,
              trade_counter, electrical, plumbing, welding, flooring}
AUTO_CATS  = {auto_parts}
SUPPORT_CATS = {self_storage, paint, parcel_depot}

qualify_vwh(cats):
  "hardware" in cats → True (hardware alone = T3 site)
  OR len(cats) >= 2 AND cats ∩ (TRADE | AUTO) → True

tier_vwh(cats):
  T1: (hardware AND len(TRADE) >= 2) OR (hardware AND TRADE AND AUTO)
      OR len(TRADE) >= 3                   [EU dense without big-box]
  T2: (hardware AND TRADE) OR len(TRADE) >= 2 OR (hardware AND AUTO AND SUPPORT)
  T3: everything else qualifying (hardware-alone, hardware+auto-only, hardware+support-only)
```

Also added: `retail_contamination` property (hypermarket within 1km of centroid).
Also added: `metro_dist < 2` count logged at build time (contamination check).

### 11.4 Production build results

**build-vwh-clusters.py** run 2026-06-11 with group-collapse tier rules:

| Metric | Value |
|---|---|
| Total clusters | 6,368 |
| T1 (Full Trade Hub) | 852 (13.4%) |
| T2 (Established) | 1,327 (20.8%) |
| T3 (Emerging / Thin) | 4,189 (65.8%) |
| CBD check (metro_dist < 2km) | 16 (0.25%) — effectively zero ✓ |
| Retail contamination flag | 3,048 (47.9%) — informational only |
| Hardware validation (from sim) | 73.4% T1+T2 within 3km hardware — PASS ✓ |

Country breakdown (top): US=3,167, DE=648, GB=543, CA=506, FR=420, NL=240, IT=226, PL=171.

**T3-heavy distribution is expected:** Most hardware store locations are "thin" VWH sites —
hardware present but minimal trade supply ecosystem nearby. Rich T1 hubs (MRO + tool rental +
builders merchant + auto) are legitimately rare. This mirrors PKS T3=51.9% (transit-only,
no commercial enrichment). VWH is thinner than PKS because the trade ecosystem is sparser
than transit infrastructure.

**Retail contamination note:** 47.9% of clusters have a hypermarket within 1km. This confirms
the profile finding (73.9% hypermarket co-occurrence). Hardware stores frequently anchor dual-use
commercial parks containing both grocery retail and trade supply chains. The `retail_contamination`
flag allows downstream filtering or display differentiation without removing these from the dataset
(they are still valid VWH co-locations — just mixed-use sites).

Deployed: `/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/archetype-vwh.geojson`
(3.0MB; 6,368 features)
