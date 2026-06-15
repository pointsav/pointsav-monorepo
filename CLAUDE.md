@~/Foundry/AGENT.md

# project-knowledge — Archive Guide

> **State:** active | **Last updated:** 2026-06-15
> **Cluster manifest:** `.agent/manifest.md`
> **Workspace AGENT.md takes precedence on conflict.**

---

## Cluster mission

`app-mediakit-knowledge` — Wikipedia-pattern HTTP knowledge wiki engine (Apache 2.0).
Single Rust binary; 3 live instances on vault-privategit-source-1.
Substrate substitution for MediaWiki per Doctrine claim #29.

**Live instances:** documentation.pointsav.com (:9090) · projects.woodfinegroup.com (:9093) · corporate (:9095).
**Phase 9 complete 2026-06-14:** WCAG 2.2 focus outline; sitemap/i18n repairs; defects 1/4/8 fixed; Sprint C 7-category IA.
**Next:** Defect 6 (images route); blueprint rendering; Phase 1 mobile foundation (Inter + Source Serif 4 approved).

## Tetrad

See `.agent/manifest.md` `tetrad:` block for the canonical declaration.

## At session start

Per `~/Foundry/AGENT.md` § Session roles:

1. Confirm role: `~/Foundry/bin/foundry-role.sh` (Totebox Session expected)
2. Write session lock: `.agent/engines/<engine-id>/session.lock`
3. Read `.agent/manifest.md` — cluster mission + tetrad
4. Call `get_session_brief(role="totebox", archive="project-knowledge")` — replaces inbox, NOTAM, session-context reads
5. Read `~/Foundry/NOTAM.md` — only if `notam_active: true` from step 4
6. Read `.agent/rules/*.md` if present

## Hard rules

`~/Foundry/AGENT.md` § Hard rules — identity store immutable, never chmod;
preview before writing; edit in place (no _V2 files); one session per repo;
Bloomberg standard; BCSC posture; SYS-ADR-07/10/19.
`~/Foundry/CLAUDE.md` § Size discipline — per-archive CLAUDE.md ≤ 150 lines.

## Build notes

- Crate: `app-mediakit-knowledge` in `pointsav-monorepo/app-mediakit-knowledge/`
- Fast gate: `cargo check -p app-mediakit-knowledge`
- Test gate: `cargo test -p app-mediakit-knowledge` (67 unit + 70+ integration)
- Lint gate: `cargo clippy -p app-mediakit-knowledge -- -D warnings`
- Run locally: `cargo run -p app-mediakit-knowledge -- serve --content-dir <path>`
- Doorman endpoint: `http://localhost:9080`

## Commit + promote

Commits via `~/Foundry/bin/commit-as-next.sh "<message>"` (from archive root or sub-clone).
Stage 6 promotion via `~/Foundry/bin/promote.sh` from Command Session.
**Stage 6 pending:** `c3261f0e` + `91e65e05` + `9a613ad6` + `f2852d5c` + `f5ac0251` (5 commits; 3 outbox messages queued).

## Conflicts

Surface conflicts via outbox to Command Session — do not silently override.

## MCP tools — `foundry` server (use at startup)

`get_session_brief(role="totebox", archive="project-knowledge")` replaces manually reading
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
