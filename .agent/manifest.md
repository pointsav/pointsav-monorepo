---
schema: foundry-cluster-manifest-v1
cluster_name: project-slm
cluster_branch: cluster/project-slm
created: 2026-04-23
backfilled: 2026-04-26 (manifest schema), 2026-04-26 (triad per Doctrine v0.0.4), 2026-04-28 (tetrad per Doctrine v0.0.10 claim #37)
state: active

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
      drafts_via: clones/project-slm/.agent/drafts-outbound/
      gateway: project-language Task
      planned_topics:
        - topic-doorman-protocol.md           # the Doorman as security boundary + three-tier compute routing
        - topic-apprenticeship-substrate.md    # service-slm as first responder + signed-verdict corpus loop (claim #32)
        - topic-zero-container-inference.md    # native-binary + GCE start/stop + idle-shutdown for SMB GPU economics
      status: leg-pending — three TOPIC skeletons staged 2026-04-28 in drafts-outbound/; substance lands as service-slm milestones progress (PS.3 AS-2 implementation, PS.4 A-1 endpoints, Yo-Yo MIN deploy)

clones:
  - repo: pointsav-monorepo
    role: primary
    path: ./
    upstream: vendor/pointsav-monorepo
trajectory_capture: pending

adapter_routing:
  trains:
    - cluster-project-slm        # own cluster adapter (Doorman + Yo-Yo client + Tier C)
    - engineering-pointsav       # Vendor engineering corpus (Ring 2+3 services)
    - apprenticeship-pointsav    # apprenticeship corpus (claim #32; AS-3/AS-4 produce its tuples)
  consumes:
    - constitutional-doctrine    # always
    - engineering-pointsav       # always — Vendor knowledge
    - cluster-project-slm        # own cluster context
    - role-task                  # current role
    - apprenticeship-pointsav    # apprenticeship adapter — composed alongside engineering at request time per claim #22
    # NOTE: no tenant-* adapter — this cluster is Vendor engineering only;
    # service-slm is multi-tenant via moduleId at runtime, not at training time
---

# Cluster manifest — project-slm

Single-clone cluster (N=1). Backfilled 2026-04-26 per Doctrine
v0.0.2 §IV.c (this cluster predates the manifest schema; the
existing clone shape is the N=1 case of the formalised schema —
no behavioural change).

## Scope

Ring 2 + Ring 3 of the three-ring architecture
(`~/Foundry/conventions/three-ring-architecture.md`):

- `service-slm` — AI Doorman + Yo-Yo orchestrator (Ring 3)
  - Status: **Active** since 2026-04-23
  - First-live cluster occupation; per-project CLAUDE.md +
    NEXT.md + ARCHITECTURE.md + DEVELOPMENT.md present
  - B1 Doorman scaffold landed 2026-04-25 in commit `78031c4`
  - B5 verification ready as of 2026-04-26 (B3 Tier A delivered
    by Master in v0.0.11 commit `68e7c16`)
- `service-content` — Taxonomy Ledger / LadybugDB graph (Ring 2;
  multi-tenant via moduleId)
  - Status: Scaffold-coded; activation when service-slm lands
- `service-extraction` — Deterministic Parser (Ring 2; ADR-07
  zero AI)
  - Status: Active per registry; CLAUDE.md present but stale
    (drift item tracked in NEXT.md)
- `service-search` — Tantivy search index (Ring 2; DARP-compliant)
  - Status: Reserved-folder

## Branch

`cluster/project-slm` (renamed from `cluster/service-slm` 2026-04-25
when the cluster scope expanded from one service to four).

## Remotes (within the clone)

- `origin` — canonical via admin SSH alias
  (`github.com-pointsav-administrator`)
- `origin-staging-j` — Jennifer's staging-tier mirror
  (`github.com-jwoodfine:jwoodfine/pointsav-monorepo.git`)
- `origin-staging-p` — Peter's staging-tier mirror
  (`github.com-pwoodfine:pwoodfine/pointsav-monorepo.git`)

Push policy: staging-tier only (`origin-staging-j` and
`origin-staging-p`). Never push to `origin` (canonical) — Stage 6
promotion is the canonical-tier path. Per Doctrine §V Action Matrix
and v0.0.10 auto-mode safety brief.

## Trajectory capture

Pending. `bin/capture-edit.sh` and `bin/capture-trajectory.sh` will
be installed by Master in a v0.1.x increment per
`conventions/trajectory-substrate.md` L1–L2. Until then, commits
are not yet flowing to corpus; behavioural changes for Task: none.

## Cross-cluster coordination

- `project-data` cluster runs Ring 1 (`service-fs`,
  `service-people`, `service-email`, `service-input`).
- `service-content` (this cluster) eventually consumes
  `service-fs` schemas via MCP. Coordinate via mailbox
  (`outbox.md` to Master, who relays).

## Mailbox

- Inbox: `~/Foundry/clones/project-slm/.agent/inbox.md`
- Outbox: `~/Foundry/clones/project-slm/.agent/outbox.md`
- Trajectory log: `~/Foundry/clones/project-slm/.agent/trajectory-log.md`
  (created on first capture — see L1 wiring)

## State as of backfill (2026-04-26)

| Item | State |
|---|---|
| B1 Doorman scaffold | Done — `78031c4` |
| B3 Tier A backend | Done by Master — `68e7c16` (v0.0.11) |
| B5 verification | Ready to execute — see inbox v0.0.2 message |
| B2 Yo-Yo client | Pending B5 |
| B4 Tier C client | Pending B5 |
| B6 lifecycle controller | Deferred per A3 viability spike outcome |

---

*Backfilled 2026-04-26 in workspace v0.1.0 / Doctrine v0.0.2.*
