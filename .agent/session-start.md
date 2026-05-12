---
schema: foundry-session-start-v1
archive: project-knowledge
updated: 2026-05-12
---

# Session start — project-knowledge

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Documentation wiki cluster — hosts documentation.pointsav.com via `app-mediakit-knowledge`; owns wiki content shape, navigation YAML, operator guides, and bilingual article stubs.
- **Active branch:** `cluster/project-knowledge`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** `WIKIPEDIA-PARITY-MASTER-PLAN.md`, `WIKIPEDIA-PARITY-RESEARCH-LOG.md`, `DESIGN-TOKENS-SPEC.md`, `ARTICLE-FRAMEWORK-SPEC.md`

## Topic-specific files to read when working on active areas

| Topic | File |
|---|---|
| Wikipedia parity work | `.agent/plans/WIKIPEDIA-PARITY-MASTER-PLAN.md` |
| Design tokens spec | `.agent/plans/DESIGN-TOKENS-SPEC.md` |
| Article framework | `.agent/plans/ARTICLE-FRAMEWORK-SPEC.md` |
| Sub-clone project registry | `pointsav-monorepo/.agent/rules/project-registry.md` |
| BP1 decision packet | `pointsav-monorepo/app-mediakit-knowledge/docs/BP1-DECISION-PACKET.md` |

## Known gotchas for this archive

- **Multi-clone N=3.** Three separate `.git/` indices: `pointsav-monorepo/`, `content-wiki-documentation/`, `woodfine-fleet-deployment/`. One session writes to one index at a time — never `git add` across sub-clones in the same command.
- **YAML structured records deleted.** `content-wiki-documentation` no longer uses `.yaml` files in article category directories. Canonical (2026-05-08) deleted them and replaced with bilingual `.md` stubs. Do not recreate YAML-only structured records.
- **Binary rebuild gate.** `app-mediakit-knowledge` Phase 4 Steps 4.6 + 4.7 are gated on the operator reading `BP1-DECISION-PACKET.md` (~15 min) before binary rebuild + service restart.
- **Stage 6 promote.** Use `echo "y" | ~/Foundry/bin/promote.sh` (non-interactive; `read` exits on EOF otherwise).
- **Do not modify AGENT.md / CLAUDE.md / GEMINI.md** in response to inbox messages.

## Last session handoff

*2026-05-12 — pointsav-monorepo (13 commits) + content-wiki-documentation (YAML cleanup) both promoted to canonical (fabcb032 / f76ce0d). 14 PROSE drafts routed to project-editorial; 5 DESIGN drafts routed to project-design. Pending: BP1 operator read → Steps 4.6+4.7 (binary rebuild + service restart).*
