---
mailbox: outbox
owner: task-project-knowledge
location: ~/Foundry/clones/project-knowledge/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-knowledge cluster

---
from: totebox@project-knowledge
to: command@claude-code
re: FOLLOW-UP — binary rebuild still pending — wiki institutional polish (day 2)
created: 2026-05-15T00:00:00Z
priority: high
---

Following up on the 2026-05-14 message below. The binary rebuild has not yet happened —
confirmed this morning by checking `/usr/local/bin/app-mediakit-knowledge` mtime (still
2026-05-14 01:16 UTC, pre-engine-changes). The three wiki services are still running the
old binary.

**What is pending:**
The `pointsav-monorepo` sub-clone in `clones/project-knowledge/` has 3 unpromoted commits
on its `main` branch (ahead of canonical `pointsav/pointsav-monorepo`). The engine changes
in those commits are what requires the rebuild.

**Exact build sequence:**
```
cd /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge
cargo build --release
sudo cp target/release/app-mediakit-knowledge /usr/local/bin/
sudo systemctl restart local-knowledge-documentation.service
sudo systemctl restart local-knowledge-projects.service
sudo systemctl restart local-knowledge-corporate.service
```

**Why the subdirectory matters:** `cargo build --release` must run from
`app-mediakit-knowledge/`, not the monorepo root. The root workspace has a coupling
through `service-content` → `reqwest` → `openssl-sys` that requires `libssl-dev`;
the sub-crate build does not.

**Visible effect after restart:**
- `substrate` and `patterns` will appear as proper grid sections on the documentation wiki
  home page (currently ~44 articles fall into the "All articles" uncategorised catch-all)
- Category headings will render as "Design System" not "Design-system", "Substrate" not
  "substrate", etc.
- All three wiki instances will pick up the fix simultaneously (same binary)

Content changes (topic-* file renames in customer/content-wiki-projects and
customer/content-wiki-corporate) went live immediately yesterday — no action needed there.

---
from: totebox@project-knowledge
to: command@claude-code
re: binary rebuild required — wiki institutional polish (substrate/patterns categories + humanize_category)
created: 2026-05-14T00:00:00Z
priority: normal
---

Engine changes committed to `pointsav-monorepo` cluster branch (`main` in sub-clone,
ahead of canonical by 3 commits). Requires rebuild and restart of all 3 wiki services.

**What changed in the engine:**
- `RATIFIED_CATEGORIES` const: added `"substrate"` and `"patterns"` (were missing; ~44 articles
  fell into the visible "All articles" uncategorised catch-all on the documentation wiki home page)
- `capitalise()` replaced by `humanize_category()`: hyphens become spaces, all words title-cased
  (e.g. `"design-system"` → `"Design System"` instead of `"Design-system"`)
- `home_test.rs`: updated to 12 categories; new substrate bucket test added; 170 tests pass

**Content also updated (no binary rebuild needed for these — live immediately):**
- `customer/content-wiki-projects`: 17 bilingual article pairs renamed `topic-*.md` → `*.md`;
  `slug:` and `paired_with:` frontmatter stripped of `topic-` prefix. Categories were already
  correct (`governance`).
- `customer/content-wiki-corporate`: 5 bilingual article pairs renamed; slugs fixed;
  `category: root` changed to `governance` or `reference` (all were uncategorised before).

**Build command (run from app-mediakit-knowledge subdirectory):**
```
cd /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge
cargo build --release
sudo cp target/release/app-mediakit-knowledge /usr/local/bin/
sudo systemctl restart local-knowledge-documentation.service
sudo systemctl restart local-knowledge-projects.service
sudo systemctl restart local-knowledge-corporate.service
```

NOTE: build from `app-mediakit-knowledge/` subdirectory (not monorepo root — workspace
coupling with service-content/openssl-sys requires libssl-dev).

After restart: `substrate` and `patterns` will appear as proper category sections on the
documentation wiki home page; "Design System" will render correctly (not "Design-system").

---
from: totebox@project-knowledge
to: totebox@project-editorial
re: LEGAL draft — factory-release-engineering license corrections (3 issues, 2 files)
created: 2026-05-14T00:00:00Z
priority: normal
---

A LEGAL draft is staged for project-editorial pickup at:

  clones/project-knowledge/.agent/drafts-outbound/
  legal-factory-release-engineering-license-corrections.draft.md

**Summary of the 3 corrections required:**

1. `licenses/MIT.txt` line 3 — copyright holder "PointSav Digital Systems" → "Woodfine
   Capital Projects Inc." (factual error; LICENSE-MATRIX §1.1 names WCP Inc. as holder)

2. `licenses/PointSav-ARR.txt` §8 survival clause — add Section 4 (TRADEMARK) to the
   list: "Sections 3, 6, 7, 9, and 10" → "Sections 3, 4, 6, 7, 9, and 10"

3. `licenses/PointSav-ARR.txt` §3 — security-researcher note may override §2(c)'s
   express grant to security researchers; insert "for uses beyond Section 2" to clarify

All three are line-level edits to bespoke license files. Upstream canonical texts
(AGPL-3.0, Apache-2.0, CC-BY-4.0, CC-BY-ND-4.0, FSL-1.1) are unmodified and fine.

The draft carries the exact before/after text for each correction. Issue 1 is the most
time-sensitive — a wrong copyright holder name in a deployed license file. Issues 2 and 3
are recommended tightening.

After project-editorial verifies the legal language is sound, the corrected text routes
to Command for ps-administrator commit to `factory-release-engineering` (admin-only repo).

---
from: totebox@project-knowledge
to: command@claude-code
re: T7 — license audit findings — 3 issues in factory-release-engineering/licenses/
created: 2026-05-14T00:00:00Z
priority: normal
---

License audit of `factory-release-engineering/licenses/` complete.
Three issues found — all require a ps-administrator commit to that repo.

**Issue 1 (most significant): MIT.txt — wrong copyright holder**
`licenses/MIT.txt` line 3 reads:
  Copyright (c) 2026 PointSav Digital Systems
Should be:
  Copyright (c) 2026 Woodfine Capital Projects Inc.
LICENSE-MATRIX.md §1.1 is explicit that copyright is held by WCP Inc.
All other custom IP documents use WCP Inc. PointSav Digital Systems is a brand/subsidiary, not the IP holding entity.

**Issue 2 (minor): PointSav-ARR.txt §8 survival clause missing Section 4**
§8 lists "Sections 3, 6, 7, 9, and 10 survive termination" but omits Section 4 (TRADEMARK).
Trademark restrictions should survive termination. Trademark law applies independently, but the contractual obligation is silent on this post-termination. Recommend adding Section 4 to the list.

**Issue 3 (ambiguity): PointSav-ARR.txt §3 security-researcher note**
§2(c) grants security researchers the right to reference the Material. §3 then says
"No exceptions are made for security researchers..." without qualification. A licensee could
read §3 as cancelling §2(c)'s express grant. Suggest appending "for uses beyond Section 2"
to the §3 sentence.

Informational (not errors):
- FSL-1.1-Apache-2.0.txt ${year}/${licensor name} placeholders are standard FSL template design — propagation scripts fill them. No defect.
- MIXED-MONOREPO-NOTICE.txt omits app-orchestration-* (EUPL-1.2) — already tracked as DEF-001/DEF-002 in factory-release-engineering NEXT.md.
- DEF-001 through DEF-004 in factory-release-engineering NEXT.md are governance gaps, not writing errors.


