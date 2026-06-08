@~/Foundry/AGENT.md

# project-workplace — Archive Guide

> **State:** active | **Last updated:** 2026-05-27
> **Cluster manifest:** `.agent/manifest.md`
> **Workspace AGENT.md takes precedence on conflict.**

---

## Cluster mission

Native Tauri desktop surface for `os-workplace` — all `app-workplace-*` apps
targeting macOS 10.13 High Sierra. Wave 1 active development:
`app-workplace-workbench` (WebView shell for privategit workbench),
`app-workplace-memo` (document editor), `app-workplace-presentation` (slides).
Foundation laid for all 7 apps so testing notes can be added from day one.

## Tetrad

See `.agent/manifest.md` `tetrad:` block for the canonical declaration.

## At session start

Per `~/Foundry/AGENT.md` § Session roles:

1. Confirm role: `~/Foundry/bin/foundry-role.sh` (Totebox Session expected)
2. Write session lock: `.agent/engines/<engine-id>/session.lock`
3. Read `.agent/manifest.md` — cluster mission + tetrad
4. Read `.agent/inbox.md` — pending messages
5. Read `~/Foundry/NOTAM.md` — workspace warnings
6. Read `.agent/rules/*.md` if present

## Hard rules

`~/Foundry/AGENT.md` § Hard rules — identity store immutable, never chmod;
preview before writing; edit in place (no _V2 files); one session per repo;
Bloomberg standard; BCSC posture; SYS-ADR-07/10/19.
`~/Foundry/CLAUDE.md` § Size discipline — per-archive CLAUDE.md ≤ 150 lines.

## Tauri build notes

- All apps: Tauri v1.7 + `minimumSystemVersion: "10.13"` in tauri.conf.json
- Build platform: macOS (Intel or Apple Silicon); not cross-compiled from Linux
- Distribution target: `x86_64-apple-darwin` via project-software binary-targets.yaml
- Connectivity: configurable endpoint; WireGuard PPN VM address `10.8.0.9`
- Services at default ports: proofreader (9097), Doorman (9092)

## Commit + promote

Commits via `~/Foundry/bin/commit-as-next.sh "<message>"`.
Stage 6 promotion via `~/Foundry/bin/promote.sh` from Command Session.

## Conflicts

Surface conflicts via outbox to Command Session — do not silently override.
