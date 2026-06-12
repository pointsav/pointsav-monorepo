#!/usr/bin/env bash
# run-infra-ingest.sh — PKS transit infrastructure ingest (airports + railway)
# Run when Overpass rate limit has cleared (overnight, or after cooldown).
# Heavy: ~15 US airport tiles + 16 countries × 2 scripts. Allow 30-60 min.
cd "$(dirname "${BASH_SOURCE[0]}")"
LOG="infra-ingest.log"
echo "──── infra ingest $(date -u '+%Y-%m-%dT%H:%M:%SZ') ────" >> "$LOG"

echo "[$(date -u '+%H:%M')] Airports --all" >> "$LOG"
python3 -u ingest-osm-airports.py --all --delay 10 >> "$LOG" 2>&1

echo "[$(date -u '+%H:%M')] Railway --all" >> "$LOG"
python3 -u ingest-osm-railway.py --all --delay 10 >> "$LOG" 2>&1

echo "──── infra DONE $(date -u '+%Y-%m-%dT%H:%M:%SZ') ────" >> "$LOG"
SP=/srv/foundry/deployments/cluster-totebox-personnel-1/service-places
echo "airports: $(wc -l < $SP/cleansed-civic-airports.jsonl 2>/dev/null || echo 0)" >> "$LOG"
echo "railway:  $(wc -l < $SP/cleansed-civic-railway.jsonl 2>/dev/null || echo 0)" >> "$LOG"
