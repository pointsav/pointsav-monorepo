#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

# build-guest-kernel.sh — CONFIG_UNIX=y rebuild of vendor-libvmm's shared
# example guest kernel, adopted directly from project-totebox's own
# already-proven fix (BRIEF-os-totebox-platform.md, 2026-07-29 session),
# not re-derived independently — matches the operator's standing instruction
# to check project-totebox's BRIEFs for how they resolve shared-plumbing
# gaps and adopt their fix rather than maintaining a second workaround.
#
# Root cause (identical on this archive's copy of the same vendored example,
# confirmed 2026-08-26): vendor-libvmm/examples/virtio's shared example
# guest kernel (Linux v6.13, normally auto-fetched prebuilt from
# trustworthy.systems) has `# CONFIG_UNIX is not set` in client_vm/
# linux_config — that example's own tiny BusyBox demo never needed AF_UNIX,
# but tokio's signal driver does (`UnixStream` for its self-pipe signal
# mechanism) — any real tokio-based service panics at startup with
# "failed to create UnixStream: Os { code: 97, ... 'Address family not
# supported by protocol' }". Hit live booting os-mediakit's G3 (real
# app-mediakit-knowledge binary) under this same shared example.
#
# Fix: real kernel rebuild, not a workaround — same recipe project-totebox
# already verified twice (diagnostic + production binary boots, zero
# panic). This script isolates the build under os-mediakit's own BUILD_DIR
# (not the shared default `build/`) per Phase 0's BUILD_DIR-isolation
# decision, unlike project-totebox's own original ad-hoc run.
#
# Requires: flex, bison, libssl-dev, libelf-dev, aarch64-linux-gnu-gcc (all
# confirmed already installed on this host 2026-08-26 — no apt-get needed).
#
# Usage: run from os-mediakit/ (same convention as the other build scripts):
#   bash scripts/build-guest-kernel.sh
set -euo pipefail

BUILD_DIR="build-os-mediakit"
LIBVMM_VIRTIO_DIR="../vendor-libvmm/examples/virtio"
LINUX_TAG="v6.13"
KERNEL_SRC="${LIBVMM_VIRTIO_DIR}/${BUILD_DIR}/guest-kernel-src/linux"
# The exact filename the Makefile's LINUX var expects — matching it exactly
# means the Makefile's own curl-download rule sees the file as already
# present and never overwrites it with the stock (CONFIG_UNIX-less) kernel.
LINUX_FILENAME="a3f4bf9e2eb24fa8fc0d3d8cd02e4d8097062e8b-linux"
LINUX_CONFIG_SRC="${LIBVMM_VIRTIO_DIR}/client_vm/linux_config"
OUTPUT="${LIBVMM_VIRTIO_DIR}/${BUILD_DIR}/${LINUX_FILENAME}"

for CMD in flex bison aarch64-linux-gnu-gcc git; do
    command -v "${CMD}" >/dev/null || { echo "error: ${CMD} not found on PATH"; exit 1; }
done
[ -f "${LINUX_CONFIG_SRC}" ] || {
    echo "error: ${LINUX_CONFIG_SRC} not found — run from os-mediakit/" >&2
    exit 1
}

# ── Preflight resource guard — same thresholds as the other os-mediakit
# build scripts. A kernel build is CPU-heavy for ~13 minutes (project-
# totebox's own measured time); hard-abort under host pressure, not warn.
_mem_total_kb=$(awk '/^MemTotal:/{print $2}' /proc/meminfo)
_mem_avail_kb=$(awk '/^MemAvailable:/{print $2}' /proc/meminfo)
_mem_used_pct=$(awk "BEGIN{printf \"%.1f\", (${_mem_total_kb}-${_mem_avail_kb})/${_mem_total_kb}*100}")
_psi_mem_some=$(awk '/^some/ {for(i=1;i<=NF;i++) if($i~/^avg10=/) {sub(/avg10=/,"",$i); print $i; exit}}' \
    /sys/fs/cgroup/memory.pressure 2>/dev/null || echo "0.00")
read -r _load_1 _ _ _ < /proc/loadavg
_nproc_count=$(nproc 2>/dev/null || echo 1)
if awk "BEGIN{exit !(${_mem_used_pct} > 85 || ${_psi_mem_some} > 10 || ${_load_1} >= ${_nproc_count})}"; then
    if [ "${FOUNDRY_RESOURCE_GUARD_BYPASS:-0}" = "1" ]; then
        echo "  WARN: host under pressure (mem_used_pct=${_mem_used_pct}% psi=${_psi_mem_some} load_1=${_load_1}/${_nproc_count}) — FOUNDRY_RESOURCE_GUARD_BYPASS=1 set, proceeding anyway (operator-approved 2026-08-26)" >&2
    else
        echo "error: refusing to start the guest kernel build — host already under pressure" >&2
        echo "  mem_used_pct=${_mem_used_pct}% psi_mem_some_avg10=${_psi_mem_some} load_1=${_load_1} nproc=${_nproc_count}" >&2
        echo "  wait for load/memory to settle and rerun -- this is a hard gate, not a warning" >&2
        exit 1
    fi
else
    echo "  preflight OK: mem_used_pct=${_mem_used_pct}% psi=${_psi_mem_some} load_1=${_load_1}/${_nproc_count}"
fi

mkdir -p "$(dirname "${KERNEL_SRC}")"
if [ ! -d "${KERNEL_SRC}/.git" ]; then
    echo "  shallow-cloning torvalds/linux.git at ${LINUX_TAG}..."
    git clone --depth 1 --branch "${LINUX_TAG}" https://github.com/torvalds/linux.git "${KERNEL_SRC}"
else
    echo "  cached: ${KERNEL_SRC} (already cloned)"
fi

echo "  applying CONFIG_UNIX=y to the example's own linux_config..."
sed 's/^# CONFIG_UNIX is not set$/CONFIG_UNIX=y/' "${LINUX_CONFIG_SRC}" > "${KERNEL_SRC}/.config"
grep -q '^CONFIG_UNIX=y$' "${KERNEL_SRC}/.config" || {
    echo "error: CONFIG_UNIX=y substitution did not take — linux_config's line may have changed upstream" >&2
    exit 1
}

echo "  make olddefconfig..."
make -C "${KERNEL_SRC}" ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- olddefconfig

echo "  building kernel (ARCH=arm64, ~13 min per project-totebox's own measured time)..."
make -C "${KERNEL_SRC}" ARCH=arm64 CROSS_COMPILE=aarch64-linux-gnu- all -j"$(nproc)"

install -D -m 0644 "${KERNEL_SRC}/arch/arm64/boot/Image" "${OUTPUT}"
echo ""
echo "  done: ${OUTPUT}"
echo "  $(du -sh "${OUTPUT}" | cut -f1)"
echo ""
echo "  Makefile's LINUX var now resolves to this CONFIG_UNIX=y kernel instead"
echo "  of downloading the stock one — no further flag needed, just rerun"
echo "  build-microkit-image.sh with the same BUILD_DIR=${BUILD_DIR}."
