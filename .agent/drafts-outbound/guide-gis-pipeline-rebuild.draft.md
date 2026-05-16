---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: woodfine/woodfine-fleet-deployment
target_path: gateway-orchestration-gis/
target_filename: guide-gis-pipeline-rebuild.md
audience: customer-operator
bcsc_class: current-fact
language_protocol: PROSE-GUIDE
authored: 2026-05-08
authored_by: project-gis Task Claude
authored_with: claude-opus-4-7
research_done_count: 4
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Derived from GIS Sprint 9, 10, 11 pipeline operations. Step times measured on
  the May 2026 deployment with ~48,000 cleansed records and ~7,000 clusters.
research_inline: false
notes_for_editor: |
  Operator-side runbook. Each command must be copy-paste runnable. Document
  failure modes the operator is likely to hit, not theoretical edge cases.
---

# Guide: GIS Pipeline Rebuild

This guide documents the end-to-end procedure for rebuilding the GIS pipeline from raw ingested data to the live deployment artefacts that serve gis.woodfinegroup.com. The full rebuild takes approximately ten minutes on the May 2026 deployment footprint.

## When to Rebuild

A full rebuild is required after any of:

- A new chain is ingested (new YAML + new JSONL data).
- An existing chain's ingest is refreshed (e.g., after an OpenStreetMap improvement campaign).
- A change to the cluster-formation algorithm in `build-clusters.py`.
- A change to the scoring algorithm in `generate-rankings.py`.
- A change to the brand-family taxonomy in `build-tiles.py`.
- A change to the region engine in `utils/region_engine.py` requires a build-clusters re-run; tile rebuild only is insufficient.

A partial rebuild — running only `build-tiles.py --layer 2` to refresh the cluster meta JSON — suffices when the only change is to the meta-JSON schema or the BentoBox renders, with no underlying data change. Most other changes require the full sequence below.

## The Five Stages

The pipeline is five sequential stages. Each stage has one primary input and one primary output; later stages depend on earlier stages.

### Stage 1 — Ingest

Pull retail and civic data from OpenStreetMap via the Overpass API.

```bash
cd pointsav-monorepo/app-orchestration-gis
python3 ingest-osm.py --chain <chain_id_1> <chain_id_2> ...
```

Or to refresh all configured chains, use `--all-pending` for chains without an existing JSONL file, or invoke per-chain.

Per chain: 30–90 seconds for a country-scale bounding box. The ingest applies a polygon-containment filter to drop bbox-contaminated cross-border records (Sprint 11). Telemetry shows `polygon-filter: dropped N cross-border records` per chain when the filter fires.

Failure modes:

- **Overpass timeout**. Three Overpass instances are tried in order; if all three fail the chain is skipped. Re-run later.
- **Wikidata returns zero**. The ingest falls back to the `name_query` field if set. If neither query returns records, the chain produces an empty JSONL and a console warning.
- **Country bbox missing**. New countries require an entry in `COUNTRY_BBOX` (see `guide-gis-adding-a-country.md`).

### Stage 2 — Cluster Entities

Deduplicate raw OSM records across chain boundaries and within-chain sub-locations.

```bash
python3 ../../service-business/cluster-entities.py
```

About 30 seconds. Two passes: same-chain spatial clustering at 200 m, then cross-brand QID dedup at 50 m. Output is `cleansed-clusters.jsonl` — the input to all subsequent stages.

This stage MUST run after any Stage 1 ingest. Skipping it means subsequent stages operate on a stale cleansed file.

### Stage 3 — Build Clusters

Form co-location clusters from the cleansed data, applying the anchor-secondary-tertiary methodology.

```bash
python3 build-clusters.py
```

About 60 seconds. Reads `cleansed-clusters.jsonl`. For each anchor-class store, evaluates secondary stores within 1 km and tertiary stores within 3 km. Assigns categorical tier composition (Hypermarket / Hardware / Warehouse / etc.) and tier descriptor (Prime / Strong / Core / Emerging). Writes `work/clusters.geojson`.

Console summary lines worth watching:

- `business: N records, places: M records` — input record counts after cleansing.
- `Tier-1 rate at 3km: NN.N%` — calibration gauge. If above 12% consider tightening the secondary radius; if below 8% the methodology may be too restrictive.
- `T3: NNN clusters` — the count of full-complement clusters.

### Stage 4 — Generate Rankings

Apply the V2 scoring algorithm, deduplication threshold, and ranking pass.

```bash
python3 generate-rankings.py
```

About 20 seconds. Reads `work/clusters.geojson`, applies dedup at 0.15 km, computes scores, assigns tiers (with country-saturation guard), assigns rankings within country, within continent, within tier. Writes back to `work/clusters.geojson`.

The dedup step records suppressed clusters' anchors as `merged_zones` annotations on the survivor; this is the data backing the BentoBox transparency disclosure.

Console summary lines:

- `1172 duplicates removed → 6422 clusters` — dedup count.
- `T3 Apex: NN  T2 Hub: MMM …` — final tier distribution.
- `Score range: 0–730` — sanity check that score_final is well-formed.

### Stage 5 — Build Tiles

Generate the PMTiles and the clusters-meta.json that the live deployment serves.

```bash
python3 build-tiles.py --layer all
```

About four minutes for full rebuild. Three layers:

- **Layer 1** (locations): individual store dots, tippecanoe-built. ~500 MB output. Layer 1 merges three sources: service-business JSONL (retail chains), Overture service-places (hospital, university, airport from `service-fs/service-places/`), and OSM civic data (hospital + university from `service-places/cleansed-civic-osm.jsonl`). All three must be present; a missing civic OSM file produces no error but silently omits ~60,000 hospital and university records from the map.
- **Layer 2** (clusters): cluster bubbles + clusters-meta.json. ~43 MB tile + ~3 MB JSON.
- **Layer 3** (radius): proximity ring shapes. ~100 MB output.

For incremental work, restrict to one or two layers:

```bash
python3 build-tiles.py --layer 2  # cluster meta refresh, ~30s
python3 build-tiles.py --layer 1  # locations refresh, ~3 minutes
```

Output is written directly to the deployment www directory; no separate sync step is needed.

## Verification

After the full rebuild:

```bash
python3 check-chain-counts.py
```

Output shows raw / cleansed counts per chain against the YAML `store_count_approx`. Status flags: OK (within ±20%), OVER (raw above 120%), UNDER (raw below 80%), EMPTY (zero records).

For a live verification, hit the deployment:

```bash
curl -s https://gis.woodfinegroup.com/data/clusters-meta.json | wc -c
```

Should return roughly the byte count of the most recent `clusters-meta.json` build (printed at end of Stage 5). If the live size diverges from the local size, the deployment www directory was not updated — investigate.

## Common Failure Modes

**Stage 4 reports zero clusters.** Stage 2 was skipped or failed. The cleansed JSONL is missing or stale. Re-run Stage 2.

**Stage 5 layer 1 takes 20 minutes instead of 3.** Tippecanoe is processing a corrupted GeoJSON. Inspect `work/layer1-locations.geojson` for empty geometries or NaN coordinates.

**Live URL shows yesterday's data.** The deployment www directory at `/srv/foundry/deployments/gateway-orchestration-gis-1/www/` was not refreshed — check write permissions or re-run Stage 5 with verbose flags.

**`check-chain-counts.py` shows new OVER for a chain that was OK yesterday.** OpenStreetMap may have added cross-border records the polygon filter does not catch. Inspect the JSONL for outlier latitudes/longitudes; tighten the country bbox in `ingest-osm.py` if the bounding box itself is too loose.

## Stage 6 — O-D Study and Catchment Layers (Sprint 14+)

This stage is required when census or spend data has been updated, or when the
catchment radius parameters change. It is independent of Stages 1–5 and can be
run separately.

### Step A — Synthesize O-D Catchment Data

Computes primary (≤35 km) and secondary (35–150 km) catchment populations and
spend for all 6,815 clusters. Also ranks clusters and updates `clusters-meta.json`.

```bash
python3 synthesize-od-study.py
```

About 15–25 minutes. Reads `census-h3-res7.jsonl` (172 MB) and
`cleansed-spend-h3-res7.jsonl` (265 MB) from `service-fs/`. Iterates all
clusters, computing H3 grid disks at resolution 7 (ring k=17 for primary,
k=72 for secondary). Outputs:

- `service-fs/service-mobility/od-summary.jsonl` — B3 artifact; one record per cluster
- `work/catchment-data.json` — aggregated catchment stats + ranks
- Updates `clusters-meta.json` in place with catchment fields

Console lines to watch:
- `[N/6815]` progress markers every 500 clusters
- `Top 5 clusters by combined catchment population` — sanity check at end

Failure modes:
- **h3 not installed**: `pip install h3` in the project Python environment.
- **clusters-meta.json missing catchment fields after run**: Check that the merge
  loop matched cluster IDs correctly; run with a debug print on the first unmatched entry.

### Step B — Build Catchment Polygon Layer

Generates two circular polygons per cluster (primary 35 km, secondary 150 km)
and passes through tippecanoe to produce the map catchment layer.

```bash
python3 build-catchment-polygons.py
tippecanoe -o /srv/foundry/deployments/gateway-orchestration-gis-1/www/tiles/layer3-catchment.pmtiles \
  --force --layer catchment --minimum-zoom 3 --maximum-zoom 10 \
  --drop-densest-as-needed \
  pointsav-monorepo/app-orchestration-gis/work/catchment-polygons.geojson
```

About 1 minute. Outputs `work/catchment-polygons.geojson` (two features per cluster:
`zone=primary` and `zone=secondary`). The MapLibre style uses `match ["get","zone"]`
to colour them distinctly.

### Step C — Build Census and Spend Data Tile Layers

Generates H3 hexagon polygon layers for census and spend, masked to catchment areas
(within 150 km of any cluster), and passes through tippecanoe.

```bash
python3 build-data-tiles.py
```

About 20–30 minutes. Outputs:
- `layer4-census.pmtiles` — population H3 hexes within catchment areas
- `layer5-spend.pmtiles` — spend H3 hexes within catchment areas

Both are written directly to the deployment tiles directory.

Note: Only H3 cells within 150 km of at least one cluster are included. The full
150 km data radius is the ingest boundary, not the display boundary.

## Phase 1 Rebuild — Taxonomy Restructure (Sprint 17, May 2026)

Phase 1 restructured the anchor taxonomy from a two-class system (ALPHA_ANCHORS +
ALPHA_HARDWARE) to a four-class system: ALPHA_HYPERMARKET, ALPHA_LIFESTYLE,
ALPHA_HARDWARE, ALPHA_WAREHOUSE. This change is config-only — no re-ingest is
required. Mercadona (Spain), Tesco (UK), and Sainsbury's (UK) were promoted to
ALPHA_HYPERMARKET in this phase. IKEA moved from ALPHA_ANCHORS to ALPHA_LIFESTYLE.

After a Phase 1 config change, run the standard pipeline from Step 1:

```bash
python3 build-clusters.py
python3 generate-rankings.py
python3 build-tiles.py
```

Phase 1 also adds per-tier civic counts to each cluster record:
`hc_count_regional`, `hc_count_district`, `he_count_regional`, `he_count_small`.
These fields drive the Civic Context cell in the bento inspector.

## Phase 2 Rebuild — Pure-Geometric Ranking Engine (planned Sprint 18)

Phase 2 replaces the V2 score-based ranking with a pure-predicate tier engine.
The new entry point is `build-geometric-ranking.py`, which supersedes
`generate-rankings.py` for tier assignment. The V2 script is retained at
`legacy/generate-rankings-v2.py` as a rollback path.

Phase 2 also extends `synthesize-od-study.py` to rank all eight catchment axes
(primary/secondary × general/grocery/hardware/wholesale) within each ISO market.

After Phase 2 is deployed, the standard rebuild sequence becomes:

```bash
python3 synthesize-od-study.py
python3 build-clusters.py
python3 build-geometric-ranking.py
python3 build-tiles.py
```

`generate-rankings.py` is no longer called in the standard pipeline after Phase 2.
The `score_final` field will not be present in cluster GeoJSON output after Phase 2;
the circle-radius map layer expression must be updated accordingly before deployment.

## See Also

- [Adding a New Chain to the GIS Pipeline](guide-gis-adding-a-chain.md)
- [Adding a Country to the GIS Pipeline](guide-gis-adding-a-country.md)
- [O-D Catchment Methodology](topic-od-catchment-methodology.md)
- [Trade Area Data Sources](topic-trade-area-data-sources.md)
- [Catchment Ranking Methodology](topic-catchment-ranking-methodology.md)
- [Retail Co-location Methodology](topic-co-location-methodology.md)
- [Cluster Deduplication Threshold](topic-cluster-deduplication-threshold.md)
