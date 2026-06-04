---
schema: foundry-session-start-v1
archive: project-orgcharts
updated: 2026-06-04
---

# Session start — project-orgcharts

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Author Woodfine corporate org charts (and corporate visualizations over
  time) using the PointSav design system. Every UI pattern that emerges during chart
  authoring — node shapes, hierarchy connectors, role badges, brand placement — is
  backfilled as design-system tokens/components via project-design gateway.
  Corporate document archive shared scope with `project-bookkeeping`.
- **Active branch:** `cluster/project-orgcharts`
- **Sub-clones:** `pointsav-design-system/` (primary), `pointsav-media-assets/` (sibling),
  `woodfine-media-assets/` (sibling)
- **Deployment instance:** `~/Foundry/deployments/cluster-totebox-corporate-1/`
  (`inputs/` ← Jennifer uploads; `working/` ← in-progress; `outputs/` ← final PDF/HTML/SVG)
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)

## Known gotchas

- Shared document scope with `project-bookkeeping` — coordinate if touching GUIDE
  artifacts that span both archives.
- UI patterns discovered here go to `pointsav-design-system` via `project-design`
  gateway — never commit directly from this archive to the design system.
- Commit via `~/Foundry/bin/commit-as-next.sh` only (staging-tier); Stage 6 via
  Command Session `bin/promote.sh`.
- Bencal naming conflict unresolved: JW2 uses "Bencal Private Capital Inc." (BPC);
  JW3 compliance doc uses "Bencal Corporation" (BCL). Check NEXT.md before chart work.

## Last session handoff

*2026-06-04 — Startup sweep (contamination cleanup):*
- Inbox/outbox/inbox-archive owner headers fixed (were project-marketing/project-intelligence).
- Valid inbox message ACK'd: Command confirmed 3 DESIGN drafts committed + promoted to
  `pointsav-design-system` (commits 0e6f37e, aca9646, 252a035). `--wf-green` updated
  to `#198038`. Green drift fully resolved.
- Identity restoration commit this session: CLAUDE.md, manifest, session-start, NEXT.md,
  session-context, briefs README all restored/rewritten from correct project-orgcharts content.
- Stage 6 pending for both mailbox sweep commit (f3e20162) and this identity commit.
