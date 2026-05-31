# Session Context — project-infrastructure

Rolling 3-session summary. Newest on top. Push oldest to `session-context-archive.md` when
a fourth entry is added.

---

## 2026-05-31 session 13–14 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- BRIEF consolidation: 7 BRIEFs → 4 (BRIEF-totebox-transformation + BRIEF-PPN-DEV-BOOTSTRAP
  archived; content merged into BRIEF-VM-ARCHITECTURE, BRIEF-OS-FAMILY, BRIEF-PPN-ARCHITECTURE).
- Cross-archive guidance messages written to project-system, project-data, project-console
  + BRIEF-OS-FAMILY §7 Totebox-Archive-as-VM scaling table added.
- `vm_spawn` module implemented (create_blank_disk + spawn_qemu + kill_qemu; qemu-img +
  qemu-system-x86_64 -daemonize; user-mode networking; ENV_LOCK mutex for test serialization).
- `vm_spawn` wired into service-vm-fleet: create_vm_handler (spawn_blocking QEMU fork after
  lock release), destroy_vm_handler (kill_qemu), GET /v1/nodes + all_nodes() in fleet.rs.
- QEMU monitor Phase 2: full QMP socket scan in service-vm-host/src/qemu_monitor.rs
  (UnixStream, 500ms timeout, query-status → VmState::Running).
- local-vm-fleet.service created; local-vm-host.service User=foundry→User=mathew fixed.
- All 19 tests pass (14 service-vm-fleet + 5 service-vm-host).
- Deployment (GCP workspace VM): all 3 services active:
  - service-ppn-pairing :9205 (port 9202 conflict with app-privategit-marketplace → moved)
  - service-vm-fleet :9203 (gcp-cloud-1 registered, kvm_available=false)
  - service-vm-host (heartbeating every 10s from gcp-cloud-1)
- /etc/default/vm-host created; /var/lib/vm-fleet created (mathew:foundry ownership).
- Editorial review received from project-editorial for PROSE-RESEARCH-ppn-architecture-phd-thesis:
  6 revision points (register, structure, contribution #4, citations, BCSC posture, abstract).
  Notes in NEXT.md.

**Commits this session:**
- `ba5a8236` (Peter) — chore(outbox): mark BRIEF Q2-Q6 + sessions 2-5 TOPIC relay messages actioned
- `7a9daa83` (Jennifer) — ops(mailbox): Laptop A KVM confirmed present — /dev/kvm verified 2026-05-30
- `7a34038e` (Peter) — ops(mailbox): mark 7 command-facing + editorial outbox messages actioned
- `45f7a255` (Jennifer) — chore(shutdown): session 12 extended close — kvm_available; GCP e2 KVM block
- `87aa0ddd` (Peter) — ops(relay): mark 6 outbox messages actioned
- `34dac679` (Jennifer) — feat(guidance): cross-archive alignment msgs + OS-FAMILY §7 Totebox VM scaling
- `567ed608` (Peter) — feat(vm-fleet): QEMU spawn module + monitor Phase 2 + /v1/nodes + systemd units
- `7cf272a7` (Peter) — fix(ppn-pairing): bind port 9202→9205
- `ab24ab4c` (Jennifer) — ops(deploy): 3 services live on GCP — ppn-pairing :9205, vm-fleet :9203, vm-host

**Pending / carry-forward:**
- Binary ledger entries for 3 new binaries (Command action — outbox written).
- software-units.yaml: add ppn-pairing-server :9205 entry (Command action).
- Stage 6 — 9 commits ahead of origin/main (Command action).
- Deploy service-vm-host on Laptop A + Laptop B (needs /etc/default/vm-host per node).
- PROSE-RESEARCH-ppn-architecture-phd-thesis: 6 editorial revision points (see NEXT.md).
- Laptop A KVM verified (/dev/kvm present) — can assign prefer_kvm=true in fleet when Laptop A vm-host deployed.
- Genesis Protocol: Q2–Q6 operator decisions still open.
- VM-Totebox Phase 1 blocked on project-data Stage 6 (service-fs needed).

**Operator preferences surfaced:**
- (no new preferences this session)

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
