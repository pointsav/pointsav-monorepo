---
schema: foundry-cluster-manifest-v1
cluster: project-design
cluster_branch: cluster/project-design
created: 2026-04-23
state: active
slm_endpoint: http://localhost:9080
module_id: design
doctrine_version: 0.0.10
doctrine_claims_codified: [37]
publication_gate: operator-explicit

operator: jennifer
working_pattern: design-system-development
input_shape: dtcg-token-files-plus-component-guides
spec_via_operation: false

# This cluster owns the PointSav Design System browser and the
# DTCG token repository. app-privategit-design is a Rust/axum SSR
# web application serving design.pointsav.com; it reads the
# pointsav-design-system sub-clone as a vault of DTCG token JSON
# and renders schema-aware views for COMPONENT, TOKEN, RESEARCH,
# MARKETING, and BUNDLE artifact types.
#
# pointsav-design-system (sub-clone) is the DTCG 2025.10 token
# source of truth — dtcg-bundle.json contains all primitive,
# semantic, component, and workplace token groups.
#
# Design asset pipelines (woodfine-media-assets, pointsav-media-assets)
# are also managed from this cluster via staging-tier commit + promote.

tetrad:
  vendor:
    - repo: project-design (archive root git)
      path: ./
      upstream: cluster/project-design → main (Stage 6)
      focus: |
        app-privategit-design — design system browser + DTCG token API
          * Schema-aware rendering (COMPONENT, TOKEN, RESEARCH, MARKETING, BUNDLE)
          * Sovereign inotify FS-watch (moonshot-fs-watch)
          * SSE live-reload sidebar (moonshot-index)
          * WYSIWYG edit overlay (PUT vault save-back)
          * AI bridge: DoormanOlmo + ClaudeCloud SSE relay
  customer:
    - repo: pointsav-design-system (sub-clone)
      path: pointsav-design-system/
      upstream: vendor/pointsav-design-system
      focus: |
        DTCG 2025.10 token definitions
          * dtcg-bundle.json — primitive + semantic + component + workplace groups
          * components/ — DESIGN-COMPONENT guides
          * research/ — DESIGN-RESEARCH files
          * assets/ — ASSET files
  deployment:
    - service: local-design.service
      port: 9094
      binary: /usr/local/bin/app-privategit-design
      version: "0.2.0"
      url: design.pointsav.com (nginx reverse-proxy)
      status: active
  wiki:
    - repo: vendor/content-wiki-documentation
      drafts_via: clones/project-design/.agent/drafts-outbound/
      gateway: project-editorial
      planned_topics: []
      status: leg-pending

clones:
  - repo: pointsav-design-system
    role: customer-token-source
    path: pointsav-design-system/
    upstream: vendor/pointsav-design-system
    focus: DTCG token definitions

adapter_routing:
  trains:
    - cluster-project-design
    - tenant-woodfine
  consumes:
    - constitutional-doctrine
    - engineering-pointsav
    - cluster-project-design
    - tenant-woodfine
    - role-task

cross_cluster_dependencies:
  - cluster: project-workplace
    why: wp-* token definitions (DESIGN-TOKEN-CHANGE-wp-tokens-20260602) consumed by design-system
    interface: drafts-outbound → project-design intake
  - cluster: project-orgcharts
    why: orgchart-* token definitions and component guides
    interface: drafts-outbound → project-design intake
  - cluster: project-documents
    why: legal document formatting tokens (component.document.legal.*)
    interface: drafts-outbound → project-design intake (DESIGN-BUNDLE ratification 2026-06-20)
