---
schema: foundry-cluster-manifest-v1
cluster: project-system
cluster_branch: main
created: 2026-04-23
state: active
slm_endpoint: http://localhost:9080
module_id: system
doctrine_version: 0.0.10
doctrine_claims_codified: [37]
publication_gate: operator-explicit

operator: jennifer
working_pattern: infrastructure-development
input_shape: rust-monorepo-crates
spec_via_operation: false

# This cluster owns the PointSav Rust monorepo (pointsav-monorepo).
# Primary focus areas:
# - PPN VM fleet stack (service-vm-fleet, service-vm-host, service-vm-tenant, system-vm-fleet-types)
# - os-totebox unikernel VM image (NetBSD; Veriexec; Phase 2 complete 2026-06-14)
# - seL4 unikernel substrate (moonshot-toolkit, moonshot-sel4-vmm; Phase H1 complete 2026-06-19)
# - app-console-* cartridges and os-console TUI
# - All other monorepo workspace members

tetrad:
  vendor:
    - repo: pointsav-monorepo (archive root git)
      path: ./
      upstream: main (Stage 6)
      focus: |
        PPN VM fleet stack — service-vm-fleet (:9203), service-vm-host, service-vm-tenant (:9221),
          system-vm-fleet-types
        os-totebox — NetBSD unikernel VM image; Phase 2 complete 2026-06-14
        seL4 substrate — moonshot-toolkit v0.3.1, moonshot-sel4-vmm Phase H1 complete 2026-06-19
        app-console-* cartridges + os-console TUI
        All other monorepo workspace members
  customer:
    - status: leg-pending
      note: >
        No woodfine-fleet-deployment catalog entries committed yet.
        os-totebox deployment docs planned as GUIDE artifacts.
  deployment:
    - status: leg-pending
      note: >
        os-totebox instances are provisioned per-node; not yet cataloged in fleet-deployment.
        service-vm-fleet/host/tenant run on vault-privategit-source-1 (ports 9203, 9221).
  wiki:
    - repo: vendor/content-wiki-documentation
      drafts_via: clones/project-system/.agent/drafts-outbound/
      gateway: project-editorial
      status: active
      note: >
        9 TOPICs promoted to canonical media-knowledge-documentation 2026-06-19 (commit 2222e42).
        5 additional TOPIC drafts staged in drafts-outbound (A2–A6).

clones: []

adapter_routing:
  trains:
    - cluster-project-system
    - tenant-woodfine
  consumes:
    - constitutional-doctrine
    - engineering-pointsav
    - cluster-project-system
    - tenant-woodfine
    - role-task

cross_cluster_dependencies:
  - cluster: project-console
    why: seL4 Phase H1 parallel work; os-console-hello.toml gate passed; Phase H2 VirtIO serial PD planned
    interface: outbox coordination → project-system provides toolkit + vmm crates
  - cluster: project-data
    why: seL4 Phase H1 parallel work; os-totebox is confirmed PD target (BRIEF-os-totebox-build-out)
    interface: os-totebox-hello.toml spec delivered 2026-06-20 (commit 23b7026d5); Stage 6 pending
