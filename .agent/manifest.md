---
cluster: project-intelligence
cluster_name: project-intelligence
cluster_branch: main
created: 2026-04-23
state: active
module_id: intelligence
slm_endpoint: http://localhost:9080

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: >
        service-slm (AI Doorman + Yo-Yo orchestrator),
        service-content (Taxonomy Ledger / LadybugDB DataGraph),
        service-extraction (Deterministic Parser, ADR-07-safe),
        service-search (Tantivy index, DARP-compliant),
        service-disclosure (planned)
      status: active — service-slm live; other services in development
  customer:
    - repo: woodfine-fleet-deployment
      path: woodfine-fleet-deployment/
      focus: Doorman deployment GUIDEs
      status: active (GUIDEs staged; woodfine-fleet-deployment sub-clone present)
  deployment:
    - status: active
      note: >
        service-slm deployed on foundry-workspace VM (port 9080 / Doorman).
        Yo-Yo tier-b-1 (europe-west4-a) available on demand.
  wiki:
    - status: leg-pending
      note: >
        TOPIC drafts about SLM routing, DataGraph architecture, and apprenticeship
        substrate to be authored and routed via project-editorial.

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
  - repo: woodfine-fleet-deployment
    role: customer-fleet
    path: woodfine-fleet-deployment/
    upstream: customer/woodfine-fleet-deployment
