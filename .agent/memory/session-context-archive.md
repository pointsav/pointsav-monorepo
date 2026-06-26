

---

## 2026-05-28 | totebox@claude-code | preview-viewers + routing-correction

**Done:**
- Created `preview/po-1-floor-plan.html` — SVG floor plan of PO-1 at 1:20 scale; 6 furniture pieces; dimension callouts; zone fills; HTML legend table
- Created `preview/interior-tokens.html` — HTML viewer for interior.dtcg.json; 7 furniture Object cards + circulation constraint panel; colour-coded by category
- Copied `preview/interior.dtcg.json` — point-in-time snapshot in preview/ for local serving
- Copied all 3 new preview files to `outputs/` for rsync access
- Copied `interior.dtcg.json` to correct location: `woodfine-bim-library/tokens/bim/interior.dtcg.json`
- Updated BRIEF-tool-keyplan.md: interior.dtcg.json and key-plans.dtcg.json locations corrected

**Pending / carry-forward:**
- Apply PO-1 structured furniture_refs to `woodfine-bim-library/tokens/bim/key-plans.dtcg.json`
- Deliverable 1b: `woodfine-bim-library/key-plans/key-plans-registry.md` standalone Markdown
- Corporate Office SVG diagrams: blocked on zone depth data
- DTCG 6 missing Object files; 5 Rust crates
- Binary ledger entry at Command Session for app-orchestration-bim v0.0.3

**Operator preferences surfaced:** say "BIM Objects" not "tokens" in project-bim; all BIM Object files route to woodfine-bim-library.

---

## 2026-05-23 | totebox@claude-code | tool-keyplan-scaffold

**Done:**
- Scaffolded `tool-keyplan` Rust crate — TOML config → validated DTCG JSON engine (v0.0.1)
- Created `pointsav-design-system/tokens/bim/interior.dtcg.json` — 7 furniture tokens + 1 circulation constraint token
- Created `pointsav-monorepo/tool-keyplan/` crate; updated workspace Cargo.toml
- Updated key-plans.dtcg.json PO-1 with structured furniture_refs + compliance
- Engine validation: ASR A1.2 ✓ European Lighting ✓ Wheelchair ✓
- Created `.agent/briefs/BRIEF-tool-keyplan.md`

**Carry-forward:** Deliverable 1b key-plans-registry.md; Corporate Office SVG; Decisions 1–4; 6 DTCG files; binary ledger v0.0.3

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

---

## 2026-05-21 | totebox@claude-code | plan-bim-objects

**Done:** Read 15+ source documents; created plan-bim-objects.md first draft; resolved Q1–Q6; read FIN.xlsx authoritative sizes; specified Deliverable 1.

**Pending:** Deliverable 1 registry MD, DTCG standardisation, HTML BIM_TOKENS removal, Rust scaffold, Stage 6 (~31 commits ahead).

**Operator preferences:** `fpull bim outputs/`; eco-region variants deferred; Corporate Office sizing deferred.
