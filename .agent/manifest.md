---
schema: foundry-cluster-manifest-v1
cluster: project-console
cluster_name: project-console
cluster_branch: cluster/project-console
created: 2026-05-27
state: active
slm_endpoint: http://localhost:9080
module_id: console

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: >
        os-console/ (SSH TUI binary) + app-console-content/ + app-console-email/ +
        app-console-input/ + app-console-keys/ + app-console-people/ +
        app-console-slm/ + app-console-system/ (active cartridges, Phase 8 complete)
      status: active (Phase 8 complete 2026-06-13; Phase 9 deployment-blocked)
  customer:
    - status: leg-pending
      note: >
        woodfine-fleet-deployment catalog entry planned when os-console binary
        is deployed to vault-privategit-source-1 (Phase 9 gate).
  deployment:
    - status: leg-pending
      note: >
        Target: vault-privategit-source-1, port 2222 (SSH server mode).
        Blocked on GCE firewall port 2222 (operator action) + vm-intelligence
        WireGuard provisioning (project-infrastructure).
        Systemd units drafted: infrastructure/systemd/console/local-console.service
        + local-pairing-server.service.
  wiki:
    - status: leg-pending
      note: >
        TOPIC-os-console-architecture.draft.md + .es.draft.md staged in
        .agent/drafts-outbound/; route to project-editorial for refinement
        and commit to content-wiki-documentation.

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
