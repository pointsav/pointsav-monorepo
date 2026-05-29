#!/usr/bin/env bash
# infrastructure/virt/provision-vm-privategit.sh
#
# VM-PrivateGit provisioner — os-privategit runtime instance.
#
# VM type:     VM-PrivateGit
# os-* source: os-privategit
# Purpose:     Sovereign source control + design system hosting.
#              Replaces third-party Git for IP + brand assets.
#              Hosts: app-privategit-source-control (Gitea), app-privategit-design-system (Storybook)
#
# Phase 1 (now):   Ubuntu 24.04 QEMU (TCG/KVM)
# Phase 2:         FreeBSD jail around Gitea
# Phase 3 target:  gVisor or seL4 microVM
#
# Deployment note: The Foundry workspace (vault-privategit-source-1) is the first
# VM-PrivateGit deployment. It currently runs on the GCP host directly, not in a VM.
# This script provisions the VM form factor for customer deployments.
#
# Usage:
#   ./provision-vm-privategit.sh

set -euo pipefail

echo "TODO: provision-vm-privategit.sh is not yet implemented." >&2
echo "Future work — no current blockers; sequenced after VM-Totebox + VM-Orchestration." >&2
echo "See BRIEF-VM-ARCHITECTURE.md §6 VM-PrivateGit Phase 1 checklist." >&2
exit 1
