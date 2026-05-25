#!/usr/bin/env bash
# On-demand Yo-Yo #1 stop with optional drain + snapshot.
#
# Under normal operation the Doorman idle monitor stops the VM automatically
# after SLM_YOYO_IDLE_MINUTES (default 30) of vLLM-reachable-zero-active-slots.
# This script is for explicit lifecycle control (e.g. nightly-run.sh end-of-
# session, cost emergency, maintenance).
#
# Drain: --drain-timeout=N sleeps N seconds before stopping so in-flight Tier B
# requests have time to complete. Simple wait; no Doorman control surface
# required.
#
# Snapshot: --snapshot-before-stop calls create-yoyo-snapshot.sh prior to stop,
# preserving the weights disk's current state for zone-migration recovery.
#
# Exit codes:
#   0 — VM stopped + verified TERMINATED (or already stopped)
#   1 — gcloud stop failure, or the VM did not reach TERMINATED within 120s
#
# Usage:
#   ./scripts/stop-yoyo.sh
#   ./scripts/stop-yoyo.sh --drain-timeout=60
#   ./scripts/stop-yoyo.sh --drain-timeout=60 --snapshot-before-stop
#   SLM_YOYO_GCP_INSTANCE=yoyo-tier-b-2 ./scripts/stop-yoyo.sh
set -uo pipefail

PROJECT="${SLM_YOYO_GCP_PROJECT:-woodfine-node-gcp-free}"
if [[ -z "${SLM_YOYO_GCP_ZONE:-}" ]] && [[ -r /etc/local-doorman/local-doorman.env ]]; then
    SLM_YOYO_GCP_ZONE=$(grep '^SLM_YOYO_GCP_ZONE=' /etc/local-doorman/local-doorman.env | cut -d= -f2- | head -1)
fi
ZONE="${SLM_YOYO_GCP_ZONE:-europe-west4-a}"
INSTANCE="${SLM_YOYO_GCP_INSTANCE:-yoyo-tier-b-1}"
LIFECYCLE_LOG="${SLM_YOYO_LIFECYCLE_LOG:-/var/log/yoyo-lifecycle.log}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# ── Flag parsing ─────────────────────────────────────────────────────────────
DRAIN_TIMEOUT=0
SNAPSHOT_BEFORE_STOP=false
while [[ $# -gt 0 ]]; do
    case "$1" in
        --drain-timeout=*)         DRAIN_TIMEOUT="${1#*=}"; shift ;;
        --drain-timeout)           DRAIN_TIMEOUT="$2"; shift 2 ;;
        --snapshot-before-stop)    SNAPSHOT_BEFORE_STOP=true; shift ;;
        --help|-h)
            sed -n '2,25p' "$0"
            exit 0
            ;;
        *) echo "Unknown flag: $1" >&2; exit 1 ;;
    esac
done

# ── Lifecycle logging ────────────────────────────────────────────────────────
log() {
    local ts msg
    ts="$(date -u +'%Y-%m-%dT%H:%M:%SZ')"
    msg="[stop-yoyo ${ts}] $*"
    echo "${msg}"
    if [[ -w "$(dirname "${LIFECYCLE_LOG}")" ]] || [[ -w "${LIFECYCLE_LOG}" ]] 2>/dev/null; then
        echo "${msg}" >> "${LIFECYCLE_LOG}" 2>/dev/null || true
    fi
}

# ── Drain ────────────────────────────────────────────────────────────────────
if [[ "${DRAIN_TIMEOUT}" -gt 0 ]]; then
    log "Drain: waiting ${DRAIN_TIMEOUT}s for in-flight Tier B requests to settle..."
    sleep "${DRAIN_TIMEOUT}"
fi

# ── Optional pre-stop snapshot ───────────────────────────────────────────────
if [[ "${SNAPSHOT_BEFORE_STOP}" == "true" ]]; then
    log "Pre-stop snapshot: invoking create-yoyo-snapshot.sh..."
    if [[ -x "${SCRIPT_DIR}/create-yoyo-snapshot.sh" ]]; then
        "${SCRIPT_DIR}/create-yoyo-snapshot.sh" 2>&1 | sed 's/^/  /' || \
            log "WARN: snapshot creation reported failure — continuing with stop."
    else
        log "WARN: ${SCRIPT_DIR}/create-yoyo-snapshot.sh not found or not executable — skipping."
    fi
fi

# ── Stop ─────────────────────────────────────────────────────────────────────
# G17: tag the stop as deliberate (operator) BEFORE stopping, so the Doorman
# idle monitor reads `last-stop-reason=operator` and treats the VM as sticky —
# a deliberate stop must never be silently auto-restarted.
gcloud compute instances add-metadata "${INSTANCE}" \
    --project="${PROJECT}" --zone="${ZONE}" \
    --metadata=last-stop-reason=operator >/dev/null 2>&1 \
    || log "WARN: could not set last-stop-reason metadata (idle monitor may auto-restart)."

log "Stopping ${INSTANCE} in ${PROJECT}/${ZONE} ..."
err=$(gcloud compute instances stop "${INSTANCE}" \
    --project="${PROJECT}" --zone="${ZONE}" 2>&1)
rc=$?
if [[ "${rc}" -ne 0 ]]; then
    if echo "${err}" | grep -qiE "already.*(stopped|terminated)|in state TERMINATED"; then
        log "VM already stopped/terminated — treating as success."
        exit 0
    fi
    log "ERROR: gcloud stop failed (rc=${rc}): ${err}"
    exit 1
fi

# G10: `instances stop` returns when the operation is ACCEPTED, not when the VM
# is actually down. Poll the instance status until it reaches TERMINATED before
# claiming success — "shut down" must mean verified-down, not just requested.
log "Stop request accepted — verifying the VM reaches TERMINATED..."
verify_deadline=$(( $(date +%s) + 120 ))
status=""
while [[ $(date +%s) -lt ${verify_deadline} ]]; do
    status=$(gcloud compute instances describe "${INSTANCE}" \
        --project="${PROJECT}" --zone="${ZONE}" \
        --format='value(status)' 2>/dev/null || echo "")
    if [[ "${status}" == "TERMINATED" || "${status}" == "STOPPED" ]]; then
        log "VM verified ${status}."
        exit 0
    fi
    sleep 5
done
log "ERROR: VM did not reach TERMINATED within 120s (last status: ${status:-unknown}). Manual check required."
exit 1
