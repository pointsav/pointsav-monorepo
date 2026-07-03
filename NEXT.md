# NEXT.md — project-bim

Open items, deferred work, and follow-up actions.
Attribution format: `[YYYY-MM-DD role@engine]`

---

## Hot — pick up here next session

- [x] **Command: resolve staging-fork anomaly + canonical merge + prod push — DONE 2026-07-03** `[2026-07-03 command@claude-code]`
  - Root cause: `self-service-promote.sh` pushed every self-service archive to the SAME `main` ref on the shared
    personal fork — always failed once a different archive's unrelated history landed there first. Worse,
    `set -e` meant the promote-queue entry silently never got written on push failure. Command fixed both:
    each archive now pushes to its own ref (`BRANCH:CLUSTER_NAME`); queue/notify steps are now unconditional.
  - 23 of 28 local commits landed on canonical (the full shell redesign, reviewed + approved). Verified live
    externally: `https://bim.woodfinegroup.com` → 200, `/healthz` ok, utility bar/theme-toggle/search markup
    all confirmed present in the served HTML.
  - Also fixed a clippy gate issue in the new `render/search.rs`/`render/sidebar.rs` (2x useless `format!()` on
    static strings) since `-D warnings` had never been run on this crate before — worth running `cargo clippy`
    locally on new Rust code going forward rather than relying on Command's promote step to catch it.
  - Prod systemd unit renamed `local-bim-orchestration`/`local-bim` → `local-woodfine-bim` as part of a
    workspace-wide naming reorg (unrelated to this work). Local workspace staging unit unaffected, still `local-bim`.

- [ ] **5 excluded commits need a dedicated reconciliation session** `[2026-07-03 command@claude-code]`
  - 2 `.agent/`-only commits (31403f27, f570b2c6) — correctly never promote, no action needed
  - 3 older tool-keyplan/app-orchestration-bim commits (8ce0b9ba, a4ba3e96, 1608fa26) — conflict heavily
    (30+ hunks) with `app-orchestration-bim/src/main.rs` on canonical, which has evolved independently and
    substantially elsewhere. Remain on local `cluster/project-bim` only. Needs someone with context on both
    sides of the conflict, not a guessed resolution.

- [ ] **Redo "Important Information" disclosure band against Command/project-knowledge's actual pattern** `[2026-07-03 totebox@claude-code]`
  - This session built an ad-hoc `<details>` disclosure band + hardcoded text directly in `shell.rs`, **without having read** inbox message `command-20260702-important-information-footer-structure-a` until shutdown
  - Real spec: disclosure text sourced from a Git-owned markdown file (counsel owns the text, not hardcoded in Rust), a persistent one-line footer disclaimer always visible (not just the collapsible band, "so a collapsed band never screenshots bare"), a dedicated `/disclaimers` page, CC BY-ND attribution to the issuer entity (Woodfine Capital Projects Inc.) for editorial/research content specifically — separate from the Apache-2.0 BIM Object data license — and NI 45-106 forward-looking-statements language mirroring home.woodfinegroup.com
  - If BIM will host research-paper journals: project-knowledge's `SPEC-journal-wiki-render-contract.md` §§9-10 governs the render contract
  - Read the full message before starting — don't re-derive from this summary alone

- [ ] **Search: doesn't index property-set/compliance text inside entity values** `[2026-07-03 totebox@claude-code]`
  - Only title/slug/IFC-class/top-level `$description` are indexed — a query like "fire door" can legitimately return 0 results even though both words exist in the corpus, if no single item's indexed fields contain both
  - Lower priority; noted as a known/accepted scope limit when built, not a bug

- [ ] **Verify corporate wiki instance (port 9095) CSS is fixed** `[2026-07-03 totebox@claude-code]`
  - 4 of 5 stylesheets 404'd during a same-session recheck (`projects`/`documentation` instances on 9093/9090 were fine)
  - Not project-bim's service (`app-mediakit-knowledge-2`, likely project-knowledge's territory) — flagged verbally to operator only, not yet escalated via mailbox

- [x] **app-privategit-bim UI polish — committed 2026-06-20** `[2026-06-20 totebox@claude-code]`
  - Monorepo: 39d3cb0b — shell.rs + card.rs + home.rs + bim-layout.css + bim-components.css
  - Hero/footer/header restored; sidebar always open; cds-data-table → plain table
  - Preview on port 9206 — verified via curl (all checks pass)
  - Brief: `.agent/briefs/BRIEF-app-privategit-bim.md`
  - Stage 6 + production deploy → Command Session outbox (sent this session)

- [x] **tool-keyplan v0.0.1 — committed 2026-05-23** `[2026-05-23 totebox@claude-code]`
  - Monorepo: a4ba3e96 — Cargo.toml + tool-keyplan/ crate (6 files)
  - Engine: ASR A1.2 ✓ European Lighting ✓ Wheelchair ✓ — ALL CONSTRAINTS SATISFIED
  - Brief: `.agent/briefs/BRIEF-tool-keyplan.md`
  - NOTE: interior.dtcg.json + key-plans.dtcg.json were committed to pointsav-design-system in error — see routing items below

- [ ] **ROUTING FIX — apply PO-1 furniture_refs to wbl key-plans.dtcg.json** `[2026-05-28 totebox@claude-code]`
  - `woodfine-bim-library/tokens/bim/key-plans.dtcg.json` PO-1 entry still has old `furniture_program` string array
  - Apply structured `furniture_refs` + `bounding_box_mm` + `circulation_ref` + `compliance` from the pointsav-design-system copy
  - interior.dtcg.json already copied to wbl (2026-05-28) ✓

- [ ] **ROUTING FIX — Command: admin-tier removal of misrouted BIM Objects** `[2026-05-28 totebox@claude-code]`
  - `pointsav-design-system/tokens/bim/interior.dtcg.json` — created in wrong repo; wbl is canonical
  - `pointsav-design-system/tokens/bim/key-plans.dtcg.json` — PO-1 update applied here in error; wbl is canonical
  - Requires Command Session (mcorp-administrator identity to remove from pointsav-design-system)

- [ ] **Deliverable 1b — key-plans-registry.md** `[2026-05-22 totebox@claude-code]`
  - Write `woodfine-bim-library/key-plans/key-plans-registry.md` standalone Markdown
  - All data in `.agent/plans/plan-bim-objects.md` Part 1

- [x] **Key Plans SVG size-specific furniture — shipped 2026-05-22** `[2026-05-22 totebox@claude-code]`
  - `app-orchestration-bim v0.0.3` (commit 8ce0b9ba) — size_tier per category from area_m²
  - PO: 1/2/3 desks; Medical: 2/4/6 chairs + 1/1/2 doc offices; Lab: 3/5/7 bench clusters
  - Business: 3×3/4×4/5×5 workstations + 2/3/5 exec offices + 1/1/2 conf tables
  - Academic: workstation bank → dual banks → theater seats; Civic: 2/4/5 offices + court room (L)
  - Brief: `.agent/briefs/BRIEF-key-plans-site.md`

- [ ] **Corporate Office SVG diagrams** `[2026-05-22 totebox@claude-code]`
  - Currently `_ => {}` — no furniture; zone structure only
  - Blocked on zone depth data for Corporate Office sizes

- [ ] **Deliverable 1b — key-plans-registry.md** `[2026-05-22 totebox@claude-code]`
  - Write `woodfine-bim-library/key-plans/key-plans-registry.md` standalone Markdown
  - All data in `.agent/plans/plan-bim-objects.md` Part 1

- [ ] **Binary ledger — app-orchestration-bim v0.0.3** `[2026-05-22 totebox@claude-code]`
  - Command Session: update `data/binary-ledger/app-orchestration-bim.jsonl`
  - sha256 `/usr/local/bin/app-orchestration-bim` must match ledger entry

- [ ] **WBL key-plans IFC files — 18 uncommitted modifications** `[2026-06-25 totebox@claude-code]`
  - `woodfine-bim-library/key-plans/` has 18 modified `.ifc` files (academic, business, civic, laboratory, medical, private-office × 3 variants each)
  - Detected at startup 2026-06-25; origin unknown — review diff before committing
  - `git -C woodfine-bim-library diff key-plans/` to inspect

- [ ] **NOTAM still permission denied — fix not applied** `[2026-06-22 totebox@claude-code]`
  - `ls -la /srv/foundry/NOTAM.md` shows `-rw------- 1 mathew mathew` as of 2026-06-22 session start
  - inbox `command-20260520-notam-permission-resolved` was inaccurate; flagged to Command via outbox `project-bim-20260622-notam-permission-still-denied`
  - Command: re-apply `chmod 644 /srv/foundry/NOTAM.md`

- [x] **Key Plans foundation — 4 operator decisions received 2026-05-20** `[2026-05-20 totebox@claude-code]`
  - All 4 decisions answered via inbox `command-20260520-bim-foundation-decisions`
  - Decision 1: descriptive display names (Index PDF style); codes (PO-1/M-1/B-1) are internal-only DTCG keys
  - Decision 2: **delete** inline BIM_TOKENS block from `building-width-calculator.html`; fetch from DTCG at render time
  - Decision 3: all 3 building types in scope now (Professional Centre + Retail Select + Tech Industrial + 12 common-area Key Plans)
  - Decision 4: type-prefixed tile codes (CO-A, RS-A, TI-A); Corridor Expander T = 300 SF; arithmetic gaps intentional by design; J/K/L/M as stub DTCG entries with `status: reserved`
  - **Now unblocked:** DTCG token standardisation, HTML BIM_TOKENS removal, Rust crate scaffold

- [x] **Deliverable 1: key-plans-registry.md — done 2026-05-21** `[2026-05-21 totebox@claude-code]`
  - Committed: d1ac026 in woodfine-bim-library (pwoodfine, main)
  - Output: `woodfine-bim-library/key-plans/key-plans-registry.md`
  - Also in `outputs/key-plans-registry.md` — pull via `fpull bim outputs/`

- [x] **Apply Decision 1–4 to existing DTCG tokens + HTML — done 2026-06-22** `[2026-06-22 totebox@claude-code]`
  - D1: descriptive names + type-prefixed codes already in place from prior sessions
  - D2: `building-width-calculator.html` — inline `BIM_TOKENS` removed; async `init()` fetches from `../woodfine-bim-library/tokens/bim/*.dtcg.json` (commits `b7ee3e6e`)
  - D3: `tile_code RS-A/B/C` → `retail-select.dtcg.json`; `TI-A/B/C` → `tech-industrial.dtcg.json`
  - D4: `tile-system.dtcg.json` — Corridor Expander T (300 SF, operative) + J/K/L/M reserved stubs added
  - Sub-clone commit: `05c8c38` (jwoodfine, woodfine-bim-library main); archive commit: `b7ee3e6e` (pwoodfine, cluster/project-bim)
  - HTML requires web server from archive root to serve DTCG fetch: `python3 -m http.server 8100` in project-bim/

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

- [ ] Stage 6 push — commits ahead of origin on `cluster/project-bim` `[2026-06-24 totebox@claude-code, superseded 2026-07-03]`
  - Superseded by the Hot-section item above (staging-fork anomaly on `origin-staging-j`, escalated to Command
    2026-07-03) — that's the current blocker, not a simple backlog of unpushed commits
