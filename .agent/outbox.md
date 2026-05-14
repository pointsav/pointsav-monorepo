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


