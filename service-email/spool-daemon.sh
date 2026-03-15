#!/usr/bin/env bash
set -euo pipefail

echo "========================================================"
echo " ⚙️ POINTSAV SPOOL DAEMON: ACTIVE WATCHDOG"
echo "========================================================"

if [ "$#" -ne 2 ]; then
    echo "[ERROR] Usage: ./spool-daemon.sh <TOTEBOX_ROOT> <PATH_TO_MIME_SPLINTER_BIN>"
    exit 1
fi

TOTEBOX_ROOT="$1"
ENGINE_BIN="$2"

NEW_DIR="$TOTEBOX_ROOT/service-email/personnel-maildir/new"
CUR_DIR="$TOTEBOX_ROOT/service-email/personnel-maildir/cur"

# Ensure physical WORM boundaries exist
mkdir -p "$NEW_DIR" "$CUR_DIR"

echo "[SYSTEM] Daemon armed. Watching: $NEW_DIR"
echo "--------------------------------------------------------"

# Lightweight continuous polling loop
while true; do
    # Find all .eml files in the new/ directory safely
    for EML_FILE in "$NEW_DIR"/*.eml; do
        # Check if the file physically exists
        if [ -f "$EML_FILE" ]; then
            FILENAME=$(basename "$EML_FILE")
            echo "[DETECTED] Inbound asset: $FILENAME"
            
            # 1. Fire the Rust Forensic Engine
            "$ENGINE_BIN" "$EML_FILE" "$TOTEBOX_ROOT"
            
            # 2. Vault the asset (Relocate to cur/)
            mv "$EML_FILE" "$CUR_DIR/"
            echo "[VAULTED] $FILENAME secured in immutable storage."
            echo "--------------------------------------------------------"
        fi
    done
    
    sleep 5
done
