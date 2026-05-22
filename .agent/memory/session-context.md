# Session Context — project-intelligence archive

Rolling 3-session summary. Newest on top. Keep only 3 entries; push oldest to `session-context-archive.md`.

⚠️ Prior entries in this file were project-knowledge content (cross-cluster contamination
from Command Stage-6 rebase 2026-05-22 — flagged for Command; see outbox).

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
- **▶ START HERE:** Phase 1 (§8.A archive alignment, low effort) + Phase 2 (`foundry-nodeclass`
  crate, ~150 LOC) — run in parallel per AUTO-TODO.md.
- Phase 3 (`SqliteGraphStore`, ~250 LOC + runtime backend selection) — THE blocker for fleet boot.
- Phase 4 (service-slm Doorman node-class fixes — stops false Tier A on Micro).
- Phase 5 (base-tier tests + cgroup sandbox).
- Phases 6–7 (small wins, Yo-Yo W5 remainder).
- Inbox `project-editorial-20260521-e4-triage-naming-blockers` — 3 drafts need Do-Not-Use
  rename ("Yo-Yo") + personal-name rename before re-staging to drafts-outbound/. Not on
  critical path; do after Phase 2.
- Stage 6 promote — 7 commits ahead of origin/main; Command scope; rebase required first.

**Operator preferences surfaced:**
- Accept binary bloat to keep one build serving all tiers — "zero friction" principle.
- AUTO-TODO style: phase-gated with explicit gates (cargo test green) between phases.
- Comprehensive task file preferred over in-session planning.
