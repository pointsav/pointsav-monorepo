#!/usr/bin/env bash
# nightly-rebuild.sh — Phase 2 cluster + tile rebuild
#
# Run at 05:00 UTC (10pm Vancouver PDT) per overnight-builds policy.
# Usage:  bash nightly-rebuild.sh [--dry-run]
#
# Rebuilds:
#   1. build-clusters.py   → work/clusters.geojson
#   2. build-tiles.py --layer 2  → layer2-clusters.pmtiles + clusters-meta.json
#
# Does NOT rebuild:
#   layer1 (all-locations) — chain JSONL unchanged
#   layer3 (catchment/radius) — separate pipeline
#   layer4-7 (census/spend/mobility) — separate pipeline
#
# On success: prints tier counts and file sizes.
# On failure: exits non-zero; partial outputs left in work/ for debugging.

set -euo pipefail

DRY_RUN=0
[[ "${1:-}" == "--dry-run" ]] && DRY_RUN=1

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG="$SCRIPT_DIR/nightly-rebuild.log"
STAMP="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"

echo "──────────────────────────────────────────────" | tee -a "$LOG"
echo "nightly-rebuild  $STAMP" | tee -a "$LOG"

# ── pre-flight checks ──────────────────────────────────────────────────────

DISK_AVAIL=$(df -BG / | awk 'NR==2 {print $4}' | tr -d 'G')
if (( DISK_AVAIL < 5 )); then
    echo "ERROR: only ${DISK_AVAIL}G free on /  — aborting to prevent ENOSPC" | tee -a "$LOG"
    exit 1
fi
echo "Disk free: ${DISK_AVAIL}G  ✓" | tee -a "$LOG"

if ! python3 -c "import taxonomy" 2>/dev/null; then
    # try from script dir
    cd "$SCRIPT_DIR"
fi

# Verify taxonomy.py is intact (guard against truncation incident)
PY_LINES=$(wc -l < "$SCRIPT_DIR/taxonomy.py")
if (( PY_LINES < 400 )); then
    echo "ERROR: taxonomy.py looks truncated ($PY_LINES lines) — aborting" | tee -a "$LOG"
    exit 1
fi
echo "taxonomy.py: $PY_LINES lines  ✓" | tee -a "$LOG"

if [[ $DRY_RUN -eq 1 ]]; then
    echo "DRY RUN — pre-flight passed, not executing build steps" | tee -a "$LOG"
    exit 0
fi

# ── step 1 — build-clusters.py ────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[1/2] build-clusters.py" | tee -a "$LOG"
cd "$SCRIPT_DIR"
python3 build-clusters.py 2>&1 | tee -a "$LOG"

CLUSTERS_OUT="$SCRIPT_DIR/work/clusters.geojson"
if [[ ! -f "$CLUSTERS_OUT" ]]; then
    echo "ERROR: clusters.geojson not produced" | tee -a "$LOG"
    exit 1
fi
CLUSTERS_SIZE=$(du -sh "$CLUSTERS_OUT" | cut -f1)
echo "  → $CLUSTERS_OUT ($CLUSTERS_SIZE)  ✓" | tee -a "$LOG"

# ── step 2 — build-tiles.py --layer 2 ────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[2/2] build-tiles.py --layer 2" | tee -a "$LOG"
python3 build-tiles.py --layer 2 2>&1 | tee -a "$LOG"

TILES_OUT="/srv/foundry/deployments/gateway-orchestration-gis-1/www/tiles/layer2-clusters.pmtiles"
META_OUT="/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"

if [[ ! -f "$TILES_OUT" ]]; then
    echo "ERROR: layer2-clusters.pmtiles not produced" | tee -a "$LOG"
    exit 1
fi
if [[ ! -f "$META_OUT" ]]; then
    echo "ERROR: clusters-meta.json not produced" | tee -a "$LOG"
    exit 1
fi

TILES_SIZE=$(du -sh "$TILES_OUT" | cut -f1)
META_SIZE=$(du -sh "$META_OUT" | cut -f1)
echo "  → $TILES_OUT ($TILES_SIZE)  ✓" | tee -a "$LOG"
echo "  → $META_OUT ($META_SIZE)  ✓" | tee -a "$LOG"

# ── summary ───────────────────────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "── Complete: $(date -u '+%Y-%m-%dT%H:%M:%SZ') ──" | tee -a "$LOG"
