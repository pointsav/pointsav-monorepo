#!/usr/bin/env bash
# nightly-rebuild.sh — Full GIS pipeline rebuild (S2 production, 2026-05-22)
# Safe window: 05:00–16:00 UTC (22:00–09:00 Vancouver PDT)
# Do NOT run during business hours — see overnight-builds memory note.
# Log: /tmp/nightly-rebuild-YYYYMMDD.log

set -euo pipefail

DIR=/srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis
LOG=/tmp/nightly-rebuild-$(date +%Y%m%d-%H%M).log

echo "=== nightly-rebuild.sh started at $(date -u) ===" | tee "$LOG"
echo "Log: $LOG"

step() {
    echo "" | tee -a "$LOG"
    echo "--- Step $1: $2 ---" | tee -a "$LOG"
}

# ── STEP 1 ── Two-pass tight-first DBSCAN → clusters.geojson
step 1 "build-clusters.py (two-pass DBSCAN, S2 schema)"
python3 "$DIR/build-clusters.py" 2>&1 | tee -a "$LOG"

# ── STEP 2 ── Geometric compactness rank → dist_rank_in_tier, dist_pctile
step 2 "build-geometric-ranking.py (dist_rank_in_tier)"
python3 "$DIR/build-geometric-ranking.py" 2>&1 | tee -a "$LOG"

# ── STEP 3 ── Demand rank (pass 1 — against existing catchment-data.json)
step 3 "build-demand-ranking.py (pass 1 — interim demand rank)"
python3 "$DIR/build-demand-ranking.py" 2>&1 | tee -a "$LOG"

# ── STEP 4 ── Regional Markets → regional-markets.json
step 4 "build-regional-markets.py"
python3 "$DIR/build-regional-markets.py" 2>&1 | tee -a "$LOG"

# ── STEP 5 ── Layer 2 clusters PMTile + clusters-meta.json
step 5 "build-tiles.py --layer 2 (layer2-clusters + meta)"
python3 "$DIR/build-tiles.py" --layer 2 2>&1 | tee -a "$LOG"

# ── STEP 6 ── Demand rank (pass 2 — synthesize-od-study produces fresh catchment)
# Note: synthesize-od-study is heavy (~2h). Run only if catchment-data.json is stale
# or absent. To force: rm work/catchment-data.json before running this script.
CATCHMENT="$DIR/work/catchment-data.json"
if [ ! -f "$CATCHMENT" ]; then
    step 6 "synthesize-od-study.py (catchment-data.json absent — running)"
    python3 "$DIR/synthesize-od-study.py" 2>&1 | tee -a "$LOG"

    step 7 "build-demand-ranking.py (pass 2 — fresh catchment)"
    python3 "$DIR/build-demand-ranking.py" 2>&1 | tee -a "$LOG"

    step 8 "build-tiles.py --layer 2 (rebuild meta with final demand ranks)"
    python3 "$DIR/build-tiles.py" --layer 2 2>&1 | tee -a "$LOG"
else
    echo "" | tee -a "$LOG"
    echo "--- Steps 6-8 SKIPPED: $CATCHMENT exists (pass-1 demand ranks used) ---" | tee -a "$LOG"
    echo "    To re-run O-D synthesis: rm $CATCHMENT and re-run this script." | tee -a "$LOG"
fi

# ── STEP 9 ── Top-400 index (NA + EU)
step 9 "generate-top400.py"
python3 "$DIR/generate-top400.py" 2>&1 | tee -a "$LOG"

# ── STEP 10 ── Layer 1 (POI details) — optional, skip if binary large
# step 10 "build-tiles.py --layer 1 (layer1-pois)"
# python3 "$DIR/build-tiles.py" --layer 1 2>&1 | tee -a "$LOG"

echo "" | tee -a "$LOG"
echo "=== nightly-rebuild.sh complete at $(date -u) ===" | tee -a "$LOG"
echo "Log saved: $LOG"
