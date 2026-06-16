@~/Foundry/AGENT.md

# project-intelligence — Archive Guide

> **State:** active | **Last updated:** 2026-06-17
> **Cluster manifest:** `.agent/manifest.md`
> **Workspace AGENT.md takes precedence on conflict.**

---

## Cluster mission

SLM inference infrastructure — Doorman (Tier A/B/C routing + circuit breaker),
OLMo 7B Tier A (local CPU inference via `local-slm.service`),
OLMo 32B Tier B (yoyo-batch L4 GPU; us-central1-b target),
DataGraph entity enrichment (LadybugDB via `service-content`),
and LoRA training pipeline.

**Live services:** `local-doorman.service` (:9080) · `local-slm.service` (OLMo 7B Tier A)
**Stage 6 pending:** `23b012a1` (LoRA target_modules fix + noise filter) · `4a9c81b9` (DOC_sweep gate + sweep ledger)
**Blocked:** yoyo-batch TERMINATED — restart requires operator approval + us-central1-b + ML libs

## Tetrad

See `.agent/manifest.md` `tetrad:` block for the canonical declaration.

## At session start

Per `~/Foundry/AGENT.md` § Session roles:

1. Confirm role: `~/Foundry/bin/foundry-role.sh` (Totebox Session expected)
2. Write session lock: `.agent/engines/<engine-id>/session.lock`
3. Read `.agent/manifest.md` — cluster mission + tetrad
4. Call `get_session_brief(role="totebox", archive="project-intelligence")` — replaces inbox, NOTAM, session-context reads
5. Read `~/Foundry/NOTAM.md` — only if `notam_active: true` from step 4
6. Read `.agent/rules/*.md` if present

## Hard rules

`~/Foundry/AGENT.md` § Hard rules — identity store immutable, never chmod;
preview before writing; edit in place (no _V2 files); one session per repo;
Bloomberg standard; BCSC posture; SYS-ADR-07/10/19.
`~/Foundry/CLAUDE.md` § Size discipline — per-archive CLAUDE.md ≤ 150 lines.

## Build notes

- Doorman: `service-slm/crates/slm-doorman-server/` — run from `service-slm/` sub-workspace
- DataGraph: `service-content/` — own Cargo workspace
- Extraction: `service-extraction/` — own Cargo workspace
- Fast gate: `cargo check -p slm-doorman-server` (from `service-slm/`)
- Test gate: `cargo test -p slm-doorman-server`
- Lint gate: `cargo clippy -p slm-doorman-server -- -D warnings`
- Doorman health: `curl http://localhost:9080/doorman/health`
- MCP module_id: `jennifer` (see `.mcp.json`)

## Commit + promote

Commits via `~/Foundry/bin/commit-as-next.sh "<message>"` (from archive root or `service-slm/`).
Stage 6 promotion via `~/Foundry/bin/promote.sh` from Command Session.

## Conflicts

Surface conflicts via outbox to Command Session — do not silently override.

## MCP tools — `foundry` server (use at startup)

`get_session_brief(role="totebox", archive="project-intelligence")` replaces manually reading
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
