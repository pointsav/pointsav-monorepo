#!/usr/bin/env bash
# build-image.sh — Build the os-totebox NetBSD 10.1 guest image.
#
# Produces: build/os-totebox.qcow2 (root filesystem, 4 GiB FFS2)
# Requires: NetBSD cross tools under TOOLS_DIR (build from NetBSD src with
#           ./build.sh -U -T /path/to/tools tools) and qemu-img on PATH.
#
# Does NOT mount UFS2 from Linux — all filesystem writes go through nbmakefs.
# The resulting image is bootable on QEMU KVM (x86_64).
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
IMAGE_RAW="${BUILD_DIR}/os-totebox.img"
IMAGE_QCOW2="${BUILD_DIR}/os-totebox.qcow2"
IMAGE_SIZE="4g"

# ── 1. Preflight ─────────────────────────────────────────────────────────────
command -v qemu-img >/dev/null || { echo "error: qemu-img not found on PATH"; exit 1; }
[ -d "${TOOLS_DIR}" ] || {
    echo "error: TOOLS_DIR=${TOOLS_DIR} not found"
    echo "  Build NetBSD cross tools first:"
    echo "    cd netbsd-src && ./build.sh -U -T ${TOOLS_DIR} tools"
    exit 1
}
NBMAKEFS="${TOOLS_DIR}/bin/nbmakefs"
NBINSTALLBOOT="${TOOLS_DIR}/bin/nbinstallboot"
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
echo "  extracting base set..."
tar -xzf "${BUILD_DIR}/sets/base.tgz"    -C "${OVERLAY}"
echo "  extracting etc set..."
tar -xzf "${BUILD_DIR}/sets/etc.tgz"     -C "${OVERLAY}"
echo "  extracting kernel..."
tar -xzf "${BUILD_DIR}/sets/kern-GENERIC.tgz" -C "${OVERLAY}"

# ── 4. Install our binaries (cross-compiled for x86_64-unknown-netbsd) ───────
echo "  installing binaries..."
install -D -m 0755 "${BINARIES_DIR}/system-ledger-server" \
    "${OVERLAY}/usr/bin/system-ledger-server"
# slm-doorman-server and service-content are built separately; copy from
# BINARIES_DIR if present, else skip with a warning.
for BIN in slm-doorman-server service-content; do
    SRC="${BINARIES_DIR}/${BIN}"
    if [ -f "${SRC}" ]; then
        install -D -m 0755 "${SRC}" "${OVERLAY}/usr/bin/${BIN}"
    else
        echo "  warning: ${BIN} not found in BINARIES_DIR — skipping"
    fi
done
# llama-server from pkgsrc — must be pre-built on a NetBSD host or via
# NetBSD cross-pkgsrc. Expect it at BINARIES_DIR/llama-server.
for BIN in llama-server; do
    SRC="${BINARIES_DIR}/${BIN}"
    if [ -f "${SRC}" ]; then
        install -D -m 0755 "${SRC}" "${OVERLAY}/usr/bin/${BIN}"
    else
        echo "  warning: ${BIN} not found — inference will be unavailable"
    fi
done

# ── 5. Install rc.d scripts ──────────────────────────────────────────────────
echo "  installing rc.d scripts..."
install -D -m 0755 scripts/rc.d/system_ledger  "${OVERLAY}/etc/rc.d/system_ledger"
install -D -m 0755 scripts/rc.d/doorman         "${OVERLAY}/etc/rc.d/doorman"
install -D -m 0755 scripts/rc.d/service_content "${OVERLAY}/etc/rc.d/service_content"
install -D -m 0755 scripts/rc.d/llama_server    "${OVERLAY}/etc/rc.d/llama_server"

# ── 6. Configure rc.conf ─────────────────────────────────────────────────────
cat >> "${OVERLAY}/etc/rc.conf" << 'EOF'
# os-totebox services
sshd=YES
system_ledger=YES
doorman=YES
service_content=YES
llama_server=YES
EOF

# ── 7. Configure WireGuard interface (wg0) ───────────────────────────────────
# Actual keys are injected at deployment time by project-infrastructure.
# This file is a template; project-infrastructure replaces <PPN_PRIVATE_KEY>
# and <GCP_PUBLIC_KEY> via cloud-init or a provisioning script.
cat > "${OVERLAY}/etc/ifconfig.wg0" << 'EOF'
create
!wgconfig wg0 set private-key /etc/wireguard/private.key
!wgconfig wg0 add peer <GCP_PUBLIC_KEY> \
    --allowed-ips 10.8.0.0/24 \
    --endpoint <GCP_ENDPOINT>:51820
inet 10.8.0.7/24
EOF
chmod 0600 "${OVERLAY}/etc/ifconfig.wg0"
mkdir -p "${OVERLAY}/etc/wireguard"
chmod 0700 "${OVERLAY}/etc/wireguard"

# ── 8. Veriexec manifest (OS-level binary signing) ───────────────────────────
# Generate SHA-256 fingerprints for all installed binaries.
echo "  generating Veriexec manifest..."
VERIEXEC_MANIFEST="${OVERLAY}/etc/signatures"
: > "${VERIEXEC_MANIFEST}"
for BIN_PATH in \
    "${OVERLAY}/usr/bin/system-ledger-server" \
    "${OVERLAY}/usr/bin/slm-doorman-server" \
    "${OVERLAY}/usr/bin/service-content" \
    "${OVERLAY}/usr/bin/llama-server"; do
    [ -f "${BIN_PATH}" ] || continue
    REL_PATH="${BIN_PATH#${OVERLAY}}"
    DIGEST=$(sha256sum "${BIN_PATH}" | awk '{print $1}')
    printf '%s %s VERIEXEC_DIRECT\n' "${REL_PATH}" "${DIGEST}" \
        >> "${VERIEXEC_MANIFEST}"
done
# Enable Veriexec enforcement at boot.
echo "veriexec=YES" >> "${OVERLAY}/etc/security.conf"

# ── 9. Build FFS2 disk image using NetBSD cross-makefs ───────────────────────
echo "  building FFS2 image (${IMAGE_SIZE})..."
"${NBMAKEFS}" -t ffs -s "${IMAGE_SIZE}" -o version=2 "${IMAGE_RAW}" "${OVERLAY}"

# Install bootloader (NetBSD BIOS GPT/MBR boot).
if [ -f "${OVERLAY}/usr/mdec/biosboot" ] && [ -x "${NBINSTALLBOOT}" ]; then
    "${NBINSTALLBOOT}" "${IMAGE_RAW}" \
        "${OVERLAY}/usr/mdec/biosboot" "/usr/mdec/boot"
else
    echo "  warning: biosboot not found — image may not be directly bootable"
    echo "    use -kernel vmlinuz or direct ELF load in QEMU instead"
fi

# ── 10. Convert to QCOW2 ─────────────────────────────────────────────────────
echo "  converting to QCOW2..."
qemu-img convert -f raw -O qcow2 "${IMAGE_RAW}" "${IMAGE_QCOW2}"
rm -f "${IMAGE_RAW}"

echo ""
echo "  done: ${IMAGE_QCOW2}"
echo "  $(du -sh ${IMAGE_QCOW2} | cut -f1)"
echo ""
echo "  launch with:"
echo "    qemu-system-x86_64 -enable-kvm -cpu host -m 8192 \\"
echo "      -drive file=${IMAGE_QCOW2},format=qcow2,if=virtio \\"
echo "      -drive file=build/os-totebox-data.qcow2,format=qcow2,if=virtio \\"
echo "      -netdev tap,id=net0,ifname=tap-intelligence \\"
echo "      -device virtio-net-pci,netdev=net0 \\"
echo "      -nographic -serial mon:stdio"
