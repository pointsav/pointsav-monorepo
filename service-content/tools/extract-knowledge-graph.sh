#!/usr/bin/env bash
set -euo pipefail

if [ -z "${REMOTE_TARGET:-}" ] || [ -z "${TOTEBOX_ROOT:-}" ]; then
    echo "[ERROR] Environment variables REMOTE_TARGET and TOTEBOX_ROOT must be set."
    exit 1
fi

LOCAL_EXPORT_DIR="${LOCAL_EXPORT_DIR:-./Sovereign-Exports/Content}"

echo "[SYSTEM] Guaranteeing remote vault boundaries exist..."
ssh "$REMOTE_TARGET" "mkdir -p \"${TOTEBOX_ROOT}/service-content/knowledge-graph\" \"${TOTEBOX_ROOT}/service-content/verified-ledger\" \"${TOTEBOX_ROOT}/service-slm/transient-queues\""

echo "[SYSTEM] Extracting Raw, Transient, and Verified Content Ledgers..."
mkdir -p "$LOCAL_EXPORT_DIR/knowledge-graph" "$LOCAL_EXPORT_DIR/verified-ledger" "$LOCAL_EXPORT_DIR/raw-transient-queues"

# Pull the Transient Queues (The raw text bodies from the Harvester)
rsync -avz "$REMOTE_TARGET:${TOTEBOX_ROOT}/service-slm/transient-queues/" "$LOCAL_EXPORT_DIR/raw-transient-queues/"

# Pull the standard Knowledge Graph
rsync -avz "$REMOTE_TARGET:${TOTEBOX_ROOT}/service-content/knowledge-graph/" "$LOCAL_EXPORT_DIR/knowledge-graph/"
rsync -avz "$REMOTE_TARGET:${TOTEBOX_ROOT}/service-content/verified-ledger/" "$LOCAL_EXPORT_DIR/verified-ledger/"

echo "[SUCCESS] Full Content Pipeline extracted safely to: $LOCAL_EXPORT_DIR"
