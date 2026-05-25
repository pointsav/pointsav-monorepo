# Session Context Archive — project-knowledge cluster

Entries pushed from session-context.md when the 3-entry rolling window fills.
Newest first.

---

## 2026-05-22 | Totebox | claude-code

**Done this session — Phase 1 + Phase 2 complete; Phase 3 Commits A–C:**
- Branch-topology blocker: `cluster/project-knowledge` diverged 374 commits; escalated to Command; `main` confirmed as working branch.
- Phase 2 (claim-authoring convention) ratified as doctrine claim #54.
- Phase 1 (engine dead-code descope): 4 commits, ~−2,600 lines, `cargo test` green.
- Phase 3 Commits A/B/C: `claim.rs` extractor, per-claim citation resolution, claim graph in redb.
- `.agent/binary-targets.yaml` written.

**Operator preferences:** Per-commit discipline; surface drift, don't absorb it into feature commits.

---

## 2026-05-21 | Totebox | claude-code

**Done this session — knowledge-platform re-architecture + plan consolidation:**
- Major strategic session. Dispatched recon agents (engine + design-system audit) and a 4-agent OPUS research sweep (leapfrog-2030 vision, AI-native contribution model, machine-readable/flat architecture, build-vs-adopt).
- Authored `KNOWLEDGE-PLATFORM-VISION.md` (rev 4, `.agent/plans/`) — upstream vision & architecture. And `KNOWLEDGE-PLATFORM-PLAN.md` — project-knowledge's 8-phase execution plan.
- **Six decisions confirmed by operator:** (1) claim-native data model — convention-first; (2) pairing contribution model (web-login retired; os-console ↔ os-mediakit); (3) MCP reversal — keep transport, re-found as claim+contribution API; (4) `INVENTIONS.md` disclosure machinery descoped → future `project-disclosure` tenant; (5) os-console/os-mediakit a later cross-project phase; (6) Stage 6 first.
- Settled: custom Rust engine continues (not MediaWiki); three sovereign deployment instances; source-of-truth inversion (instance content repo canonical, GitHub downstream); rename `content-wiki-* → media-knowledge-*` (operator doing the GitHub side); Main Page owned by project-knowledge.
- Cross-checked with project-editorial — accepted; reconciled (Gate-0 editorial standard; one-ruleset/two-consumers linter). Recorded as Vision §14.
- Cleaned `.agent/plans/`: 12 superseded plans deleted, 3 Wikipedia-parity archived. Committed `80d50931` (Jennifer).
- Drafted project-editorial's plan → `.agent/drafts-outbound/KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.draft.md`.
- Outbox: Doctrine-amendment request to Command (`project-knowledge-20260521-doctrine-amendment-request`); consolidated-plan handoff + strict 7-step cleanup to project-editorial (`project-knowledge-20260521-editorial-plan-handoff`).

**Pending / carry-forward:** All execution phases subsequently completed 2026-05-22/23.

**Operator preferences surfaced:**
- Collapse plan sprawl: one upstream vision + one execution plan per cluster.
- "We Own It" — sovereign in-house stack; custom-built, not adopted.
- Respect archive boundaries — draft for other clusters, route via outbox.
- Decisive, fast pace; confirm decisions in batches.

---

## 2026-05-20b | Totebox | claude-code

**Done this session:** Startup sequence only. No work performed — operator issued shutdown immediately after startup.

**Pending / carry-forward:**
- Items 6 + 7 deferred (corporate glossary + thin-category audit).
- D10 wikilink validation — blocked on Stage 6 binary rebuild (Command Session scope).
- G2 canonical: `README-TOTEBOX-EGRESS.md` in `customer/woodfine-fleet-deployment/` — Command Session `git rm` + admin-tier commit needed.
- Stage 6 outstanding: content-wiki-projects (6), content-wiki-corporate (10, blocked on divergence), content-wiki-documentation (4), monorepo (16).

**Operator preferences surfaced:** None new this session.

---

## 2026-05-20 | Totebox | claude-code

**Done this session:**
- G2 (`a06f64f`, Peter): `README-TOTEBOX-EGRESS.md` removed from woodfine-fleet-deployment cluster-clone. Canonical copy still present — Command Session admin-tier commit required (outbox updated).
- PJ2 (`b138b99`, Jennifer): 5 country co-location index stubs expanded (Italy, Mexico, Nordics, Poland, Spain); ES frontmatter `paired_with:` bug fixed; `language_protocol` → `TRANSLATE-ES`.
- C8–C10 (`cb53200`, Peter): 10 new corporate wiki topics + 10 ES bilingual pairs (20 files).
- Corporate wiki housekeeping (`ebc2939`, Peter): `featured-topic.yaml` 15-topic pool; `leapfrog-facts.yaml` 9 facts; `.agent/rules/` bootstrap.
- Projects wiki housekeeping (`bffe4e3`, Jennifer): `NEXT.md` current; `CLAUDE.md` created.

**Pending / carry-forward:**
- Items 6 + 7 deferred (corporate glossary + thin-category audit).
- D10 wikilink validation — blocked on Stage 6 rebuild.
- G2 canonical removal — Command Session.
- Stage 6 outstanding: content-wiki-projects (6), content-wiki-corporate (10, blocked on divergence), content-wiki-documentation (4), monorepo (16).

**Operator preferences surfaced:**
- "do them all in parallel" / "yes" → execute immediately in parallel without re-asking.
- No trailing summaries mid-session; concise updates only.

## 2026-05-19 | Totebox | claude-code

**Done:** D3 (`cf72e67` — substrate/patterns `_index` MOC expanded 7→32 and 3→10, bilingual). D6 (`a07bdf5` — governance category complete: `sovereign-airlock-doctrine` EN+ES rewritten, 3 stubs elevated, `_index` expanded). Projects wiki PJ1/PJ4/PJ5/PJ7 committed. Outbox: consolidated 4-commit documentation promote request.

**Pending:** D10 (wikilink validation, blocked on Stage 6 rebuild). PJ2 (6 country index stubs — needs real GIS data). content-wiki-documentation +4, content-wiki-projects +4, monorepo +16 — all Stage 6 pending.

---

## 2026-05-18 | Totebox | claude-code

**Done:** D5 (short_description on 162 EN+ES docs wiki articles), D8 (governance/_index + design-system/_index frontmatter), D1/D2/D4/D7/D9 verified moot or done. PJ3 (short_description on 26 EN+ES projects wiki articles). nightly-datagraph-rebuild stub expanded. All P0 engine bugs (A–H) shipped in Sprints AD+AE.

**Pending:** D3 (substrate/patterns _index MOC), D6 (governance stubs), D10 (post-Stage-6 validation). PJ1/PJ5/PJ7 carried to next session.

---

## 2026-05-17 | Totebox | claude-code

**Done:** Full UI/UX + content + link audit across all 3 wikis (304 + 18 + 5 sitemap URLs). THREE-WIKI-REBUILD-MASTER.md plan authored from 4 content audit sub-agents + 3 UI/UX audit sub-agents. C1–C7 corporate wiki fixes committed. PJ6/PJ8 verified.

**Pending:** Stage 6 + binary rebuild (P1). Engine bugs P0-A through P0-H identified; Sprint AD candidates listed.
