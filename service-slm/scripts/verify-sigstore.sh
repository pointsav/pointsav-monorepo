#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
# Copyright (c) 2026 Woodfine Capital Projects Inc.
#
# Verify the Sigstore signature on a downloaded release artefact.
# Usage: verify-sigstore.sh <path/to/slm-cli-binary>

set -euo pipefail

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <path-to-slm-cli-binary>"
    exit 2
fi

BINARY="$1"

if [ ! -f "$BINARY" ]; then
    echo "error: $BINARY not found"
    exit 2
fi

if ! command -v cosign >/dev/null 2>&1; then
    echo "error: cosign not installed. See https://docs.sigstore.dev/cosign/installation/"
    exit 2
fi

SIG="$BINARY.sig"
CERT="$BINARY.pem"

for f in "$SIG" "$CERT"; do
    if [ ! -f "$f" ]; then
        echo "error: $f not found (expected alongside the binary)"
        exit 2
    fi
done

echo "→ Verifying $BINARY against $SIG and $CERT"
cosign verify-blob \
    --signature "$SIG" \
    --certificate "$CERT" \
    --certificate-identity-regexp 'https://github.com/woodfinegroup/.+' \
    --certificate-oidc-issuer 'https://token.actions.githubusercontent.com' \
    "$BINARY"

echo "✓ Signature verified."
