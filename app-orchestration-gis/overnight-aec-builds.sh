#!/usr/bin/env bash
# overnight-aec-builds.sh — AEC + park_ride ingest builds
#
# Scheduled to run after 05:00 UTC (22:00 PDT) per overnight-builds policy.
# DO NOT run during business hours — heavy downloads and compute.
#
# Steps:
#   1. park_ride ingest for US/CA/DE/FR/IT/PL/NO/IS
#   2. PKS cluster rebuild (if ingest succeeded)
#   3. build-aec-seismic.sh (wetland VRT + US/CA USGS/NRCan seismic)
#   4. build-aec-flood.sh (wildfire downloads ~1.5GB + classify)
#
# Logs written to app-orchestration-gis/overnight-aec-builds.log

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG="$SCRIPT_DIR/overnight-aec-builds.log"

echo "=== overnight-aec-builds.sh started $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" | tee -a "$LOG"

# Step 1 — park_ride ingest
echo "[1/4] park_ride ingest: US CA DE FR IT PL NO IS" | tee -a "$LOG"
cd "$SCRIPT_DIR"
python3 ingest-osm-parking.py --countries US CA DE FR IT PL NO IS >> "$LOG" 2>&1
echo "  → park_ride ingest complete $(date -u +%H:%MZ)" | tee -a "$LOG"

# Step 2 — PKS rebuild
echo "[2/4] PKS cluster rebuild" | tee -a "$LOG"
python3 build-pks-clusters.py >> "$LOG" 2>&1
echo "  → PKS rebuild complete $(date -u +%H:%MZ)" | tee -a "$LOG"

# Step 3 — AEC seismic (wetland + USGS/NRCan)
echo "[3/4] build-aec-seismic.sh" | tee -a "$LOG"
bash build-aec-seismic.sh >> "$LOG" 2>&1
echo "  → AEC seismic complete $(date -u +%H:%MZ)" | tee -a "$LOG"

# Step 4 — AEC flood + wildfire
echo "[4/4] build-aec-flood.sh" | tee -a "$LOG"
bash build-aec-flood.sh >> "$LOG" 2>&1
echo "  → AEC flood complete $(date -u +%H:%MZ)" | tee -a "$LOG"

echo "=== overnight-aec-builds.sh finished $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" | tee -a "$LOG"
