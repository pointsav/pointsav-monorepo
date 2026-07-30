---
schema: foundry-cluster-manifest-v1
cluster: project-console
cluster_branch: main
created: 2026-04-23
state: active
slm_endpoint: http://localhost:9080
module_id: console
doctrine_version: 0.0.10
doctrine_claims_codified: [37]
publication_gate: operator-explicit

operator: jennifer
working_pattern: application-development
input_shape: rust-monorepo-crates
spec_via_operation: false

# This cluster owns the os-console TUI binary and app-console-* cartridge crates.
# Primary focus areas:
# - os-console — keyboard-native TUI binary (ratatui + crossterm); F-key cartridge chassis
# - app-console-* cartridges — F2 People, F3 Email, F4 Content, F6 Bookkeeper, F9 SLM, F11 System, F12 Input
# - seL4 unikernel substrate (moonshot-toolkit v0.3.1, moonshot-sel4-vmm Phase H8 complete 2026-06-20)
# - Phase H9 next: VirtIO serial PD + ratatui in seL4

tetrad:
  vendor:
    - repo: pointsav-monorepo (archive root git)
      path: ./
      upstream: main (Stage 6)
      focus: |
        os-console TUI binary + app-console-* cartridges
          F2 People, F3 Email, F4 Content, F6 Bookkeeper, F9 SLM, F11 System, F12 Input
        seL4 substrate — moonshot-toolkit v0.3.1, moonshot-sel4-vmm Phase H8 complete 2026-06-20
        Phase H9 planned: VirtIO serial PD; Phase 11 planned: F7 BIM cartridge
  customer:
    - status: leg-pending
      note: >
        No woodfine-fleet-deployment catalog entries committed yet.
        os-console deployment guide planned as GUIDE artifact when binary is deployed.
  deployment:
    - status: leg-pending
      note: >
        Target: vault-privategit-source-1; SSH server mode port 2222.
        Blocked on GCE firewall port 2222 (operator action).
  wiki:
    - repo: vendor/content-wiki-documentation
      drafts_via: clones/project-console/.agent/drafts-outbound/
      gateway: project-editorial
      status: active
      note: >
        TOPIC-geometric-protection, TOPIC-os-console-totebox-browser,
        TOPIC-sel4-unikernel-substrate, TOPIC-three-binary-architecture staged 2026-06-19;
        routed to project-editorial.

clones: []

adapter_routing:
  trains:
    - cluster-project-console
    - tenant-woodfine
  consumes:
    - constitutional-doctrine
    - engineering-pointsav
    - cluster-project-console
    - tenant-woodfine
    - role-task

cross_cluster_dependencies:
  - cluster: project-system
    why: seL4 Phase H work — moonshot-toolkit + moonshot-sel4-vmm crates live in this archive
    interface: archive root; Phase H9 VirtIO serial PD next
