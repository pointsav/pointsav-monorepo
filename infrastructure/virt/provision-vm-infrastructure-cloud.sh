#!/usr/bin/env bash
# infrastructure/virt/provision-vm-infrastructure-cloud.sh
#
# VM-Infrastructure provisioner — cloud node (GCP).
#
# VM type:     VM-Infrastructure (cloud node)
# os-* source: os-infrastructure
# Node role:   cloud relay + web-facing endpoint; hosts VM-MediaKit, VM-Orchestration, VM-PrivateGit
#
# Phase 1 (now):   Linux + KVM/TCG on GCP Compute Engine (e2-standard-4 or better)
# Phase 2:         NetBSD + bhyve on x86-64 (compat bottom)
# Phase 3 target:  seL4 + Microkit 2.2.0 on AArch64 (native bottom;
#                  requires GCP C4A or equivalent AArch64 instance;
#                  gated on operator AArch64 hw decision — see NEXT.md)
#
# Genesis Protocol role: cloud node seeds the PPN mesh (--genesis) for all other nodes.
# GCP is the canonical genesis-seed for the internet-facing hub. On-prem nodes join after.
#
# Node constants (ratified 2026-05-30, Q2+Q3):
#   PPN mesh IP:   10.8.0.9/24  (WireGuard wg0 address)
#   GCP static IP: 34.53.65.203 (external; WireGuard listen port 51820)
#   Mesh cmd port: 8090 UDP (app-network-admin broadcast)
#   Pairing srv:   http://localhost:9205 (service-ppn-pairing, already deployed)
#
# Usage:
#   ./provision-vm-infrastructure-cloud.sh --genesis          # first time; seeds WireGuard
#   ./provision-vm-infrastructure-cloud.sh --join <code>      # joining node side
#   ./provision-vm-infrastructure-cloud.sh --status           # print current WireGuard state

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck source=lib/ppn-join.sh
source "${SCRIPT_DIR}/lib/ppn-join.sh"

PPN_MESH_IP="10.8.0.9"
PPN_SUBNET="10.8.0.0/24"
PPN_LISTEN_PORT="51820"
WG_CONF="/etc/wireguard/wg0.conf"
WG_KEY="/etc/wireguard/ppn-genesis.key"
WG_PUB="/etc/wireguard/ppn-genesis.pub"
PAIRING_SRV="http://localhost:9205"
PAIRING_SRV_EXTERNAL="http://34.53.65.203:9205"

MODE="${1:-}"
SHORT_CODE="${2:-}"

# ── helpers ──────────────────────────────────────────────────────────────────

require_root() {
    if [[ "${EUID}" -ne 0 ]]; then
        echo "ERROR: this script must be run as root (sudo)." >&2
        exit 1
    fi
}

require_wg() {
    if ! command -v wg &>/dev/null; then
        echo "Installing wireguard-tools..."
        apt-get install -y wireguard-tools
    fi
    if ! command -v wg-quick &>/dev/null; then
        apt-get install -y wireguard
    fi
}

require_jq() {
    if ! command -v jq &>/dev/null; then
        echo "Installing jq..."
        apt-get install -y jq
    fi
}

# ── mode dispatch ─────────────────────────────────────────────────────────────

case "${MODE}" in

    --genesis)
        require_root
        require_wg
        require_jq

        echo "provision-vm-infrastructure-cloud: Genesis-seed mode"
        echo "  PPN mesh IP:   ${PPN_MESH_IP}/24"
        echo "  GCP static IP: 34.53.65.203"
        echo "  Listen port:   ${PPN_LISTEN_PORT}/UDP"
        echo "  Pairing srv:   ${PAIRING_SRV}"
        echo ""

        # Verify service-ppn-pairing is reachable before proceeding.
        if ! curl -sf "${PAIRING_SRV}/v1/node-join/pending" > /dev/null 2>&1; then
            echo "WARNING: service-ppn-pairing not responding at ${PAIRING_SRV}" >&2
            echo "  Ensure the binary is running: systemctl status service-ppn-pairing" >&2
            echo "  Continuing — WireGuard setup can proceed independently." >&2
        else
            echo "service-ppn-pairing: responding at ${PAIRING_SRV}"
        fi

        # Generate WireGuard keypair if not already present.
        if [[ ! -f "${WG_KEY}" ]]; then
            echo "Generating WireGuard keypair..."
            wg genkey | tee "${WG_KEY}" | wg pubkey > "${WG_PUB}"
            chmod 600 "${WG_KEY}"
            echo "  Private key: ${WG_KEY}"
            echo "  Public key:  $(cat "${WG_PUB}")"
        else
            echo "WireGuard keypair already present at ${WG_KEY}"
        fi

        WG_PUBKEY=$(cat "${WG_PUB}")
        WG_PRIVKEY=$(cat "${WG_KEY}")

        # Write wg0.conf (idempotent — overwrites on re-genesis).
        echo "Writing ${WG_CONF}..."
        cat > "${WG_CONF}" <<WG_EOF
[Interface]
Address = ${PPN_MESH_IP}/24
ListenPort = ${PPN_LISTEN_PORT}
PrivateKey = ${WG_PRIVKEY}
# Masquerade outbound mesh traffic over the default GCP NIC.
PostUp   = iptables -A FORWARD -i wg0 -j ACCEPT; iptables -t nat -A POSTROUTING -o $(ip route | awk '/^default/ {print $5; exit}') -j MASQUERADE
PostDown = iptables -D FORWARD -i wg0 -j ACCEPT; iptables -t nat -D POSTROUTING -o $(ip route | awk '/^default/ {print $5; exit}') -j MASQUERADE

# Peers are added here as joining nodes complete the ceremony via service-ppn-pairing.
# Do not edit manually — operator edits may race with automated additions.
WG_EOF
        chmod 600 "${WG_CONF}"

        # Enable IP forwarding.
        if ! grep -q "^net.ipv4.ip_forward=1" /etc/sysctl.conf 2>/dev/null; then
            echo "net.ipv4.ip_forward=1" >> /etc/sysctl.conf
        fi
        sysctl -q -p

        # Bring up WireGuard (idempotent via wg-quick).
        if ip link show wg0 &>/dev/null; then
            echo "wg0 already up — reloading..."
            wg-quick down wg0 2>/dev/null || true
        fi
        wg-quick up wg0

        # Enable at boot.
        systemctl enable wg-quick@wg0 2>/dev/null || true

        echo ""
        echo "Cloud genesis node READY."
        echo "  WireGuard public key: ${WG_PUBKEY}"
        echo "  Mesh IP:              ${PPN_MESH_IP}/24 (wg0)"
        echo "  Pairing server:       ${PAIRING_SRV_EXTERNAL} (external)"
        echo "                        ${PAIRING_SRV} (internal)"
        echo ""
        echo "Distribute this public key to on-prem nodes joining the mesh."
        echo "On-prem join: ./provision-vm-infrastructure-onprem.sh --join <code>"
        echo "Cloud join:   ./provision-vm-infrastructure-cloud.sh --join <code>"
        ;;

    --join)
        if [[ -z "${SHORT_CODE}" ]]; then
            echo "Usage: $0 --join <short-code>" >&2
            exit 1
        fi
        ppn_join "${SHORT_CODE}" "${PAIRING_SRV_EXTERNAL}"
        ;;

    --status)
        echo "provision-vm-infrastructure-cloud: WireGuard status"
        if ip link show wg0 &>/dev/null 2>&1; then
            wg show wg0
        else
            echo "  wg0: not up (run --genesis to initialise)"
        fi
        echo ""
        echo "Pairing server (internal): ${PAIRING_SRV}"
        curl -sf "${PAIRING_SRV}/v1/node-join/pending" | jq '.' 2>/dev/null \
            || echo "  service-ppn-pairing: not responding"
        ;;

    *)
        echo "Usage: $0 --genesis | --join <short-code> | --status" >&2
        exit 1
        ;;
esac
