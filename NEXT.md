# NEXT.md — project-knowledge (app-mediakit-knowledge)

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.

---

## Pending Command Session

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

- [ ] **DESIGN-docs-sidenav-component** — staged in drafts-outbound; routed via outbox
      [2026-06-01 totebox@claude-code]
- [ ] **DESIGN-doc-header-component** — staged in drafts-outbound; routed via outbox
      [2026-06-01 totebox@claude-code]

## Code fix needed

- [ ] **Doorman stub routes missing** — `tests/doorman_test.rs` expects `POST /api/doorman/complete`
      and `POST /api/doorman/instruct` to return 501; both return 404 because the routes don't
      exist in server.rs. Add two minimal stub handlers returning `StatusCode::NOT_IMPLEMENTED`.
      Pre-existing gap; not a regression from UX batch. [2026-06-02 totebox@claude-code]

## Standing deferred

- Phase 12 (AI marginalia): gated on BP5 + SYS-ADR-07/10/19 review [2026-06-01]
- Phase 5.1+ (ACLs/OIDC/webhooks): gated on BP5 clearance [2026-06-01]
- Phase 6 (three-instance deployment split): gated on GitHub renames + Doctrine amendment [2026-06-01]
- claim-rail nightly URL validator: gated on server infrastructure [2026-06-01]
