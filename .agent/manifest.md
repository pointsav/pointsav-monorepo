---
schema: foundry-cluster-manifest-v1
cluster_name: project-intelligence
cluster_branch: cluster/project-intelligence
renamed_from: project-slm
renamed: 2026-05-05
created: 2026-04-23
backfilled: 2026-04-26 (manifest schema), 2026-04-26 (triad per Doctrine v0.0.4), 2026-04-28 (tetrad per Doctrine v0.0.10 claim #37)
state: active
slm_endpoint: http://localhost:8011
module_id: intelligence

tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: ./
      upstream: vendor/pointsav-monorepo
      focus: service-slm/ (Doorman + slm-core + slm-doorman + slm-doorman-server), Apprenticeship Substrate routing endpoints (claim #32)
    - repo: pointsav-monorepo
      path: ./service-content/
      upstream: vendor/pointsav-monorepo
      focus: service-content/ (semantic entity extractor + CORPUS ledger watcher + LadybugDB graph — Ring 2)
      note: formally absorbed into project-slm scope per Master P4 ratification 2026-04-30; every LLM wire change routes through Doorman and the apprenticeship arm; datagraph is the grounding surface for Doctrine claim #44
  customer:
    - fleet_deployment_repo: vendor/pointsav-fleet-deployment
      catalog_subfolder: vault-privategit-source/
      tenant: pointsav
      purpose: documentation-of-Doorman-installation-on-the-workspace-VM
      status: leg-pending — Task to draft guide-doorman-deployment.md (PS.8 in v0.1.42 plan; Q1-Q4 answered)
  deployment:
    - path: /srv/foundry  # vault-privategit-source-1 (the workspace itself)
      tenant: pointsav
      shape: long-running-service
      shared_with: [project-data]   # workspace VM hosts both Ring 1 (project-data) and Ring 2+3 (project-slm) services
      runtime_artifacts:
        - /usr/local/bin/llama-server (v0.0.11)
        - /etc/systemd/system/local-slm.service (v0.0.11)
        - /var/lib/local-slm/weights/Olmo-3-1125-7B-Think-Q4_K_M.gguf
        - (planned) /usr/local/bin/slm-doorman-server + local-doorman.service
      status: tier-A-live; Doorman deployment pending B7 redeploy with SLM_APPRENTICESHIP_ENABLED=true
  wiki:
    - repo: vendor/content-wiki-documentation
      drafts_via: clones/project-intelligence/.agent/drafts-outbound/
      gateway: project-editorial Task
      planned_topics:
        - topic-doorman-protocol.md           # the Doorman as security boundary + three-tier compute routing
        - topic-apprenticeship-substrate.md    # service-slm as first responder + signed-verdict corpus loop (claim #32)
        - topic-elastic-compute-lora-training-pipeline.md  # Elastic Compute nightly LoRA training pipeline
        - topic-service-slm-graph-store-migration.md       # service-slm graph store migration
      status: leg-pending — TOPIC skeletons staged in drafts-outbound/; substance lands as service-slm milestones progress

clones:
  - repo: pointsav-monorepo
    role: primary
    path: ./
    upstream: vendor/pointsav-monorepo
  - repo: woodfine-fleet-deployment
    role: secondary
    path: ./woodfine-fleet-deployment/
    upstream: customer/woodfine-fleet-deployment
    focus: vault-privategit-source/ (Doorman deployment GUIDEs + Elastic Compute runbooks)
    added: 2026-05-05
trajectory_capture: pending

software_footprint:
  target_os: os-totebox
  monorepo: pointsav-monorepo
  branch: cluster/project-intelligence
  owns:
    - service-slm/        # AI Doorman + Elastic Compute orchestrator
    - service-content/    # Taxonomy Ledger / LadybugDB knowledge graph
    - service-disclosure/ # Disclosure substrate

adapter_routing:
  trains:
    - cluster-project-intelligence   # own cluster adapter (Doorman + Elastic Compute client + Tier C)
    - engineering-pointsav           # Vendor engineering corpus (Ring 2+3 services)
    - apprenticeship-pointsav        # apprenticeship corpus (claim #32; AS-3/AS-4 produce its tuples)
  consumes:
    - constitutional-doctrine    # always
    - engineering-pointsav       # always — Vendor knowledge
    - cluster-project-intelligence  # own cluster context
    - role-task                  # current role
    - apprenticeship-pointsav    # apprenticeship adapter — composed alongside engineering at request time per claim #22
---

# Cluster manifest — project-intelligence

Single-clone cluster (N=1). Renamed from project-slm 2026-05-05.

## Scope

Ring 2 + Ring 3 of the three-ring architecture
(`~/Foundry/conventions/three-ring-architecture.md`):

- `service-slm` — AI Doorman + Elastic Compute orchestrator (Ring 3)
- `service-content` — Taxonomy Ledger / LadybugDB graph (Ring 2)

## Branch

`cluster/project-intelligence`

## Remotes

- `origin` — canonical via admin SSH alias
- `origin-staging-j` — Jennifer's staging-tier mirror
- `origin-staging-p` — Peter's staging-tier mirror

Push policy: staging-tier only. Stage 6 for canonical promotion.

---

*Restored by Command Session 2026-05-22 — manifest was contaminated by Stage-6 rebase that pulled project-knowledge .agent/ content into project-intelligence working tree. Restored from git history (bd2cb2c8^:.agent/manifest.md) with updated planned_topics to reflect current draft filenames.*
