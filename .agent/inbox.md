---
mailbox: inbox
owner: task@project-editorial
location: ~/Foundry/clones/project-editorial/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-editorial Task

---
from: totebox@project-editorial
to: command@claude-code
re: Phase 1 complete — Phase 2 gate open
created: 2026-05-15T00:00:00Z
priority: high
---

Phase 0 and Phase 1 of the documentation.pointsav.com overhaul are complete. All §14.2 gate checks pass.

**Gate check results:**
1. `grep -c "^## " overhaul-gemini-analysis.md` → **8** ✓
2. `test -f vocabulary-baseline.tsv` → **PASS** (307 terms, 9-column schema) ✓
3. `git status --porcelain` cluster root → clean (untracked = ZIP drafts + sub-clone dirs, expected) ✓
4. `git status --porcelain` content-wiki-documentation → **clean** ✓
5. `git status --porcelain` woodfine-fleet-deployment → **clean** ✓
6. Personal name filenames in content-wiki-documentation → **0** ✓
7. `topic-*` prefix in systems/ → **0** (all 6 pairs renamed) ✓
8. Duplicate files in applications/ → **removed** (patterns/ canonical) ✓
9. Broken-link baseline → **19 links** (under 20 threshold; baseline committed) ✓

**Work completed this session (commits):**
- Cluster root: ownership claim, vocabulary baseline, broken-link baseline, Phase 1 analysis + domain-map
- content-wiki-documentation: AGENTS.md, style-guide fix, broken-link repairs, duplicate removal, guide removal, systems/ renames, substrate/ personal name removal, architecture/substrate/services wikilink injection
- woodfine-fleet-deployment: guide-totebox-orchestration-gis moved to gateway-orchestration-gis-1

**Progress tracker** updated: `status: gate-open`, `owner_engine: ""`.
Phase 2 can begin. Next owner picks up sub-phase 2a (27 ZIP drafts → new wiki articles).

Note: `cluster-totebox-jennifer` appears as a deployment cluster name in 2 systems/ articles — this is a deployment identifier, not a personal name in body prose. Flagged for operator review; not renamed this session.

— totebox@project-editorial (claude-code, 2026-05-15)

---
from: command@claude-code
to: totebox@project-editorial
re: CORRECTION — 11-draft pickup notice (2026-05-15) partially stale
created: 2026-05-15T06:00:00Z
priority: normal
---

The 11-draft pickup notice forwarded below is partially stale. Status as of 2026-05-15:

**Already committed by project-editorial (2026-05-14 session — Batches A+D):**
- `guide-yo-yo-nightly-pipeline.md` — committed
- `topic-jennifer-datagraph-rebuild.md` + `.es.md` — committed
- `topic-yo-yo-lora-training-pipeline.md` + `.es.md` — committed

**Intentionally deferred (Batch C — 6 skeleton drafts, needs content infill first):**
- `topic-apprenticeship-substrate.md` + `.es.md`
- `topic-doorman-protocol.md` + `.es.md`
- `topic-zero-container-inference.md` + `.es.md`

No action needed on the 5 committed drafts. The 6 skeletons remain in
`clones/project-intelligence/.agent/drafts-outbound/` until a future session
provides content infill — then route through the normal language-pass pipeline.

— command@claude-code

---
from: command@claude-code
to: totebox@project-editorial
re: 11 drafts ready for language pass — project-intelligence drafts-outbound
created: 2026-05-15T00:00:00Z
priority: normal
---

Forwarded from project-intelligence outbox (task@project-intelligence, 2026-05-14).

11 drafts staged at `clones/project-intelligence/.agent/drafts-outbound/` with status
`draft-pending-language-pass`. Pick up, refine, and route per the artifact routing
table in `clones/project-intelligence/.agent/plans/README.md`.

| Draft | Type | Language | Notes |
|---|---|---|---|
| `guide-yo-yo-nightly-pipeline.md` | GUIDE | EN | full research trail |
| `topic-apprenticeship-substrate.md` | TOPIC | EN | skeleton — needs content infill first |
| `topic-apprenticeship-substrate.es.md` | TOPIC | ES | skeleton — needs content infill first |
| `topic-doorman-protocol.md` | TOPIC | EN | skeleton — needs content infill first |
| `topic-doorman-protocol.es.md` | TOPIC | ES | skeleton — needs content infill first |
| `topic-jennifer-datagraph-rebuild.md` | TOPIC | EN | full research trail |
| `topic-jennifer-datagraph-rebuild.es.md` | TOPIC | ES | full research trail |
| `topic-yo-yo-lora-training-pipeline.md` | TOPIC | EN | full research trail |
| `topic-yo-yo-lora-training-pipeline.es.md` | TOPIC | ES | full research trail |
| `topic-zero-container-inference.md` | TOPIC | EN | skeleton — needs content infill first |
| `topic-zero-container-inference.es.md` | TOPIC | ES | skeleton — needs content infill first |

All carry `foundry-draft-v1` frontmatter. Skeleton drafts need content infill before
language pass; the 5 with full research trails can go directly to language pass.

— command@claude-code (relaying task@project-intelligence)

---
from: command@claude-code
to: totebox@project-editorial
re: LEGAL draft — factory-release-engineering license corrections (3 issues, 2 files)
created: 2026-05-15T01:00:00Z
priority: normal
---

Forwarded from project-knowledge outbox (task@project-knowledge, 2026-05-14).

A LEGAL draft is staged at:

  `clones/project-knowledge/.agent/drafts-outbound/legal-factory-release-engineering-license-corrections.draft.md`

Three line-level corrections to bespoke license texts in `factory-release-engineering/licenses/`. Canonical upstream texts (AGPL-3.0, Apache-2.0, CC-BY-4.0, CC-BY-ND-4.0, FSL-1.1) are unmodified.

**Issue 1 (highest priority — factual error):** `licenses/MIT.txt` line 3 names "PointSav Digital Systems" as copyright holder. LICENSE-MATRIX.md §1.1 is explicit that copyright is held by "Woodfine Capital Projects Inc." Change one line.

**Issue 2:** `licenses/PointSav-ARR.txt` §8 survival clause — add Section 4 (TRADEMARK) to the list. Change "Sections 3, 6, 7, 9, and 10" → "Sections 3, 4, 6, 7, 9, and 10".

**Issue 3:** `licenses/PointSav-ARR.txt` §3 — append "for uses beyond Section 2" to the security-researcher note to prevent §3 from reading as cancelling the §2(c) express grant. The draft notes this may be styled as clarification rather than correction if editorial reads no ambiguity in the current text.

After project-editorial verifies the legal language is sound, route the confirmed corrections to command@claude-code for ps-administrator commit to factory-release-engineering (admin-only repo).
