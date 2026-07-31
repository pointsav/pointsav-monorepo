#!/usr/bin/env bash
# SPDX-License-Identifier: FSL-1.1-ALv2
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

# build-guest-rootfs.sh — Build the os-totebox seL4/libvmm guest rootfs.
#
# Produces: build/guest-rootfs/rootfs.cpio.gz (aarch64, glibc — Ubuntu 24.04
#           "noble" minimal base via debootstrap, overlaid with our binaries)
#
# This is the seL4-hosted-guest counterpart to build-image.sh (which builds
# the separate NetBSD/QEMU-KVM compat-bottom image) — same overlay shape
# (base OS -> install our binaries -> install init -> package), different
# target: a libvmm/Microkit-hosted Linux guest rootfs, consumed by
# vendor-libvmm's own build (packaged via vendor-libvmm/tools/packrootfs,
# then baked into a loader.img alongside the seL4 kernel + VMM).
#
# Why debootstrap + glibc, not the libvmm examples' own uClibc/BusyBox
# rootfs: service-content depends unconditionally on `lbug` (LadybugDB,
# C++/cmake FFI) which needs glibc — confirmed no musl/uClibc build exists
# or is verified anywhere in this repo (see BRIEF-os-totebox-platform.md
# Session 18, G2.5 section). GLiNER goes through `gline-rs` (ONNX Runtime,
# see build step 5) rather than a Python/PyTorch stack, so a full distro
# base is used for `lbug`'s sake specifically, not for Python.
#
# Requires: debootstrap, qemu-user-static (for the aarch64 chroot second
#           stage), aarch64-linux-gnu-gcc (cross-toolchain, for anything
#           built directly rather than cross-compiled via cargo), cpio.
#
# Usage:
#   BINARIES_DIR=/path/to/aarch64-unknown-linux-gnu/release \
#   bash scripts/build-guest-rootfs.sh
set -euo pipefail

ARCH="arm64"
RUST_TARGET="aarch64-unknown-linux-gnu"
UBUNTU_RELEASE="noble"  # 24.04 LTS
UBUNTU_MIRROR="http://ports.ubuntu.com/ubuntu-ports"
BUILD_DIR="build"
BASE_DIR="${BUILD_DIR}/guest-rootfs-base"
OVERLAY="${BUILD_DIR}/guest-rootfs-overlay"
_CARGO_RELEASE="${CARGO_TARGET_DIR:-../../target}/${RUST_TARGET}/release"
BINARIES_DIR="${BINARIES_DIR:-${_CARGO_RELEASE}}"
OUTPUT_ROOTFS="${BUILD_DIR}/guest-rootfs/rootfs.cpio.gz"
LIBVMM_TOOLS="../vendor-libvmm/tools"

# ── 1. Preflight ─────────────────────────────────────────────────────────────
for CMD in debootstrap cpio; do
    command -v "${CMD}" >/dev/null || { echo "error: ${CMD} not found on PATH"; exit 1; }
done
[ -x "${LIBVMM_TOOLS}/packrootfs" ] || {
    echo "error: ${LIBVMM_TOOLS}/packrootfs not found — run from os-totebox/ with vendor-libvmm as a sibling"
    exit 1
}

# ── 2. Debootstrap base (two-stage: foreign-arch extract, then qemu-user chroot) ──
if [ ! -f "${BASE_DIR}/.debootstrap-complete" ]; then
    echo "  debootstrapping ${UBUNTU_RELEASE} ${ARCH} base (stage 1: foreign)..."
    sudo debootstrap --arch="${ARCH}" --foreign "${UBUNTU_RELEASE}" "${BASE_DIR}" "${UBUNTU_MIRROR}"
    echo "  debootstrap stage 2 (qemu-user chroot)..."
    sudo cp "$(command -v qemu-aarch64-static)" "${BASE_DIR}/usr/bin/"
    sudo chroot "${BASE_DIR}" /debootstrap/debootstrap --second-stage
    sudo touch "${BASE_DIR}/.debootstrap-complete"
else
    echo "  cached: ${BASE_DIR} (debootstrap already complete)"
fi

# ── 3. Assemble overlay (copy base, don't mutate it — keep it re-usable/cached) ──
sudo rm -rf "${OVERLAY}"
sudo cp -a "${BASE_DIR}" "${OVERLAY}"
sudo rm -f "${OVERLAY}/.debootstrap-complete" "${OVERLAY}/usr/bin/qemu-aarch64-static"

# ── 4. Install our binaries (cross-compiled for aarch64-unknown-linux-gnu) ──────
echo "  installing binaries..."
# Required — a rootfs without these isn't a valid os-totebox appliance image at
# all, but would still "boot" (the shell/init layer doesn't need them) and
# silently produce a guest with no service running. A soft warn-and-continue
# here previously cost two full build+boot cycles this session (2026-07-29,
# G4 smoke test debugging) chasing what looked like a connectivity/timing bug
# but was actually just a missing binary — fail the build instead.
for BIN in os-totebox; do
    SRC="${BINARIES_DIR}/${BIN}"
    if [ -f "${SRC}" ]; then
        sudo install -D -m 0755 "${SRC}" "${OVERLAY}/usr/bin/${BIN}"
    else
        echo "  error: required binary '${BIN}' not found in BINARIES_DIR=${BINARIES_DIR}" >&2
        echo "  (a rootfs built without it would boot but run no service — refusing to continue)" >&2
        exit 1
    fi
done
# gline-rs shim (GLiNER replacement, ONNX Runtime via `ort` — see
# BRIEF-os-totebox-platform.md Session 18) — real binary + ONNX model +
# tokenizer once built; not yet integrated (tracked separately, T8/T9).
# Genuinely optional today — no code path depends on it yet — so this one
# stays soft warn-and-continue, unlike os-totebox above.
for BIN in gline-rs-shim; do
    SRC="${BINARIES_DIR}/${BIN}"
    if [ -f "${SRC}" ]; then
        sudo install -D -m 0755 "${SRC}" "${OVERLAY}/usr/bin/${BIN}"
    else
        echo "  note: ${BIN} not found — entity extraction unavailable (expected until T8/T9 land)"
    fi
done

# ── 5. Install /init — a direct appliance-style PID 1, not full systemd ────────
# This guest is a single-purpose appliance (run os-totebox, expose its health
# endpoint), not a general interactive system — running full systemd as PID 1
# needs cgroups/dbus/proper unit setup this minimal environment doesn't have
# and buys nothing here. A raw debootstrap tree also has no top-level /init
# (only /sbin/init, normal FHS layout) — Linux's initramfs mechanism execs
# /init specifically as PID 1; without it the kernel falls through to
# searching for a separate root= block device instead of using the unpacked
# initramfs directly (hit in practice this session).
INIT_TMP="$(mktemp)"
cat > "${INIT_TMP}" << 'INIT_EOF'
#!/bin/sh
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev 2>/dev/null || true
mkdir -p /dev/pts && mount -t devpts devpts /dev/pts 2>/dev/null || true
echo "os-totebox appliance init starting..."
ip link set lo up 2>/dev/null || ifconfig lo up 2>/dev/null || true
ip link set eth0 up 2>/dev/null || ifconfig eth0 up 2>/dev/null || true
udhcpc -i eth0 -q 2>/dev/null || dhclient eth0 2>/dev/null || true
export RUST_BACKTRACE=full
export SLM_TIER=0
export SLM_BIND_ADDR=0.0.0.0:9080
export SLM_ORCHESTRATION_ENDPOINT="${SLM_ORCHESTRATION_ENDPOINT:-}"
export SERVICE_CONTENT_HTTP_BIND=127.0.0.1:9081
export SERVICE_CONTENT_BASE_DIR=/data/service-content
mkdir -p /data/service-content
echo "starting os-totebox..."
/usr/bin/os-totebox &
OS_TOTEBOX_PID=$!
echo "os-totebox started (pid ${OS_TOTEBOX_PID})"

# ── G4 parity smoke test ──────────────────────────────────────────────────
# Confirms both halves of the bundle actually answer HTTP, not just that
# they logged a "ready" line. No curl/wget in this pruned base — python3
# is present (debootstrap default), so it does the HTTP GETs directly.
# readyz is expected to report 503/degraded whenever no Tier 0 backend is
# configured (standalone mode) — that's correct behavior, not a failure;
# only a connection error or an unexpected exception (after the retry
# budget below is exhausted) counts as FAIL. QEMU TCG boot timing varies
# run to run, so this polls with retries instead of a single fixed sleep
# (a fixed 5s sleep was observed to be too short on a slower boot).
echo "waiting for services to come up before smoke test..."
python3 - << 'SMOKETEST_EOF'
import time
import urllib.error
import urllib.request

def get(url, timeout=3):
    try:
        resp = urllib.request.urlopen(url, timeout=timeout)
        return resp.status, resp.read().decode("utf-8", errors="replace")
    except urllib.error.HTTPError as e:
        return e.code, e.read().decode("utf-8", errors="replace")

def check(name, url, expect_status=None, retries=20, delay=1.5):
    last_exc = None
    for attempt in range(1, retries + 1):
        try:
            status, body = get(url)
            ok = expect_status is None or status == expect_status
            marker = "PASS" if ok else "FAIL"
            print(f"[G4-SMOKE] {marker} {name}: HTTP {status} (attempt {attempt}) — {body[:200]}")
            return ok
        except Exception as e:
            last_exc = e
            time.sleep(delay)
    print(f"[G4-SMOKE] FAIL {name}: unreachable after {retries} attempts — {type(last_exc).__name__}: {last_exc}")
    return False

results = [
    check("doorman /healthz", "http://127.0.0.1:9080/healthz", expect_status=200),
    check("doorman /readyz (503 expected in standalone mode)", "http://127.0.0.1:9080/readyz"),
    check("service-content /healthz", "http://127.0.0.1:9081/healthz", expect_status=200),
]
print(f"[G4-SMOKE] SUMMARY: {sum(results)}/{len(results)} reachable and returned an HTTP response")
SMOKETEST_EOF

# ── SIGTERM graceful-shutdown self-test ────────────────────────────────────
# No interactive console in this automated boot flow, so os-totebox signals
# itself: send a real SIGTERM to the process that G4 just confirmed healthy,
# then watch for the graceful-shutdown log lines and confirm the process
# actually exits (not just logs a message and keeps running) and the ports
# genuinely stop answering — not just that a shutdown message was printed.
echo "[SIGTERM-TEST] sending SIGTERM to os-totebox (pid ${OS_TOTEBOX_PID})..."
kill -TERM "${OS_TOTEBOX_PID}"
for i in $(seq 1 20); do
    if ! kill -0 "${OS_TOTEBOX_PID}" 2>/dev/null; then
        echo "[SIGTERM-TEST] PASS process exited (confirmed via kill -0) after ${i}s"
        break
    fi
    sleep 1
done
if kill -0 "${OS_TOTEBOX_PID}" 2>/dev/null; then
    echo "[SIGTERM-TEST] FAIL process still running 20s after SIGTERM"
fi
python3 - << 'PORTCHECK_EOF'
import urllib.error
import urllib.request

def port_closed(name, url):
    try:
        urllib.request.urlopen(url, timeout=3)
        print(f"[SIGTERM-TEST] FAIL {name}: still answering after shutdown")
        return False
    except urllib.error.URLError:
        print(f"[SIGTERM-TEST] PASS {name}: no longer answering (connection refused/reset)")
        return True
    except Exception as e:
        print(f"[SIGTERM-TEST] FAIL {name}: unexpected {type(e).__name__}: {e}")
        return False

results = [
    port_closed("doorman :9080", "http://127.0.0.1:9080/healthz"),
    port_closed("service-content :9081", "http://127.0.0.1:9081/healthz"),
]
print(f"[SIGTERM-TEST] SUMMARY: {sum(results)}/{len(results)} ports confirmed closed after shutdown")
PORTCHECK_EOF

echo "smoke test complete — dropping to a shell"
exec /bin/sh
INIT_EOF
sudo cp "${INIT_TMP}" "${OVERLAY}/init"
rm -f "${INIT_TMP}"
sudo chmod +x "${OVERLAY}/init"

# ── 5b. Prune non-server content (matches build-image.sh's NetBSD pruning) ──────
# The debootstrap base includes apt's package cache, docs, man pages, and locale
# data none of which a single-service guest needs — and a smaller initrd avoids
# overlapping the guest's fixed DTB load address (hit in practice: a full,
# unpruned base produced a rootfs large enough to collide with the DTB region).
echo "  pruning non-server content..."
sudo rm -rf "${OVERLAY}/var/cache/apt/archives" \
    "${OVERLAY}/var/lib/apt/lists" \
    "${OVERLAY}/usr/share/doc" \
    "${OVERLAY}/usr/share/man" \
    "${OVERLAY}/usr/share/locale" \
    "${OVERLAY}/usr/share/i18n" \
    "${OVERLAY}/usr/share/lintian" \
    "${OVERLAY}/usr/share/zoneinfo"
sudo find "${OVERLAY}/usr/share/locale-langpack" -mindepth 1 -maxdepth 1 ! -name "en*" -exec rm -rf {} \; 2>/dev/null || true

# ── 6. Package via packrootfs ────────────────────────────────────────────────
# packrootfs takes a rootfs.cpio.gz as its base (not a directory) — convert
# our debootstrap-derived overlay directory into that form first.
echo "  converting overlay directory to cpio.gz..."
mkdir -p "$(dirname "${OUTPUT_ROOTFS}")"
BASE_CPIO="${BUILD_DIR}/guest-rootfs-overlay.cpio.gz"
( cd "${OVERLAY}" && sudo find . | sudo cpio -o -H newc 2>/dev/null | gzip -9 ) > "${BASE_CPIO}"

echo "  packing final rootfs..."
mkdir -p "${BUILD_DIR}/packrootfs-tmp"
"${LIBVMM_TOOLS}/packrootfs" "${BASE_CPIO}" "${BUILD_DIR}/packrootfs-tmp" \
    -o "${OUTPUT_ROOTFS}" \
    --startup "${BUILD_DIR}/guest-rootfs-init/S99os-totebox"

echo ""
echo "  done: ${OUTPUT_ROOTFS}"
echo "  $(du -sh "${OUTPUT_ROOTFS}" | cut -f1)"
echo ""
echo "  use with vendor-libvmm's examples/virtio build via:"
echo "    make MICROKIT_BOARD=qemu_virt_aarch64 MICROKIT_SDK=/opt/microkit-sdk-2.2.0 \\"
echo "      INITRD=$(pwd)/${OUTPUT_ROOTFS} qemu"
