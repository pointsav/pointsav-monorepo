#!/usr/bin/env bash
# Purpose: Remote Orchestration of Rust Engine and Data Mesh Anchoring.

FILENAME=$1
PROTOCOL=$2
SILO_PATH=$3

# VM Pathing
ENGINE_DIR="/home/foundry/node-gcp-free/factory-pointsav/pointsav-monorepo/service-content"
DNA_PATH="/home/foundry/node-gcp-free/factory-pointsav/pointsav-design-system/tokens/linguistic"
PROTO_FILE="${DNA_PATH}/protocol-${PROTOCOL,,}.yaml"

echo "⚙️  SURGICAL ENGINE: Processing $FILENAME via $PROTOCOL..."

cd "$ENGINE_DIR"
if [ -f "./target/release/service-content" ]; then
    ./target/release/service-content "$PROTO_FILE" "./input/$FILENAME" "./outbox"
else
    echo "⚠️  Release binary not found. Running cargo..."
    cargo run --release -- "$PROTO_FILE" "./input/$FILENAME" "./outbox"
fi

# Anchor to Mesh
LATEST=$(ls -t ./outbox | head -n 1)
if [ -n "$LATEST" ]; then
    echo "⚓ ANCHORING: Moving $LATEST to $SILO_PATH"
    mkdir -p "$SILO_PATH"
    cp "./outbox/$LATEST" "$SILO_PATH/"
fi
echo "✅ REMOTE EXECUTION COMPLETE."
