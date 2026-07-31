#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
#
# Builds the seL4 kernel for qemu-arm-virt/aarch64 — the target the raw-Rust
# H1-H8 moonshot-sel4-vmm ladder (and moonshot-toolkit's assemble-image step)
# actually needs. See BRIEF-os-totebox-platform.md Section 16 Phase 4 for the
# full story: the kernel.elf that shipped in this repo's `build/` directory
# was built for KernelPlatform=pc99 / KernelSel4Arch=x86_64 (a different
# vendoring, from a stale path — CMAKE_HOME_DIRECTORY in its CMakeCache.txt
# points at a pre-rename workspace layout), not qemu-arm-virt/aarch64. That
# mismatch — not a missing/wrong device-tree-blob — was the real reason H8
# stopped reproducing: the elfloader would start, find *a* DTB, then hang,
# because the kernel payload itself was the wrong CPU architecture.
#
# This script builds a correct aarch64 kernel into build-aarch64-qemu/ (a
# fresh directory — the original build/ tree, with whatever it's actually
# used for, is left untouched) and symlinks vendor-sel4-kernel/build/
# aarch64-qemu/{kernel.elf,kernel.dtb,autoconf,gen_config} at it, which is
# the exact path moonshot-toolkit/src/main.rs's KERNEL_BUILD constant
# expects.
#
# Usage: run once (or whenever you need a fresh kernel):
#   vendor-sel4-kernel/build-aarch64-qemu.sh
# Then moonshot-toolkit's documented build command works as written:
#   cargo run --manifest-path moonshot-toolkit/Cargo.toml -- \
#     build moonshot-toolkit/examples/<some-phase>.toml

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$ROOT/build-aarch64-qemu"
LINK_DIR="$ROOT/build/aarch64-qemu"

# seL4's build tooling needs a handful of Python packages
# (jinja2, ply, pyyaml, pyfdt, lxml) and the Ninja generator. None of these
# are installed system-wide on this host (and shouldn't be — PEP 668 /
# externally-managed-environment blocks a bare `pip install` anyway). A
# throwaway venv is cheap to rebuild (~15s) and keeps this fully
# self-contained; no system package changes, no sudo.
VENV_DIR="${SEL4_BUILD_VENV:-/tmp/sel4-build-venv}"
if [ ! -x "$VENV_DIR/bin/ninja" ]; then
  python3 -m venv "$VENV_DIR"
  "$VENV_DIR/bin/pip" install --quiet ninja jinja2 ply pyyaml pyfdt lxml
fi
export PATH="$VENV_DIR/bin:$PATH"

cmake -S "$ROOT/src" -B "$BUILD_DIR" \
  -DCMAKE_TOOLCHAIN_FILE="$ROOT/src/gcc.cmake" \
  -DCROSS_COMPILER_PREFIX=aarch64-linux-gnu- \
  -DKernelPlatform=qemu-arm-virt \
  -DKernelSel4Arch=aarch64 \
  -DKernelVerificationBuild=OFF \
  -DKernelDebugBuild=ON \
  -DKernelPrinting=ON \
  -G Ninja \
  --fresh

ninja -C "$BUILD_DIR"

mkdir -p "$LINK_DIR"
ln -sfn "$BUILD_DIR/kernel.elf" "$LINK_DIR/kernel.elf"
ln -sfn "$BUILD_DIR/qemu-arm-virt.dtb" "$LINK_DIR/kernel.dtb"
# NOTE: -sfn (not -sf) is required here — autoconf/gen_config are directories,
# and `ln -sf` on a symlink that resolves to an existing directory silently
# writes *inside* that directory instead of replacing the symlink. -n treats
# the link name itself as the target, which is what we actually want.
ln -sfn "$BUILD_DIR/autoconf" "$LINK_DIR/autoconf"
ln -sfn "$BUILD_DIR/gen_config" "$LINK_DIR/gen_config"

echo "aarch64/qemu-arm-virt kernel ready: $LINK_DIR -> $BUILD_DIR"
file "$BUILD_DIR/kernel.elf"
