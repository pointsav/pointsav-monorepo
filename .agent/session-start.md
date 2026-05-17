---
schema: foundry-session-start-v1
archive: project-proofreader
updated: 2026-05-17
---

# Session start — project-proofreader

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Leapfrog 2030 TUI write-assistant over SSH. Users type `ssh proof@host`
  and receive a keyboard-native proofreader + content drafter in terminal. Owns
  `app-console-content/` (new TUI binary) + `os-console/` (chassis, future) +
  `service-proofreader/` (HTTP backend, unchanged) in `pointsav-monorepo`.
- **Active branch:** `cluster/project-proofreader`
- **Strategic pivot:** 2026-05-16 — web UI → TUI over SSH. Plan at `.agent/plans/tui-pivot-2030.md`.
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **Active plan:** `.agent/plans/tui-pivot-2030.md` — read this before any engineering work

## Critical state

- **Phase 0 COMPLETE** — `app-console-content` crate committed; `cargo build` green; SSH TUI gate passed (ratatui frame confirmed over port 2222 via `ssh -p 2222 -i ~/.ssh/google_compute_engine mathew@localhost`). Next: Phase 1 (auth + session management).
- **SSH key note**: `mathew` user has no standard `id_ed25519`; use `-i ~/.ssh/google_compute_engine` for localhost testing until Phase 1 adds `proofctl user add`.
- **russh 0.60 API**: native async fn in impl (no async_trait); `russh::keys::PrivateKey::random(&mut rand::rng(), Algorithm::Ed25519)`; `session.channel_success(channel)?` in pty_request + shell_request; TerminalHandle uses sink Vec<u8> + flush sends. See memory for full reference.
- **No `os-console/` exists yet** — Phase 7 milestone; standalone binary first.
- **Web UI is to be taken down** — `local-proofreader-console.service` (9091) and
  `local-proofreader-public.service` (9097) + nginx vhost + cert. Teardown checklist
  in the plan document. Requires Command Session (sudo).

## Known gotchas

- **Do not swap or upgrade the SLM model** until the operator explicitly lifts the constraint.
- `service-proofreader` backend (9092) stays live and unchanged throughout the pivot.
- Verdict POST to `/v1/verdict` closes the apprenticeship loop — preserve this path.
- Long-poll timeout: 300s on `/v1/proofread`, 30s everywhere else (feedback_upstream_timeout).
- Doorman wire: response carries `.content`, not `.choices[0].message.content`.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier).
- Port 2222 needs a GCE firewall rule — coordinate with Command Session / operator.

## Technology stack (decided 2026-05-16)

ratatui + crossterm + russh (embedded SSH server, port 2222) + tui-textarea + similar + syntect + tokio

## Last session handoff

*2026-05-17: Phase 0 COMPLETE. `app-console-content` crate committed, `cargo build` green, gate passed (ratatui TUI renders over SSH port 2222). Next session: Phase 1 (auth + session management) — SQLite user store, `auth_publickey` fingerprint lookup, `proofctl user add`, identity in status bar. See NEXT.md Phase 1 checklist and the autonomous dev plan at `~/.claude/plans/can-you-make-a-deep-naur.md` Session 2.*
