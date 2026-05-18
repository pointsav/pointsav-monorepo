# Open items — project-editorial

> Persistent TODO tracker. Updated at session end. Last updated: 2026-05-18.
> Completed items remain here (checked) for one session before archiving to `archive/`.

---

## DONE — BIM editorial sweep (2026-05-17)

- [x] **Batch 1** — 5 BIM architecture TOPICs (EN+ES) → `content-wiki-documentation/architecture/`
  - city-code-as-composable-geometry, open-bim-regulatory-acceptance, asset-anchored-bim-vault,
    aec-interface-conventions, property-manager-bim-gap
  - Committed: `a73723f` (docs wiki) — BIM Objects language pass on 10 TOPIC pairs
- [x] **Batch 2** — 5 BIM GUIDEs → `woodfine-fleet-deployment`
  - guide-deploy-bim-substrate, guide-bim-archive-operations, guide-bim-object-authoring (renamed),
    guide-climate-zone-tokens, guide-regulation-overlay-publishing
  - Files present in woodfine-fleet-deployment (staging)
- [x] **Batch 3** — 6 BIM methodology TOPICs (EN+ES) → `content-wiki-projects` root
  - bim-building-width-method, bim-floor-plate-methodology, bim-floor-plate-tile-combinations,
    bim-zone-depths-per-use-type, bim-key-plans-index, bim-tile-system
  - Committed: `a2c0b78` (projects wiki)
- [x] **Batch 4** — 5 BIM key-plan TOPICs (EN+ES) → `content-wiki-projects` root
  - bim-leasing-plan-efficiencies, bim-private-office-key-plans, bim-medical-key-plans,
    bim-business-key-plans, bim-professional-office-key-plans
  - Committed: `88c0fdf` (projects wiki)
- [x] **Inbox archived** — 4 messages actioned and moved to inbox-archive.md (`24569e2e`)

## DONE — Wiki main page redesign (2026-05-17/18)

- [x] **10-agent Opus synthesis** — research complete; findings documented in session
- [x] **DYK prefix bug fixed** — all 3 wikis: renderer prepends "… that "; facts corrected
- [x] **index.md ledes** — docs: editorial standard link + disclosure note (`17d7750`);
    projects: forward-looking paragraph (`4df475b`);
    corporate: BCSC posture + status active + 5-framework scope (`0c0035b`)
- [x] **reference-invariants.yaml created** — all 3 wikis, content ready for Phase D engine
  - docs: "From the engineering record" (compute boundary / data ownership / audit ledger)
  - projects: "Reference geometry" (102 NA Tier 5 / 0 EU / 7,594 clusters)
  - corporate: "Holding structure" (isolation / no-redemption / 1.2× ICR)
- [x] **home-chrome-v2 design draft staged** — `4a2fbf34`
    at `.agent/drafts-outbound/design-home-chrome-v2.draft.md`

---

## PENDING — Command Session

- [ ] **Stage 6: content-wiki-documentation** — promote staging commits to canonical
- [ ] **Stage 6: content-wiki-projects** — promote staging commits to canonical (includes all BIM batches)
- [ ] **Stage 6: content-wiki-corporate** — promote staging commits to canonical
- [ ] **Stage 6: woodfine-fleet-deployment** — promote BIM GUIDE commits
- [ ] **pointsav-monorepo branch merge** — `readme-fixes-2026-05-16` → main (pending before this session)
- [ ] **project-knowledge services restart** — cargo build --release + restart 3 systemd services after monorepo merge
- [ ] **Admin README fixes** — pointsav-media-assets + woodfine-media-assets (admin-tier commits)

## DONE — home_chrome() Phase D (2026-05-18)

- [x] **short_description added** — `governance/_index.md` + `design-system/_index.md` + ES pairs (`0ed9e12`, Peter)
- [x] **Phase D Rust + JS committed** — `pointsav-monorepo` commit `d929a382` (Jennifer):
  - `ReferenceInvariants` structs + `load_reference_invariants()` + `load_category_descriptions()`
  - "From the doctrine" hardcoded panel → data-driven YAML panel
  - Sister surfaces 10 → 4 per wiki (per-theme branching)
  - Hero search `<form>` in welcome banner
  - Compact category grid (`short_description` cards replacing 8-article preview lists)
  - Cmd-K / Ctrl-K shortcut in `wiki.js`
  - `cargo check` passed ×2 (exit 0)

## PENDING — project-knowledge (Rust / monorepo work)

All Phase D steps ✅ complete. Remaining Rust work deferred to Phase E.

> Phase D design spec: `.agent/drafts-outbound/design-home-chrome-v2.draft.md`

## PENDING — project-editorial (next session)

- [ ] **Check project-bim drafts-outbound** — any new drafts since 2026-05-17 sweep?
- [ ] **Update overhaul-progress.md** — mark BIM batches done; add home redesign milestone
- [ ] **Bilingual home routing** (Phase E) — `index.es.md` exists but `index()` never reads it; deferred to Phase E per design draft

---

## Key file paths

| Path | Role |
|---|---|
| `.agent/drafts-outbound/design-home-chrome-v2.draft.md` | Phase D design spec for home_chrome() |
| `content-wiki-documentation/reference-invariants.yaml` | "From the engineering record" panel data |
| `content-wiki-projects/reference-invariants.yaml` | "Reference geometry" panel data |
| `content-wiki-corporate/reference-invariants.yaml` | "Holding structure" panel data |
| `pointsav-monorepo/app-mediakit-knowledge/src/server.rs:1150` | Hardcoded "From the doctrine" block to replace |
| `pointsav-monorepo/app-mediakit-knowledge/src/server.rs:1271` | Hardcoded sister surfaces to replace |
| `.agent/plans/overhaul-progress.md` | Broader wiki overhaul progress tracker |
