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

- **Layer 1** (locations): individual store dots, tippecanoe-built. ~400 MB output.
- **Layer 2** (clusters): cluster bubbles + clusters-meta.json. ~40 MB tile + ~3 MB JSON.
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

## See Also

- [Adding a New Chain to the GIS Pipeline](guide-gis-adding-a-chain.md)
- [Adding a Country to the GIS Pipeline](guide-gis-adding-a-country.md)
- [Retail Co-location Methodology](topic-co-location-methodology.md)
- [Cluster Deduplication Threshold](topic-cluster-deduplication-threshold.md)
