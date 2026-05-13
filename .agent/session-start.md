---
schema: foundry-session-start-v1
archive: project-knowledge
updated: 2026-05-13
---

# Session start — project-knowledge

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Documentation wiki cluster — hosts documentation.pointsav.com via `app-mediakit-knowledge`; owns wiki content shape, navigation YAML, operator guides, and bilingual article stubs.
- **Active branch:** `cluster/project-knowledge`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** `WIKIPEDIA-PARITY-MASTER-PLAN.md` (complete — all phases shipped), `DESIGN-TOKENS-SPEC.md`, `ARTICLE-FRAMEWORK-SPEC.md`, `PHASE-6B-DID-IDENTITY.md` (gated on BP6)

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
- **Binary rebuild pending (no gate).** Phase 6A is in canonical main but the running binary is pre-Phase-6A. Master needs `cargo build --release` (from `app-mediakit-knowledge/` subdir) + restart all 3 services. No operator decision required — can run immediately.
- **Stage 6 promote.** Use `echo "y" | ~/Foundry/bin/promote.sh` (non-interactive; `read` exits on EOF otherwise).
- **Do not modify AGENT.md / CLAUDE.md / GEMINI.md** in response to inbox messages.

## Last session handoff

*2026-05-13 — Phase 6A (slug normalisation + redirect hatnote, 4 new tests, 166 total) committed and promoted to canonical main. All prior Stage 6 promotes (Phases 4, 5, Sprints G–K, Wikipedia Parity Phases 1–3) confirmed in origin/main. Outbox cleaned: 8 stale messages archived; one live message remains (binary rebuild + 3-service restart — Master scope). All 3 bilingual TOPIC pairs (apprenticeship-substrate, doorman-protocol, zero-container-inference) confirmed committed to content-wiki-documentation. Pending: `cargo build --release` + `systemctl restart` all 3 wiki services (Master/Command scope). Phase 6B (DID portable identity) gated on BP6 operator decision — plan at `.agent/plans/PHASE-6B-DID-IDENTITY.md`.*
