#!/usr/bin/env bash
set -euo pipefail

TOTEBOX_ROOT="$1"
ENGINE_BIN="$2"
NEW_DIR="$TOTEBOX_ROOT/service-email/personnel-maildir/new"
CUR_DIR="$TOTEBOX_ROOT/service-email/personnel-maildir/cur"

mkdir -p "$NEW_DIR" "$CUR_DIR"
echo "[SYSTEM] Daemon armed with EPHEMERAL logic. Watching: $NEW_DIR"

while true; do
    for EML_FILE in "$NEW_DIR"/*.eml; do
        if [ -f "$EML_FILE" ]; then
            FILENAME=$(basename "$EML_FILE")
            echo "[DETECTED] Inbound asset: $FILENAME"
            
            # 1. Fire the Rust Forensic Engine
            "$ENGINE_BIN" "$EML_FILE" "$TOTEBOX_ROOT"
            
            # 2. Ephemeral vs. Permanent Vaulting Logic
            if [[ "$FILENAME" == NOSAVE_* ]]; then
                rm -f "$EML_FILE"
                echo "[OBLITERATED] Ephemeral asset $FILENAME destroyed (NO SAVE)."
            else
                mv "$EML_FILE" "$CUR_DIR/"
                echo "[VAULTED] $FILENAME secured in immutable storage."
            fi
            echo "--------------------------------------------------------"
        fi
    done
    sleep 5
done
