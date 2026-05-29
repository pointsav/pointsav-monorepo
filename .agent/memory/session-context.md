# Session Context — project-infrastructure

Rolling 3-session summary. Newest on top. Push oldest to `session-context-archive.md` when
a fourth entry is added.

---

## 2026-05-29 session 11 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- Committed modified Cargo.lock (PPN workspace crate join; commit 49d07990).
- Established VM-* architecture: 5 VM types mirror the 5 os-* source binaries exactly.
  VM-Totebox · VM-MediaKit · VM-Orchestration · VM-PrivateGit · VM-Infrastructure.
  Placement principle: service belongs in VM whose os-* namespace owns its data lifecycle.
- Reframed VM-MediaKit Phase 1 as 6/6 COMPLETE: bim-orchestration correctly scoped to
  VM-Orchestration; service-fs correctly scoped to VM-Totebox. No more "6/8 blocked" framing.
- Wrote BRIEF-VM-ARCHITECTURE.md (new durable planning artifact; 7 sections; commit 93949411).
- Restructured NEXT.md: 5 VM-typed sections replacing the flat vm-mediakit section.
- Restructured `infrastructure/systemd/` into per-VM subdirs (mediakit/ orchestration/ ppn/ totebox/)
  using `git mv` — history preserved (commit c0b14bf8).
- Added `infrastructure/virt/lib/common.sh` + `ppn-join.sh` (shared shell functions).
- Added provision script stubs for all 5 VM types (provision-vm-totebox.sh,
  provision-vm-orchestration.sh, provision-vm-privategit.sh,
  provision-vm-infrastructure-cloud.sh, provision-vm-infrastructure-onprem.sh).
- Added cloud-init stubs for VM-Totebox and VM-Orchestration.
- Wrote bilingual TOPIC pair `topic-vm-architecture` (EN + ES); staged to drafts-outbound.
- Updated manifest.md wiki leg + BRIEF README index.
- Sent outbox to project-editorial: 12 TOPIC pairs + 3 GUIDEs staged total.

**Commits this session:**
- `49d07990` — chore(deps): regenerate Cargo.lock — workspace PPN crate join
- `93949411` — docs(vm-arch): BRIEF-VM-ARCHITECTURE + NEXT.md — 5 VM types, MediaKit Phase 1 complete
- `c0b14bf8` — feat(infra): VM-* directory restructure — per-VM systemd/ subdirs + provision stubs + topic-vm-architecture TOPIC pair
- `5edf44b4` — chore(outbox): topic-vm-architecture pickup notice to project-editorial

**Pending / carry-forward:**
- VM-Totebox Phase 1: service-fs blocked on Command promoting project-data (23 commits).
- VM-Orchestration Phase 1: bim-orch depends on VM-Totebox service-fs.
- Genesis Protocol code steps Q2–Q6 operator decisions still open.
- J4 ORCID IDs: operator action required.
- Stage 6 from Command Session (all 4 new commits above + prior session commits).
- migrate-service-to-vm.sh references old systemd path (`infrastructure/systemd/*.service`);
  update to `infrastructure/systemd/mediakit/*.service` when next migration runs.
- 12 TOPIC pairs + 3 GUIDEs in drafts-outbound awaiting project-editorial pickup.

**Operator preferences surfaced:**
- Wants VM-* naming to mirror os-* exactly so development mirrors customer deployment.
- Uses Opus research agents proactively for deep architectural decisions.
- AArch64 hardware decision deferred; seL4 Phase 3 stays on roadmap explicitly.

---

## 2026-05-29 session 9+10 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- Switched vm-mediakit base image from Debian 12 to Ubuntu 24.04 (glibc 2.39 required by
  all host-compiled Rust binaries; Debian 12 only has 2.36 — would segfault on load).
- Booted Ubuntu 24.04 QEMU/TCG VM (PID 4113435). cloud-init completed at guest t=504s.
  SSH confirmed working: kernel 6.8.0-117-generic, glibc 2.39, user foundry.
- Fixed `migrate-service-to-vm.sh`: SCP_OPTS uppercase -P 10022; tar pipe replaces rsync
  (rsync not in Ubuntu minimal); WorkingDirectory creation before systemctl enable; curl
  double-output bug (removed || echo "000"); port-suffixed tmp path prevents binary race;
  smoke test curl non-fatal + 60s timeout for TCG.
- Created 7 systemd unit files in infrastructure/systemd/ for vm-mediakit:
  local-proofreader, local-knowledge-documentation, local-knowledge-corporate,
  local-knowledge-projects, local-marketing-pointsav, local-marketing, local-bim-orchestration.
  All use User=foundry, 0.0.0.0:PORT binds.
- Migrated 6/8 services into vm-mediakit (all originals still running on host, no DNS changes):
  proofreader (9092) ✓ · knowledge-documentation (9090) ✓ HTTP 200 · knowledge-corporate
  (9095) ✓ HTTP 200 · knowledge-projects (9093) ✓ HTTP 200 · marketing-pointsav (9101) ✓
  HTTP 200 · marketing/woodfine (9102) ✓ HTTP 200.
- Installed nginx/1.24.0 and build-essential in Ubuntu VM.
- Sent status update to Command Session outbox: bim-orch blocked on service-fs.

**Commits this session:**
- `96ae4c77` — fix(vm-mediakit): minimal cloud-init — remove package stanza; serial log
- `a52a9cca` — feat(vm-mediakit): VM-adapted unit files + migration script content rsync
- `a23f3d82` — fix(vm-mediakit): use tar pipe instead of rsync for content dirs
- `11acd012` — fix(vm-mediakit): Ubuntu 24.04 base image — glibc 2.39 required
- `2e325dea` — fix(vm-mediakit): smoke test curl 000 double-output bug
- `dd0bd69d` — fix(vm-migrate): port-suffixed tmp path prevents binary race; smoke test non-fatal
- `4be18e37` — chore(vm-mediakit): session 9 status — 6/8 services active; bim-orch blocked
- `df6e4cc3` — chore(session): session context update
- `4a53d3af` — docs(vm-mediakit): topic-os-mediakit Ubuntu 24.04 fix; 2 new GUIDEs
- `658e6876` — chore(outbox): editorial pickup notice for 2 GUIDEs + topic correction

**Pending / carry-forward:**
- service-fs migration (port 9100) — BLOCKED: Command Session must promote project-data's
  23 commits. Status update sent to command@claude-code outbox.
- bim-orchestration migration (port 9096) — BLOCKED on service-fs in VM.
- system-core + system-ledger install — pending project-system reading outbox.
- system-* P0 fixes — pending project-system (outbox sent).
- J4 final gates: ORCID IDs (operator); §4–§5 language pass at project-editorial.
- Operator decision: AArch64 GCP C4A vs Firecracker x86_64 for Phase 3 seL4.
- Q2–Q5 operator decisions still open.
- 11 TOPIC pairs + 3 GUIDEs in drafts-outbound awaiting project-editorial pickup.
- Stage 6 from Command Session (commits ahead of origin/main).
- Cargo.lock modified pre-session, NOT committed.

**Operator preferences surfaced:**
- (no new preferences this session)

---

## 2026-05-29 session 8 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- Ran internet + multi-project research to confirm Microkit 2.2.0 = AArch64/RISC-V only,
  no x86_64 target. This gates the entire seL4 Phase 3 path.
- Appended BRIEF-totebox-transformation.md §9/§10/§11:
  - §9: seL4 Architecture Decision — AArch64-first (Option A: GCP C4A Arm; Option B:
    Firecracker x86_64 on Laptop A; Option C: not recommended)
  - §10: project-data Role — service-fs in Phase 1, Envelope B seL4 PD reference design,
    Ring 1 migration sequence table
  - §11: Firecracker Alternative — specs, guarantees vs seL4, prerequisites
- Appended BRIEF-PPN-DEV-BOOTSTRAP.md §12: seL4 First-Boot Path — Phase 1C.d achievement
  (moonshot-toolkit v0.3.0 boots AArch64 seL4 on qemu-arm-virt), gap table, 7 ordered steps
  + stretch step 8 (x86_64 pc99 Multiboot2).
- Created `infrastructure/virt/provision-vm-mediakit.sh` — Debian 12 QEMU/TCG, 6 GiB,
  9 port-forwards (10022/19090/19092/19093/19095/19096/19100/19101/19102), genisoimage seed
  ISO, virtio-balloon, daemonize + PID file + UNIX monitor socket.
- Created `infrastructure/virt/cloud-init-mediakit/user-data` + `meta-data` — hostname
  vm-mediakit, user foundry, SSH key, NOPASSWD sudo, packages, /opt/mediakit dirs.
- Created `infrastructure/virt/migrate-service-to-vm.sh` — per-service migration helper,
  5 steps (SSH verify, binary, data, systemd unit, smoke test), service→binary map.
- Created `infrastructure/local-vm-mediakit/vm-mediakit.service` — host systemd unit
  managing QEMU process; ExecStop via socat → system_powerdown.
- Generated SSH key pair at `infrastructure/virt/work/foundry-vm-key` (gitignored).
- Wrote topic-os-mediakit.draft.md + .es.draft.md (bilingual TOPIC pair): stack position,
  Phase 1 Debian 12 (present tense), Phase 3 seL4 Microkit (planned/intended), PD layout
  table, system-substrate-sel4 shim, moonshot-toolkit build command, comparison table.
- Outbox: comprehensive message to project-system (Phase 1 system-core install, Phase 2 P0
  blockers with exact line fixes, Phase 3 8-step seL4 build instructions, stretch step 8)
  and to project-data (service-fs Phase 1 install, Ring 1 Phase 2 sequence, Envelope B
  Phase 3 path, binary-targets.yaml ownership ambiguity flag).
- Updated NEXT.md: vm-mediakit Phase 1 (5 items), Phase 2 (P0 blocker), Phase 3 (seL4,
  operator decision needed), topic-os-mediakit TOPIC.
- Fixed inbox.md header (was incorrectly showing project-editorial owner/location).
- New inbox: JOURNAL J4 §4–§5 benchmark data needed (tunnel establishment, rekey latency,
  policy-change propagation, failure-mode behaviour).

**Commits this session:**
- `717ee173` — chore(shutdown): session 6 context — vm-prove complete; sessions 5+6 added; sessions 2+3 archived
- `7e559971` — feat(vm): provision-vm-mediakit — Debian 12 QEMU/TCG, 6GiB, port-forward NAT; BRIEFs §9-§11; os-mediakit TOPIC; outbox to project-system + project-data

**Pending / carry-forward:**
- Install prerequisites on GCP host: `sudo apt install -y genisoimage socat`
- Run `infrastructure/virt/provision-vm-mediakit.sh` to boot vm-mediakit
- Migrate services: service-fs first (blocked on Command Session promoting project-data's
  23 commits), then proofreader → knowledge-* → marketing-* → bim-orchestration
- system-core + system-ledger install — pending project-system reading outbox
- system-* P0 fixes — pending project-system: system-udp (10.50.0.255→10.42.255.255),
  app-network-admin (peer addresses), system-gateway-mba (BASE_DEPLOYMENT_DIR→env var)
- JOURNAL J4 §4–§5 benchmark data — new inbox item (WireGuard tunnel/rekey benchmarks)
- Operator decision: AArch64 GCP C4A vs Firecracker x86_64 before Phase 3 work begins
- Q2–Q5 operator decisions still open from session 4
- 11 TOPIC pairs + 3 GUIDEs in drafts-outbound awaiting project-editorial pickup
- Stage 6 from Command Session (4 commits ahead of origin/main)
- Cargo.lock modified but NOT committed — change pre-dates this session

**Operator preferences surfaced:**
- Wants seL4 on the roadmap explicitly, even if it can't ship in Phase 1 (AArch64 constraint).
- Wants all live services accessible on localhost port-forwards inside vm-mediakit for testing.
- Plans are approved after multiple revision cycles; user redirects plan scope until satisfied.
- "run complete OPUS agents on everything we have here against the internet" — user expects
  internet research agents to be used proactively to validate architectural direction.

---

## 2026-05-28 session 6 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- Wrote two new TOPIC bilingual pairs (EN+ES) and committed to drafts-outbound:
  - `topic-totebox-archive` — sovereign WORM data vault; disk image IS the archive;
    freely transferable; JSONL/GeoParquet/Markdown; Diode + PSP access only; MBA keypair;
    cluster naming convention; what it is NOT.
  - `topic-ppn-architecture-overview` — four-layer architecture overview (operator / PPN /
    hypervisor / Totebox Orchestration); three key properties (isolation invariant, freely
    transferable archives, zero crypto authority at network plane); what PPN is NOT; links
    to all 8 detailed TOPICs.
- Wrote three GUIDE drafts and committed to drafts-outbound:
  - `guide-ppn-first-deployment` — 5-step first-deployment sequence from BRIEF §7, all steps
    unblocked, with exact commands and troubleshooting table.
  - `guide-node-join-ceremony` — approval workflow (node side: Crockford base32 short code;
    operator side: poll + approve via curl); CPace PAKE + SAS; nodes.jsonl; 600s TTL.
  - `guide-vm-prove-balloon-demo` — vm-prove.sh walkthrough; virtio_balloon demo from QEMU
    monitor; pool formula; GCP nested virt enablement; proves-vs-not-yet table.
- Updated manifest.md: wiki leg leg-pending → leg-active; 7 TOPICs + 3 GUIDEs in
  staged_for_pickup.
- VM proof on GCP TCG: Alpine Linux 3.20 (kernel 6.6.31-0-virt) booted in 114s; full
  virtio_balloon cycle confirmed: `balloon 128` → `actual=128`; `balloon 256` → `actual=256`.
- Added `infrastructure/virt/.gitignore` — excludes Alpine ISO + QCOW2 work artifacts.
- Sent outbox session 6 pickup notice to project-editorial (9 TOPIC pairs + 3 GUIDEs total).

**Commits this session:**
- `5029e0fd` — docs(ppn): totebox-archive + ppn-architecture-overview TOPICs; 3 GUIDE drafts; manifest leg-active
- `04388865` — chore(vm-prove): mark GCP TCG balloon proof complete
- `d608f18b` — chore: gitignore virt/work/ — Alpine ISO + QCOW2 are build artifacts

**Pending / carry-forward:**
- Q2: Ratify `10.50.0.0/24` as canonical PPN subnet (de facto confirmed in guide-lxc-network-admin)
- Q3: GCP static IP for cloud relay
- Q4: Laptop B local IP + `network.woodfinegroup.com` DNS status
- Q5: Is service-slm Doorman deployed at `localhost:9080`? (app-network-admin F8 still uses subprocess)
- Q6: Flag stale editorial pickup to Command Session?
- All 7 Genesis Protocol code steps in BRIEF §9.2 gated on Q2–Q6
- 9 TOPIC pairs + 3 GUIDEs in drafts-outbound awaiting project-editorial pickup
- 12 commits ahead of origin/main — Stage 6 from Command Session when ready

**Operator preferences surfaced:**
- "keep going" / "what can we do next" workflow: plans work items in NEXT.md, approves plan,
  then runs phases sequentially; no need to pause between phases once plan is approved.

