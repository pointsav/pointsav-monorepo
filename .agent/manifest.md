---
schema: foundry-cluster-manifest-v1
cluster: project-knowledge
cluster_name: project-knowledge
cluster_branch: cluster/project-knowledge
created: 2026-05-23
state: active
slm_endpoint: http://localhost:8011
module_id: knowledge

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: >
        app-mediakit-knowledge — Wikipedia-pattern wiki engine; single Rust binary;
        Apache 2.0; three production instances (9090/9093/9095).
      status: active (canonical tip 9a1326df; Phase 0 complete 2026-06-12)
  customer:
    - status: leg-pending
      note: >
        Customer-tier deliverable is the Doctrine §IV.g exception itself:
        media-knowledge-* repos are canonical (GitHub is downstream mirror).
        No woodfine-fleet-deployment catalog entry needed for the engine;
        content repos are the customer-tier artifact.
  deployment:
    - status: active
      note: >
        Three instances on vault-privategit-source-1:
          documentation.pointsav.com  → port 9090 (documentation.toml; TOPIC+GUIDE)
          projects.woodfinegroup.com  → port 9093 (projects.toml; TOPIC)
          corporate.woodfinegroup.com → port 9095 (corporate.toml; TOPIC)
        Binary sha256: e5e8995efc7d6da2f1eba10c235161a90e6c4290aa2b65951c54eb92948c8cd1
        Systemd: local-knowledge-documentation.service, local-knowledge-projects.service,
                 local-knowledge-corporate.service
        Config: /etc/local-knowledge/*.toml
  wiki:
    - status: active
      note: >
        Three content repos (media-knowledge-documentation, media-knowledge-projects,
        media-knowledge-corporate) managed in project-editorial; GitHub is downstream.
        Doctrine §IV.g exception ratified 2026-05-21 (amended 2026-06-11).

clones:
  - repo: media-knowledge-documentation
    role: primary
    path: media-knowledge-documentation/
    upstream: customer/media-knowledge-documentation
  - repo: media-knowledge-projects
    role: primary
    path: media-knowledge-projects/
    upstream: customer/media-knowledge-projects
  - repo: media-knowledge-corporate
    role: primary
    path: media-knowledge-corporate/
    upstream: customer/media-knowledge-corporate
  - repo: woodfine-fleet-deployment
    role: routing
    path: woodfine-fleet-deployment/
    upstream: customer/woodfine-fleet-deployment
