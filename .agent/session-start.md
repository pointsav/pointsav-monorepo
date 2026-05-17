---
schema: foundry-session-start-v1
archive: project-design
updated: 2026-05-17
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

- **DESIGN-TOKEN-CHANGE requires Master co-sign.** Any draft with `type: DESIGN-TOKEN-CHANGE` needs `master_cosign:` populated in frontmatter before it can be committed.
- **Multi-clone N=3.** Three sub-clones: `pointsav-monorepo/`, `pointsav-design-system/`, `pointsav-fleet-deployment/`. One session writes to one `.git/` index at a time.
- **pointsav-design-system is the canonical token source.** project-orgcharts is a downstream consumer — do not route bim-* artifacts here; those belong in `woodfine-design-bim`.
- **tokens/main-page/ now on canonical** (0955b5c). design-main-page-token-2 extraction complete. project-knowledge implements P2 items in app-mediakit-knowledge/src/server.rs.
- **12 GUIDE/TOPIC drafts routed to project-editorial** (2026-05-17). drafts-outbound is now clean of editorial scope — only ASSET and committed DESIGN-* remain.
- **design.pointsav.com/tokens.full.json is LIVE** — nginx serves exports/ statically. project-* archives curl this for the full DTCG bundle.
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

*2026-05-17 — Drafts cleanup session: research-ps-badge-favicon-design committed (cbfaad7); design-main-page-token-2 token extracted to tokens/main-page/main-page.dtcg.json (0955b5c); Stage 6 (b29b0a9 → 0955b5c); 12 GUIDE/TOPIC drafts routed to project-editorial; PRODUCT_VISION inbox actioned; session-start updated.*
