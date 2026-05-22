---
schema: foundry-session-context-v1
archive: project-bim
---

# Session context — project-bim

Rolling 3-session summary. Newest entry first. Keep only 3; push oldest to session-context-archive.md.

---

## 2026-05-22 | totebox@claude-code | key-plans-site-size-specific-furniture

**Done:**
- Implemented size-specific furniture in `render_kp_zone_svg()` — `app-orchestration-bim v0.0.3` (commit 8ce0b9ba)
- Added `size_tier: u8` computation from `area_m2` × `category` thresholds (S/M/L per category)
- Replaced plan_w-based furniture counts with tier-exact counts from architect sketches (`DISCOVERY_MCorp_Sketches_Key Plans_Summary.pdf`)
- Key differences now visible: PO 1/2/3 desks; Medical 2/4/6 dental chairs + 1/1/2 doc offices; Lab 3/5/7 bench clusters; Business 3×3/4×4/5×5 workstations + 2/3/5 exec offices; Academic workstation bank→dual banks→theater seats; Civic 2/4/5 offices + 1/2/2 conf rooms + court room (L)
- Deployed binary; service active at bim.woodfinegroup.com
- Updated `plan-bim-objects.md` brief: status active-live, Medical/Lab display names corrected, SVG system section added (commit 9b27506, v0.0.4)
- Created `.agent/briefs/BRIEF-key-plans-site.md`

**Pending / carry-forward:**
- Corporate Office SVG diagrams: `_ => {}` (no furniture); awaiting zone depth data
- Deliverable 1b: `woodfine-bim-library/key-plans/key-plans-registry.md` standalone Markdown
- Apply Decisions 1–4 to DTCG tokens + delete BIM_TOKENS block from HTML
- DTCG 6 missing files (unblocked)
- Rust crate scaffold (deferred until DTCG complete)
- Binary ledger entry at Command Session
- Stage 6: 34+ commits ahead of origin on cluster/project-bim

**Operator preferences surfaced:** none new this session.

---

## 2026-05-21b | totebox@claude-code | plan-bim-objects-v2-committed

**Done:**
- Merged Deliverable 1 spec (authoritative FIN.xlsx sizes, Q1–Q6, Decisions 1–4) into plan-bim-objects.md → status: draft-v2
- Committed: 667c5f2 "plan: BIM Objects draft-v2 — authoritative sizes + Q1-Q6 + Decisions 1-4"
- Copied v2 to `outputs/plan-bim-objects.md` — accessible via `fpull bim outputs/`

**Inbox state at session end:** unchanged from prior session.

**Pending / carry-forward:**
- Deliverable 1: write `woodfine-bim-library/key-plans/key-plans-registry.md` — READY, all data in plan-bim-objects.md v2
- Apply Decisions 1–4 to existing DTCG tokens + delete BIM_TOKENS block from HTML
- B5: Rust source for app-orchestration-bim (HIGH)
- DTCG 6 missing files (unblocked by Decision 3)
- Rust crate scaffold (deferred until DTCG complete)
- Stage 6: 32+ commits ahead of origin

**Operator preferences surfaced:** will proceed with Deliverable 1 in next session.

---

## 2026-05-21 | totebox@claude-code | plan-bim-objects

**Done:**
- Read all 15+ source documents from `inputs/` (15 PDFs, XLSX, DOCX, 3 collaborator folders)
- Created `.agent/plans/plan-bim-objects.md` — first draft BIM Objects specification (Key Plans / Tiles / Floor Plates / Building Width Calculator / Rust engine data model)
- Created `outputs/` directory at `/srv/foundry/clones/project-bim/outputs/` — accessible via `fpull bim outputs/`
- Resolved Q1–Q6 with operator for Deliverable 1 (Key Plans registry)
- Read FIN.xlsx `Summary_Key Plans` tab — authoritative sizes for all professional office Key Plans
- Specified Deliverable 1 fully in plan: `woodfine-bim-library/key-plans/key-plans-registry.md`
- Read inbox: received 4 Key Plans foundation decisions from Command (all answered) + NOTAM permission resolved notification

**Inbox state at session end:**
- `command-20260520-bim-foundation-decisions` (HIGH / pending): 4 decisions received — naming, HTML BIM_TOKENS deletion, scope (all 3 building types), tile disambiguation. **Unblocks** DTCG work + Rust scaffold
- B5 (HIGH / in-progress): Rust source for app-orchestration-bim — not started this session; pivoted to plan-bim-objects
- DTCG accuracy errors (2 messages / operator-pending): unchanged

**Pending / carry-forward:**
- Deliverable 1: write `woodfine-bim-library/key-plans/key-plans-registry.md` (READY — plan complete)
- Apply Decisions 1–4 to existing DTCG tokens + delete BIM_TOKENS block from HTML
- B5: Rust source for app-orchestration-bim (still HIGH)
- DTCG 6 missing files (now unblocked by Decision 3 scope confirmation)
- Rust crate scaffold (still deferred until DTCG complete)
- Stage 6: 31+ commits ahead of origin

**Operator preferences surfaced:**
- Pull pattern: `fpull bim outputs/` → `/srv/foundry/clones/project-bim/outputs/` on VM
- Eco-region variants (landscaping/parking) deferred to later iteration
- Corporate Office sizing deferred until Floor Plate dimensions confirmed

---

## 2026-05-21 | totebox@claude-code | plan-bim-objects

**Done:** Read 15+ source documents; created plan-bim-objects.md first draft; resolved Q1–Q6; read FIN.xlsx authoritative sizes; specified Deliverable 1.

**Pending:** Deliverable 1 registry MD, DTCG standardisation, HTML BIM_TOKENS removal, Rust scaffold, Stage 6 (~31 commits ahead).

**Operator preferences:** `fpull bim outputs/`; eco-region variants deferred; Corporate Office sizing deferred.
