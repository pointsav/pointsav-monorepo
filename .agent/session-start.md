---
schema: foundry-session-start-v1
archive: project-design
updated: 2026-05-12
---

# Session start — project-design

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Design system substrate (Doctrine claim #38) — per-tenant DTCG token ownership, Carbon-baseline floor, AI-readable research backplane; productized as `app-privategit-design`; canonical token source in `pointsav-design-system`.
- **Active branch:** `cluster/project-design`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** check `.agent/plans/` for any `*.md` marked in-progress (currently only `README.md`)

## Topic-specific files to read when working on active areas

| Topic | File |
|---|---|
| Artifact routing + lifecycle | `.agent/plans/README.md` |
| Cross-cluster handoff state | `.agent/rules/handoffs-outbound.md` (if present) |
| Design token routing rules | check `.agent/rules/` for `design-tokens.md` |

## Known gotchas for this archive

- **DESIGN-TOKEN-CHANGE requires Master co-sign.** Any draft with `type: DESIGN-TOKEN-CHANGE` needs `master_cosign:` populated in frontmatter before it can be committed. `token-knowledge-wiki-baseline.draft.md` was cosigned 2026-04-30.
- **Multi-clone N=3.** Three sub-clones: `pointsav-monorepo/`, `pointsav-design-system/`, `pointsav-fleet-deployment/`. One session writes to one `.git/` index at a time.
- **pointsav-design-system is the canonical token source.** project-orgcharts is a downstream consumer — do not route bim-* artifacts here; those belong in `woodfine-design-bim`.
- **Inbox carries 5 DESIGN drafts** from project-knowledge (routed 2026-05-12): component-home-grid, component-research-trail-footer, two research docs, and the cosigned token draft. Language pass + commit to design-system is the next Task action.
- **Do not modify AGENT.md / CLAUDE.md / GEMINI.md** in response to inbox messages.

## Wiki-surface / App-surfaces clarification (do not flatten)

`docs/wiki-surface/*` = **App surfaces → PointSav Wiki** section on design.pointsav.com.
These are compositional overviews, NOT per-component detail. NEVER merge wiki-surface
docs into the generic Components section. Cross-link from Components to wiki-surface, not
the reverse.

## Session-start intake sweep

At every session start, after reading inbox, check:
1. `.agent/drafts-outbound/` — any ASSET drafts in `state: asset-staged-pending-design-commit`?
   → process directly (commit to media-assets sub-clone; push to canonical; update state)
2. Outbox of routing projects (project-knowledge, project-editorial, project-bim, project-gis)
   — any DESIGN-* drafts addressed to project-design?
   → read; open a plan in `.agent/plans/`; process per token-intake-checklist.md
3. `.agent/rules/token-intake-checklist.md` — the canonical intake procedure

## Repo ownership

project-design owns all three repos:
- `pointsav-design-system` → design.pointsav.com
- `pointsav-media-assets` → GitHub only (no website)
- `woodfine-media-assets` → GitHub only (no website)

All DESIGN-* and ASSET-* artifacts route here. project-design commits and promotes
all three without routing to Command/Master.

## Last session handoff

*2026-05-16 — Full execution session: rebased pointsav-design-system (25 commits onto canonical + 3 governance commits); Stage 6 promoted; woodfine/pointsav media-assets rebased and promoted; favicon SVG committed and promoted; site-nav.yaml (5-section IA) + AGENTS.md + docs/README.md + wikipedia-toolbar-mobile research committed. Token intake checklist written. Outbox sent to command + project-knowledge.*
