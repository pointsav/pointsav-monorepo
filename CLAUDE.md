@~/Foundry/AGENT.md

# project-console — Archive Guide

> **State:** active | **Last updated:** 2026-06-14
> **Cluster manifest:** `.agent/manifest.md`
> **Workspace AGENT.md takes precedence on conflict.**

---

## Cluster mission

`pointsav-monorepo` — the os-console TUI binary and all `app-console-*` cartridge crates.
`os-console` is a keyboard-native SSH terminal interface to the Totebox Archive.
Cartridges registered on F-keys; chassis in `app-console-keys`.

**Active cartridges:** F4 Content, F9 SLM, F11 System, F12 Input (Anchor), F3 Email, F6 Bookkeeper.
**Phase 9 complete 2026-06-14:** SIGTERM, Prometheus :9299, fail2ban port 2222, multi-tab F4.
**Next:** Phase 10 — F2 People cartridge, chassis reconnect watchdog.

## Tetrad

See `.agent/manifest.md` `tetrad:` block for the canonical declaration.

## At session start

Per `~/Foundry/AGENT.md` § Session roles:

1. Confirm role: `~/Foundry/bin/foundry-role.sh` (Totebox Session expected)
2. Write session lock: `.agent/engines/<engine-id>/session.lock`
3. Read `.agent/manifest.md` — cluster mission + tetrad
4. Call `get_session_brief(role="totebox", archive="project-console")` — replaces inbox, NOTAM, session-context reads
5. Read `~/Foundry/NOTAM.md` — workspace warnings
6. Read `.agent/rules/*.md` if present

## Hard rules

`~/Foundry/AGENT.md` § Hard rules — identity store immutable, never chmod;
preview before writing; edit in place (no _V2 files); one session per repo;
Bloomberg standard; BCSC posture; SYS-ADR-07/10/19.
`~/Foundry/CLAUDE.md` § Size discipline — per-archive CLAUDE.md ≤ 150 lines.

## Build notes

- All cartridges: lib crates; `os-console` is the binary
- Workspace members declared in `pointsav-monorepo/Cargo.toml`
- `cargo check -p app-console-keys -p app-console-content -p os-console` is the fast gate
- Doorman endpoint: `http://localhost:9080` (confirmed `009b2e04`; not 8011)
- Prometheus metrics: `http://127.0.0.1:9299/metrics` (configurable via `metrics_port`)
- SSH server: `--features ssh-server`; listens port 2222

## Commit + promote

Commits via `~/Foundry/bin/commit-as-next.sh "<message>"`.
Stage 6 promotion via `~/Foundry/bin/promote.sh` from Command Session.
**Stage 6 pending:** commits through `a27860b3` need promote.

## Conflicts

Surface conflicts via outbox to Command Session — do not silently override.

## MCP tools — `foundry` server (use at startup)

`get_session_brief(role="totebox", archive="project-console")` replaces manually reading
inbox.md, outbox.md, NOTAM.md, session-context.md. Call it first.

| Tool | When to use |
|---|---|
| `get_session_brief` | **First call at startup** — inbox, outbox, NOTAM, session-context |
| `send_mailbox_message` | Send any mailbox message (M-2/M-10 audit compliant) |
| `get_doorman_status` | Tier A/B/C + circuit state |
| `query_datagraph` | Entity lookup before answering about people/projects |
| `ask_local` | OLMo 7B local inference — free, SYS-ADR-07-safe |

## Artifact types — bright-line rules

TOPIC = explains WHAT/WHY; public wiki; bilingual EN+ES.
GUIDE = instructs HOW-NOW; woodfine-fleet-deployment/<name>/; English-only.
CODE = runs our systems; no customer license; internal deploy only.
