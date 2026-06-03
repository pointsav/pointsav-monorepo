# Project GIS — Data Manifest & Licensing

This document serves as the master record of all data sources, methodologies, and licensing agreements used within the Woodfine Location Intelligence platform.

## 1. Global Data Layers

### OpenStreetMap (OSM)
*   **Purpose**: Base map data, Point-of-Interest (POI) locations for retail and civic infrastructure.
*   **License**: [Open Database License (ODbL) 1.0](https://opendatacommons.org/licenses/odbl/).
*   **Attribution**: © OpenStreetMap contributors.

### Overture Maps Foundation — Places
*   **Purpose**: Global POI theme (Places), Transportation network, and Building footprints.
*   **License**: [CDLA-Permissive-2.0](https://cdla.dev/permissive-2-0/) (Places) and [ODbL](https://opendatacommons.org/licenses/odbl/) (Transportation/Buildings).
*   **Attribution**: © Overture Maps Foundation.

### Overture Maps Foundation — Addresses
*   **Release version**: 2026-04-15.0
*   **S3 path**: `s3://overturemaps-us-west-2/release/2026-04-15.0/theme=addresses/type=address/*.parquet`
*   **Purpose**: Street-level address backfill for OSM-sourced service-business JSONL records that carry null `street_address` fields. Matched by H3 res-11 spatial proximity (≤ ~53m). Backfilled fields: `street_address`, `city`, `region`, `postal_code`. Provenance tag `address_source: overture_addresses`, confidence 0.90.
*   **Markets**: US, CA, MX, ES, FR, DE, GB, IT, NL, AT, PL, GR, PT + SE, NO, DK, FI, IS (18 countries).
*   **License**: [Open Database License (ODbL) 1.0](https://opendatacommons.org/licenses/odbl/) — share-alike; derived datasets must retain ODbL.
*   **Attribution**: © Overture Maps Foundation contributors.
*   **Script**: `app-orchestration-gis/extract-overture-addresses.py`

### WorldPop
*   **Purpose**: High-resolution (100m) gridded population data for global demographic analysis.
*   **License**: [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/).
*   **Attribution**: WorldPop (www.worldpop.org).

### WorldMove
*   **Purpose**: Synthetic human mobility trajectories for city-scale flow analysis.
*   **License**: [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/).
*   **Attribution**: Yuan et al. (2025). "WorldMove: A Global Open Data for Human Mobility".

## 2. Regional & Administrative Data

### Statistics Canada (StatCan)
*   **Purpose**: Canadian Census 2021 demographics and Household Spending surveys.
*   **License**: [Statistics Canada Open Licence](https://www.statcan.gc.ca/en/reference/licence).
*   **Attribution**: Adapted from Statistics Canada data. This does not constitute an endorsement by Statistics Canada.

### INEGI (Mexico)
*   **Purpose**: Mexican geostatistical framework and DENUE business registry.
*   **License**: [Terms of Free Use of Information (INEGI)](https://www.inegi.org.mx/servicios/api_indicadores.html).
*   **Attribution**: Source: INEGI.

### Eurostat (European Union)
*   **Purpose**: GEOSTAT population grids (1km) and Household Budget Survey (HBS) data for EU member states.
*   **License**: [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://ec.europa.eu/eurostat/about-us/policies/copyright).
*   **Attribution**: © European Union, 1995-2026.

## 3. Co-Location Tier Methodology

### Anchor Taxonomy (V3, May 2026)

The co-location engine classifies stores into four alpha anchor classes. Stores in these classes can initiate clusters; stores in other categories can join clusters but cannot initiate them.

| Class | Representative chains |
|---|---|
| **ALPHA_HYPERMARKET** | Walmart (US/CA/MX), Target (US), Fred Meyer (US), Whole Foods Market (US), H-E-B (US), Wegmans (US), WinCo Foods (US), Sprouts Farmers Market (US), Real Canadian Superstore (CA), Soriana (MX), Chedraui (MX), Mercadona (ES), Tesco (UK), Sainsbury's (UK), ASDA (UK), Morrisons (UK), Carrefour hypermarket (FR/ES/IT/PL), Auchan (FR), Bilka (DK), K-Citymarket (FI), Prisma (FI), Obs Coop (NO), Hagkaup (IS) |
| **ALPHA_LIFESTYLE** | IKEA (all regions) |
| **ALPHA_HARDWARE** | Home Depot (US/CA/MX), Lowe's (US/CA), Canadian Tire (CA), Leroy Merlin (EU), Brico Dépôt (FR/ES), Bauhaus (EU), Woodies (IE) |
| **ALPHA_WAREHOUSE** | Costco (all regions), Sam's Club (US/MX), BJ's (US), Makro (ES/NL/PL) |

Rewe (DE), Lidl, and Aldi are not ingested by design. These chains operate in neighbourhood grocery formats; their density would produce thousands of clusters below any useful district threshold. Their exclusion is a deliberate semantic decision, not a data gap.

### Tier Assignment (V3, May 2026)

Tiers are assigned by binary predicate gates (pure-geometric engine, `build-geometric-ranking.py`). A cluster is Tier 1 Regional if it passes all five gates: composition (Warehouse+Hypermarket or Lifestyle+Hypermarket), top-10% national primary catchment population, top-20% national secondary catchment population, regional hospital present, and IoU ≤ 0.10 with any stronger co-located cluster. The full predicate specification is in `SCORING-METHODOLOGY.md`.

Prior to Sprint 17 (May 2026), tiers were assigned by a composite score (V2: sum of base score, count bonus, diversity bonus, civic depth, overlap penalty). V2 sub-scores (`score_final`, `rank_v2`, and related fields) have been removed from emitted geometry; the V2 scripts are retained at `legacy/generate-rankings-v2.py`.

### Civic Data

Hospital and university classification is sourced from OpenStreetMap. Hospitals are classified as `regional`, `district`, or `clinic` based on the OSM `healthcare` tag and mapped name patterns. Universities are classified as `regional`, `small`, or `excluded`. Per-tier civic counts (`hc_count_regional`, `hc_count_district`, `he_count_regional`, `he_count_small`) are computed within a 5 km tertiary ring around each cluster anchor.

## 3b. Location Intelligence Archetypes (added 2026-06-03)

Beyond the PRO/Retail tier system above, two additional co-location archetypes are published as
GeoJSON layers loaded directly by the map (`gateway www/data/`, fetched with a `?v=` cache token).
Both are built independently of the retail clusters and target a similar bubble density (~Retail
scale) so all three archetype maps read at the same fullness.

### Urban Fringe — `archetype-vwh.geojson` (`build-vwh-clusters.py`)

The light-industrial fringe near demand — areas with industrial-ish zoning close enough to a
retail/residential catchment to host the trades that need industrial space but customer proximity
(JIT delivery depots, dry-clean plants, plumbers, painters, builders). Built from trade-supply
chain point data (hardware, mro_industrial, tool_rental, auto_parts, electrical, flooring, lumber,
plumbing, paint, welding) via two-pass DBSCAN. `qualify_vwh()` admits a cluster with ≥2 distinct
categories OR any single STRONG/BROAD trade-supply store; tier by composition strength
`tier_vwh(cats, n)`. **7,028 features** (2026-06-03). Feature properties: `vwh_tier`, `vwh_signal`
(category list), `span_km`, `member_count`, `hardware_chains`, `enrichment_chains`, `metro_dist_km`,
`nearest_metro`, `iso`.

### Commuter — `archetype-pks.geojson` (`build-pks-clusters.py`)

A park-and-ride development thesis: regional airports and outer commuter-rail termini where
commuters drive in, that do not yet have the parking structures major metro hubs already have.
Purely geometric (no metadata tags): a candidate is a sized regional airport (park-and-fly, within
~600 km of a metro reference) OR an outer commuter-rail-belt station (15–110 km ring, connected
toward the metro core, ≤4 stops from the line end). Airports lead — they are geographically
distributed and fill the map where rail does not. Tier by `tier_pks_geo(metro_d, inward, iso_km,
outward, is_airport)`. **5,977 features** (2026-06-03). Feature properties: `commuter_tier`,
`transit_categories`, `multi_modal`, `car_rental`, `metro_dist_km`, `nearest_metro`, `node_count`, `iso`.

### Planned source layers (June 4 overnight ingest)

`cleansed-civic-parking.jsonl` (OSM `park_ride` + structured parking via `ingest-osm-parking.py`)
labels each Commuter candidate BUILT / PARTIAL / GREENFIELD — the "no parkade yet" opportunity
filter. `parcel-depot-osm.jsonl` (OSM `post_depot` + `office=logistics`) plus four new VWH brand
categories (builders' merchants, self-storage, trade counters, parcel depots) add genuine
co-locations to the Urban Fringe layer.

## 4. Disclaimers & Terms of Use

The Woodfine Location Intelligence platform (`gis.woodfinegroup.com`) provides synthesized location intelligence metrics "as is." 

1.  **Metric Synthesis**: Scores, ranks, and spend estimates are the result of computational modeling using the open-source inputs listed above.
2.  **Accuracy**: While we strive for high fidelity by combining multiple authoritative sources, Woodfine Group does not guarantee the precision of specific retailer coordinates or synthesized economic metrics.
3.  **Privacy**: No individual-level tracking or personally identifiable information (PII) is utilized. All mobility data is synthetic or aggregated.
4.  **Non-Navigation**: This data is not intended for real-time navigation or critical infrastructure planning.
