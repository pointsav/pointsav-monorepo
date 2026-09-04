#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

# build-microkit-image.sh — guarded wrapper around vendor-libvmm's Microkit
# `make ... qemu` link step for os-mediakit.
#
# Ported from project-totebox's os-totebox/scripts/build-microkit-image.sh
# (2026-08-25), including its resource-pressure preflight guard verbatim —
# that guard exists because project-totebox found via filesystem-mtime
# forensics that this exact kind of build (C/C++ compile + link of
# blk_virt/net_virt/serial_virt/client_vmm, culminating in loader.img) was
# actively running at the moment of a real host crash. Not a hypothetical
# risk; reuse the same thresholds, don't invent a third variant.
#
# G1 (2026-08-25): proved the Microkit+libvmm+TCG pipeline itself boots to a
# real login prompt on this host, using vendor-libvmm's own bundled example
# Linux kernel + initrd (auto-downloaded by virtio.mk if not already
# present) — unrelated to app-mediakit-knowledge, deliberately not
# overridden that pass, to keep G1 and G2.5 separately-verifiable gates.
#
# G3 (2026-08-26): G2.5 landed a real os-mediakit guest rootfs
# (build/guest-rootfs/rootfs.cpio.gz, all 3 wiki tenants baked in) — pass
# INITRD= pointing at it to boot the real appliance instead of the stock
# example. Also required a CONFIG_UNIX=y guest kernel rebuild
# (build-guest-kernel.sh) — the stock example kernel panics any real
# tokio-based service.
#
# G4 (2026-08-26): pass FOUNDRY_EXTRA_BOOTARGS='foundry.mode=smoketest' to
# run /init's smoke-test + SIGTERM self-test path instead of staying up as
# a production appliance. Ported the substitution mechanism itself (this
# script's pass-through plus vendor-libvmm/examples/virtio's own
# linux.dts/@@FOUNDRY_EXTRA_BOOTARGS@@ + virtio.mk changes) from
# project-totebox's identical fix to this same shared example — QEMU's
# `-append` doesn't work with this image's `-device loader` boot path, so
# runtime config has to be baked into the DTB at build time instead.
#
# Usage: run from os-mediakit/ (same convention as os-totebox):
#   bash scripts/build-microkit-image.sh                                # G1: stock example
#   INITRD=$(pwd)/build/guest-rootfs/rootfs.cpio.gz \
#     bash scripts/build-microkit-image.sh                              # G3: real rootfs
#   INITRD=$(pwd)/build/guest-rootfs/rootfs.cpio.gz \
#     FOUNDRY_EXTRA_BOOTARGS='foundry.mode=smoketest' \
#     bash scripts/build-microkit-image.sh                              # G4: smoke test
set -euo pipefail

BUILD_DIR="build-os-mediakit"
MICROKIT_BOARD="qemu_virt_aarch64"
MICROKIT_SDK="${MICROKIT_SDK:-/opt/microkit-sdk-2.2.0}"
LIBVMM_VIRTIO_DIR="../vendor-libvmm/examples/virtio"
INITRD="${INITRD:-}"
FOUNDRY_EXTRA_BOOTARGS="${FOUNDRY_EXTRA_BOOTARGS:-}"

[ -d "${MICROKIT_SDK}" ] || {
    echo "error: MICROKIT_SDK not found at ${MICROKIT_SDK}" >&2
    exit 1
}
[ -d "${LIBVMM_VIRTIO_DIR}" ] || {
    echo "error: ${LIBVMM_VIRTIO_DIR} not found — run from os-mediakit/" >&2
    exit 1
}

# ── Preflight resource guard — identical thresholds/logic to
# os-totebox's build-microkit-image.sh (itself sourced from
# bin/resource-log.sh's pressure-snapshot trigger). Hard-abort, not
# warn-and-continue: this host is shared across concurrent Totebox
# sessions and a live inference service, and a heavy Microkit/libvmm
# make build has a real, evidenced crash history when run under
# pressure.
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
        echo "error: refusing to start the Microkit/libvmm make build — host already under pressure" >&2
        echo "  mem_used_pct=${_mem_used_pct}% psi_mem_some_avg10=${_psi_mem_some} load_1=${_load_1} nproc=${_nproc_count}" >&2
        echo "  (thresholds match bin/resource-log.sh's own pressure-snapshot trigger)" >&2
        echo "  wait for load/memory to settle and rerun -- this is a hard gate, not a warning" >&2
        echo "  confirmed operator override: FOUNDRY_RESOURCE_GUARD_BYPASS=1 bash ${0} ..." >&2
        exit 1
    fi
else
    echo "  preflight OK: mem_used_pct=${_mem_used_pct}% psi=${_psi_mem_some} load_1=${_load_1}/${_nproc_count}"
fi

if [ -n "${INITRD}" ]; then
    [ -f "${INITRD}" ] || {
        echo "error: INITRD=${INITRD} not found" >&2
        exit 1
    }
    echo "  building Microkit/libvmm image (BUILD_DIR=${BUILD_DIR}, INITRD=${INITRD}, FOUNDRY_EXTRA_BOOTARGS='${FOUNDRY_EXTRA_BOOTARGS}')..."
    exec make -C "${LIBVMM_VIRTIO_DIR}" \
        MICROKIT_BOARD="${MICROKIT_BOARD}" \
        MICROKIT_SDK="${MICROKIT_SDK}" \
        BUILD_DIR="${BUILD_DIR}" \
        INITRD="${INITRD}" \
        FOUNDRY_EXTRA_BOOTARGS="${FOUNDRY_EXTRA_BOOTARGS}" \
        qemu
else
    echo "  building Microkit/libvmm image (BUILD_DIR=${BUILD_DIR}, stock example LINUX/INITRD, FOUNDRY_EXTRA_BOOTARGS='${FOUNDRY_EXTRA_BOOTARGS}')..."
    exec make -C "${LIBVMM_VIRTIO_DIR}" \
        MICROKIT_BOARD="${MICROKIT_BOARD}" \
        MICROKIT_SDK="${MICROKIT_SDK}" \
        BUILD_DIR="${BUILD_DIR}" \
        FOUNDRY_EXTRA_BOOTARGS="${FOUNDRY_EXTRA_BOOTARGS}" \
        qemu
fi
