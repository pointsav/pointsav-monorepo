---
schema: foundry-cluster-manifest-v1
cluster: project-data
cluster_name: GIS Data Pipeline
cluster_branch: cluster/project-data
created: 2026-05-07
state: active (v0-1 scoring live; Stage 6 BLOCKED — 376 commits behind canonical; rebase request outstanding)
slm_endpoint: http://localhost:9080
module_id: data
doctrine_version: 0.0.14
doctrine_claims_codified: [37]
doctrine_claims_proposed: []

tetrad:
  vendor: pointsav-monorepo (cluster/project-data) — GIS pipeline crates
  customer: woodfine-fleet-deployment/gis — leg-pending
  deployment: gateway-orchestration-gis-1 (live; gis.woodfinegroup.com)
  wiki: content-wiki-documentation — leg-pending

datagraph_module_id: data
cross_cluster_dependencies:
  - project-gis (AEC scripts; GIS map tile generation; overlapping domain)
  - project-system (infrastructure dependencies)

provisioning_notes: |
  Archive cloned from pointsav-monorepo cluster/project-data branch.
  Working in: ~/Foundry/clones/project-data/
  Sub-clone (monorepo): ~/Foundry/clones/project-data/pointsav-monorepo/
  Stage 6 BLOCKED: rebase against canonical main required before next promote.
  Rebase request outstanding — see NEXT.md and outbox for status.

session_role: totebox
default_starting_dir: ~/Foundry/clones/project-data/
---

## Cluster mission

Maintain the GIS co-location data pipeline that powers `gis.woodfinegroup.com` —
a scored property-suitability map for retail and commuter-transit site selection.

### Pipeline overview

Two scoring archetypes:
- **PKS** (Commuter / airport-led): transit-proximity weighting; DBSCAN clustering on transit nodes
- **VWH** (Retail-density): foot-traffic and retail-density weighting; DBSCAN on commercial clusters

Output format: **pmtiles** (Protocol Buffers + Map Tiles) served via
`gateway-orchestration-gis-1` at `gis.woodfinegroup.com`.

V2 scoring (0–1000 scale) is live in production tiles.

### Current blockers

**Stage 6 BLOCKED:** cluster/project-data branch is 376+ commits behind canonical `main`
in pointsav-monorepo. A `git fetch + git rebase origin/main` is required before the next
`bin/promote.sh` call. Rebase request sent to Command Session via outbox.

Do not attempt `bin/promote.sh` until the rebase is confirmed complete.
