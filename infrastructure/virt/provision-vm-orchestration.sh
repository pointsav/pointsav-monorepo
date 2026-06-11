#!/usr/bin/env bash
# infrastructure/virt/provision-vm-orchestration.sh
#
# VM-Orchestration provisioner — os-orchestration runtime instance.
#
# VM type:     VM-Orchestration
# os-* source: os-orchestration (canonical rename of os-interface; in flight)
# Purpose:     Stateless multi-archive aggregator; commercial paid tier.
#
# Service port assignments (guest-side):
#   app-orchestration-bim  :9096  (blocked on VM-Totebox service-fs at :9100)
#   app-orchestration-gis  :9097
#   app-orchestration-slm  :9180
#
# app-orchestration-gis and app-orchestration-slm are independent — deployable now.
# app-orchestration-bim requires VM-Totebox Phase 1 (service-fs) to be complete first.
#
# Phase 1: Ubuntu 24.04 x86_64 guest (KVM/TCG).
# Phase 2: + gVisor sandboxing for aggregator processes.
# Phase 3: NanoVMs/OPS unikernel (planned; gated on operator decision).
#
# Usage:
#   ./provision-vm-orchestration.sh            # auto-detect KVM
#   ./provision-vm-orchestration.sh --tcg      # force TCG
#   ./provision-vm-orchestration.sh --fg       # foreground (serial console; for debugging)
#
# Prerequisites:
#   sudo apt install -y qemu-system-x86 qemu-utils genisoimage socat
#
# After first boot (wait ~60s for cloud-init):
#   ssh -p 10023 -i infrastructure/virt/work/foundry-vm-key foundry@localhost

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work"
CLOUD_INIT_DIR="${SCRIPT_DIR}/cloud-init-orchestration"

UBUNTU_IMAGE="ubuntu-24.04-server-cloudimg-amd64.img"
UBUNTU_URL="https://cloud-images.ubuntu.com/releases/noble/release/${UBUNTU_IMAGE}"
BASE_DISK="${WORK_DIR}/${UBUNTU_IMAGE}"
VM_DISK="${WORK_DIR}/vm-orchestration.qcow2"
SEED_ISO="${WORK_DIR}/vm-orchestration-seed.iso"
PID_FILE="${WORK_DIR}/vm-orchestration.pid"
MONITOR_SOCK="${WORK_DIR}/vm-orchestration.monitor"

RAM_MB=4096
DISK_GB=20

mkdir -p "$WORK_DIR"

# --- KVM detection -----------------------------------------------------------

ACCEL="tcg"
FOREGROUND=0
for arg in "$@"; do
    case "$arg" in
        --tcg) ACCEL="tcg"; echo "provision-vm-orchestration: TCG mode forced" ;;
        --fg)  FOREGROUND=1 ;;
    esac
done

if [[ "$ACCEL" != "tcg" ]]; then
    if [[ -e /dev/kvm ]]; then
        ACCEL="kvm"
        echo "provision-vm-orchestration: KVM available — using hardware acceleration"
    else
        echo "provision-vm-orchestration: /dev/kvm not found — using QEMU TCG (Phase 1 adequate)"
    fi
fi

# --- Check for existing running instance -------------------------------------

if [[ -f "$PID_FILE" ]]; then
    OLD_PID=$(cat "$PID_FILE")
    if kill -0 "$OLD_PID" 2>/dev/null; then
        echo "provision-vm-orchestration: vm-orchestration already running (pid $OLD_PID)"
        echo "to stop: echo system_powerdown | socat - UNIX-CONNECT:$MONITOR_SOCK"
        exit 1
    else
        echo "provision-vm-orchestration: stale pid file removed"
        rm -f "$PID_FILE"
    fi
fi

# --- Download Ubuntu 24.04 base image ----------------------------------------

if [[ ! -f "$BASE_DISK" ]]; then
    echo "provision-vm-orchestration: downloading Ubuntu 24.04 server cloud image (~630 MB)..."
    curl -fL --progress-bar "$UBUNTU_URL" -o "$BASE_DISK"
else
    echo "provision-vm-orchestration: using cached $BASE_DISK"
fi

# --- Create VM disk -----------------------------------------------------------

if [[ ! -f "$VM_DISK" ]]; then
    echo "provision-vm-orchestration: creating ${DISK_GB}GB VM disk..."
    cp "$BASE_DISK" "$VM_DISK"
    qemu-img resize "$VM_DISK" "${DISK_GB}G"
else
    echo "provision-vm-orchestration: using existing $VM_DISK"
fi

# --- Build cloud-init seed ISO -----------------------------------------------

if [[ ! -f "$SEED_ISO" ]]; then
    echo "provision-vm-orchestration: building cloud-init seed ISO..."
    if ! command -v genisoimage &>/dev/null; then
        echo "provision-vm-orchestration: genisoimage not found — run: sudo apt install -y genisoimage"
        exit 1
    fi
    genisoimage \
        -output "$SEED_ISO" \
        -volid cidata \
        -joliet -rock \
        "${CLOUD_INIT_DIR}/meta-data" \
        "${CLOUD_INIT_DIR}/user-data"
    echo "provision-vm-orchestration: seed ISO built at $SEED_ISO"
else
    echo "provision-vm-orchestration: using existing seed ISO"
fi

# --- SSH key check -----------------------------------------------------------

KEY_PATH="${SCRIPT_DIR}/work/foundry-vm-key"
if [[ ! -f "$KEY_PATH" ]]; then
    echo "provision-vm-orchestration: generating SSH key..."
    ssh-keygen -t ed25519 -C "foundry@vm-orchestration" -f "$KEY_PATH" -N ""
    echo ""
    echo "  NOTE: cloud-init user-data embeds a placeholder public key."
    echo "  Rebuild the seed ISO with the generated key:"
    echo "    rm $SEED_ISO"
    echo "    Update ${CLOUD_INIT_DIR}/user-data with: $(cat "${KEY_PATH}.pub")"
    echo "    Then re-run this script."
fi

# --- Boot VM -----------------------------------------------------------------

echo "provision-vm-orchestration: booting vm-orchestration (accel=$ACCEL, RAM=${RAM_MB}MB)"
echo "provision-vm-orchestration: SSH at localhost:10023 (wait ~60s for cloud-init)"
echo "provision-vm-orchestration: service port-forwards:"
echo "  localhost:10023 → :22    (SSH)"
echo "  localhost:19096 → :9096  (app-orchestration-bim; blocked on VM-Totebox)"
echo "  localhost:19097 → :9097  (app-orchestration-gis)"
echo "  localhost:19180 → :9180  (app-orchestration-slm)"
echo ""

if [[ "$FOREGROUND" -eq 1 ]]; then
    DISPLAY_ARGS=(-nographic)
    DAEMON_ARGS=()
else
    DISPLAY_ARGS=(-display none -serial "file:${WORK_DIR}/vm-orchestration-serial.log")
    DAEMON_ARGS=(-daemonize -pidfile "$PID_FILE")
fi

exec qemu-system-x86_64 \
    -accel "$ACCEL" \
    -m "${RAM_MB}M" \
    -smp 2 \
    "${DISPLAY_ARGS[@]}" \
    -drive "file=${VM_DISK},format=qcow2,if=virtio" \
    -drive "file=${SEED_ISO},format=raw,if=virtio,media=cdrom,readonly=on" \
    -netdev "user,id=net0,\
hostfwd=tcp::10023-:22,\
hostfwd=tcp::19096-:9096,\
hostfwd=tcp::19097-:9097,\
hostfwd=tcp::19180-:9180" \
    -device "virtio-net-pci,netdev=net0" \
    -device "virtio-balloon" \
    -monitor "unix:${MONITOR_SOCK},server,nowait" \
    "${DAEMON_ARGS[@]+"${DAEMON_ARGS[@]}"}"
