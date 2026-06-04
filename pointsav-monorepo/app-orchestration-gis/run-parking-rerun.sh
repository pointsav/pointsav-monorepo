#!/usr/bin/env bash
# run-parking-rerun.sh — Re-ingest parking for the 7 countries that returned 0
# on Jun 4 due to Overpass OOM on large bboxes. Now uses automatic tiling
# (TILE_GRIDS in ingest-osm-parking.py) for US, CA, FR, DE, IT, PL, NO.
#
# Appends to the existing cleansed-civic-parking.jsonl (does NOT replace).
# The 15 countries that succeeded on Jun 4 (AT, GB, ES, MX, NL, BE, CH, CZ,
# HU, DK, FI, SE, GR, PT, RO) remain in the file and are not re-queried.
#
# Run after 05:00 UTC per Overnight Builds Policy. Allow 2–3 hours.

set -u
cd "$(dirname "$0")" || exit 1
ts() { date -u +"%Y-%m-%dT%H:%M:%SZ"; }
LOG="parking-rerun.log"

echo "=================================================================" >> "$LOG"
echo "[$(ts)] run-parking-rerun.sh START" >> "$LOG"
echo "  Countries: US CA FR DE IT PL NO" >> "$LOG"
echo "  Mode: append (preserving Jun 4 data for 15 working countries)" >> "$LOG"
echo "=================================================================" >> "$LOG"

python3 ingest-osm-parking.py --countries US CA FR DE IT PL NO >> "$LOG" 2>&1 \
    || echo "[$(ts)] WARN: parking rerun exited non-zero (see log)" >> "$LOG"

echo "=================================================================" >> "$LOG"
echo "[$(ts)] run-parking-rerun.sh DONE" >> "$LOG"
echo "  Next: wire cleansed-civic-parking.jsonl GREENFIELD filter into" >> "$LOG"
echo "  build-pks-clusters.py once US+CA data is confirmed non-zero." >> "$LOG"
echo "=================================================================" >> "$LOG"
