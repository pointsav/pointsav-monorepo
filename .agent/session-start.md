---
schema: foundry-session-start-v1
archive: project-workplace
updated: 2026-06-19
---

# Session start — project-workplace

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Workbench suite — `app-workplace-*` Tauri desktop apps and moonshot crates.
  Active: memo, presentation, workbench, proforma, aibridge. Moonshot: docengine, parser, crdt, editor, bim-engine.
- **Active branch:** `main` (monorepo sub-clone tracks `main`; `cluster/project-workplace` is a stale ancestor — ignore)
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **Stage 6 pending:** workbench monorepo commits f00e676a ec305edc 8412516b (+ 75aa556c if ahead)

## Known gotchas

- This archive contains the `pointsav-monorepo` sub-clone. The sub-clone has its own `.git/` — one session per `.git/index` rule applies separately to it.
- `.agent/briefs/` contains ~20 foreign BRIEFs from M-17 contamination — do not act on them; route to correct archives.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier).

## Last session handoff

Session 11 (2026-06-16): workbench moonshot crates (aibridge Phase 3 stub), BRIEF audit done,
contamination reported via outbox, prototype service manual-started. Stage 6 pending for Command.
