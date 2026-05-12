#!/usr/bin/env bash
# On-demand Yo-Yo #1 start with two-tier zone cycling and optional vLLM wait-ready.
#
# Mode 1 — Preemption recovery (normal case):
#   The existing VM is TERMINATED in SLM_YOYO_GCP_ZONE due to preemption.
#   Try gcloud instances.start in that zone; if the zone has capacity it
#   comes back in ~60 s. This is the fast, cheap path.
#
# Mode 2 — Zone stockout recovery (fallback):
#   The zone has ZONE_RESOURCE_POOL_EXHAUSTED and can't restart the VM.
#   Try each FALLBACK_ZONES entry in order, provisioning a fresh VM if needed.
#
# Day-time stockout: --retry-cycles=N --retry-wait-seconds=M lets the script
# spend a longer wall-clock budget hunting for L4 capacity (sleep + try again).
#
# Wait-ready: --wait-ready[=SECONDS] polls https://<vm-ip>:9443/health with
# bearer until it returns 200 (vLLM finished loading) or the timeout fires.
#
# Auto-snapshot: --auto-snapshot creates a snapshot the first time vLLM is
# verified ready, so subsequent zone migrations can restore the weights disk.
#
# Exit codes:
#   0 — VM up + (vLLM ready, if --wait-ready)
#   1 — GCE start failure (auth/quota/permission/unknown)
#   2 — vLLM ready-poll timeout (VM up, but model not loaded in time)
#   3 — zone stockout cascade exhausted across all retries
#
# Usage:
#   ./scripts/start-yoyo.sh
#   ./scripts/start-yoyo.sh --wait-ready=300 --auto-snapshot
#   ./scripts/start-yoyo.sh --retry-cycles=3 --retry-wait-seconds=300
#   SLM_YOYO_GCP_INSTANCE=yoyo-tier-b-2 ./scripts/start-yoyo.sh
set -uo pipefail

PROJECT="${SLM_YOYO_GCP_PROJECT:-woodfine-node-gcp-free}"
PRIMARY_ZONE="${SLM_YOYO_GCP_ZONE:-us-west1-b}"
INSTANCE="${SLM_YOYO_GCP_INSTANCE:-yoyo-tier-b-1}"
DOORMAN_ENV="${DOORMAN_ENV_FILE:-/etc/local-doorman/local-doorman.env}"
BEARER_TOKEN="${SLM_YOYO_BEARER:-}"
IMAGE_FAMILY="${SLM_YOYO_IMAGE_FAMILY:-slm-yoyo}"
IMAGE_PROJECT="${SLM_YOYO_IMAGE_PROJECT:-${PROJECT}}"
WEIGHTS_DISK="${INSTANCE}-weights"
# When set, new weights disks are restored from this snapshot instead of created empty.
# Set this after uploading weights: create-yoyo-snapshot.sh → SLM_YOYO_WEIGHTS_SNAPSHOT
WEIGHTS_SNAPSHOT="${SLM_YOYO_WEIGHTS_SNAPSHOT:-}"
LIFECYCLE_LOG="${SLM_YOYO_LIFECYCLE_LOG:-/var/log/yoyo-lifecycle.log}"

# ── Flag parsing ─────────────────────────────────────────────────────────────
WAIT_READY=0       # 0 = no wait, >0 = poll seconds before exiting
AUTO_SNAPSHOT=false
RETRY_CYCLES=1
RETRY_WAIT=300
WEIGHTS_GCS_BUCKET="${SLM_YOYO_WEIGHTS_GCS_BUCKET:-woodfine-node-gcp-free-foundry-substrate}"
while [[ $# -gt 0 ]]; do
    case "$1" in
        --wait-ready=*)         WAIT_READY="${1#*=}"; shift ;;
        --wait-ready)           WAIT_READY=5400; shift ;;
        --auto-snapshot)        AUTO_SNAPSHOT=true; shift ;;
        --retry-cycles=*)       RETRY_CYCLES="${1#*=}"; shift ;;
        --retry-wait-seconds=*) RETRY_WAIT="${1#*=}"; shift ;;
        --help|-h)
            sed -n '2,30p' "$0"
            exit 0
            ;;
        *) echo "Unknown flag: $1" >&2; exit 1 ;;
    esac
done

# ── Lifecycle logging ────────────────────────────────────────────────────────
log() {
    local ts msg
    ts="$(date -u +'%Y-%m-%dT%H:%M:%SZ')"
    msg="[start-yoyo ${ts}] $*"
    echo "${msg}"
    if [[ -w "$(dirname "${LIFECYCLE_LOG}")" ]] || [[ -w "${LIFECYCLE_LOG}" ]] 2>/dev/null; then
        echo "${msg}" >> "${LIFECYCLE_LOG}" 2>/dev/null || true
    fi
}

# Ordered fallback zone list — used when the primary zone is exhausted or when
# provisioning fresh. Order is tuned to L4 GPU capacity observed during recent
# Packer image builds (most-likely-available zones first). us-east4 omitted —
# does not stock g2-standard-4. Update when GCP capacity patterns shift.
FALLBACK_ZONES=(
    "us-west1-a"
    "us-central1-a"
    "us-central1-b"
    "us-central1-c"
    "us-east1-b"
    "us-east1-c"
    "us-east1-d"
    "us-west1-b"
    "us-west4-a"
    "northamerica-northeast1-b"
    "northamerica-northeast1-c"
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

    # Create weights disk — restore from snapshot if one exists, otherwise blank.
    # 256GB pd-balanced fits the first-boot bootstrap peak (safetensors 64GB + intermediate
    # fp16 GGUF 64GB during convert step, before cleanup) PLUS steady-state (base 20GB +
    # LoRA adapters 3GB + tokenizer + checkpoints + headroom). pd-balanced is much cheaper
    # than pd-ssd; LoRA I/O is fine on balanced. ~$26/mo always-attached.
    echo "  [PROVISION] Creating weights disk ${WEIGHTS_DISK} (256GB pd-balanced) in ${zone}..."
    local disk_create_args=(
        "${WEIGHTS_DISK}"
        --project="${PROJECT}"
        --zone="${zone}"
        --type=pd-balanced
        --labels=role=yoyo-weights
    )
    if [[ -n "${WEIGHTS_SNAPSHOT}" ]]; then
        echo "  [PROVISION] Restoring from snapshot ${WEIGHTS_SNAPSHOT} (weights preserved)."
        disk_create_args+=(--source-snapshot="${WEIGHTS_SNAPSHOT}")
    else
        echo "  [PROVISION] No snapshot set — empty disk; vllm-weights-prep.service will bootstrap from GCS or AllenAI."
        disk_create_args+=(--size=256GB)
    fi
    if ! gcloud compute disks create "${disk_create_args[@]}" 2>&1; then
        echo "  [PROVISION] Disk creation failed in ${zone} — trying next zone."
        return 1
    fi

    # Build metadata arg — bearer-token (nginx auth) + weights-gcs-bucket
    # (consumed by vllm-weights-prep.service to know where to fetch/upload).
    local meta_kv=()
    [[ -n "${BEARER_TOKEN}" ]] && meta_kv+=("bearer-token=${BEARER_TOKEN}")
    [[ -n "${WEIGHTS_GCS_BUCKET}" ]] && meta_kv+=("weights-gcs-bucket=${WEIGHTS_GCS_BUCKET}")
    local meta_arg=""
    if [[ "${#meta_kv[@]}" -gt 0 ]]; then
        meta_arg="--metadata=$(IFS=','; printf '%s' "${meta_kv[*]}")"
    fi

    # Create the instance.
    # Ephemeral external IP is allocated by default (no --no-address flag) so:
    #   (a) wait_for_vllm_ready can probe https://<ip>:9443/health from outside
    #   (b) Doorman's existing SLM_YOYO_ENDPOINT pattern (https://<ip>:9443) works
    #   (c) the VM has internet egress for HF download during first-boot bootstrap
    # Scope is restricted by VPC firewall rule (only the workspace VM IP allowed
    # through to port 9443; SSH is via IAP). This is the existing operational pattern.
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
        ${meta_arg} 2>&1)

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

# ── Helper: update Doorman env with new zone, IP, snapshot ──────────────────
# After every successful provision/start, the VM may have a new external IP.
# Doorman's SLM_YOYO_ENDPOINT must reflect this for the health probe to land.
# Best-effort: writes if the env file is writable; otherwise emits the new
# values to stdout so an operator can apply them via sudo.
update_doorman_env() {
    local new_zone="$1"
    local new_ip
    new_ip=$(gcloud compute instances describe "${INSTANCE}" \
            --project="${PROJECT}" --zone="${new_zone}" \
            --format='value(networkInterfaces[0].accessConfigs[0].natIP)' 2>/dev/null || echo "")
    local new_endpoint=""
    [[ -n "${new_ip}" ]] && new_endpoint="https://${new_ip}:9443"

    if [[ ! -w "${DOORMAN_ENV}" ]]; then
        echo "Note: ${DOORMAN_ENV} not writable by this process. Apply these as root:"
        echo "  SLM_YOYO_GCP_ZONE=${new_zone}"
        [[ -n "${new_endpoint}" ]] && echo "  SLM_YOYO_ENDPOINT=${new_endpoint}"
        [[ -n "${WEIGHTS_SNAPSHOT}" ]] && echo "  SLM_YOYO_WEIGHTS_SNAPSHOT=${WEIGHTS_SNAPSHOT}"
        echo "Then: sudo systemctl restart local-doorman.service"
        return 0
    fi

    sed -i "s|^SLM_YOYO_GCP_ZONE=.*|SLM_YOYO_GCP_ZONE=${new_zone}|" "${DOORMAN_ENV}"
    echo "Updated SLM_YOYO_GCP_ZONE=${new_zone} in ${DOORMAN_ENV}."

    if [[ -n "${new_endpoint}" ]]; then
        if grep -q "^SLM_YOYO_ENDPOINT=" "${DOORMAN_ENV}"; then
            sed -i "s|^SLM_YOYO_ENDPOINT=.*|SLM_YOYO_ENDPOINT=${new_endpoint}|" "${DOORMAN_ENV}"
        else
            echo "SLM_YOYO_ENDPOINT=${new_endpoint}" >> "${DOORMAN_ENV}"
        fi
        echo "Updated SLM_YOYO_ENDPOINT=${new_endpoint} in ${DOORMAN_ENV}."
    fi

    if [[ -n "${WEIGHTS_SNAPSHOT}" ]]; then
        if grep -q "^SLM_YOYO_WEIGHTS_SNAPSHOT=" "${DOORMAN_ENV}"; then
            sed -i "s|^SLM_YOYO_WEIGHTS_SNAPSHOT=.*|SLM_YOYO_WEIGHTS_SNAPSHOT=${WEIGHTS_SNAPSHOT}|" "${DOORMAN_ENV}"
        else
            echo "SLM_YOYO_WEIGHTS_SNAPSHOT=${WEIGHTS_SNAPSHOT}" >> "${DOORMAN_ENV}"
        fi
        echo "Updated SLM_YOYO_WEIGHTS_SNAPSHOT=${WEIGHTS_SNAPSHOT} in ${DOORMAN_ENV}."
    fi
}

# ── Helper: poll vLLM /health endpoint until 200 or timeout ──────────────────
# Returns 0 on ready, 1 on timeout. Uses bearer auth via nginx (port 9443).
wait_for_vllm_ready() {
    local zone="$1"
    local ip endpoint deadline http_code
    ip=$(gcloud compute instances describe "${INSTANCE}" \
            --project="${PROJECT}" --zone="${zone}" \
            --format='value(networkInterfaces[0].accessConfigs[0].natIP)' 2>/dev/null)
    if [[ -z "${ip}" ]]; then
        log "ERROR: could not determine VM external IP in ${zone} for wait-ready."
        return 1
    fi
    endpoint="https://${ip}:9443/health"
    deadline=$(( $(date +%s) + WAIT_READY ))
    log "Waiting for vLLM at ${endpoint} (timeout ${WAIT_READY}s)..."
    while [[ $(date +%s) -lt ${deadline} ]]; do
        http_code=$(curl -k -sS -o /tmp/yoyo-health.json -w '%{http_code}' \
            --max-time 5 -H "Authorization: Bearer ${BEARER_TOKEN}" \
            "${endpoint}" 2>/dev/null || echo "000")
        if [[ "${http_code}" == "200" ]]; then
            log "vLLM ready (HTTP 200 from ${endpoint})."
            return 0
        fi
        sleep 10
    done
    log "ERROR: vLLM did not become ready at ${endpoint} within ${WAIT_READY}s (last HTTP ${http_code:-???})."
    return 1
}

# ── Helper: trigger weights snapshot via create-yoyo-snapshot.sh ─────────────
maybe_create_snapshot() {
    local zone="$1"
    local snap_script
    snap_script="$(dirname "$0")/create-yoyo-snapshot.sh"
    if [[ ! -x "${snap_script}" ]]; then
        log "WARN: ${snap_script} not found or not executable — skipping auto-snapshot."
        return 0
    fi
    log "Auto-snapshot: creating snapshot of weights disk in ${zone}..."
    SLM_YOYO_GCP_ZONE="${zone}" "${snap_script}" 2>&1 | sed 's/^/  /' || \
        log "WARN: snapshot creation reported failure — continuing."
}

# ── Helper: print operator post-provisioning steps after Mode 2 ──────────────
print_post_provision_steps() {
    local zone="$1"
    cat <<EOF

IMPORTANT — post-provisioning steps:

  0. Add an external IP (if IAP is not available):
     gcloud compute instances add-access-config ${INSTANCE} --zone=${zone} --project=${PROJECT}
     NEW_IP=\$(gcloud compute instances describe ${INSTANCE} --zone=${zone} --project=${PROJECT} --format='value(networkInterfaces[0].accessConfigs[0].natIP)')

  1. Set bearer token in instance metadata (if not set before provisioning):
     gcloud compute instances add-metadata ${INSTANCE} --zone=${zone} --project=${PROJECT} --metadata=bearer-token=\${SLM_YOYO_BEARER}

  2. rc.local auto-mounts the weights disk at /data/weights on first boot.
     Verify: gcloud compute ssh ${INSTANCE} --zone=${zone} --project=${PROJECT} --command='mountpoint /data/weights'

  3. Upload weights:
     gcloud compute scp <weights.gguf> ${INSTANCE}:/data/weights/olmo-3-32b-think-q4.gguf --zone=${zone} --project=${PROJECT}

  4. Start vllm: gcloud compute ssh ${INSTANCE} --zone=${zone} --project=${PROJECT} --command='sudo systemctl start vllm.service'

  5. Update SLM_YOYO_ENDPOINT in ${DOORMAN_ENV} with new external IP:
     sudo sed -i "s|SLM_YOYO_ENDPOINT=.*|SLM_YOYO_ENDPOINT=https://\${NEW_IP}:9443|" ${DOORMAN_ENV}

  6. Restart Doorman: sudo systemctl restart local-doorman.service
EOF
}

# ── attempt_start_once: one full Mode-1+Mode-2 pass ──────────────────────────
# Sets STARTED_ZONE on success.
# Returns: 0 success, 1 hard failure (auth/permission), 3 stockout in all zones.
attempt_start_once() {
    local known_zone err
    known_zone=$(current_vm_zone)

    if [[ -n "${known_zone}" ]]; then
        # Mode 1: VM exists — try to start it in its current zone
        log "Found ${INSTANCE} in ${PROJECT}/${known_zone}. Attempting start (Mode 1)..."
        err=$(gcloud compute instances start "${INSTANCE}" \
            --project="${PROJECT}" --zone="${known_zone}" 2>&1)
        if [[ $? -eq 0 ]]; then
            log "VM started in ${known_zone} (Mode 1: preemption recovery)."
            STARTED_ZONE="${known_zone}"
            update_doorman_env "${known_zone}"
            return 0
        fi
        if is_stockout "${err}"; then
            log "Zone ${known_zone} has no L4 capacity. Falling through to Mode 2."
        else
            log "ERROR: failed to start ${INSTANCE} in ${known_zone}: ${err}"
            return 1
        fi
    else
        log "No existing ${INSTANCE} in project ${PROJECT} — entering Mode 2 (provision)."
    fi

    # Mode 2: provision a new VM in a fallback zone
    local zones_to_try=()
    for z in "${FALLBACK_ZONES[@]}"; do
        [[ "${z}" != "${known_zone:-}" ]] && zones_to_try+=("${z}")
    done
    for zone in "${zones_to_try[@]}"; do
        log "Trying to provision ${INSTANCE} in ${PROJECT}/${zone} ..."
        if provision_vm_in_zone "${zone}" >&2; then
            log "VM provisioned in ${zone} (Mode 2: zone relocation)."
            STARTED_ZONE="${zone}"
            update_doorman_env "${zone}"
            print_post_provision_steps "${zone}"
            return 0
        fi
    done

    log "All fallback zones exhausted in this cycle."
    return 3
}

# ─────────────────────────────────────────────────────────────────────────────
# Main — retry-cycle loop wraps Mode 1 + Mode 2; then optional wait-ready + snapshot
# ─────────────────────────────────────────────────────────────────────────────

log "Session start. instance=${INSTANCE} primary_zone=${PRIMARY_ZONE} retry_cycles=${RETRY_CYCLES} retry_wait=${RETRY_WAIT}s wait_ready=${WAIT_READY}s auto_snapshot=${AUTO_SNAPSHOT}"

STARTED_ZONE=""
cycle=0
while [[ "${cycle}" -lt "${RETRY_CYCLES}" ]]; do
    if [[ "${cycle}" -gt 0 ]]; then
        log "Stockout retry cycle ${cycle}/${RETRY_CYCLES} — sleeping ${RETRY_WAIT}s before next attempt..."
        sleep "${RETRY_WAIT}"
    fi

    attempt_start_once
    rc=$?
    if [[ "${rc}" -eq 0 ]]; then
        break
    elif [[ "${rc}" -eq 1 ]]; then
        log "Hard failure during start attempt — aborting (exit 1)."
        exit 1
    fi
    # rc == 3: stockout cascade exhausted in this cycle — retry if cycles remain
    cycle=$(( cycle + 1 ))
done

if [[ -z "${STARTED_ZONE}" ]]; then
    log "ERROR: could not start or provision ${INSTANCE} in any zone after ${RETRY_CYCLES} cycle(s). Exit 3."
    exit 3
fi

# ── Optional wait-ready + auto-snapshot ──────────────────────────────────────
if [[ "${WAIT_READY}" -gt 0 ]]; then
    if ! wait_for_vllm_ready "${STARTED_ZONE}"; then
        exit 2
    fi
    if [[ "${AUTO_SNAPSHOT}" == "true" ]] && [[ -z "${WEIGHTS_SNAPSHOT}" ]]; then
        maybe_create_snapshot "${STARTED_ZONE}"
    fi
else
    log "Allow ~2 minutes for vLLM to finish loading the model."
    log "Doorman health probe will detect readiness within 30 seconds."
fi

log "Session done. Exit 0."
exit 0
