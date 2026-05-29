#!/usr/bin/env bash
# infrastructure/virt/provision-vm-orchestration.sh
#
# VM-Orchestration provisioner — os-orchestration runtime instance.
#
# VM type:     VM-Orchestration
# os-* source: os-orchestration (canonical rename of os-interface; in flight)
# Purpose:     Stateless multi-archive aggregator; commercial paid tier.
#              Hosts: app-orchestration-bim, app-orchestration-gis, app-orchestration-slm
#
# Phase 1 (now):   Ubuntu 24.04 QEMU (TCG/KVM), separate instance from vm-mediakit
# Phase 2:         + gVisor sandboxing for aggregator processes
# Phase 3 target:  NanoVMs for aggregators; fat Linux stays for SLM/GPU inference
#
# Port allocations (guest-side; host port-forwards TBD):
#   app-orchestration-bim  :9096  (depends on VM-Totebox service-fs at :9100)
#   app-orchestration-gis  :9097  (TBD)
#   app-orchestration-slm  :9180
#
# Prerequisites:
#   - VM-Totebox Phase 1 complete (service-fs available; bim-orchestration depends on it)
#   - See BRIEF-VM-ARCHITECTURE.md §6 "VM-Orchestration Phase 1" checklist
#
# Usage:
#   ./provision-vm-orchestration.sh

set -euo pipefail

echo "TODO: provision-vm-orchestration.sh is not yet implemented." >&2
echo "Blocked on: VM-Totebox Phase 1 (service-fs required by app-orchestration-bim)." >&2
echo "See BRIEF-VM-ARCHITECTURE.md §6 VM-Orchestration Phase 1 checklist." >&2
exit 1
