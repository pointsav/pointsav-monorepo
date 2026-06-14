---
schema: foundry-cluster-manifest-v1
cluster: project-bim
cluster_name: project-bim
cluster_branch: main
created: 2026-04-23
state: active
slm_endpoint: http://localhost:9080
module_id: bim

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: >
        app-orchestration-bim (live, v0.0.2) +
        app-privategit-bim (clean-sheet rewrite planned, port 9204 on os-privategit) +
        service-bim (Reserved-folder, archive daemon) +
        app-console-bim (Reserved-folder, coordination terminal) +
        app-workplace-bim (Reserved-folder, Wave 3 editor)
      status: active (app-orchestration-bim deployed; app-privategit-bim rewrite planned 2026-06-14)
    - repo: woodfine-bim-library
      path: woodfine-bim-library/
      upstream: woodfine/woodfine-bim-library
      focus: >
        BIM Object vault: 17 DTCG schema files, 18 IFC Key Plan compositions,
        tiles-registry.md, key-plans-registry.md (66 entries), nightly furniture pipeline
      status: active (3 commits ahead of origin — Stage 6 push pending operator auth)
  customer:
    - repo: woodfine-fleet-deployment
      path: woodfine-fleet-deployment/
      upstream: woodfine/woodfine-fleet-deployment
      focus: >
        gateway-orchestration-bim/ deployment guides and MANIFEST.md
      status: active (gateway-orchestration-bim-1 live at bim.woodfinegroup.com)
  deployment:
    - instance: gateway-orchestration-bim-1
      host: bim.woodfinegroup.com
      port: 9096
      binary: app-orchestration-bim
      status: active (live; keep running throughout app-privategit-bim rewrite)
      note: >
        Second deployment planned: app-privategit-bim on vault-privategit-source-1
        port 9204 once clean-sheet rewrite passes verification.
  wiki:
    - status: leg-pending
      note: >
        BIM product family material staged in handoffs-outbound.md for
        content-wiki-documentation. JOURNAL-aec-data-layers (J3) in JOURNAL/ directory.
        Architecture TOPICs pending when app-privategit-bim v1 ships.

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
  - repo: woodfine-bim-library
    role: bim-object-vault
    path: woodfine-bim-library/
    upstream: woodfine/woodfine-bim-library
  - repo: woodfine-fleet-deployment
    role: customer-deployment-guides
    path: woodfine-fleet-deployment/
    upstream: woodfine/woodfine-fleet-deployment
