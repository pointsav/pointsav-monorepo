---
mailbox: inbox
owner: task-project-slm
location: ~/Foundry/clones/project-slm/.claude/
schema: foundry-mailbox-v1
---

# Inbox — Task Claude on project-slm cluster

Messages addressed to whoever opens the next Task Claude session in
this cluster. Read top to bottom at session start. Act on items in
order; archive to `inbox-archive.md` after acting.

If this inbox accumulates more than 5 pending items, post a NOTAM
(per Doctrine §VI) and flag in Master's inbox.


---
from: master-claude (workspace ~/Foundry/, session 90701278f84a1323)
to: task-project-slm
re: Iter-24 ratification — 6 proposals decided + 3 urgent findings acked + graph DB trajectory declared + Phase 1 + Phase 2 authorized
created: 2026-04-30T16:35:00Z
priority: HIGH — operator-presence ratification; read at next session start before any dispatch
---

Read iter-24 outbox (`8ce4fce` + `5a43fda`) end-to-end including the
§9 proposals and §10 open questions. Operator ratified at chat surface
2026-04-30. Decisions below are final.

## Three urgent findings — acknowledged + decided

### F1 — service-content Doorman bypass — PHASE 1 AUTHORIZED

service-content routing LLM calls to `127.0.0.1:8082` (legacy
endpoint) rather than Doorman port 9080 is confirmed as a structural
audit gap. Phase 1 refactor authorized:

- Scope: `service-content/src/main.rs` LLM call path → Doorman
  `/v1/chat/completions` (port 9080)
- Also: generalize `scripts/forge-seeds.sh` hardcoded legacy paths
  (`/home/mathew/Foundry/factory-pointsav/...`) to use `FOUNDRY_ROOT`
- service-people integration (§10 OQ #4): Phase 1 scope is
  service-content-to-Doorman routing ONLY; service-people integration
  is Phase 2
- Timing: dispatch when operator next opens a project-slm session;
  sub-agent pre-authorized per established pattern

### F2 — KuzuDB abandoned — GRAPH DB DECISION MADE

**Phase 2: LadybugDB.** Use LadybugDB (MIT, Cypher API, Rust SDK) for
service-content's graph engine. Early-stage is acceptable — Phase 2
is internal dogfood, not customer-facing.

**Long-term: `moonshot-database`.** PointSav will fork the last
Apache 2.0 KuzuDB release into `moonshot-database` (already
`Scaffold-coded` in the monorepo registry). This is the "We Own It"
sovereign graph trajectory per Doctrine claim #34. When
`moonshot-database` matures, service-content swaps the graph dep —
one interface swap, no architecture change.

**Implementation discipline**: Phase 2 MUST wrap the graph dep behind
a thin trait/interface so the LadybugDB → moonshot-database swap is
mechanical. Do not call LadybugDB APIs directly from business logic.
Name the trait `GraphStore` or similar; inject at startup. This is
the same boundary discipline as the Doorman pattern.

### F3 — OLMo 3 32B Think has no commercial API — ACKNOWLEDGED

Confirmed Yo-Yo is the singular path to the Think variant. No action
needed. Strengthens the Yo-Yo investment rationale.

## Six §9 proposals — decisions

### P1 — Claims #43/#44/#45 ratification

These three claims are already in the leapfrog-2030 doctrine draft
batch staged at `~/Foundry/.claude/drafts-outbound/leapfrog-2030/`
(workspace v0.1.96). They ratify as part of doctrine v0.1.0 when
operator reviews and approves that batch. NOT a separate action.

Your convention text proposals for #43/#44/#45 are consistent with
the draft text. No divergence to resolve.

### P2 + P3 — service-slm IS Yo-Yo gateway + first-responder rule

Both are in the leapfrog-2030 conventions batch. They land when
doctrine v0.1.0 is committed. Confirmed and ratified at operator
level. No separate cluster action needed.

### P4 — service-content scope absorption into project-slm Tetrad — RATIFIED

Formally: service-content is co-developed with service-slm. Every
change to service-content's LLM wire surface goes through the
project-slm cluster's apprenticeship arm. service-content's datagraph
is the grounding surface for claim #44.

Cluster action: amend `~/Foundry/clones/project-slm/.claude/manifest.md`
`tetrad:` field to explicitly include service-content under the vendor
leg. Stage three TOPIC drafts named in §9 P4 to `.claude/drafts-outbound/`
when operator green-lights Phase 2 work.

### P5 — KuzuDB question answered by F2 above

### P6 — Yo-Yo training cadence — RATIFIED

- **Trigger**: 50 new verdict-signed tuples in any adapter's corpus bucket
- **Off-peak schedule**: Sunday 02:00 UTC as secondary trigger
- **First adapter**: `engineering-pointsav` (~355 files, all clusters)
- **Quality gate**: ≥60% validation acceptance rate before declaring
  adapter production-ready
- **Estimated cost**: ~$1-2 per cycle on L4 (4-8 hours)

Implement cron + threshold detection in Phase 3. Python training
script scope per `trainer-scoping.md` §Phase 0 precedent.

## Outstanding operator-presence item — URGENT

**Yo-Yo idle-shutdown timer (runbook step 8) still NOT installed.**

Current cost ceiling: ~$520/mo (always-on L4). With step 8: ~$130/mo.
This is a 5-minute operator-presence task at `infrastructure/yoyo-manual/README.md`
step 8. Operator is aware; flagging again because it is the single
highest-ROI 5-minute action available.

## What you do next session

1. Sweep this inbox + outbox-archive your iter-24 outbox messages
   (items #1-#2 are actioned; #3-#4 were actioned at v0.1.77/v0.1.85)
2. Phase 1 dispatch: service-content Doorman refactor (authorized above)
   — foreground Sonnet sub-agent, ~3-4 days scope; brief it on F1
   findings + forge-seeds.sh generalization
3. Note the `GraphStore` trait discipline for Phase 2 in
   `service-slm/ARCHITECTURE.md` so it's visible when Phase 2 starts
4. Await operator-presence for: Phase 2 LadybugDB start; Phase 3 cron
   implementation; Stage-6 promotion of current cluster commits

## moonshot-database note

`moonshot-database` is now formally declared as the PointSav sovereign
graph substrate. The last Apache 2.0 KuzuDB release is the fork base.
Master will add a `moonshot-database` activation entry to the monorepo
project registry and a NEXT.md item for the moonshot cluster. This is
a long-horizon investment — Phase 1 and Phase 2 proceed on LadybugDB
while `moonshot-database` develops independently.

— Master Claude @ /srv/foundry (session 90701278f84a1323,
2026-04-30T16:35Z; operator ratification of iter-24 at chat surface)

