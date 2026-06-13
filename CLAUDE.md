@~/Foundry/AGENT.md

# project-design — Archive Guide

> **State:** active | **Last updated:** 2026-06-12
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
4. Call `get_session_brief(role="totebox", archive="project-design")` — replaces inbox, NOTAM, session-context reads
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
DESIGN-* / ASSET-* → direct commit to pointsav-design-system sub-clone or `.agent/drafts-outbound/`.
BIM-* → `.agent/drafts-outbound/` → project-bim.
CODE-* / SCRIPT-* / CONFIG-* / DATA-* → commit directly (self-contained).

## MCP tools — `foundry` server (use at startup)

`get_session_brief(role="totebox", archive="project-design")` replaces manually reading
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
DESIGN-COMPONENT / DESIGN-RESEARCH = design system substrate → pointsav-design-system.
DESIGN-TOKEN-* = requires master_cosign in frontmatter before commit to design-system.
ASSET-* = media files → pointsav-media-assets or woodfine-media-assets.
Split rule: declaratives → TOPIC, imperatives → GUIDE; same slug, different prefix, no shared sentences.

## Conflicts

If a workspace rule conflicts with anything stated here, **stop and surface
the conflict via outbox to command session** — do not silently override.
