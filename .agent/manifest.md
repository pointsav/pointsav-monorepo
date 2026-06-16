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
        yoyo-batch L4 GPU (Tier B) — TERMINATED; restart pending operator approval
        (us-central1-b; image slm-yoyo-20260512-111846; ML libs install required).
  wiki:
    - status: leg-pending
      note: >
        TOPIC/GUIDE drafts in .agent/drafts-outbound/.
        T1 (PPN VM Architecture) and T2 (Tenant VM Isolation) STAGED.
        Pipeline through project-editorial to media-knowledge-documentation.

clones: []
