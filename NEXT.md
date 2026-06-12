# NEXT.md — project-data

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-01

---

## State files — repaired 2026-06-01

- [x] **cleanup-log.md contamination** — 9 entries from other archives removed. Committed `59373c45`.
- [x] **Briefs sweep** — 10 contaminated briefs archived (project-intelligence x6, project-knowledge x2,
      project-console x1, project-infrastructure x1). README rewritten.
- [x] **CLAUDE.md title** — corrected to project-data.
- [x] **session-start.md** — replaced with project-data orientation.
- [x] **manifest.md** — replaced with project-data GIS/JOURNAL tetrad.
- [x] **session-context.md** — rewritten with correct project-data context.
- [x] **NEXT.md** — this file; contaminated project-gis items removed.

---

## os-totebox PPN build-out (active — session 1 complete 2026-06-11)

- [x] service-people HTTP API v0.1 — GET /v1/people + GET /v1/people/{id}, port :9091 (997b8d22) [2026-06-11]
- [x] service-extraction workspace unification — added to root Cargo.toml members (997b8d22) [2026-06-11]
- [ ] **Stage 6 promotion** — Command must run bin/promote.sh (25 commits ahead); outbox sent 2026-06-11
- [ ] service-people CRUD — POST /v1/people, PATCH /v1/people/{id}; deferred; unblock after F2 cartridge validates read-only
- [ ] os-totebox startup script — `os-totebox/scripts/start-stack.sh` ordered service startup
- [ ] service-people ledger enrichment — join substrate/ledger_personnel.jsonl email fields into API response
- [ ] J7 §4 Implementation — fill after first os-totebox deployment; §5 Evaluation after benchmark harness built
See BRIEF-os-totebox-ppn-build-out.md for full state.

---

## JOURNAL programme — active blockers

- [x] **J7 HOLD lifted** — Abstract, Introduction, Literature Review, Methodology, Hypotheses, Falsification
      Programme written (~2,600 words); committed 8ab01ff2. §4-§8 stubs pending implementation evidence. [2026-06-11]
- [ ] **J7 language pass** — forbidden_terms_cleared: false; language pass required before submission.
- [ ] **J1 §7.2 OLS regression** — `log(catchment_entropy) ~ tier + log(pop_150km) + C(country)`.
      Blocked on Phase 24B: Kontur H3 population join to `work/clusters-ols.csv`. [2026-05-28]
- [ ] **J1 permutation test** — `sim-tier-permutation.py`; 10,000 spatial shuffles, one-tailed p-value. [2026-05-28]
- [ ] **J3 §6 Results** — AEC flood + seismic build coverage metrics required. [2026-05-29]
      AEC build results checked 2026-06-01:
      - **Seismic (cron 2026-06-01T05:00Z): partial — EXIT 0, but 3 layers skipped:**
        USGS NSHM CONUS (curl empty reply from ScienceBase), ESHM20 EU (GeoJSON not produced),
        GWL_FCS30 wetland (download failed). EU + wetland skip → coverage incomplete for J3.
        Ran twice (05:00Z + 05:12Z); second run also stuck at [1/9]. Needs fix in project-gis.
      - **Flood (cron 2026-06-01T05:18Z): FAILED at step [15/17]** — Python `$META_PATH`
        env var not expanded (`Path('$META_PATH').read_text()` literal, not `os.environ`).
        Needs fix in project-gis `build-aec-flood.sh`. Outbox flagged to Command.
      Coverage metrics still blocked. Will unblock after fixes + re-runs.
- [ ] **J3 coverage metrics export** — write `export-aec-coverage.py` additions after both builds complete and produce valid data.
- [ ] **J4 §4–§5 language pass** — at project-editorial (outbox ref: 952b2b09). [2026-05-29]

---

## Stage 6

- **2026-06-11: monorepo code changes require promotion.** Commits 997b8d22 and 8ab01ff2 include
  service-people/src/bin/server.rs, service-people/Cargo.toml, service-extraction/Cargo.toml,
  Cargo.toml (root), Cargo.lock, and JOURNAL/. Total 25 commits ahead of canonical main.
  Outbox sent to Command (msg-id: command-20260612-promote-project-data-contaminated-brief-).
- Prior archive-local-only commits (59373c45, 005cc299, 161efbd1) also included in the 25.

---

## Command actions pending

- [ ] **Brief redistribution** — 6 project-intelligence briefs archived in this archive need
      Command to copy to `clones/project-intelligence/.agent/briefs/`:
      BRIEF-slm-substrate-master, BRIEF-slm-learning-loop, BRIEF-project-intelligence-active-work,
      AI-AUDIT-baseline-2026-05-31, BRIEF-substrate-phd-thesis-2026-05-27, BRIEF-vm-hardening-and-consolidation.
      Outbox message `project-data-20260601-brief-redistribution` queued.

---

## Regional Markets (editorial — dispatched to project-editorial)

- [ ] **294488f gap** — re-dispatch signal sent to project-editorial
      (outbox `project-gis-20260531-rm-redispatch-294488f`). Command to verify
      content-wiki-projects after project-editorial re-commits.
- [ ] **A10/A11/A12** — hold pending revision (methodology correction applies; project-gis archive
      handles the data verification; this archive tracks JOURNAL dependency only).

---

## Operator-blocked (no Totebox action until input received)

- J1/J2/J3/J4/J6 — ORCID IDs required for submission; operator action.
- J1 Phase 24B — Kontur H3 population join; operator must initiate data acquisition.
- CBRE/JLL leasing data — Year 2 research; operator action.
# NEXT.md — project-knowledge

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-12 [totebox@claude-code]

---

## Phase 9 — Production Deploy — **DEPLOYED 2026-06-11**

- [x] Gate 1: Stage 6 promote — tip 0e18aff3 promoted by Command 2026-06-11. [2026-06-11]
- [x] Gate 2: DESIGN-TOKEN-CHANGE master_cosign — CLEARED 2026-06-09; af51d86. [2026-06-09]
- [x] Binary rebuild and deploy — all 3 instances healthy (9090/9093/9095). [2026-06-11]

---

## Phase 0 — Federation Engine (in progress)

- [x] Refactor `AppState` to `mounts: Vec<Mount>`; delete hardcoded content/guide dir fields — dea5e8ae [2026-06-10]
- [x] Wire `blueprints.rs` into render pipeline — AppState loading (dea5e8ae); `relates_to` rail in `wiki_page_inner` (bd435cc3) [2026-06-11]
- [x] `tokens.css` regenerated from `dtcg-bundle.json`; added back to git tracking — bd435cc3 [2026-06-11]
- [x] Slug normalization: `/wiki/topic-foo` → 301 → `/wiki/foo`; `topic-foo.md` fallback; ES-locale aware — bd435cc3 [2026-06-11]
- [x] L25: `/edit/{slug}` route stub + CodeMirror 6 bundle + `toc-persistence.js` + conditional chrome load — bd435cc3 + 7a2b9b42 [2026-06-11]
- [x] M8/M5: Mobile drawer animations + tap-popover flip + Cmd+K trigger — 7a2b9b42 [2026-06-11]
- [x] Stage 6 promote for bd435cc3 + 7a2b9b42 — CONFIRMED; origin/main at 7a2b9b42 [2026-06-11]
- [x] `inject_wiki_prefixes` cross-mount resolution — ALREADY DONE; link_roots() used at all call sites [2026-06-11]
- [x] Wire `check --strict` as xtask CI gate — `scripts/stage6-gate.sh` committed 9a1326df [2026-06-11]
- [x] Remove `wikilink-unresolved` render path from `render.rs` — DONE 9a1326df; display text only (L18 complete) [2026-06-11]
- [ ] Stage 6 promote for 9a1326df — Stage 6 READY pending; awaiting Command. [2026-06-11 totebox@claude-code]
- [ ] Content fix blocking full 3-mount gate pass — `media-knowledge-documentation/substrate/substrate-without-inference-base-case.es.md` line ~26: `[[service-fs|...]]` → `[[service-fs-architecture|service-fs]]`; flagged to project-editorial (msg-id: command-20260612-dead-link-fix-needed-substrate-without-i) [2026-06-11]

---

## Code fixes — pre-promote

- [x] Doorman stub routes: stubs implemented at server/mod.rs:302-315; return 501 NOT_IMPLEMENTED + JSON. All 129 tests pass. [2026-06-12 totebox@claude-code]
- [x] Navigation portlet test: `wiki_page_renders_navigation_portlet` passes; correctly asserts `wiki-page-tabs` (not `sidebar`). [2026-06-12 totebox@claude-code]

---

## Content sync — standing session-start procedure

```bash
git -C content-wiki-documentation pull --no-rebase /srv/foundry/clones/project-editorial/media-knowledge-documentation main
git -C content-wiki-projects pull /srv/foundry/clones/project-editorial/media-knowledge-projects main
git -C content-wiki-corporate pull /srv/foundry/clones/project-editorial/media-knowledge-corporate main
```

---

## Standing deferred

- Phase 12 (AI marginalia): gated on BP5 + SYS-ADR-07/10/19 review [2026-06-01]
- Phase 5.1+ (ACLs/OIDC/webhooks): gated on BP5 clearance [2026-06-01]
- Phase 6 code refactor (mounts: Vec<Mount>): DONE — dea5e8ae [2026-06-10]
- Phase 6 content-tier GitHub renames + Doctrine amendment: gated on Command + MASTER scope [2026-06-01]

---

## Carry-forward — blocked cross-archive

- [ ] project-jennifer MCP tasks — blocked: jennifer:jennifer filesystem ownership;
      needs Command or jennifer session [2026-06-10 totebox@claude-code]
- [ ] project-console manifest contamination — needs project-console Totebox [2026-06-10]
- [ ] project-bim woodfine-bim-library Stage 6 — needs Command [2026-06-10]
- [ ] project-intelligence residual commit fix — needs project-intelligence Totebox [2026-06-10]
- [ ] Phase E: TOKEN-CHANGE cosign propagation to editorial copy — Command-scope [2026-06-10]
