#!/usr/bin/env bash
# run-enrichment-ingest.sh — VWH enrichment-coverage ingest + rebuild (Phase 2)
#
# Run overnight (after 05:00 UTC / 10pm Vancouver) per Overnight Builds Policy —
# heavy Overpass queries get rate-limited on shared GCP outbound IPs in daytime.
#
# What it does:
#   1. Ingest all enrichment chain YAMLs that have no JSONL yet (--all-pending).
#      New Phase 2 chains: autozone-mx, comex-mx, napa-ca, partsource-ca,
#      princess-auto-ca, norauto-fr, atu-de, feuvert-fr, euromaster-eu.
#   2. Rebuild clusters (build-clusters.py runs enrich_with_vwh → attaches the new
#      enrichment stores to clusters as members; raises vwh_strength → more T1/T2).
#   3. Re-run the archetype tiering (test-cluster-archetypes.py) → regenerate
#      archetype-vwh.geojson + archetype-pks.geojson with updated tiers.
#   4. Deploy the GeoJSONs to the gateway.
#
# Usage:  bash run-enrichment-ingest.sh
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"
LOG="enrichment-ingest.log"
GW=/srv/foundry/deployments/gateway-orchestration-gis-1/www/data
STAMP() { date -u '+%Y-%m-%dT%H:%M:%SZ'; }

echo "──── enrichment ingest $(STAMP) ────" | tee -a "$LOG"

# 1. Ingest pending enrichment chains (polite delay for Overpass)
echo "[$(STAMP)] [1/4] ingest-osm.py --all-pending" | tee -a "$LOG"
python3 -u ingest-osm.py --all-pending --delay 8 >> "$LOG" 2>&1 || {
    echo "WARN: some chains failed to ingest (see log); continuing with rebuild" | tee -a "$LOG"; }

# 2. Rebuild clusters (enrich_with_vwh attaches new enrichment stores)
echo "[$(STAMP)] [2/4] build-clusters.py" | tee -a "$LOG"
python3 -u build-clusters.py >> "$LOG" 2>&1

# 3. Re-run archetype tiering
echo "[$(STAMP)] [3/4] test-cluster-archetypes.py" | tee -a "$LOG"
python3 -u test-cluster-archetypes.py >> "$LOG" 2>&1

# 4. Deploy GeoJSONs
echo "[$(STAMP)] [4/4] deploy archetype GeoJSONs to gateway" | tee -a "$LOG"
cp work/archetype-vwh.geojson "$GW/archetype-vwh.geojson"
cp work/archetype-pks.geojson "$GW/archetype-pks.geojson"

# Summary
echo "[$(STAMP)] DONE — tier counts:" | tee -a "$LOG"
python3 - <<'PY' | tee -a "$LOG"
import json
from collections import Counter
for name, key in [("vwh", "vwh_tier"), ("pks", "commuter_tier")]:
    d = json.load(open(f"work/archetype-{name}.geojson"))
    c = Counter(f["properties"][key] for f in d["features"])
    by_iso_t12 = Counter(f["properties"]["iso"] for f in d["features"]
                         if f["properties"][key] in (1, 2))
    print(f"  {name}: tiers={dict(sorted(c.items()))} total={len(d['features'])}")
    if name == "vwh":
        print(f"    VWH T1+T2 by country: {dict(sorted(by_iso_t12.items(), key=lambda x:-x[1]))}")
PY
echo "──── enrichment ingest COMPLETE $(STAMP) ────" | tee -a "$LOG"
