#!/usr/bin/env bash
# ==============================================================================
# Script: trigger_extraction.sh (Remote Side)
# Purpose: Orchestrate the Rust engine on GCP and anchor results to the Data Mesh.
# ==============================================================================

FILENAME=$1
PROTOCOL=$2
SILO_PATH=$3

# Paths relative to the GCP VM environment
REMOTE_ROOT="/home/foundry/node-gcp-free"
ENGINE_DIR="${REMOTE_ROOT}/factory-pointsav/pointsav-monorepo/service-content"
PROTO_FILE="${REMOTE_ROOT}/factory-pointsav/pointsav-design-system/tokens/linguistic/protocol-${PROTOCOL,,}.yaml"
OUTBOX="${ENGINE_DIR}/outbox"

echo "⚙️  REMOTE ENGINE: Processing $FILENAME via $PROTOCOL..."

# 1. Execute the Service-Content Engine (Rust)
cd "$ENGINE_DIR"
if [ -f "./target/release/service-content" ]; then
    ./target/release/service-content "$PROTO_FILE" "./input/$FILENAME" "./outbox"
else
    echo "⚠️  Release binary not found. Falling back to 'cargo run'..."
    cargo run --release -- "$PROTO_FILE" "./input/$FILENAME" "./outbox"
fi

# 2. Anchor to the Data Mesh (Persistence Layer)
LATEST_ARTIFACT=$(ls -t "$OUTBOX" | head -n 1)

if [ -n "$LATEST_ARTIFACT" ]; then
    echo "⚓ ANCHORING: Moving $LATEST_ARTIFACT to $SILO_PATH"
    mkdir -p "$SILO_PATH"
    cp "$OUTBOX/$LATEST_ARTIFACT" "$SILO_PATH/"
    echo "✅ ANCHORED: $LATEST_ARTIFACT is now part of the Machine-Readable Mesh."
else
    echo "❌ ERROR: No artifact was generated in the outbox."
    exit 1
fi

echo "✅ REMOTE EXECUTION COMPLETE."
