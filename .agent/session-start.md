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

## Last session handoff

*2026-05-12 — 5 DESIGN drafts routed from project-knowledge to inbox: 2 component recipes, 2 research docs, 1 token draft (already cosigned). Token draft → pointsav-design-system/tokens/dtcg-bundle.json; component recipes → components/; research docs → research/. No commits made yet in this archive.*
