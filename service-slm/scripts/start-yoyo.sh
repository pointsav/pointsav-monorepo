#!/usr/bin/env bash
# On-demand Yo-Yo #1 start with two-tier zone cycling.
#
# Mode 1 — Preemption recovery (normal case):
#   The existing VM is TERMINATED in SLM_YOYO_GCP_ZONE due to preemption.
#   Try gcloud instances.start in that zone; if the zone has capacity it
#   comes back in ~60 s. This is the fast, cheap path.
#
# Mode 2 — Zone stockout recovery (fallback):
#   The zone has ZONE_RESOURCE_POOL_EXHAUSTED and can't restart the VM.
#   Try each FALLBACK_ZONES entry in order:
#     a. gcloud instances.start --zone=<fallback>  ← still fails (wrong zone), but
#        we detect stockout vs. "instance not found" to pick the right action
#     b. On ZONE_RESOURCE_POOL_EXHAUSTED: try to create a fresh VM in the
#        fallback zone using the slm-yoyo image family.
#     c. Create a weights disk in the new zone.
#     d. Attach the disk to the new VM.
#     e. Update SLM_YOYO_GCP_ZONE in DOORMAN_ENV and local-doorman.env endpoint.
#
# The script reads and writes the same env vars as the Doorman idle monitor so
# the operator can export them once and use both scripts without flags.
#
# Usage:
#   ./scripts/start-yoyo.sh
#   SLM_YOYO_GCP_INSTANCE=yoyo-tier-b-2 ./scripts/start-yoyo.sh
set -uo pipefail

PROJECT="${SLM_YOYO_GCP_PROJECT:-woodfine-node-gcp-free}"
PRIMARY_ZONE="${SLM_YOYO_GCP_ZONE:-us-central1-a}"
INSTANCE="${SLM_YOYO_GCP_INSTANCE:-yoyo-tier-b-1}"
DOORMAN_ENV="${DOORMAN_ENV_FILE:-/etc/local-doorman/local-doorman.env}"
BEARER_TOKEN="${SLM_YOYO_BEARER:-}"
IMAGE_FAMILY="${SLM_YOYO_IMAGE_FAMILY:-slm-yoyo}"
IMAGE_PROJECT="${SLM_YOYO_IMAGE_PROJECT:-${PROJECT}}"
WEIGHTS_DISK="${INSTANCE}-weights"

# Ordered fallback zone list — used ONLY when the current zone is exhausted.
FALLBACK_ZONES=(
    "us-central1-a"
    "us-central1-b"
    "us-central1-c"
    "northamerica-northeast1-b"
    "northamerica-northeast1-c"
    "us-east1-b"
    "us-east1-c"
)

# ── Helper: check if gcloud error output indicates zone stockout ──────────────
is_stockout() {
    local stderr_output="$1"
    echo "${stderr_output}" | grep -q "ZONE_RESOURCE_POOL_EXHAUSTED\|does not have enough resources\|stockout"
}

# ── Helper: get the VM's current zone ────────────────────────────────────────
# If PRIMARY_ZONE is set and the VM exists there, return that zone (preferred).
# Otherwise return the first zone from the project-wide list (handles name
# collisions when the same instance name exists in multiple zones).
current_vm_zone() {
    if [[ -n "${PRIMARY_ZONE}" ]]; then
        local z
        z=$(gcloud compute instances list \
                --project="${PROJECT}" \
                --filter="name=${INSTANCE} AND zone:${PRIMARY_ZONE}" \
                --format="value(zone.basename())" 2>/dev/null | head -1)
        if [[ -n "${z}" ]]; then
            echo "${z}"
            return
        fi
    fi
    gcloud compute instances list \
        --project="${PROJECT}" \
        --filter="name=${INSTANCE}" \
        --format="value(zone.basename())" 2>/dev/null | head -1
}

# ── Helper: create a fresh VM in a zone ──────────────────────────────────────
provision_vm_in_zone() {
    local zone="$1"
    echo "  [PROVISION] Creating ${INSTANCE} in ${PROJECT}/${zone}..."

    # Create weights disk first
    echo "  [PROVISION] Creating weights disk ${WEIGHTS_DISK} in ${zone}..."
    if ! gcloud compute disks create "${WEIGHTS_DISK}" \
            --project="${PROJECT}" \
            --zone="${zone}" \
            --type=pd-ssd \
            --size=100GB \
            --labels=role=yoyo-weights 2>&1; then
        echo "  [PROVISION] Disk creation failed in ${zone} — trying next zone."
        return 1
    fi

    # Build metadata arg
    local meta_arg=""
    if [[ -n "${BEARER_TOKEN}" ]]; then
        meta_arg="--metadata=bearer-token=${BEARER_TOKEN}"
    fi

    # Create the instance
    local err_output
    err_output=$(gcloud compute instances create "${INSTANCE}" \
        --project="${PROJECT}" \
        --zone="${zone}" \
        --machine-type=g2-standard-4 \
        --accelerator=type=nvidia-l4,count=1 \
        --maintenance-policy=TERMINATE \
        --provisioning-model=SPOT \
        --instance-termination-action=STOP \
        --image-family="${IMAGE_FAMILY}" \
        --image-project="${IMAGE_PROJECT}" \
        --boot-disk-size=50GB \
        --boot-disk-type=pd-balanced \
        --disk=name="${WEIGHTS_DISK}",device-name=yoyo-weights,auto-delete=no \
        --tags=yoyo-tier-b \
        --scopes=cloud-platform \
        ${meta_arg} \
        --no-address 2>&1)

    if [[ $? -ne 0 ]]; then
        if is_stockout "${err_output}"; then
            echo "  [PROVISION] Stockout in ${zone} — deleting disk, trying next."
            gcloud compute disks delete "${WEIGHTS_DISK}" --project="${PROJECT}" --zone="${zone}" --quiet 2>/dev/null || true
            return 1
        else
            echo "  [PROVISION] VM creation failed in ${zone}: ${err_output}"
            gcloud compute disks delete "${WEIGHTS_DISK}" --project="${PROJECT}" --zone="${zone}" --quiet 2>/dev/null || true
            return 1
        fi
    fi

    echo "${zone}"
    return 0
}

# ── Helper: update Doorman env with new zone ─────────────────────────────────
update_doorman_env() {
    local new_zone="$1"
    if [[ -w "${DOORMAN_ENV}" ]]; then
        sed -i "s|^SLM_YOYO_GCP_ZONE=.*|SLM_YOYO_GCP_ZONE=${new_zone}|" "${DOORMAN_ENV}"
        echo "Updated SLM_YOYO_GCP_ZONE=${new_zone} in ${DOORMAN_ENV}."
    fi
}

# ─────────────────────────────────────────────────────────────────────────────
# Main
# ─────────────────────────────────────────────────────────────────────────────

KNOWN_ZONE=$(current_vm_zone)

if [[ -n "${KNOWN_ZONE}" ]]; then
    # ── Mode 1: VM exists — try to start it in its current zone ──────────────
    echo "Found ${INSTANCE} in ${PROJECT}/${KNOWN_ZONE} (current zone). Attempting start..."
    err=$(gcloud compute instances start "${INSTANCE}" \
        --project="${PROJECT}" \
        --zone="${KNOWN_ZONE}" 2>&1)
    if [[ $? -eq 0 ]]; then
        echo "VM started in zone ${KNOWN_ZONE} (Mode 1: preemption recovery)."
        [[ "${KNOWN_ZONE}" != "${PRIMARY_ZONE}" ]] && update_doorman_env "${KNOWN_ZONE}"
        echo "Allow ~2 minutes for vLLM to finish loading the model."
        echo "Doorman health probe will detect readiness within 30 seconds."
        exit 0
    fi

    if is_stockout "${err}"; then
        echo "Zone ${KNOWN_ZONE} has no L4 capacity (stockout). Entering Mode 2: zone relocation."
        echo "Note: weights disk must be uploaded again after VM is moved to a new zone."
    else
        echo "ERROR: Failed to start ${INSTANCE} in ${KNOWN_ZONE}: ${err}" >&2
        exit 1
    fi
else
    # No existing VM found — go straight to provisioning
    echo "No existing ${INSTANCE} found in project ${PROJECT}. Entering Mode 2: provisioning."
fi

# ── Mode 2: Provision a new VM in a fallback zone ────────────────────────────
# Build the fallback list, skipping the exhausted zone.
ZONES_TO_TRY=()
for z in "${FALLBACK_ZONES[@]}"; do
    [[ "${z}" != "${KNOWN_ZONE:-}" ]] && ZONES_TO_TRY+=("${z}")
done
# Try the new zone list for provisioning
for ZONE in "${ZONES_TO_TRY[@]}"; do
    echo "Trying to provision ${INSTANCE} in ${PROJECT}/${ZONE} ..."
    new_zone_result=$(provision_vm_in_zone "${ZONE}" 2>&1)
    if [[ $? -eq 0 ]]; then
        STARTED_ZONE="${ZONE}"
        echo "VM provisioned in zone ${STARTED_ZONE} (Mode 2: zone relocation)."
        update_doorman_env "${STARTED_ZONE}"
        echo ""
        echo "IMPORTANT — post-provisioning steps:"
        echo ""
        echo "  0. Add an external IP (if IAP is not available):"
        echo "     gcloud compute instances add-access-config ${INSTANCE} --zone=${STARTED_ZONE} --project=${PROJECT}"
        echo "     NEW_IP=\$(gcloud compute instances describe ${INSTANCE} --zone=${STARTED_ZONE} --project=${PROJECT} --format='value(networkInterfaces[0].accessConfigs[0].natIP)')"
        echo ""
        echo "  1. Set bearer token in instance metadata (if not set before provisioning):"
        echo "     gcloud compute instances add-metadata ${INSTANCE} --zone=${STARTED_ZONE} --project=${PROJECT} --metadata=bearer-token=\${SLM_YOYO_BEARER}"
        echo ""
        echo "  2. rc.local auto-mounts the weights disk at /data/weights on first boot."
        echo "     Verify: gcloud compute ssh ${INSTANCE} --zone=${STARTED_ZONE} --project=${PROJECT} --command='mountpoint /data/weights'"
        echo ""
        echo "  3. Upload weights:"
        echo "     gcloud compute scp <weights.gguf> ${INSTANCE}:/data/weights/olmo-3-32b-think-q4.gguf --zone=${STARTED_ZONE} --project=${PROJECT}"
        echo ""
        echo "  4. Start vllm: gcloud compute ssh ${INSTANCE} --zone=${STARTED_ZONE} --project=${PROJECT} --command='sudo systemctl start vllm.service'"
        echo ""
        echo "  5. Update SLM_YOYO_ENDPOINT in ${DOORMAN_ENV} with new external IP:"
        echo "     sudo sed -i \"s|SLM_YOYO_ENDPOINT=.*|SLM_YOYO_ENDPOINT=https://\${NEW_IP}:9443|\" ${DOORMAN_ENV}"
        echo "  6. Restart Doorman: sudo systemctl restart local-doorman.service"
        exit 0
    fi
done

echo "ERROR: Could not start or provision ${INSTANCE} in any zone." >&2
exit 1
