# Session Context — project-infrastructure

Rolling 3-session summary. Newest on top. Push oldest to `session-context-archive.md` when
a fourth entry is added.

---

## 2026-05-29 session 12 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- Brief corrections (NetBSD/NVMM replacing bhyve everywhere; Microkit x86-64 constraint
  documented — 1 vCPU/VM, Intel VT-x only; AArch64 remains correct Phase 3 path).
- WireGuard Part A-lite LIVE status documented: Laptop A (10.8.0.6), Laptop B hub
  (10.8.0.1, 24.86.192.209:51820), GCP (10.8.0.9). SSH verified between all nodes.
- GCP KVM absence documented: `/dev/kvm` not present; all QEMU runs TCG; operator action
  required (GCP console nested virtualization).
- New durable artifacts: BRIEF-LEAPFROG-2030.md + BRIEF-OS-FAMILY.md (consolidated os-*
  reference with Phase 1/2/3 targets for all 5 os-* types).
- Updated BRIEF-PPN-ARCHITECTURE.md (§12 resource pooling + §13 GCP KVM),
  BRIEF-PPN-DEV-BOOTSTRAP.md (NVMM + Microkit x86-64 corrections), BRIEF-VM-ARCHITECTURE.md
  (NVMM correction + §8 resource pooling + §9 Leapfrog 2030 table), briefs README.
- Staged editorial: topic-vm-architecture (EN+ES) updated with NVMM correction + new
  Resource Pooling section; topic-os-infrastructure-ppn-node (EN+ES) new bilingual pair;
  guide-vm-infrastructure-resource-pool new GUIDE.
- Three new Rust crates scaffolded and tested:
  - system-vm-fleet-types: wire types (NodeHeartbeat, VmRecord, PlacementAdvice, etc.);
    4/4 serde round-trip tests passing
  - service-vm-fleet: axum :9203, fleet controller, heartbeat ingestion, advisory placement;
    8/8 tests passing (fleet.rs 4 + placement.rs 4)
  - service-vm-host: per-node heartbeat agent, /proc/meminfo reader, QEMU monitor stub;
    2/2 tests passing; `current_thread` Tokio throughout
- Added Rust `[profile.release]` size discipline to workspace Cargo.toml
  (opt-level="z", lto, codegen-units=1, panic="abort", strip).
- Two systemd unit stubs: local-vm-fleet.service (orchestration/) + local-vm-host.service (ppn/).
- Project registry updated: 3 new rows (system-vm-fleet-types, service-vm-fleet, service-vm-host);
  Scaffold-coded 56→59, Total 105→108.
- NEXT.md: fixed 23→33 commit count; added VM-Infrastructure Phase 1 resource pool
  checklist; added Leapfrog 2030 section.
- Outbox: 3 messages (project-editorial pickup; project-system Leapfrog discipline + bench
  #9 coordination; command Stage 6 urgency + GCP KVM operator action).

**Commits this session:**
- `9fec6e35` (Jennifer) — feat(vm-fleet): system-vm-fleet-types + service-vm-fleet — fleet
  controller :9203 + advisory placement; brief corrections NVMM/Microkit; BRIEF-LEAPFROG-2030
  + BRIEF-OS-FAMILY; topic-vm-architecture updated; topic-os-infrastructure-ppn-node +
  guide-vm-infrastructure-resource-pool staged
- `cdc044e9` (Jennifer) — feat(vm-host): service-vm-host per-node heartbeat agent;
  local-vm-host.service; registry rows; NEXT.md session 12; outbox to project-system +
  project-data + command
- `97f8b81c` (Peter) — feat(vm-fleet): kvm_available field + prefer_kvm placement —
  Laptop A/B as primary KVM compute nodes; TCG fallback for GCP e2
  (GCP e2 cannot do nested KVM at all; no migration yet; Laptop A/B = KVM pool;
  5+10+3=18 tests pass)

**Pending / carry-forward:**
- GCP e2 cannot do nested KVM (family-level block; e2→n2 migration deferred until os-* proven on laptops).
- Run `ls /dev/kvm` on Laptop A locally (not from GCP — port 22 refused on WireGuard interface); if absent: `sudo modprobe kvm_intel`.
- Ratify 10.50.0.0/24 as canonical PPN subnet Q2 (operator).
- AArch64 hardware acquisition decision (gates Phase 3 seL4).
- Stage 6 from Command Session: 33 project-data commits + these new commits.
- Deploy service-vm-fleet + service-vm-host after Stage 6 binary rebuild.
- VM-Totebox Phase 1: service-fs still blocked on project-data Stage 6 (33 commits).
- VM-Orchestration Phase 1: blocked on VM-Totebox service-fs.
- Genesis Protocol code steps Q2–Q6 still open.
- J4 ORCID IDs: operator action required.
- manifest.md prose tetrad section still shows old counts (edit failed; YAML frontmatter OK).
- 12+ TOPIC pairs + 4 GUIDEs in drafts-outbound awaiting project-editorial pickup.

**Operator preferences surfaced:**
- Leapfrog 2030 targets: Phase 3 os-* must be 4–10× lighter than Lambda 128 MB.
- `current_thread` Tokio + `opt-level="z"` `[profile.release]` as mandatory engineering
  discipline for all new system-* and service-* crates going forward.
- NetBSD/NVMM (not bhyve) — critical correction to hold across all future briefs.
- GCP e2 is a hard KVM block (not a config issue); Laptop A/B are the KVM pool; don't suggest nested KVM steps for e2 instances.
- Old laptops (Sandy Bridge i5-2400S etc.) are intentionally the Leapfrog 2030 test targets — proving freely-transferable os-* on constrained bare metal is the point.

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


