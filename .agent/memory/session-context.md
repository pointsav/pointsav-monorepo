---
schema: foundry-session-context-v1
archive: project-bim
---

# Session context — project-bim

Rolling 3-session summary. Newest entry first. Keep only 3; push oldest to session-context-archive.md.

---

## 2026-06-20 | totebox@claude-code | app-privategit-bim UI polish

**Done:**
- Applied three UI fixes to `app-privategit-bim` (clean-sheet Carbon rewrite):
  1. **Hero + header/footer restored** — `cds-header-name` now shows "Woodfine | BIM Object Library"; home page has hero tagline, three article sections (problem statement, BIM Objects answer, browse CTA), dark three-column footer. Content sourced verbatim from `app-orchestration-bim/src/main.rs`.
  2. **Sidebar always open** — removed hamburger toggle (`cds-header-menu-button`), added `expanded` attribute to `cds-side-nav`. Resolves Carbon conflict between hamburger pattern and `is-not-child-of-header` persistent rail.
  3. **Token table fixed** — replaced `cds-data-table` / `cds-table-*` web components (inline-rendered before JS initialises) with plain `<table class="bim-token-table">`. CSS selectors added to `bim-components.css`; hero/footer CSS added to `bim-layout.css`.
- Files changed: `shell.rs`, `card.rs`, `home.rs`, `bim-layout.css`, `bim-components.css`
- Committed `39d3cb0b` to monorepo cluster branch: "feat(app-privategit-bim): restore header/footer/intro; fix sidebar; replace cds-data-table with plain table"
- Preview running on port 9206 (SSH tunnel: `ssh -L 9206:localhost:9206 jennifer@34.53.65.203 -N`); verified all fixes via curl
- Created `BRIEF-app-privategit-bim.md` + `briefs/README.md` (new files)

**Root cause learned:** Previous session edits landed in phantom `/srv/foundry/clones/project-bim/app-privategit-bim/` path (doesn't exist). Actual source is always in `/srv/foundry/clones/project-bim/pointsav-monorepo/app-privategit-bim/`.

**Pending / carry-forward:**
- Stage 6 promotion of monorepo cluster branch → Command Session (outbox message sent)
- Production deploy of `app-privategit-bim` (replace `app-orchestration-bim` on port 9096, nginx config, systemd service) → Command Session

**Operator preferences surfaced:**
- User browses via SSH tunnel (`ssh -L 9206:localhost:9206 jennifer@34.53.65.203 -N`) — not WireGuard VPN

---

## 2026-05-28 | totebox@claude-code | preview-viewers + routing-correction

**Done:**
- Created `preview/po-1-floor-plan.html` — SVG floor plan of PO-1 at 1:20 scale; 6 furniture pieces; dimension callouts; zone fills; HTML legend table
- Created `preview/interior-tokens.html` — HTML viewer for interior.dtcg.json; 7 furniture Object cards + circulation constraint panel; colour-coded by category
- Copied `preview/interior.dtcg.json` — point-in-time snapshot in preview/ for local serving
- Copied all 3 new preview files to `outputs/` for rsync access
- Copied `interior.dtcg.json` to correct location: `woodfine-bim-library/tokens/bim/interior.dtcg.json`
- Updated BRIEF-tool-keyplan.md: interior.dtcg.json and key-plans.dtcg.json locations corrected

**Operator corrections (saved to memory):**
- In project-bim, say **BIM Objects** not "tokens"
- All BIM Object files route to `woodfine-bim-library`, not `pointsav-design-system`

**CRITICAL — routing errors from prior session to resolve:**
- `pointsav-design-system/tokens/bim/interior.dtcg.json` — misrouted; pending admin-tier removal by Command
- `pointsav-design-system/tokens/bim/key-plans.dtcg.json` — PO-1 structured furniture_refs applied here in error; `woodfine-bim-library/tokens/bim/key-plans.dtcg.json` still needs that update

**Pending / carry-forward:**
- Apply PO-1 structured furniture_refs to `woodfine-bim-library/tokens/bim/key-plans.dtcg.json`
- Deliverable 1b: `woodfine-bim-library/key-plans/key-plans-registry.md` standalone Markdown
- Corporate Office SVG diagrams: no furniture; blocked on zone depth data
- Apply Decisions 1–4 to DTCG Objects + delete BIM_TOKENS block from HTML
- DTCG 6 missing Object files
- Binary ledger entry at Command Session for app-orchestration-bim v0.0.3
- Stage 6: 37+ commits ahead of origin on cluster/project-bim

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

