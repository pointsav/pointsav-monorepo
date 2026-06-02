@~/Foundry/AGENT.md

# project-marketing — Archive Guide

> **State:** active | **Last updated:** 2026-06-02
> **Cluster manifest:** `.agent/manifest.md`
> **Workspace AGENT.md takes precedence on conflict.**

---

## Cluster mission

See `.agent/manifest.md` for full mission statement.

## Tetrad

See `.agent/manifest.md` `tetrad:` block for the canonical declaration
across vendor / customer / deployment / wiki legs.

## CRITICAL — Live site edit workflow

**Git commits do NOT update the live marketing sites.** The live sites read from
gitignored deployment directories. To make a change appear on `home.woodfinegroup.com`
or `home.pointsav.com`:

```bash
bash scripts/edit-live-content.sh woodfine    # opens deployment HTML in $EDITOR
bash scripts/edit-live-content.sh pointsav    # same for PointSav
bash scripts/verify-live.sh                   # confirm both tenants, run after edits
```

Live files (canonical source for the served HTML):
- `home.woodfinegroup.com` → `/srv/foundry/deployments/media-marketing-landing-1/content/index.html`
- `home.pointsav.com`      → `/srv/foundry/deployments/media-marketing-landing-2/content/index.html`

Changes take effect immediately — no service restart needed. Run `verify-live.sh` at
session **start** (baseline) and **end** (confirm changes are live).

## At session start

Per `~/Foundry/AGENT.md` § Session roles:

1. Confirm role: `~/Foundry/bin/foundry-role.sh` (Totebox Session expected)
2. Write session lock: `.agent/engines/<engine-id>/session.lock`
3. Read `.agent/manifest.md` — cluster mission + tetrad
4. Read `.agent/inbox.md` — pending messages
5. Read `~/Foundry/NOTAM.md` — workspace warnings
6. Read `.agent/rules/*.md` if present (may be absent for newer archives)
7. **Run `bash scripts/verify-live.sh`** — confirm both sites are up and show current deployed state

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
