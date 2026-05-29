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
# Genesis Protocol role: cloud node joins the mesh (--join) or seeds it (--genesis) for
# the first deployment. Typical: GCP is the genesis-seed for the internet-facing hub.
#
# Usage:
#   ./provision-vm-infrastructure-cloud.sh --genesis          # first node; seeds WireGuard
#   ./provision-vm-infrastructure-cloud.sh --join <code>      # subsequent nodes; join via ceremony
#
# Requires: Q2 (subnet ratification), Q3 (GCP static IP), Q5 (Doorman port) — see NEXT.md

set -euo pipefail

# shellcheck source=lib/ppn-join.sh
source "$(dirname "$0")/lib/ppn-join.sh"

MODE="${1:-}"
SHORT_CODE="${2:-}"

case "${MODE}" in
    --genesis)
        echo "TODO: genesis-seed provisioner not yet implemented." >&2
        echo "Gated on: Q2 (subnet ratification) + Q3 (GCP static IP)." >&2
        echo "See BRIEF-PPN-ARCHITECTURE.md §9.2 Step 1 + BRIEF-VM-ARCHITECTURE.md §3." >&2
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
