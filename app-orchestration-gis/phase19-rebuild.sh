#!/usr/bin/env bash
# phase19-rebuild.sh — Phase 19 sport ingest + full cluster/tile rebuild
#
# Runs automatically after nightly-rebuild.sh completes.
# Ingests all 16 sport chains, then rebuilds clusters + tiles (layer 2).
#
# Sport chains: Decathlon (EU 12 countries + CA) + REI + Bass Pro + Cabela's (US)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG="$SCRIPT_DIR/phase19-rebuild.log"
STAMP="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"

echo "──────────────────────────────────────────────" | tee -a "$LOG"
echo "phase19-rebuild  $STAMP" | tee -a "$LOG"

cd "$SCRIPT_DIR"

# ── ingest: Decathlon EU ──────────────────────────────────────────────────────

EU_DECATHLON=(
    decathlon-fr decathlon-de decathlon-gb decathlon-es decathlon-it
    decathlon-nl decathlon-pl decathlon-pt decathlon-se decathlon-dk
    decathlon-no decathlon-fi
)

echo "" | tee -a "$LOG"
echo "[ingest] Decathlon EU (12 countries)" | tee -a "$LOG"
for chain in "${EU_DECATHLON[@]}"; do
    echo "  → $chain" | tee -a "$LOG"
    python3 ingest-osm.py --chain "$chain" 2>&1 | tee -a "$LOG"
done

# ── ingest: Decathlon CA ─────────────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[ingest] Decathlon CA" | tee -a "$LOG"
python3 ingest-osm.py --chain decathlon-ca 2>&1 | tee -a "$LOG"

# ── ingest: US sport chains ───────────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[ingest] US sport (REI + Bass Pro Shops + Cabela's)" | tee -a "$LOG"
for chain in rei-us bass-pro-shops-us cabelas-us; do
    echo "  → $chain" | tee -a "$LOG"
    python3 ingest-osm.py --chain "$chain" 2>&1 | tee -a "$LOG"
done

# ── rebuild clusters + tiles ──────────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[1/2] build-clusters.py (Phase 19 — 5 categories)" | tee -a "$LOG"
python3 build-clusters.py 2>&1 | tee -a "$LOG"

CLUSTERS_OUT="$SCRIPT_DIR/work/clusters.geojson"
if [[ ! -f "$CLUSTERS_OUT" ]]; then
    echo "ERROR: clusters.geojson not produced" | tee -a "$LOG"
    exit 1
fi

echo "" | tee -a "$LOG"
echo "[2/2] build-tiles.py --layer 2" | tee -a "$LOG"
python3 build-tiles.py --layer 2 2>&1 | tee -a "$LOG"

TILES_OUT="/srv/foundry/deployments/gateway-orchestration-gis-1/www/tiles/layer2-clusters.pmtiles"
META_OUT="/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"

if [[ ! -f "$TILES_OUT" ]]; then
    echo "ERROR: layer2-clusters.pmtiles not produced" | tee -a "$LOG"
    exit 1
fi

TILES_SIZE=$(du -sh "$TILES_OUT" | cut -f1)
META_SIZE=$(du -sh "$META_OUT" | cut -f1)
echo "  → $TILES_OUT ($TILES_SIZE)  ✓" | tee -a "$LOG"
echo "  → $META_OUT ($META_SIZE)  ✓" | tee -a "$LOG"

echo "" | tee -a "$LOG"
echo "── Phase 19 Complete: $(date -u '+%Y-%m-%dT%H:%M:%SZ') ──" | tee -a "$LOG"
