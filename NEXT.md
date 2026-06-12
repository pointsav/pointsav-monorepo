# NEXT.md — project-data
# NEXT.md — project-editorial

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.
> **Note: content below the horizontal rule is contaminated from other archives — cleanup needed.**
Last updated: 2026-06-11 [Jennifer Woodfine / claude-code]
# NEXT.md — project-system (cluster/project-system branch)

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.
> Architecture: VM-* naming mirrors the os-* product lineup exactly. See `BRIEF-VM-ARCHITECTURE.md`.

Last updated: 2026-06-11 (session 22 — Phase 1B client_pd.c complete; app-orchestration-gis pipeline scripts committed; GeoLite2 mmdb stale item closed; commit cb03d930 Version 1.4.0; Stage 6 pending).

---

## Blocked externally (no action here)

- [ ] **B19/B20/B21** — inbox `operator-pending`; relay references files not yet staged at project-gis Totebox; awaiting Command response
- [ ] **J1 §7.2 primary spec** — Phase 24B Kontur population join + O-D data needed (project-gis scope)
- [ ] **J2 Bench #9 re-run** — pending at project-intelligence; `forbidden_terms_cleared: true` otherwise
- [ ] **J3 §6 Results** — Night 4 seismic re-run + Night 5 flood build (project-gis scope); `forbidden_terms_cleared: true` otherwise
- [ ] **J6 §6 Results** — user study not yet run; `forbidden_terms_cleared: true` otherwise
- [ ] **ORCID IDs** — all three authors (Peter, Jennifer, Mathew); operator action required before any journal submission
- [ ] **J4 word count** — currently 6,400 / target 9,000 (2,600 words short); needs content input from project-private-network before gap can close

---

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
# NEXT.md — project-marketing

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.
> **Scope: this archive only.** Cross-repo items live at `~/Foundry/NEXT.md`.

Last updated: 2026-06-12 (close-out) [totebox@claude-code]

---

## Phase 9 — Production Deploy — **DEPLOYED 2026-06-11**

- [x] Gate 1: Stage 6 promote — tip 0e18aff3 promoted by Command 2026-06-11. [2026-06-11]
- [x] Gate 2: DESIGN-TOKEN-CHANGE master_cosign — CLEARED 2026-06-09; af51d86. [2026-06-09]
- [x] Binary rebuild and deploy — all 3 instances healthy (9090/9093/9095). [2026-06-11]

---

## Phase 0 — Federation Engine — **COMPLETE 2026-06-12**

- [x] Refactor `AppState` to `mounts: Vec<Mount>`; delete hardcoded content/guide dir fields — dea5e8ae [2026-06-10]
- [x] Wire `blueprints.rs` into render pipeline — AppState loading (dea5e8ae); `relates_to` rail in `wiki_page_inner` (bd435cc3) [2026-06-11]
- [x] `tokens.css` regenerated from `dtcg-bundle.json`; added back to git tracking — bd435cc3 [2026-06-11]
- [x] Slug normalization: `/wiki/topic-foo` → 301 → `/wiki/foo`; `topic-foo.md` fallback; ES-locale aware — bd435cc3 [2026-06-11]
- [x] L25: `/edit/{slug}` route stub + CodeMirror 6 bundle + `toc-persistence.js` + conditional chrome load — bd435cc3 + 7a2b9b42 [2026-06-11]
- [x] M8/M5: Mobile drawer animations + tap-popover flip + Cmd+K trigger — 7a2b9b42 [2026-06-11]
- [x] Stage 6 promote for bd435cc3 + 7a2b9b42 — CONFIRMED; origin/main at 7a2b9b42 [2026-06-11]
- [x] `inject_wiki_prefixes` cross-mount resolution — DONE; link_roots() used at all call sites [2026-06-11]
- [x] Wire `check --strict` as xtask CI gate — `scripts/stage6-gate.sh` committed 9a1326df [2026-06-11]
- [x] Remove `wikilink-unresolved` render path from `render.rs` — DONE 9a1326df; display text only (L18 complete) [2026-06-11]
- [x] Stage 6 promote for 9a1326df — CONFIRMED; origin/main at 9a1326df [2026-06-12]
- [x] Content dead link fix — project-editorial applied fix; gate passes 0 dead links [2026-06-12]
- [ ] Archive ops Stage 6 — 4e2ddf95 → e6d01e9c (+ this commit) pending Command promote; binary rebuild + redeploy required after. [2026-06-12 totebox@claude-code]

---

## Code fixes — pre-promote

- [x] Doorman stub routes: stubs implemented at server/mod.rs:302-315; return 501 NOT_IMPLEMENTED + JSON. All 129 tests pass. [2026-06-12 totebox@claude-code]
- [x] Navigation portlet test: `wiki_page_renders_navigation_portlet` passes; correctly asserts `wiki-page-tabs` (not `sidebar`). [2026-06-12 totebox@claude-code]

---

## Content sync — standing session-start procedure
## Archive cleanup — pending Command action

- [x] **Command: relay project-gis contamination files** — COMPLETE 2026-06-11 session 18.
  23 GIS drafts removed from drafts-outbound/, 3 GIS memory files removed, artifact-registry.md replaced.
- [x] **Replace `.agent/rules/artifact-registry.md`** — COMPLETE 2026-06-11 session 18.
  project-system-native version written covering J2, J5, A1, and routing rules.
- [ ] **Uncommitted code in sub-clone** — `app-network-admin/`, `system-gateway-mba/`, `system-udp/`
  have modified Cargo.toml + src files. Review and commit or discard before next Stage 6.
- [x] **`app-mediakit-telemetry/assets/GeoLite2-City.mmdb`** — confirmed not tracked and not on disk; item was stale. Closed 2026-06-11 session 22.
- [x] **Untracked in sub-clone** — `app-orchestration-gis/` pipeline scripts committed: 83 files (`.gitignore`, 50+ Python/shell pipeline scripts, `www/` HTML research files, `utils/`, `legacy/`, `archive/`) in commit `cb03d930` Version 1.4.0. Closed 2026-06-11 session 22.

## Capability Ledger Substrate — seL4 porting

**Phase 0 — no_std port — COMPLETE 2026-06-11 (session 19)**

Commit `ba4e1de8` in sub-clone `pointsav-monorepo`, Version 1.1.0.
13 files across `system-core` and `system-ledger`. Both crates build
clean on `x86_64-unknown-none --features sel4`. All tests pass:
62+1 (system-core) + 47 (system-ledger). All 12 bench functions pass
under `--test` mode.

- [x] `system-core`: `sel4`/`alloc` feature gates; ciborium CBOR hash; no_std headers
- [x] `system-ledger`: `sel4` feature; BTreeSet/BTreeMap aliases; `tempfile` optional
- [x] `system-ledger/src/witness.rs`: std shellout preserved + no_std W1 in-process Ed25519 verifier
- [x] `.cargo/config.toml`: fiat backend for curve25519-dalek on `x86_64-unknown-none`
- [x] `sha2/force-soft` in both `sel4` features (bypasses SHA-NI intrinsics on bare-metal)
- [ ] **Stage 6 pending** — Command Session to promote sub-clone commit `ba4e1de8` (Version 1.1.0)

**Phase 1A — NetBSD compat-bottom pipeline — COMPLETE 2026-06-11 (session 20)**

Commit `8b0b491e` in sub-clone, Version 1.2.0. Stage 6 pending.

- [x] `system-ledger-proto` — no_std ConsultRequest/ConsultResponse; ckpt_wire (C2SP signed-note); 6 tests
- [x] `system-ledger-server` — Unix socket daemon; tokio; 5 tests; deterministic readiness channel
- [x] `system-substrate-netbsd` — NetBSD 10.1 constants; workspace member
- [x] `os-totebox/src/lib.rs` — build metadata constants; os-totebox-release.img removed from git
- [x] `os-totebox/scripts/build-image.sh` — nbmakefs + Veriexec + rc.d install
- [x] `os-totebox/scripts/provision-data-disk.sh` — 8 GiB OLMo data QCOW2
- [x] `os-totebox/scripts/rc.d/{system_ledger,doorman,service_content,llama_server}`
- [x] `os-interface/scripts/build-image.sh` + `rc.d/orchestration_slm`
- [ ] **Stage 6 pending** — Command to promote `8b0b491e` (Version 1.2.0)

**Phase 1B — seL4 PD scaffold — COMPLETE 2026-06-11 (session 21)**

Commit `428b5086` in sub-clone, Version 1.3.0. Stage 6 pending.

- [x] `system-substrate/src/lib.rs` — `CapabilityInvoker` trait + `VerdictWire`; 3 tests; workspace member
- [x] `system-ledger-pd/` — standalone workspace; `init()`/`protected()`/`notified()`;
  `linked_list_allocator` (use_spin); 512 KiB heap; `ledger.system` XML; `Makefile` with `SDK_PATH ?=`
- [x] Fix `system-security/Makefile` — `SDK_PATH :=` → `SDK_PATH ?= $(error ...)`
- [x] CI check: `cargo build --no-default-features --features sel4 --target x86_64-unknown-none` clean
- [x] Write `src/client_pd.c` test harness — committed in cb03d930 Version 1.4.0. Closed 2026-06-11 session 22.
- [ ] **Download Microkit SDK** — operator action; needed for `make` + QEMU boot test
- [ ] Minimum viable milestone: 2-PD system boots; `client_pd` → `system_ledger` PPC channel 1;
  `VERDICT: Error, code=1` via `microkit_dbg_puts` (SDK download required; then `SDK_PATH=... make`)

**Phase 1C — pre-flight spike (Laptop A, before full VM deploy)**

- [ ] Boot NetBSD 10.1 QCOW2 with llama-server + OLMo 7B weights
- [ ] Verify FFS mmap for large model files, throughput parity vs Linux, wg(4) peer to GCP
- [ ] Cross-compile `system-ledger-server` for `x86_64-unknown-netbsd` target

---

## Completed this cluster (archived for reference)

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
## In scope — project-editorial

- [ ] **Phase F+G scoping** — BRIEF-phase-fg-institutional-redesign.md written 2026-06-11; 6 GUIDEs + DESIGN-wiki-institutional-redesign; awaiting operator go-ahead before any content work
Last updated: 2026-06-08

---

## Operator-gated (browser action required)

- [ ] **GSC: submit sitemaps** — in GSC Sitemaps panel submit:
      `https://home.woodfinegroup.com/sitemap.xml` and `https://home.pointsav.com/sitemap.xml`
      [2026-06-08 operator-pending]
- [ ] **GSC: request indexing** — after sitemap submission; URL Inspection → Request Indexing
      for each homepage [2026-06-03 operator-pending]
- [ ] **Bing Webmaster Tools** — optional; import from GSC at https://www.bing.com/webmasters
      [2026-06-03 operator-pending]
- [ ] **`sameAs` social profiles** — any LinkedIn or company profile URLs to add to JSON-LD?
      [2026-05-20 operator-pending]

---

## UX audit sprint (from 2026-06-03 project-knowledge memo)

All 10 items actioned 2026-06-09. Script: `scripts/ux-audit-sprint.py` (commit `a1d3247b`).

- [x] **Unpacking splash suppressed** — `__bundler_loading` div cleared; setStatus silenced. [2026-06-09]
- [x] **Woodfine P0 typo fixed** — "is an real property" → "is a real property". [2026-06-09]
- [x] **PointSav P0 typos fixed** — "F*KEYS CONSSOLE" → "F-KEYS CONSOLE", "DIGTIAL TWIN" → "DIGITAL TWIN". [2026-06-09]
- [x] **PointSav `<h1>` confirmed present** — `<h1>PointSav</h1>` in outer HTML shell; no change needed. [2026-06-09]
- [x] **Nav text minimum 14px** — topnav + subnav bumped from 10–11px to 14px on mobile breakpoints. [2026-06-09]
- [x] **Google Fonts self-hosted on all routes** — 125 @font-face blocks (Woodfine), 95 (PointSav); GDPR-clean. [2026-06-09]
- [x] **PointSav navy #164679 dominant** — #1d5594 replaced; #164679 appears 23× in template. [2026-06-09]
- [x] **Shared global nav consolidation** — Disclaimer + Contact us consistent across all pages both sites. [2026-06-09]
- [x] **Internal repo path removed from footer** — factory-release-engineering path removed from all pages. [2026-06-09]
- [x] **Dead `href="#"` fixed in contact footers** — replaced with `/page/contact` on both sites. [2026-06-09]

Deferred (out of scope for this sprint):
- [ ] **Real contact form** — server-side POST handler; mailto fix is done; form requires server work. [deferred]

---

## Future mobile improvements (from 2026-06-04 hyperscaler research)

- [ ] Hero photography — Apollo/Brookfield pattern; full-bleed image under hero text
- [ ] Hamburger nav — collapse to icon at <768px; current nav wraps badly
- [ ] Full-width CTAs repeated beneath each feature card
- [ ] Persistent Enquire / click-to-call button (fixed footer on mobile)

---

## Mailbox contamination — ongoing issue

Inbox, outbox, and NEXT.md have been overwritten by Stage 6 rebase operations three times
since 2026-06-05. Each time content from other archives (project-console, project-knowledge,
project-gis, project-intelligence) overwrites this archive's files. Command Session should
investigate the Stage 6/sync-local.sh path that is causing this.

Relay message in outbox: `project-marketing-20260608-contamination-relay` [2026-06-08]

---

## Completed this sprint

- [x] UX audit sprint — all 10 items (`scripts/ux-audit-sprint.py`, commit `a1d3247b`) [2026-06-09]
- [x] robots.txt deployed to both sites [2026-05-24]
- [x] sitemap.xml deployed to both sites [2026-05-24]
- [x] SEO head block applied to both sites [2026-05-24]
- [x] `<noscript>` content fallback added [2026-06-03]
- [x] GSC verification meta tags added [2026-06-03]
- [x] Hero description font standardized 19px (PointSav inline 20px override removed) [2026-06-05]
- [x] Mobile improvements S1–S10, W1, P1–P4 applied across both sites [2026-06-04/05]
- [x] `<link rel="canonical">` added to both sites [2026-06-08]
- [x] JSON-LD Organization/SoftwareApplication structured data added to both sites [2026-06-08]
- [x] `<lastmod>` dates added to all sitemap URLs [2026-06-08]
- [x] TLS (certbot) confirmed live on both domains [2026-06-08]
- [x] Sweep project-intelligence contamination from archive (session 1)
- [x] Fix session-start.md, manifest.md, NEXT.md, memory init (session 1)
- [x] Stage sovereign-mesh.md + .es.md drafts (session 2)
- [x] Fix os-infrastructure/Makefile + forge_iso.sh paths (session 2)
- [x] Gitignore build artifacts in os-infrastructure/ and os-network-admin/ (session 2)
- [x] Create app-infrastructure-onprem/-leased/-cloud/ Reserved-folder scaffolds (session 2)
- [x] PPN architecture: BRIEF-PPN-ARCHITECTURE.md (385 lines, 57 citations) (session 7)
- [x] vm-prove.sh Alpine TCG proof: virtio_balloon confirmed (session 7)
- [x] service-ppn-pairing deployed :9205 (session 13-14)
- [x] service-vm-fleet + service-vm-host deployed on GCP (session 13-14)
- [x] vm_spawn module + QEMU monitor Phase 2 (session 13-14)
- [x] PROSE-RESEARCH v0.2 editorial revision (session 15)
- [x] service-ppn-pairing normalize bug fix + 4 integration tests (session 15)
- [x] service-ppn-pairing fixed binary deployed to :9205 (session 16)
- [x] system-core + system-ledger no_std port — Phase 0 seL4 substrate; 13 files; W1 witness verifier;
  curve25519-dalek fiat backend; sha2/force-soft; ciborium CBOR; Version 1.1.0 (session 19)
- [x] Phase 1A NetBSD pipeline — system-ledger-proto (6 tests) + system-ledger-server (5 tests) +
  system-substrate-netbsd + os-totebox image builder scripts + rc.d + os-totebox-release.img removed;
  root cause: SignedCheckpoint no serde → ckpt_wire (C2SP wire format); Version 1.2.0 (session 20)
- [x] Phase 1B seL4 PD scaffold — system-ledger-pd (standalone workspace; PPC shared-memory ring;
  CI clean on x86_64-unknown-none); system-substrate CapabilityInvoker + VerdictWire (3 tests);
  system-security Makefile SDK_PATH guard; Version 1.3.0 (session 21)
- [x] Phase 1B complete — client_pd.c test harness added; client writes hardcoded ConsultRequest to
  CAP_REQUEST_MR, calls microkit_ppcall(1,...), reads ConsultResponse from CAP_RESPONSE_MR, prints
  verdict; QEMU boot milestone ready once SDK downloaded; app-orchestration-gis 83 pipeline scripts
  committed; Version 1.4.0 (session 22)
