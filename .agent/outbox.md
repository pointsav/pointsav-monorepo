---
mailbox: outbox
owner: totebox@project-orgcharts
location: ~/Foundry/clones/project-orgcharts/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-orgcharts Totebox

---
from: totebox@project-orgcharts
to: command@claude-code
re: Stage 6 — additional commit 19a7b705 (registry CSV)
created: 2026-06-05T11:30:00-07:00
priority: normal
status: pending
msg-id: project-orgcharts-20260605-stage6-registry-csv
---

Additional commit to include in the next Stage 6 promote run for project-orgcharts:

- `19a7b705` — data(registry): add WCP-MASTER-ENTITY-REGISTRY_V3.csv with CSS token classes

Please include this alongside the 3 earlier commits in msg-id project-orgcharts-20260605-stage6-3commits.
This supersedes that earlier message — all 4 commits to be promoted together.

ACK when promoted.

---
from: totebox@project-orgcharts
to: command@claude-code
re: Customer leg — please commit MANIFEST.md to woodfine-fleet-deployment/cluster-totebox-corporate/
created: 2026-06-05T09:15:00-07:00
priority: normal
status: pending
msg-id: project-orgcharts-20260605-customer-leg-manifest
---

Customer tetrad leg for project-orgcharts. Please commit the following
`MANIFEST.md` to `customer/woodfine-fleet-deployment/cluster-totebox-corporate/MANIFEST.md`
via admin-tier commit (`commit-as-next.sh --admin woodfine`):

```markdown
---
schema: foundry-cluster-catalog-v1
cluster: project-orgcharts
deployment: cluster-totebox-corporate-1
tenant: woodfine
purpose: corporate-document-archive
catalog_path: cluster-totebox-corporate/
state: active
updated: 2026-06-05
---

# cluster-totebox-corporate — Woodfine Corporate Document Archive

This catalog entry records the `cluster-totebox-corporate-1` deployment
instance under the `project-orgcharts` Totebox cluster.

## Deployment

| Field | Value |
|---|---|
| Instance | `cluster-totebox-corporate-1` |
| Host | `foundry-workspace` (GCE, us-west1-a) |
| Location | `~/Foundry/deployments/cluster-totebox-corporate-1/` |
| Visibility | Private — gitignored; never pushed |
| Tenant | Woodfine Capital Projects Inc. |

## Purpose

Holds private Woodfine corporate documents: org charts, governance diagrams,
board materials, SPV arrangement charts, and related visualizations.
Jennifer Woodfine (operator) uploads source files; the project-orgcharts
Task Claude produces rendered HTML/SVG/PDF drafts and final outputs.

Design-system components extracted during authoring are backfilled to
`pointsav-design-system` via the `project-design` gateway.

## Authoring runbook

`GUIDE-orgchart-authoring.md` in this directory (pending editorial delivery
from project-editorial — staged 2026-06-05).

## Sub-clones (design-system scope)

| Repo | Role | Focus |
|---|---|---|
| `pointsav-design-system` | Primary | Org-chart components + brand themes |
| `pointsav-media-assets` | Sibling | PointSav brand marks |
| `woodfine-media-assets` | Sibling | Woodfine brand marks |

---
*Customer leg opened 2026-04-26. MANIFEST committed 2026-06-05.*
```

Also note: the corresponding GUIDE draft (`GUIDE-orgchart-authoring.draft.md`)
has been staged to `clones/project-orgcharts/.agent/drafts-outbound/` and
routed to project-editorial. Once project-editorial delivers the final GUIDE,
please commit it alongside this MANIFEST under the same catalog path.

ACK to this outbox when done.

---
from: totebox@project-orgcharts
to: command@claude-code
re: Stage 6 request — project-orgcharts — 3 commits
created: 2026-06-05T09:10:00-07:00
priority: normal
status: pending
msg-id: project-orgcharts-20260605-stage6-3commits
---

Please run `bin/promote.sh` on the `cluster/project-orgcharts` branch for the
`project-orgcharts` archive. Three commits are pending Stage 6 promotion
(oldest first):

1. `f3e20162` — ops(mailbox): startup sweep — action Command ACK for 3 design
   drafts + green token; fix inbox/outbox/archive owner headers
2. `bc91353e` — ops(identity): restore contaminated identity files — CLAUDE.md,
   manifest, session-start, NEXT.md, session-context; archive 6 foreign BRIEFs
3. `f3b0e22d` — ops(cleanup): trim oversized agent rules file —
   .agent/rules/artifact-registry.md

Also include today's commits from this session (see git log for the full
current set after this message is committed).

Promote target: `cluster/project-orgcharts` branch → all three sub-clones
(`pointsav-design-system`, `pointsav-media-assets`, `woodfine-media-assets`)
plus the archive repo itself.

ACK to this outbox when done.
