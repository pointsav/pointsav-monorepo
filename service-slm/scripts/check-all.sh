#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
# Copyright (c) 2026 Woodfine Capital Projects Inc.
#
# Run the full local check suite: fmt, clippy, test, audit, deny.
# Matches what CI runs. Use this before opening a PR.

set -euo pipefail

cd "$(dirname "$0")/.."

echo "→ cargo fmt --all -- --check"
cargo fmt --all -- --check

echo "→ cargo clippy --workspace --all-targets --all-features -- -D warnings"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "→ cargo test --workspace --all-features"
cargo test --workspace --all-features

if command -v cargo-audit >/dev/null 2>&1; then
    echo "→ cargo audit"
    cargo audit
else
    echo "→ cargo audit (skipped: not installed; \`cargo install cargo-audit\`)"
fi

if command -v cargo-deny >/dev/null 2>&1; then
    # Matches .github/workflows/ci.yml: runs `bans`, `sources`, and `licenses`.
    # `advisories` is deferred — see TASKS.md [33] (depends on MSRV bump).
    echo "→ cargo deny check bans sources licenses"
    cargo deny check bans sources licenses
else
    echo "→ cargo deny check (skipped: not installed; \`cargo install cargo-deny\`)"
fi

echo
echo "✓ All checks passed."
