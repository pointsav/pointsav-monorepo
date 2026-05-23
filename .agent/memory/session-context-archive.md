# Session Context Archive — project-knowledge cluster

Entries pushed from session-context.md when the 3-entry rolling window fills.
Newest first.

---

## 2026-05-22 | Totebox | claude-sonnet-4-6

**Done this session:**
- **lbug decision locked:** Option 1 — accept ~13.5 MB disk bloat; lbug C++ stays compiled
  into binary on all nodes, dormant on Micro. Agent-confirmed: current binary is 4.2 MB
  (shared) + 27 MB .so; static ~17.7 MB. The 2 GB RAM issue is LadybugDB mmap — solved
  by SqliteGraphStore (Phase 3), not by linking mode. Decision is final; do not revisit.
- **Phase 0-A** (`b2a09597`, Jennifer): `.agent/binary-targets.yaml` written; declares
  `slm-doorman-server` as service-package/extension for SOFT- pipeline. Inbox message
  `command-20260522-binary-targets-project-intelligence` marked actioned.
- **Phase 0-B** (`9fbff79d` Peter, `335a8575` Jennifer): all `.agent/plans/*.md` migrated
  to `.agent/briefs/BRIEF-*.md`; archive files to `briefs/archive/`; frontmatter added;
  `briefs/README.md` index created; 2 workspace briefs picked up
  (BRIEF-phase-3c-service-content-loRA-stub, BRIEF-layer3-compliance-report). Inbox
  message `command-20260521-briefs-migration-project-intelligence` marked actioned.
- **AUTO-TODO.md created** at `.agent/AUTO-TODO.md` — Phases 0–8 with gates, commit
  guidance, and lbug decision baked in. Ready for AUTO session.
- **BRIEF-flow-restructure.md** Status section updated with lbug decision + session 2
  done items + correct resume point.

**Pending / carry-forward:**
Phase 1–7 all subsequently completed in sessions 3–6 (see active session-context.md).

---

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
