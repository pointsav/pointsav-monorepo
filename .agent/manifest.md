---
schema: foundry-cluster-manifest-v1
cluster: project-gis
cluster_name: project-gis
cluster_branch: main
created: 2026-04-01
state: active
module_id: gis
slm_endpoint: http://localhost:9080

tetrad:
  vendor:
    repo: pointsav-monorepo
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
    crates: [app-orchestration-gis]
    state: active
  customer:
    repo: woodfine-fleet-deployment
    path: gateway-orchestration-gis-1/
    state: active
  deployment:
    name: gateway-orchestration-gis-1
    host: vault-privategit-source-1
    surface: gis.woodfinegroup.com
    state: active
  wiki:
    repo: content-wiki-projects
    state: active

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
    branch: main
---

# project-gis — Cluster Manifest

Location Intelligence GIS cluster. Builds and deploys the co-location archetype
system (PRO / VWH / PKS), the Top 400 Regional Markets dataset, catchment and
O-D analysis pipelines, AEC enrichment layers (Köppen, ecoregion, GHI, seismic,
flood, wildfire), and the journal research programme (J1 Retail Co-location,
J7 Urban Fringe, J8 Commuter). Primary surface: gis.woodfinegroup.com
(gateway-orchestration-gis-1). All pipelines run from
pointsav-monorepo/app-orchestration-gis/. Nightly rebuild: 22:00 PDT.
