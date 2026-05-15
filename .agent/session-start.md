---
schema: foundry-session-start-v1
archive: project-proofreader
updated: 2026-05-14
---

# Session start — project-proofreader

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Operational write-assistant for SMB-shaped editorial work. Vendor-tier deployment instance at **https://proofreader.pointsav.com**. Owns `service-proofreader/` Rust crate in `pointsav-monorepo`. Phase A (dark mode) + Phase D-lite (chat UI) shipped. Phase B prompt fix in `generative.rs` is the active next milestone.
- **Active branch:** `cluster/project-proofreader`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** none (check `.agent/plans/` for any new files)

## Known gotchas

- **Do not swap or upgrade the SLM model** until the operator explicitly lifts the constraint.
- **Dark mode stays basic** — Phase A shipped (commit 10f062a); do not extend dark mode until the chat UI (Phase D-lite) is fully stable.
- `PROOFREADER_AUTH_REQUIRED=false` — the service is public on port 9097 with HTTPS live. Rate-limited at 20r/min.
- Phase B prompt fix is in `generative.rs` — this is the primary engineering target for the next session.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier).

## Last session handoff

*Phase A (dark mode) + Phase D-lite (chat UI) shipped. Service public at proofreader.pointsav.com. Phase B prompt fix in generative.rs is next.*
