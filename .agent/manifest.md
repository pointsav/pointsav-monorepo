---
cluster: project-design
cluster_name: project-design
cluster_branch: cluster/project-design
created: 2026-06-09
state: active
module_id: design

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: app-privategit-design (design.pointsav.com binary)
      status: active (MVP deployed 2026-06-08 commit e16545e8; Stage 6 pending)
    - repo: pointsav-design-system
      path: pointsav-design-system/
      upstream: vendor/pointsav-design-system
      focus: design tokens, component guides, DESIGN-RESEARCH and DESIGN-TOKEN-CHANGE intake
      status: active (tip 5e2a854; Stage 6 pending as of 2026-06-08)
    - repo: pointsav-media-assets
      path: pointsav-media-assets/
      upstream: vendor/pointsav-media-assets
      status: active
    - repo: woodfine-media-assets
      path: woodfine-media-assets/
      upstream: customer/woodfine-media-assets
      status: active (tip a752b21; push pending as of 2026-06-08)
  customer:
    - status: leg-pending
      note: >
        woodfine-fleet-deployment catalog entry planned when design.pointsav.com
        reaches a public-distribution milestone.
  deployment:
    - status: active
      note: >
        vault-privategit-design-1 at deployments/vault-privategit-design-1/.
        Service: local-design.service, port 9094. Binary: app-privategit-design
        (dynamic nav + pulldown-cmark MVP, commit e16545e8, deployed 2026-06-08).
  wiki:
    - status: leg-pending
      note: >
        DESIGN-RESEARCH files accumulated in pointsav-design-system/dtcg-vault/research/.
        Public-facing TOPICs to be drafted and routed via project-editorial when
        design system reaches a stable public release.

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
  - repo: pointsav-design-system
    role: design-system
    path: pointsav-design-system/
    upstream: vendor/pointsav-design-system
  - repo: pointsav-media-assets
    role: media-assets
    path: pointsav-media-assets/
    upstream: vendor/pointsav-media-assets
  - repo: woodfine-media-assets
    role: media-assets
    path: woodfine-media-assets/
    upstream: customer/woodfine-media-assets
