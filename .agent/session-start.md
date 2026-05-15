---
schema: foundry-session-start-v1
archive: project-source
updated: 2026-05-14
---

# Session start — project-source

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** PointSav canonical-tier development archive. Replaces Root sessions in `vendor/`. Owns `pointsav-monorepo` and `pointsav-design-system` sub-clones (to be provisioned on first Task session). Work here promotes to canonical via Stage 6.
- **Active branch:** `cluster/project-source`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** none (check `.agent/plans/` for any new files)

## Known gotchas

- **Sub-repos are NOT yet cloned.** `pointsav-monorepo/` and `pointsav-design-system/` sub-clones must be provisioned (`git clone`) before any work on them. They are gitignored in this cluster.
- This is a **newly provisioned** archive (2026-05-14). First-use setup is required.
- Commits here are staging-tier — they promote to canonical `pointsav` org via `bin/promote.sh` (Stage 6), not directly.
- Never open a session directly in `vendor/pointsav-monorepo/` or `vendor/pointsav-design-system/` — use this archive instead.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier).

## Last session handoff

*Archive provisioned 2026-05-14. Sub-repo cloning pending first Task session use.*
