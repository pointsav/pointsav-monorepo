---
schema: foundry-cluster-manifest-v1
cluster_name: project-orchestration
cluster_branch: cluster/project-orchestration
created: 2026-05-08
state: active
doctrine_version: 0.1.x
doctrine_claims_codified: [37, 43, 44, 52]
doctrine_claims_proposed: []

operator: woodfine (Mathew)
working_pattern: production-first-mvp
input_shape: totebox-orchestration-transition-phases-1-3

slm_endpoint: http://localhost:8011
module_id: orchestration

# Cluster mission:
# Implement the Totebox Orchestration transition — Phases 1, 2, and 3.
#
# Phase 1 (vocabulary): update CLAUDE.md §11, AGENT.md, bin/claude-role.sh to
#   use Command/Totebox vocabulary; add MANIFEST.md "As a Totebox Orchestration"
#   section; correct user-guide article on NetworkAdminOS/MBA.
#
# Phase 2 (formalize): create pairings.yaml; add slm_endpoint to all 13 cluster
#   manifests; create slm/ dirs; provision project-source + project-woodfine archives.
#
# Phase 3 (instrument): write bin/open-archive.sh, bin/list-archives.sh;
#   scaffold app-orchestration-command v0.0.1 in this cluster's pointsav-monorepo.

tetrad:
  vendor:
    - source_repo: pointsav-monorepo
      project_path: app-orchestration-command/
      status: sub-clone provisioned 2026-05-08 on cluster/project-orchestration branch; awaiting Task session to author app-orchestration-command/ scaffolding
  customer:
    - fleet_deployment_repo: woodfine-fleet-deployment
      catalog_subfolder: gateway-orchestration-command/
      status: leg-pending — guides to be authored after v0.0.1 ships
  deployment:
    - instance_name: orchestration-command-1
      instance_path: ~/Foundry/deployments/orchestration-command-1/
      status: leg-pending — provision after app-orchestration-command v0.0.1 compiles
  wiki:
    - target_repo: content-wiki-documentation
      articles:
        - architecture/totebox-orchestration-development.md
        - architecture/pairing-as-permission.md
        - systems/os-orchestration.md
        - architecture/totebox-session.md
        - architecture/personnel-permissions.md
      status: leg-pending — drafts routed to project-editorial 2026-05-08
