# NEXT.md — project-bim

Open items, deferred work, and follow-up actions.
Attribution format: `[YYYY-MM-DD role@engine]`

---

## Hot — pick up here next session

- [x] **Reposition app-privategit-bim as a BIM Objects CMS, not a wiki clone — Steps 1-4 DONE 2026-07-03** `[2026-07-03 totebox@claude-code]`
  - Operator judged yesterday's wiki-engine-modeled shell redesign (`.bim-utility` bar, in-header search,
    accent-left-border cards, dark-navy-Carbon-adjacent palette) the wrong direction for a product whose job
    is presenting BIM Objects like a design-system catalog presents tokens/components — full research trail
    (7 agents, 3 rounds, live-site audit, git archaeology, competitive resurvey via Fable, synthesis) at
    `.agent/sub-agent-results/RD.1` through `RD.7-visual-direction-synthesis-2026-07-03.md`.
  - Fixed a real bug found along the way: `content::load_categories` only enumerated `.md` sidecars, so the
    4 DTCG files added earlier the same session were invisible (no nav, no card, unreachable by search) —
    flipped to token-file-driven enumeration with sidecar as optional enrichment.
  - Implemented: 4-section IA (Taxonomy/Objects/Compositions/Context, `Section` enum in `content.rs`,
    `section:` frontmatter on all 24 sidecars), sidebar rebuilt as a native-`<details>` section tree, utility
    bar + in-header search deleted, header rebuilt as a single 48px "title block" bar, full color system swap
    to `#1A4480` drafting blue (color-collision-checked against the sibling design.pointsav.com), dark mode
    re-palettized to desaturated instrument-navy, `carbon.min.css`/`carbon.esm.js` scoped to `/edit/*` only.
  - **Deferred, fully specced in RD.7 for a follow-up pass — nothing lost by waiting:** hero isometric-building
    SVG, tab-bar page anatomy, IFC GUID monospace markers, classification chip restyle, dark viewport preview
    frames, section landing pages (2×2 panel grid replacing the flat 24-card homepage grid).
  - **Real gap, not deferred by choice:** Geist Sans / Geist Mono / Source Serif 4 font files don't exist
    anywhere in this workspace and this environment has no way to fetch/subset them — `fonts.css` currently
    uses honest system-font fallback stacks (documented inline) rather than fabricated `@font-face` rules.
    Needs the operator or a session with font-fetch capability to source and self-host actual woff2 subsets
    (OFL 1.1, same vendoring pattern as the removed Oswald/Nunito/Roboto Slab set) before the typography
    fully lands as specced.
  - **Not yet pushed to staging** — verified locally only (`local-bim.service`, port 9096). Run
    `self-service-promote.sh` once operator has reviewed the live preview.
  - PointSav-branded/dedicated-domain positioning (`bim-token-strategy.md`'s fuller recommendation) was
    explicitly deferred as a separate, much larger decision — not touched this pass.

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

- [x] **Redo "Important Information" disclosure band against Command/project-knowledge's actual pattern — DONE 2026-07-03** `[2026-07-03 totebox@claude-code]`
  - Disclosure text now sourced from `woodfine-bim-library/site-content/pages/important-information.md` (Git-owned, request-time read with a safe fallback if missing); persistent one-line footer disclaimer always visible; new `/disclaimers` page (`site-content/pages/disclaimers.md`); real CC BY-ND 4.0 marks (copied from the reference wiki's own assets) + genuine creativecommons.org deed link as a footer badge, separate from the existing Apache-2.0 BIM-data badge
  - Verified live on local preview: band shows real file content (not fallback), `/disclaimers` renders all 4 sections correctly, badges/disclaimer confirmed on both desktop and mobile
  - Found and fixed a real bug while verifying: `content::load_page()` only splits on `## ` headings, so `disclaimers.md`'s initial `# Disclaimers` H1 was silently absorbed into the first section and rendered as a duplicate heading — fixed by dropping the redundant H1 (matches `about.md`'s existing convention)
  - Also fixed 2 clippy warnings Command had already fixed on canonical but which never synced back to this local branch
  - **Not yet pushed to staging/canonical** — commits `68e3406f` (pointsav-monorepo) + `4176389` (woodfine-bim-library) are local-only; run `self-service-promote.sh` next session (should work cleanly now that Command fixed the per-archive-ref bug) or ask Command directly
  - If BIM will host research-paper journals: project-knowledge's `SPEC-journal-wiki-render-contract.md` §§9-10 still governs the render contract — not addressed here

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

- [ ] **ROUTING FIX — apply PO-1 furniture_refs to wbl key-plans.dtcg.json — STALE PREMISE, needs rescoping** `[2026-05-28 totebox@claude-code, re-investigated 2026-07-03]`
  - `woodfine-bim-library/tokens/bim/key-plans.dtcg.json` PO-1 entry still has old `furniture_program` string array
  - Original plan: apply structured `furniture_refs` + `bounding_box_mm` + `circulation_ref` + `compliance` from "the pointsav-design-system copy" of `key-plans.dtcg.json`
  - **2026-07-03: that source file no longer exists anywhere in the `pointsav-design-system` clone** (confirmed via `find`, zero matches). `furniture_refs` only appears in `tool-keyplan`'s Rust generator source (`pointsav-monorepo/tool-keyplan/src/main.rs`), not in any live JSON data file — likely because the admin-tier removal item below already ran. Don't attempt a data copy against a source that isn't there; needs a real decision on where this data should actually come from (regenerate via `tool-keyplan`? hand-author?) before touching it.

- [x] **ROUTING FIX — Command: admin-tier removal of misrouted BIM Objects — CONFIRMED DONE (found stale) 2026-07-03** `[2026-05-28 totebox@claude-code]`
  - Both `pointsav-design-system/tokens/bim/interior.dtcg.json` and `.../key-plans.dtcg.json` confirmed absent from that repo as of 2026-07-03 (`find` returns zero matches for either) — the admin-tier removal this item asked for has already happened, just never marked done here
  - This is also *why* the PO-1 furniture_refs item above lost its data source

- [x] **Key Plans SVG size-specific furniture — shipped 2026-05-22** `[2026-05-22 totebox@claude-code]`
  - `app-orchestration-bim v0.0.3` (commit 8ce0b9ba) — size_tier per category from area_m²
  - PO: 1/2/3 desks; Medical: 2/4/6 chairs + 1/1/2 doc offices; Lab: 3/5/7 bench clusters
  - Business: 3×3/4×4/5×5 workstations + 2/3/5 exec offices + 1/1/2 conf tables
  - Academic: workstation bank → dual banks → theater seats; Civic: 2/4/5 offices + court room (L)
  - Brief: `.agent/briefs/BRIEF-key-plans-site.md`

- [ ] **Corporate Office SVG diagrams** `[2026-05-22 totebox@claude-code]`
  - Currently `_ => {}` — no furniture; zone structure only
  - Blocked on zone depth data for Corporate Office sizes

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

- [x] **DTCG token files — 6 missing files created — DONE 2026-07-03** `[2026-05-17 totebox@claude-code, completed 2026-07-03]`
  - Full spec was `.agent/plans/tool-buildingwidth-architecture.md`; commit `ae153aa` (woodfine-bim-library, main)
  - All 6 delivered: `furniture.dtcg.json`, `floor-plate-assembly-rules.dtcg.json`, `building-grid.dtcg.json`, `tenant-mix.dtcg.json` (new), Medium Tile family + Special Tiles added to `tile-system.dtcg.json`
  - All 6 internal inconsistencies also fixed (5 resolved with real data; #3 professional-office medium/large marked `status: reserved` — no source document located, not fabricated)
  - Verified against the live app: `local-bim.service` restarts healthy, `token_count` 80→102, all new files present via `/api/tokens.json`, search indexes the new entities
  - **Two genuine data gaps found and flagged in the token data itself** (not silently resolved): the Medium-family end-cap tiles (E-1/E-2 Medium) have no sourced composition (`composition_status: not-sourced`); Tile F-medium's stated "3× PO Small + PO Medium" composition sums to 1,440 SF against a 3,500 SF target — a 2,060 SF gap (`composition_status: arithmetic-does-not-reconcile`). Needs a source document or operator decision, not a guess.
  - **Not done**: dedicated `site-content/categories/*.md` pages for the 4 new token files — they're loaded/valid/searchable but won't get their own sidebar card without a category page (distinct UI-work scope from token authoring)

- [ ] **Rust crate scaffold** — DTCG files are now complete, this is unblocked `[2026-05-17 totebox@claude-code, unblocked 2026-07-03]`
  - 5 crates: `bim-units`, `bim-tokens`, `bim-furniture`, `tool-buildingwidth`, `tool-floorplates`
  - Full architecture in `.agent/plans/tool-buildingwidth-architecture.md`
  - This is a genuinely large undertaking (new Rust workspace, ILP solver via `good_lp`, bidirectional adjustment logic) — was explicitly held back from the 2026-07-03 DTCG-authoring pass pending its own dedicated planning session, not attempted opportunistically

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
