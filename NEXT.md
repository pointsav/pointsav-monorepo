# NEXT.md — project-knowledge (app-mediakit-knowledge)

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-03

---

## Archetype model rework — 2026-06-03 (DEPLOYED LIVE)
## Content sync protocol — standing session-start procedure

**Root cause identified 2026-06-02:** project-editorial commits content to its local
media-knowledge-* clones but does not push to GitHub. project-knowledge's live services
read from project-knowledge's content-wiki-* clones, which fall behind silently.

**Standing rule — start of every project-knowledge session:**
```bash
git -C /srv/foundry/clones/project-knowledge/content-wiki-documentation pull --no-rebase /srv/foundry/clones/project-editorial/media-knowledge-documentation main
git -C /srv/foundry/clones/project-knowledge/content-wiki-projects pull /srv/foundry/clones/project-editorial/media-knowledge-projects main
git -C /srv/foundry/clones/project-knowledge/content-wiki-corporate pull /srv/foundry/clones/project-editorial/media-knowledge-corporate main
```

Run before any content work. If any pull reports "Already up to date", skip it.
If diverged branches require `--no-rebase`, use it (creates a merge commit — acceptable).

Note sent to project-editorial outbox (msg-id: project-knowledge-20260602-editorial-push-protocol)
requesting they push to GitHub at session end so both routes work.

---

## Content audit — 2026-06-01 batch (remaining items)

- [x] **Commuter overnight ingest complete; PKS then redefined.** The commuter/metro ingest
      finished; PKS was then **redefined as a geometric airport-led park-and-ride model**
      (`build-pks-clusters.py`, commit `aec2187e`). Candidate = sized regional airport
      (park-and-fly) OR outer commuter-rail-belt station (15–110 km ring, connected toward core,
      ≤4 stops from line end). Airports lead → fixed map spread (NA cell coverage 96 → 957).
      **5,977 features** live. [2026-06-03 totebox@claude-code]
- [x] **Urban Fringe → Retail-density model.** `qualify_vwh()` admits ≥2-category co-locations OR
      any lone STRONG/BROAD trade-supply store; composition-score `tier_vwh(cats,n)`.
      **7,028 features** live. Both archetypes now ≈ Retail bubble density (~6,500). [2026-06-03]
- [x] **Mobile BentoBox footbar hardening + cache-busting.** visualViewport detent heights +
      resize re-snap, `overscroll-behavior: contain`, modal `dvh`; `?v=` cache token on archetype
      data URLs (current `20260603d`). Verified via `tools/shoot.mjs` browser-in-the-loop. [2026-06-03]

- [ ] **June 4 05:00 UTC overnight ingest — SCHEDULED (crontab).** `run-overnight-ingests.sh`:
      parking layer (`ingest-osm-parking.py`) + parcel depots (`ingest-osm-parcel-depot.py`) +
      20 new VWH brand chains (builders' merchants, self-storage, trade counters via `ingest-osm.py`).
      Brand YAMLs in the local-only deployment data dir. Log: `overnight-ingests.log`. [2026-06-03]
- [ ] **After June 4 — wire the parkade GREENFIELD filter into `build-pks-clusters.py`.** Join
      `cleansed-civic-parking.jsonl`: label each candidate BUILT (≤800 m of a multi-storey/garage
      → exclude/down-weight) / PARTIAL (surface park_ride) / GREENFIELD (nothing). The "no parkade
      yet" filter that ranks park-and-ride opportunity. [2026-06-03]
- [ ] **After June 4 — wire new categories into `VWH_CHAINS`.** Add `builders_merchant`,
      `self_storage`, `trade_counter`, `parcel_depot` (parcel via the `parcel-depot-osm`
      pseudo-chain) once the ingest data lands. [2026-06-03]
- [ ] **Cache-token convention.** Bump `?v=` in `index.html` on EVERY archetype-data rebuild +
      redeploy (browser caches geojson as fresh; stale-cache trap otherwise). [2026-06-03]
- [ ] **Tier-refinement / single-store density.** VWH T3 is now mostly single-store fringe markers
      (3,549 of 7,028). Revisit once the June 4 categories add genuine co-locations — may tighten
      back toward co-location-only. [2026-06-03]

- [ ] **Stage 6** — promote this session's archive-local commits (`aec2187e` + the docs commit) to
      canonical monorepo. Command Session runs `bin/promote.sh`. [2026-06-03]
### Cluster 1 — stray-backslash links ~~DONE 2026-06-02~~

- [x] `systems/mediakit-os` → `[[app-mediakit-marketing\]]` — fixed 2026-06-02
- [x] `systems/mediakit-os` → `[[app-mediakit-knowledge\]]` — fixed 2026-06-02
- [x] `systems/os-family-overview` → 8 backslash links — fixed 2026-06-02

### Cluster 2 — genuinely-missing targets

- [x] `contribute` → `[[all-pages]]` (×2) + `[[wanted]]` (×2) — links removed 2026-06-02
- [x] `reference/bim-market-context` → `[[os-totebox]]` — stub created 2026-06-02
- [ ] `architecture/gis-as-bim-substrate` → `[[regional-name-resolution-architecture]]` — write page (B13 dispatch would create this)
- [ ] `substrate/substrate-without-inference-base-case` → `[[service-fs]]` — write page or correct slug
- [ ] `media-knowledge-documentation/guide-keep-the-home-page-the-gold-standard` → `[[topic-knowledge-wiki-home-page-design]]` — write page

### Missing slug / type mismatch ~~DONE 2026-06-02~~

- [x] All 6 flagged guide files in pointsav-fleet-deployment — `type: guide` + `slug:` added 2026-06-02

---

## VWH data quality — iso_country_code mismatches — FIXED 2026-06-03

- [x] **Coordinate-based ISO assignment** — `ingest-osm.py` now resolves `iso_country_code` from
      the node's lat/lon via Shapely point-in-polygon for multi-country chains (instead of
      defaulting to the YAML `country:` field). Re-ran wurth-de, rexel-fr, loxam-fr, kiloutou-fr.
      Würth now correctly DE 411 / FI 72 / IT 59 / PL 52 / AT 44 / NL 29 / PT 27 … (was DE 847).
      VWH per-country counts corrected (IT/FI/PL/AT/NL/PT all rose). [2026-06-03]
- [x] **A19 TOPIC: Vertical Warehouse** — committed to archetypes/ 2026-06-02 (commit 9fb4e0e, Peter); live at documentation.pointsav.com/archetypes/vertical-warehouse
- [x] **A20 TOPIC: Parking Structures** — committed to archetypes/ 2026-06-02 (commit 9fb4e0e, Peter); live at documentation.pointsav.com/archetypes/parking-structures
- [x] **A21 GUIDE: Location Intelligence Data Collection** — routed to Command via outbox 2026-06-02 (msg-id: project-knowledge-20260602-guide-location-intelligence-route)

---

## Urban Fringe chain expansion — 2026-06-03 session

Independent strict co-location model (≥2 distinct categories, no metro gate, no hypermarket
disqualifier) — `build-vwh-clusters.py`. VWH total was **3,520** under the strict model;
**superseded 2026-06-03** by the Retail-density model (7,028; admits lone STRONG/BROAD stores).
Country chain additions (still in effect):

- [x] **CA/MX** — RONA, Home Hardware (469), Fastenal CA, United Rentals CA, PartSource, Truper MX,
      Enterprise CA, Hertz MX. CA 32→170, MX→72.
- [x] **ES/IT** — Norauto + Feu Vert (auto parts), Bauhaus/Brico Dépôt/AKI (108)/Bricomart/Bricoman
      ES hardware, Bricocenter/Brico io (72)/Bricoman IT hardware, Rexel/Sonepar electrical,
      Loxam/Kiloutou tool rental. **Spain 6→69, Italy 11→44.**
- [ ] **`bricomart-es` returns 0 in OSM** — brand:wikidata + name query both empty; ~55 stores
      exist but are tagged differently (possibly under Leroy Merlin brand). Re-investigate tagging
      or skip. Wired but contributes nothing. [2026-06-03]
- [ ] **ES/IT B2B electrical/tool chains thinly mapped** — Loxam/Kiloutou/Rexel/Sonepar ES/IT
      yielded only ~20 usable records (trade counters under-mapped in OSM). Low-priority backlog.
      [2026-06-03]

---

## AEC builds — STILL FAILING — do NOT auto-run overnight

- [ ] **Seismic + flood builds fail** (see JOURNAL J3 block below). Excluded from the overnight
      run by design — a broken build must not run unattended. Daytime fix needed in
      `build-aec-seismic.sh` / `build-aec-flood.sh` before scheduling. [2026-06-03]

---

## Pending Command Session

- [ ] Provision DNS A-record + nginx vhost for documentation.pointsav.com (Q3 confirmed 2026-06-04) [2026-06-04 totebox@claude-code]
- [ ] Stage 6 promotion: promote night-build + auth-removal commits to canonical [2026-06-04 totebox@claude-code]
- [ ] Route DESIGN-TOKEN-CHANGE-knowledge-platform-theming.draft.md to project-design for master_cosign [2026-06-04 totebox@claude-code]

## Font / zoom fix — 2026-06-02

- [x] **"50% zoom" on all devices fixed** — root causes: (1) `font-size: 14px` base (vs web standard 16px), (2) `font-size: 11px` nav text unusually small, (3) `.doc { margin: 0 56px }` inside `max-width:1440px` means on monitors wider than ~2000px the white content card occupies ~50% of screen width. Fix: 14px→16px base, 11px nav→13px, 11px tabs→12px, wide-screen @media rules added. Applied to both deployed bundles 2026-06-02. [2026-06-02 totebox@claude-code]
- [x] **`scripts/fix-font-zoom.sh` created** — idempotent patch script re-applies all font/zoom fixes after any bundle rebuild. Run `bash scripts/fix-font-zoom.sh` after `fix-viewport.sh`. [2026-06-02 totebox@claude-code]

## Live-site edit workflow fix — 2026-06-02

- [x] **Root cause identified** — git commits do NOT update the live sites. The binary reads
  from gitignored deployment dirs. Changes made to the monorepo were never reflected live because
  no step copies from monorepo → deployment dirs. [2026-06-02 totebox@claude-code]
- [x] **`scripts/verify-live.sh` created** — run at session start and end; shows file size,
  mtime, `<title>`, viewport patch state, and HTTP 200 health for both tenants. [2026-06-02 totebox@claude-code]
- [x] **`scripts/edit-live-content.sh` created** — single entry point for live edits; opens
  the correct deployment HTML in `$EDITOR`, re-applies fix-viewport.sh, and confirms the change
  landed. Usage: `bash scripts/edit-live-content.sh woodfine|pointsav` [2026-06-02 totebox@claude-code]
- [x] **CLAUDE.md updated** — "CRITICAL" section at top with workflow and `verify-live.sh` in
  session start checklist. [2026-06-02 totebox@claude-code]

## Viewport / zoom fix — 2026-06-02

- [x] **iOS Safari 30% zoom** — `documentElement.replaceWith()` dropped viewport-meta re-evaluation; iOS fell back to 980px desktop width → `max-width:1440px` layout at ~27% on 390px screens. Fix: swap `<head>` content and `<body>` separately (never replace `documentElement`). Applied to both deployed bundles 2026-06-02. Verified: `body_w == viewport_w` at 375/768/1280/1440px on both tenants.
- [x] **Regen guard** — `scripts/fix-viewport.sh` created 2026-06-02; idempotent patch script re-applies the body-only swap to both deployment `index.html` files after any bundle rebuild. Run `bash scripts/fix-viewport.sh` before restarting services. Detects already-patched files safely. [2026-06-02 totebox@claude-code]
- [x] **Monitoring** — daily remote agent `trig_01P7iwnuwpPShgaivbg4m2gq` created 2026-06-02; fires 07:00 UTC; checks HTTP 200 + viewport meta + no replaceWith regression on both tenants. Dashboard: https://claude.ai/code/routines/trig_01P7iwnuwpPShgaivbg4m2gq [2026-06-02 totebox@claude-code]
- [x] **J1 v0.5 language pass** — COMPLETE 2026-06-02: §5.4 new section scanned clean; 9× "thirteen"→"eighteen" country count fixes; §3.7 trim ~300w; `forbidden_terms_cleared: true`; JOURNAL/JOURNAL-retail-colocation-v0.5.draft.md committed `1a30310f`. [2026-06-02 totebox@project-editorial]
- [ ] **J1 §7.2 primary spec** — `log(catchment_entropy) ~ tier + log(pop_150km) + C(country)` — blocked on Phase 24B (Kontur population join to clusters-ols.csv + O-D data join). project-gis owns. Outbox sent `project-editorial-20260528-j1-j3-return`. [2026-05-28 totebox@project-editorial]
- [ ] **J1 permutation test** — `sim-tier-permutation.py` needs writing (10,000 shuffles, one-tailed p-value, cluster coords at project-gis `work/clusters-ols.csv`). project-gis owns. [2026-05-28 totebox@project-editorial]
- [ ] **J2 Bench #9 re-run** — `verify_inclusion_proof` 1024-leaf; ±11% CI → <5% CI; quiet GCP n2 host. project-system owns. Outbox sent `project-editorial-20260528-j2-return`. [2026-05-28 totebox@project-editorial]
- [ ] **J2 citation placeholders** — 9 `[external: ...]` stubs need stable IDs in `citations.yaml`. project-system owns. [2026-05-28 totebox@project-editorial]
- [ ] **J3 §6 Results** — AEC nightly build coverage metrics (4 scripts × N countries). project-gis owns. Outbox sent `project-editorial-20260528-j3-coverage-metrics` + `project-editorial-20260528-j1-j3-return`. [2026-05-28 totebox@project-editorial]
- [x] **J4 §4–§5 benchmarks** — COMPLETE 2026-05-29: §4 Implementation (WireGuard config, BLAKE2s audit daemon) + §5 Evaluation (5 benchmarks, Table 1 vs Mackey et al.) fully written; `forbidden_terms_cleared: true`; commit `77063dc3`. [2026-05-29 totebox@project-editorial]
- [x] **J4 two unresolved citations** — RESOLVED 2026-05-29: Birge-Lee et al. 2024 replaces Cameron placeholder; Mackey et al. 2020 (DOI:10.1145/3374664.3379532) replaces ZTA [CITATION NEEDED]. [2026-05-29 totebox@project-editorial]
- [ ] **J4 word count gap** — current ~6,400 words vs 9,000-word target. ~2,600 words needed in §4–§5 expansion. project-infrastructure scope. [2026-05-29 totebox@project-editorial]
- [x] **J4 final language pass** — COMPLETE 2026-05-31: §4+§5 scanned clean; no forbidden terms found; `forbidden_terms_cleared: true` confirmed accurate; stale notes_for_editor warning removed. [2026-05-31 totebox@project-editorial]
- [ ] **J5 full writing pass** — HOLD until J2 submitted. project-orchestration owns. Outbox sent `project-editorial-20260528-j5-return`. [2026-05-28 totebox@project-editorial]
- [ ] **J6 §6 Results** — user study execution (§5 protocol in JOURNAL file). project-bim owns. Outbox sent `project-editorial-20260528-j6-return`. [2026-05-28 totebox@project-editorial]

## JOURNAL programme — operator actions (all papers)

- [ ] **ORCID IDs** for Jennifer M. Woodfine, Peter M. Woodfine, Mathew Woodfine — required before any submission; not urgent — no paper is submission-ready; all blocked on data. [2026-05-30 totebox@project-editorial]
- [x] **J1 bilingual ES sibling** — COMPLETE 2026-05-31: `JOURNAL/JOURNAL-retail-colocation-v0.1.es.draft.md` written; all 22 sections in Spanish academic register; ~8,500 words; `forbidden_terms_cleared: true`. [2026-05-31 totebox@project-editorial]

## Inbox — pending pickup

- [x] `command-20260530-infrastructure-sessions2-7-topic-relay` (INF-A) — ACTIONED 2026-05-30: 11 bilingual TOPIC pairs committed to media-knowledge-documentation `277847a`; sovereign-mesh IP fix applied. [2026-05-30 totebox@project-editorial]
- [x] `command-20260530-infrastructure-sessions6-7-editorial` (INF-B) — ACTIONED 2026-05-30: same TOPIC pairs (overlapping coverage with INF-A); 4 GUIDEs staged `955d6f34` + routed to Command; PROSE-RESEARCH review returned to project-infrastructure outbox. [2026-05-30 totebox@project-editorial]
- [x] `command-20260530-infrastructure-session12-editorial` (INF-C) — ACTIONED 2026-05-30 (folded into INF-B action): vm-architecture + os-infrastructure-ppn-node bilingual committed `277847a`; guide-vm-infrastructure-resource-pool staged `955d6f34`. [2026-05-30 totebox@project-editorial]
- [x] GIS-2/GIS-3/GIS-4 (project-gis outbox) — ACTIONED 2026-05-30: 12 bilingual TOPICs committed to media-knowledge-projects `294488f` (prior session); GUIDE A14 test-market refs updated + staged; A13 DESIGN routed to project-design; ack sent. [2026-05-30 totebox@project-editorial]
- [x] `command-20260529-intelligence-guides-relay` — ACTIONED 2026-05-29: guide-post-commit-training-hook + guide-goose-local-doorman staged (`72761f65`); routed to Command via outbox `project-editorial-20260529-intelligence-guides-routing`. [2026-05-29 totebox@project-editorial]
- [x] `command-20260529-infrastructure-editorial-relay` — ACTIONED 2026-05-29: topic-os-mediakit bilingual committed to wiki (`81ca9aa`); guide-vm-mediakit-provision + guide-vm-mediakit-service-migration staged (`0d9da8ed`); J4 v0.4 canonical updated (`77063dc3`); vm-mediakit GUIDEs routed to Command via outbox `project-editorial-20260529-infrastructure-guides-routing`. [2026-05-29 totebox@project-editorial]
- [x] `command-20260529-system-phase1c-v2-relay` — ACTIONED 2026-05-29: moonshot-toolkit-build-orchestrator + sel4-aarch64-qemu-substrate-target bilingual committed to wiki (`95f6beb`); guide-moonshot-toolkit-phase1c-build-setup staged (`fbde41fa`); GUIDE routed to Command via outbox `project-editorial-20260529-system-guide-routing`. [2026-05-29 totebox@project-editorial]
- [x] `command-20260526-dev-phase3-drafts-relay` — ACTIONED 2026-05-28: TOPIC committed to `media-knowledge-documentation/applications/app-privategit-workbench.md` + ES stub; GUIDE staged + routed to Command via outbox `project-editorial-20260528-guide-workbench-routing`.

## Session shutdown — push media-knowledge-* to GitHub

Per project-knowledge request (msg-id: project-knowledge-20260602-editorial-push-protocol):
After any content commit session, push all three knowledge wiki repos to GitHub so
project-knowledge's live services can sync (their workaround is a direct local pull,
which works only while project-editorial's clone is at its current path):

```bash
git -C /srv/foundry/clones/project-editorial/media-knowledge-documentation push origin-staging-j main
git -C /srv/foundry/clones/project-editorial/media-knowledge-projects push origin-staging-j main
git -C /srv/foundry/clones/project-editorial/media-knowledge-corporate push origin-staging-j main
```

Add this to the shutdown checklist before the commit step. [2026-06-02 totebox@project-editorial]

## Backlog drift — registry items needing source-project action

- [x] **B13 Regional Name Resolution TOPIC** — COMMITTED 2026-06-02: `architecture/regional-name-resolution.md` + `.es.md` — commit `29c35f3`. [2026-06-02 totebox@project-editorial]
- [x] **B14 Co-location Tier Nomenclature TOPIC** — COMMITTED 2026-06-02: `architecture/colocation-tier-nomenclature.md` + `.es.md` — commit `29c35f3`. [2026-06-02 totebox@project-editorial]
- [x] **B15 GIS as BIM Substrate TOPIC** — already committed in prior session; dead link fixed 2026-06-02 (`regional-name-resolution-architecture` → `regional-name-resolution`) — commit `612aa03`. [2026-06-02 totebox@project-editorial]
- [x] **B16 UK/EU Food Retail Coverage TOPIC** — COMMITTED 2026-06-02: `reference/uk-eu-food-retail-coverage.md` + `.es.md` — commit `29c35f3`. [2026-06-02 totebox@project-editorial]

## drafts-outbound — unresolved groups (cleanup 2026-05-31)

- [x] **Group 3 — 10 unregistered guides** — FULLY ACTIONED: 3 NEW guides placed by Command WFD `7e77081` (cluster-intelligence/guide-activate-anthropic-shim, guide-local-circuit-tier-a-only, guide-proofreader-distillation); 6 guides not re-placed (canonical already more refined — see ack msg-id: command-20260531-editorial-group3-routing-ack); all 6 source drafts already archived in `0b5814a1`. [2026-05-31 totebox@project-editorial]
- [x] **Group 4 — LICENSE artifacts** — FULLY ACTIONED 2026-05-31: LICENSE-DATA-MANIFEST.refined.md + LICENSE-DISCLAIMER.refined.md confirmed placed in gateway-orchestration-gis/ (WFD `7e77081`); refined copies archived from drafts-outbound. [2026-05-31 totebox@project-editorial]
- [x] **Group 5 — 3 unregistered TOPICs** — ACTIONED 2026-05-31: topic-co-location-intelligence-overview.draft.md confirmed superseded (already committed to media-knowledge-projects, 5 edit passes since 2026-05-02 authoring); topic-customer-tier-catalog-pattern.md + topic-radical-proofreader-ui.md archived as stale project-proofreader skeletons (source project must resubmit if still relevant). All 3 archived. [2026-05-31 totebox@project-editorial]
- [ ] **Group 1 — 15 files actively pending Command routing** — CARRY FORWARD: 2 COMMS-bencal (+ 2 renderings), 2 RESEARCH-bencal, 5 infrastructure GUIDEs (A8/A9/A10/A11/A24 batch), GUIDE-workbench-setup, GUIDE-regional-market-topic-production (A21), guide-moonshot-toolkit-phase1c-build-setup (A14); LICENSE refined files now archived (Group 4 closed). Outbox messages sent; Command action required.
- [ ] **guide-proofreader-distillation** — routing decision pending Command (msg-id: project-editorial-20260531-guides-proofreader-routing-flag). [2026-05-31 totebox@project-editorial]

## Convention layer — pending (from earlier session)

- [ ] `conventions/artifact-classification.yaml` — add JOURNAL entry (schema, gateway, destinations, bilingual_pair: false). [2026-05-27 totebox@project-editorial]
- [ ] `conventions/journal-artifact-discipline.md` — new file; copy/adapt from `.agent/rules/journal-artifact-discipline.md`. Command Session scope. [2026-05-27 totebox@project-editorial]
- [ ] `conventions/artifact-registry.md` — add JOURNAL section row. Command Session scope. [2026-05-27 totebox@project-editorial]
- [ ] Foundry `NEXT.md` — add JOURNAL programme tracking checkbox. Command Session scope. [2026-05-27 totebox@project-editorial]

## Command-scope items surfaced this session

- [x] **A4 text-gis-data-methodology-dialog** — PLACED 2026-06-01: WFD commit `8d412a6` per Command ack (project-editorial-20260601-compound-reply-followup). [2026-05-31 totebox@project-editorial]
- [ ] **Legal tokens** — routing message sent (msg-id: project-editorial-20260531-legal-tokens-route); Command must commit legal-tokens-pointsav.yaml + legal-tokens-woodfine.yaml to factory-release-engineering/tokens/ via admin-tier. [2026-05-31 totebox@project-editorial]
- [x] **from-project-system READMEs** — CLOSED 2026-06-01: canonical already at v1.0.0 (62 tests); drafts were v0.2.0 (51 tests, refined 2026-05-22); all 6 draft files archived; routing request withdrawn. [2026-06-01 totebox@project-editorial]
- [x] **GUIDE v0.2 local-circuit-tier-a-only** — PLACED 2026-06-01: `cluster-intelligence/guide-local-circuit-tier-a-only.md` WFD commit `35a2341` pushed to GitHub per Command ack. [2026-06-01 totebox@project-editorial]
- [x] **Directive §D/§E/§F (knowledge-platform doc alignment)** — FULLY ACTIONED 2026-06-01:
  - §D governance docs committed (naming-convention.md + content-contract.md + leapfrog mobile-first + contribute slug discipline) — `de4e611` (Jennifer)
  - §D alias pass: 30 files in media-knowledge-corporate — `ac7cccb` (Jennifer); 86 files in media-knowledge-projects — `476ebb2` (Peter)
  - §E mobile-first: 8 design-system files — `42e7f57` (Peter)
  - §F combined GUIDE staged to drafts-outbound; routing message sent (msg-id: project-editorial-20260601-guide-knowledge-wiki-deployment-route)
  [2026-06-01 totebox@project-editorial]
- [ ] **§F GUIDE routing** — guide-knowledge-wiki-deployment.draft.md staged; Command must place in `woodfine-fleet-deployment/gateway-knowledge-documentation-1/`. [2026-06-01 totebox@project-editorial]
- [ ] **Binary deploy — UX batch** — 7 commits on canonical; run from Command workspace:
      `~/Foundry/bin/deploy-binary.sh app-mediakit-knowledge --note "UX batch: Phase 5/4/M1/Phase 2/Phase 3/wikilink-parser/check (39f4dcd1)"`
      Services: local-knowledge-documentation (9090), local-knowledge-projects (9093), local-knowledge-corporate (9095)
      [2026-06-01 totebox@claude-code]
- [ ] **GUIDE placement** — `GUIDE-location-intelligence-data-collection.draft.md` from project-gis drafts-outbound
      → `woodfine-fleet-deployment/gateway-orchestration-gis-1/guide-location-intelligence-data-collection.md`
      Blocked by pretool-scope-check.sh hook; requires admin-tier commit from Command
      [2026-06-02 totebox@claude-code]

## Pending project-editorial

- [ ] **Content audit triage** — 17 dead wikilinks + 6 missing-slug guides in documentation corpus
      Report: `.agent/drafts-outbound/CONTENT-AUDIT-dead-links-2026-06-01.md`
      Missing targets: `[[os-totebox]]`, `[[regional-name-resolution-architecture]]`,
      `[[topic-knowledge-wiki-home-page-design]]`; stray-backslash links in 2 files
      [2026-06-01 totebox@claude-code]
- [ ] **`check --strict`** CI gate — wire as pre-promote gate once dead-link count reaches 0
      [2026-06-01 totebox@claude-code]

## Pending project-design

- [ ] **DESIGN-wiki-institutional-redesign** — staged in `.agent/drafts-outbound/DESIGN-wiki-institutional-redesign.draft.md`; **`master_cosign:` required** before token changes committed (color #0E3A66, body 18px, nav 14px) [2026-06-03 totebox@claude-code]
- [ ] **DESIGN-docs-sidenav-component** — staged in drafts-outbound; routed via outbox
      [2026-06-01 totebox@claude-code]
- [ ] **DESIGN-doc-header-component** — staged in drafts-outbound; routed via outbox
      [2026-06-01 totebox@claude-code]

## Institutional UX — P0 implementation (project-knowledge owns)

From Opus browser audit 2026-06-03. Full brief: `.agent/drafts-outbound/DESIGN-wiki-institutional-redesign.draft.md`

- [ ] **Build-time link checker** — add internal href validation to Rust render binary; fail build on any 404 in chrome or featured slots. Catches corporate slug bugs + projects footer 404s in CI [2026-06-03 totebox@claude-code]
- [ ] **projects.woodfinegroup.com vendor brand leak** — nav/login/footer still leaks pointsav.com nav links + /wiki/pointsav-media-kit footer link; feeds.rs "PointSav Knowledge" DONE (86db62e9); server.rs nav/footer/wordmark fix is a separate large scope item [2026-06-03 totebox@claude-code]
- [x] **Author Disclaimer + Contact pages for projects.woodfinegroup.com** — DONE 1cd2644 (4 files: contact.md + .es.md + disclaimers.md + .es.md); footer links now resolve [2026-06-03 totebox@claude-code]
- [x] **Category empty state redesign** — DONE 86db62e9; `wiki-empty-state` div replaces bare `p.wiki-cat-page-empty`; CSS added; takes effect on next binary deploy [2026-06-03 totebox@claude-code]
- [x] **corporate.woodfinegroup.com featured-article slug fix** — already fixed in commit 326d6e2 (2026-05-17); confirmed by content audit [2026-06-03 totebox@claude-code]

## Code fix needed

- [ ] **Doorman stub routes missing** — `tests/doorman_test.rs` expects `POST /api/doorman/complete`
      and `POST /api/doorman/instruct` to return 501; both return 404 because the routes don't
      exist in server.rs. Add two minimal stub handlers returning `StatusCode::NOT_IMPLEMENTED`.
      Pre-existing gap; not a regression from UX batch. [2026-06-02 totebox@claude-code]
- [ ] **Navigation portlet test failing** — `server::tests::wiki_page_renders_navigation_portlet`
      asserts `html.contains("sidebar")` but article pages use `docs-sidenav` class (no "sidebar"
      string unless TOC rail is present). Test was written for a planned Wikipedia-parity portlet
      feature not yet implemented. Pre-existing gap. [2026-06-03 totebox@claude-code]

## Standing deferred

- Phase 12 (AI marginalia): gated on BP5 + SYS-ADR-07/10/19 review [2026-06-01]
- Phase 5.1+ (ACLs/OIDC/webhooks): gated on BP5 clearance [2026-06-01]
- Phase 6 (three-instance deployment split): gated on GitHub renames + Doctrine amendment [2026-06-01]
- claim-rail nightly URL validator: gated on server infrastructure [2026-06-01]
# NEXT.md — project-bim

> **Scope: this archive only.** Cross-repo and workspace-level items live
> at `~/Foundry/NEXT.md`.

Last updated: 2026-06-03 [totebox@claude-code]

---

## Immediate — needs operator action

- [ ] Install nightly BIM pipeline timer [2026-06-03 totebox@claude-code]
  ```bash
  sudo cp infrastructure/systemd/foundry-bim-furniture.{service,timer} /etc/systemd/system/
  sudo systemctl daemon-reload
  sudo systemctl enable --now foundry-bim-furniture.timer
  ```
- [x] Apply SEO head block to `home.pointsav.com` + `home.woodfinegroup.com` — done 2026-05-24 [totebox@claude-code]
- [x] Add `robots.txt` + `sitemap.xml` to both sites — done 2026-05-24 [totebox@claude-code]
- [x] Add `<noscript>` content fallback to both sites — done 2026-06-03 [totebox@claude-code]
- [x] Add Google Search Console verification meta tags to both sites — done 2026-06-03 [totebox@claude-code]
- [ ] **GSC: submit sitemaps** — operator-gated; in GSC Sitemaps panel submit `https://home.woodfinegroup.com/sitemap.xml` and `https://home.pointsav.com/sitemap.xml` [2026-06-03 operator-pending]
- [ ] **GSC: request indexing** — after sitemap submission; URL Inspection → Request Indexing for each homepage [2026-06-03 operator-pending]
- [ ] **Bing Webmaster Tools** — optional; import from GSC at https://www.bing.com/webmasters [2026-06-03 operator-pending]
- [ ] Confirm: any LinkedIn or social profiles to add to `sameAs` in JSON-LD? [2026-05-20 operator-pending]

## UX audit — project-knowledge memo (HIGH, 2026-06-03)

Inbox msg-id: `project-knowledge-20260603-ux-audit-memo-home-woodfinegroup-com-hom`. Five findings:

- [ ] **Self-host all fonts on every route** — contact pages load 6–7 families from fonts.googleapis.com (GDPR exposure); homepages already self-host. Remove all CDN font refs + dead preconnect hints. Subset to Inter + Source Serif 4 only. [2026-06-03]
- [ ] **Nav text minimum 14px** — current 9–11px is sub-legible at institutional viewing distance. Nav/label min: 14px weight 500. Reserve 11px for legal microcopy only. [2026-06-03]
- [ ] **Server-render marketing HTML** — eliminate "Unpacking N assets…" splash; binary already serves flat-file HTML, so serve decoded template directly. Kill no-cache/no-store header. [2026-06-03]
- [ ] **PointSav: promote navy #164679 as dominant brand color** — demote steel gray #B4C5D5 to surface/tint role only. [2026-06-03]
- [ ] **Woodfine P0 typo** — "AN real property developer" → "a real property developer" in hero copy. [2026-06-03]
- [ ] **PointSav P0 typos** — "F*KEYS CONSSOLE", "DIGTIAL TWIN" — fix product headings; add build-time name-lint. [2026-06-03]
- [ ] **PointSav: add true `<h1>`** — largest text currently 19px paragraph; no h1/h2 present. [2026-06-03]
- [ ] **Both: shared global nav** — item sets differ page-to-page; consolidate. [2026-06-03]
- [ ] **Both: remove internal repo path from footer** — "Source: factory-release-engineering/policies/DISCLAIMER.md" is visible publicly. [2026-06-03]
- [ ] **Both: add real contact form** on /page/contact (currently email-only); fix dead `href="#"` "Contact us" anchor. [2026-06-03]

## Pending Command Session

- [ ] Stage 6 promotion — app-orchestration-bim [2026-06-03 totebox@claude-code]
  - `d412d9f8` feat: DWG/RFA external links + Cargo.lock fix
  - `5acbab54` feat: /key-plans route + furniture page IFC-first
- [ ] woodfine-bim-library push to origin (own remote, separate from Stage 6)

## Next Totebox session

- [ ] Extend `generate-key-plan-ifc.py` to additional Key Plan categories
  beyond PO (Corporate Office, Medical, Business etc — 66 total in registry)
- [ ] Resolve archive contamination: manifest.md (project-infrastructure),
  briefs/ (project-knowledge, project-gis, project-console), NEXT.md (replaced)
- [ ] app-workplace-bim Wave 3 scaffold — Tauri v1.7, Phase 1 AutoCAD muscle
  memory, IfcOpenShell subprocess, EUPL-1.2 licence
- [ ] BIM_DESIGN_SYSTEM_DIR decision: woodfine-bim-library tokens (key-plans,
  spatial, etc.) are not loaded by the site — either extend token loading to
  include library_dir/tokens, or keep reading directly per-handler
