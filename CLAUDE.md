@~/Foundry/AGENT.md

# project-bim — Archive Guide

> **State:** active | **Last updated:** 2026-05-18
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

Commits via `~/Foundry/bin/commit-as-next.sh "<message>"` from archive root.
**Stage 6 self-service (this archive):** `~/Foundry/bin/self-service-promote.sh`
— pushes code commits to staging mirrors + appends to `promote-queue.jsonl`.
Command Session processes canonical merge. Do NOT run `promote.sh` directly.

## Deploy model — local-first, Command takes it live

This archive builds/deploys to `local-bim` on the workspace VM (port 9096) as
a self-service step. It does NOT push to foundry-prod directly.

Review: operator runs `ssh foundry-workspace-preview` and browses
`localhost:9096` to approve changes before they go live.

Going live: Command Session runs `~/Foundry/bin/push-to-prod.sh bim`
(`target_bim()`) after operator approval — pushes the binary + vault +
design-system + library dirs, restarts `local-bim-orchestration` on
foundry-prod.

## Artifacts produced here

For each piece of work, classify per `~/Foundry/conventions/artifact-classification.yaml`:
TOPIC-* / GUIDE-* / COMMS-* → `.agent/drafts-outbound/` → project-editorial.
DESIGN-* / ASSET-* → `.agent/drafts-outbound/` → project-design.
BIM-* → `.agent/drafts-outbound/` → project-bim.
CODE-* / SCRIPT-* / CONFIG-* / DATA-* → commit directly (self-contained).

## Conflicts

If a workspace rule conflicts with anything stated here, **stop and surface
the conflict via outbox to command session** — do not silently override.

## MCP tools — `foundry` server (use at startup)

`get_session_brief(role="totebox", archive="project-bim")` replaces manually reading
inbox.md, outbox.md, NOTAM.md, session-context.md. Call it first.

| Tool | When to use |
|---|---|
| `get_session_brief` | **First call at startup** — inbox, outbox, NOTAM, session-context |
| `send_mailbox_message` | Send any mailbox message (M-2/M-10 audit compliant) |
| `query_datagraph` | Entity lookup before answering about people/projects |
| `ask_local` | OLMo 7B local inference — free, SYS-ADR-07-safe |

## pointsav-monorepo sub-clone

Generic sub-clone conventions (fast gates, commit rules, layout) live at
`@~/Foundry/conventions/pointsav-monorepo-subclone-guide.md` — that file is
never archive-specific and is never touched by any archive's Stage-6
promotion. This archive's own identity/mission content belongs only here,
never in the sub-clone.

## Real BIM reference material — `business-admin/project-floorplates`

**2026-07-30 split:** `content_class: business-admin` (hybrid archive, same
pattern as `project-proforma`/`project-orgcharts`). Real Key Plans/Tiles/
Floor Plates source docs (was tracked here as `inputs/`/`outputs/`/`preview/`)
purged from git history, moved to `business-admin/project-floorplates/`.
**Reference in place by absolute path — do not copy back or `git add`/commit
here or in `pointsav-monorepo`.** See AGENT.md's business-admin section +
`business-admin/project-floorplates/README.md`.
