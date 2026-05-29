#!/usr/bin/env bash
# infrastructure/virt/provision-vm-infrastructure-onprem.sh
#
# VM-Infrastructure provisioner — on-prem node (Laptop A / Laptop B).
#
# VM type:     VM-Infrastructure (on-prem node)
# os-* source: os-infrastructure
#
# Node roles:
#   Laptop A (iMac 12,1)  — genesis-seed node; TCG-only (no VT-d/IOMMU)
#                           Hosts: VM-Totebox-1 (first Totebox archive)
#   Laptop B              — WireGuard hub/relay; KVM if VT-x available
#                           Hosts: PPN relay + optional VM-Orchestration
#
# Phase 1 (now):   Linux + KVM (Laptop B) / TCG (Laptop A); QEMU-managed VMs
# Phase 2:         NetBSD + bhyve (x86-64 compat bottom; no VT-d requirement for basic hosting)
# Phase 3 target:  seL4 + Microkit 2.2.0 on AArch64 (native bottom;
#                  requires AArch64 hardware acquisition on at least one node)
#
# iMac 12,1 constraint: Sandy Bridge i5-2400S; no VT-d; Broadcom 14e4:16b4 NIC.
# TCG-only is acceptable for genesis-seed + Totebox-1 hosting.
# Not suitable for SLM/GPU inference or high-throughput MediaKit traffic.
#
# Genesis Protocol role:
#   Laptop A: genesis-seed (--genesis) — first node, seeds WireGuard, opens short-code server
#   Laptop B: joiner (--join) — second node, joins via Crockford base32 short-code ceremony
#
# Usage:
#   ./provision-vm-infrastructure-onprem.sh --genesis          # Laptop A
#   ./provision-vm-infrastructure-onprem.sh --join <code>      # Laptop B
#
# Requires: Q2 (subnet ratification), Q4 (Laptop B IP + DNS), Q5 (Doorman port) — see NEXT.md

set -euo pipefail

# shellcheck source=lib/common.sh
source "$(dirname "$0")/lib/common.sh"
# shellcheck source=lib/ppn-join.sh
source "$(dirname "$0")/lib/ppn-join.sh"

MODE="${1:-}"
SHORT_CODE="${2:-}"

case "${MODE}" in
    --genesis)
        echo "TODO: genesis-seed provisioner for on-prem node not yet implemented." >&2
        echo "Gated on: Q2 (subnet ratification) + Broadcom silicon_ping() implementation." >&2
        echo "See BRIEF-PPN-ARCHITECTURE.md §9.2 Steps 1+2 + BRIEF-VM-ARCHITECTURE.md §3." >&2
        exit 1
        ;;
    --join)
        if [[ -z "${SHORT_CODE}" ]]; then
            echo "Usage: $0 --join <short-code>" >&2
            exit 1
        fi
        ppn_join "${SHORT_CODE}" "${PAIRING_SERVER:-http://10.8.0.9:9202}"
        ;;
    *)
        echo "Usage: $0 --genesis | --join <short-code>" >&2
        exit 1
        ;;
esac
