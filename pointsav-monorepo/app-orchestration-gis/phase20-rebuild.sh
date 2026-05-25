#!/usr/bin/env bash
# phase20-rebuild.sh — Phase 20 chain ingest + H2b taxonomy + full cluster/tile rebuild
#
# Runs automatically after phase19-rebuild.sh completes (schedule at 05:00 UTC 2026-05-25,
# but only start if phase19 log shows completion first).
#
# New chains (9): OBI-AT, Bauhaus-AT, Decathlon-AT/GR/MX, Auchan-PT, XXL NO/SE/FI
# Taxonomy changes: H2b rule (tight 3-anchor → T1), Makro demoted from price_club
#
# Rebuilds: build-clusters.py → clusters.geojson → build-tiles.py --layer 2

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG="$SCRIPT_DIR/phase20-rebuild.log"
STAMP="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"

echo "──────────────────────────────────────────────" | tee -a "$LOG"
echo "phase20-rebuild  $STAMP" | tee -a "$LOG"

cd "$SCRIPT_DIR"

# ── pre-flight: verify phase19 completed ─────────────────────────────────────

PHASE19_LOG="$SCRIPT_DIR/phase19-rebuild.log"
if [[ -f "$PHASE19_LOG" ]]; then
    if grep -q "Phase 19 Complete" "$PHASE19_LOG"; then
        echo "[pre-flight] phase19 complete marker found  ✓" | tee -a "$LOG"
    else
        echo "WARNING: phase19-rebuild.log exists but 'Phase 19 Complete' not found — proceeding anyway" | tee -a "$LOG"
    fi
else
    echo "WARNING: phase19-rebuild.log not found — phase19 may not have run" | tee -a "$LOG"
fi

# ── ingest: AT hardware ──────────────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[ingest] Austria hardware (OBI-AT, Bauhaus-AT)" | tee -a "$LOG"
for chain in obi-at bauhaus-at; do
    echo "  → $chain" | tee -a "$LOG"
    python3 ingest-osm.py --chain "$chain" 2>&1 | tee -a "$LOG"
done

# ── ingest: AT/GR/MX sport ──────────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[ingest] Decathlon AT/GR/MX" | tee -a "$LOG"
for chain in decathlon-at decathlon-gr decathlon-mx; do
    echo "  → $chain" | tee -a "$LOG"
    python3 ingest-osm.py --chain "$chain" 2>&1 | tee -a "$LOG"
done

# ── ingest: PT hypermarket ───────────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[ingest] Auchan Portugal" | tee -a "$LOG"
python3 ingest-osm.py --chain auchan-pt 2>&1 | tee -a "$LOG"

# ── ingest: Scandinavian XXL ─────────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[ingest] XXL Sport NO/SE/FI" | tee -a "$LOG"
for chain in xxl-no xxl-se xxl-fi; do
    echo "  → $chain" | tee -a "$LOG"
    python3 ingest-osm.py --chain "$chain" 2>&1 | tee -a "$LOG"
done

# ── rebuild clusters + tiles ──────────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[1/2] build-clusters.py (Phase 20 — H2b rule active)" | tee -a "$LOG"
python3 build-clusters.py 2>&1 | tee -a "$LOG"

CLUSTERS_OUT="$SCRIPT_DIR/work/clusters.geojson"
if [[ ! -f "$CLUSTERS_OUT" ]]; then
    echo "ERROR: clusters.geojson not produced" | tee -a "$LOG"
    exit 1
fi
CLUSTERS_SIZE=$(du -sh "$CLUSTERS_OUT" | cut -f1)
echo "  → $CLUSTERS_OUT ($CLUSTERS_SIZE)  ✓" | tee -a "$LOG"

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

echo "" | tee -a "$LOG"
echo "── Phase 20 Complete: $(date -u '+%Y-%m-%dT%H:%M:%SZ') ──" | tee -a "$LOG"
