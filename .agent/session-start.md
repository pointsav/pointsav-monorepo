---
schema: foundry-session-start-v1
archive: project-marketing
updated: 2026-05-24
---

# Session start — project-marketing

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** `app-mediakit-marketing` — Rust server delivering WordPress.org muscle-memory at the user-facing layer (Dashboard, Pages, Posts, Media, Themes, Plugins, Settings vocabulary millions already know) with a leapfrog-2030 internal architecture. v0.0.1 MVP shipped 2026-05-06; `cargo check` clean. Bootstrap deploy and certbot TLS are the immediate next operator actions.
- **Active branch:** `cluster/project-marketing`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** none

## Known gotchas

- v0.0.1 is shipped but not yet deployed. Bootstrap + certbot TLS are operator-gated (not Command-session scope).
- WordPress leapfrog framing is intentional — WordPress.org muscle-memory + leapfrog-2030 internals. Do not propose replacing WordPress vocabulary.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier).
- `home.woodfinegroup.com` nginx proxies to port 9102 (not 9100 as the comment suggests — comment is stale from before the port was finalized).
- robots.txt + sitemap.xml are served as nginx static files from the deployment content dirs, not by the binary.

## Last session handoff

*2026-05-24 — Three items completed:*
*1. binary-targets.yaml declared (app-mediakit-marketing, FSL, app-bundle, extension layer) — inbox actioned.*
*2. SEO head blocks applied to both home pages (meta description, canonical, OG, Twitter card, JSON-LD Organization schema). Live and verified on home.pointsav.com + home.woodfinegroup.com.*
*3. robots.txt + sitemap.xml added to both sites via nginx static-file location blocks. Live on HTTPS.*
*All outbox messages archived (project-design ACKs received). Outbox is clean. No open items.*
