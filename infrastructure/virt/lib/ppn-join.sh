#!/usr/bin/env bash
# infrastructure/virt/lib/ppn-join.sh
# Genesis Protocol --join ceremony wrapper.
#
# Wraps the node-join HTTP ceremony for nodes 2+ joining an existing PPN mesh.
# Contacts service-ppn-pairing (deployed at :9205 on the genesis node) to:
#   1. Register this node's WireGuard public key
#   2. Display the returned short code for operator confirmation
#   3. Poll for approval (operator enters code on genesis node's F12 panel)
#   4. Save the approved WireGuard keypair to /etc/wireguard/
#
# Consumed by:
#   provision-vm-infrastructure-onprem.sh --join <code>
#   provision-vm-infrastructure-cloud.sh  --join <code>
#
# Wire types: system-pairing-codes/src/lib.rs
# Full ceremony specification: BRIEF-PPN-ARCHITECTURE.md §CPace + SAS

set -euo pipefail

# Perform a Genesis Protocol node-join ceremony.
# Usage: ppn_join <operator_code_or_empty> <pairing_server_url>
#
# operator_code: the Crockford base32 code the operator received out-of-band
#   from the genesis node's F12 panel. When supplied, it is included in the
#   join request body so the server can pre-match. Pass "" to skip.
# pairing_server: base URL of service-ppn-pairing (default: GCP relay at :9205).
ppn_join() {
    local operator_code="${1:-}"
    local pairing_server="${2:-http://34.53.65.203:9205}"

    # Require jq for JSON parsing.
    if ! command -v jq &>/dev/null; then
        echo "ppn_join: ERROR — jq is required. Install with: sudo apt install -y jq" >&2
        return 1
    fi

    # Require wg (wireguard-tools).
    if ! command -v wg &>/dev/null; then
        echo "ppn_join: wireguard-tools not found. Installing..."
        sudo apt-get install -y wireguard-tools
    fi

    echo "ppn_join: contacting pairing server at ${pairing_server}"

    # --- 1. Generate WireGuard keypair -------------------------------------------

    local wg_private wg_public
    wg_private=$(wg genkey)
    wg_public=$(echo "${wg_private}" | wg pubkey)

    # --- 2. Build join request body ----------------------------------------------

    local node_id arch bottom
    node_id=$(hostname -s 2>/dev/null || echo "ppn-node-$(cat /proc/sys/kernel/random/boot_id | cut -c1-8)")
    arch=$(uname -m)
    bottom="linux-phase1"  # Phase 2: netbsd-compat; Phase 3: sel4-microkit

    local request_body
    request_body=$(jq -n \
        --arg nid "${node_id}" \
        --arg wpk "${wg_public}" \
        --arg bot "${bottom}" \
        --arg arc "${arch}" \
        '{"node_id":$nid,"wireguard_pubkey":$wpk,"bottom":$bot,"arch":$arc}')

    # --- 3. POST /v1/node-join/request ------------------------------------------

    local response http_code
    http_code=$(curl -sf -o /tmp/ppn-join-response.json -w "%{http_code}" \
        -X POST "${pairing_server}/v1/node-join/request" \
        -H "Content-Type: application/json" \
        -d "${request_body}") || true

    if [[ "${http_code}" != "200" ]]; then
        echo "ppn_join: ERROR — pairing server returned HTTP ${http_code}" >&2
        echo "ppn_join: verify service-ppn-pairing is running at ${pairing_server}" >&2
        return 1
    fi

    local request_id code expires_at
    request_id=$(jq -r '.request_id' /tmp/ppn-join-response.json)
    code=$(jq -r '.code' /tmp/ppn-join-response.json)
    expires_at=$(jq -r '.expires_at' /tmp/ppn-join-response.json)

    echo ""
    echo "  ppn_join: request registered"
    echo "  request_id : ${request_id}"
    echo "  expires_at : ${expires_at}"
    echo ""
    echo "  ╔══════════════════════════════════════╗"
    echo "  ║  SHORT CODE: ${code}               ║"
    echo "  ╚══════════════════════════════════════╝"
    echo ""
    echo "  Enter this code on the genesis node's F12 panel (app-console-keys)."
    echo "  Or: POST ${pairing_server}/v1/node-join/approve {\"code\":\"${code}\"}"
    echo ""

    # --- 4. Poll for operator approval ------------------------------------------

    local elapsed=0
    local state="pending"
    echo "ppn_join: polling for approval (timeout 600s)..."
    while [[ "${state}" == "pending" ]] && [[ ${elapsed} -lt 600 ]]; do
        sleep 5
        elapsed=$((elapsed + 5))
        state=$(curl -sf "${pairing_server}/v1/node-join/status/${request_id}" 2>/dev/null \
            | jq -r '.state' 2>/dev/null || echo "pending")
        printf "  %4ds  state: %s\n" "${elapsed}" "${state}"
    done

    # --- 5. Handle outcome -------------------------------------------------------

    case "${state}" in
        approved)
            echo ""
            echo "ppn_join: APPROVED — saving WireGuard keypair..."
            sudo mkdir -p /etc/wireguard
            echo "${wg_private}" | sudo tee /etc/wireguard/ppn-node.key > /dev/null
            sudo chmod 600 /etc/wireguard/ppn-node.key
            echo "${wg_public}" | sudo tee /etc/wireguard/ppn-node.pub > /dev/null
            echo ""
            echo "  WireGuard keypair saved to /etc/wireguard/ppn-node.{key,pub}"
            echo "  Public key: ${wg_public}"
            echo ""
            echo "  Next steps:"
            echo "    1. Add this node's pubkey to the genesis node's /etc/wireguard/wg0.conf"
            echo "       [Peer]"
            echo "       PublicKey = ${wg_public}"
            echo "       AllowedIPs = <this-node-ppn-ip>/32"
            echo "    2. Run: sudo wg-quick up wg0  (on this node)"
            rm -f /tmp/ppn-join-response.json
            return 0
            ;;
        denied)
            echo "ppn_join: DENIED by operator." >&2
            rm -f /tmp/ppn-join-response.json
            return 1
            ;;
        *)
            echo "ppn_join: TIMED OUT after ${elapsed}s (final state: ${state})." >&2
            rm -f /tmp/ppn-join-response.json
            return 1
            ;;
    esac
}
