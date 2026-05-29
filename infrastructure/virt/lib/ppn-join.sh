#!/usr/bin/env bash
# infrastructure/virt/lib/ppn-join.sh
# Genesis Protocol --join ceremony wrapper.
#
# Wraps the CPace PAKE + SAS short-code confirmation flow for nodes 2+3
# joining an existing PPN mesh seeded by the genesis node.
#
# Consumed by:
#   provision-vm-infrastructure-onprem.sh --join <short-code>
#   provision-vm-infrastructure-cloud.sh  --join <short-code>
#
# Full ceremony specification: BRIEF-PPN-ARCHITECTURE.md §CPace + SAS
# Genesis Protocol overview: BRIEF-VM-ARCHITECTURE.md §3

set -euo pipefail

# Perform a Genesis Protocol join ceremony.
# Usage: ppn_join <short_code> <pairing_server_url>
# The pairing server (service-ppn-pairing) runs on the genesis-seed node at :9202.
ppn_join() {
    local short_code="$1"
    local pairing_server="${2:-http://10.8.0.9:9202}"

    # TODO: implement CPace PAKE exchange + SAS confirmation.
    # Stub: prints the ceremony parameters and exits 1 to prevent
    # accidental use before the implementation is complete.
    echo "ppn_join: Genesis Protocol join ceremony — STUB (not yet implemented)"
    echo "  Short code:     ${short_code}"
    echo "  Pairing server: ${pairing_server}"
    echo ""
    echo "To implement: wire os-infrastructure --join flag through this function."
    echo "See BRIEF-PPN-ARCHITECTURE.md §9.2 Steps 1+4 for full specification."
    return 1
}
