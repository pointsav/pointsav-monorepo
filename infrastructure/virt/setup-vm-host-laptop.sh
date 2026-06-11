#!/usr/bin/env bash
# infrastructure/virt/setup-vm-host-laptop.sh
#
# Run this ON each laptop (Laptop A or Laptop B) to deploy service-vm-host
# as a pooled compute node for the PPN fleet.
#
# Requires: sshd running, WireGuard mesh active (can reach 10.8.0.9).
#
# Usage (from the laptop):
#   NODE_ID=laptop-a-1  WG_IP=10.8.0.6  bash <(curl -s mathew@10.8.0.9:'/srv/foundry/clones/project-infrastructure/infrastructure/virt/setup-vm-host-laptop.sh')
#   NODE_ID=laptop-b-1  WG_IP=10.8.0.1  bash ...
#
# Or after scp:
#   NODE_ID=laptop-a-1 WG_IP=10.8.0.6 ./setup-vm-host-laptop.sh

set -euo pipefail

NODE_ID="${NODE_ID:?NODE_ID must be set (e.g. laptop-a-1)}"
WG_IP="${WG_IP:?WG_IP must be set (e.g. 10.8.0.6)}"
FLEET_ENDPOINT="${FLEET_ENDPOINT:-http://10.8.0.9:9203}"
SPAWN_PORT="${SPAWN_PORT:-9220}"
BASE_IMAGE_DIR="${BASE_IMAGE_DIR:-/var/lib/vm-fleet}"
GCP_IP="10.8.0.9"

FLEET_SSH_PUBKEY="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIM8cK5fafqK4ZMqoLuAQC8hory12qPTW1jCqBTYMRgBq fleet-automation@gcp-cloud-1"

echo "=== PPN vm-host setup for node ${NODE_ID} (${WG_IP}) ==="

# --- 1. Packages ---
echo "[1] Installing prerequisites..."
sudo apt-get update -qq
sudo apt-get install -y -qq qemu-system-x86 qemu-utils genisoimage curl ca-certificates

# --- 2. VM disk directory ---
echo "[2] Creating disk directory ${BASE_IMAGE_DIR}..."
sudo mkdir -p "$BASE_IMAGE_DIR"
sudo chown "$(id -un):$(id -gn)" "$BASE_IMAGE_DIR"

# --- 3. Ubuntu base image ---
BASE_IMAGE="${BASE_IMAGE_DIR}/ubuntu-24.04-server-cloudimg-amd64.img"
if [[ ! -f "$BASE_IMAGE" ]]; then
    echo "[3] Downloading Ubuntu 24.04 cloud image (~630 MB)..."
    curl -fL --progress-bar \
        "https://cloud-images.ubuntu.com/releases/noble/release/ubuntu-24.04-server-cloudimg-amd64.img" \
        -o "$BASE_IMAGE"
else
    echo "[3] Base image already present: $BASE_IMAGE"
fi

# --- 4. Binary ---
echo "[4] Fetching service-vm-host binary from GCP fleet..."
scp -i ~/.ssh/fleet-automation_ed25519 -o StrictHostKeyChecking=no \
    "mathew@${GCP_IP}:/usr/local/bin/service-vm-host" /tmp/service-vm-host
sudo install -m 755 /tmp/service-vm-host /usr/local/bin/service-vm-host
echo "    installed at /usr/local/bin/service-vm-host"

# --- 5. Fleet automation SSH pubkey ---
echo "[5] Adding fleet automation SSH pubkey..."
mkdir -p ~/.ssh
chmod 700 ~/.ssh
if ! grep -qF "$FLEET_SSH_PUBKEY" ~/.ssh/authorized_keys 2>/dev/null; then
    echo "$FLEET_SSH_PUBKEY" >> ~/.ssh/authorized_keys
    chmod 600 ~/.ssh/authorized_keys
    echo "    added fleet-automation@gcp-cloud-1 pubkey"
else
    echo "    fleet pubkey already present"
fi

# --- 6. /etc/default/vm-host ---
echo "[6] Writing /etc/default/vm-host..."
sudo tee /etc/default/vm-host > /dev/null <<EOF
VM_FLEET_ENDPOINT=${FLEET_ENDPOINT}
VM_NODE_ID=${NODE_ID}
VM_WG_IP=${WG_IP}
VM_HEARTBEAT_INTERVAL_S=10
VM_SPAWN_PORT=${SPAWN_PORT}
VM_BASE_IMAGE=${BASE_IMAGE}
VM_DISK_DIR=${BASE_IMAGE_DIR}
VM_SSH_PUBKEY=${FLEET_SSH_PUBKEY}
EOF

# --- 7. systemd unit ---
echo "[7] Installing systemd unit..."
sudo tee /etc/systemd/system/local-vm-host.service > /dev/null <<'UNIT'
[Unit]
Description=PPN VM resource pool — per-node heartbeat + spawn agent
Documentation=https://github.com/pointsav/pointsav-monorepo
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=LAPTOP_USER
EnvironmentFile=-/etc/default/vm-host
ExecStart=/usr/local/bin/service-vm-host
Restart=on-failure
RestartSec=10s
StandardOutput=journal
StandardError=journal
SyslogIdentifier=local-vm-host

[Install]
WantedBy=multi-user.target
UNIT
# Patch user into unit
sudo sed -i "s/User=LAPTOP_USER/User=$(id -un)/" /etc/systemd/system/local-vm-host.service

sudo systemctl daemon-reload
sudo systemctl enable --now local-vm-host.service

echo ""
echo "=== Setup complete for ${NODE_ID} ==="
echo "    Heartbeating to ${FLEET_ENDPOINT} every 10s"
echo "    Spawn endpoint: http://${WG_IP}:${SPAWN_PORT}/v1/spawn"
echo ""
echo "    Verify from GCP:"
echo "    curl http://10.8.0.9:9203/v1/fleet"
echo ""
echo "    Check status:"
echo "    systemctl status local-vm-host"
