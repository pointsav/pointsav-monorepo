#!/usr/bin/env bash
# One-shot nightly DataGraph rebuild session.
#
# Mimics what the nightly timers do when Yo-Yo #1 is live:
#   - Feeds CORPUS batches (jennifer + foundry-workspace) to service-content
#   - Stops after 4-hour wall clock OR 30-minute idle (no new SEMANTIC files)
#   - Runs corpus-threshold.py at session end to check DPO/SFT training triplets
#
# Run AFTER local-content.service is live.
#
# Usage:
#   ./scripts/nightly-run.sh [--batch-size N]
#
# Env vars:
#   JENNIFER_BASE          — jennifer deployment root (default: ~/deployments/...)
#   BATCH_SIZE             — corpus batch size per round (default: 50)
#   FOUNDRY_ROOT           — Foundry workspace root (default: ~/Foundry)

set -uo pipefail

JENNIFER_BASE="${JENNIFER_BASE:-/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer}"
FOUNDRY_ROOT="${FOUNDRY_ROOT:-${HOME}/Foundry}"
CRM_DIR="${JENNIFER_BASE}/service-fs/data/service-people/ledgers"
JENNIFER_SOURCE_DIR="${JENNIFER_BASE}/service-fs/data/service-people/source"
BATCH_SIZE=50
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Defaults for the loop's two timers — env vars override at process scope;
# --test-mode swaps the defaults to short values so the timers can be exercised
# in seconds rather than hours.
IDLE_SECONDS_DEFAULT=1800        # 30 minutes
HARD_STOP_SECONDS_DEFAULT=14400  # 4 hours

NO_YOYO=false
TEST_MODE=false
while [[ $# -gt 0 ]]; do
    case "$1" in
        --batch-size) BATCH_SIZE="$2"; shift 2 ;;
        --no-yoyo)    NO_YOYO=true; shift ;;
        --test-mode)  TEST_MODE=true; shift ;;
        --help|-h)    sed -n '2,18p' "$0"; exit 0 ;;
        *) echo "Unknown flag: $1" >&2; exit 1 ;;
    esac
done

if [[ "${TEST_MODE}" == "true" ]]; then
    IDLE_SECONDS_DEFAULT=30
    HARD_STOP_SECONDS_DEFAULT=60
fi

IDLE_SECONDS="${IDLE_SECONDS:-${IDLE_SECONDS_DEFAULT}}"
HARD_STOP_SECONDS="${HARD_STOP_SECONDS:-${HARD_STOP_SECONDS_DEFAULT}}"

DEADLINE=$(( $(date +%s) + HARD_STOP_SECONDS ))
LAST_SEMANTIC_COUNT=0
IDLE_SINCE=0
LOOP=0

log() { echo "[nightly-run $(date '+%H:%M:%S')] $*"; }

semantic_count() {
    find "${CRM_DIR}" -maxdepth 1 -name "SEMANTIC_*.json" 2>/dev/null | wc -l
}

log "Session start. Hard stop: $((HARD_STOP_SECONDS / 60))m. Idle timeout: $((IDLE_SECONDS / 60))m (=${IDLE_SECONDS}s). Batch size: ${BATCH_SIZE}. test_mode=${TEST_MODE} no_yoyo=${NO_YOYO}."
log "CRM dir: ${CRM_DIR}"
mkdir -p "${CRM_DIR}"

# ── Yo-Yo lifecycle: start (with wait-ready) ─────────────────────────────────
# Exit codes from start-yoyo.sh:
#   0 = VM up + vLLM ready
#   1 = GCE start failure (auth/permission/unknown)
#   2 = vLLM ready-poll timeout (VM up; falls back to Tier A only)
#   3 = zone stockout cascade exhausted
# We continue the loop in all cases — Doorman handles tier fallback transparently.
TIER_B_AVAILABLE=false
if [[ "${NO_YOYO}" != "true" ]]; then
    log "Starting Yo-Yo #1 (wait-ready=300s, auto-snapshot)..."
    if "${SCRIPT_DIR}/start-yoyo.sh" --wait-ready=300 --auto-snapshot 2>&1 | sed 's/^/  /'; then
        TIER_B_AVAILABLE=true
        log "Yo-Yo #1 ready — Tier B available."
    else
        rc=${PIPESTATUS[0]}
        log "WARN: start-yoyo.sh exited rc=${rc} (1=hard fail, 2=ready timeout, 3=stockout). Proceeding with Tier A fallback only."
    fi
else
    log "Skipping Yo-Yo lifecycle (--no-yoyo)."
fi

# ── Pre-flight: feed availability ────────────────────────────────────────────
JENNIFER_SOURCE_COUNT=0
if [[ -d "${JENNIFER_SOURCE_DIR}" ]]; then
    JENNIFER_SOURCE_COUNT=$(find "${JENNIFER_SOURCE_DIR}" -maxdepth 2 -type f 2>/dev/null | wc -l)
fi
log "Jennifer source files available: ${JENNIFER_SOURCE_COUNT} (in ${JENNIFER_SOURCE_DIR})"

LAST_SEMANTIC_COUNT=$(semantic_count)
log "Current SEMANTIC count: ${LAST_SEMANTIC_COUNT}"

# Run workspace feeder once at start
log "Running foundry-workspace-feeder (batch-size 20)..."
"${SCRIPT_DIR}/foundry-workspace-feeder.sh" --batch-size 20 2>&1 | sed 's/^/  /' || true

while true; do
    NOW=$(date +%s)

    # Hard stop
    if [[ "${NOW}" -ge "${DEADLINE}" ]]; then
        log "4-hour hard stop reached. Exiting."
        break
    fi

    REMAINING=$(( DEADLINE - NOW ))
    log "$(( REMAINING / 60 ))m remaining. Loop ${LOOP}."

    # Idle check
    CURRENT=$(semantic_count)
    DELTA=$(( CURRENT - LAST_SEMANTIC_COUNT ))

    if [[ "${DELTA}" -gt 0 ]]; then
        log "Progress: +${DELTA} SEMANTIC files (total: ${CURRENT}). Idle clock reset."
        LAST_SEMANTIC_COUNT="${CURRENT}"
        IDLE_SINCE=0
    else
        if [[ "${IDLE_SINCE}" -eq 0 ]]; then
            IDLE_SINCE="${NOW}"
            log "No new SEMANTIC files. Idle clock started (total: ${CURRENT})."
        else
            IDLE_ELAPSED=$(( NOW - IDLE_SINCE ))
            log "Idle ${IDLE_ELAPSED}s / ${IDLE_SECONDS}s (total SEMANTIC: ${CURRENT})."
            if [[ "${IDLE_ELAPSED}" -ge "${IDLE_SECONDS}" ]]; then
                log "30-minute idle timeout. Final SEMANTIC count: ${CURRENT}. Exiting."
                break
            fi
        fi
    fi

    # Drop next corpus batch
    log "Corpus batch (batch-size ${BATCH_SIZE})..."
    "${SCRIPT_DIR}/corpus-batch-jennifer.sh" --batch-size "${BATCH_SIZE}" 2>&1 | \
        grep -E "^\[corpus-batch\]|dropped|skipped|SKIP|OK|ERROR" | sed 's/^/  /' || true

    # Top up workspace feeder every 10 loops
    LOOP=$(( LOOP + 1 ))
    if (( LOOP % 10 == 0 )); then
        log "Workspace feeder top-up..."
        "${SCRIPT_DIR}/foundry-workspace-feeder.sh" --batch-size 20 2>&1 | sed 's/^/  /' || true
    fi

    log "Sleeping 60s..."
    sleep 60
done

FINAL=$(semantic_count)
log "Session complete. Final SEMANTIC count: ${FINAL}."

# ── Yo-Yo lifecycle: stop (graceful drain) ───────────────────────────────────
# Stops the GCE VM before the threshold-check Python step so cost stops
# accruing as early as possible. stop-yoyo.sh treats already-terminated as
# success, so this is safe even if start-yoyo failed earlier.
if [[ "${NO_YOYO}" != "true" ]]; then
    log "Stopping Yo-Yo #1 (drain-timeout 60s)..."
    "${SCRIPT_DIR}/stop-yoyo.sh" --drain-timeout=60 2>&1 | sed 's/^/  /' || true
fi

# ── Training triplet threshold check ─────────────────────────────────────────
# Run corpus-threshold.py every night so training fires as soon as enough
# DPO/SFT tuples have accumulated — not just on the Sunday cron.
THRESHOLD_SCRIPT="${SCRIPT_DIR}/corpus-threshold.py"
if [[ -f "${THRESHOLD_SCRIPT}" ]]; then
    log "Running corpus-threshold check (training triplets)..."
    FOUNDRY_ROOT="${FOUNDRY_ROOT}" python3 "${THRESHOLD_SCRIPT}" 2>&1 | sed 's/^/  /' || true
else
    log "WARN: corpus-threshold.py not found at ${THRESHOLD_SCRIPT} — skipping."
fi
