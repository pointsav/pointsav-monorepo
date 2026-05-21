# Session Context — project-knowledge cluster

Rolling 3-session summary. Newest on top. Keep only 3 entries; push oldest to `session-context-archive.md`.

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

**Pending / carry-forward:**
- **Execution not started.** `KNOWLEDGE-PLATFORM-PLAN.md` Phase 0 (Stage 6 — Command) and Phases 1–2 (dead-code descope + claim-authoring convention spec) are the next Totebox work. Start here next session.
- **Awaiting:** Command relay + response on the Doctrine amendment; project-editorial to finalize+commit its plan, run the strict cleanup, and reply.
- `~/.claude/plans/` (~60 stray plan-mode files) — decided: left as-is (inert scratch).
- Carried: D10 wikilink validation; production binary ~16 commits behind canonical, Stage 6 blocked.

**Operator preferences surfaced:**
- Collapse plan sprawl: one upstream vision + one execution plan per cluster; old plans deleted once the new exist.
- "We Own It" — sovereign in-house stack; the engine is custom-built, not adopted (saved as memory `principle-we-own-it`).
- Respect archive boundaries — draft for other clusters, route via outbox; never write/delete in another archive.
- Decisive, fast pace; reframe a question when asked rather than pushing it; confirm decisions in batches.

---

## 2026-05-20b | Totebox | claude-code

**Done this session:** Startup sequence only. No work performed — operator issued shutdown immediately after startup.

**Pending / carry-forward:**
- **Items 6 + 7 (start here next):** corporate glossary expansion (`glossary-corporate.csv`, 459 rows, many incomplete) + documentation wiki thin-category audit.
- **D10:** wikilink validation pass — blocked on Stage 6 binary rebuild (Command Session scope).
- **G2 canonical:** `README-TOTEBOX-EGRESS.md` still in `/srv/foundry/customer/woodfine-fleet-deployment/` — Command Session `git rm` + admin-tier commit needed.
- **Stage 6 outstanding:** content-wiki-projects (6 commits), content-wiki-corporate (10 commits, blocked on cluster/canonical divergence), content-wiki-documentation (4 commits), monorepo (16 commits).

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
