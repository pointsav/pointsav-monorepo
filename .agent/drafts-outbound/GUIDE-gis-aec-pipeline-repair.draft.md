---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: PROSE-GUIDE
title: "AEC Hazard Pipeline Repair"
slug: guide-gis-aec-pipeline-repair
language: en
status: draft
target_repo: woodfine-fleet-deployment
target_path: gateway-orchestration-gis-1/guide-gis-aec-pipeline-repair.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 5
research_suggested_count: 0
open_questions_count: 0
research_provenance: "build-aec-flood.sh (script header + pre-flight logic); build-aec-seismic.sh (script header); build-aec-global.sh (script header); build-aec-flood.log (failure pattern review); phase17-session-state memory (AEC nights 4/5 history); NEXT.md Session 71 (URL fix bd17a348 for seismic)"
research_inline: true
created: 2026-06-11
author: totebox@project-gis (claude-sonnet-4-6)
---

# AEC Hazard Pipeline Repair

This guide covers diagnosing and repairing failures in the Atmospheric,
Environmental, and Climatic (AEC) hazard data pipeline. The pipeline enriches
cluster records with flood, wildfire, seismic, and climate-zone data through
a five-night staged rollout, plus a weekly global build.

## Pipeline overview

The AEC pipeline runs in five sequential nights. Each night is independent and
can be re-run without repeating earlier nights.

| Night | Script | Data layers produced |
|-------|--------|----------------------|
| Night 1 | `build-aec-global.sh` | Köppen climate zones, biome polygons |
| Night 2 | `build-aec-global.sh` | Ecoregion boundaries (WWF Terrestrial) |
| Night 3 | `build-aec-global.sh` | Wetland extent (GWL_FCS30), solar GHI |
| Night 4 | `build-aec-seismic.sh` | Seismic hazard zones (ESHM20 / USGS) |
| Night 5 | `build-aec-flood.sh` | Flood hazard (WRI AQUEDUCT 3.0 + FEMA NFHL), wildfire risk (GWIS) |

All scripts reside in
`/srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/`.
Log files are written to the same directory.

### Weekly global rebuild

`build-aec-global.sh` runs the full Nights 1–3 sequence in a single pass.
It is intended for weekly execution when upstream data sources publish updates,
not nightly.

## Checking pipeline state

**Marker files** indicate which nights completed successfully:

```bash
WORK=/srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/work/aec
ls -la "$WORK/.night"*-complete 2>/dev/null || echo "no complete markers found"
```

Expected after a full five-night run:
```
-rw-r--r-- 1 mathew foundry 0 Jun 10 06:14 .night4-complete
-rw-r--r-- 1 mathew foundry 0 Jun 10 06:14 .night5-complete
```

**Skip flags** suppress re-download of slow external data sources when a
prior attempt was interrupted mid-download:

```bash
ls -la "$WORK/.aqueduct.skip" "$WORK/.fema.skip" 2>/dev/null || echo "no skip flags"
```

Skip flags are self-healing: the scripts remove them on a clean run. If a skip
flag is stale (present after the corresponding download succeeded), remove it
manually.

**Log locations:**

```bash
SCRIPTS=/srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis
ls -lh "$SCRIPTS"/build-aec*.log "$SCRIPTS"/aec-*.log 2>/dev/null
```

## Diagnosing a failed night

1. Identify which log contains the failure:

   ```bash
   grep -l "ERROR\|WARN\|exit" \
     /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/build-aec*.log
   ```

2. Read the tail of the relevant log:

   ```bash
   tail -50 build-aec-flood.log
   ```

3. Look for:
   - `ERROR:` lines — hard failures that stopped the script
   - `WARN:` lines — skipped non-fatal steps (GWL_FCS30 tiles, EU INSPIRE sources)
   - Exit code: `echo $?` immediately after a run; non-zero means failure

## Pre-flight requirements

All AEC scripts check these before running:

| Requirement | Night 4 | Night 5 | Check |
|-------------|---------|---------|-------|
| Disk free ≥ 5 GB | ✓ | — | `df -BG /srv/foundry` |
| Disk free ≥ 10 GB | — | ✓ | `df -BG /srv/foundry` |
| `ogr2ogr` (GDAL ≥ 3.8) | ✓ | ✓ | `ogr2ogr --version` |
| `tippecanoe` | ✓ | ✓ | `tippecanoe --version` |
| `python3`, `curl`, `unzip`, `jq` | ✓ | ✓ | `command -v <tool>` |
| `7z` (p7zip-full) | — | ✓ | `command -v 7z` |

Night 5 (flood/wildfire) is the most resource-intensive: peak transient disk
usage reaches ~2 GB (AQUEDUCT GeoTIFF + FEMA/EU GeoPackages + GWIS NetCDFs).

## Night-by-night repair procedures

### Nights 1–3 (Köppen / ecoregions / wetland / solar)

Re-run the weekly global build:

```bash
cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis
bash build-aec-global.sh
```

GWL_FCS30 tile downloads occasionally fail with HTTP 429 or connection resets.
These are non-fatal — the script logs a `WARN: skipping GWL_FCS30 tile` and
continues. The wetland layer will be absent for affected clusters.

### Night 4 (seismic — ESHM20 + USGS)

Re-run:

```bash
bash build-aec-seismic.sh
```

**Known issue — EU seismic 0 assignments (root cause: ESHM20 API URL):**
The ESHM20 (European Seismic Hazard Model 2020) REST endpoint URL changed in
early 2026. Commit `bd17a348` (2026-05-31) contains the correct URL. If the
EU cluster seismic join produces 0 assignments, verify the endpoint in
`build-aec-seismic.sh`:

```bash
grep "ESHM20\|eshm\|seismicportal" build-aec-seismic.sh | head -5
```

The current correct base URL is `https://www.seismicportal.eu/`. If the script
uses a stale URL, apply the fix from commit `bd17a348` manually.

NA seismic data (USGS shakemap) is unaffected by this issue.

### Night 5 (flood + wildfire)

Re-run:

```bash
bash build-aec-flood.sh
```

**AQUEDUCT download failure:**
WRI AQUEDUCT 3.0 data (global riverine flood, ~90 MB GeoTIFF) is fetched via
a direct URL. If the download fails (HTTP timeout, 5xx, or DNS failure):

1. Check whether a stale `.aqueduct.skip` flag exists and remove it if the
   download actually needs to re-run:

   ```bash
   rm -f work/aec/.aqueduct.skip
   ```

2. Re-run `build-aec-flood.sh`. The script will retry the download.

3. If the WRI endpoint is persistently unavailable, set the skip flag manually
   to bypass the download and proceed with FEMA + wildfire layers only:

   ```bash
   touch work/aec/.aqueduct.skip
   bash build-aec-flood.sh
   ```

**FEMA NFHL REST query timeout:**
The FEMA National Flood Hazard Layer is fetched via a market-windowed REST
query (not the 30 GB state GDB download). Occasional timeout errors are
non-fatal — the affected market cells are skipped. Re-running the script
picks up any missing cells from a clean state.

**GWL_FCS30 tile failures:**
Global wetland tile downloads from the GWL_FCS30 dataset occasionally fail
(HTTP 429 or connection reset). These failures are logged as warnings and
skipped — not build-blocking. The wetland layer is absent for affected
clusters until the next successful run.

**EU regulatory flood zones (INSPIRE WFS):**
The GB, FR, ES, IT, and DE flood zone sources are fetched via INSPIRE WFS
endpoints. Failures in any single country are non-fatal; the affected
country's clusters have no EU flood zone assignment for that run.

## Known failure modes

| Failure | Severity | Action |
|---------|----------|--------|
| GWL_FCS30 tile HTTP 429 | Non-fatal | Logged as WARN; retry next run |
| EU seismic 0 assignments | Data gap | Check ESHM20 URL (fix in `bd17a348`) |
| AQUEDUCT download timeout | Skippable | `rm .aqueduct.skip`; re-run; or use skip flag |
| FEMA REST timeout (partial) | Non-fatal | Re-run; partial cells recovered |
| Disk < 10 GB (Night 5) | Hard stop | Free disk before re-running |

## Full rebuild from scratch

When upstream data sources publish major updates, or after a clean VM
provision, run the full five-night sequence in order:

```bash
cd /srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis

# Remove stale markers
rm -f work/aec/.night4-complete work/aec/.night5-complete

# Nights 1–3 (Köppen, ecoregions, wetland, solar)
bash build-aec-global.sh

# Night 4 (seismic)
bash build-aec-seismic.sh

# Night 5 (flood + wildfire)
bash build-aec-flood.sh
```

Allow 4–8 hours total. Run after 10:00 pm Vancouver time to avoid peak
working-hours resource contention.

## Output verification

After a completed Night 5 run, verify the hazard tile files are present:

```bash
TILES=/srv/foundry/deployments/gateway-orchestration-gis-1/www/tiles
ls -lh "$TILES"/layer11-flood-global.pmtiles \
        "$TILES"/layer12-fema-sfha-us.pmtiles \
        "$TILES"/layer12-flood-eu-regulatory.pmtiles \
        "$TILES"/layer15-wildfire-global.pmtiles
```

Check that `clusters-meta.json` includes `flood_hazard` and `wildfire_hazard`
fields:

```bash
jq 'to_entries | map(select(.key | test("hazard"))) | .[0:4]' \
   /srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json
```
