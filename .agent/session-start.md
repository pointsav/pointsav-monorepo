---
schema: foundry-session-start-v1
archive: project-proofreader
updated: 2026-05-20
---

# Session start — project-proofreader

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** os-console platform — chassis-first Leapfrog 2030. Single binary (`os-console`) with
  `app-console-keys` base chassis + compiled-in cartridges (F1–F12). `app-console-content` is the
  F4 cartridge (proofreader + drafter). All owned in `pointsav-monorepo/`.
- **Active branch:** `cluster/project-proofreader` (pending rename → `cluster/project-console`)
- **Architecture plan:** `.agent/plans/os-console-platform.md` — read this before any architecture discussion
- **Coding roadmap:** `.agent/plans/leapfrog-2030-coding.md` — phased plan; Phase 1 is next
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)

## Critical state

- **Phase 0 COMPLETE** — `app-console-content` crate committed; `cargo build` green; SSH TUI gate
  passed (ratatui frame confirmed over port 2222 via `ssh -p 2222 -i ~/.ssh/google_compute_engine
  mathew@localhost`).
- **Phase 1 = chassis first** — next task is creating `app-console-keys` lib crate and converting
  `app-console-content` from a standalone binary to a lib crate (F4 Cartridge). See
  `leapfrog-2030-coding.md` Phase 1 checklist.
- **No `app-console-keys/` Cargo.toml yet** — Reserved-folder in catalog; Phase 1 creates it.
- **SSH key note**: `mathew` user has no standard `id_ed25519`; use `-i ~/.ssh/google_compute_engine`
  for localhost testing until Phase 2 adds `proofctl user add`.
- **russh 0.60 API**: native async fn in impl (no async_trait); `russh::keys::PrivateKey::random(&mut rand::rng(), Algorithm::Ed25519)`; `session.channel_success(channel)?` in pty_request + shell_request; TerminalHandle uses sink Vec<u8> + flush sends. See memory for full reference.
- **Web UI is to be taken down** — blocked on Command Session (sudo). Teardown checklist in
  `tui-pivot-2030.md` §Part 6. `local-proofreader-console.service` (9091) + `local-proofreader-public.service` (9097) + nginx vhost + cert.
- **Pending rename:** project-proofreader → project-console. Outbox msg sent to Command.

## Architecture at a glance (2026-05-20)

- `os-console` = single binary; `app-console-keys` = base chassis (always-installed, like
  `service-fs` for os-totebox); other cartridges are optional compiled-in lib crates.
- **F-key map (WIP):** F1=help, F2=people, F3=email, F4=content, F5=minutebook, F6=bookkeeper,
  F7=bim, F8=gis, F9=slm, F10=**app-console-mesh**, F11=system, **F12=input (The Anchor, immovable)**
- **MBA** = peer-to-peer (os-console ↔ os-totebox/os-orchestration/etc.); NOT network-layer.
  `system-gateway-mba` is server side; `app-console-keys` is client side (shows `MBA LINK ACTIVE`).
- **PPN** = WireGuard infrastructure only; deliberately isolated from os-* application layer.
- **Doorman:** `http://localhost:8011` (NOT 9080). Response field: `.content`.
- **Input Machine (F12):** global intercept → POST to `service-input` → classify + route + audit.
  SYS-ADR-10: cannot be bypassed from other panes.

## Known gotchas

- **Do not swap or upgrade the SLM model** until the operator explicitly lifts the constraint.
- `service-proofreader` backend (9092) stays live and unchanged.
- Verdict POST to `/v1/verdict` closes the apprenticeship loop — preserve this path.
- Long-poll timeout: 300s on `/v1/proofread`, 30s everywhere else.
- Doorman wire: response carries `.content`, not `.choices[0].message.content`.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier).
- Port 2222 needs a GCE firewall rule — coordinate with Command Session / operator.
- `tui-pivot-2030.md` is superseded by the new plans; ignore its Phase 7 chassis deferral and 9080 port references.

## Technology stack

ratatui 0.30 + crossterm 0.28 + russh 0.60 (feature-gated ssh-server) + tui-textarea 0.7 +
similar 2.5 + syntect 5.2 + nucleo 0.5 + rusqlite 0.32 + pdfium-render 0.8 (Phase 7) + tokio

## Last session handoff

*2026-05-20 (session 3): Architecture fully resolved. os-console platform plans written
(`os-console-platform.md`, `leapfrog-2030-coding.md`). 4 TOPIC drafts (MBA, PPN, os-console,
Input Machine) + 2 GUIDE drafts (MBA pairing ceremony, operator reference) staged to
`drafts-outbound/`. Outbox msg to Command requesting rename + catalog additions. Phase 1
(chassis: `app-console-keys` lib crate + Cartridge trait) is next coding task.*
