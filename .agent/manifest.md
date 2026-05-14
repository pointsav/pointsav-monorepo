---
schema: cluster-manifest-v1
cluster: project-infrastructure
opened: 2026-05-14
state: active
slm_endpoint: http://localhost:8011
module_id: infrastructure
---

# project-infrastructure — Cluster Manifest

## Mission

PPN cartridges and network OS work — the software layer that constitutes the
PointSav Private Network and the infrastructure nodes that run it.

This cluster is the dedicated Totebox Session for all work on the
`app-infrastructure-*`, `app-network-admin`, `os-infrastructure`, and
`os-network-admin` crates.

## Tetrad

vendor:
  repo: pointsav-monorepo
  branch: cluster/project-infrastructure
  focus: [app-infrastructure-onprem, app-infrastructure-leased, app-infrastructure-cloud, app-network-admin, os-infrastructure, os-network-admin]
  status: active

customer:
  repo: woodfine-fleet-deployment
  focus: [fleet-infrastructure-onprem, fleet-infrastructure-leased, fleet-infrastructure-cloud, route-network-admin]
  status: leg-pending

deployment:
  instances: [fleet-infrastructure-onprem-1, fleet-infrastructure-leased-1, fleet-infrastructure-cloud-1, route-network-admin-1]
  status: leg-pending — gated on WireGuard Part A

wiki:
  target: content-wiki-documentation
  planned_topics: [topic-ppn-architecture.md, topic-os-infrastructure.md, topic-os-network-admin.md]
  status: leg-pending

## Notes

- WireGuard Part A gates the full three-node topology
- Master key authority resides physically on Laptop A — never delegated to cloud
- Layer 3 instance configs in ~/Foundry/deployments/fleet-infrastructure-*-1/ are local-only, gitignored
- New crates (app-infrastructure-*, app-network-admin) scaffolded in project-intelligence
  commit 0cbf81d (2026-05-14) — this cluster is the intended working home going forward
