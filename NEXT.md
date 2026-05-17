# NEXT.md — project-bim

Open items, deferred work, and follow-up actions.
Attribution format: `[YYYY-MM-DD role@engine]`

---

## Hot — pick up here next session

- [ ] **HTML print layout — page 1 still needs tuning** `[2026-05-17 totebox@claude-code]`
  - File: `preview/building-width-calculator.html`
  - Section 1 (The Backwards Method) must fit on one landscape letter page: masthead + h2 + lede + 2 paragraphs + strip-wrap diagram + Zone 1/2/3 descriptions + callout
  - Print CSS spacing already tightened this session (line-height 1.4, p margin 5px, lede 4px/8px, h2/h3 tighter, callout 8px/14px, strip-wrap h3:first-child margin-top 0, max-width none on p and lede)
  - Not yet verified in browser print-preview — confirm Section 1 fits cleanly on page 1 before next work
  - Lever remaining if still overflowing: `.strip-row { padding: 4px 12px; }` saves ~30px

- [ ] **DTCG token files — 6 missing files to create** `[2026-05-17 totebox@claude-code]`
  - Full spec in `.agent/plans/tool-buildingwidth-architecture.md`
  - Priority order:
    1. `furniture.dtcg.json` — manufacturer SKUs + zone derivation (highest priority; missing foundation)
    2. `floor-plate-assembly-rules.dtcg.json` — FP-* validation rules
    3. `building-grid.dtcg.json` — structural module + tolerance bands
    4. Medium Tile family additions to `tile-system.dtcg.json`
    5. Special Tiles additions to `tile-system.dtcg.json`
    6. `tenant-mix.dtcg.json` (move from `floor-plate-standards.dtcg.json`)
  - Also: 6 internal inconsistencies to fix (see plan §Internal inconsistencies)

- [ ] **Rust crate scaffold** — after DTCG files are complete: `[2026-05-17 totebox@claude-code]`
  - 5 crates: `bim-units`, `bim-tokens`, `bim-furniture`, `tool-buildingwidth`, `tool-floorplates`
  - Full architecture in `.agent/plans/tool-buildingwidth-architecture.md`

---

## Operator-pending (blocked — do not touch)

- [ ] **DTCG accuracy errors** — 3 files pending source citations from operator `[2026-05-13 command@claude-code]`
  - `climate-zones.dtcg.json` — ASHRAE 90.1 zones + valid bSDD URIs needed
  - `performance.dtcg.json` — IFC4 Pset_DoorCommon.IsFireExit property name
  - `materials.dtcg.json` — IFC/ISO 10077 material vs. assembly thermal property distinction
  - **Do not edit without operator-confirmed citations**

---

## Deferred

- [ ] Stage 6 push — 8 commits ahead of origin on `cluster/project-bim` `[2026-05-17 totebox@claude-code]`
