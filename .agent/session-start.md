---
schema: foundry-session-start-v1
archive: project-command
updated: 2026-05-14
---

# Session start — project-command

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** `app-orchestration-command` — the user-facing hub in the hub-and-spoke orchestration topology (Ring 2). Aggregates outputs from Ring 3 services (service-slm, service-content, service-fs) and presents them to customer entities with permission boundary enforcement. Phase 3 Rust: v0.0.1 is the immediate next milestone.
- **Active branch:** `cluster/project-command`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** none (check `.agent/plans/` for any new files)

## Known gotchas

- This archive contains the `pointsav-monorepo` sub-clone. The sub-clone has its own `.git/` — one session per `.git/index` rule applies separately to it.
- Phase 3 Rust (v0.0.1) is the active engineering goal. Read `.agent/manifest.md` for full topology spec.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier).

## Last session handoff

*No prior handoff recorded.*
