---
schema: foundry-draft-v1
state: draft
language_protocol: LEGAL-corrections
originating_cluster: project-knowledge
target_repo: factory-release-engineering
target_path: licenses/
audience: ps-administrator (via project-editorial review)
bcsc_class: governance
authored: 2026-05-14
authored_by: task@project-knowledge
authored_with: claude-sonnet-4-6
references:
  - factory-release-engineering/licenses/MIT.txt
  - factory-release-engineering/licenses/PointSav-ARR.txt
  - factory-release-engineering/LICENSE-MATRIX.md
notes_for_editor: >
  Three targeted corrections to custom license texts in factory-release-engineering.
  All corrections are line-level edits — no structural changes to any license.
  Please verify each correction is legally sound before routing to ps-administrator
  for commit. The MIT.txt correction (Issue 1) is the most time-sensitive — it names
  the wrong legal entity as copyright holder in a deployed license text.
---

# LEGAL corrections — factory-release-engineering license texts

Audit source: T7 session (project-knowledge cluster, 2026-05-14).
Three issues found in bespoke license files. Canonical upstream texts (AGPL-3.0.txt,
Apache-2.0.txt, CC-BY-4.0.txt, CC-BY-ND-4.0.txt, FSL-1.1-Apache-2.0.txt) were not
modified and require no action.

---

## Issue 1 — MIT.txt: wrong copyright holder (factual error — highest priority)

**File:** `licenses/MIT.txt`

**Current line 3:**
```
Copyright (c) 2026 PointSav Digital Systems
```

**Corrected line 3:**
```
Copyright (c) 2026 Woodfine Capital Projects Inc.
```

**Rationale:** LICENSE-MATRIX.md §1.1 states explicitly: "Copyright across all
repositories listed in this matrix is held by Woodfine Capital Projects Inc. ('WCP Inc.')".
Every other custom IP document in this repository uses WCP Inc. as the copyright holder
(PointSav-ARR.txt line 3, PointSav-Commercial.txt line 3, MIXED-MONOREPO-NOTICE.txt line 3).
PointSav Digital Systems is a subsidiary brand, not the IP holding entity. A deployed
license file that names the wrong copyright holder is a factual error in the chain of
title. No other content in MIT.txt requires change.

---

## Issue 2 — PointSav-ARR.txt §8: Section 4 (TRADEMARK) missing from survival clause

**File:** `licenses/PointSav-ARR.txt`

**Current §8 text (last sentence of the section):**
```
Sections 3, 6, 7, 9, and 10 survive termination.
```

**Corrected §8 text:**
```
Sections 3, 4, 6, 7, 9, and 10 survive termination.
```

**Rationale:** Section 4 of PointSav-ARR.txt is the TRADEMARK clause. Trademark
restrictions should survive license termination — they prevent a former licensee from
using Woodfine Marks after their license ends. While trademark law independently provides
this protection, the contractual silence means the contract itself does not bind the
former licensee to this obligation post-termination. Adding Section 4 to the survival
clause makes the obligation explicit in the agreement and removes ambiguity. No other
content in §8 requires change.

---

## Issue 3 — PointSav-ARR.txt §3: security-researcher note potentially overrides §2(c) grant

**File:** `licenses/PointSav-ARR.txt`

**Current last sentence of §3 (after the restriction list):**
```
No exceptions are made for security researchers, named partners, or reviewers
acting under non-disclosure agreements. Any broader use by such parties requires
a separate written agreement with Woodfine.
```

**Corrected text:**
```
No exceptions are made for security researchers, named partners, or reviewers
acting under non-disclosure agreements for uses beyond Section 2. Any broader use
by such parties requires a separate written agreement with Woodfine.
```

**Rationale:** Section 2(c) expressly grants security researchers the right to
"reference the Material in good-faith academic, journalistic, or security-research
contexts with attribution to Woodfine." The §3 note as currently drafted could be
read as cancelling that express §2(c) grant — a licensee could argue that "no
exceptions for security researchers" means §2(c) does not apply to them. The
intended meaning is that security researchers receive no exceptions from the §3
restrictions for uses *beyond* the §2 view/reference grant. Inserting "for uses
beyond Section 2" makes this explicit without weakening the restriction on broader use.

---

## Research trail

**Sources consulted:**
- Full text of `licenses/MIT.txt`, `licenses/PointSav-ARR.txt`, `licenses/PointSav-Commercial.txt`
- `licenses/MIXED-MONOREPO-NOTICE.txt`
- `LICENSE-MATRIX.md` §1.1 (copyright posture), §2 (authority), §5 (propagation artifacts)
- `NEXT.md` — DEF-001 through DEF-004 (pre-existing defects, not related to these corrections)

**Informational items not requiring correction:**
- `licenses/FSL-1.1-Apache-2.0.txt` — `${year}` and `${licensor name}` placeholders are
  standard FSL template design; propagation scripts fill them at deploy time. No defect.
- `licenses/MIXED-MONOREPO-NOTICE.txt` — omits `app-orchestration-*` under EUPL-1.2;
  this is a known pending governance defect tracked as DEF-001/DEF-002 in NEXT.md.
- DEF-001 through DEF-004 in NEXT.md are governance gaps (EUPL-1.2 catalog, SPDX headers,
  DTCG data layer), not writing errors in the license texts themselves.

**Open questions for project-editorial:**
- Issue 3 involves legal interpretation of how §2 and §3 interact. If the editorial
  team or counsel reads the current §3 note as *not* creating ambiguity (i.e., the
  "broader use" qualifier is already implied by context), Issue 3 may be styled as
  a clarification rather than a correction. Issue 1 and Issue 2 are unambiguous errors.
