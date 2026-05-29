# Session Context — project-infrastructure

Rolling 3-session summary. Newest on top. Push oldest to `session-context-archive.md` when
a fourth entry is added.

---

## 2026-05-29 session 9 | Totebox | claude-code (Sonnet 4.6)

**Done this session:**
- Committed session 8 work (J4 v0.3 + vm-mediakit boot fix) from prior conversation that
  ran out of context.
- Fixed `provision-vm-mediakit.sh` daemon mode: replaced mutually-exclusive `-nographic` with
  `-display none -serial none` when using `-daemonize` (commit 539e8494).
- Booted vm-mediakit on GCP TCG: QEMU PID 3949093. Running ~35+ min; cloud-init apt-get
  install still completing over SLIRP NAT on TCG (expected ~60+ min total). SSH banner
  timeout: TCP connects (SLIRP ACK) but sshd not yet responding.
- Wrote JOURNAL J4 §4 (Implementation) + §5 (Evaluation) with empirical WireGuard benchmarks
  on GCP e2-standard-8 (commit 149a8b39); bumped to v0.3.
- Resolved both [CITATION NEEDED] placeholders via internet research (commit b3e8190a):
  - Birge-Lee et al. 2024 "Global BGP Attacks" (DOI 10.1007/978-3-031-85960-1_14) replaces
    fabricated [Cameron et al. 2019]
  - Mackey et al. 2020 "WireGuard vs. OpenVPN" (DOI 10.1145/3374664.3379532) for latency
- Expanded §4.5 with full daemon loop + event detection + signal handling + JSONL log format
  spec; added §5.5 performance comparison table vs. Mackey et al. 2020 (commit 2a79e728).
- J4 now at v0.4 (~8,100 words; target 9,000). Sent to project-editorial outbox for §4–§5
  language pass + `forbidden_terms_cleared` verification (commit a8e6c4ce).
- Flagged two misrouted project-editorial messages in inbox to Command Session for re-routing.
- Cleaned up NEXT.md: struck completed vm-mediakit prerequisites; removed duplicate TOPIC+GUIDE
  section; marked J4 inbox message actioned.
- Updated artifact-registry.md and journal-artifact-discipline.md to reflect J4 v0.4 state.

**Commits this session:**
- `539e8494` — fix(vm): provision-vm-mediakit daemon mode QEMU display args
- `149a8b39` — feat(journal): J4 v0.3 — §4 Implementation + §5 Evaluation empirical benchmarks
- `d6cef558` — chore(outbox): J4 v0.3 handoff to project-editorial
- `cbc1fc01` — chore(registry): J4 v0.3 status
- `4cef97af` — chore(housekeeping): inbox + NEXT.md cleanup
- `223cd3a0` — feat(journal): J4 v0.4 citation rename (Birge-Lee + Mackey)
- `b3e8190a` — chore(journal): J4 v0.4 content — version bump + citations
- `952b2b09` — chore(outbox): J4 v0.4 editorial handoff updated
- `80cbb8a4` — chore(registry): J4 v0.4 file ref + blockers
- `a8e6c4ce` — chore(outbox): flag misrouted messages to Command Session
- `2a79e728` — feat(journal): J4 §4.5 full daemon + §5.5 comparison table; ~8100 words

**Pending / carry-forward:**
- vm-mediakit SSH — cloud-init still running on TCG (PID 3949093); retry SSH after ~60 min
  total uptime. TCP connects (port 10022 SLIRP ACK) but sshd not yet responding.
- Service migration (once SSH up): service-fs → proofreader → knowledge-* → marketing-*
  → bim-orchestration. `migrate-service-to-vm.sh` is ready.
- service-fs migration blocked on Command Session promoting project-data's 23 commits.
- system-core + system-ledger install — pending project-system reading outbox
- system-* P0 fixes — pending project-system (outbox sent)
- J4 final gates: ORCID IDs (operator); §4–§5 language pass at project-editorial; word count
  ~8,100 vs 9,000 target (~900 words short — §4.5 or §6 expandable)
- Operator decision: AArch64 GCP C4A vs Firecracker x86_64 for Phase 3 seL4
- Q2–Q5 operator decisions still open
- 11 TOPIC pairs + 3 GUIDEs in drafts-outbound awaiting project-editorial pickup
- Stage 6 from Command Session — 7 commits ahead of origin/main this session
- Cargo.lock modified pre-session, NOT committed

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

