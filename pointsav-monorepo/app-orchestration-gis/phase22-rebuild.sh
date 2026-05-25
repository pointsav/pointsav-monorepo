#!/usr/bin/env bash
# phase22-rebuild.sh — Phase 22 taxonomy revision: Change A + B3 tier rebalancing
#
# Change A: T2 now requires has_hypermarket AND has_hardware (not just has_hyper + n≥2).
# Change B3: T1.a disjunction extended — hypermarket + hardware + (price_club OR lifestyle OR electronics).
#            Electronics clause is load-bearing for EU T1: ~210 additional EU T1 clusters.
# Removes geometric downgrade patch (span < 1.25km and len ≤ 2 → T3) — redundant under Change A.
#
# No new chain ingests in this phase — taxonomy-only rebuild.
#
# Projected distribution:
#   T1: 1,537 → ~1,747 (+210 from electronics clause in EU)
#   T2: 3,090 → ~3,392 (net: -621 from Change A, +723 from removing downgrade, approx)
#   T3: 1,866 → ~1,354
#
# Schedule: overnight window ≥05:00 UTC (after 10pm Vancouver PDT)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG="$SCRIPT_DIR/phase22-rebuild.log"
STAMP="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"

echo "──────────────────────────────────────────────" | tee -a "$LOG"
echo "phase22-rebuild  $STAMP" | tee -a "$LOG"

cd "$SCRIPT_DIR"

# ── pre-flight: verify phase21 completed ─────────────────────────────────────

PHASE21_LOG="$SCRIPT_DIR/phase21-rebuild.log"
if [[ -f "$PHASE21_LOG" ]]; then
    if grep -q "Phase 21 Complete" "$PHASE21_LOG"; then
        echo "[pre-flight] phase21 complete marker found  ✓" | tee -a "$LOG"
    else
        echo "WARNING: phase21-rebuild.log exists but 'Phase 21 Complete' not found — proceeding anyway" | tee -a "$LOG"
    fi
else
    echo "WARNING: phase21-rebuild.log not found — phase21 may not have run" | tee -a "$LOG"
fi

# ── verify taxonomy change is in place ───────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[pre-flight] Verifying taxonomy.py Change A + B3" | tee -a "$LOG"
if grep -q "has_hyper and has_hw and (has_pc or has_life or has_elec)" taxonomy.py; then
    echo "  → B3 electronics clause present  ✓" | tee -a "$LOG"
else
    echo "ERROR: taxonomy.py does not contain the B3 electronics clause — aborting" | tee -a "$LOG"
    exit 1
fi
if grep -q "if has_hyper and has_hw:" taxonomy.py; then
    echo "  → Change A T2 rule present  ✓" | tee -a "$LOG"
else
    echo "ERROR: taxonomy.py does not contain Change A T2 rule — aborting" | tee -a "$LOG"
    exit 1
fi
if grep -q "span < 1.25 and len(members) <= 2" build-clusters.py; then
    echo "ERROR: geometric downgrade patch still present in build-clusters.py — remove it first" | tee -a "$LOG"
    exit 1
else
    echo "  → geometric downgrade removed from build-clusters.py  ✓" | tee -a "$LOG"
fi

# ── rebuild clusters ──────────────────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[1/2] build-clusters.py (Phase 22 — Change A + B3 taxonomy)" | tee -a "$LOG"
python3 build-clusters.py 2>&1 | tee -a "$LOG"

CLUSTERS_OUT="$SCRIPT_DIR/work/clusters.geojson"
if [[ ! -f "$CLUSTERS_OUT" ]]; then
    echo "ERROR: clusters.geojson not produced" | tee -a "$LOG"
    exit 1
fi
CLUSTERS_SIZE=$(du -sh "$CLUSTERS_OUT" | cut -f1)
echo "  → $CLUSTERS_OUT ($CLUSTERS_SIZE)  ✓" | tee -a "$LOG"

# ── spot-check tier distribution ─────────────────────────────────────────────

echo "" | tee -a "$LOG"
echo "[spot-check] Tier distribution from clusters-meta.json" | tee -a "$LOG"
python3 - <<'PYEOF' 2>&1 | tee -a "$LOG"
import json, pathlib
meta = pathlib.Path("/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json")
if not meta.exists():
    meta = pathlib.Path("work/clusters-meta.json")
data = json.loads(meta.read_text())
clusters = data.get("clusters", data) if isinstance(data, dict) else data
t = {1: 0, 2: 0, 3: 0}
for c in clusters:
    t[c.get("t", c.get("tier", 3))] += 1
total = sum(t.values())
print(f"  T1={t[1]} ({t[1]/total*100:.1f}%)  T2={t[2]} ({t[2]/total*100:.1f}%)  T3={t[3]} ({t[3]/total*100:.1f}%)  total={total}")
# Sanity: T1 should be ≥1,700 and T2 should be ≤3,500
if t[1] >= 1700:
    print(f"  T1 ≥1700  ✓")
else:
    print(f"  WARNING: T1={t[1]} below expected ~1,747")
if t[2] <= 3500:
    print(f"  T2 ≤3500  ✓")
else:
    print(f"  WARNING: T2={t[2]} above expected ceiling")
PYEOF

# ── rebuild tiles ─────────────────────────────────────────────────────────────

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
echo "── Phase 22 Complete: $(date -u '+%Y-%m-%dT%H:%M:%SZ') ──" | tee -a "$LOG"
