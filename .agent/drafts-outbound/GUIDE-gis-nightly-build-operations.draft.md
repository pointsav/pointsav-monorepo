---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: PROSE-GUIDE
title: "GIS Nightly Rebuild Operations"
slug: guide-gis-nightly-build-operations
language: en
status: draft
target_repo: woodfine-fleet-deployment
target_path: gateway-orchestration-gis-1/guide-gis-nightly-build-operations.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 4
research_suggested_count: 0
open_questions_count: 0
research_provenance: "nightly-rebuild.sh (full script); build-vwh-clusters.py comment headers; build-pks-clusters.py comment headers; config.py; feedback-overnight-builds memory; overnight-builds policy; 2026-06-12 decontamination session (deploy-guard, flock, crontab fix)"
research_inline: true
created: 2026-06-11
updated: 2026-06-13
author: totebox@project-gis (claude-sonnet-4-6)
---

# GIS Nightly Rebuild Operations

This guide covers the scheduled nightly cluster rebuild for the Location Intelligence
platform. The rebuild refreshes co-location cluster geometries and deploys updated
map tiles and metadata to `gis.woodfinegroup.com`.

## Overview

The nightly rebuild is a two-step pipeline:

1. **`build-clusters.py`** — re-runs the co-location algorithm against current chain JSONL
   and produces `work/clusters.geojson`
2. **`build-tiles.py --layer 2`** — converts clusters to PMTiles and writes
   `clusters-meta.json` for the BentoBox inspector

After the cluster rebuild, two archetype pipelines run as non-fatal steps:

- **`build-vwh-clusters.py`** (VWH — Urban Fringe, ~60 s) — refreshes
  `archetype-vwh.geojson`
- **`build-pks-clusters.py`** (PKS — Commuter, ~90 s) — refreshes
  `archetype-pks.geojson`

All scripts run from
`/srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/`.

## Schedule

The rebuild runs at **10:00 pm Vancouver PDT** (05:00 UTC, 9:00 pm PST).

Check the active crontab:

```bash
crontab -l | grep nightly-rebuild
```

Expected output:
```
0 22 * * * cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis && bash nightly-rebuild.sh >> /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/nightly-rebuild.log 2>&1
```

The AEC enrichment job runs one hour later on Mondays (11:00 pm PDT = 06:00 UTC):

```
0 23 * * 1 cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis && bash build-aec-global.sh >> .../aec-global.log 2>&1
```

## Pre-flight checks

Before running the rebuild manually, confirm:

```bash
# 1. Disk: must have ≥5 GB free on /
df -BG / | awk 'NR==2 {print $4, "free"}'

# 2. Taxonomy integrity: taxonomy.py must be ≥400 lines
wc -l /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/taxonomy.py

# 3. No prior run in progress (flock prevents concurrent runs automatically)
pgrep -af "build-clusters.py|build-tiles.py" && echo "RUNNING" || echo "clear"

# 4. Verify this archive is the declared gateway owner
cat /srv/foundry/deployments/gateway-orchestration-gis-1/.owner  # must print: project-gis
```

The script performs these checks automatically on startup and exits non-zero on
failure before touching any output files.

A full dry-run (pre-flight only, no build):

```bash
bash nightly-rebuild.sh --dry-run
```

## Running the rebuild

**Scheduled run (no action required):** cron handles the nightly execution.

**Manual full rebuild:**

```bash
cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis
bash nightly-rebuild.sh
```

**Detached overnight rebuild** (if starting from an interactive session, ensures the
job survives session close):

```bash
cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis
nohup bash nightly-rebuild.sh >> nightly-rebuild.log 2>&1 &
echo "PID $! — tail -f nightly-rebuild.log to monitor"
```

Note: use `>>` (append) not `>` (truncate) — the log is shared with the cron run.

## Output verification

After a successful run, confirm all four output files are present and recently
modified:

```bash
WWW=/srv/foundry/deployments/gateway-orchestration-gis-1/www
ls -lh "$WWW/tiles/layer2-clusters.pmtiles" \
        "$WWW/data/clusters-meta.json" \
        "$WWW/data/archetype-vwh.geojson" \
        "$WWW/data/archetype-pks.geojson"
```

Check tier counts in `clusters-meta.json` (file is an array of cluster records):

```bash
python3 -c "
import json, collections
d = json.load(open('$WWW/data/clusters-meta.json'))
tc = collections.Counter(r['t'] for r in d)
print(f'Total: {len(d)}  T1={tc[1]}  T2={tc[2]}  T3={tc[3]}')
has_aec = sum(1 for r in d if 'koppen_class' in r)
print(f'AEC fields: {has_aec}/{len(d)}')
"
```

Expected baseline (Phase 23+Change B, 2026-05-28):

| Tier | Count |
|------|-------|
| T1   | 1,746 |
| T2   | 2,726 |
| T3   | 2,021 |
| **Total** | **6,493** |

A count significantly below baseline indicates a build failure or input data
regression. Do not deploy if the total falls below 5,500.

## Deployment

The rebuild writes directly to the deployment directory — no separate deploy step
is required. The gateway is a static nginx server; file replacement is atomic from
nginx's perspective. No service restart is needed.

Verify the map reflects the update by hard-refreshing `gis.woodfinegroup.com` and
checking the cluster count in the browser console:

```javascript
// In browser console at gis.woodfinegroup.com
fetch('/data/clusters-meta.json').then(r=>r.json()).then(d=>console.log(d.cluster_count))
```

## Log monitoring

```bash
# Follow a running rebuild in real time
tail -f /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/nightly-rebuild.log

# Check the last run result (look for "Complete" marker)
tail -20 /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/nightly-rebuild.log
```

A successful run ends with:
```
── Complete: 2026-06-11T05:14:22Z ──
```

A failed run exits with a `ERROR:` line. Partial outputs remain in `work/` for
debugging — they do not overwrite the deployed files unless the tile-build step
also succeeds.

## Known large-build timing

The following scripts are too resource-intensive to run during working hours.
Schedule all of them after 10:00 pm Vancouver time:

| Script | Typical duration | Peak RAM | Notes |
|--------|-----------------|----------|-------|
| `build-tiles.py` (layer 1) | 2–4 hours | ~40 % | 324 K features, tippecanoe layer 1/2 |
| `build-tiles.py` (layer 4/5) | 2–3 hours | ~40 % | Census / spend overlays |
| `build-mobility-tiles.py` | 1–2 hours | ~46 % | tippecanoe layer 6/7 |
| `synthesize-od-study.py` | 30–60 min | ~30 % | 13,657-cluster H3 computation |
| `ingest-lodes.py` | 4–8 hours | ~30 % | Multi-state LODES download + processing |
| `ingest-kontur.py` | 30–60 min | moderate | GeoPackage decompression |
| `build-catchment-polygons.py` | 30–60 min | ~30 % | Large GeoJSON generation |

The nightly rebuild itself (build-clusters + build-tiles layer 2 + VWH + PKS)
typically completes in 20–40 minutes and stays well within the RAM headroom.

## Gateway ownership and deploy-guard

The nightly rebuild writes directly to `gateway-orchestration-gis-1`. Only one archive
is permitted to write to this gateway at a time. The permitted archive is declared in:

```
/srv/foundry/deployments/gateway-orchestration-gis-1/.owner   (contains: project-gis)
```

`nightly-rebuild.sh` checks this at startup. If the script is run from any other archive,
it logs a DEPLOY-GUARD error and exits with code 78 — the gateway is never touched:

```
DEPLOY-GUARD: project-orgcharts is not declared owner of gateway-orchestration-gis-1 — aborting
```

A concurrency lock (`gateway-orchestration-gis-1/.rebuild.lock`) prevents `nightly-rebuild.sh`
and `build-aec-global.sh` from running simultaneously. The second script to start will log
`"another rebuild already running — aborting"` and exit cleanly.

## AEC enrichment fields

`clusters-meta.json` carries additional atmospheric, environmental, and climatic fields
(`koppen_class`, `ecoregion_name`, `ghi_kwh_m2_yr`, `seismic_pga_g`, etc.) that are written
by the weekly `build-aec-global.sh` job. The nightly rebuild preserves these fields using a
coordinate-based merge — any cluster whose centroid is within ~300 m of a known AEC record
carries its fields forward.

If AEC fields are absent (shown as `AEC fields: 0/6493` in the verification command), run the
backfill manually:

```bash
cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis
nohup bash build-aec-global.sh >> aec-global.log 2>&1 &
```

This takes approximately 2 hours and requires the following source files in `work/aec/`:
- `koppen-simplified.geojson` (255 MB)
- `ecoregions-global.geojson` (631 MB)
- `gwl-fcs30-global.tif` (15 MB)

## Script dependencies

All scripts require the following files in the same directory:

| File | Role |
|------|------|
| `config.py` | Paths (WORK_DIR, TILES_DIR, WWW_DIR), chain families, REGION_CONFIG |
| `taxonomy.py` | Chain categorisation, tier logic, display names |
| `utils/region_engine.py` | Boundary polygon lookup for market assignment |
| `utils/spatial_filter.py` | Spatial filtering utilities |

These files must be present in `pointsav-monorepo/app-orchestration-gis/`. If any are missing,
copy from the orgcharts archive (`project-orgcharts/pointsav-monorepo/app-orchestration-gis/`).

## Failure recovery

If the nightly rebuild fails:

1. Check the log for the specific `ERROR:` line.
2. Inspect `work/clusters.geojson` — if present, the cluster step succeeded and
   only the tile step failed. Re-run `build-tiles.py --layer 2` directly.
3. If `work/clusters.geojson` is absent, the cluster build failed. Check for
   taxonomy import errors (`taxonomy.py` truncation) or chain JSONL corruption.
4. For archetype failures (VWH or PKS), these steps are non-fatal — the main
   cluster tiles are unaffected. Re-run the individual script when convenient.
