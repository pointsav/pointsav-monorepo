# NEXT.md — project-infrastructure (cluster/project-infrastructure branch)

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.
> Architecture: VM-* naming mirrors the os-* product lineup exactly. See `BRIEF-VM-ARCHITECTURE.md`.

Last updated: 2026-06-01

---

## VM-MediaKit — Phase 1 COMPLETE (6/6) [2026-05-29]

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

## Backlog drift — registry items needing source-project action

- [ ] **B13 Regional Name Resolution TOPIC** — file missing from drafts-outbound; project-gis must write and dispatch. [2026-05-28 totebox@project-editorial]
- [ ] **B14 Co-location Tier Nomenclature TOPIC** — file missing from drafts-outbound; project-gis must write and dispatch. [2026-05-28 totebox@project-editorial]
- [ ] **B15 GIS as BIM Substrate TOPIC** — file missing from drafts-outbound; project-gis must write and dispatch. [2026-05-28 totebox@project-editorial]
- [ ] **B16 UK/EU Food Retail Coverage TOPIC** — file missing from drafts-outbound; project-gis must write and dispatch. [2026-05-28 totebox@project-editorial]

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

---

## Completed this session (2026-06-01 — open inbox + directive §A/§B/§C)

- [x] Compound reply flag (command-20260601-editorial-compound-reply-incomplete): §§2-3 confirmed present; follow-up sent to Command re-stating HIGH-priority items. [Jennifer]
- [x] Knowledge-platform directive §A typography fix: 8 files in design-system/ updated; IBM Plex → Inter + Source Serif 4; measure 65→68ch — commit `1bf7129` (Peter)
- [x] Knowledge-platform directive §B engine TOPIC: app-mediakit-knowledge.md + .es.md federation section added — commit `7c9194a` (Peter)
- [x] Knowledge-platform directive §C new pattern: patterns/federation-via-content-mounts.md + .es.md created — commit `7c9194a` (Peter)
- [x] TOPIC-slm-tiered-substrate: Bloomberg pass + ES sibling authored + committed to media-knowledge-documentation/substrate/ — commit `473716c` (Jennifer)
- [x] GUIDE v0.2 local-circuit-tier-a-only: Bloomberg pass complete; staged to drafts-outbound; routing message sent to Command. [Peter]

---

## Completed this session (2026-05-31 — Command ack + housekeeping)

- [x] Command ack actioned: Group 3 FULLY ACTIONED (3 new placed WFD `7e77081`; 6 non-placed source drafts already archived); Group 4 FULLY ACTIONED (LICENSE refined archived, placement confirmed) — (Jennifer)
- [x] archive-2026-04/ reviewed: stale April 2026 epoch content; effectively archived in subfolder; no active items require re-dispatch (Peter)
- [x] from-project-system/ READMEs routed to Command: 3 bilingual pairs (moonshot-toolkit, system-core, system-ledger) for pointsav-monorepo placement (Peter)
- [x] J1 Spanish bilingual sibling written: JOURNAL-retail-colocation-v0.1.es.md — ~8,500-word academic adaptation in Spanish (Jennifer)

---

## Completed this session (2026-05-31 — full sweep)

- [x] J4 final language pass: §4+§5 confirmed clean; `forbidden_terms_cleared: true` validated; stale notes_for_editor warning removed — `0fa3dfc6` (Peter)
- [x] Group 2 backlog: A3/B1/B2 source drafts confirmed superseded and archived; A4 routing message sent to Command
- [x] Unexpected files resolved: colocation-tier-summary.json archived (data misfile); legal-tokens-*.draft.yaml routing sent to Command
- [x] Artifact registry: A3/A4/B1/B2/J4-language-pass all updated — `1e91a6a1` (Jennifer)

---

## Completed this session (2026-05-30 — infrastructure/GIS pickup)

- [x] GIS Regional Markets batch (12 bilingual TOPICs) committed to media-knowledge-projects — `294488f` (carried from prior session)
- [x] GUIDE A14 (regional-market-topic-production) test-market refs updated: Wichita→Plano TX, Nürnberg→Krefeld DE — staged to drafts-outbound, routed Command
- [x] A13 DESIGN routed to project-design via outbox
- [x] INF-A + INF-B + INF-C: 11 bilingual TOPIC pairs (22 files) committed — `277847a` (Jennifer)
- [x] 4 infrastructure GUIDEs staged to drafts-outbound — `955d6f34` (Peter); routed to Command via outbox
- [x] PROSE-RESEARCH review of PPN architecture phd thesis: 6-point findings returned to project-infrastructure outbox
- [x] Artifact registry A23–A25 added + NEXT.md inbox items actioned

## Completed this session (2026-05-30 — JOURNAL readability overhaul)

- [x] Pass 1 (history-language cleanup) COMPLETE — J1 (`c8a9c3b2`, Peter), J2 (`9218940f`, Jennifer), J3 (`69b6bfd9`, Jennifer), J4 (clean), J6 (`d7b0979f`, Jennifer); internal phase numbers, WIP framing, and development-history narration stripped from all five papers
- [x] Pass 2 (readability overhaul) COMPLETE — J3 (`ab276f23`, ~28 expansions, v0.5), J2 (`cf302a90`, 22 expansions, v0.2), J1 (`62fa554b`, 6 expansions, v0.7), J6 (`0861de14`, 5 expansions, v0.4), J4 (`37733685`, 4 expansions, v0.5); all non-universal abbreviations expanded on first body-text use per R1; topic sentences added to all section openings per R4
- [x] Artifact registry updated — J1–J6 status + version numbers — `a48d395f`

## Completed this session (2026-05-29 — continued)

- [ ] **service-fs binary available on host** — blocked: Command must promote project-data.
  Outbox sent to project-data with install instructions.
- [ ] **VM-Totebox QEMU instance provisioned** — `infrastructure/virt/provision-vm-totebox.sh`
  (stub exists; implementation follows service-fs availability)
- [ ] **service-fs install inside VM-Totebox** — follows promotion
- [ ] **system-core + system-ledger install** — pending project-system outbox response (95 tests)

---

## VM-Orchestration — Phase 1

- [ ] **Provision VM-Orchestration** — `infrastructure/virt/provision-vm-orchestration.sh` (stub)
- [ ] **app-orchestration-bim (9096)** — depends on VM-Totebox service-fs
- [ ] **app-orchestration-gis instance**
- [ ] **app-orchestration-slm instance (:9180)**

---

## VM-PrivateGit — Phase 1 (future)

- [ ] **Provision VM-PrivateGit** — `infrastructure/virt/provision-vm-privategit.sh` (stub)
- [ ] **app-privategit-source-control install** (Gitea + SSH)
- [ ] **app-privategit-design-system** (Storybook)

---

## VM-Infrastructure — Phase 1 Resource Pool

- [x] `system-vm-fleet-types` scaffolded (4/4 tests pass) [session 12]
- [x] `service-vm-fleet` scaffolded (8/8 tests pass; axum :9203; advisory placement) [session 12]
- [x] `service-vm-host` scaffolded (5/5 tests pass; /proc/meminfo; QEMU monitor) [session 12]
- [x] `kvm_available` field + `prefer_kvm` placement; KVM-first with TCG fallback [session 12]
- [x] `vm_spawn` module: create_blank_disk + spawn_qemu + kill_qemu (14/14 tests) [session 13-14]
- [x] QEMU monitor Phase 2: QMP socket scan → VmState::Running (5/5 tests) [session 13-14]
- [x] GET /v1/nodes endpoint + all_nodes() [session 13-14]
- [x] service-vm-fleet deployed on GCP (:9203, gcp-cloud-1, kvm_available=false) [session 13-14]
- [x] service-vm-host deployed on GCP (heartbeating every 10s) [session 13-14]
- **Node roles:**
  - GCP e2-standard-8: fleet coordinator + TCG-only; `prefer_kvm: false` (e2 hard KVM block)
  - Laptop A (10.8.0.6): primary KVM compute; `prefer_kvm: true`
  - Laptop B (10.8.0.1): primary KVM compute (KVM TBD); `prefer_kvm: true`
- [ ] **GCP nested KVM: NOT available on e2.** Migrate to n2-standard-8 if needed later. Deferred.
- [ ] **Verify Laptop A KVM** — `ls /dev/kvm`; if absent: `sudo modprobe kvm kvm_intel`
- [ ] **Deploy service-vm-host on Laptop A + B** — copy binary + `/etc/default/vm-host`
- [ ] **Binary ledger entries (Command action)** — sha256 values:
  - service-ppn-pairing: `dc29e89ac6b0c12fc01407d4c4c7960477bbcab92efd3849d6b9260d10999137`
  - service-vm-fleet: (Command to measure)
  - service-vm-host: (Command to measure)
- [ ] **Stage 6 — session 15-16 commits** — 4-5 commits ahead of origin/main; promote when ready
- [ ] **software-units.yaml: add ppn-pairing-server :9205** (Command action)

---

## VM-Infrastructure — Phase 1 Genesis Protocol

- [x] Alpine Linux TCG proof-of-concept (`vm-prove.sh`) — virtio_balloon confirmed [session 7]
- [x] `service-ppn-pairing` deployed :9205 [session 13-14]
- [x] `service-ppn-pairing` normalize bug fix deployed (approve/deny working) [session 16]
- [ ] **Build + copy `os-network-admin` to Laptop A** — `cargo build --release -p os-network-admin`
  then `scp target/release/os-network-admin mathew@10.8.0.6:~/bin/`.
  Run with `PAIRING_SERVER=http://10.8.0.9:9205 ~/bin/os-network-admin`.
- [ ] **`provision-vm-infrastructure-cloud.sh --genesis`** — stub exists; implement after Q2–Q6
- [ ] **`provision-vm-infrastructure-onprem.sh`** — stub exists; implement after Q2–Q6
- [ ] **Deferred: os-network-admin ratatui TUI** — keyboard approve/deny; QR; expiry countdown

---

## Leapfrog 2030 — resource targets

Phase 3 targets: os-infrastructure 8 MB disk / 12 MB RAM idle; os-totebox 16 MB / 24 MB;
os-mediakit 24 MB† / 32 MB†; os-orchestration 12 MB / 18 MB; os-privategit 20 MB† / 24 MB†.
(† contingent on retiring MediaWiki/PHP and Gitea/Go respectively.)

- [ ] **Operator decision: retire MediaWiki/PHP for Rust static renderer** — gates os-mediakit P3
- [ ] **Operator decision: retire Gitea/Go for gitoxide-based server** — gates os-privategit P3
- [ ] **AArch64 hardware acquisition** — gates os-infrastructure Phase 3 seL4 Microkit
- [ ] **Phase 2: NetBSD 11.0 provision scripts** — `provision-vm-infrastructure-netbsd.sh`

---

## os-mediakit seL4 — Phase 3 (planned — operator decision needed first)

- [ ] **Operator decision: AArch64 GCP C4A vs Firecracker x86_64 on Laptop A**
  Option A: AArch64 GCP C4A Arm instance — Microkit 2.2 native, formal proof (~$50-100/mo)
  Option B: Firecracker + WireGuard on Laptop A — x86_64, KVM-native, pragmatic, free

---

## TOPIC + GUIDE leg — drafts staged, awaiting project-editorial pickup

12 TOPIC pairs + 4 GUIDEs in `.agent/drafts-outbound/`. Pickup notice sent to project-editorial.

**PROSE-RESEARCH:** v0.2 re-staged to project-editorial (session 15). 6 editorial points applied.
Awaiting editorial review and acceptance.

---

## Code — Genesis Protocol (gated on Q2–Q6)

- [ ] Rewrite `os-infrastructure/src/main.rs` — Genesis Protocol boot sequence (Step 1)
- [ ] Implement `system-substrate-broadcom/src/lib.rs` — silicon_ping() (Step 2)
- [ ] Implement `system-network-interface/src/lib.rs` — WireGuard/mDNS substrate (Step 3)
- [ ] Short-code pairing ceremony for node join (Step 4)
- [ ] Replace F8 Gateway subprocess with HTTP to localhost:9080 (Step 5, gated on Q5)
- [ ] Replace JSON mesh payloads with 16-byte binary protocol (Step 6)
- [ ] Add focus crates to root `Cargo.toml` workspace members (Step 8)

---

## Blocking — operator decisions needed first (Q2–Q6)

- [ ] **Q2: Ratify `10.50.0.0/24` as canonical PPN subnet** (code hardcodes 10.50.0.x)
- [ ] **Q3: Provide GCP static IP for cloud relay** (guide-provision-relay.md placeholder)
- [ ] **Q4: Confirm Laptop B local IP + network.woodfinegroup.com DNS status**
- [ ] **Q5: Is service-slm Doorman deployed at localhost:9080?** (gates Step 5 above)
- [ ] **Q6: Stage 6 + editorial pickup confirmed** (flag to Command Session; TOPIC drafts 7+ days)

---

## GUIDE leg — cross-repo fix (Command Session scope)

- [ ] `fleet-infrastructure-leased/guide-deploy-vpn.md` — fix hardcoded path
  `$HOME/Foundry/pointsav-monorepo/` → `/srv/foundry/vendor/pointsav-monorepo/`
  Lives in `customer/woodfine-fleet-deployment` — Command Session admin-tier.

---

## Completed this cluster (archived for reference)

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
