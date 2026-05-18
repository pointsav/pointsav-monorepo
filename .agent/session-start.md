---
schema: foundry-session-start-v1
archive: project-marketing
updated: 2026-05-14
---

# Session start — project-marketing

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** `app-mediakit-marketing` — Rust server delivering WordPress.org muscle-memory at the user-facing layer (Dashboard, Pages, Posts, Media, Themes, Plugins, Settings vocabulary millions already know) with a leapfrog-2030 internal architecture. v0.0.1 MVP shipped 2026-05-06; `cargo check` clean. Bootstrap deploy and certbot TLS are the immediate next operator actions.
- **Active branch:** `cluster/project-marketing`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** none (check `.agent/plans/` for any new files)

## Known gotchas

- v0.0.1 is shipped but not yet deployed. Bootstrap + certbot TLS are operator-gated (not Command-session scope).
- WordPress leapfrog framing is intentional — WordPress.org muscle-memory + leapfrog-2030 internals. Do not propose replacing WordPress vocabulary.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier).

## Last session handoff

*2026-05-18 — PointSav site content editing session. Changes to software.pointsav.com (software + licensing pages) and home.pointsav.com (home, disclaimer, contact). All committed to cluster/project-software; Stage 6 pending. Disk-based HTML serving implemented — no rebuild needed for future HTML edits. See memory/project_pointsav_site_files.md for all file locations.*
