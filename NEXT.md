# NEXT.md — project-bim

Open items, deferred work, and follow-up actions.
Attribution format: `[YYYY-MM-DD role@engine]`

---

## Hot — pick up here next session

- [ ] **`pointsav-monorepo` Cargo.lock is stale relative to Cargo.toml workspace members — not this archive's to fix alone** `[2026-07-06 totebox@claude-code]`
  - Working tree has an uncommitted +6035/-1453 line `Cargo.lock` diff. Root cause: the committed `Cargo.lock`
    at current HEAD (`45039f1f`) was last content-updated at old commit `82e10457`, well before many current
    workspace members (`app-console-email`, `app-console-input`, `app-console-keys`, `app-console-people`,
    `app-console-search`, `app-console-slm`, `app-console-system`, `app-mediakit-*`, etc.) existed in
    `Cargo.toml`. A local `cargo build`/`check` regenerated the lockfile to match. This is a canonical-repo
    lockfile-hygiene gap (workspace-wide, not BIM-specific), not something introduced by this archive's work.
  - **Deliberately NOT committed** — a diff this size touching every workspace member's transitive deps risks
    merge noise for every other archive's next Stage 6 promote (project-console/project-knowledge already
    have pending promotes touching `app-console-*` crates per session-context carry-forward). Left in the
    working tree uncommitted; flag to Command/operator for a decision on who should own regenerating and
    committing this (likely belongs with whoever last added workspace members, or a dedicated
    `cargo update --workspace` housekeeping commit reviewed by Command).

- [ ] **Command: admin-tier push for `woodfine-bim-library` `ee089b2`** `[2026-07-04 totebox@claude-code, updated 2026-07-06]`
  - `pointsav-monorepo` `0d72def7` half of this item is DONE — confirmed merged to canonical as `58fa91c0`
    and deployed live (inbox msg `command-20260704-bim-second-rebuild-live`, 2026-07-04). Struck 2026-07-06.
  - `woodfine-bim-library` `ee089b2` (zone-depth data-bug fix, legal text rewrite, home.md content) is
    still committed locally only — its remote is now correctly configured (fixed since last flagged) but has no
    staging mirror, so it still needs Command's admin-tier push directly.
  - See `BRIEF-app-privategit-bim.md`'s 2026-07-04 entry for full detail on the now-fully-resolved
    envelope-as-navigation pass (`dbb74ff8`) and real-object rebuild (`0d72def7`).

- [ ] **woodfine-bim-library has no staging-mirror remotes at all** `[2026-07-04 totebox@claude-code]`
  - Only a single `origin` (woodfine-administrator identity) — every real content/data fix here gets stuck
    waiting on Command's admin-tier push. Flagged twice now (`cleanup-log.md` 2026-05-16; this session).
    Worth Command setting up a proper staging mirror for this repo rather than a recurring one-off unblock.

- [ ] **CC BY-ND 4.0 editorial-content license claim has no backing LICENSE file** `[2026-07-04 totebox@claude-code]`
  - Found while rewriting `disclaimers.md`'s legal text. Neither `woodfine-bim-library`'s `LICENSE` nor its
    `README.md` mention CC BY-ND; the app footer only states Apache-2.0 (data) + AGPL (code). Needs an
    operator decision — add a real instrument backing the claim, or correct it — before treating the new
    disclosure text as fully settled.

- [ ] **`building-width-calculator.dtcg.json` still has unreconciled Private Office zone depths** `[2026-07-04 totebox@claude-code]`
  - A third token file, with yet another set of numbers, different from both the old wrong "confirmed"
    value and the real CAD-sourced one now correct in `key-plans.dtcg.json`. Same bug family as the fix
    made this session — not yet applied here.

- [ ] **"Swiss air requirement" — not found, don't fabricate** `[2026-07-04 totebox@claude-code]`
  - Operator mentioned this; searched all text-based methodology docs this session, no match. May exist in
    a PDF not yet read (check `inputs/Sketches/` more thoroughly), or may not be a real citable fact.

- [ ] **Deferred design work, re-scoped under the "Anatomy of a Key Plan" direction** `[2026-07-04 totebox@claude-code]`
  - Drafting-sheet layout system (sheet numbers, title blocks, cross-sheet references) for category pages.
  - GUID-as-visual-mark (IFC GlobalId as a recurring, owned visual signature).
  - Live constraint-composition tool — pick tokens, watch a real composition recompute. The most
    engineering-heavy of the deferred ideas; a stretch goal, not a first step.

- [ ] **3D-viewport decision gate — needs explicit operator sign-off, not a default** `[2026-07-03/04]`
  - xeokit-sdk (AGPL-3.0, better BIM-viewer completeness + double-precision georeferencing, requires
    open-sourcing the client bundle or a paid Creoox commercial license) vs. `@thatopen/components`
    (MIT/MPL, license-safe, more integration work). Real sample IFC data already exists
    (`woodfine-bim-library`'s key-plan models); no conversion pipeline built yet either way. Same tradeoff
    is already an open question elsewhere in the project (`design-component-bim-viewport-3d.draft.md`'s
    `open_question_1`) — don't resolve it by default here either.

- [x] **5 excluded commits — RESOLVED, no reconciliation session needed** `[2026-07-03 command@claude-code, verified resolved 2026-07-06 totebox@claude-code]`
  - 2 `.agent/`-only commits (31403f27, f570b2c6) — correctly never promote, no action needed.
  - 3 older tool-keyplan/app-orchestration-bim commits (8ce0b9ba, a4ba3e96, 1608fa26) — verified 2026-07-06:
    their content is already present on `origin/main` under different SHAs (`9bb81f99`, `96225980`,
    `add53f64`, `78384aae`, `96485c5d` — diffed identical / equivalent commit messages). Someone (likely
    Command) already did the reconciliation directly on canonical between 2026-07-03 and 2026-07-05. The
    2026-07-05 hard-reset of local `cluster/project-bim` to `origin/main` (which orphaned these 3 SHAs
    locally) was this reconciliation completing, not a bug or data loss. No further action needed.

- [ ] **Search: doesn't index property-set/compliance text inside entity values** `[2026-07-03 totebox@claude-code]`
  - Only title/slug/IFC-class/top-level `$description` are indexed — a query like "fire door" can legitimately return 0 results even though both words exist in the corpus, if no single item's indexed fields contain both
  - Lower priority; noted as a known/accepted scope limit when built, not a bug

- [ ] **Verify corporate wiki instance (port 9095) CSS is fixed** `[2026-07-03 totebox@claude-code]`
  - 4 of 5 stylesheets 404'd during a same-session recheck (`projects`/`documentation` instances on 9093/9090 were fine)
  - Not project-bim's service (`app-mediakit-knowledge-2`, likely project-knowledge's territory) — flagged verbally to operator only, not yet escalated via mailbox

- [ ] **ROUTING FIX — apply PO-1 furniture_refs to wbl key-plans.dtcg.json — STALE PREMISE, needs rescoping** `[2026-05-28 totebox@claude-code, re-investigated 2026-07-03]`
  - `woodfine-bim-library/tokens/bim/key-plans.dtcg.json` PO-1 entry still has old `furniture_program` string array
  - Original plan: apply structured `furniture_refs` + `bounding_box_mm` + `circulation_ref` + `compliance` from "the pointsav-design-system copy" of `key-plans.dtcg.json`
  - **That source file no longer exists anywhere in the `pointsav-design-system` clone** (confirmed via `find`, zero matches). `furniture_refs` only appears in `tool-keyplan`'s Rust generator source, not in any live JSON data file. Needs a real decision on where this data should come from (regenerate via `tool-keyplan`? hand-author?) before touching it.

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
  - `ls -la /srv/foundry/NOTAM.md` shows `-rw------- 1 mathew mathew` as of last check
  - inbox `command-20260520-notam-permission-resolved` was inaccurate; flagged to Command via outbox `project-bim-20260622-notam-permission-still-denied`
  - Command: re-apply `chmod 644 /srv/foundry/NOTAM.md`

- [ ] **Rust crate scaffold** — DTCG files are complete, this is unblocked `[2026-05-17 totebox@claude-code, unblocked 2026-07-03]`
  - 5 crates: `bim-units`, `bim-tokens`, `bim-furniture`, `tool-buildingwidth`, `tool-floorplates`
  - Full architecture in `.agent/plans/tool-buildingwidth-architecture.md`
  - This is a genuinely large undertaking (new Rust workspace, ILP solver via `good_lp`, bidirectional adjustment logic) — deliberately held back pending its own dedicated planning session, not attempted opportunistically

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

(empty — the prior Stage 6 push item is now fully superseded; see the Command canonical-merge item at the top of Hot.)
