---
schema: foundry-session-start-v1
archive: project-infrastructure
updated: 2026-06-08
---

# Session start — project-infrastructure

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** PPN (PointSav Private Network) cartridges and network OS work.
  Owns vendor-side crates: `app-infrastructure-onprem`, `app-infrastructure-leased`,
  `app-infrastructure-cloud`, `app-network-admin`, `os-infrastructure`, `os-network-admin`.
  Also maintains the pointsav-monorepo project registry and repo-layout rules.
- **Active branch:** `cluster/project-infrastructure`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)

## Critical state

- **WireGuard Part A gates the deployment + customer tetrad legs.** Master key authority
  resides physically on Laptop A — never delegated to cloud. Until Part A is complete,
  `fleet-infrastructure-*` and `route-network-admin-1` deployment instances are `leg-pending`.

- **JOURNAL J2 (ASPLOS — "Composing Trustworthy Systems from Verified Primitives"):**
  Bench #9 (`SignedCheckpoint::verify_inclusion_proof`, 1024-leaf tree) is CANCELLED
  (2026-06-04). Decision required in §5 Evaluation: either submit with ±11% CI noted
  as a known limitation, or remove the Bench #9 data point. Notify project-knowledge
  via outbox once §5 decision is made. Brief: `.agent/briefs/BRIEF-substrate-phd-thesis-2026-05-27.md`.

- **PROSE-RESEARCH PPN v0.2 pending word fix:**
  One editorial finding: line 173 `"robust"` → `"reliable"` (banned vocabulary).
  File: `.agent/drafts-outbound/PROSE-RESEARCH-ppn-architecture-phd-thesis.draft.md`.
  Fix this and re-dispatch to project-editorial.

- **Wiki leg active:** 8 topics + 4 guides staged for pickup by project-editorial.
  Staged in `.agent/drafts-outbound/` — see manifest `wiki.planned_topics.staged_for_pickup`.

## Key ports / services in scope

| Service | Port | Notes |
|---|---|---|
| service-ppn-pairing | :9205 | Node pairing ceremony; binary on GCP |

## Known gotchas

- One session per `.git/` index — do not open concurrent sessions in this archive.
- Commit via `~/Foundry/bin/commit-as-next.sh` only. Direct `git commit` is blocked.
- Stage 6 promotion is Command Session scope only (`~/Foundry/bin/promote.sh`).
