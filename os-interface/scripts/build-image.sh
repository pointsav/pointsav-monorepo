#!/usr/bin/env bash
# build-image.sh — Build the os-orchestration NetBSD 10.1 guest image.
#
# (Directory currently: os-interface — canonical name os-orchestration, rename in flight.)
#
# Produces: build/os-orchestration.qcow2 (root filesystem, 3 GiB FFS2)
# Lighter than os-totebox: no OLMo weights, no data disk.
# Runs: app-orchestration-slm (:9180), gateway-orchestration-bim,
#       gateway-orchestration-gis, nginx (TLS termination + reverse proxy).
#
# Requires: NetBSD cross tools under TOOLS_DIR and qemu-img on PATH.
#
# Usage:
#   TOOLS_DIR=/path/to/netbsd-tools \
#   BINARIES_DIR=/path/to/cross-compiled-binaries \
#   bash scripts/build-image.sh
set -euo pipefail

NETBSD_VER="10.1"
ARCH="amd64"
SETS_URL="https://cdn.netbsd.org/pub/NetBSD/NetBSD-${NETBSD_VER}/${ARCH}/binary/sets"
TOOLS_DIR="${TOOLS_DIR:-build/netbsd-tools}"
BINARIES_DIR="${BINARIES_DIR:-../../target/x86_64-unknown-netbsd/release}"
BUILD_DIR="build"
OVERLAY="${BUILD_DIR}/overlay"
IMAGE_RAW="${BUILD_DIR}/os-orchestration.img"
IMAGE_QCOW2="${BUILD_DIR}/os-orchestration.qcow2"
IMAGE_SIZE="3g"

# ── 1. Preflight ─────────────────────────────────────────────────────────────
command -v qemu-img >/dev/null || { echo "error: qemu-img not found on PATH"; exit 1; }
[ -d "${TOOLS_DIR}" ] || {
    echo "error: TOOLS_DIR=${TOOLS_DIR} not found"
    echo "  Build NetBSD cross tools first:"
    echo "    cd netbsd-src && ./build.sh -U -T ${TOOLS_DIR} tools"
    exit 1
}
NBMAKEFS="${TOOLS_DIR}/bin/nbmakefs"
[ -x "${NBMAKEFS}" ] || { echo "error: nbmakefs not found in TOOLS_DIR"; exit 1; }

# ── 2. Download official NetBSD binary sets ───────────────────────────────────
mkdir -p "${BUILD_DIR}/sets"
for SET in base etc kern-GENERIC; do
    DEST="${BUILD_DIR}/sets/${SET}.tgz"
    [ -f "${DEST}" ] && { echo "  cached: ${SET}.tgz"; continue; }
    echo "  fetching: ${SET}.tgz"
    curl -fSL --output "${DEST}" "${SETS_URL}/${SET}.tgz"
done

# ── 3. Assemble rootfs overlay ───────────────────────────────────────────────
rm -rf "${OVERLAY}"
mkdir -p "${OVERLAY}"
echo "  extracting sets..."
tar -xzf "${BUILD_DIR}/sets/base.tgz"         -C "${OVERLAY}"
tar -xzf "${BUILD_DIR}/sets/etc.tgz"          -C "${OVERLAY}"
tar -xzf "${BUILD_DIR}/sets/kern-GENERIC.tgz" -C "${OVERLAY}"

# ── 4. Install our binaries ───────────────────────────────────────────────────
echo "  installing binaries..."
for BIN in orchestration-slm-server; do
    SRC="${BINARIES_DIR}/${BIN}"
    if [ -f "${SRC}" ]; then
        install -D -m 0755 "${SRC}" "${OVERLAY}/usr/bin/${BIN}"
    else
        echo "  warning: ${BIN} not found in BINARIES_DIR — skipping"
    fi
done
# nginx from pkgsrc — expected pre-built; skip gracefully if absent.
if [ -f "${BINARIES_DIR}/nginx" ]; then
    install -D -m 0755 "${BINARIES_DIR}/nginx" "${OVERLAY}/usr/sbin/nginx"
else
    echo "  warning: nginx not found — install from pkgsrc on the guest"
fi

# ── 5. Install rc.d scripts ──────────────────────────────────────────────────
echo "  installing rc.d scripts..."
install -D -m 0755 scripts/rc.d/orchestration_slm "${OVERLAY}/etc/rc.d/orchestration_slm"

# ── 6. Configure rc.conf ─────────────────────────────────────────────────────
cat >> "${OVERLAY}/etc/rc.conf" << 'EOF'
# os-orchestration services
sshd=YES
orchestration_slm=YES
EOF

# ── 7. WireGuard interface ────────────────────────────────────────────────────
cat > "${OVERLAY}/etc/ifconfig.wg0" << 'EOF'
create
!wgconfig wg0 set private-key /etc/wireguard/private.key
!wgconfig wg0 add peer <GCP_PUBLIC_KEY> \
    --allowed-ips 10.8.0.0/24 \
    --endpoint <GCP_ENDPOINT>:51820
inet 10.8.0.5/24
EOF
chmod 0600 "${OVERLAY}/etc/ifconfig.wg0"
mkdir -p "${OVERLAY}/etc/wireguard"
chmod 0700 "${OVERLAY}/etc/wireguard"

# ── 8. Veriexec manifest ─────────────────────────────────────────────────────
echo "  generating Veriexec manifest..."
VERIEXEC_MANIFEST="${OVERLAY}/etc/signatures"
: > "${VERIEXEC_MANIFEST}"
for BIN_PATH in \
    "${OVERLAY}/usr/bin/orchestration-slm-server" \
    "${OVERLAY}/usr/sbin/nginx"; do
    [ -f "${BIN_PATH}" ] || continue
    REL_PATH="${BIN_PATH#${OVERLAY}}"
    DIGEST=$(sha256sum "${BIN_PATH}" | awk '{print $1}')
    printf '%s %s VERIEXEC_DIRECT\n' "${REL_PATH}" "${DIGEST}" \
        >> "${VERIEXEC_MANIFEST}"
done
echo "veriexec=YES" >> "${OVERLAY}/etc/security.conf"

# ── 9. Build FFS2 image ───────────────────────────────────────────────────────
echo "  building FFS2 image (${IMAGE_SIZE})..."
"${NBMAKEFS}" -t ffs -s "${IMAGE_SIZE}" -o version=2 "${IMAGE_RAW}" "${OVERLAY}"

# ── 10. Convert to QCOW2 ─────────────────────────────────────────────────────
echo "  converting to QCOW2..."
qemu-img convert -f raw -O qcow2 "${IMAGE_RAW}" "${IMAGE_QCOW2}"
rm -f "${IMAGE_RAW}"

echo ""
echo "  done: ${IMAGE_QCOW2}"
echo "  $(du -sh ${IMAGE_QCOW2} | cut -f1)"
echo ""
echo "  launch with:"
echo "    qemu-system-x86_64 -enable-kvm -cpu host -m 6144 \\"
echo "      -drive file=${IMAGE_QCOW2},format=qcow2,if=virtio \\"
echo "      -netdev tap,id=net0,ifname=tap-orchestration \\"
echo "      -device virtio-net-pci,netdev=net0 \\"
echo "      -nographic -serial mon:stdio"
