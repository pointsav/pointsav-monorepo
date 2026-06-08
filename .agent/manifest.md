---
schema: cluster-manifest-v1
cluster: project-system
opened: 2026-05-14
state: active
slm_endpoint: http://localhost:8011
module_id: editorial
doctrine_version: 0.0.14
doctrine_claims_codified: []
doctrine_claims_proposed: []

operator: pointsav (Mathew, Jennifer)
working_pattern: editorial-pipeline

tetrad:
  vendor:
    repo: pointsav-monorepo
    branch: cluster/project-system
    focus: [app-infrastructure-onprem, app-infrastructure-leased, app-infrastructure-cloud, app-network-admin, os-infrastructure, os-network-admin]
    status: active
  customer:
    - fleet_deployment_repo: customer/woodfine-fleet-deployment
      catalog_subfolder: (varies per originating project)
      status: active — GUIDEs committed here each session; checked out as sibling repo
  deployment:
    status: leg-pending — no running service; editorial is a Totebox process, not a deployed binary
  wiki:
    - target: media-knowledge-documentation
      status: active — TOPIC drafts committed each session; checked out as sibling repo
    - target: media-knowledge-corporate
      status: active — COMMS/corporate content committed here; checked out as sibling repo
    - target: media-knowledge-projects
      status: active — project-narrative topics committed here; checked out as sibling repo

datagraph_module_id: data
cross_cluster_dependencies:
  all-archives: drafts routed to project-editorial via .agent/drafts-outbound/ handoff pattern

provisioning_notes:
  - media-knowledge-documentation: checked out as sibling directory (canonical per DOCTRINE §IV.e)
  - media-knowledge-corporate: checked out as sibling directory (canonical per DOCTRINE §IV.e)
  - media-knowledge-projects: checked out as sibling directory (canonical per DOCTRINE §IV.e)
  - woodfine-fleet-deployment: checked out as sibling directory
  - factory-release-engineering: checked out as sibling directory

session_role: totebox
default_starting_dir: ~/Foundry/clones/project-editorial/
---

# project-system — Cluster Manifest

Editorial pipeline gateway for the Foundry ecosystem. Receives TOPIC/GUIDE/COMMS/JOURNAL/PROSE-RESEARCH drafts from all other Totebox archives, applies Bloomberg-register language and quality passes, and commits to canonical destinations.

## Artifact routing

| Artifact type | Source | Destination |
|---|---|---|
| TOPIC-* | any archive | media-knowledge-documentation / media-knowledge-projects |
| GUIDE-* | any archive | woodfine-fleet-deployment/<cluster>/ |
| COMMS-* | any archive | media-knowledge-corporate |
| JOURNAL-* | any archive | JOURNAL/ (this archive) |
| PROSE-RESEARCH | any archive | review + return or accept |
| DESIGN-* / ASSET-* | (pass-through) | relay to project-design |

## Status (as of 2026-05-31 — from BRIEF-project-console-master.md)

vendor:
  repo: pointsav-monorepo
  branch: cluster/project-system
  focus: [app-infrastructure-onprem, app-infrastructure-leased, app-infrastructure-cloud, app-network-admin, os-infrastructure, os-network-admin]
  status: active

Running in production at `documentation.pointsav.com` (port 9090) and
`projects.woodfinegroup.com` (port 9093) on vault-privategit-source-1.

- `~/Foundry/AGENT.md` §Artifacts — full artifact-type routing table
- `conventions/artifact-classification.yaml` — machine-readable routing
- `conventions/cluster-wiki-draft-pipeline.md` — full editorial pipeline spec
- `.agent/rules/journal-artifact-discipline.md` — JOURNAL manuscript rules
