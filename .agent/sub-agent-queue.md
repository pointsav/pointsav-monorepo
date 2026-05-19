---
queue: cluster-sub-agent-queue
owner: task-project-knowledge
location: ~/Foundry/clones/project-knowledge/.claude/
schema: foundry-sub-agent-queue-v1
ratified: 2026-04-28T04:00:00Z by master-claude
operator_authorization: 2026-04-28 "take care of all open issues" broad framing
---

# Cluster sub-agent queue — project-knowledge

Cluster-scope sub-agent briefs ratified by Master for dispatch. Per
v0.1.30 §1A.4 layer-scope rule, this queue holds briefs that touch
only this cluster's territory (drafts-outbound/, prose-edit corpus
path, app-mediakit-knowledge/, pointsav-monorepo cleanup-log within
this cluster's sub-clone). Master-scope briefs (workspace docs, IaC,
conventions, cross-cluster propagation) live in
`~/Foundry/.claude/sub-agent-queue.md`.

Brief files at `~/Foundry/clones/project-knowledge/.claude/proposed-briefs/`
are the authoritative content; this queue links by reference.

Parent review discipline (§1A rule 6): the parent Opus session reviews
each sub-agent output before commit-or-queue-next. Never delegate the
commit decision.

---

## Phase 4 — app-mediakit-knowledge

BP1 cleared 2026-04-28 (operator via Master workspace v0.1.54). Brief
01-phase4-decomposition.md executed and produced the 8 sub-step briefs
below. Each brief is a separate file in `proposed-briefs/`. Brief 4.1 and
4.2 require PK.3 (libgit2-dev system-lib install). Brief 4.6 is
outbox-first; its first dispatch writes an outbox to project-slm and
implementation gates on project-slm's reply.

| # | Brief file | Gated by | Status |
|---|---|---|---|
| 4.1 | `proposed-briefs/phase4-41-git2-wiring.md` | PK.3 (libgit2-dev install alongside libssl-dev) | READY-after-PK.3 |
| 4.2 | `proposed-briefs/phase4-42-history-blame.md` | Step 4.1 complete; PK.3 for link-time libgit2 | READY-after-PK.3 + 4.1 |
| 4.3 | `proposed-briefs/phase4-43-diff.md` | Step 4.2 complete | READY-after-4.2 |
| 4.4 | `proposed-briefs/phase4-44-redb-linkgraph.md` | Step 4.1 complete | READY-after-4.1 |
| 4.5 | `proposed-briefs/phase4-45-blake3-hashing.md` | Steps 4.1 + 4.4 complete | READY-after-4.4 |
| 4.6 | `proposed-briefs/phase4-46-mcp-server.md` | Step 4.1 complete; DISPATCH 1 = outbox-only; DISPATCH 2 gates on project-slm reply | OUTBOX-FIRST — two-dispatch |
| 4.7 | `proposed-briefs/phase4-47-git-remote.md` | Step 4.1 complete | READY-after-4.1 |
| 4.8 | `proposed-briefs/phase4-48-openapi.md` | All steps 4.1–4.7 complete | READY-after-all |

---

## Wiki-leg expansion — project-knowledge cluster

| # | Brief file | Tier | Sequencing | Status |
|---|---|---|---|---|
| 2 | `proposed-briefs/02-collab-relay-expansion.md` | Sonnet | sequential (writes to drafts-outbound/) | DONE |
| 3 | `proposed-briefs/03-source-of-truth-inversion.md` | Sonnet | sequential | DONE |
| 4 | `proposed-briefs/04-wikipedia-leapfrog-design.md` | Sonnet | sequential | DONE |

---

## Read-only audits — project-knowledge cluster (parallelisable batch)

Per §1A rule 2, read-only sub-agents may parallelise.

| # | Brief file | Tier | Status |
|---|---|---|---|
| 5 | `proposed-briefs/05-jsonl-corpus-audit.md` | Sonnet | DONE |
| 6 | `proposed-briefs/06-frontmatter-validation.md` | Sonnet | DONE |
| 7 | `proposed-briefs/07-static-asset-audit.md` | Haiku | DONE |
| 8 | `proposed-briefs/08-cleanup-log-triage.md` | Sonnet | DONE |

---

## Completed

| # | Brief | Outcome | Commit / fix |
|---|---|---|---|
| 5 | JSONL corpus integrity audit | All 6 clean against foundry-draft-v1; corpus ready for Stage-1 DPO | (no action — clean) |
| 6 | Frontmatter validation | 5 of 6 compliant; .es draft missing `references:` | Fix applied to `topic-collab-via-passthrough-relay.es.draft.md` (mirror sibling, 9 entries) |
| 7 | static/ asset audit | 1 unused: `wikilink-redlink` + `--link-redlink` var | `c4a5677` — both removed from style.css per CLAUDE.md §6 |
| 8 | cleanup-log triage | A:3 / B:3 / C:5 / D:10 | `c4a5677` — 3 Category A fixes applied (registry state + 2 cleanup-log reclassifications); B/C carried |
| 2 | collab-relay skeleton expansion | Both files (English + Spanish) expanded to bulk-draft; 7 placeholders → substantive prose; frontmatter `draft_shape: bulk-draft` + `authored_with: sonnet-4-6` | (drafts-outbound/ — operational state, no commit; project-language sweeps via bin/draft-sweep.sh) |
| 3 | source-of-truth-inversion TOPIC | New 149-line bulk draft authored at `topic-source-of-truth-inversion.draft.md`; 6 sections; pattern + 4 application instances + BCSC posture + claim #34 connection; thin source on §3-§5 flagged with planned qualifiers | (drafts-outbound/ — operational state; JSONL event emitted to apprenticeship corpus) |
| 4 | wikipedia-leapfrog-design TOPIC | New 478-line bulk draft authored at `topic-wikipedia-leapfrog-design.draft.md`; 5 sections; muscle-memory contract + 5 additions + visual divergence + two-audience contract + forward reference; some sections slightly over target lengths (project-language pares) | (drafts-outbound/ — operational state; JSONL event emitted to apprenticeship corpus) |

---

## Dispatch log

| Timestamp | Brief # | Sub-agent | Tier | Outcome |
|---|---|---|---|---|
| 2026-04-28T04:10:00Z | 5 | a3bbf3265932f3bc2 | sonnet | All 6 JSONL clean; report 60 lines |
| 2026-04-28T04:10:00Z | 6 | a714f9cbe5750e806 | sonnet | 5/6 PASS, 1 FAIL (.es missing references); fix applied |
| 2026-04-28T04:10:00Z | 7 | a62be694a339e1029 | haiku | 1 unused class + var; both removed in c4a5677 |
| 2026-04-28T04:10:00Z | 8 | aec72226002fdd9ee | sonnet | Triage 4 categories; A applied in c4a5677; B/C carried in cleanup-log session entry |
| 2026-04-28T04:25:00Z | 2 | afb428295fc17051f | sonnet | Both English + Spanish skeletons expanded to bulk-draft; all 7 placeholders replaced; frontmatter updated |
| 2026-04-28T04:30:00Z | 3 | a09c5835d16eaf6e1 | sonnet | New TOPIC drafted (149 lines); thin source on app-workplace projects flagged with planned qualifiers |
| 2026-04-28T04:35:00Z | 4 | a04ac301a03633778 | sonnet | New TOPIC drafted (478 lines); two-audience contract preserved; some sections slightly over target |
