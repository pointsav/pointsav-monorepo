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

*2026-05-14 — Housekeeping session (T1–T7) complete. T1: systemd unit site_title + guide_dir_2 verified correct, NEXT.md closed. T2: app-mediakit-knowledge README.es.md refreshed to match English (all phases, production URL). T3: 6 OS stub articles in content-wiki-documentation/systems/ expanded (files renamed from topic-* to canonical os-* slugs; os-workplace EN+ES written from scratch; frontmatter + wikilinks updated; 15 files in one commit). T4: 3 bilingual draft pairs archived in drafts-outbound/archived/ (confirmed committed downstream). T5: pointsav-fleet-deployment CLAUDE.md created (86 lines). T6: 4 missing README.es.md files added to fleet-deployment subdirectories. T7: factory-release-engineering license audit — 3 issues found, outbox message sent to Command for ps-administrator action. Pending: binary rebuild + 3-service restart (Master scope, unchanged from prior session). Phase 6B gated on BP6.*
