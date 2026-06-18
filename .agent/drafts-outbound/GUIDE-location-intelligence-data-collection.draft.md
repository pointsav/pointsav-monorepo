---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: GUIDE-OPERATIONS
status: draft
created: 2026-06-01
author: totebox@project-gis
destination: project-editorial
target_repo: media-knowledge-documentation
bcsc_class: no-disclosure-implication
research_done_count: 3
research_suggested_count: 2
open_questions_count: 1
research_provenance: |
  Existing ingest-osm.py pipeline, taxonomy.py BRAND_FILL structure, ingest-overture.py
  CATEGORY_FILTERS pattern, cleansed-places.jsonl airport data analysis, YAML chain schema
  from leroy-merlin-it.yaml reference, COUNTRY_BBOX dict in ingest-osm.py.
research_inline: |
  YAML schema derived from direct reading of existing chain YAML files. Overpass query
  patterns from ingest-osm.py source code. Taxonomy category structure from taxonomy.py
  CATEGORIES dict and _RETAIL_CATS exclusion pattern.
---

# Location Intelligence — Data Collection Guide

> **Status (2026-06-11):** All seven steps below are complete. VWH production build:
> 6,368 clusters (T1=852/T2=1,327/T3=4,189). PKS production build: 6,953 clusters
> (T1=691/T2=2,658/T3=3,604). This guide is retained as a reference runbook for
> re-ingesting or extending to new chains and countries.

Initial pipeline setup for ingesting VWH (Vertical Warehouse) and PKS (Parking Structures)
chain and infrastructure data. Steps are recorded in execution order.

Working directory for all commands: `/srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/`

---

## Prerequisites

- Overpass API access (queries run via `ingest-osm.py`; no API key required)
- Python 3.11+ with pipeline dependencies installed
- TOTEBOX_DATA_PATH: `/srv/foundry/deployments/cluster-totebox-personnel-1`

Verify the pipeline is clean:
```bash
python3 -c "from taxonomy import CATEGORIES, BRAND_FILL; print('taxonomy OK')"
python3 -c "from config import TOTEBOX_DATA_PATH; print('config OK')"
```

---

## Step 1 — Run existing YAML ingests (VWH auto-parts + paint)

Five chain YAMLs were scaffolded on 2026-06-01. Run the ingest to download OSM records:

```bash
python3 ingest-osm.py --chain \
  autozone-us \
  oreilley-auto-us \
  napa-us \
  sherwin-williams-us \
  halfords-uk
```

Expected output files in `$TOTEBOX_DATA_PATH/service-fs/service-business/`:
- `autozone-us.jsonl` — expect 5,000–7,000 records
- `oreilley-auto-us.jsonl` — expect 5,000–7,000 records
- `napa-us.jsonl` — expect 3,000–6,000 records (franchise network; OSM partial)
- `sherwin-williams-us.jsonl` — expect 3,000–5,000 records
- `halfords-uk.jsonl` — expect 300–450 records

Verify:
```bash
for f in autozone-us oreilley-auto-us napa-us sherwin-williams-us halfords-uk; do
  echo "$f: $(wc -l < $TOTEBOX_DATA_PATH/service-fs/service-business/$f.jsonl) records"
done
```

If a chain returns 0 records, check if Wikidata tag coverage is sparse in OSM and add
`name_query:` fallback to the YAML (see YAML schema below).

---

## Step 2 — Add Würth (biggest EU MRO gap)

Würth is the single highest-value chain not yet in the taxonomy. ~1,500 EU branches in
industrial parks across DE/FR/IT/PL/AT/NL and beyond. OSM has `brand:wikidata=Q183759`
applied to many records.

**2a. Create YAML:**

```bash
cat > $TOTEBOX_DATA_PATH/service-fs/service-business/wurth-de.yaml << 'EOF'
schema: service-business-chain-v1
chain_id: wurth-de
country: Germany
country_code: DE
region: europe-central
category: Industrial MRO supply
category_slug: mro-industrial
naics_code: "423840"
top_category: Industrial and Personal Service Paper and Related Products
sub_category: Industrial MRO Supply
overture_taxonomy:
  - industrial
  - wholesale
brand_family: MROIndustrial
retailer: Würth
canonical_name: "Adolf Würth GmbH & Co. KG"
parent_company: "Würth Group"
website: wuerth.de
wikidata_id: Q183759
osm_overpass_tag: brand:wikidata=Q183759
store_count_approx: 1500
locations_file: locations/wurth-de.jsonl
locations_status: pending
last_updated: 2026-06-01
notes: "VWH archetype signal. MRO distributor; branches in industrial parks across EU. Multi-country — set country_code per-ISO ingest or use multi_country: true."
EOF
```

**2b. Add `mro_industrial` category to taxonomy.py:**

In `taxonomy.py`, inside the CATEGORIES dict, after the `paint` block:
```python
"mro_industrial": {
    "label": "Industrial MRO Supply",
    "naics": "423840",
    "description": "Maintenance, repair, and operations distributor — VWH signal. Never gates tier.",
},
"flooring": {
    "label": "Flooring & Tile Supply",
    "naics": "442210",
    "description": "Contractor-facing flooring/tile warehouse — VWH signal. Never gates tier.",
},
"tool_rental": {
    "label": "Tool & Equipment Rental",
    "naics": "532412",
    "description": "Equipment rental branch — VWH signal; deliberate hardware co-location. Never gates tier.",
},
"lumber": {
    "label": "Lumber & Building Materials",
    "naics": "444190",
    "description": "Lumber yard or building materials dealer — VWH signal. Never gates tier.",
},
"car_rental": {
    "label": "Car Rental",
    "naics": "532111",
    "description": "Car rental branch — PKS signal; defines transit node commercial zone. Never gates tier.",
},
```

None of these go into `_RETAIL_CATS` — they never affect T1/T2/T3 tier logic.

**2c. Add BRAND_FILL entries for all new categories:**

In the BRAND_FILL dict, after the existing `paint` block:
```python
"mro_industrial": {
    "DE": ["wurth-de"],
    "FR": ["wurth-de"],   # Würth is multi-country; chain_id shared
    "IT": ["wurth-de"], "ES": ["wurth-de"], "PL": ["wurth-de"],
    "AT": ["wurth-de"], "NL": ["wurth-de"], "GB": ["wurth-de"],
    "US": ["fastenal-us", "grainger-us"],
    "CA": [], "MX": [],
    "SE": ["wurth-de"], "DK": ["wurth-de"], "NO": ["wurth-de"],
    "FI": ["wurth-de"], "IS": [], "GR": [], "PT": [],
},
"flooring": {
    "US": ["floor-decor-us"],
    "GB": ["topps-tiles-uk"],
    "CA": [], "MX": [], "FR": [], "DE": [], "ES": [], "IT": [],
    "GR": [], "PL": [], "AT": [], "NL": [], "PT": [],
    "SE": [], "DK": [], "NO": [], "FI": [], "IS": [],
},
"tool_rental": {
    "US": ["united-rentals-us", "sunbelt-rentals-us"],
    "CA": ["united-rentals-us"],
    "FR": ["loxam-fr", "kiloutou-fr"],
    "MX": [], "GB": [], "DE": [], "ES": [], "IT": [],
    "GR": [], "PL": [], "AT": [], "NL": [], "PT": [],
    "SE": [], "DK": [], "NO": [], "FI": [], "IS": [],
},
"lumber": {
    "US": ["84-lumber-us", "builders-firstsource-us"],
    "CA": ["kent-building-supplies-ca"],
    "MX": [], "GB": [], "FR": [], "DE": [], "ES": [], "IT": [],
    "GR": [], "PL": [], "AT": [], "NL": [], "PT": [],
    "SE": [], "DK": [], "NO": [], "FI": [], "IS": [],
},
"car_rental": {
    "US": ["enterprise-us", "hertz-us", "avis-us"],
    "CA": ["enterprise-us", "hertz-us"],
    "MX": [],
    "DE": ["sixt-de"], "FR": ["europcar-fr", "sixt-de"],
    "GB": ["europcar-fr", "sixt-de"], "ES": ["europcar-fr"],
    "IT": ["europcar-fr"], "NL": ["europcar-fr"],
    "GR": [], "PL": [], "AT": ["sixt-de"], "PT": [],
    "SE": ["sixt-de"], "DK": ["sixt-de"], "NO": [], "FI": [], "IS": [],
},
```

**2d. Run Würth ingest:**
```bash
python3 ingest-osm.py --chain wurth-de
```
Expect 800–1,500 records. Würth branches are often tagged `shop=trade` or `shop=wholesale`
in OSM — the brand:wikidata tag is the reliable signal.

---

## Step 3 — Add Tier A retail/rental VWH chains

Create YAMLs following the schema below, then run ingest.

**YAML schema reference:**
```yaml
schema: service-business-chain-v1
chain_id: floor-decor-us          # matches BRAND_FILL key
country: United States
country_code: US
region: north-america
category: Flooring supply          # display only
category_slug: flooring-supply
naics_code: "442210"
top_category: Floor Covering Stores
sub_category: Floor Covering Stores
brand_family: Flooring             # matches category
retailer: Floor & Decor
canonical_name: "Floor & Decor Holdings, Inc."
parent_company: "Floor & Decor Holdings, Inc. (public; NYSE: FND)"
website: flooranddecor.com
wikidata_id: Q22350998
osm_overpass_tag: brand:wikidata=Q22350998
store_count_approx: 240
locations_file: locations/floor-decor-us.jsonl
locations_status: pending
last_updated: 2026-06-01
notes: "VWH Tier A. Warehouse-format contractor flooring; same footprint as Home Depot."
```

**Chains to create (copy and adapt schema above):**

| chain_id | retailer | wikidata_id | country_code | approx_count |
|---|---|---|---|---|
| `floor-decor-us` | Floor & Decor | Q22350998 | US | 240 |
| `topps-tiles-uk` | Topps Tiles | Q7825827 | GB | 300 |
| `united-rentals-us` | United Rentals | Q7889284 | US | 1400 |
| `sunbelt-rentals-us` | Sunbelt Rentals | Q7645154 | US | 1100 |
| `loxam-fr` | Loxam | Q6692217 | FR | 1100 |
| `kiloutou-fr` | Kiloutou | Q3197034 | FR | 600 |
| `fastenal-us` | Fastenal | Q1394323 | US | 3400 |
| `grainger-us` | Grainger | Q904633 | US | 600 |
| `hilti-ch` | Hilti | Q565285 | CH | 600 |
| `84-lumber-us` | 84 Lumber | Q4641204 | US | 310 |
| `builders-firstsource-us` | Builders FirstSource | Q4934620 | US | 570 |
| `kent-building-supplies-ca` | Kent Building Supplies | Q6383907 | CA | 45 |

Run all at once after YAMLs are created:
```bash
python3 ingest-osm.py --chain \
  floor-decor-us topps-tiles-uk \
  united-rentals-us sunbelt-rentals-us loxam-fr kiloutou-fr \
  fastenal-us grainger-us hilti-ch \
  84-lumber-us builders-firstsource-us kent-building-supplies-ca
```

Note: `fastenal-us` and `84-lumber-us` may return 0 records via wikidata — add
`name_query: "Fastenal"` / `name_query: "84 Lumber"` fallback if needed.

---

## Step 4 — Write `ingest-osm-airports.py` (commercial airport filter)

The existing Overture airport data (20,841 records) includes private airstrips, heliports,
and military fields. This script replaces it with OSM-sourced commercial airports only.

**Pattern:** copy `ingest-osm-civic.py`, change the Overpass query to:
```python
QUERY = """
[out:json][timeout:60];
(
  node["aeroway"="aerodrome"]
    ["aerodrome:type"~"^(public|international|regional|domestic)$"]
    ({bbox});
  node["aeroway"="aerodrome"]["iata"~"."]({bbox});
  way["aeroway"="aerodrome"]
    ["aerodrome:type"~"^(public|international|regional|domestic)$"]
    ({bbox});
  way["aeroway"="aerodrome"]["iata"~"."]({bbox});
);
out center;
"""
```

Output: `$TOTEBOX_DATA_PATH/service-places/cleansed-civic-airports.jsonl`

Schema: same as cleansed-places.jsonl with `category_id: "airport"`, `naics_code: "488119"`.
Enrich with IATA code from OSM `iata` tag where present — store in `brand_wikidata` field
or a new `iata_code` field.

Run per country using `COUNTRY_BBOX` from `config.py` (already defined for all 17 display ISOs).

Expected output: ~5,000–8,000 records (vs. 20,841 from Overture).

---

## Step 5 — Write `ingest-osm-railway.py` (intercity stations)

Output: `$TOTEBOX_DATA_PATH/service-places/cleansed-civic-railway.jsonl`

**Overpass query per country:**
```python
QUERY = """
[out:json][timeout:60];
(
  node["railway"="station"]
    ["station"!="subway"]
    ["station"!="light_rail"]
    ["station"!="tram"]
    ["station"!="monorail"]
    ({bbox});
  way["railway"="station"]
    ["station"!="subway"]
    ["station"!="light_rail"]
    ["station"!="tram"]
    ["station"!="monorail"]
    ({bbox});
);
out center;
"""
```

**Post-processing:** filter to intercity operators only using the `operator=` tag:

```python
INTERCITY_OPERATORS = {
    "US": ["Amtrak"],
    "CA": ["VIA Rail Canada", "Via Rail"],
    "FR": ["SNCF", "Société Nationale des Chemins de fer Français"],
    "DE": ["Deutsche Bahn", "DB", "DB Regio"],
    "ES": ["Renfe", "Renfe Operadora"],
    "IT": ["Trenitalia", "Italo", "Ferrovie dello Stato"],
    "AT": ["ÖBB", "Österreichische Bundesbahnen"],
    "NL": ["NS", "Nederlandse Spoorwegen"],
    "SE": ["SJ", "Norrtåg"],
    "DK": ["DSB", "Danske Statsbaner"],
    "NO": ["Vy", "NSB"],
    "FI": ["VR", "VR Group"],
    "PT": ["CP", "Comboios de Portugal"],
    "PL": ["PKP Intercity", "RegioJet"],
    "GB": None,  # All Network Rail TOCs qualify — no filtering needed
    "MX": None,  # No passenger rail — skip
    "IS": None,  # No passenger rail — skip
    "GR": ["TrainOSE"],
}
```

For countries where `operator=` filtering applies: only keep stations where `operator` tag
matches (case-insensitive substring). For GB: keep all (Network Rail TOCs all serve intercity).
For MX and IS: skip entirely (no intercity rail).

Schema: `category_id: "railway_station"`, `naics_code: "482111"`.

---

## Step 6 — Add PKS car rental chain YAMLs

Create five YAMLs following the same schema as Step 3:

| chain_id | retailer | wikidata_id | notes |
|---|---|---|---|
| `enterprise-us` | Enterprise Rent-A-Car | Q2283517 | Multi-country NA |
| `hertz-us` | Hertz | Q379425 | Multi-country NA + EU |
| `avis-us` | Avis | Q849144 | Multi-country |
| `sixt-de` | Sixt | Q704156 | EU primary; some NA |
| `europcar-fr` | Europcar | Q466704 | EU primary |

`naics_code: "532111"` (Passenger Car Rental). `brand_family: CarRental`.

These all need `multi_country: true` in the YAML since they operate across multiple ISOs
in the display list.

Run ingest:
```bash
python3 ingest-osm.py --chain enterprise-us hertz-us avis-us sixt-de europcar-fr
```

Expect high counts due to airport ubiquity: enterprise-us ~4,000–8,000 records; sixt-de
~500–800 EU records.

---

## Step 7 — Production calibration and deployment (COMPLETE 2026-06-11)

`test-cluster-archetypes.py` was superseded by dedicated DBSCAN build scripts:
- `build-vwh-clusters.py` → `archetype-vwh.geojson` (6,368 features)
- `build-pks-clusters.py` → `archetype-pks.geojson` (6,953 features)

Both deployed to:
```
/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/archetype-vwh.geojson
/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/archetype-pks.geojson
```

To re-run after adding new chains:
```bash
python3 build-vwh-clusters.py   # VWH — outputs work/archetype-vwh.geojson
python3 build-pks-clusters.py   # PKS — outputs work/archetype-pks.geojson
cp work/archetype-vwh.geojson /srv/foundry/deployments/gateway-orchestration-gis-1/www/data/
cp work/archetype-pks.geojson /srv/foundry/deployments/gateway-orchestration-gis-1/www/data/
```

---

## Open question

**Würth multi-country YAML**: the current YAML is `wurth-de` with `country_code: DE`.
The ingest-osm.py pipeline uses `country_code` to select the country bbox for the Overpass
query. To get Würth records across all EU countries, either:
a. Create per-country YAMLs (`wurth-fr.yaml`, `wurth-it.yaml`, etc.) — verbose but clean
b. Set `multi_country: true` in `wurth-de.yaml` — check if ingest-osm.py handles this for
   non-Nordic markets (the existing multi_country support was built for ikea-nordics)

Confirm approach before ingesting.
