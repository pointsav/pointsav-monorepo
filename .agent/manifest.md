---
schema: foundry-cluster-manifest-v1
cluster: project-editorial
cluster_name: project-editorial
cluster_branch: cluster/project-editorial
created: 2026-05-01
state: active
slm_endpoint: http://localhost:8011
module_id: editorial

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: >
        system-core/ + system-ledger/ + system-ledger-proto/ + system-ledger-server/
        + system-ledger-pd/ + system-substrate/ + system-substrate-netbsd/
        + service-vm-fleet/ + service-vm-tenant/ + service-vm-host/
        + service-extraction/ + os-totebox/ + app-privategit-source/
        + app-privategit-marketplace/ + tool-wallet/
      status: active (Phase 1 UEFI boot complete 2026-06-12; Phase 2 in progress)
  customer:
    - status: leg-pending
      note: >
        GUIDE entries for os-totebox deployment procedures and PPN fleet operations
        planned for woodfine-fleet-deployment once Phase 2 hardening is complete.
  deployment:
    - status: active
      note: >
        os-totebox Phase 1 UEFI boot milestone COMPLETE 2026-06-12 (commit 92692800).
        NetBSD 10.1 multiuser under QEMU TCG on GCP e2; system-ledger-server + sshd up;
        /healthz 200 + /readyz 503 COLD. Phase 2 (Veriexec strict=1) in progress.
  wiki:
    - status: leg-pending
      note: >
        TOPICs for Capability Ledger Substrate architecture, PPN VM fleet architecture,
        and seL4 PD design to be written after Phase 2 complete;
        route via project-editorial.

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
