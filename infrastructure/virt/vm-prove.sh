#!/usr/bin/env bash
# vm-prove.sh — boot a minimal VM and verify it can host a Totebox service
#
# Proves the virtualization layer concept. Uses KVM if available (Laptop A /
# real hardware); falls back to QEMU TCG if /dev/kvm is absent (GCP VM
# without nested virt). The TCB delta between KVM and seL4/bhyve is documented
# in BRIEF-PPN-DEV-BOOTSTRAP.md §6.
#
# Usage:
#   ./vm-prove.sh            # auto-detect KVM
#   ./vm-prove.sh --tcg      # force TCG (no KVM)
#
# Prerequisites:
#   apt install -y qemu-system-x86 qemu-utils curl

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work"
ALPINE_VERSION="3.20"
ALPINE_ISO="alpine-virt-${ALPINE_VERSION}.0-x86_64.iso"
ALPINE_URL="https://dl-cdn.alpinelinux.org/alpine/v${ALPINE_VERSION}/releases/x86_64/${ALPINE_ISO}"
DISK="$WORK_DIR/ppn-prove.qcow2"
PORT_SSH=10022      # host port forwarding → VM port 22
PORT_SVC=10202      # host port forwarding → VM port 9202 (service-ppn-pairing)

mkdir -p "$WORK_DIR"

# --- KVM detection ----------------------------------------------------------

ACCEL="tcg"
if [[ "${1:-}" == "--tcg" ]]; then
    echo "vm-prove: TCG mode forced"
elif [[ -e /dev/kvm ]]; then
    ACCEL="kvm"
    echo "vm-prove: KVM available — using hardware acceleration"
else
    echo "vm-prove: /dev/kvm not found — using QEMU TCG (slower; proves concept without KVM)"
    echo "vm-prove: to enable KVM on Laptop A: confirm VT-x is on in BIOS"
    echo "vm-prove: to enable nested virt on GCP: stop VM, gcloud compute instances update"
    echo "          --enable-nested-virtualization <name>"
fi

# --- Download Alpine ISO -----------------------------------------------------

ISO_PATH="$WORK_DIR/$ALPINE_ISO"
if [[ ! -f "$ISO_PATH" ]]; then
    echo "vm-prove: downloading Alpine ${ALPINE_VERSION} ISO (~50 MB)..."
    curl -fL --progress-bar "$ALPINE_URL" -o "$ISO_PATH"
else
    echo "vm-prove: using cached $ISO_PATH"
fi

# --- Create disk image -------------------------------------------------------

if [[ ! -f "$DISK" ]]; then
    echo "vm-prove: creating 512 MB disk image..."
    qemu-img create -f qcow2 "$DISK" 512M
fi

# --- Boot VM -----------------------------------------------------------------

echo "vm-prove: booting VM (accel=$ACCEL)"
echo "vm-prove: SSH available at localhost:$PORT_SSH after Alpine boots"
echo "vm-prove: service-ppn-pairing will forward to localhost:$PORT_SVC"
echo "vm-prove: type 'root' at login prompt (no password in Alpine virt image)"
echo "vm-prove: to stop: shutdown -h now inside the VM, or Ctrl-A X in QEMU console"
echo ""

exec qemu-system-x86_64 \
    -accel "$ACCEL" \
    -m 256M \
    -smp 1 \
    -nographic \
    -drive "file=$DISK,format=qcow2" \
    -cdrom "$ISO_PATH" \
    -boot d \
    -netdev "user,id=net0,hostfwd=tcp::${PORT_SSH}-:22,hostfwd=tcp::${PORT_SVC}-:9202" \
    -device "virtio-net-pci,netdev=net0" \
    -virtfs local,path="$SCRIPT_DIR",mount_tag=host0,security_model=none \
    -append "console=ttyS0" \
    -kernel /dev/null 2>/dev/null || \
exec qemu-system-x86_64 \
    -accel "$ACCEL" \
    -m 256M \
    -smp 1 \
    -nographic \
    -drive "file=$DISK,format=qcow2" \
    -cdrom "$ISO_PATH" \
    -boot d \
    -netdev "user,id=net0,hostfwd=tcp::${PORT_SSH}-:22,hostfwd=tcp::${PORT_SVC}-:9202" \
    -device "virtio-net-pci,netdev=net0"
