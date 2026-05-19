---
schema: foundry-session-start-v1
archive: project-knowledge
updated: 2026-05-18
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

*2026-05-18 — Three-wiki UX audit + Sprints AC through AE shipped.*

*Sprint AC (`35f787e3`, Jennifer): infobox title/image support, `{{Main}}` hatnote fenced block, r#unsafe=true fix; 7 new tests → 205 total.*

*Sprint AD (`dc0d3af3`, Peter + `3514904e`, Jennifer): Engine P0: AGENT.md system-file filter, hidden-dir walk skip, per-article `<title>` tag, bare-slug 301-redirect resolver (P0-C) fixing ~280+ broken wikilinks.*

*Sprint AE (`ecd6b74a`, Jennifer): P0-E tagline from site_title (trim_end_matches " Wiki"), P0-F search index excludes system/hidden files + test. P0-G and P0-H were already present. All P0 items closed.*

*Content (projects wiki): featured-topic.yaml slug fix (`fd8848c`), short_description added to 4 key articles (`a5ffa1f`). 30 articles still missing short_description (PJ3 — continuous task).*

*DIVERGENCE WARNING (corporate wiki): cluster-clone uses `topic-*` prefix; canonical (`b0c78f6`) stripped the prefix. Master must reconcile before Stage 6 for content-wiki-corporate.*

*State: 206+ tests passing. Monorepo sub-clone 16 commits ahead of origin/main. Stage 6 promotion + binary rebuild + 3-service restart pending (Master scope; outbox updated). Next: Sprint AF — projects wiki content, remaining short_descriptions, and any P1/P2 items from THREE-WIKI-REBUILD-MASTER.md.*
