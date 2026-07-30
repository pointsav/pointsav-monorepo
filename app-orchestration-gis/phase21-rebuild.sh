#!/usr/bin/env bash
# phase21-rebuild.sh — Phase 21 electronics category + XXXLutz/Höffner lifestyle + full cluster/tile rebuild
#
# Runs automatically after phase20-rebuild.sh completes (schedule 2026-05-26, 05:00 UTC).
#
# New chains (16):
#   Electronics (11): mediamarkt-de, saturn-de, mediamarkt-at, mediamarkt-nl, mediamarkt-es,
#                     mediaworld-it, mediamarkt-gr, mediamarkt-pl, mediamarkt-se,
#                     boulanger-fr, darty-fr
#   Lifestyle (5):    xxxlutz-at, xxxlutz-de, xxxlutz-se, xxxlutz-fr, hoeffner-de
#
# Taxonomy changes: 6th anchor category "electronics" added to _RETAIL_CATS;
#                   XXXLutz added to lifestyle AT/DE/SE/FR; Höffner added to lifestyle DE
#
# Rebuilds: build-clusters.py → clusters.geojson → build-tiles.py --layer 2

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG="$SCRIPT_DIR/phase21-rebuild.log"
STAMP="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"

echo "──────────────────────────────────────────────" | tee -a "$LOG"
echo "phase21-rebuild  $STAMP" | tee -a "$LOG"

cd "$SCRIPT_DIR"

# ── pre-flight: verify phase20 completed ─────────────────────────────────────

PHASE20_LOG="$SCRIPT_DIR/phase20-rebuild.log"
if [[ -f "$PHASE20_LOG" ]]; then
    if grep -q "Phase 20 Complete" "$PHASE20_LOG"; then
        echo "[pre-flight] phase20 complete marker found  ✓" | tee -a "$LOG"
    else
        echo "WARNING: phase20-rebuild.log exists but 'Phase 20 Complete' not found — proceeding anyway" | tee -a "$LOG"
    fi
else
    echo "WARNING: phase20-rebuild.log not found — phase20 may not have run" | tee -a "$LOG"
fi

# ── ingest: electronics — MediaMarkt/Saturn DE ───────────────────────────────

echo "" | tee -a "$LOG"
echo "[ingest] Electronics DE (MediaMarkt, Saturn)" | tee -a "$LOG"
for chain in mediamarkt-de saturn-de; do
    echo "  → $chain" | tee -a "$LOG"
    python3 ingest-osm.py --chain "$chain" 2>&1 | tee -a "$LOG"
done

# ── ingest: electronics — MediaMarkt AT/NL/ES/GR/PL/SE ──────────────────────

echo "" | tee -a "$LOG"
echo "[ingest] Electronics AT/NL/ES/GR/PL/SE (MediaMarkt)" | tee -a "$LOG"
for chain in mediamarkt-at mediamarkt-nl mediamarkt-es mediamarkt-gr mediamarkt-pl mediamarkt-se; do
    echo "  → $chain" | tee -a "$LOG"
    python3 ingest-osm.py --chain "$chain" 2>&1 | tee -a "$LOG"
done

# ── ingest: electronics — MediaWorld IT ──────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[ingest] Electronics IT (MediaWorld)" | tee -a "$LOG"
python3 ingest-osm.py --chain mediaworld-it 2>&1 | tee -a "$LOG"

# ── ingest: electronics — Boulanger + Darty FR ───────────────────────────────

echo "" | tee -a "$LOG"
echo "[ingest] Electronics FR (Boulanger, Darty)" | tee -a "$LOG"
for chain in boulanger-fr darty-fr; do
    echo "  → $chain" | tee -a "$LOG"
    python3 ingest-osm.py --chain "$chain" 2>&1 | tee -a "$LOG"
done

# ── ingest: lifestyle — XXXLutz AT/DE/SE/FR + Höffner DE ─────────────────────

echo "" | tee -a "$LOG"
echo "[ingest] Lifestyle additions (XXXLutz AT/DE/SE/FR, Höffner DE)" | tee -a "$LOG"
for chain in xxxlutz-at xxxlutz-de xxxlutz-se xxxlutz-fr hoeffner-de; do
    echo "  → $chain" | tee -a "$LOG"
    python3 ingest-osm.py --chain "$chain" 2>&1 | tee -a "$LOG"
done

# ── rebuild clusters + tiles ──────────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[1/2] build-clusters.py (Phase 21 — electronics category active)" | tee -a "$LOG"
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
echo "── Phase 21 Complete: $(date -u '+%Y-%m-%dT%H:%M:%SZ') ──" | tee -a "$LOG"
