#!/usr/bin/env bash
# infrastructure/virt/provision-vm-infrastructure-onprem.sh
#
# VM-Infrastructure provisioner — on-prem node (Laptop A / Laptop B).
#
# VM type:     VM-Infrastructure (on-prem node)
# os-* source: os-infrastructure
#
# Node roles (ratified 2026-05-30, Q2+Q4):
#   Laptop A (iMac 12,1)  — PPN mesh IP 10.8.0.6/24; genesis-seed or first joiner
#                           Sandy Bridge i5-2400S; BCM57765 NIC (14E4:16B4); no VT-d
#                           Hosts: VM-Totebox-1 (first Totebox archive)
#   Laptop B              — PPN mesh IP 10.8.0.1/24; WireGuard hub/relay
#                           LAN IP: 10.0.0.224 (DNS stable — no DNS config change)
#                           Public: 24.86.192.209:51820 (Laptop B WireGuard endpoint)
#
# Phase 1 (now):   Linux + KVM (Laptop B) / TCG (Laptop A); QEMU-managed VMs
# Phase 2:         NetBSD + NVMM (x86-64 compat bottom; bhyve-compatible)
# Phase 3 target:  seL4 + Microkit 2.2.0 on AArch64 (gated on hw acquisition)
#
# iMac 12,1 constraint: VT-x present (EPT); VT-d absent → no PCI passthrough.
# TCG-only fallback acceptable for genesis-seed + Totebox-1 hosting.
# BCM57765 served by Linux bge(4) driver; Step 2 probe_nic() detects it at bare-metal boot.
#
# Genesis Protocol role:
#   Laptop A: --genesis — first on-prem node; configures WireGuard as a peer of GCP hub
#   Laptop B: --genesis-hub — Laptop B acts as WireGuard hub for on-prem LAN
#   Either:   --join   — joins an existing mesh via service-ppn-pairing ceremony
#
# Usage:
#   ./provision-vm-infrastructure-onprem.sh --genesis              # Laptop A (joiner of GCP hub)
#   ./provision-vm-infrastructure-onprem.sh --genesis-hub          # Laptop B (LAN WireGuard hub)
#   ./provision-vm-infrastructure-onprem.sh --join <code>          # any on-prem node, joining
#   ./provision-vm-infrastructure-onprem.sh --status               # print WireGuard state
#
# Pairing server: service-ppn-pairing on GCP relay (34.53.65.203:9205)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck source=lib/common.sh
source "${SCRIPT_DIR}/lib/common.sh"
# shellcheck source=lib/ppn-join.sh
source "${SCRIPT_DIR}/lib/ppn-join.sh"

# Node IPs (ratified 2026-05-30)
LAPTOP_A_PPN_IP="10.8.0.6"
LAPTOP_B_PPN_IP="10.8.0.1"
GCP_PPN_IP="10.8.0.9"
PPN_SUBNET="10.8.0.0/24"
PPN_LISTEN_PORT="51820"

# WireGuard hub endpoint (Laptop B public)
HUB_ENDPOINT="24.86.192.209:51820"
GCP_ENDPOINT="34.53.65.203:51820"

# Pairing server on GCP relay
PAIRING_SRV="http://34.53.65.203:9205"

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

# ── mode dispatch ─────────────────────────────────────────────────────────────

case "${MODE}" in

    --genesis)
        # Laptop A joins GCP relay hub.
        # Sets up WireGuard as a PEER node (not a relay); GCP hub at 34.53.65.203:51820.
        require_root
        require_wg

        NODE_IP="${LAPTOP_A_PPN_IP}"
        WG_KEY="/etc/wireguard/ppn-laptop-a.key"
        WG_PUB="/etc/wireguard/ppn-laptop-a.pub"
        WG_CONF="/etc/wireguard/wg0.conf"

        echo "provision-vm-infrastructure-onprem: Genesis mode (Laptop A / iMac 12,1)"
        echo "  PPN mesh IP:   ${NODE_IP}/24 (wg0)"
        echo "  GCP hub:       ${GCP_ENDPOINT}"
        echo "  Pairing srv:   ${PAIRING_SRV}"
        echo ""

        # Generate keypair.
        if [[ ! -f "${WG_KEY}" ]]; then
            echo "Generating WireGuard keypair for Laptop A..."
            wg genkey | tee "${WG_KEY}" | wg pubkey > "${WG_PUB}"
            chmod 600 "${WG_KEY}"
        else
            echo "WireGuard keypair present at ${WG_KEY}"
        fi

        WG_PUBKEY=$(cat "${WG_PUB}")
        WG_PRIVKEY=$(cat "${WG_KEY}")

        # Write wg0.conf.
        # GCP relay (10.8.0.9) is the hub peer; allowed-ips covers full /24 for mesh routing.
        echo "Writing ${WG_CONF}..."
        cat > "${WG_CONF}" <<WG_EOF
[Interface]
Address = ${NODE_IP}/24
PrivateKey = ${WG_PRIVKEY}
DNS = 10.8.0.9

[Peer]
# GCP relay hub — 10.8.0.9 / 34.53.65.203
PublicKey = PLACEHOLDER_GCP_HUB_PUBKEY
AllowedIPs = ${PPN_SUBNET}
Endpoint = ${GCP_ENDPOINT}
PersistentKeepalive = 25
WG_EOF
        chmod 600 "${WG_CONF}"

        echo ""
        echo "  IMPORTANT: Replace PLACEHOLDER_GCP_HUB_PUBKEY in ${WG_CONF}"
        echo "  with the GCP genesis node's public key (from --genesis --status on GCP)."
        echo "  Then: sudo wg-quick up wg0"
        echo ""
        echo "Laptop A genesis node READY (pending GCP pubkey fill)."
        echo "  Local public key: ${WG_PUBKEY}"
        echo ""
        echo "Add this key to GCP's /etc/wireguard/wg0.conf under [Peer]:"
        echo "  PublicKey  = ${WG_PUBKEY}"
        echo "  AllowedIPs = ${NODE_IP}/32"
        ;;

    --genesis-hub)
        # Laptop B as LAN WireGuard hub/relay.
        require_root
        require_wg

        NODE_IP="${LAPTOP_B_PPN_IP}"
        WG_KEY="/etc/wireguard/ppn-laptop-b.key"
        WG_PUB="/etc/wireguard/ppn-laptop-b.pub"
        WG_CONF="/etc/wireguard/wg0.conf"

        echo "provision-vm-infrastructure-onprem: Hub mode (Laptop B)"
        echo "  PPN mesh IP:   ${NODE_IP}/24 (wg0)"
        echo "  Listen port:   ${PPN_LISTEN_PORT}/UDP"
        echo "  Public addr:   ${HUB_ENDPOINT}"
        echo ""

        if [[ ! -f "${WG_KEY}" ]]; then
            echo "Generating WireGuard keypair for Laptop B hub..."
            wg genkey | tee "${WG_KEY}" | wg pubkey > "${WG_PUB}"
            chmod 600 "${WG_KEY}"
        else
            echo "WireGuard keypair present at ${WG_KEY}"
        fi

        WG_PUBKEY=$(cat "${WG_PUB}")
        WG_PRIVKEY=$(cat "${WG_KEY}")

        # Hub config: listens on the public port; routes between all mesh peers.
        cat > "${WG_CONF}" <<WG_EOF
[Interface]
Address = ${NODE_IP}/24
ListenPort = ${PPN_LISTEN_PORT}
PrivateKey = ${WG_PRIVKEY}
PostUp   = iptables -A FORWARD -i wg0 -j ACCEPT; iptables -t nat -A POSTROUTING -o $(ip route | awk '/^default/ {print $5; exit}') -j MASQUERADE
PostDown = iptables -D FORWARD -i wg0 -j ACCEPT; iptables -t nat -D POSTROUTING -o $(ip route | awk '/^default/ {print $5; exit}') -j MASQUERADE

[Peer]
# GCP relay — 10.8.0.9 / 34.53.65.203
PublicKey = PLACEHOLDER_GCP_HUB_PUBKEY
AllowedIPs = ${GCP_PPN_IP}/32
Endpoint = ${GCP_ENDPOINT}
PersistentKeepalive = 25

[Peer]
# Laptop A — 10.8.0.6
PublicKey = PLACEHOLDER_LAPTOP_A_PUBKEY
AllowedIPs = ${LAPTOP_A_PPN_IP}/32
WG_EOF
        chmod 600 "${WG_CONF}"

        echo "net.ipv4.ip_forward=1" >> /etc/sysctl.conf
        sysctl -q -p

        echo ""
        echo "Laptop B hub config written to ${WG_CONF}."
        echo "Replace PLACEHOLDER_* values with actual public keys, then:"
        echo "  sudo wg-quick up wg0"
        echo "  sudo systemctl enable wg-quick@wg0"
        echo ""
        echo "Laptop B public key: ${WG_PUBKEY}"
        ;;

    --join)
        if [[ -z "${SHORT_CODE}" ]]; then
            echo "Usage: $0 --join <short-code>" >&2
            exit 1
        fi
        ppn_join "${SHORT_CODE}" "${PAIRING_SRV}"
        ;;

    --status)
        echo "provision-vm-infrastructure-onprem: WireGuard status"
        if ip link show wg0 &>/dev/null 2>&1; then
            wg show wg0
        else
            echo "  wg0: not up"
        fi
        ;;

    *)
        echo "Usage: $0 --genesis | --genesis-hub | --join <short-code> | --status" >&2
        exit 1
        ;;
esac
