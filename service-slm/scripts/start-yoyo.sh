#!/usr/bin/env bash
# On-demand Yo-Yo #1 start.
# Reads the same env vars as the Doorman idle monitor so the operator can
# export them once (e.g. in ~/.bashrc) and use both scripts without flags.
#
# Usage:
#   ./scripts/start-yoyo.sh
#   SLM_YOYO_GCP_INSTANCE=yoyo-tier-b-2 ./scripts/start-yoyo.sh
set -euo pipefail

PROJECT=${SLM_YOYO_GCP_PROJECT:-woodfine-node-gcp-free}
ZONE=${SLM_YOYO_GCP_ZONE:-us-west1-b}
INSTANCE=${SLM_YOYO_GCP_INSTANCE:-yoyo-tier-b-1}

echo "Starting ${INSTANCE} in ${PROJECT}/${ZONE} ..."
gcloud compute instances start "${INSTANCE}" \
    --project="${PROJECT}" \
    --zone="${ZONE}"

echo "VM started. Allow ~2 minutes for vLLM to finish loading the model."
echo "Doorman health probe will detect readiness and close the circuit within 30 seconds."
