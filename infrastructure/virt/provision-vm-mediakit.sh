#!/usr/bin/env bash
# provision-vm-mediakit.sh — create and launch vm-mediakit (Debian 12, Phase 1)
#
# Boots vm-mediakit as the os-mediakit guest on foundry-workspace.
# Uses KVM if available (Laptop A / real hardware); falls back to QEMU TCG
# (GCP workspace without nested virt). TCG is adequate for Phase 1 testing.
#
# Phase 1: Debian 12 x86_64 guest — seL4 AArch64 guest is Phase 3 (planned).
# See BRIEF-totebox-transformation §9 for the seL4 architecture decision.
#
# Usage:
#   ./provision-vm-mediakit.sh            # auto-detect KVM
#   ./provision-vm-mediakit.sh --tcg      # force TCG
#   ./provision-vm-mediakit.sh --fg       # foreground (no -daemonize; for debugging)
#
# Prerequisites:
#   sudo apt install -y qemu-system-x86 qemu-utils genisoimage socat
#
# After first boot (wait ~60s for cloud-init):
#   ssh -p 10022 -i infrastructure/virt/work/foundry-vm-key foundry@localhost
#
# Migrate a service:
#   ./infrastructure/virt/migrate-service-to-vm.sh <service-name> <port>

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work"
CLOUD_INIT_DIR="${SCRIPT_DIR}/cloud-init-mediakit"

DEBIAN_IMAGE="debian-12-genericcloud-amd64.qcow2"
DEBIAN_URL="https://cloud.debian.org/images/cloud/bookworm/latest/${DEBIAN_IMAGE}"
BASE_DISK="${WORK_DIR}/${DEBIAN_IMAGE}"
VM_DISK="${WORK_DIR}/vm-mediakit.qcow2"
SEED_ISO="${WORK_DIR}/vm-mediakit-seed.iso"
PID_FILE="${WORK_DIR}/vm-mediakit.pid"
MONITOR_SOCK="${WORK_DIR}/vm-mediakit.monitor"

RAM_MB=6144
DISK_GB=20

mkdir -p "$WORK_DIR"

# --- KVM detection -----------------------------------------------------------

ACCEL="tcg"
FOREGROUND=0
for arg in "$@"; do
    case "$arg" in
        --tcg) ACCEL="tcg"; echo "provision-vm-mediakit: TCG mode forced" ;;
        --fg)  FOREGROUND=1 ;;
    esac
done

if [[ "$ACCEL" != "tcg" ]]; then
    if [[ -e /dev/kvm ]]; then
        ACCEL="kvm"
        echo "provision-vm-mediakit: KVM available — using hardware acceleration"
    else
        echo "provision-vm-mediakit: /dev/kvm not found — using QEMU TCG (Phase 1 adequate)"
        echo "provision-vm-mediakit: for KVM on Laptop A: confirm VT-x on in BIOS"
    fi
fi

# --- Check for existing running instance -------------------------------------

if [[ -f "$PID_FILE" ]]; then
    OLD_PID=$(cat "$PID_FILE")
    if kill -0 "$OLD_PID" 2>/dev/null; then
        echo "provision-vm-mediakit: vm-mediakit already running (pid $OLD_PID)"
        echo "provision-vm-mediakit: to stop: echo system_powerdown | socat - UNIX-CONNECT:$MONITOR_SOCK"
        exit 1
    else
        echo "provision-vm-mediakit: stale pid file removed"
        rm -f "$PID_FILE"
    fi
fi

# --- Download Debian 12 base image -------------------------------------------

if [[ ! -f "$BASE_DISK" ]]; then
    echo "provision-vm-mediakit: downloading Debian 12 genericcloud (~400 MB)..."
    curl -fL --progress-bar "$DEBIAN_URL" -o "$BASE_DISK"
else
    echo "provision-vm-mediakit: using cached $BASE_DISK"
fi

# --- Create VM disk (overlay on base) ----------------------------------------

if [[ ! -f "$VM_DISK" ]]; then
    echo "provision-vm-mediakit: creating ${DISK_GB}GB VM disk..."
    cp "$BASE_DISK" "$VM_DISK"
    qemu-img resize "$VM_DISK" "${DISK_GB}G"
else
    echo "provision-vm-mediakit: using existing $VM_DISK"
fi

# --- Build cloud-init seed ISO -----------------------------------------------

if [[ ! -f "$SEED_ISO" ]]; then
    echo "provision-vm-mediakit: building cloud-init seed ISO..."
    if ! command -v genisoimage &>/dev/null; then
        echo "provision-vm-mediakit: genisoimage not found — run: sudo apt install -y genisoimage"
        exit 1
    fi
    genisoimage \
        -output "$SEED_ISO" \
        -volid cidata \
        -joliet -rock \
        "${CLOUD_INIT_DIR}/meta-data" \
        "${CLOUD_INIT_DIR}/user-data"
    echo "provision-vm-mediakit: seed ISO built at $SEED_ISO"
else
    echo "provision-vm-mediakit: using existing seed ISO"
fi

# --- SSH key check -----------------------------------------------------------

KEY_PATH="${SCRIPT_DIR}/work/foundry-vm-key"
if [[ ! -f "$KEY_PATH" ]]; then
    echo "provision-vm-mediakit: generating SSH key for vm-mediakit..."
    ssh-keygen -t ed25519 -C "foundry@vm-mediakit" -f "$KEY_PATH" -N ""
    echo "provision-vm-mediakit: key at $KEY_PATH (public key embedded in cloud-init)"
    echo ""
    echo "  NOTE: The cloud-init user-data embeds a placeholder public key."
    echo "  To use this generated key, rebuild the seed ISO:"
    echo "    rm $SEED_ISO"
    echo "    Update ${CLOUD_INIT_DIR}/user-data with the contents of ${KEY_PATH}.pub"
    echo "    Then re-run this script."
fi

# --- Boot VM -----------------------------------------------------------------

echo "provision-vm-mediakit: booting vm-mediakit (accel=$ACCEL, RAM=${RAM_MB}MB)"
echo "provision-vm-mediakit: SSH at localhost:10022 (wait ~60s for cloud-init)"
echo "provision-vm-mediakit: service port-forwards active:"
echo "  localhost:10022 → :22   (SSH)"
echo "  localhost:19090 → :9090 (knowledge-documentation)"
echo "  localhost:19092 → :9092 (proofreader)"
echo "  localhost:19093 → :9093 (knowledge-projects)"
echo "  localhost:19095 → :9095 (knowledge-corporate)"
echo "  localhost:19096 → :9096 (bim-orchestration)"
echo "  localhost:19100 → :9100 (service-fs WORM ledger)"
echo "  localhost:19101 → :9101 (marketing-pointsav)"
echo "  localhost:19102 → :9102 (marketing-woodfine)"
echo ""

DAEMON_ARGS=()
if [[ "$FOREGROUND" -eq 0 ]]; then
    DAEMON_ARGS=(-daemonize -pidfile "$PID_FILE")
fi

exec qemu-system-x86_64 \
    -accel "$ACCEL" \
    -m "${RAM_MB}M" \
    -smp 2 \
    -nographic \
    -drive "file=${VM_DISK},format=qcow2,if=virtio" \
    -drive "file=${SEED_ISO},format=raw,if=virtio,media=cdrom,readonly=on" \
    -netdev "user,id=net0,\
hostfwd=tcp::10022-:22,\
hostfwd=tcp::19090-:9090,\
hostfwd=tcp::19092-:9092,\
hostfwd=tcp::19093-:9093,\
hostfwd=tcp::19095-:9095,\
hostfwd=tcp::19096-:9096,\
hostfwd=tcp::19100-:9100,\
hostfwd=tcp::19101-:9101,\
hostfwd=tcp::19102-:9102" \
    -device "virtio-net-pci,netdev=net0" \
    -device "virtio-balloon" \
    -monitor "unix:${MONITOR_SOCK},server,nowait" \
    "${DAEMON_ARGS[@]+"${DAEMON_ARGS[@]}"}"
