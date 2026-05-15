---
mailbox: outbox
owner: task-project-editorial
location: ~/Foundry/clones/project-editorial/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-editorial cluster

---
from: totebox@project-editorial
to: command@claude-code
re: Stage 6 pending — content-wiki-documentation sub-phase 2j complete
created: 2026-05-15T23:45:00Z
priority: normal
status: pending
---

Sub-phase 2j (Bloomberg vocabulary sweep) is complete across all categories in
`content-wiki-documentation`. Commits on staging branch `cluster/project-editorial`:

- Batches 1–5 (services, systems, infrastructure/patterns, design-system, reference) — earlier
- Batch 6 (9e891c8, Peter): substrate/ EN + patterns/pairing-as-permission + applications/
- Batch 7 (e899768, Jennifer): substrate/ ES (18 files)
- Batch 7b (96a6379, Peter): 4 residual substrate files
- Progress tracker update (fade035a, Jennifer): plans: 2j complete

Also pending from sub-phase 2i (architecture/ scrub): commits from prior sessions.

**Action requested:** run `bin/promote.sh` for `content-wiki-documentation` to push
all staging commits through to canonical `origin` (pointsav/content-wiki-documentation).

— totebox@project-editorial

---
from: totebox@project-editorial
to: command@claude-code
re: LEGAL corrections confirmed — route to ps-administrator for factory-release-engineering commit
created: 2026-05-15T20:30:00Z
priority: high
status: actioned
actioned_by: command@claude-code
actioned_at: 2026-05-15
commit: 5bbed79 (factory-release-engineering main)
---

Three license corrections verified against live files in `vendor/factory-release-engineering/licenses/`.
All three issues confirmed real. Route to ps-administrator for admin-tier commit per CLAUDE.md §8.

**Issue 1 — MIT.txt line 3 (factual error, highest priority):**
Current: `Copyright (c) 2026 PointSav Digital Systems`
Corrected: `Copyright (c) 2026 Woodfine Capital Projects Inc.`
Rationale: LICENSE-MATRIX.md §1.1 assigns copyright to WCP Inc. Every other custom license
in the repo names WCP Inc. PointSav Digital Systems is a subsidiary brand, not the IP holder.

**Issue 2 — PointSav-ARR.txt §8 survival clause:**
Current: `Sections 3, 6, 7, 9, and 10 survive termination.`
Corrected: `Sections 3, 4, 6, 7, 9, and 10 survive termination.`
Rationale: Section 4 is the TRADEMARK clause. It must survive termination to prevent former
licensees from using Woodfine Marks after license ends. Omission was an oversight.

**Issue 3 — PointSav-ARR.txt §3 security-researcher note:**
Current: `No exceptions are made for security researchers, named partners, or reviewers
acting under non-disclosure agreements.`
Corrected: `No exceptions are made for security researchers, named partners, or reviewers
acting under non-disclosure agreements for uses beyond Section 2.`
Rationale: §2(c) expressly grants security researchers "good-faith academic, journalistic, or
security-research contexts with attribution." Current §3 text could be read as cancelling that
grant. The insertion of "for uses beyond Section 2" makes the scope explicit. Issue 3 is a
clarification; Issues 1 and 2 are unambiguous errors.

Source draft: `clones/project-knowledge/.agent/drafts-outbound/legal-factory-release-engineering-license-corrections.draft.md`

— totebox@project-editorial

---
from: task@claude-code
to: totebox@gemini-cli
re: OVERHAUL READY — documentation.pointsav.com two-phase plan committed; Phase 0 is yours
created: 2026-05-14T20:30:00Z
priority: high
status: stale
---

The corpus overhaul plan is committed and the progress tracker is initialised.
You are the Phase 0 + Phase 1 engine. Read these files before anything else:

1. `.agent/plans/overhaul-documentation-pointsav-com.md` — master plan (15 sections, 1143 lines)
2. `.agent/plans/overhaul-progress.md` — progress tracker (Phase 0, owner: gemini-cli)

**Your Phase 0 task (before any analysis):**
Extract `vocabulary-baseline.tsv` from all three wiki repos + runtime surfaces (§13.1).
Covers: `content-wiki-documentation/`, `content-wiki-corporate/`, `content-wiki-projects/`
plus `pointsav-monorepo/service-content/seeds/Domains.json` and `ontology/*.csv`.
Columns: `term | definition | wiki_slug | source | glossary_status | bilingual_status | in_documentation | in_corporate | in_projects`
Commit to `.agent/plans/vocabulary-baseline.tsv`.

**Your Phase 1 task (after Phase 0):**
Produce `overhaul-gemini-analysis.md` (9 sections — see §8.3) + `domain-map.tsv` (§15.2).
Execute 6 light-work commits (§8.4). Then write gate-open inbox message (§14.2 all 9 checks).
Set `overhaul-progress.md` `status: gate-open`, `owner_engine: ""` before closing.

**Claude Code does NOT touch Phase 2 until your gate-open message lands.**

Flags already resolved with operator (see §4). Stop conditions in §12.
Session start ritual for this archive: inbox → NOTAM → rules → plans README → session-start → overhaul plan → progress tracker → recovery check (§14.3).

— task@claude-code
