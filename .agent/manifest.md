---
schema: foundry-cluster-manifest-v1
cluster: project-console
cluster_name: project-console
cluster_branch: main
created: 2026-05-06
state: active (Phases C/D/E/6/7/A/B complete; Phase 8+ pending; Phase B Stage 6 promote pending Command)
slm_endpoint: http://localhost:9080
module_id: console
doctrine_version: 0.0.14
doctrine_claims_codified: [45, 49, 54]

operator: pointsav (Mathew, Jennifer)
working_pattern: production-first-mvp

tetrad:
  vendor:
    - source_repo: pointsav-monorepo
      project_path: app-console-* / os-console/
      cluster_branch: cluster/project-console
      status: Active — Phases C/D/E/6/7/A/B complete; Phase B commits (6f21f580 + 2 ops) pending Stage 6 promote
  customer:
    - fleet_deployment_repo: woodfine-fleet-deployment
      catalog_subfolder: vm-intelligence-1/
      status: leg-pending — deployment guide pending Phase 9 milestone
  deployment:
    - unit: os-console SSH server
      host: vm-intelligence (WireGuard 10.42.1.1)
      port: 2222
      status: planned — blocked on project-infrastructure WireGuard provisioning + GCE firewall port 2222
    - unit: pairing-server
      host: vm-intelligence
      port: 9201
      status: planned — systemd unit not yet authored
  wiki:
    - target: content-wiki-documentation
      status: leg-pending — TOPIC-os-console-architecture not yet authored

datagraph_module_id: console
cross_cluster_dependencies:
  - project-infrastructure: WireGuard provisioning for vm-intelligence (port 2222 deployment gate)
  - project-editorial: TOPIC/GUIDE drafts from this archive route to project-editorial

session_role: task
default_starting_dir: ~/Foundry/clones/project-console/
---

# project-console — SSH TUI console gateway to the Totebox platform

This cluster owns the `app-console-*` cartridge ecosystem and `os-console` — the Rust
SSH server that delivers the operator TUI for Totebox deployments. The console connects
to every running service through a set of F-key cartridges (F1–F12), each scoped to one
domain: keys (F1), content (F2), email (F3), SLM/Doorman (F9), system/pairing (F11).

## Status (as of 2026-06-12)

Phases C–E, 6–7, and cross-platform A–B complete:
- Phase C: app-console-email F3 lib crate — inbox list, read, compose/send, plain mode
- Phase D: app-console-slm F9 lib crate — Doorman health dashboard, circuit state, 10s poll
- Phase E: Orchestration wiring audited; clean mba_client; added to ConsoleConfig
- Phase 6: Offline mode + Tantivy search (/readyz poll; greyed widgets; /search command)
- Phase 7: PDF viewing via pdfium-render → Kitty/Sixel; text fallback; /pdf command
- Phase A: Doorman port 9080 fix; configurable endpoints; GitHub Actions Linux CI
- Phase B: 4-target release matrix (Linux musl, macOS Intel, ARM, universal); rustls-tls; TerminalCaps probe

Pending: Phase 8 (Polish) + Phase 9 (Operations); pairing-server + os-console systemd units;
first internet-facing SSH deployment to vm-intelligence provisioned by project-infrastructure.

Stage 6 (Phase B): commits 6f21f580 + 2 ops commits pending Command promote (authorized 2026-05-28).

- `BRIEF-os-console-active-dev.md` — active state tracker for Phases 8–10; read before each session
- `BRIEF-os-console-platform.md` — F-key map, Cartridge trait, MBA topology, platform targets (in monorepo sub-clone)
- `conventions/orchestration-architecture.md` — Cartridge composition pattern
- `.agent/rules/datagraph-discipline.md` — entity lookups via Doorman :9080
