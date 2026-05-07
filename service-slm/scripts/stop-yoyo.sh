#!/usr/bin/env bash
# On-demand Yo-Yo #1 stop (manual override).
# Under normal operation the idle monitor stops the VM automatically after
# SLM_YOYO_IDLE_MINUTES (default 30) of inactivity. Use this script only
# when an immediate stop is required (e.g. cost emergency, maintenance).
#
# Usage:
#   ./scripts/stop-yoyo.sh
#   SLM_YOYO_GCP_INSTANCE=yoyo-tier-b-2 ./scripts/stop-yoyo.sh
set -euo pipefail

PROJECT=${SLM_YOYO_GCP_PROJECT:-woodfine-node-gcp-free}
ZONE=${SLM_YOYO_GCP_ZONE:-us-west1-b}
INSTANCE=${SLM_YOYO_GCP_INSTANCE:-yoyo-tier-b-1}

echo "Stopping ${INSTANCE} in ${PROJECT}/${ZONE} ..."
gcloud compute instances stop "${INSTANCE}" \
    --project="${PROJECT}" \
    --zone="${ZONE}"

echo "VM stopped."
