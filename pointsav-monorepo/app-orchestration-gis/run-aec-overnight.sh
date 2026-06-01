#!/usr/bin/env bash
# run-aec-overnight.sh — sequential overnight AEC refresh (race-free).
#
# The seismic and flood builds BOTH read-modify-write the same clusters-meta.json
# (patching seismic / flood_hazard / wildfire_hazard) and the enrichment refresh
# rebuilds clusters.geojson. Running them as concurrent crons races on those files,
# so this wrapper serialises them. Run after 05:00 UTC (10pm Vancouver) per the
# Overnight Builds Policy.
#
# Order: seismic (PGA) → flood (AQUEDUCT + FEMA REST + GB/IT + GFWED wildfire) →
#        enrichment refresh (rebuild clusters.geojson + archetype tiers).
#
# Not set -e: a single build failing must not abort the others.
set -uo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"
LOG="aec-overnight.log"
S() { date -u '+%Y-%m-%dT%H:%M:%SZ'; }

echo "──── AEC overnight $(S) ────" >> "$LOG"

echo "[$(S)] [1/3] seismic (PGA + ESHM20 + wetland)" >> "$LOG"
bash build-aec-seismic.sh >> "$LOG" 2>&1 || echo "  seismic FAILED (see build-aec-seismic.log)" >> "$LOG"

echo "[$(S)] [2/3] flood + wildfire (AQUEDUCT/FEMA/GB/IT + GFWED)" >> "$LOG"
bash build-aec-flood.sh >> "$LOG" 2>&1 || echo "  flood FAILED (see build-aec-flood.log)" >> "$LOG"

echo "[$(S)] [3/3] enrichment refresh" >> "$LOG"
bash run-enrichment-ingest.sh >> "$LOG" 2>&1 || echo "  enrichment FAILED (see enrichment-ingest.log)" >> "$LOG"

# Summary
echo "[$(S)] DONE — clusters-meta AEC coverage:" >> "$LOG"
python3 - >> "$LOG" 2>&1 <<'PY'
import json
m = "/srv/foundry/deployments/gateway-orchestration-gis-1/www/data/clusters-meta.json"
try:
    d = json.load(open(m))
    flood = sum(1 for c in d if c.get('flood_hazard'))
    wild  = sum(1 for c in d if c.get('wildfire_hazard'))
    seis  = sum(1 for c in d if c.get('seismic_pga') or c.get('seismic_hazard') or c.get('seismic'))
    print(f"  clusters={len(d)} flood_hazard={flood} wildfire_hazard={wild} seismic={seis}")
except Exception as e:
    print(f"  (could not read clusters-meta: {e})")
PY
echo "──── AEC overnight COMPLETE $(S) ────" >> "$LOG"
