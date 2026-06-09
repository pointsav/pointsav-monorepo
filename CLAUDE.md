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
4. Call `get_session_brief(role="totebox", archive="project-workplace")` — replaces inbox, NOTAM, session-context reads
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

## MCP tools — `foundry` server (use at startup)

`get_session_brief(role="totebox", archive="project-workplace")` replaces manually reading
inbox.md, outbox.md, NOTAM.md, session-context.md. Call it first.
`send_mailbox_message()` replaces hand-editing YAML frontmatter.

| Tool | When to use |
|---|---|
| `get_session_brief` | **First call at startup** — inbox, outbox, NOTAM, session-context |
| `send_mailbox_message` | Send any mailbox message (M-2/M-10 audit compliant) |
| `query_mailbox` | Sweep archives — scope="all" in one call |
| `get_doorman_status` | Tier A/B/C + circuit state |
| `get_service_status` | Apprenticeship queue + audit-ledger counts |
| `query_datagraph` | Entity lookup before answering about people/projects |
| `ask_local` | OLMo 7B local inference — free, SYS-ADR-07-safe; graph context auto-injected |
| `cast_apprenticeship_verdict` | Sign + submit verdict on a shadow-captured attempt |
| `mutate_datagraph` | Create/update graph entities (requires explicit operator intent) |
| `submit_extraction` | Queue prose for entity extraction pipeline |

## Artifact types — bright-line rules

TOPIC = explains WHAT/WHY; public wiki; bilingual EN+ES; survives decommission; reader has no login.
GUIDE = instructs HOW-NOW; woodfine-fleet-deployment/<name>/; English-only; dies with deployment.
SOFT  = Ed25519 license key + marketplace listing + price → software.pointsav.com.
CODE  = runs our systems; no customer license; internal deploy only (published OSS with no key = CODE).
Split rule: declaratives → TOPIC, imperatives → GUIDE; same slug, different prefix, no shared sentences.
Cash register test: licensable + marketplace-listed → SOFT; everything else → CODE.
Storefront (app-privategit-marketplace) is CODE; the merchandise it sells is SOFT.
