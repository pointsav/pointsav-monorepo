---
schema: foundry-session-start-v1
archive: project-bim
updated: 2026-07-15
---

# Session start — project-bim

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Building Information Modeling (BIM) design system + City Code as Composable Geometry pattern (Doctrine claim #41). Owns `woodfine-bim-library` repo (renamed from `woodfine-design-bim` 2026-05-17, see `.agent/rules/cleanup-log.md` 2026-05-16 entry) and BIM-specific tokens/components that are NEVER routed to `pointsav-design-system`.
- **Active branch:** `cluster/project-bim`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** none (check `.agent/plans/` for any new files)

## Known gotchas

- **BIM artifacts never go to `pointsav-design-system`.** BIM tokens/components route to `woodfine-bim-library` only. This is a hard architectural boundary.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier).
- See `.agent/manifest.md` and `manifest-notes.md` for full strategic context (split 2026-05-09 to keep manifest under 30 KB cap).

## Last session handoff

**Corrected 2026-07-15:** this section previously described the 2026-05-17
session and had gone two months stale relative to actual activity (last
real entry in `.agent/rules/cleanup-log.md` is 2026-07-13; `NEXT.md`'s Hot
section is current as of the same date). Rather than re-freehand a summary
here that will drift again, read `NEXT.md`'s "Hot — pick up here next
session" section directly — it is actively maintained and is the
authoritative pointer to where to resume. The 2026-05-17 print/PDF-pipeline note's underlying work is still
findable in `.agent/plans/`, `.agent/drafts-outbound/`, and `.agent/outbox.md`
if historical detail is needed.
