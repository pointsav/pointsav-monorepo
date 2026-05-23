---
schema: foundry-session-context-v1
archive: project-bim
---

# Session context — project-bim

Rolling 3-session summary. Newest entry first. Keep only 3; push oldest to session-context-archive.md.

---

## 2026-05-23 | totebox@claude-code | tool-keyplan-scaffold

**Done:**
- Scaffolded `tool-keyplan` Rust crate — TOML config → validated DTCG JSON engine (v0.0.1)
- Sent 5 parallel web agents to steelcase.com; confirmed specs for Leap V2, Migration SE, Groupwork 36", TS Pedestal, Currency Bookcase, Wing Chair CH445
- Created `pointsav-design-system/tokens/bim/interior.dtcg.json` — 7 furniture tokens + 1 circulation constraint token
- Created `pointsav-monorepo/tool-keyplan/` (Cargo.toml, src/main.rs, configs/po-1.toml)
- Updated `pointsav-monorepo/Cargo.toml` — added tool-keyplan to workspace members
- Updated `key-plans.dtcg.json` PO-1 — replaced `furniture_program` string array with structured `furniture_refs` + `bounding_box_mm` + `circulation_ref` + structured `compliance`
- Engine validation passed: ASR A1.2 ✓ (30.19 m²/person ≥ 8.0) European Lighting ✓ Wheelchair ✓ (2558 mm ≥ 1500 mm)
- Created `.agent/briefs/BRIEF-tool-keyplan.md`

**CRITICAL — ALL FILES UNCOMMITTED (shutdown interrupted before git add):**
- Monorepo sub-clone: `Cargo.toml` + `Cargo.lock` + `app-orchestration-bim/Cargo.toml` + `tool-keyplan/`
- Design system: `tokens/bim/interior.dtcg.json` (NEW) + `tokens/bim/key-plans.dtcg.json` (modified PO-1)
- Archive: `.agent/briefs/BRIEF-tool-keyplan.md`
- Commit these FIRST at next session start

**Pending / carry-forward:**
- Deliverable 1b: `woodfine-bim-library/key-plans/key-plans-registry.md` standalone Markdown
- Corporate Office SVG diagrams: `_ => {}` (no furniture); awaiting zone depth data
- Apply Decisions 1–4 to DTCG tokens + delete BIM_TOKENS block from HTML
- DTCG 6 missing files (unblocked)
- Binary ledger entry at Command Session for v0.0.3
- Stage 6: 37+ commits ahead of origin on cluster/project-bim

**Operator preferences surfaced:** none new this session.

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

**Pending / carry-forward:**
- Deliverable 1: write `woodfine-bim-library/key-plans/key-plans-registry.md` — READY, all data in plan-bim-objects.md v2
- Apply Decisions 1–4 to existing DTCG tokens + delete BIM_TOKENS block from HTML
- B5: Rust source for app-orchestration-bim (HIGH)
- DTCG 6 missing files (unblocked by Decision 3)
- Rust crate scaffold (deferred until DTCG complete)
- Stage 6: 32+ commits ahead of origin

**Operator preferences surfaced:** will proceed with Deliverable 1 in next session.
