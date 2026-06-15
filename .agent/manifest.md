---
schema: foundry-cluster-manifest-v1
cluster: project-knowledge
cluster_name: project-knowledge
cluster_branch: main
created: 2026-05-27
state: active
slm_endpoint: http://localhost:9080
module_id: knowledge

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: >
        app-mediakit-knowledge/ (Wikipedia-pattern wiki engine; Apache 2.0;
        Phases 0–9 shipped; E5/E6/E7 Stage 6 pending)
      status: active
  customer:
    - status: leg-pending
      note: >
        Three live instances exist (local-knowledge-{documentation,projects,corporate})
        but no woodfine-fleet-deployment catalog entries have been committed yet.
        Catalog entries planned as part of Phase 6 deployment split ratification.
  deployment:
    - status: active
      note: >
        Three live instances on vault-privategit-source-1:
        port 9090 (documentation.pointsav.com),
        port 9093 (projects.woodfinegroup.com),
        port 9095 (corporate.woodfinegroup.com).
        content-wiki-{documentation,projects,corporate} are canonical content repos.
  wiki:
    - status: active
      note: >
        Editorial drafts dispatched from .agent/drafts-outbound/ to project-editorial.
        Committed articles live in media-knowledge-documentation and
        media-knowledge-projects. TOPIC/GUIDE pipeline active.

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
