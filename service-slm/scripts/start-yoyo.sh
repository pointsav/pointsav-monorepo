#!/usr/bin/env bash
# On-demand Yo-Yo #1 start with Spot preemption zone cycling.
# Reads the same env vars as the Doorman idle monitor so the operator can
# export them once (e.g. in ~/.bashrc) and use both scripts without flags.
# On failure in the primary zone, tries each fallback zone in order and
# updates SLM_YOYO_GCP_ZONE in /etc/local-doorman/local-doorman.env so the
# idle monitor and subsequent starts use the same zone.
#
# Usage:
#   ./scripts/start-yoyo.sh
#   SLM_YOYO_GCP_INSTANCE=yoyo-tier-b-2 ./scripts/start-yoyo.sh
set -uo pipefail

PROJECT=${SLM_YOYO_GCP_PROJECT:-woodfine-node-gcp-free}
PRIMARY_ZONE=${SLM_YOYO_GCP_ZONE:-southamerica-east1-b}
INSTANCE=${SLM_YOYO_GCP_INSTANCE:-yoyo-tier-b-1}
DOORMAN_ENV=${DOORMAN_ENV_FILE:-/etc/local-doorman/local-doorman.env}

# Fallback zone list — primary zone is always tried first regardless of this list.
FALLBACK_ZONES=(
    "southamerica-east1-b"
    "southamerica-east1-c"
    "southamerica-east1-a"
    "northamerica-northeast1-a"
    "us-central1-a"
)

# Build the ordered list: primary zone first, then fallbacks (deduplicated).
ZONES_TO_TRY=("${PRIMARY_ZONE}")
for z in "${FALLBACK_ZONES[@]}"; do
    if [[ "${z}" != "${PRIMARY_ZONE}" ]]; then
        ZONES_TO_TRY+=("${z}")
    fi
done

STARTED_ZONE=""
for ZONE in "${ZONES_TO_TRY[@]}"; do
    echo "Trying ${INSTANCE} in ${PROJECT}/${ZONE} ..."
    if gcloud compute instances start "${INSTANCE}" \
            --project="${PROJECT}" \
            --zone="${ZONE}" 2>/dev/null; then
        STARTED_ZONE="${ZONE}"
        break
    else
        echo "  Zone ${ZONE} unavailable or preempted — trying next."
    fi
done

if [[ -z "${STARTED_ZONE}" ]]; then
    echo "ERROR: Could not start ${INSTANCE} in any zone." >&2
    exit 1
fi

echo "VM started in zone ${STARTED_ZONE}."

# Persist the successful zone so the idle monitor and next start use it.
if [[ "${STARTED_ZONE}" != "${PRIMARY_ZONE}" ]] && [[ -w "${DOORMAN_ENV}" ]]; then
    sed -i "s|^SLM_YOYO_GCP_ZONE=.*|SLM_YOYO_GCP_ZONE=${STARTED_ZONE}|" "${DOORMAN_ENV}"
    echo "Updated SLM_YOYO_GCP_ZONE=${STARTED_ZONE} in ${DOORMAN_ENV}."
fi

echo "Allow ~2 minutes for vLLM to finish loading the model."
echo "Doorman health probe will detect readiness and close the circuit within 30 seconds."
