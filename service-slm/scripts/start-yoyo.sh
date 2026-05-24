#!/usr/bin/env bash
# On-demand Yo-Yo #1 start with two-tier zone cycling and optional vLLM wait-ready.
#
# Mode 1 — Preemption recovery (normal case):
#   The existing VM is TERMINATED in SLM_YOYO_GCP_ZONE due to preemption.
#   Try gcloud instances.start in that zone; if the zone has capacity it
#   comes back in ~60 s. This is the fast, cheap path.
#
# Mode 2 — Zone migration ONLY (not for stockouts):
#   Provision a fresh VM in a different zone. REQUIRES --enable-zone-fallback flag.
#   Each zone attempt creates a 256 GB disk ($2-20 per probe). Weights not present
#   on the new disk — must be restored from snapshot (30-60 min). Use only for a
#   deliberate operator-approved zone migration, never as a stockout workaround.
#
# Default stockout behavior: retry 4 times at 15-min intervals in europe-west4-a.
# Override with --retry-cycles=N --retry-wait-seconds=M. Zone fallback is not
# attempted unless --enable-zone-fallback is explicitly passed on the command line.
#
# Wait-ready: --wait-ready[=SECONDS] polls https://<vm-ip>:9443/health with
# bearer until it returns 200 (vLLM finished loading) or the timeout fires.
#
# Auto-snapshot: --auto-snapshot creates a snapshot the first time vLLM is
# verified ready, so subsequent zone migrations can restore the weights disk.
#
# Cost guardrails (Phase 0 G1/G3/G8 — BRIEF-flow-restructure.md):
#   G8  daily $-cap — refuse a start once today's runtime spend hits the cap;
#       grant the VM a max-lifetime derived from the *remaining* daily budget.
#   G3  the VM-side dead-man's-switch reads max-lifetime-seconds and self-stops.
#   G1  attempt cap — bound provision attempts within one invocation.
#
# Exit codes:
#   0 — VM up + (vLLM ready, if --wait-ready)
#   1 — GCE start failure (auth/quota/permission/unknown)
#   2 — vLLM ready-poll timeout (VM up, but model not loaded in time)
#   3 — zone stockout cascade exhausted across all retries
#   4 — cost guardrail tripped (daily budget cap or launch attempt cap)
#
# Usage:
#   ./scripts/start-yoyo.sh
#   ./scripts/start-yoyo.sh --wait-ready=300 --auto-snapshot
#   ./scripts/start-yoyo.sh --wait-ready=300 --runtime=1h
#   ./scripts/start-yoyo.sh --retry-cycles=3 --retry-wait-seconds=300
#   SLM_YOYO_GCP_INSTANCE=yoyo-tier-b-2 ./scripts/start-yoyo.sh
#
# --runtime=<duration>  Hard wall-clock stop cap. After this duration a background
#                       watchdog calls stop-yoyo.sh regardless of activity. The
#                       Doorman idle monitor (30 min idle) is the earlier-exit path;
#                       whichever fires first wins. Format: 1h, 90m, or bare seconds.
set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

PROJECT="${SLM_YOYO_GCP_PROJECT:-woodfine-node-gcp-free}"
PRIMARY_ZONE="${SLM_YOYO_GCP_ZONE:-europe-west4-a}"
INSTANCE="${SLM_YOYO_GCP_INSTANCE:-yoyo-tier-b-1}"
DOORMAN_ENV="${DOORMAN_ENV_FILE:-/etc/local-doorman/local-doorman.env}"
# Zone fallback is HARDCODED false — it cannot be enabled via env var.
# Env vars in /etc/local-doorman/local-doorman.env persist and would silently
# trigger fallback on every automated restart, costing $2-20 per zone probe.
# Zone fallback requires --enable-zone-fallback on the command line (explicit,
# operator-typed, not persistent). See Mode 2 header comment above.
ALLOW_ZONE_FALLBACK=false
BEARER_TOKEN="${SLM_YOYO_BEARER:-}"
IMAGE_FAMILY="${SLM_YOYO_IMAGE_FAMILY:-slm-yoyo}"
IMAGE_PROJECT="${SLM_YOYO_IMAGE_PROJECT:-${PROJECT}}"
WEIGHTS_DISK="${INSTANCE}-weights"
# When set, new weights disks are restored from this snapshot instead of created empty.
# Set this after uploading weights: create-yoyo-snapshot.sh → SLM_YOYO_WEIGHTS_SNAPSHOT
WEIGHTS_SNAPSHOT="${SLM_YOYO_WEIGHTS_SNAPSHOT:-}"
LIFECYCLE_LOG="${SLM_YOYO_LIFECYCLE_LOG:-/var/log/yoyo-lifecycle.log}"

# ── Cost guardrails — env-tunable (Phase 0 G1/G3/G8 hardening) ───────────────
# The daily Yo-Yo runs on-demand (decision D1): no preemption, so an un-stopped
# VM bills until something stops it. These bound the spend; all are env-tunable.
DAILY_BUDGET_USD="${SLM_YOYO_DAILY_BUDGET_USD:-3.00}"
RATE_USD_PER_HOUR="${SLM_YOYO_RATE_USD_PER_HOUR:-0.71}"   # g2-standard-4 + L4 on-demand
MAX_LAUNCH_ATTEMPTS="${SLM_YOYO_MAX_LAUNCH_ATTEMPTS:-3}"
MAX_LIFETIME_SECONDS="${SLM_YOYO_MAX_LIFETIME_SECONDS:-14400}"   # absolute ceiling, 4 h

# ── Flag parsing ─────────────────────────────────────────────────────────────
WAIT_READY=0       # 0 = no wait, >0 = poll seconds before exiting
RUNTIME_SECONDS=0  # 0 = no hard cap; >0 = watchdog stops VM after this many seconds
AUTO_SNAPSHOT=false
RETRY_CYCLES=4    # retry 4× before giving up (~1 h total at 15-min intervals)
RETRY_WAIT=900    # 15 min between retries — stockouts typically clear within 30 min
WEIGHTS_GCS_BUCKET="${SLM_YOYO_WEIGHTS_GCS_BUCKET:-woodfine-node-gcp-free-foundry-substrate}"
while [[ $# -gt 0 ]]; do
    case "$1" in
        --wait-ready=*)         WAIT_READY="${1#*=}"; shift ;;
        --wait-ready)           WAIT_READY=5400; shift ;;
        --auto-snapshot)        AUTO_SNAPSHOT=true; shift ;;
        --retry-cycles=*)       RETRY_CYCLES="${1#*=}"; shift ;;
        --retry-wait-seconds=*) RETRY_WAIT="${1#*=}"; shift ;;
        --runtime=*)
            raw="${1#*=}"
            if [[ "${raw}" =~ ^([0-9]+)h$ ]]; then
                RUNTIME_SECONDS=$(( ${BASH_REMATCH[1]} * 3600 ))
            elif [[ "${raw}" =~ ^([0-9]+)m$ ]]; then
                RUNTIME_SECONDS=$(( ${BASH_REMATCH[1]} * 60 ))
            elif [[ "${raw}" =~ ^([0-9]+)$ ]]; then
                RUNTIME_SECONDS="${raw}"
            else
                echo "Unknown --runtime format: ${raw} (use 1h, 90m, or bare seconds)" >&2
                exit 1
            fi
            shift ;;
        --enable-zone-fallback)
            ALLOW_ZONE_FALLBACK=true
            log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            log "WARNING: Zone fallback ENABLED via --enable-zone-fallback."
            log "  Each fallback zone attempt creates a 256 GB disk (\$2-20 per probe)."
            log "  Weights are NOT present on the new disk — restore from snapshot"
            log "  takes 30-60 min. Use ONLY for deliberate zone migration."
            log "  On stockout, the correct response is wait and retry in primary zone."
            log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            shift ;;
        --help|-h)
            sed -n '2,46p' "$0"
            exit 0
            ;;
        *) echo "Unknown flag: $1" >&2; exit 1 ;;
    esac
done

# Phase 0 G3: an explicit --runtime cap, if set, becomes the max-lifetime base
# (the daily-budget clamp in derive_max_lifetime may shorten it further). The
# +10 min grace lets the in-process watchdog fire first; the VM-side deadman is
# the backstop.
if [[ "${RUNTIME_SECONDS}" -gt 0 ]]; then
    MAX_LIFETIME_SECONDS=$(( RUNTIME_SECONDS + 600 ))
fi

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

# ── Cost guardrails — daily $-cap ledger (Phase 0 G8) ────────────────────────
# bash has no float math — all money is integer cents.
usd_to_cents() { awk "BEGIN{printf \"%d\", ($1)*100 + 0.5}"; }
cents_to_usd() { awk "BEGIN{printf \"%.2f\", ($1)/100}"; }

resolve_daily_ledger() {
    local want dir
    want="${SLM_YOYO_DAILY_LEDGER:-/var/lib/yoyo/daily-spend}"
    dir="$(dirname "${want}")"
    if mkdir -p "${dir}" 2>/dev/null && [[ -w "${dir}" ]]; then
        echo "${want}"
    else
        echo "${TMPDIR:-/tmp}/yoyo-daily-spend"
    fi
}
DAILY_LEDGER="$(resolve_daily_ledger)"
DAILY_SPENT_CENTS=0
LAUNCH_ATTEMPTS=0

# daily_reconcile — book the VM's last completed run against today's ledger,
# reading GCE's authoritative lastStart/lastStopTimestamp so it is correct no
# matter which path stopped the VM (deadman, idle monitor, stop-yoyo, preempt).
# Resets at 00:00 UTC. Sets DAILY_SPENT_CENTS.
# Ledger line format: "<utc-date> <cents-spent> <booked-stop-epoch>".
daily_reconcile() {
    local today led_date led_cents led_booked zone fields ls lp s_ep p_ep
    today=$(date -u +%Y-%m-%d)
    led_date=""; led_cents=0; led_booked=0
    if [[ -r "${DAILY_LEDGER}" ]]; then
        read -r led_date led_cents led_booked < "${DAILY_LEDGER}" 2>/dev/null || true
    fi
    if [[ "${led_date:-}" != "${today}" ]]; then
        led_date="${today}"; led_cents=0; led_booked=0
    fi
    zone=$(current_vm_zone)
    if [[ -n "${zone}" ]]; then
        fields=$(gcloud compute instances describe "${INSTANCE}" \
            --project="${PROJECT}" --zone="${zone}" \
            --format='value(lastStartTimestamp,lastStopTimestamp)' 2>/dev/null || echo "")
        ls=$(awk '{print $1}' <<<"${fields}")
        lp=$(awk '{print $2}' <<<"${fields}")
        if [[ -n "${ls}" && -n "${lp}" ]]; then
            s_ep=$(date -d "${ls}" +%s 2>/dev/null || echo 0)
            p_ep=$(date -d "${lp}" +%s 2>/dev/null || echo 0)
            # Book a completed run once, and only if it stopped today.
            if [[ "${p_ep}" -gt "${led_booked}" && "${p_ep}" -gt "${s_ep}" ]]; then
                if [[ "$(date -u -d "@${p_ep}" +%Y-%m-%d 2>/dev/null)" == "${today}" ]]; then
                    local run_s cost_c
                    run_s=$(( p_ep - s_ep ))
                    cost_c=$(awk "BEGIN{printf \"%d\", ${run_s}*${RATE_USD_PER_HOUR}/3600*100 + 0.5}")
                    led_cents=$(( led_cents + cost_c ))
                    log "Daily ledger: booked prior run ${run_s}s = \$$(cents_to_usd "${cost_c}")."
                fi
                led_booked="${p_ep}"
            fi
        fi
    fi
    printf '%s %s %s\n' "${led_date}" "${led_cents}" "${led_booked}" > "${DAILY_LEDGER}" 2>/dev/null || true
    DAILY_SPENT_CENTS="${led_cents}"
}

# daily_guard — refuse the launch if today's runtime spend has hit the cap.
daily_guard() {
    local budget_cents
    budget_cents=$(usd_to_cents "${DAILY_BUDGET_USD}")
    if [[ "${DAILY_SPENT_CENTS}" -ge "${budget_cents}" ]]; then
        log "ABORT: daily Yo-Yo budget reached — \$$(cents_to_usd "${DAILY_SPENT_CENTS}") spent today >= \$${DAILY_BUDGET_USD} cap. Resets 00:00 UTC; raise SLM_YOYO_DAILY_BUDGET_USD to override."
        return 1
    fi
    return 0
}

# derive_max_lifetime — clamp the VM's max-lifetime so the dead-man's-switch
# self-stops the VM when the day's remaining budget would be spent.
derive_max_lifetime() {
    local budget_cents remaining_cents budget_seconds
    budget_cents=$(usd_to_cents "${DAILY_BUDGET_USD}")
    remaining_cents=$(( budget_cents - DAILY_SPENT_CENTS ))
    [[ "${remaining_cents}" -lt 0 ]] && remaining_cents=0
    budget_seconds=$(awk "BEGIN{printf \"%d\", ${remaining_cents}/100/${RATE_USD_PER_HOUR}*3600}")
    if [[ "${budget_seconds}" -lt "${MAX_LIFETIME_SECONDS}" ]]; then
        MAX_LIFETIME_SECONDS="${budget_seconds}"
    fi
    log "Daily budget \$$(cents_to_usd "${DAILY_SPENT_CENTS}")/\$${DAILY_BUDGET_USD} spent -> VM max-lifetime ${MAX_LIFETIME_SECONDS}s."
}

# UTC offsets for L4-capable GCP zones (integer hours; DST ignored — 1h precision
# is sufficient for demand-pattern scoring). Spot capacity correlates with commercial
# compute demand, which follows business hours in each zone's local market.
# Scoring: local hour 01-07 = deep night (5), 20-01 = late evening (4),
#          07-09 = early morning (3), daytime = 1.
# Zones where it is currently night float to the top of the fallback list,
# maximising the chance of finding available L4 Spot capacity.
# us-east4 omitted — does not stock g2-standard-4.
declare -A ZONE_UTC_OFFSET=(
    ["us-west1-a"]=-8    ["us-west1-b"]=-8
    ["us-west4-a"]=-8
    ["us-central1-a"]=-6 ["us-central1-b"]=-6 ["us-central1-c"]=-6
    ["us-east1-b"]=-5    ["us-east1-c"]=-5    ["us-east1-d"]=-5
    ["northamerica-northeast1-b"]=-5 ["northamerica-northeast1-c"]=-5
    ["europe-west1-b"]=1 ["europe-west1-c"]=1 ["europe-west4-a"]=1
    ["europe-west2-a"]=0 ["europe-west2-b"]=0
    ["asia-east1-a"]=8   ["asia-east1-b"]=8
    ["asia-southeast1-a"]=8 ["asia-southeast1-b"]=8
)

# Returns zone names one-per-line, sorted by night-score descending.
# Excludes the zone passed as $1 (already tried in Mode 1).
# Ties broken by $RANDOM so repeated runs don't hammer the same zone.
sorted_fallback_zones() {
    local skip_zone="${1:-}"
    local utc_hour
    utc_hour=$(date -u +%-H)   # %-H strips leading zero for bash arithmetic
    local -a scored=()
    local zone offset local_hour score
    for zone in "${!ZONE_UTC_OFFSET[@]}"; do
        [[ "${zone}" == "${skip_zone}" ]] && continue
        offset="${ZONE_UTC_OFFSET[$zone]}"
        local_hour=$(( (utc_hour + offset + 24) % 24 ))
        if   (( local_hour >= 1  && local_hour <  7 )); then score=5
        elif (( local_hour >= 20 || local_hour <  1 )); then score=4
        elif (( local_hour >= 7  && local_hour <  9 )); then score=3
        else score=1
        fi
        scored+=("${score}.${RANDOM}:${zone}")
    done
    printf '%s\n' "${scored[@]}" | sort -t: -k1 -rn | sed 's/^[^:]*://'
}

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
    # Phase 0 G1: bound provision attempts within one invocation. Each attempt
    # creates a billed disk + VM; the cap stops a runaway zone cascade.
    LAUNCH_ATTEMPTS=$(( LAUNCH_ATTEMPTS + 1 ))
    if [[ "${LAUNCH_ATTEMPTS}" -gt "${MAX_LAUNCH_ATTEMPTS}" ]]; then
        log "ABORT: launch attempt cap reached (${MAX_LAUNCH_ATTEMPTS}). Raise SLM_YOYO_MAX_LAUNCH_ATTEMPTS to override."
        return 4   # caller must abort the campaign, not retry the next zone
    fi
    echo "  [PROVISION] Creating ${INSTANCE} in ${PROJECT}/${zone} (attempt ${LAUNCH_ATTEMPTS}/${MAX_LAUNCH_ATTEMPTS})..."

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
    # Phase 0 G3/G8: the VM-side dead-man's-switch reads max-lifetime-seconds and
    # self-stops the VM when the day's remaining budget would be spent.
    meta_kv+=("max-lifetime-seconds=${MAX_LIFETIME_SECONDS}")
    # G17: a fresh VM starts in the `running` state — no deliberate-stop tag.
    meta_kv+=("last-stop-reason=running")
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

    # SLM_YOYO_GCP_ZONE — use grep+sed||append; bare sed is silent if key absent.
    if grep -q "^SLM_YOYO_GCP_ZONE=" "${DOORMAN_ENV}"; then
        sed -i "s|^SLM_YOYO_GCP_ZONE=.*|SLM_YOYO_GCP_ZONE=${new_zone}|" "${DOORMAN_ENV}"
    else
        echo "SLM_YOYO_GCP_ZONE=${new_zone}" >> "${DOORMAN_ENV}"
    fi
    if ! grep -q "^SLM_YOYO_GCP_ZONE=${new_zone}$" "${DOORMAN_ENV}"; then
        log "ERROR: failed to write SLM_YOYO_GCP_ZONE to ${DOORMAN_ENV}"; return 1
    fi
    echo "Updated SLM_YOYO_GCP_ZONE=${new_zone} in ${DOORMAN_ENV}."

    if [[ -n "${new_endpoint}" ]]; then
        if grep -q "^SLM_YOYO_ENDPOINT=" "${DOORMAN_ENV}"; then
            sed -i "s|^SLM_YOYO_ENDPOINT=.*|SLM_YOYO_ENDPOINT=${new_endpoint}|" "${DOORMAN_ENV}"
        else
            echo "SLM_YOYO_ENDPOINT=${new_endpoint}" >> "${DOORMAN_ENV}"
        fi
        if ! grep -q "^SLM_YOYO_ENDPOINT=${new_endpoint}$" "${DOORMAN_ENV}"; then
            log "ERROR: failed to write SLM_YOYO_ENDPOINT to ${DOORMAN_ENV}"; return 1
        fi
        echo "Updated SLM_YOYO_ENDPOINT=${new_endpoint} in ${DOORMAN_ENV}."
    fi

    if [[ -n "${WEIGHTS_SNAPSHOT}" ]]; then
        if grep -q "^SLM_YOYO_WEIGHTS_SNAPSHOT=" "${DOORMAN_ENV}"; then
            sed -i "s|^SLM_YOYO_WEIGHTS_SNAPSHOT=.*|SLM_YOYO_WEIGHTS_SNAPSHOT=${WEIGHTS_SNAPSHOT}|" "${DOORMAN_ENV}"
        else
            echo "SLM_YOYO_WEIGHTS_SNAPSHOT=${WEIGHTS_SNAPSHOT}" >> "${DOORMAN_ENV}"
        fi
        if ! grep -q "^SLM_YOYO_WEIGHTS_SNAPSHOT=${WEIGHTS_SNAPSHOT}$" "${DOORMAN_ENV}"; then
            log "ERROR: failed to write SLM_YOYO_WEIGHTS_SNAPSHOT to ${DOORMAN_ENV}"; return 1
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
# Returns: 0 success, 1 hard failure (auth/permission), 3 stockout in all zones,
#          4 cost guardrail tripped (launch attempt cap).
attempt_start_once() {
    local known_zone err
    known_zone=$(current_vm_zone)

    if [[ -n "${known_zone}" ]]; then
        # Mode 1: VM exists — try to start it in its current zone
        log "Found ${INSTANCE} in ${PROJECT}/${known_zone}. Attempting start (Mode 1)..."
        # Phase 0 G3/G8: refresh the dead-man's-switch max-lifetime with today's
        # budget-derived value before the VM boots, so an existing VM honours
        # the remaining daily budget rather than a stale create-time value.
        # G17: also clear last-stop-reason to `running` — a fresh start cancels
        # any prior deliberate-stop tag so a later genuine preemption is still
        # recoverable by the idle monitor.
        gcloud compute instances add-metadata "${INSTANCE}" \
            --project="${PROJECT}" --zone="${known_zone}" \
            --metadata="max-lifetime-seconds=${MAX_LIFETIME_SECONDS},last-stop-reason=running" >/dev/null 2>&1 \
            || log "WARN: could not refresh instance metadata (deadman / idle monitor may use stale values)."
        err=$(gcloud compute instances start "${INSTANCE}" \
            --project="${PROJECT}" --zone="${known_zone}" 2>&1)
        if [[ $? -eq 0 ]]; then
            log "VM started in ${known_zone} (Mode 1: preemption recovery)."
            STARTED_ZONE="${known_zone}"
            update_doorman_env "${known_zone}"
            return 0
        fi
        if is_stockout "${err}"; then
            log "Zone ${known_zone} has no L4 capacity."
            if [[ "${ALLOW_ZONE_FALLBACK}" != "true" ]]; then
                log "Zone fallback disabled — waiting out stockout is the correct response."
                log "To perform a deliberate zone migration, run: start-yoyo.sh --enable-zone-fallback"
                return 3
            fi
            log "Falling through to Mode 2 (--enable-zone-fallback was passed)."
        else
            log "ERROR: failed to start ${INSTANCE} in ${known_zone}: ${err}"
            return 1
        fi
    else
        log "No existing ${INSTANCE} in project ${PROJECT}."
        if [[ "${ALLOW_ZONE_FALLBACK}" != "true" ]]; then
            log "Zone fallback disabled — no existing VM found and fallback not enabled."
            log "To perform a deliberate zone migration, run: start-yoyo.sh --enable-zone-fallback"
            return 3
        fi
        log "Entering Mode 2 (provision) — --enable-zone-fallback was passed."
    fi

    # Mode 2: provision a new VM in a time-scored fallback zone.
    # sorted_fallback_zones() ranks zones where it is currently night first —
    # lower commercial GPU demand means more L4 Spot capacity available.
    log "Zone order (top 3 by night-score): $(sorted_fallback_zones "${known_zone:-}" | head -3 | tr '\n' ' ')"
    local zone
    local prc
    while IFS= read -r zone; do
        log "Trying to provision ${INSTANCE} in ${PROJECT}/${zone} ..."
        provision_vm_in_zone "${zone}" >&2
        prc=$?
        if [[ "${prc}" -eq 0 ]]; then
            log "VM provisioned in ${zone} (Mode 2: zone relocation)."
            STARTED_ZONE="${zone}"
            update_doorman_env "${zone}"
            print_post_provision_steps "${zone}"
            return 0
        elif [[ "${prc}" -eq 4 ]]; then
            log "Launch attempt cap tripped during provision — aborting campaign."
            return 4
        fi
    done < <(sorted_fallback_zones "${known_zone:-}")

    log "All fallback zones exhausted in this cycle."
    return 3
}

# ─────────────────────────────────────────────────────────────────────────────
# Main — retry-cycle loop wraps Mode 1 + Mode 2; then optional wait-ready + snapshot
# ─────────────────────────────────────────────────────────────────────────────

log "Session start. instance=${INSTANCE} primary_zone=${PRIMARY_ZONE} retry_cycles=${RETRY_CYCLES} retry_wait=${RETRY_WAIT}s wait_ready=${WAIT_READY}s runtime=${RUNTIME_SECONDS}s auto_snapshot=${AUTO_SNAPSHOT}"
log "Cost guardrails: daily cap \$${DAILY_BUDGET_USD}, attempt cap ${MAX_LAUNCH_ATTEMPTS}, rate \$${RATE_USD_PER_HOUR}/hr."

# Phase 0 G8: book the previous run, refuse if today's $-cap is reached, then
# derive the VM max-lifetime from the remaining daily budget.
daily_reconcile
if ! daily_guard; then
    log "Daily budget exhausted — not starting the Yo-Yo. Exit 4."
    exit 4
fi
derive_max_lifetime

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
    elif [[ "${rc}" -eq 4 ]]; then
        log "Cost guardrail (launch attempt cap) tripped — aborting (exit 4)."
        exit 4
    fi
    # rc == 3: stockout cascade exhausted in this cycle — retry if cycles remain
    cycle=$(( cycle + 1 ))
done

if [[ -z "${STARTED_ZONE}" ]]; then
    log "ERROR: could not start or provision ${INSTANCE} in any zone after ${RETRY_CYCLES} cycle(s). Exit 3."
    exit 3
fi

# ── Runtime cap watchdog ──────────────────────────────────────────────────────
# Independent of the Doorman idle monitor: whichever fires first stops the VM.
# Note: this in-process watchdog dies with its parent shell — the VM-side
# dead-man's-switch (G3, max-lifetime-seconds metadata) is the durable backstop.
if [[ "${RUNTIME_SECONDS}" -gt 0 ]]; then
    log "Runtime cap: VM will auto-stop in ${RUNTIME_SECONDS}s ($(( RUNTIME_SECONDS / 60 )) min)."
    (
        sleep "${RUNTIME_SECONDS}"
        log "Runtime cap reached — stopping Yo-Yo VM now."
        "${SCRIPT_DIR}/stop-yoyo.sh" 2>&1 | sed 's/^/  [watchdog] /'
    ) &
    WATCHDOG_PID=$!
    log "Watchdog PID ${WATCHDOG_PID} armed."
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
