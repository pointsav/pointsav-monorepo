---
schema: foundry-cluster-manifest-v1
cluster: project-intelligence
cluster_name: project-intelligence
cluster_branch: main
created: 2026-05-27
state: active
slm_endpoint: http://localhost:9080
module_id: jennifer

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: ./
      upstream: vendor/pointsav-monorepo
      focus: >
        service-slm/crates/slm-doorman-server/ (Doorman + circuit breaker + LoRA training),
        service-content/ (DataGraph entity extraction, LadybugDB),
        service-extraction/ (extraction pipeline)
      status: active
  customer:
    - status: leg-pending
      note: >
        No woodfine-fleet-deployment catalog entries committed yet.
        local-doorman.service is live but not cataloged.
  deployment:
    - status: active
      note: >
        local-doorman.service (:9080) on vault-privategit-source-1.
        local-slm.service (OLMo 7B Tier A).
        yoyo-batch L4 GPU (Tier B) — TERMINATED; restart requires operator approval.
  wiki:
    - status: leg-pending
      note: >
        TOPIC/GUIDE drafts in .agent/drafts-outbound/.
        Pipeline through project-editorial to media-knowledge-documentation.

clones: []
---

# project-intelligence — Cluster Manifest

SLM inference infrastructure cluster. Builds and operates local-doorman.service
(Tier A/B/C routing + circuit breaker), local-slm.service (OLMo 7B Tier A
CPU inference), DataGraph entity enrichment (LadybugDB via service-content),
and the LoRA training pipeline. Primary crates: slm-doorman-server,
service-content, service-extraction.
