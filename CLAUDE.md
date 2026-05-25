@~/Foundry/AGENT.md

# project-command — Archive Guide

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

## Conflicts

If a workspace rule conflicts with anything stated here, **stop and surface
the conflict via outbox to command session** — do not silently override.
