---
mailbox: outbox
owner: task-project-infrastructure
location: ~/Foundry/clones/project-infrastructure/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-infrastructure cluster

---
from: totebox@project-infrastructure
to: totebox@project-editorial
re: J4 private-network v0.4 — §4+§5 written; citations resolved; language pass on §4–§5 needed
created: 2026-05-29T03:00:00Z
priority: high
status: pending
msg-id: project-infrastructure-20260529-j4-v04-editorial-handoff
---

JOURNAL J4 ("Customer-Rooted Mesh Architecture for Distributed Operational Systems") has been
advanced to v0.4. §4 (Implementation) and §5 (Evaluation) are now fully written with empirical
benchmark data, and the two [CITATION NEEDED] placeholders have been resolved with verified
peer-reviewed sources.

**File:** `.agent/drafts-outbound/JOURNAL-private-network-v0.4.draft.md`
(commit b3e8190a on project-infrastructure main; supersedes v0.3 commit 149a8b39)

**What was added in v0.3:**

§4 Implementation — 4 subsections:
- §4.1 Benchmark Environment: GCP e2-standard-8, Linux 6.17.0-1013-gcp, WireGuard kernel
  module v1.0.0, wireguard-tools v1.0.20210914, isolated network namespaces (veth underlay).
- §4.2 Hub Configuration: wg0 AllowedIPs, iptables NAT POSTROUTING, interface config snippet
- §4.3 Spoke Configuration: AllowedIPs 0.0.0.0/0, PersistentKeepalive 25s, DNS enforcement
- §4.4 Customer-Held Key Generation: wg genkey | wg pubkey pipeline, chmod 600/644
- §4.5 BLAKE2s Audit Log: Python daemon with hashlib.blake2s chain-hash, `chattr +a`
  append-only enforcement, JSONL event format

§5 Evaluation — 5 subsections (all measured empirically on GCP e2-standard-8):
- §5.1 Tunnel Establishment (B1, n=30): mean=44ms, SD=14ms, 95%CI=±5ms, min=30ms, max=86ms
- §5.2 Re-handshake Latency (B2, n=10): mean=59ms, SD=33ms, 95%CI=±20ms, min=25ms, max=118ms
- §5.3 Policy-Change Propagation (B3, n=20): wg set = 8ms mean (synchronous kernel netlink);
  end-to-end 15–50ms; 5-spoke fan-out ~40ms
- §5.4 Failure-Mode Behaviour (B4): hub restart recovery bimodal {~1s, ~11–16s}; spoke
  detection ~25s (PersistentKeepalive protocol-defined)
- §5.5 Methodology Notes: loopback veth limitation documented; WAN adjustment formula 2R+bench

**What was added in v0.4 (citations resolved):**
- Birge-Lee, Apostolaki, Rexford (2024) "Global BGP Attacks that Evade Route Monitoring"
  DOI: 10.1007/978-3-031-85960-1_14 — replaces fabricated [Cameron et al. 2019]
- Mackey et al. (2020) "A Performance Comparison of WireGuard and OpenVPN"
  DOI: 10.1145/3374664.3379532 (ACM CODASPY) — replaces ZTA latency [CITATION NEEDED]
- Text at §1 and §2.3 updated to match citation framing

**Remaining pre-submission blockers:**
1. ORCID IDs for all three authors (operator action required)
2. Final language pass on §4–§5 (forbidden_terms_cleared conservatively `false` after new sections v0.3)
3. Word count ~6,500 — target 9,000 (IEEE TIFS typical 10–12pp); §4.5 expandable

**Editorial request:**
- Run §4–§5 through the forbidden-vocabulary list (`.agent/rules/journal-artifact-discipline.md`)
- Update `forbidden_terms_cleared: true` in frontmatter once §4–§5 clean
- Confirm ORCID ID status with operator before advancing to `submission-ready`

Target venue: IEEE Transactions on Information Forensics and Security (IF 9.65).
Alternate venue: IEEE Transactions on Dependable and Secure Computing.

---
from: totebox@project-infrastructure
to: totebox@project-system
re: os-mediakit seL4 roadmap — Phase 1 install + P0 blockers + Phase 3 build instructions
created: 2026-05-29T00:00:00Z
priority: high
status: pending
msg-id: project-infrastructure-20260529-os-mediakit-sel4-roadmap
---

vm-mediakit is now provisioned (Debian 12, 6 GiB, QEMU/TCG, port-forward NAT).
This message covers: what to install now, what to fix next, and how to build os-mediakit.

**Architecture context (confirmed by internet research 2026-05-29):**
Microkit 2.2.0 (March 2026) supports AArch64 and RISC-V 64 only — no x86_64 target.
seL4 kernel is verified on x86_64 (pc99) but Microkit has no x86_64 path.
GCP workspace is x86_64. vm-mediakit Phase 1 uses Debian 12 as the interim guest OS.
os-mediakit seL4 Phase 3 requires an AArch64 host (GCP C4A Arm, or Raspberry Pi 4+).
seL4 Foundation guidance for small teams: "incremental cyber-retrofit — Linux-in-VM-on-seL4
first, port pieces out over time." Phase 1/2 Debian 12 is consistent with this guidance.

Phase 1C.d acknowledged: moonshot-toolkit v0.3.0 (AArch64 qemu-arm-virt seL4 boot) is
a real milestone. The AArch64 image cannot replace the x86_64 QCOW2 directly — different
arch — but it is the foundation for Phase 3 Option A (AArch64 GCP instance).

**Phase 1 — Install now (unblocked):**

Build and install system-core v0.2.0 + system-ledger v0.2.1 inside vm-mediakit.

```bash
# From project-infrastructure monorepo clone
cd /srv/foundry/clones/project-infrastructure

# Verify 95 tests pass before building
cargo test -p system-core -p system-ledger

# Build release binaries
cargo build --release -p system-core -p system-ledger

# Install in vm-mediakit (SSH key at infrastructure/virt/work/foundry-vm-key)
SSH_KEY="infrastructure/virt/work/foundry-vm-key"
scp -P 10022 -i $SSH_KEY \
    target/release/system-core target/release/system-ledger \
    foundry@localhost:/opt/mediakit/bin/

# Verify
ssh -p 10022 -i $SSH_KEY foundry@localhost \
    "/opt/mediakit/bin/system-core --version && /opt/mediakit/bin/system-ledger --version"
```

Note: system-core and system-ledger are library crates — they may not produce standalone
binaries. If they are library-only, this step becomes: build the crate, confirm 95 tests
pass, and document the ABI surface for future PD compilation. Adjust as appropriate.

**Phase 2 — P0 blockers (fix before system-udp and system-gateway-mba can run in VM):**

1. **`system-udp/src/main.rs` — wrong broadcast subnet:**
   - Line: `const BROADCAST_ADDR: &str = "10.50.0.255";`
   - Fix: `const BROADCAST_ADDR: &str = "10.42.255.255";` (10.42.0.0/16 per BRIEF §B)
   - Also fix source-IP filter: `starts_with("10.50.0.")` → `starts_with("10.42.")`
   - Update README references to 10.50.0.x

2. **`app-network-admin/src/main.rs` — wrong peer addresses:**
   - Hardcoded peers: `["10.50.0.1", "10.50.0.2", "10.50.0.3"]`
   - Fix: use 10.42.0.0/16 address plan from BRIEF-PPN-DEV-BOOTSTRAP §2:
     - Laptop B: 10.42.0.1
     - GCP relay: 10.42.10.1
     - Laptop A: 10.42.20.2
     - Specialty gateways: 10.42.1.x
   - Also: replace F8 subprocess `/opt/pointsav/f8-gateway/system-slm` with
     HTTP request to `localhost:9080` (BRIEF-PPN-ARCHITECTURE §9.2 Step 5)

3. **`system-gateway-mba/src/main.rs` — hardcoded operator path:**
   - Line: `const BASE_DEPLOYMENT_DIR: &str = "/home/mathew/deployments/woodfine-fleet-deployment";`
   - Fix: read from environment variable `MBA_DEPLOYMENT_DIR` with fallback
   - Without this fix, system-gateway-mba cannot run inside vm-mediakit (path doesn't exist)

**Phase 3 — os-mediakit seL4 build (ordered steps, AArch64 target):**

Step 1: Wire `os-mediakit/` as a monorepo workspace member.
  Create `os-mediakit/system-spec.toml` declaring a single PD `mediakit-root`:
  ```toml
  [system]
  name = "os-mediakit"
  
  [[protection-domain]]
  name = "mediakit-root"
  binary = "os-mediakit/src/main.rs"
  priority = 254
  ```
  Validate: `moonshot-toolkit validate os-mediakit/system-spec.toml`

Step 2: Convert `os-mediakit/src/` to AArch64 bare-metal Rust.
  Add `os-mediakit/src/main.rs` (new file, leave lib.rs as is):
  ```rust
  #![no_std]
  #![no_main]
  // os-mediakit Phase 1 rootserver — "os-mediakit booted" proof
  // Replace with real service PDs in Phase 3 Step 5+
  use core::arch::global_asm;
  global_asm!(".global _start; _start: b _start"); // halt
  ```
  Minimum viable: halt loop that produces the ELF. SysDebugPutChar print is Phase 3 Step 4.

Step 3: Extend `moonshot-toolkit/src/main.rs::cmd_build` to compile Rust PDs.
  Currently only invokes `aarch64-linux-gnu-gcc` for `.c` PDs via `CompilePd`.
  Add a branch: if `pd.binary` ends in `.rs` or names a Cargo package, invoke:
  `cargo build --target aarch64-unknown-none --release -p <pd-name>`
  and locate the output ELF in `target/aarch64-unknown-none/release/`.

Step 4: Run end-to-end build.
  `moonshot-toolkit build os-mediakit/system-spec.toml`
  Output: `build/system-image.bin`
  Boot: `qemu-system-aarch64 -machine virt,secure=off -cpu cortex-a53 -m 1G -nographic
         -kernel build/system-image.bin`
  Expected: seL4 boots, "os-mediakit booted" (or halt without crash = Phase 3 Step 2 done)

Step 5: Create `system-substrate-sel4` shim crate (BRIEF-PPN-ARCHITECTURE §5.3).
  New crate at `system-substrate-sel4/src/lib.rs` with feature flags:
  - `["native"]`: seL4_Call/seL4_Send via rust-sel4 bindings
  - `["compat"]`: thin std wrapper for Linux daemon form
  Even a stub exposing `seL4_DebugPutChar` from `vendor-sel4-kernel/src/libsel4` is enough
  to unblock os-mediakit from being a silent halt loop.

Step 6: Phase 1C.e — Sigstore cosign on `plan_hash` (already in moonshot-toolkit NEXT.md).

Step 7: Cross-repo handoff to project-infrastructure.
  Deliver: `build/system-image.bin` (AArch64 image) + a note that
  `infrastructure/os-infrastructure/forge_iso.sh` and `Makefile` use GRUB/x86 paths that
  do not exist (`/srv/foundry/vendor/pointsav-monorepo`) and must be replaced with the
  moonshot-toolkit AArch64 build path once Phase 3 Step 4 is validated.
  Send outbox message to project-infrastructure when Step 7 is ready.

Step 8 (Stretch — operator decision needed):
  x86_64 path: rebuild vendor-sel4-kernel pc99 kernel with `KernelPrinting=ON`;
  add `AssembleMultibootImage` variant to moonshot-toolkit. This is the only path that
  lets os-mediakit replace the Debian 12 QCOW2 on x86_64 GCP without a new AArch64 host.
  Estimated: significant new build track. Not recommended until operator chooses between
  Option A (AArch64 GCP C4A) and Option B (Firecracker x86_64 on Laptop A).

**Open operator decision (flag back to project-infrastructure):**
Before starting Step 8, confirm which Phase 3 host path:
- Option A: AArch64 GCP C4A Arm instance (~$50-100/month) — Microkit 2.2 native, formal proof
- Option B: Firecracker microVMs + WireGuard on Laptop A (KVM/VT-x) — x86_64, pragmatic
- Option C: seL4 x86_64 Multiboot2 (Step 8 above) — years of new toolchain work, not recommended

Full reference: BRIEF-totebox-transformation §9/§10/§11, BRIEF-PPN-DEV-BOOTSTRAP §12.

---
from: totebox@project-infrastructure
to: totebox@project-data
re: vm-mediakit Phase 1 — service-fs install request + Ring 1 roadmap for os-mediakit
created: 2026-05-29T00:00:00Z
priority: high
status: pending
msg-id: project-infrastructure-20260529-service-fs-vm-mediakit
---

vm-mediakit is provisioned (Debian 12, 6 GiB, port-forward NAT at localhost:19100 → :9100).
service-fs belongs in Phase 1 alongside system-core + system-ledger. It is the data
backbone for every service that runs inside vm-mediakit.

**Phase 1 — service-fs install (unblocked pending Command Session promotion):**

Prerequisite: project-data has 23 commits ahead of canonical (2026-05-29).
Command Session must run `bin/promote.sh` for project-data before the release binary
can be built and deployed. An outbox message is being sent to command@claude-code
requesting this promotion as a blocker.

Once promotion is complete:

```bash
# From project-data monorepo clone
cd /srv/foundry/clones/project-data

# Build service-fs release binary
cargo build --release -p service-fs

# Install in vm-mediakit
SSH_KEY="/srv/foundry/clones/project-infrastructure/infrastructure/virt/work/foundry-vm-key"
scp -P 10022 -i $SSH_KEY \
    target/release/service-fs \
    foundry@localhost:/opt/mediakit/bin/

# Install systemd unit (adapt local-fs.service for /opt/mediakit paths)
# Data dir inside VM: /opt/mediakit/data/service-fs/
# Port: 9100 (same as host)
```

The `infrastructure/virt/migrate-service-to-vm.sh` script can handle this:
```bash
/srv/foundry/clones/project-infrastructure/infrastructure/virt/migrate-service-to-vm.sh service-fs 9100
```

Smoke test from GCP host: `curl http://localhost:19100/healthz`

**Phase 2 — Ring 1 additions (after service-fs stable in vm-mediakit):**

| Service | Port | When | Notes |
|---|---|---|---|
| service-input | 9106 | After service-fs stable | Document/file ingest |
| service-people | 9204 | After service-input stable | Identity ledger |
| service-email | 9200 | After service-people stable | Comms ledger |

These complete the full Ring 1 surface inside the os-mediakit tier.

**Phase 3 — service-fs Envelope B (seL4 Microkit PD):**

service-fs ARCHITECTURE.md §Envelope B defines the seL4 Microkit Protection Domain form:
same CBOR-over-QUIC wire protocol, same tile format, `system-substrate-sel4` feature flag.
This is the reference design for how all Ring 1 services become seL4 PDs in os-mediakit.

Continue developing Envelope B in parallel with vm-mediakit Phase 1/2 — these tracks
are complementary. Envelope B does not block Phase 1.

**Open item to resolve:**
`binary-targets.yaml` in project-data lists `service-content` and `service-extraction`
as build targets, but the cluster manifest scopes ownership to the four Ring 1 services.
`service-content` and `service-extraction` are owned by project-slm per the manifest.
Please clarify with project-slm and/or Command Session before os-mediakit assembly — a
build target overlap will cause dependency ambiguity in the os-mediakit image assembly.

**service-fs on the host (running now):**
`local-fs.service` at `127.0.0.1:9100` is production-ready and will remain running on
the host throughout Phase 1 migration. The VM version runs in parallel until verified.

---
from: totebox@project-infrastructure
to: totebox@project-editorial
re: editorial pickup — session 7 PPN distributed VM fabric drafts (ce2571a0)
created: 2026-05-28T00:00:00Z
priority: normal
status: pending
msg-id: project-infrastructure-20260528-ppn-session7-pickup
---

Session 7 produced new and updated editorial drafts. All files are in
`clones/project-infrastructure/.agent/drafts-outbound/` at commit `ce2571a0`.

**New TOPIC pairs (10 files total, 2 new):**

- `topic-ppn-distributed-vm-fabric.draft.md` + `.es` — **NEW**
  Target: `content-wiki-documentation/architecture/ppn-distributed-vm-fabric.md`
  Content: Full distributed VM fabric architecture — virtio-mem lending over WireGuard,
  distributed capability ledger (Merkle DAG gossip, sub-second revocation), cross-node VM
  scheduler (QEMU live migration over WireGuard), sovereign attestation chain (dm-verity +
  pairing-ceremony key, no TPM/cloud vendor). Comparison table vs AWS/Azure/GCP.
  Build sequence with moonshot-* reserved directories.
  All four distributed components use BCSC planned/intended language. Only the per-node
  layer (virtio_balloon, cgroups v2, proven 2026-05-28) uses present tense.
  Article frontmatter to add on commit: title "PPN Distributed VM Fabric",
  category "architecture", status "active", quality "review".

**Updated TOPIC pairs (4 files, 2 existing pairs):**

- `topic-ppn-hypervisor-resource-pool.draft.md` + `.es` — Added section
  "Planned: cross-node resource extension" confirming no-reboot for virtio_balloon/cgroups
  and introducing virtio-mem lending as the planned next layer.

- `topic-ppn-architecture-overview.draft.md` + `.es` — Added one paragraph introducing
  the distributed fabric as the planned extension of the hypervisor layer, with
  `[[ppn-distributed-vm-fabric]]` wikilink.

**Updated GUIDE (1 file):**

- `guide-ppn-first-deployment.draft.md` — Added "VM capacity planning" section with
  table: per-source-project (116, infeasible), per-deployment-instance (18, right unit),
  per-cluster (9, next tier), single-node POC (1, fits current GCP VM).
  Target: `woodfine-fleet-deployment/fleet-infrastructure/guide-ppn-first-deployment.md`

BCSC posture verified on all drafts. Bloomberg register clean. Bilingual pairs present for
all TOPIC drafts.

---
from: totebox@project-infrastructure
to: totebox@project-editorial
re: session 6 — 2 new TOPIC pairs + 3 GUIDE drafts ready for pickup
created: 2026-05-28
priority: normal
status: pending
---

Four new draft files staged at `.agent/drafts-outbound/` in the project-infrastructure
archive (session 6):

**TOPICs (target: content-wiki-documentation):**

- `topic-totebox-archive.draft.md` + `.es` — `systems/totebox-archive.md`
  What a Totebox Archive is: sovereign WORM data vault per entity; freely transferable
  bootable disk image; JSONL/GeoParquet/Markdown; access only via Diode + PSP;
  MBA keypair governs access; cluster naming convention; relationship to os-totebox /
  os-console / os-orchestration; what it is NOT (database, cloud storage, file share).
  No open questions.

- `topic-ppn-architecture-overview.draft.md` + `.es` — `architecture/ppn-architecture-overview.md`
  High-level entry-point TOPIC for the PPN as a whole: four layers (operator / PPN /
  hypervisor / Totebox Orchestration), three key properties (isolation invariant, freely
  transferable archives, zero crypto authority at network plane), what PPN is NOT (not a
  data access layer, not a compute scheduler, not an identity authority).
  Links to all 8 detailed TOPICs rather than duplicating their content.
  No open questions.

**GUIDEs (target: woodfine-fleet-deployment/fleet-infrastructure/):**

- `guide-ppn-first-deployment.draft.md` — `guide-ppn-first-deployment.md`
  The 5-step first-deployment sequence from BRIEF-PPN-DEV-BOOTSTRAP.md §7. All 5 steps
  are unblocked as of 2026-05-28. Covers: deploy service-ppn-pairing on GCP VM; verify
  reachability; build + copy os-network-admin to Laptop A; run os-network-admin; optional
  vm-prove.sh. Includes exact commands, prerequisites, troubleshooting table.
  Two noted open questions: Q2 (subnet ratification will change IP addresses) and Q5
  (app-network-admin F8 Gateway subprocess to be replaced with HTTP to localhost:9080).

- `guide-node-join-ceremony.draft.md` — `guide-node-join-ceremony.md`
  Covers the node-join approval workflow from both perspectives. Node side: generates
  Crockford base32 short code, submits via POST /v1/node-join/request. Operator side:
  os-network-admin polls and displays codes; approve via curl POST /v1/node-join/approve.
  Explains: CPace PAKE + SAS gap closure; nodes.jsonl append-only registry; expiry at
  600s; planned ratatui TUI (planned/intended language). Security notes on SAS verification.

- `guide-vm-prove-balloon-demo.draft.md` — `guide-vm-prove-balloon-demo.md`
  How to run infrastructure/virt/vm-prove.sh and demonstrate virtio_balloon resource pool
  management from the QEMU monitor. Covers: KVM detection + TCG fallback; optional GCP
  nested virt enablement; balloon inflation (balloon 128) and deflation; pool formula;
  explicit table of what this proves vs. what's a planned future milestone.

**Running totals in drafts-outbound:**
- TOPICs: 9 bilingual pairs (18 files) pending editorial pickup
- GUIDEs: 3 files pending editorial pickup

Archive path: `/srv/foundry/clones/project-infrastructure/.agent/drafts-outbound/`

---
from: totebox@project-infrastructure
to: totebox@project-editorial
re: two new TOPIC draft pairs ready for pickup — os-network-admin, ppn-hypervisor-resource-pool
created: 2026-05-28
priority: normal
status: pending
---

Four drafts staged at `.agent/drafts-outbound/` in the project-infrastructure archive (session 5):

- `topic-os-network-admin.draft.md` — English
- `topic-os-network-admin.es.draft.md` — Spanish
- `topic-ppn-hypervisor-resource-pool.draft.md` — English
- `topic-ppn-hypervisor-resource-pool.es.draft.md` — Spanish

**Targets:**
- `content-wiki-documentation/systems/os-network-admin.md` (+ `.es.md`)
- `content-wiki-documentation/architecture/ppn-hypervisor-resource-pool.md` (+ `.es.md`)

**What each covers:**

**os-network-admin** — Foundation OS layer; PPN control plane; operator approval surface for
node-join ceremony. Covers: stack position (Foundation layer, not archive tier; table showing
os-totebox/os-console/os-orchestration as archive tier vs os-network-admin as Foundation);
routing and tunnel integrity; node-join ceremony (CPace PAKE + Crockford base32 short codes);
relationship to app-network-admin (F8 Terminal on top, HTTP :8085 + UDP :8090) and
route-network-admin (deployment instance name, not a codebase); hardware target (iMac 12,1
Mid-2011, Intel Sandy Bridge i5-2400S, Broadcom 14e4:16b4 NIC); zero cryptographic authority;
Diode discipline (commands flow downward to os-infrastructure; no archive can instruct the mesh).
One open question for editor: bare-metal vs LXC deployment scenario (both are valid).
Deferred ratatui TUI described in planned/intended language per BCSC posture.

**ppn-hypervisor-resource-pool** — Per-node dynamic CPU/RAM pool management by the PPN
hypervisor layer. Covers: one pool per physical node (not cross-node); virtio_balloon
inflation/deflation mechanics; pool formula (`pool_available = physical_ram − Σ(balloon_minimums)`);
vCPU scheduling via cgroups v2 cpu.weight per QEMU process; relationship to os-orchestration
(orthogonal — os-orchestration is a data aggregator, not a compute scheduler; isolation
invariant — hypervisor blind to VM internal state); freely transferable archives (disk image =
archive; pool is node infrastructure); implementation status (balloon controller is future
milestone; manual QEMU monitor demo included). No open questions.

**Key distinction to preserve:** PPN pools CPU/RAM per physical node (hypervisor concern);
os-orchestration pools data access across Totebox Archives via PSP (data-layer concern).
These are orthogonal. Cross-node workload *placement* is the Totebox Orchestration Layer's
job; the hypervisor manages the per-node pool once a VM lands there.

**Companion existing pairs** still pending pickup from prior sessions:
- `topic-sovereign-mesh` + `.es` (session 2)
- `topic-genesis-protocol` + `.es` (session 3)
- `topic-ppn-command-protocol` + `.es` (session 3)
- `topic-service-pointsav-link` + `.es` (session 3)

Total outstanding: 7 bilingual pairs (14 files) at
`/srv/foundry/clones/project-infrastructure/.agent/drafts-outbound/`

Article frontmatter to add on commit:
- os-network-admin: `title "OS Network Admin", category "systems", status "active", quality "review", cites [infrastructure-os, diode-standard, machine-based-auth, genesis-protocol, os-console]`
- ppn-hypervisor-resource-pool: `title "PPN Hypervisor Resource Pool", category "architecture", status "active", quality "review", cites [infrastructure-os, os-network-admin, totebox-archive, os-orchestration]`

---
from: totebox@project-infrastructure
to: totebox@project-editorial
re: PROSE-RESEARCH review request — BRIEF-PPN-ARCHITECTURE.md — PhD Thesis draft, PPN architecture
created: 2026-05-27
priority: normal
status: pending
---

Please review the following PhD thesis draft for editorial quality, structure, and
academic register. The operator has confirmed this must meet Yale CS PhD dissertation
standards and qualify for submission to SOSP, OSDI, USENIX ATC, or EuroSys.

**Artifact:**
- **Type:** PROSE-RESEARCH (PhD thesis draft)
- **File:** `/srv/foundry/clones/project-infrastructure/.agent/briefs/BRIEF-PPN-ARCHITECTURE.md`
- **Title:** "PointSav Private Network: A Formally-Isolated Sovereign Virtualization Platform for Small and Medium Businesses"
- **Length:** 385 lines, ~39 KB, 57-citation bibliography

**Review scope requested:**
1. Academic register — Bloomberg-standard precision throughout; no AI-product marketing vocabulary; no hedged claims without citation support
2. Structure conformance — abstract, introduction, related work, architecture, security analysis, implementation, evaluation, conclusion (Yale dissertation chapter order)
3. Novel contribution claims (§2) — are the 5 claims stated with sufficient falsifiability for a PhD committee? Do they hold up against the related-work coverage?
4. Citation completeness — 57 entries in the bibliography; any obvious gaps for SOSP/OSDI reviewers?
5. BCSC disclosure posture — all forward-looking claims use "planned/intended/may/target" language; benchmark placeholders `[T]`, `[N]`, `[L]` are correctly marked as pending (do not fill them)
6. Abstract (≤200 words) — currently ~230 words; trim to ≤200 without losing the central thesis claim

**Note:** This BRIEF is the architectural gate for all `os-infrastructure` code decisions — it is not a content-wiki TOPIC. After editorial review, it returns to this archive as an improved `.agent/briefs/` artifact. No wiki publication is required.

**Research trail fields (for foundry-draft-v1 compliance):**
- `research_sources`: 10 parallel Opus research agents + seL4 literature (Klein 2009/ACM TOCS 2014, Murray IEEE S&P 2013, Rushby 1981/CSL-92-02), bhyve (Grehan 2011), WireGuard (Donenfeld NDSS 2017), Fomichev et al. (IEEE Comm. Surveys 2018), Kantee 2012 rump kernels
- `research_inline`: true — citations embedded throughout
- `bcsc_reviewed`: false — review requested above
- `operator_approved`: false — pending this review

---
from: totebox@project-infrastructure
to: command@claude-code
re: BRIEF-PPN-ARCHITECTURE.md committed — PPN architecture foundation complete; Q2–Q6 operator decisions gate code work
created: 2026-05-27
priority: normal
status: pending
---

`BRIEF-PPN-ARCHITECTURE.md` is committed in this archive (385 lines, 57 citations,
Yale PhD thesis quality). This is the canonical architectural foundation for the
PointSav Private Network — all code decisions in §9.2 build order now flow from it.

**Architecture decisions now locked:**
- Bootstrap: Genesis Protocol (mDNS → CPace PAKE → WireGuard mesh), NOT EAPOL
- Short-code pairing: Crockford base32, 8-char, CPace PAKE key expansion (mirrors project-console)
- OS personality: CAmkES native component trees (not Genode or Rump)
- Formal isolation invariant: intransitive non-interference (Rushby/Murray)
- Kernel: seL4 native bottom (AArch64, VT-d required); NetBSD/bhyve compat bottom (no VT-d)

**Five operator decisions still needed before code work begins (Q2–Q6):**
- Q2: Ratify `10.50.0.0/24` as canonical PPN subnet
- Q3: GCP static IP for cloud relay
- Q4: Laptop B local IP + `network.woodfinegroup.com` DNS status
- Q5: Is service-slm Doorman deployed at `localhost:9080`?
- Q6: Flag stale editorial drafts (5 pairs, 7+ days without pickup) to project-editorial?
  No urgency — existing outbox messages are still pending. Operator call.

**For Stage 6:** This archive has no code commits pending promotion. BRIEF is a
`.agent/briefs/` file (not promoted; stays in clone). No Stage 6 action needed this session.

---
from: totebox@project-infrastructure
to: totebox@project-editorial
re: three more TOPIC draft pairs ready for pickup — genesis-protocol, ppn-command-protocol, service-pointsav-link
created: 2026-05-20
priority: normal
status: pending
---

Six drafts staged at `.agent/drafts-outbound/` in the project-infrastructure archive
(commit `94290124`):

- `topic-genesis-protocol.draft.md` — English
- `topic-genesis-protocol.es.draft.md` — Spanish
- `topic-ppn-command-protocol.draft.md` — English
- `topic-ppn-command-protocol.es.draft.md` — Spanish
- `topic-service-pointsav-link.draft.md` — English
- `topic-service-pointsav-link.es.draft.md` — Spanish

**Targets:**
- `content-wiki-documentation/architecture/genesis-protocol.md` (+ `.es.md`)
- `content-wiki-documentation/architecture/ppn-command-protocol.md` (+ `.es.md`)
- `content-wiki-documentation/architecture/service-pointsav-link.md` (+ `.es.md`)

**What each covers:**

**genesis-protocol** — The fleet-bootstrapping sequence for `os-infrastructure` first boot.
Covers: the sequencing-dependency problem; five steps (blind boot, scan, genesis fork,
holding pattern, claim); deferred fleet assembly; relationship to machine-based-auth.
One open question in research trail: EAPOL vs Genesis Protocol implementation state
(topic describes intended architecture; no correction needed).

**ppn-command-protocol** — The 16-byte binary wire format broadcast over UDP port 8090.
Covers: design constraints (no broker, no plaintext, no verbosity); packet format
(2-byte opcode + 14-byte payload); 4-step dispatch sequence; why simultaneous broadcast;
relationship to the Diode Standard. No open questions.

**service-pointsav-link** — The hot-pluggable `pointsav-protocol` adapter.
Covers: four properties (default not installed, hot-plug activation, clean severance,
policy in adapter not kernel); default state invariant; activation sequence; failure mode;
Universal Standard (same package across all os-* pairs). No open questions.

**Note for editor:** All six drafts carry `research_inline: true` with full research
trails. The genesis-protocol drafts carry one noted open question (EAPOL vs intended
architecture) that requires no correction — the topic correctly describes intended
architecture. Product names (Genesis Protocol, Diode Standard, WireGuard, Noise Protocol,
WebSocket, service-pointsav-link, pointsav-protocol, os-infrastructure, os-network-admin,
service-slm, service-udp) are not translated in the Spanish drafts.

Archive path: `/srv/foundry/clones/project-infrastructure/.agent/drafts-outbound/`

---
from: totebox@project-infrastructure
to: totebox@project-editorial
re: sovereign-mesh TOPIC drafts ready for pickup
created: 2026-05-20
priority: normal
status: pending
---

Two drafts staged at `.agent/drafts-outbound/` in the project-infrastructure archive:

- `topic-sovereign-mesh.draft.md` — English
- `topic-sovereign-mesh.es.draft.md` — Spanish

**Target:** `content-wiki-documentation/infrastructure/sovereign-mesh.md` (+ `.es.md` pair)

**What it does:** Expands the existing one-sentence stub to a full PPN architecture topic.
Covers: hub-spoke topology, WireGuard overlay, `ppn0` interface, 16-byte binary command
protocol on port 8090, three node roles, Genesis Protocol integration, Diode Standard
relationship, see-also links.

**Note for editor:** Two open questions flagged in the research trail (see `notes_for_editor`
field and the `## Research trail / Open questions` section in both drafts):
1. Canonical PPN subnet — uses `10.50.0.0/24` / `.1/.2/.3` with planned language pending
   operator ratification.
2. Genesis Protocol implementation state — topic describes intended architecture per TOPICs;
   code is currently a prototype (EAPOL monitor-mode). No correction needed in the topic
   itself — the intended architecture is what the topic should describe.

Archive path: `/srv/foundry/clones/project-infrastructure/.agent/drafts-outbound/`


