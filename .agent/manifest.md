---
schema: foundry-cluster-manifest-v1
cluster_name: project-command
cluster_branch: cluster/project-command
created: 2026-05-05
state: active (provisioned 2026-05-06; pointsav-monorepo sub-clone ready for Task)
doctrine_version: 0.0.14
doctrine_claims_codified: [37, 43, 44]
doctrine_claims_proposed: []

operator: woodfine (Mathew, Jennifer)
working_pattern: production-first-mvp
input_shape: app-orchestration-command-as-user-aggregator

# Cluster mission per conventions/orchestration-architecture.md
# (ratified 2026-05-05): build app-orchestration-command as the
# user-facing aggregator across Totebox Archives. Production-first
# principle — development topology equals production topology.
# Project-* clusters today are operational shape-tests for future
# Totebox Archives; command is the hub that aggregates them. Two
# users (Jennifer, Mathew) on one side; many archives on the other.
#
# Per Model B (operator ratified 2026-05-05): command is a peer
# app-orchestration-* sibling to existing -bim, -gis,
# -proofreader, future -slm and -content. Each is single-concern;
# command focuses on user/data aggregation, not specialty workflows.
#
# Tier 0 compatible: TUI-first form factor; runs on $7/mo node;
# substrate works without AI (graph queries via Doorman with no
# tier configured still functional).

tetrad:
  vendor:
    - source_repo: pointsav-monorepo
      project_path: app-orchestration-command/
      status: provisioned 2026-05-06; sub-clone at clones/project-command/pointsav-monorepo/ on cluster/project-command branch with three remotes configured (origin admin SSH, origin-staging-j, origin-staging-p); ready for Task to author app-orchestration-command/ scaffolding (CLAUDE.md, NEXT.md, src/, etc.)
  customer:
    - fleet_deployment_repo: vendor/pointsav-fleet-deployment
      catalog_subfolder: gateway-orchestration-command/
      status: leg-pending — catalog folder authoring deferred until v0.0.1 ready
  deployment:
    - path: deployments/gateway-orchestration-command-1/
      status: leg-pending — provisioned when first MVP ships
  wiki:
    - target: vendor/content-wiki-documentation
      drafts_via: clones/project-editorial/.agent/drafts-outbound/
      status: leg-pending — TOPIC drafts staged when MVP demonstrates the architecture

datagraph_module_id: both
# Command queries both pointsav (vendor entities) and woodfine
# (customer entities) per user permissions; explicit module_id
# per call per conventions/datagraph-access-discipline.md

mvp_scope:
  v0.0.1:
    - TUI binary that lists archives the current user has access to
    - User picks an archive
    - Federates a graph query against service-content via Doorman
    - Displays results
  v0.0.2:
    - Cross-archive query federation (one user query → multiple Doorman calls → aggregated result)
    - Permission boundary enforcement
  v0.0.3:
    - Web UI parallel to TUI (Tier 0 compatible)
    - Audit log capture for every user action

cross_cluster_dependencies:
  - project-intelligence: service-content + Doorman (graph endpoints, audit ledger)
  - project-data: Totebox archive data shape (consumer)
  - project-knowledge: media-knowledge-* wiki rendering (potential future integration)

provisioning_notes:
  - pointsav-monorepo sub-clone provisioned 2026-05-06 (468 MB; small; fit cleanly within disk budget). Disk usage 81% → 82% (11 GB free).
  - Cluster branch cluster/project-command created from local upstream main; three remotes configured and fetched.
  - pointsav-design-system + vendor/pointsav-fleet-deployment sub-clones still deferred; first MVP can proceed with pointsav-monorepo alone.
  - Pattern: project-bim/-gis equivalent.
---

# project-command — User-facing aggregator hub

This cluster owns `app-orchestration-command` source — the user-facing aggregator that Jennifer and Mathew use to interact with all Totebox Archives via a single interface. Per the orchestration-architecture convention, this is the production-tier hub; development today must match production tomorrow.

## Status

Manifest authored 2026-05-05. Sub-clones not yet provisioned (deferred to session with disk headroom). Master to provision when ready.

## Cross-references

- `conventions/orchestration-architecture.md` — hub-and-spoke topology
- `conventions/datagraph-access-discipline.md` — graph access pattern
- `.agent/plans/2026-05-05-publishing-tier-naming-cross-check.md` — origin discussion
