#!/bin/bash
# Start the os-totebox service stack in dependency order.
# Run inside an os-totebox VM after binaries are installed.
#
# Env overrides:
#   TOTEBOX_BIN_DIR   — directory containing installed binaries (default: /usr/local/bin)
#   TOTEBOX_DATA_DIR  — data root passed to services (default: /var/lib/pointsav)
#   TOTEBOX_LOG_DIR   — log directory (default: /var/log/pointsav)
#   TOTEBOX_PID_DIR   — PID file directory (default: /run/pointsav)
#
# For dev on the workspace host, set TOTEBOX_BIN_DIR to the cargo target:
#   TOTEBOX_BIN_DIR=$(cargo build --release -q 2>&1; echo target/release) \
#     ./os-totebox/scripts/start-stack.sh

set -euo pipefail

BIN_DIR="${TOTEBOX_BIN_DIR:-/usr/local/bin}"
DATA_DIR="${TOTEBOX_DATA_DIR:-/var/lib/pointsav}"
LOG_DIR="${TOTEBOX_LOG_DIR:-/var/log/pointsav}"
PID_DIR="${TOTEBOX_PID_DIR:-/run/pointsav}"

HEALTH_TIMEOUT=15   # seconds to wait for each HTTP service
HEALTH_INTERVAL=1   # poll interval in seconds

mkdir -p "$LOG_DIR" "$PID_DIR"

log() { echo "[start-stack] $*"; }
die() { echo "[start-stack] FATAL: $*" >&2; exit 1; }

wait_http() {
    local name="$1" url="$2"
    local elapsed=0
    log "  waiting for $name at $url ..."
    while ! curl -sf "$url" >/dev/null 2>&1; do
        sleep "$HEALTH_INTERVAL"
        elapsed=$(( elapsed + HEALTH_INTERVAL ))
        if [ "$elapsed" -ge "$HEALTH_TIMEOUT" ]; then
            die "$name did not become healthy within ${HEALTH_TIMEOUT}s"
        fi
    done
    log "  $name ready"
}

start_svc() {
    local name="$1"; shift
    local pid_file="$PID_DIR/$name.pid"
    if [ -f "$pid_file" ] && kill -0 "$(cat "$pid_file")" 2>/dev/null; then
        log "$name already running (pid $(cat "$pid_file"))"
        return 0
    fi
    log "starting $name ..."
    "$@" >> "$LOG_DIR/$name.log" 2>&1 &
    echo $! > "$pid_file"
    log "$name started (pid $!)"
}

log "============================================================"
log " os-totebox service stack — startup"
log "============================================================"

# ── Tier 0: verify binaries present ──────────────────────────────────────────
for bin in slm-doorman-server service-content server service-extraction; do
    [ -x "$BIN_DIR/$bin" ] || die "binary not found: $BIN_DIR/$bin"
done
log "all binaries found"

# ── Tier 1: service-slm (Doorman — inference gateway) ────────────────────────
# Must start before service-content (content uses Doorman for Tier C drafts).
start_svc service-slm \
    env SLM_BIND_ADDR="127.0.0.1:9080" \
        SLM_LOCAL_ENDPOINT="http://127.0.0.1:8080" \
        SLM_MODULE_ID="data" \
        "$BIN_DIR/slm-doorman-server"
wait_http service-slm "http://127.0.0.1:9080/health"

# ── Tier 2: service-content (DataGraph / LadybugDB) ──────────────────────────
start_svc service-content \
    env SERVICE_CONTENT_HTTP_BIND="127.0.0.1:9081" \
        SERVICE_CONTENT_DATA_DIR="$DATA_DIR/service-content" \
        "$BIN_DIR/service-content"
wait_http service-content "http://127.0.0.1:9081/health"

# ── Tier 3: service-people (personnel ledger HTTP API) ───────────────────────
start_svc service-people \
    env SERVICE_PEOPLE_PORT="9091" \
        SERVICE_PEOPLE_LEDGER_PATH="$DATA_DIR/service-people/ledger_personnel.json" \
        "$BIN_DIR/server"
wait_http service-people "http://127.0.0.1:9091/v1/people"

# ── Tier 4: service-extraction (filesystem watcher — no HTTP surface) ────────
# service-extraction tails ingestion queues and writes to service-content.
# No health endpoint; process start is the signal.
start_svc service-extraction \
    "$BIN_DIR/service-extraction"
sleep 1
pid_file="$PID_DIR/service-extraction.pid"
kill -0 "$(cat "$pid_file")" 2>/dev/null \
    || die "service-extraction exited immediately; check $LOG_DIR/service-extraction.log"
log "service-extraction running"

# ── service-fs note ───────────────────────────────────────────────────────────
# service-fs is a no_std seL4 unikernel (WORM storage enforcer). It is started
# by the seL4 root task, not by this script. Verify its protection domain is
# active via the seL4 IPC monitor before routing writes to it.

log "============================================================"
log " stack ready"
log "   service-slm       http://127.0.0.1:9080"
log "   service-content   http://127.0.0.1:9081"
log "   service-people    http://127.0.0.1:9091"
log "   service-extraction  (background watcher)"
log "   service-fs          (seL4 protection domain — verify separately)"
log "PID files: $PID_DIR/"
log "Logs:      $LOG_DIR/"
log "============================================================"
