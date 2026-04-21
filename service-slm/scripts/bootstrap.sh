#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
# Copyright (c) 2026 Woodfine Capital Projects Inc.
#
# First-time developer setup. Run this after cloning the repository.

set -euo pipefail

cd "$(dirname "$0")/.."

echo "→ Verifying rustup is installed"
if ! command -v rustup >/dev/null 2>&1; then
    echo "rustup not found. Install from https://rustup.rs/ then re-run this script."
    exit 1
fi

echo "→ Installing pinned Rust toolchain from rust-toolchain.toml"
rustup show

echo "→ Installing supplementary tools"
for tool in cargo-audit cargo-deny cargo-sbom cargo-about; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "  installing $tool"
        cargo install "$tool"
    else
        echo "  $tool already installed"
    fi
done

echo "→ Fetching dependencies"
cargo fetch

echo
echo "✓ Bootstrap complete."
echo "  Next: run ./scripts/check-all.sh to verify the workspace builds green."
