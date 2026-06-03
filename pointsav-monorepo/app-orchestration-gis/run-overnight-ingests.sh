#!/usr/bin/env bash
# run-overnight-ingests.sh — overnight OSM ingests for the Commuter + Urban Fringe rework.
# Scheduled via crontab at 05:00 UTC (after-10pm-Vancouver heavy-build policy).
# Produces DATA only — does NOT edit build scripts, rebuild, or sync. Review next session.
#
#   Commuter  : parking layer (park-and-ride / structured parking) for the geometric
#               park-and-ride model's BUILT/PARTIAL/GREENFIELD filter.
#   Urban Fringe : 3 new brand-keyed categories (builders' merchants, self-storage,
#               trade counters) + 1 tag-based category (parcel depots).
#
# Each step is independent; a failure in one does not abort the others.

set -u
cd "$(dirname "$0")" || exit 1
ts() { date -u +"%Y-%m-%dT%H:%M:%SZ"; }
echo "=================================================================="
echo "[$(ts)] run-overnight-ingests.sh START"
echo "=================================================================="

# 1. Commuter — parking layer (park_ride + structured parking, all countries)
echo "[$(ts)] STEP 1/3 — parking layer"
python3 ingest-osm-parking.py --all --replace || echo "[$(ts)] WARN: parking ingest exited non-zero"

# 2. Urban Fringe — parcel depots (tag-based: post_depot + office=logistics)
echo "[$(ts)] STEP 2/3 — parcel depots"
python3 ingest-osm-parcel-depot.py --all --replace || echo "[$(ts)] WARN: parcel-depot ingest exited non-zero"

# 3. Urban Fringe — new brand chains (builders' merchants, self-storage, trade counters)
echo "[$(ts)] STEP 3/3 — VWH brand chains"
VWH_CHAINS="
  travis-perkins-uk jewson-uk selco-uk buildbase-uk mkm-uk
  point-p-fr bigmat-fr bigmat-es bigmat-it bauking-de raab-karcher-de
  public-storage-us extra-space-storage-us cubesmart-us u-haul-us
  big-yellow-uk safestore-uk shurgard-eu
  screwfix-uk toolstation-uk
"
# shellcheck disable=SC2086
python3 ingest-osm.py --chain $VWH_CHAINS || echo "[$(ts)] WARN: brand-chain ingest exited non-zero"

echo "=================================================================="
echo "[$(ts)] run-overnight-ingests.sh DONE"
echo "  Next session: review counts, wire new categories into build-vwh-clusters.py,"
echo "  apply geometric park-and-ride model to build-pks-clusters.py, rebuild, sync."
echo "=================================================================="
