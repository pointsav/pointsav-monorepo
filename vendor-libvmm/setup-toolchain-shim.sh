#!/usr/bin/env bash
# setup-toolchain-shim.sh — local, no-sudo PATH shim for libvmm's build toolchain.
#
# libvmm's Makefiles hard-code unversioned names (clang, ld.lld, llvm-ar,
# llvm-as, llvm-objcopy, llvm-ranlib, dtc). This VM has clang-18's tools
# installed only under versioned names (llvm-ar-18, llvm-as-18, etc.) —
# ld.lld and clang themselves already have unversioned symlinks, but the
# other four don't. Installing unversioned system-wide symlinks is an
# apt/system-package-adjacent change requiring operator approval per
# AGENT.md's permission table ("ask first — system package changes");
# this script is the local, reversible, no-sudo alternative: a small
# directory of symlinks the caller prepends to PATH for the build only.
#
# Usage:
#   source vendor-libvmm/setup-toolchain-shim.sh
#   cd examples/simple && make MICROKIT_BOARD=qemu_virt_aarch64 \
#       MICROKIT_SDK=/opt/microkit-sdk-2.2.0 qemu
#
# Verified 2026-07-16: clang, ld.lld, dtc, qemu-system-aarch64 already have
# unversioned binaries/symlinks on this VM. iasl (ACPI compiler) is genuinely
# absent — not shimmed here since the qemu_virt_aarch64 target is
# device-tree-based (dtc), not ACPI-based; if a build step does invoke iasl
# for this target, that is itself a new finding, not something this script
# should silently paper over.

set -euo pipefail

SHIM_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/.toolchain-shim"
mkdir -p "$SHIM_DIR"

for versioned in llvm-ar-18 llvm-as-18 llvm-objcopy-18 llvm-ranlib-18; do
    unversioned="${versioned%-18}"
    src="/usr/bin/${versioned}"
    if [[ -x "$src" ]]; then
        ln -sf "$src" "${SHIM_DIR}/${unversioned}"
    else
        echo "WARNING: ${src} not found — ${unversioned} shim not created" >&2
    fi
done

export PATH="${SHIM_DIR}:${PATH}"
echo "Toolchain shim active: ${SHIM_DIR} prepended to PATH"
echo "Verify: $(command -v llvm-ar) $(command -v llvm-as) $(command -v llvm-objcopy) $(command -v llvm-ranlib)"
