---
artifact: brief
status: active
created: 2026-05-28 (totebox@project-infrastructure)
supersedes: nothing — new document
cross-ref: /srv/foundry/.agent/briefs/BRIEF-sovereign-os-family-master-plan.md §R–§W
---

# Totebox Transformation — VM Fabric Implementation

Mechanism-layer companion to `BRIEF-sovereign-os-family-master-plan.md §R–§W`.
The COMMAND BRIEF covers governance (which folders move where, in what order, gated on what).
This BRIEF covers execution: hypervisor setup, archive assignments, session discipline.

---

## 1. Purpose — project-infrastructure Ships First

project-infrastructure IS the foundation tier. `os-infrastructure` (hypervisor substrate)
and `os-network-admin` (WireGuard routing hub) must exist before any other archive can
separate onto its own VM. No other archive can graduate from `simulation_status: co-tenant`
to `simulation_status: live` until project-infrastructure delivers:

1. A working QEMU/KVM host configuration on foundry-workspace
2. A WireGuard mesh with at least two real peers (Laptop B + GCP VM — already live)
3. Per-VM systemd unit files for `vm-intelligence` and `vm-mediakit` guest images
4. A proven `os-infrastructure` Genesis Protocol boot (gated on later milestone)

The vm-prove.sh proof (2026-05-28) established the floor. The next step is provisioning
real guest VMs with the production workloads, not toy Alpine images.

---

## 2. Three-VM Layout (Recommended Target — Tier B)

**Measured RAM on foundry-workspace (2026-05-28):** 18 GiB used / 32 GiB total.

| VM | os-* Tier | Workload | RAM allocation |
|---|---|---|---|
| `vm-workspace` (HOST — permanent) | `os-privategit` | vault-privategit-source-1: identity store, clones/, vendor/, customer/, 18 Claude sessions, bin/, infrastructure/ tooling | 10 GiB |
| `vm-intelligence` (guest 1) | `os-totebox` + inference | llama-server/OLMo 7B (5.2 GiB), Doorman (:8011), service-content (2.7 GiB), cluster-totebox-* runtime data | 8 GiB |
| `vm-mediakit` (guest 2) | `os-mediakit` | All 6 media-* deployments, gateway-orchestration-{bim,gis,proofreader}-N, nginx TLS termination, public HTTPS | 6 GiB |

**Total: 24 GiB → 8 GiB headroom. No balloon overcommit required.**

Disk note: workspace is 80% full (32 GiB free). Disk may pressure before RAM.
QCOW2 images for guests add ~10–20 GiB; plan disk expansion before provisioning guests.

### Fallback ladder

| Tier | VMs | Total RAM | Headroom | When |
|---|---|---|---|---|
| A | 2 (host + vm-intelligence only) | ~18 GiB | ~14 GiB | Start here — isolates OLMo, highest single-consumer risk |
| B | 3 (+ vm-mediakit) | ~24 GiB | ~8 GiB | **Recommended target** |
| C+ | 4 (+ vm-totebox separate) | ~26 GiB | ~6 GiB | Only after MBA Phase 4 is live |

**Hard ceiling: 4 VMs on foundry-workspace.** vm-console, vm-workplace, vm-orchestration-standalone
belong on `foundry-prod` (separate GCE VM per BRIEF §J.3 Part B).

---

## 3. Archive → VM Assignment

All 23 `clones/project-*` archives are assigned a target VM home. Until Part C VMs are
provisioned, all run co-tenant on `vm-workspace`. The assignment governs where a Totebox
session will live once separation is complete.

| Target VM | Archives |
|---|---|
| `vm-workspace` (permanent) | project-source, project-woodfine, project-design, project-development, project-jennifer, project-mathew, project-documents, project-proforma |
| `vm-intelligence` | project-intelligence (Doorman + OLMo), project-orchestration, project-command, project-bim, project-gis, project-orgcharts, project-bookkeeping, project-data |
| `vm-mediakit` | project-knowledge, project-marketing, project-editorial, project-software, project-console, project-workplace, project-system |
| `vm-workspace` (infrastructure scope — never migrates) | project-infrastructure — owns the hypervisor that runs all guests; stays on host |

### Archive-spanning rule

**One archive = one Totebox session = one `.git/index`. Always.**

project-bim owns both `gateway-orchestration-bim-1` (vm-intelligence target) and
`cluster-totebox-property-1` (vm-intelligence target — same VM, no split needed here).
project-gis similarly. Neither archive splits. Artifacts deploy to their respective
VM destinations; the archive's session working directory does not move across VMs.

---

## 4. Hypervisor Mechanism

### Proven (present tense)

- **virtio_balloon**: inflation/deflation without guest reboot. Proven 2026-05-28 via
  `infrastructure/virt/vm-prove.sh`: Alpine 3.20, `balloon 128→128` in 114s under TCG.
  QCOW2 at `infrastructure/virt/work/ppn-prove.qcow2`.
- **cgroups v2 cpu.weight**: per-VM CPU scheduling without reboot. Dynamic adjustment
  to running QEMU processes.
- **QEMU/KVM**: hardware acceleration available on Laptop A (Sandy Bridge i5-2400S).
  Workspace GCE VM uses TCG (no nested KVM on GCP).

### Planned extensions (virtio-mem + seL4)

- **virtio-mem cross-node lending** (planned): hot-plug/unplug memory blocks over
  WireGuard mesh. Unlike virtio_balloon (single-node pool), virtio-mem intended to
  allow vm-intelligence on Node A to borrow RAM from vm-workspace on Node B.
  Reserved: `moonshot-network/`.
- **seL4 capability model** (planned): formally verified isolation invariant —
  hypervisor has zero read capability over guest VM state. Reserved: `moonshot-hypervisor/`,
  `vendor-sel4-kernel/` (1,074 files, vendored source).
- **dm-verity sovereign attestation** (planned): guest OS root filesystem anchored to
  pairing-ceremony key hash. No Intel Trust Authority, no AMD KMS.

Full architecture: `BRIEF-PPN-ARCHITECTURE.md` Appendix B (Session 7 distributed fabric).

---

## 5. Simulation Discipline

While VMs are co-tenant (before Part C), session discipline enforces VM boundaries:

- **`simulation_status` field** in deployment MANIFEST.md: `co-tenant` (today) vs.
  `live` (after Part C step). Archives in `vm-intelligence` target group behave as if
  they cannot directly read `vm-workspace` filesystems — outbox mailbox protocol for
  all cross-archive communication, no direct file paths.
- **`bin/foundry-role.sh`**: currently resolves CWD to `command` or `totebox`. When
  guest VMs are provisioned, the hostname check (`$HOSTNAME` = `vm-intelligence` etc.)
  becomes an additional role signal. No changes to the script until Part C Step 1.
- **Cross-VM writes blocked by AGENT.md scope re-verification**: "If `pwd` is inside
  `vendor/` or `customer/`: STOP." The same check will apply per-VM-scope once VMs
  exist. No new tooling needed; existing gate mechanism is correct.

---

## 6. Part C Sequencing (Execution Detail)

Governance overview: COMMAND BRIEF §W. Execution detail here.

### Step C1 — Launch vm-intelligence (gate: WireGuard Part A + app-orchestration-command v0.0.1)

1. Provision guest QCOW2 image: Debian/Ubuntu base (production-grade vs. Alpine TCG proof)
2. Systemd unit: `infrastructure/local-vm-intelligence/vm-intelligence.service`
   (`-m 8192 -cpu host -enable-kvm` on KVM-capable host; `-cpu qemu64` on GCE)
3. WireGuard peer entry: `10.42.1.1/32` (specialty gateway subnet per BRIEF §B)
4. Migrate workloads: llama-server unit, Doorman unit, service-content unit
5. MBA pairing: vm-intelligence establishes outbound pairing to gateway-orchestration-command-1
6. Smoke test: `curl http://10.42.1.1:8011/healthz` from workspace host

**What this proves:** OLMo (5.2 GiB, the single largest consumer) crosses a real VM
boundary. Doorman's MBA pairing becomes a real TCP connection over WireGuard, not a
localhost shortcut. The §F.3 "media reads via gateway, never directly" rule becomes
enforceable.

### Step C2 — Launch vm-mediakit (gate: Part B foundry-prod migration complete)

1. Provision guest QCOW2: same base as vm-intelligence
2. Systemd unit: `infrastructure/local-vm-mediakit/vm-mediakit.service`
3. WireGuard peer: `10.42.1.2/32`
4. Migrate workloads: all 6 `media-*` systemd units + nginx config + certbot state
5. DNS remains pointed at foundry-workspace public IP until nginx proxy is confirmed working
6. Smoke test: all 6 media-* services respond on guest; nginx routes correctly

**What this proves:** Public-facing services are isolated from source vault. Workspace VM
has zero public exposure after C2. Architecture matches the §O moonshot diagram.

### Step C3+ — Further splits

Only after C1 and C2 stable for ≥1 week. Add vm-totebox (separate `cluster-totebox-*`
from vm-intelligence) only if MBA Phase 4 is live and Totebox isolation has a runtime
justification (audit-ledger separation, not just theoretical). Do not add for
architectural symmetry alone — disk pressure at 80% full argues against adding QCOW2s.

---

## 7. What project-infrastructure Owns for the Transformation

| Asset | Path | Status |
|---|---|---|
| WireGuard topology | `infrastructure/wireguard/` (to be created) | Planned — COMMAND BRIEF §B |
| vm-prove.sh proof | `infrastructure/virt/vm-prove.sh` | Complete — 2026-05-28 |
| QCOW2 proof disk | `infrastructure/virt/work/ppn-prove.qcow2` | Complete — in .gitignore |
| vm-intelligence systemd unit | `infrastructure/local-vm-intelligence/` | Planned — Part C Step 1 |
| vm-mediakit systemd unit | `infrastructure/local-vm-mediakit/` | Planned — Part C Step 2 |
| ISO build pipeline | `os-infrastructure/` crate | Planned — Genesis Protocol milestone |
| Per-VM network bridge config | `infrastructure/wireguard/` | Planned — Part C Step 1 |

The `simulation_status: co-tenant → live` graduation for each archive is tracked in
its own `deployments/<name>/MANIFEST.md`. project-infrastructure does not maintain a
central graduation register — each archive owns its own readiness.

---

## 9. seL4 Architecture Decision — AArch64-First

**Microkit 2.2.0 (released March 2026) targets AArch64 and RISC-V 64 only. No x86_64.**

seL4 kernel source is verified on x86_64 (pc99 target), but the Microkit framework — the
seL4 Foundation's recommended programming model for new projects — has no x86_64 target.
CAmkES (the predecessor) supports x86_64 but is not recommended for new work.

**What this means for vm-mediakit / os-mediakit:**

| Phase | Host OS | Rationale |
|---|---|---|
| Phase 1 (today) | Debian 12 x86_64 QCOW2, GCP TCG | Proven path; unblocked; gets services running |
| Phase 2 | Debian 12 + system-* P0 fixes + WireGuard | Subnet blockers resolved; PPN mesh active |
| Phase 3 | seL4 Microkit AArch64 bare metal | Replace Debian 12; true formally-verified isolation |

**Two paths for Phase 3:**

**Option A — AArch64 GCP instance (recommended for formal verification story):**
Add a GCP C4A Arm instance to the PPN mesh. project-system's Phase 1C.d achievement
(AArch64 qemu-arm-virt seL4 boot via moonshot-toolkit v0.3.0) targets this exact platform.
Cost: ~$50–100/month for a T2A or C4A Arm instance.

**Option B — Firecracker + WireGuard on Laptop A (recommended for x86_64 production):**
Firecracker microVMs (Rust, 125ms boot, KVM-native, AWS Lambda-proven). Laptop A has real
KVM (VT-x, Sandy Bridge i5-2400S). Not formally verified but sovereign-by-design.
No additional hardware cost. Blocked by real KVM requirement (not available on GCP TCG).

**Option C — seL4 x86_64 Multiboot2:** Requires new AssembleMultibootImage variant in
moonshot-toolkit; loses Microkit programming model; no x86_64 Microkit available. Stretch
goal for project-system, years of work for a small team. Not recommended for Phase 3.

**seL4 Foundation guidance for small teams:** "incremental cyber-retrofit — Linux-in-VM-on-seL4
first, port pieces out over time." Phase 1/2 on Debian 12 is consistent with this guidance.

**project-system Phase 1C.d status (2026-05-29):** `moonshot-toolkit v0.3.0` produces a
bootable AArch64 seL4 system image (elfloader + seL4 kernel + rootserver). QEMU boot:
`qemu-system-aarch64 -machine virt,secure=off -cpu cortex-a53 -m 1G -nographic -kernel build/system-image.bin`
→ "Bootstrapping kernel" → "hello from seL4 rootserver". Architecture: `aarch64`, platform:
`qemu-arm-virt`. This is the foundation for Option A Phase 3.

**Open operator decision (not needed for Phase 1):** Choose Option A or Option B before
project-system begins the os-mediakit seL4 root task wiring (Phase 3 Step 1).

---

## 10. project-data Role in vm-mediakit / os-mediakit

**service-fs (WORM ledger)** is project-data's primary contribution to the Totebox stack.
It is production-ready (v0.1.0+, active on host at `127.0.0.1:9100`) and belongs in
vm-mediakit Phase 1 alongside system-core + system-ledger.

**Host → VM migration:** `local-fs.service` currently runs on host at port 9100.
Inside vm-mediakit: same binary, same port, data dir at `/opt/mediakit/data/service-fs/`.
Host port forward: `19100 → :9100` in `provision-vm-mediakit.sh`.

**service-fs Envelope B (seL4 path):** `service-fs/ARCHITECTURE.md` §Envelope B defines
the seL4 Microkit Protection Domain unikernel form: same CBOR-over-QUIC wire protocol,
same tile format, `system-substrate-sel4` feature flag. This is the reference design
for how Ring 1 services become seL4 PDs in os-mediakit Phase 3.

**Ring 1 surface inside vm-mediakit (migration sequence):**

| Service | Port | Phase | Status |
|---|---|---|---|
| service-fs | 9100 | Phase 1 | Production-ready on host; install in VM |
| service-input | 9106 | Phase 2 | After service-fs stable in VM |
| service-people | 9204 | Phase 2 | After service-fs stable |
| service-email | 9200 | Phase 2 | After service-people stable |

**Prerequisite:** project-data has 23 commits ahead of canonical (2026-05-29). Command
Session must promote these via `bin/promote.sh` before project-data can build+deploy
inside vm-mediakit. Outbox message sent to project-data and command@claude-code.

**Ownership boundary:** `binary-targets.yaml` in project-data lists `service-content` and
`service-extraction` as build targets, but project-data's manifest scopes ownership to the
four Ring 1 services only. Cross-cluster ownership ambiguity must be resolved before
os-mediakit assembly — flag to project-slm.

---

## 11. Firecracker x86_64 Alternative Path

If formal seL4 verification is deferred and x86_64 production is the constraint,
**Firecracker + WireGuard** is the pragmatic sovereign-isolation substrate.

**What Firecracker provides:**
- Rust microVM manager (Amazon, Apache 2.0)
- KVM-native: requires `/dev/kvm` (real VT-x/AMD-V, not TCG)
- 125ms boot to user space (Lambda-production proven)
- Single-process threat model: no QEMU, no BIOS, no PCI bus emulation
- jailer process for seccomp-BPF + cgroups v2 isolation
- WireGuard tap interface assignable to each microVM — same as vm-mediakit user-mode NAT today

**Guarantees vs seL4:**
Firecracker is NOT formally verified. "Well-designed isolation, audited codebase" vs
"machine-checked proof of intransitive non-interference." For the BCSC disclosure posture:
Firecracker path = strong sovereignty claim; seL4 path = formally-verifiable sovereignty claim.

**Prerequisites for Firecracker:**
- Real KVM host: Laptop A (VT-x i5-2400S) or GCP with nested virtualization enabled
- `/dev/kvm` available — GCP workspace VM does not have this
- `firecracker` binary + `jailer` binary (Apache 2.0, installable via apt or release tarball)

**Relationship to vm-mediakit:** Phase 1/2 vm-mediakit uses QEMU user-mode NAT (TCG compatible).
Firecracker would replace QEMU for Phase 3+ once KVM is available. The `provision-vm-mediakit.sh`
script can be adapted to a Firecracker boot config JSON with the same tap+WireGuard network model.

**Decision point:** Operator must choose between Option A (AArch64 seL4) and Option B
(Firecracker x86_64) before Phase 3 provisioning work begins. Both are valid;
they trade "mathematically proven isolation" for "x86_64 compatibility."

---

## 13. Resource Pool Live Status — 2026-06-11 Test Results

Live test run from GCP (foundry-workspace) with all three PPN nodes active.

### WireGuard mesh — CONFIRMED LIVE

```
GCP wg0 (10.8.0.9/24): UP — port 51820 listening
  Peer: Laptop B hub (24.86.192.209:51820) — last handshake 1m52s; 49MB rx / 177MB tx (active)
  Peer: 10.8.0.3/32 — no endpoint (stale entry; some node never connected)

Reachability test:
  10.8.0.1 (Laptop B): 21ms RTT — REACHABLE ✓
  10.8.0.6 (Laptop A): 27ms RTT — REACHABLE via Laptop B hub ✓
```

Active WireGuard traffic between GCP and Laptop B confirms the mesh is live, not just configured.

### Fleet coordination plane — CONFIRMED LIVE

Service unit names are `local-vm-fleet` / `local-vm-host` (not `service-vm-fleet`).
Both active since Jun 10 12:25:54 (restarted after OOM-kill; 1 day uptime).

```
GET /v1/fleet →
  nodes: [{node_id: "gcp-cloud-1", wg_ip: "10.8.0.9",
           ram_available_mb: 12813, vm_count: 0, kvm_available: false}]
```

Only GCP node is registered. Laptop A/B are not in the pool — service-vm-host not deployed there.

### VM spawn test — PARTIAL FAILURE

Submitted: `POST /v1/vms {"vm_type":"vm-orchestration","ram_mb":2048,"vcpu_count":2,"prefer_kvm":false}`

Fleet accepted the request and logged "VM provisioning dispatched" (advisory placement to gcp-cloud-1).
`vm_spawn::create_blank_disk` + `vm_spawn::spawn_qemu` ran in a blocking thread. Result:
- `/var/lib/vm-fleet/` is empty — no qcow2 disk file was created
- No QEMU process appeared
- VM record was evicted as stale (DELETE returned 404) — never transitioned Provisioning → Running

**Root cause:** `vm_spawn.rs` calls `qemu-img create` to make a **blank** qcow2 disk, then boots
it with QEMU. A blank disk has no bootable OS — QEMU starts and immediately halts. The fleet
spawner is missing cloud-image integration. The `provision-vm-*.sh` scripts correctly download
Ubuntu 24.04 and build cloud-init ISOs, but that logic is not wired into `service-vm-fleet`.

### Laptop deployment gap — SSH not reachable

- Laptop A (10.8.0.6): SSH port 22 refused. SSHD not running or on non-standard port.
- Laptop B (10.8.0.1): SSH up but no matching key. Staging identity keys (`id_jwoodfine`,
  `id_pwoodfine`) are GitHub authentication keys — not laptop SSH host keys. Correct key unknown.

### Gap summary

| Gap | Impact | Fix |
|---|---|---|
| `vm_spawn.rs` creates blank disk, not Ubuntu image | Local QEMU spawn produces non-bootable VM | Integrate cloud-image download + cloud-init ISO into fleet spawner, OR call `provision-vm-*.sh` via exec |
| No remote execution API in service-vm-host | Fleet can only spawn VMs on GCP (where fleet runs) | Add POST /v1/vms endpoint to service-vm-host; fleet controller delegates to node-local agent |
| service-vm-host not deployed on Laptop A/B | Only 1 of 3 nodes in pool | Deploy binary + configure env (NODE_ID, WG_IP, FLEET_ENDPOINT) |
| SSH not configured for GCP→Laptop access | Cannot push binaries from GCP over mesh | Use operator's laptop-side terminal to scp from GCP, or add GCP pub key to laptop authorized_keys |

### What "true pooled virtualization" means here

The **coordination plane** works: single API endpoint accepts VM requests across the mesh,
does advisory placement against registered nodes with RAM/KVM metadata, tracks VM records.

The **execution plane** has two gaps: (1) GCP-local QEMU spawn uses blank disks, not production
images; (2) there is no remote execution path from fleet controller to Laptop A/B yet — the
host agent only heartbeats, it does not have a spawn API.

**Verdict:** Coordination layer is real and tested. Execution layer needs two targeted fixes
before full cross-node pooling is functional. Laptop A KVM will be the primary compute tier
once both gaps are closed.

---

## 14. os-network-admin as PPN Control Plane

### What it is today

`os-network-admin` (`app-network-admin` in the monorepo, deploying as the `F8 Terminal`) is
currently a simulation of a WireGuard routing authority. It does three things in the current
stub implementation:

1. `handle_translation()` — HTTP POST to Doorman (`localhost:9080/v1/translate`): routes
   intent expressions to the correct target node ID (F5 Gateway)
2. `handle_authorization()` — emits a 16-byte binary mesh frame: `op_code u16 BE` (ping=0x0001,
   isolate=0x0002) + `target_node u16 BE` + `timestamp u32 BE` + `reserved [u8;8]`
3. Polls `service-ppn-pairing` at `:9205` for pending join requests

It runs as a CLI process today. The binary is built at
`/srv/foundry/cargo-target/mathew/release/os-network-admin`.

### What it should become

os-network-admin is the **PPN control plane** — the authoritative routing authority for the
WireGuard mesh. In the target architecture it owns:

| Responsibility | Current state | Target state |
|---|---|---|
| WireGuard peer table | Managed manually via `wg set` | os-network-admin writes peer configs programmatically |
| Node join approval | CLI poll + curl to approve | os-network-admin TUI (ratatui) with keyboard approve/deny |
| Mesh frame routing | 16-byte binary frame stub over UDP | Real delivery via WireGuard TAP or UDP on 10.8.0.0/24 |
| Fleet coordination | Not connected | os-network-admin notifies `service-vm-fleet` when a new node joins |
| Fault isolation | `isolate` op_code stub | os-network-admin issues `wg set` to remove a peer on ISOLATE |

### Simulation strategy — how it becomes a real component

The simulation discipline is: **write the real interfaces now, substitute real I/O when
the NIC driver and WireGuard bindings are ready.**

**Phase S1 (today — in place):** os-network-admin emits correctly-framed 16-byte binary
mesh frames and polls the pairing server. The binary protocol is real; the transport is
simulated (frames printed/logged, not transmitted over wire).

**Phase S2 (next):** Add a UDP socket on `0.0.0.0:9206` inside os-network-admin.
Send and receive mesh frames over the PPN WireGuard mesh (10.8.0.0/24). No NIC driver
needed — WireGuard is already up and UDP works on it. Nodes on Laptop A and B can receive
these frames. This makes os-network-admin a real network peer without requiring os-infrastructure
to be deployed bare-metal.

**Phase S3 (after Laptop A/B vm-host deployed):** os-network-admin watches
`service-vm-fleet /v1/fleet` for node changes. When a new node heartbeats in, os-network-admin
validates the WireGuard public key against the pairing ceremony (via service-ppn-pairing) and
updates the WireGuard peer table. This closes the join→run lifecycle.

**Phase S4 (Genesis Protocol milestone):** os-infrastructure boots on Laptop A bare-metal.
`system-network-interface::conduct_pairing_ceremony()` contacts os-network-admin's UDP server
directly. os-network-admin issues `wg addconf` to live-add the new peer. From this point, the
Genesis Protocol and the F8 control plane are integrated — the simulation dissolves into the
real thing.

### Where os-network-admin fits in the BRIEF

os-network-admin belongs in the project-infrastructure archive's scope (it ships with the
infrastructure OS tier). Its role in the VM fabric:

- **Gate for Part C Step 1 (vm-intelligence):** The WireGuard peer for vm-intelligence must be
  approved via os-network-admin's join approval before the VM can participate in the mesh.
  Currently this is a manual `wg set` step — os-network-admin Phase S3 automates it.
- **Isolation authority:** If vm-intelligence is compromised, os-network-admin Phase S3 can
  remove its peer entry from all nodes' WireGuard configs by sending an ISOLATE frame.
- **Audit ledger:** Every mesh-frame op (join, isolate, ping) should write to a WORM ledger
  via `service-fs` — gives the BCSC-posture audit trail for all PPN topology changes.

### BRIEF update: os-network-admin next steps

- [ ] **Phase S2:** Add UDP server on :9206 to os-network-admin; send/receive 16-byte frames
      over WireGuard mesh. Verify frames received by Laptop A and B once vm-host is deployed.
- [ ] **Phase S3:** Subscribe os-network-admin to fleet changes; automate peer-table updates
      on join approval. Write each topology change to `service-fs` WORM ledger.
- [ ] **Phase S4:** Wire `system-network-interface::conduct_pairing_ceremony()` to os-network-admin
      UDP server. Test Genesis Protocol end-to-end on Laptop A (bare metal boot, not VM).

---

## 12. Cross-References

- `BRIEF-sovereign-os-family-master-plan.md §R–§W` — governance layer (which folders move where)
- `BRIEF-PPN-ARCHITECTURE.md` — hypervisor TCB, seL4 proof, distributed fabric design
- `BRIEF-PPN-DEV-BOOTSTRAP.md §11` — Session 7 Q&A: no-reboot confirmation, capacity planning
- `BRIEF-sovereign-os-family-master-plan.md §B` — WireGuard address plan (10.42.0.0/16)
- `BRIEF-sovereign-os-family-master-plan.md §J` — Part A (WireGuard) + Part B (foundry-prod) prerequisites
- `infrastructure/virt/vm-prove.sh` — the proven foundation
- `infrastructure/virt/work/ppn-prove.qcow2` — proof QCOW2 (gitignored; rebuild with vm-prove.sh)
