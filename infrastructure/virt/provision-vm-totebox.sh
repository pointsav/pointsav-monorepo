#!/usr/bin/env bash
# infrastructure/virt/provision-vm-totebox.sh
#
# VM-Totebox provisioner — os-totebox runtime instance.
#
# VM type:     VM-Totebox
# os-* source: os-totebox
# Purpose:     Per-entity sovereign WORM vault + Ring 1+2 services
#              (service-fs, service-people, service-extraction, etc.)
#
# Phase 1 (now):   Ubuntu 24.04 QEMU (TCG/KVM)
# Phase 2:         Alpine Linux + musl-static binary
# Phase 3 target:  NanoVMs/OPS unikernel or seL4 Microkit (AArch64-only;
#                  gated on AArch64 hw acquisition decision)
#
# Prerequisites:
#   - service-fs binary available on host (blocked: project-data 23-commit promotion)
#   - system-core + system-ledger available (blocked: project-system outbox)
#   - See BRIEF-VM-ARCHITECTURE.md §6 "VM-Totebox Phase 1" checklist
#
# Usage:
#   ./provision-vm-totebox.sh [--entity <name>] [--port <base-port>]

set -euo pipefail

echo "TODO: provision-vm-totebox.sh is not yet implemented." >&2
echo "Blocked on: project-data service-fs binary promotion (23 commits, Command Session)." >&2
echo "See BRIEF-VM-ARCHITECTURE.md §6 VM-Totebox Phase 1 checklist." >&2
exit 1
