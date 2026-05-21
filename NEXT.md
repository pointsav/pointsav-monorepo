# NEXT.md — project-bim

Open items, deferred work, and follow-up actions.
Attribution format: `[YYYY-MM-DD role@engine]`

---

## Hot — pick up here next session

- [x] **NOTAM unreadable — resolved 2026-05-20** `[2026-05-20 totebox@claude-code]`
  - Fixed by Command: NOTAM.md now `-rw-r--r-- mathew:foundry` (world-readable). Outbox message actioned.

- [x] **Key Plans foundation — 4 operator decisions received 2026-05-20** `[2026-05-20 totebox@claude-code]`
  - All 4 decisions answered via inbox `command-20260520-bim-foundation-decisions`
  - Decision 1: descriptive display names (Index PDF style); codes (PO-1/M-1/B-1) are internal-only DTCG keys
  - Decision 2: **delete** inline BIM_TOKENS block from `building-width-calculator.html`; fetch from DTCG at render time
  - Decision 3: all 3 building types in scope now (Professional Centre + Retail Select + Tech Industrial + 12 common-area Key Plans)
  - Decision 4: type-prefixed tile codes (CO-A, RS-A, TI-A); Corridor Expander T = 300 SF; arithmetic gaps intentional by design; J/K/L/M as stub DTCG entries with `status: reserved`
  - **Now unblocked:** DTCG token standardisation, HTML BIM_TOKENS removal, Rust crate scaffold

- [ ] **Deliverable 1: key-plans-registry.md — ready to implement** `[2026-05-21 totebox@claude-code]`
  - Plan: `.agent/plans/plan-bim-objects.md` (full spec with authoritative sizes from FIN.xlsx)
  - Output file: `woodfine-bim-library/key-plans/key-plans-registry.md`
  - Also copy to `outputs/` for `fpull bim outputs/` access
  - All Q1–Q6 resolved; authoritative sizes sourced from `CONSTRUCTION_2026_01_06_Key Plan_Professional Office_FFE_FIN.xlsx` Tab `Summary_Key Plans`
  - Infrastructure Key Plan m² = TBD (architect drawings); equipment programs documented in plan

- [ ] **Apply Decision 1–4 to existing DTCG tokens + HTML** `[2026-05-21 totebox@claude-code]`
  - Standardise naming in all existing DTCG entries to Decision 1 convention
  - Delete BIM_TOKENS block from `building-width-calculator.html` (Decision 2)
  - Add stub entries for RS/TI tiles and J/K/L/M placeholders (Decisions 3 + 4)

- [x] **HTML print layout — resolved 2026-05-17** `[2026-05-17 totebox@claude-code]`
  - Root cause: `@page { size: landscape; margin: 0.3in }` + `slide { width: 10.4in }` triple-stacked margins; Chrome silently ignored
  - Fix: `@page { size: 11in 8.5in; margin: 0; }` + `slide { width: 11in; height: 8.5in; transform: none }` in all 3 preview HTMLs
  - PDF generator: `preview/build-pdf.mjs` (Playwright + Chromium); confirmed 792×612pt = 11×8.5in per page
  - Generate: `NODE_PATH=/home/jennifer/sandbox/working/ps-talking-points/node_modules node build-pdf.mjs <file.html>` or `all`
  - Do NOT use the browser print dialog — output varies by operator; use the script

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

- [ ] **Opus army synthesis — 5 operator decisions surfaced** `[2026-05-17 totebox@claude-code]`
  - Source: `.agent/plans/agent-{1,2,3}-*-report.md`
  1. **Academic Small area** — 105 m² (V3 Master Summary, authoritative) vs 87.7 m² in `woodfine-bim-library/tokens/bim/professional-office-subtypes.dtcg.json`. Token file needs update commit.
  2. **Civic zone depths** — still synthesised; no DISCOVERY sketch exists. Field-research pass needed.
  3. **Professional Office Z2/Z3** — V12 carries TBD placeholders (3.0/3.0). Confirm or specify.
  4. **Business Building Width option** — A/A (32.29 m, widest) is currently in HTML; operator may prefer C/C (27.27 m, balanced). Confirm.
  5. **End-cap tile sizing** — tokens say E-1/E-2 = 2,700 SF; V12 Methodology end-cap diagrams show 3,500–5,500 SF. Token file fix needed.

---

## Artifact dispatch status

**→ project-editorial (outbox pending since 2026-05-17):**
- [x] 10 TOPIC drafts (all planned_topics from manifest)
- [x] 5 GUIDE drafts (cluster-totebox-property + gateway-orchestration-bim)

**→ project-design (outbox pending since 2026-05-17):**
- [x] 7 DESIGN-COMPONENT (bim-spatial-tree, properties-panel, viewport-3d, view-navigator, guid-search, audit-log, regulation-rs1)
- [x] 4 DESIGN-RESEARCH (bim-token-taxonomy, asset-woodfine-logo, climate-zone-constraints, mobile-bim-ux)
- [x] 1 DESIGN-TOKEN-CHANGE (design-token-private-office; master-cosigned)

**→ project-design (supplemental dispatch 2026-05-17):**
- [x] design-research-html-print-pdf-pipeline.draft.md (NEW — print/PDF architecture)
- [x] design-index.md (BIM extension review index; accept/refine request)
- [x] design-generic-components-index.md (9 generic patterns flowback)

**→ Command (admin-tier; outbox pending 2026-05-17):**
- [x] woodfine-palette-additions.md (mcorp-administrator; woodfine-media-assets)

**→ project-editorial (supplemental dispatch 2026-05-17 — Opus army synthesis):**
- [x] 11 NEW TOPIC drafts for `content-wiki-projects/topics/bim/`
  - topic-bim-building-width-method + zone-depths-per-use-type (Agent 1)
  - topic-bim-floor-plate-methodology + tile-system + tile-combinations + leasing-efficiencies (Agent 2)
  - topic-bim-key-plans-index + private-office + medical + business + professional-office (Agent 3)
- All structured as living documents (Future research sections for iteration)

---

## Deferred

- [ ] Stage 6 push — 18 commits ahead of origin on `cluster/project-bim` `[2026-05-17 totebox@claude-code]`
