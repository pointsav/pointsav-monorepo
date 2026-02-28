#!/usr/bin/env bash
# ==============================================================================
# Script: forge_iso.sh (Workspace-Aware Assembly)
# Product: os-infrastructure (Node 1 - Muscle)
# Purpose: Compiles and extracts the binary from the Workspace Root.
# ==============================================================================

set -euo pipefail

# In a Workspace, the target folder is at the Monorepo Root
WORKSPACE_ROOT="/home/mathew/Foundry/factory-pointsav/pointsav-monorepo"
PRODUCT_DIR="${WORKSPACE_ROOT}/os-infrastructure"
BUILD_DIR="${PRODUCT_DIR}/build_iso"
TARGET="x86_64-unknown-none"

echo "[INFO] Starting Workspace Forge for os-infrastructure..."

# 1. Compile from Workspace Root
cd "${WORKSPACE_ROOT}"
cargo build --release --target "${TARGET}" -p os-infrastructure --config "target.x86_64-unknown-none.rustflags=['-C', 'link-arg=-Tos-infrastructure/linker.ld']"

# 2. Extract Binary from WORKSPACE_ROOT/target
BINARY="${WORKSPACE_ROOT}/target/${TARGET}/release/os-infrastructure"
cp "${BINARY}" "${BUILD_DIR}/os-infrastructure.elf"

echo "[INFO] Binary successfully extracted to: ${BUILD_DIR}/os-infrastructure.elf"
echo "[SUCCESS] Product ready for ISO wrapping."
