---
schema: foundry-session-start-v1
archive: project-system
updated: 2026-05-14
---

# Session start — project-system

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Substrate-shaped cluster — the substrate touches every numbered runtime under `~/Foundry/deployments/`. Owns vendor-side showcase content (public-facing Customer and Community Members) and customer-tier operational mirror content. Receives `GUIDE-substrate-rollout-{onprem,cloud,leased}.md` drafted Task-side; public bundle per Doctrine §VIII. Real-time feedback loop for every cluster's Task when substrate breaks.
- **Active branch:** `cluster/project-system`
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
- **Doorman:** `http://localhost:9080`. Response field: `.content`.
- **Input Machine (F12):** global intercept → POST to `service-input` → classify + route + audit.
  SYS-ADR-10: cannot be bypassed from other panes.

## Known gotchas

- Substrate changes have system-wide impact — changes here propagate to every deployment runtime. Coordinate with Command Session before any substrate modification.
- Vendor-side and customer-tier content is mirrored — ensure changes are consistent across both branches of content.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier).

## Last session handoff

*No prior handoff recorded.*
