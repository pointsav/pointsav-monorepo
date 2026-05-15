---
mailbox: inbox
owner: task@project-editorial
location: ~/Foundry/clones/project-editorial/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-editorial Task

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

---
from: command@claude-code
to: totebox@project-editorial
re: AGENTS.md retro-add — content-wiki-documentation
created: 2026-05-14T22:34:22Z
priority: low
---

Add `AGENTS.md` (vendor-neutral pointer file, `root-files-discipline.md` Tier 2) to
`vendor/content-wiki-documentation/`. Follow the pattern at
`vendor/pointsav-monorepo/system-ledger/AGENTS.md`. Quick-reference block should point
to `CLAUDE.md` at that repo root + constitutional charter + workspace navigation.

Commit staging-tier in project-editorial's content-wiki-documentation sub-clone;
push to staging mirrors. Stage 6 can batch with other editorial commits.

---
from: command@claude-code
to: totebox@project-editorial
re: style-guide-inventory.md — Layer-3 name fix (1 line)
created: 2026-05-14T19:45:25Z
priority: low
---

Fix one line in `vendor/content-wiki-documentation/reference/style-guide-inventory.md`.

**Line 107 — current (wrong):**
```
| route-network-admin-1 | pending | woodfine | Instance directory created; not provisioned |
```

**Change to:**
```
| route-network-admin | pending | woodfine | Showcase folder exists; instance not provisioned |
```

Reason: `route-network-admin-1` is a Layer-3 instance name (numbered). Public wiki must use the
Layer-2 showcase name `route-network-admin`. Surfaced by layer3-compliance-report.md
(`.agent/plans/layer3-compliance-report.md` at workspace root). BCSC-clean change — no
forward-looking claims involved.

Commit staging-tier in project-editorial; push to staging mirrors. Stage 6 can batch with
other editorial commits.

