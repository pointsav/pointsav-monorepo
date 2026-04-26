---
schema: foundry-cluster-manifest-v1
cluster_name: project-slm
cluster_branch: cluster/project-slm
created: 2026-04-23
backfilled: 2026-04-26
state: active
clones:
  - repo: pointsav-monorepo
    role: primary
    path: ./
    upstream: vendor/pointsav-monorepo
deployment_instance: null
trajectory_capture: pending
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

- Inbox: `~/Foundry/clones/project-slm/.claude/inbox.md`
- Outbox: `~/Foundry/clones/project-slm/.claude/outbox.md`
- Trajectory log: `~/Foundry/clones/project-slm/.claude/trajectory-log.md`
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
