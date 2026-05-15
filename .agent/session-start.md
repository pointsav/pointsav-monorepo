---
schema: foundry-session-start-v1
archive: project-bim
updated: 2026-05-14
---

# Session start — project-bim

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Building Information Modeling (BIM) design system + City Code as Composable Geometry pattern (Doctrine claim #41). Owns `woodfine-design-bim` repo and BIM-specific tokens/components that are NEVER routed to `pointsav-design-system`.
- **Active branch:** `cluster/project-bim`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** none (check `.agent/plans/` for any new files)

## Known gotchas

- **BIM artifacts never go to `pointsav-design-system`.** BIM tokens/components route to `woodfine-design-bim` only. This is a hard architectural boundary.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier).
- See `.agent/manifest.md` and `manifest-notes.md` for full strategic context (split 2026-05-09 to keep manifest under 30 KB cap).

## Last session handoff

*No prior handoff recorded.*
