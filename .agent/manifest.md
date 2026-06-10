---
schema: foundry-cluster-manifest-v1
cluster: project-console
cluster_name: project-marketing
cluster_branch: cluster/project-marketing
created: 2026-05-06
state: active (v0.0.1 MVP shipped 2026-05-06; cargo check clean; bootstrap + deploy pending Master)
slm_endpoint: http://localhost:8011
module_id: knowledge
doctrine_version: 0.0.14
doctrine_claims_codified: [29]

operator: pointsav (Mathew, Jennifer)
working_pattern: production-first-mvp
input_shape: app-mediakit-knowledge-wikipedia-pattern-wiki

# Cluster mission: build and maintain app-mediakit-knowledge — the
# Wikipedia-pattern HTTP knowledge wiki for os-mediakit. Serves three
# content-wiki-* repositories as fully navigable wikis:
#
#   - documentation.pointsav.com (content-wiki-documentation, port 9090)
#   - projects.woodfinegroup.com  (content-wiki-projects,      port 9093)
#   - corporate.woodfinegroup.com (content-wiki-corporate,     port 9095, planned)
#
# Single Rust binary; optional SQLite auth DB (bundled; no runtime system
# dependencies). Substrate substitution for MediaWiki per Doctrine claim #29.
#
# Phase 6 (three-instance deployment split) is gated on content-wiki-*
# GitHub rename + MASTER Doctrine amendment. Phases 1–8 are shipped.
# Phase 9 (production deploy of Phase 8 binary) is the next milestone.
#
# Deploy targets: vault-privategit-source-1 (GCE VM, same host as other
# local-* services). Systemd units: local-knowledge-documentation.service
# (port 9090), local-knowledge-projects.service (port 9093),
# local-knowledge-corporate.service (port 9095, planned).

tetrad:
  vendor:
    - source_repo: pointsav-monorepo
      project_path: app-mediakit-knowledge/
      status: Active — Phases 1–8 shipped; sub-clone cluster branch pending Stage 6 promote
        (commit chain ff7cd16d → 64f07900 → dc15a93f on local main; Command to promote)
  customer:
    - fleet_deployment_repo: woodfine-fleet-deployment
      catalog_subfolder: gateway-knowledge-documentation-1/
      status: active — guide-knowledge-wiki-deployment.md staged; pending Command routing
    - fleet_deployment_repo: woodfine-fleet-deployment
      catalog_subfolder: gateway-knowledge-projects-1/
      status: leg-pending — no catalog entry yet; Phase 6 milestone
    - fleet_deployment_repo: woodfine-fleet-deployment
      catalog_subfolder: gateway-knowledge-corporate-1/
      status: leg-pending — no catalog entry yet; Phase 6 milestone
  deployment:
    - unit: local-knowledge-documentation.service
      host: vault-privategit-source-1
      port: 9090
      domain: documentation.pointsav.com
      status: running (Phase 5 binary); Phase 9 binary pending Stage 6 promote
    - unit: local-knowledge-projects.service
      host: vault-privategit-source-1
      port: 9093
      domain: projects.woodfinegroup.com
      status: running (Phase 5 binary); Phase 9 binary pending Stage 6 promote
    - unit: local-knowledge-corporate.service
      host: vault-privategit-source-1
      port: 9095
      domain: corporate.woodfinegroup.com
      status: planned — Phase 6 milestone (gated on content-wiki-* rename + Doctrine amendment)
  wiki:
    - target: pointsav-monorepo (content-wiki-documentation sub-clone)
      mount_role: primary — TOPIC + GUIDE blueprints
      status: active — served at documentation.pointsav.com
    - target: pointsav-monorepo (content-wiki-projects sub-clone)
      mount_role: primary — projects wiki
      status: active — served at projects.woodfinegroup.com
    - target: pointsav-monorepo (content-wiki-corporate sub-clone)
      mount_role: primary — corporate wiki
      status: planned — Phase 6 milestone

datagraph_module_id: knowledge

datagraph_module_id: data
cross_cluster_dependencies:
  - project-design: DESIGN-TOKEN-CHANGE cosign required before Phase 9 deploy
  - project-editorial: TOPIC/GUIDE drafts from this archive route to project-editorial

provisioning_notes:
  - pointsav-monorepo sub-clone: provisioned on cluster/project-knowledge branch (3 remotes)
  - content-wiki-documentation: sub-clone present; serves as primary wiki mount
  - content-wiki-projects: sub-clone present; serves as secondary wiki mount
  - content-wiki-corporate: sub-clone present; Phase 6 tertiary mount (planned)

session_role: totebox
default_starting_dir: ~/Foundry/clones/project-knowledge/
---

# project-knowledge — Wikipedia-pattern knowledge wiki

This cluster owns `app-mediakit-knowledge` source — the Rust binary serving
`content-wiki-documentation`, `content-wiki-projects`, and `content-wiki-corporate`
as fully navigable Wikipedia-pattern wikis. Substrate substitution for MediaWiki
per Doctrine claim #29.

### Pipeline overview

Phases 1–8 shipped. Phase 9 (production deploy) is the next milestone, gated on:
1. Stage 6 promote of sub-clone commit chain (ff7cd16d → 64f07900 → dc15a93f)
2. DESIGN-TOKEN-CHANGE master_cosign from Command

Running in production at `documentation.pointsav.com` (port 9090) and
`projects.woodfinegroup.com` (port 9093) on vault-privategit-source-1.

Output format: **pmtiles** (Protocol Buffers + Map Tiles) served via
`gateway-orchestration-gis-1` at `gis.woodfinegroup.com`.

- `app-mediakit-knowledge/CLAUDE.md` — project-level state + phase table
- `app-mediakit-knowledge/ARCHITECTURE.md` — phase plan, conventions
- `.agent/drafts-outbound/PHASE-9-DEPLOY-CHECKLIST.md` — Phase 9 deploy procedure
- `.agent/drafts-outbound/DESIGN-TOKEN-CHANGE-knowledge-platform-theming.draft.md` — pending cosign
- `conventions/compounding-substrate.md` — sovereign + Tier 0 + optional intelligence
