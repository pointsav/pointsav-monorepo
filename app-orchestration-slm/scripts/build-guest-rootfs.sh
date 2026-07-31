#!/usr/bin/env bash
# SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

# build-guest-rootfs.sh — Build app-orchestration-slm's seL4/libvmm guest rootfs.
#
# Produces: build/guest-rootfs/rootfs.cpio.gz (aarch64, glibc — Ubuntu 24.04
#           "noble" minimal base via debootstrap, overlaid with the
#           orchestration-slm-server binary)
#
# Mirrors os-totebox/scripts/build-guest-rootfs.sh's exact mechanism
# (base OS -> install binary -> install init -> package) deliberately, not
# by default/laziness: the operator's explicit requirement is that this
# guest ships "as portable as os-totebox" — using the identical, already-
# proven-portable debootstrap+glibc approach is the most direct way to
# guarantee that, rather than introducing a second, untested toolchain
# (musl was tried first and rejected 2026-07-30 — see BRIEF-os-totebox-
# platform.md: `ring`, a transitive rustls dependency, needs a real C
# cross-compiler even for a "pure Rust" binary, so musl would have needed
# a brand-new aarch64-linux-musl-gcc toolchain for a marginal gain, since
# the whole rootfs ships bundled inside loader.img either way).
#
# Simpler than os-totebox's guest in one real way: orchestration-slm-server
# has zero native/C dependencies (no lbug, no GLiNER/ONNX) — it's a stateless
# HTTP broker that proxies to existing Yo-Yo GPU nodes over the network, not
# a bundled inference runtime. No cross-compiled C++ toolchain juggling here.
#
# Requires: debootstrap, qemu-user-static (for the aarch64 chroot second
#           stage), cpio.
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
    echo "error: ${LIBVMM_TOOLS}/packrootfs not found — run from app-orchestration-slm/ with vendor-libvmm as a sibling (via project-totebox's clone)"
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

# ── 4. Install our binary (cross-compiled for aarch64-unknown-linux-gnu) ────────
echo "  installing binary..."
# Required, fail hard rather than warn-and-continue — a rootfs built without
# it would still boot (the init script itself needs nothing from it) but run
# no service at all, indistinguishable from a real connectivity/timing bug at
# the HTTP layer. This exact silent-skip mistake cost two full build+boot
# cycles on os-totebox's own script before being fixed there 2026-07-29 —
# fixed here from the start rather than repeating it.
for BIN in orchestration-slm-server; do
    SRC="${BINARIES_DIR}/${BIN}"
    if [ -f "${SRC}" ]; then
        sudo install -D -m 0755 "${SRC}" "${OVERLAY}/usr/bin/${BIN}"
    else
        echo "  error: required binary '${BIN}' not found in BINARIES_DIR=${BINARIES_DIR}" >&2
        echo "  (a rootfs built without it would boot but run no service — refusing to continue)" >&2
        exit 1
    fi
done

# ── 5. Install /init — a direct appliance-style PID 1, not full systemd ────────
# Same reasoning as os-totebox's init: single-purpose appliance, not a
# general interactive system; a raw debootstrap tree has no top-level /init.
INIT_TMP="$(mktemp)"
cat > "${INIT_TMP}" << 'INIT_EOF'
#!/bin/sh
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev 2>/dev/null || true
mkdir -p /dev/pts && mount -t devpts devpts /dev/pts 2>/dev/null || true
echo "app-orchestration-slm appliance init starting..."
ip link set lo up 2>/dev/null || ifconfig lo up 2>/dev/null || true
ip link set eth0 up 2>/dev/null || ifconfig eth0 up 2>/dev/null || true
udhcpc -i eth0 -q 2>/dev/null || dhclient eth0 2>/dev/null || true
export RUST_BACKTRACE=full
export ORCHESTRATION_BIND_ADDR=0.0.0.0:9180
# No Yo-Yo endpoints configured in this smoke-test boot — the chassis is
# meant to run standalone-reachable (health/fleet/discovery endpoints work;
# /v1/yoyo/* correctly report unavailable) same as os-totebox's own
# standalone-first design, not a hard boot precondition.
export ORCHESTRATION_YOYO_DEFAULT_ENDPOINT="${ORCHESTRATION_YOYO_DEFAULT_ENDPOINT:-}"
export ORCHESTRATION_YOYO_TRAINER_ENDPOINT="${ORCHESTRATION_YOYO_TRAINER_ENDPOINT:-}"
export ORCHESTRATION_YOYO_GRAPH_ENDPOINT="${ORCHESTRATION_YOYO_GRAPH_ENDPOINT:-}"
export ORCHESTRATION_ALLOCATION_LEDGER_PATH=/data/orchestration-slm/allocated-ids.jsonl
mkdir -p /data/orchestration-slm
echo "starting orchestration-slm-server..."
/usr/bin/orchestration-slm-server &
ORCH_PID=$!
echo "orchestration-slm-server started (pid ${ORCH_PID})"

# ── smoke test — same shape as os-totebox's G4 smoke test ──────────────────
echo "waiting for the chassis to come up before smoke test..."
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
            print(f"[O-SMOKE] {marker} {name}: HTTP {status} (attempt {attempt}) — {body[:200]}")
            return ok
        except Exception as e:
            last_exc = e
            time.sleep(delay)
    print(f"[O-SMOKE] FAIL {name}: unreachable after {retries} attempts — {type(last_exc).__name__}: {last_exc}")
    return False

results = [
    check("chassis /healthz", "http://127.0.0.1:9180/healthz", expect_status=200),
    check("chassis /readyz", "http://127.0.0.1:9180/readyz"),
    check("chassis /v1/fleet", "http://127.0.0.1:9180/v1/fleet", expect_status=200),
]
print(f"[O-SMOKE] SUMMARY: {sum(results)}/{len(results)} reachable and returned an HTTP response")
SMOKETEST_EOF

# ── SIGTERM graceful-shutdown self-test — same pattern as os-totebox's ─────
echo "[SIGTERM-TEST] sending SIGTERM to orchestration-slm-server (pid ${ORCH_PID})..."
kill -TERM "${ORCH_PID}"
for i in $(seq 1 20); do
    if ! kill -0 "${ORCH_PID}" 2>/dev/null; then
        echo "[SIGTERM-TEST] PASS process exited (confirmed via kill -0) after ${i}s"
        break
    fi
    sleep 1
done
if kill -0 "${ORCH_PID}" 2>/dev/null; then
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

results = [port_closed("chassis :9180", "http://127.0.0.1:9180/healthz")]
print(f"[SIGTERM-TEST] SUMMARY: {sum(results)}/{len(results)} ports confirmed closed after shutdown")
PORTCHECK_EOF

echo "smoke test complete — dropping to a shell"
exec /bin/sh
INIT_EOF
sudo cp "${INIT_TMP}" "${OVERLAY}/init"
rm -f "${INIT_TMP}"
sudo chmod +x "${OVERLAY}/init"

# ── 5b. Prune non-server content (matches os-totebox's own pruning) ────────────
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
echo "  converting overlay directory to cpio.gz..."
mkdir -p "$(dirname "${OUTPUT_ROOTFS}")"
BASE_CPIO="${BUILD_DIR}/guest-rootfs-overlay.cpio.gz"
( cd "${OVERLAY}" && sudo find . | sudo cpio -o -H newc 2>/dev/null | gzip -9 ) > "${BASE_CPIO}"

echo "  packing final rootfs..."
mkdir -p "${BUILD_DIR}/packrootfs-tmp"
"${LIBVMM_TOOLS}/packrootfs" "${BASE_CPIO}" "${BUILD_DIR}/packrootfs-tmp" \
    -o "${OUTPUT_ROOTFS}"

echo ""
echo "  done: ${OUTPUT_ROOTFS}"
echo "  $(du -sh "${OUTPUT_ROOTFS}" | cut -f1)"
echo ""
echo "  use with vendor-libvmm's examples/virtio build via:"
echo "    make MICROKIT_BOARD=qemu_virt_aarch64 MICROKIT_SDK=/opt/microkit-sdk-2.2.0 \\"
echo "      INITRD=$(pwd)/${OUTPUT_ROOTFS} qemu"
