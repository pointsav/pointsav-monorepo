---
schema: foundry-cluster-manifest-v1
cluster: project-editorial
cluster_name: project-editorial
cluster_branch: cluster/project-editorial
created: 2026-05-16
state: active
slm_endpoint: http://localhost:8011
module_id: editorial

tetrad:
  vendor:
    - status: leg-pending
      note: >
        No direct monorepo crate. Editorial pipeline produces content artifacts
        (TOPIC, GUIDE, JOURNAL) consumed downstream by media-knowledge repos and
        woodfine-fleet-deployment. The vendor leg tracks pipeline conventions
        committed at ~/Foundry/conventions/.
  customer:
    - repo: media-knowledge-documentation
      path: media-knowledge-documentation/
      upstream: customer/media-knowledge-documentation
      focus: >
        Architecture, services, systems, governance TOPICs for
        documentation.pointsav.com — all platform engineering wiki content.
      status: active
    - repo: media-knowledge-projects
      path: media-knowledge-projects/
      upstream: customer/media-knowledge-projects
      focus: >
        Location intelligence, GIS archetypes, research project TOPICs for
        projects.woodfinegroup.com.
      status: active
    - repo: media-knowledge-corporate
      path: media-knowledge-corporate/
      upstream: customer/media-knowledge-corporate
      focus: >
        Corporate identity, governance, disclosure TOPICs.
      status: active
    - repo: woodfine-fleet-deployment
      path: woodfine-fleet-deployment/
      upstream: customer/woodfine-fleet-deployment
      focus: >
        Operational GUIDEs routed through Command Session for placement.
      status: active
  deployment:
    - status: leg-pending
      note: >
        No deployment instance. project-editorial is a Totebox editorial gateway,
        not a deployed service. Content it produces deploys via app-mediakit-knowledge.
  wiki:
    - status: active
      note: >
        The media-knowledge-* repos ARE the wiki output of this archive.
        Committed content renders at documentation.pointsav.com and
        projects.woodfinegroup.com via app-mediakit-knowledge.

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
