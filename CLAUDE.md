@~/Foundry/AGENT.md

# project-knowledge — Archive Guide

> **State:** active | **Last updated:** 2026-06-02
> **Cluster manifest:** `.agent/manifest.md`
> **Workspace AGENT.md takes precedence on conflict.**

---

## Cluster mission

See `.agent/manifest.md` for full mission statement.

## Tetrad

See `.agent/manifest.md` `tetrad:` block for the canonical declaration
across vendor / customer / deployment / wiki legs.

## At session start

Per `~/Foundry/AGENT.md` § Session roles:

1. Confirm role: `~/Foundry/bin/foundry-role.sh` (Totebox Session expected)
2. Write session lock: `.agent/engines/<engine-id>/session.lock`
3. Read `.agent/manifest.md` — cluster mission + tetrad
4. Read `.agent/inbox.md` — pending messages
5. Read `~/Foundry/NOTAM.md` — workspace warnings
6. Read `.agent/rules/*.md` if present (may be absent for newer archives)

## Hard rules (workspace-level, do not duplicate; reference only)

- `~/Foundry/AGENT.md` § Hard rules — identity store immutable, never
  chmod; preview before writing; edit in place (no _V2 files);
  one session per repo; Bloomberg standard; BCSC posture; SYS-ADR-07/10/19.
- `~/Foundry/CLAUDE.md` § Size discipline — per-archive CLAUDE.md ≤ 150 lines.

## Commit + promote

- Commits via `~/Foundry/bin/commit-as-next.sh "<message>"`. Direct
  `git commit` is blocked by the pre-commit gate (Phase 1.13).
- Stage 6 promotion via `~/Foundry/bin/promote.sh` from the
  Command Session, not from this Totebox.

## Artifacts produced here

For each piece of work, classify per `~/Foundry/conventions/artifact-classification.yaml`:
TOPIC-* / GUIDE-* / COMMS-* → `.agent/drafts-outbound/` → project-editorial.
DESIGN-* / ASSET-* → `.agent/drafts-outbound/` → project-design.
BIM-* → `.agent/drafts-outbound/` → project-bim.
CODE-* / SCRIPT-* / CONFIG-* / DATA-* → commit directly (self-contained).

## MCP tools available — `foundry` server

The `foundry` MCP server is registered globally (`~/.claude.json`) and active in every
session without setup. Prefer these over curl/grep for live system state queries.

| Tool | When to use |
|---|---|
| `query_datagraph(q, limit?)` | Look up entities before answering questions about people, companies, projects |
| `get_entity_context(entity)` | Full profile for a specific named entity |
| `ask_local(prompt, max_tokens?)` | Submit prompt to OLMo 7B — free, local, SYS-ADR-07-safe |
| `doorman_health()` | Check tier / circuit state before inference decisions |
| `get_corpus_stats()` | Queue depth + daily cost summary |
| `mutate_datagraph(mutation)` | Create / update graph entities (requires explicit operator intent) |
| `submit_extraction(text, schema)` | Queue prose for entity extraction pipeline |

**Known fault (2026-06-05):** `ask_local` does not receive DataGraph entity context —
graph injection in the Doorman appears broken. Workaround: `query_datagraph` first,
then pass relevant entities in the prompt manually. Investigation steps: BRIEF §9c.

## Conflicts

If a workspace rule conflicts with anything stated here, **stop and surface
the conflict via outbox to command session** — do not silently override.
