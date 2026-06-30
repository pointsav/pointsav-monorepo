---
artifact: brief
schema: foundry-brief-v1
brief-id: project-infrastructure-ppn-reference
title: PPN Infrastructure — Architecture Reference
status: reference
owner: project-infrastructure
created: 2026-05-28
updated: 2026-06-29
parent: BRIEF-totebox-transformation.md
---

# PPN Infrastructure — Architecture Reference

Stable architecture decisions extracted from `BRIEF-totebox-transformation.md` (2026-06-14
consolidation). Read this for design rationale. Read the active BRIEF for live state and
open work items.

Cross-refs: [[BRIEF-PPN-ARCHITECTURE]], [[BRIEF-sovereign-os-family-master-plan]]

---

## §1. Purpose — project-infrastructure Ships First

project-infrastructure IS the foundation tier. `os-infrastructure` (hypervisor substrate)
and `os-network-admin` (WireGuard routing hub) must exist before any other archive can
separate onto its own VM. No other archive can graduate from `simulation_status: co-tenant`
to `simulation_status: live` until project-infrastructure delivers:

1. A working QEMU/KVM host configuration on foundry-workspace
2. A WireGuard mesh with at least two real peers (Laptop B + GCP VM — live as of 2026-06-12)
3. Per-VM systemd unit files for `vm-intelligence` and `vm-mediakit` guest images
4. A proven `os-infrastructure` Genesis Protocol boot (gated on later milestone)

The vm-prove.sh proof (2026-05-28) established the floor. The next step is provisioning
real guest VMs with the production workloads, not toy Alpine images.

---

## §2. Three-VM Layout (Recommended Target — Tier B)

**Measured RAM on foundry-workspace (2026-05-28):** 18 GiB used / 32 GiB total.

| VM | os-* Tier | Workload | RAM allocation |
|---|---|---|---|
| `vm-workspace` (HOST — permanent) | `os-privategit` | vault-privategit-source-1: identity store, clones/, vendor/, customer/, Claude sessions, bin/, infrastructure/ tooling | 10 GiB |
| `vm-intelligence` (guest 1) | `os-totebox` + inference | llama-server/OLMo 7B (5.2 GiB), Doorman (:8011), service-content (2.7 GiB), cluster-totebox-* runtime data | 8 GiB |
| `vm-mediakit` (guest 2) | `os-mediakit` | All 6 media-* deployments, gateway-orchestration-{bim,gis,proofreader}-N, nginx TLS termination, public HTTPS | 6 GiB |

**Total: 24 GiB → 8 GiB headroom. No balloon overcommit required.**

Disk note: workspace is ~80% full (32 GiB free). QCOW2 images for guests add ~10–20 GiB;
plan disk expansion before provisioning guests.

### Fallback ladder

| Tier | VMs | Total RAM | Headroom | When |
|---|---|---|---|---|
| A | 2 (host + vm-intelligence only) | ~18 GiB | ~14 GiB | Start here — isolates OLMo, highest single-consumer risk |
| B | 3 (+ vm-mediakit) | ~24 GiB | ~8 GiB | **Recommended target** |
| C+ | 4 (+ vm-totebox separate) | ~26 GiB | ~6 GiB | Only after MBA Phase 4 is live |

**Hard ceiling: 4 VMs on foundry-workspace.** vm-console, vm-workplace, vm-orchestration-standalone
belong on `foundry-prod` (separate GCE VM per BRIEF §J.3 Part B).

---

## §3. Archive → VM Assignment

All `clones/project-*` archives are assigned a target VM home. Until Part C VMs are
provisioned, all run co-tenant on `vm-workspace`. The assignment governs where a Totebox
session will live once separation is complete.

| Target VM | Archives |
|---|---|
| `vm-workspace` (permanent) | project-source, project-woodfine, project-design, project-development, project-jennifer, project-mathew, project-documents, project-proforma |
| `vm-intelligence` | project-intelligence (Doorman + OLMo), project-orchestration, project-command, project-bim, project-gis, project-orgcharts, project-bookkeeping, project-data |
| `vm-mediakit` | project-knowledge, project-marketing, project-editorial, project-software, project-console, project-workplace, project-system |
| `vm-workspace` (infrastructure scope — never migrates) | project-infrastructure — owns the hypervisor that runs all guests; stays on host |

**One archive = one Totebox session = one `.git/index`. Always.**

project-bim owns both `gateway-orchestration-bim-1` (vm-intelligence target) and
`cluster-totebox-property-1` (vm-intelligence target — same VM, no split needed here).
Artifacts deploy to their respective VM destinations; the archive's session working
directory does not move across VMs.

---

## §4. Hypervisor Mechanism

### Proven (as of 2026-05-28)

- **virtio_balloon**: inflation/deflation without guest reboot. Proven via
  `infrastructure/virt/vm-prove.sh`: Alpine 3.20, `balloon 128→128` in 114s under TCG.
  QCOW2 at `infrastructure/virt/work/ppn-prove.qcow2`.
- **cgroups v2 cpu.weight**: per-VM CPU scheduling without reboot. Dynamic adjustment
  to running QEMU processes.
- **QEMU/KVM**: hardware acceleration available on Laptop A (Sandy Bridge i5-2400S).
  Workspace GCE VM uses TCG (no nested KVM on GCP).

### Planned extensions

- **virtio-mem cross-node lending** (planned): hot-plug/unplug memory blocks over
  WireGuard mesh. Unlike virtio_balloon (single-node pool), virtio-mem intended to
  allow vm-intelligence on Node A to borrow RAM from vm-workspace on Node B.
  Reserved: `moonshot-network/`.
- **seL4 capability model** (planned): formally verified isolation invariant —
  hypervisor has zero read capability over guest VM state. Reserved: `moonshot-hypervisor/`,
  `vendor-sel4-kernel/` (1,074 files, vendored source).
- **dm-verity sovereign attestation** (planned): guest OS root filesystem anchored to
  pairing-ceremony key hash. No Intel Trust Authority, no AMD KMS.

---

## §5. Simulation Discipline

While VMs are co-tenant (before Part C), session discipline enforces VM boundaries:

- **`simulation_status` field** in deployment MANIFEST.md: `co-tenant` (today) vs.
  `live` (after Part C step). Archives in `vm-intelligence` target group behave as if
  they cannot directly read `vm-workspace` filesystems — outbox mailbox protocol for
  all cross-archive communication, no direct file paths.
- **`bin/foundry-role.sh`**: currently resolves CWD to `command` or `totebox`. When
  guest VMs are provisioned, the hostname check (`$HOSTNAME` = `vm-intelligence` etc.)
  becomes an additional role signal. No changes to the script until Part C Step 1.
- **Cross-VM writes blocked by AGENT.md scope re-verification**: the same check will
  apply per-VM-scope once VMs exist. No new tooling needed; existing gate mechanism is correct.

---

## §6. Part C Sequencing (Execution Detail)

Governance overview: COMMAND BRIEF §W. Execution detail here.

### Step C1 — Launch vm-intelligence (gate: WireGuard Part A + app-orchestration-command v0.0.1)

1. Provision guest QCOW2 image: Debian/Ubuntu base (production-grade vs. Alpine TCG proof)
2. Systemd unit: `infrastructure/local-vm-intelligence/vm-intelligence.service`
   (`-m 8192 -cpu host -enable-kvm` on KVM-capable host; `-cpu qemu64` on GCE)
3. WireGuard peer entry: `10.42.1.1/32` (specialty gateway subnet per COMMAND BRIEF §B)
4. Migrate workloads: llama-server unit, Doorman unit, service-content unit
5. MBA pairing: vm-intelligence establishes outbound pairing to gateway-orchestration-command-1
6. Smoke test: `curl http://10.42.1.1:8011/healthz` from workspace host

**What this proves:** OLMo (5.2 GiB, the single largest consumer) crosses a real VM
boundary. Doorman's MBA pairing becomes a real TCP connection over WireGuard, not a
localhost shortcut.

### Step C2 — Launch vm-mediakit (gate: Part B foundry-prod migration complete)

1. Provision guest QCOW2: same base as vm-intelligence
2. Systemd unit: `infrastructure/local-vm-mediakit/vm-mediakit.service`
3. WireGuard peer: `10.42.1.2/32`
4. Migrate workloads: all 6 `media-*` systemd units + nginx config + certbot state
5. DNS remains pointed at foundry-workspace public IP until nginx proxy is confirmed working
6. Smoke test: all 6 media-* services respond on guest; nginx routes correctly

**What this proves:** Public-facing services are isolated from source vault.

### Step C3+ — Further splits

Only after C1 and C2 stable for ≥1 week. Add vm-totebox only if MBA Phase 4 is live
and Totebox isolation has a runtime justification (audit-ledger separation, not just
theoretical). Disk pressure (~80% full) argues against adding QCOW2s unnecessarily.

---

## §7. What project-infrastructure Owns for the Transformation

| Asset | Path | Status |
|---|---|---|
| WireGuard topology | `infrastructure/wireguard/` (to be created) | Planned — COMMAND BRIEF §B |
| vm-prove.sh proof | `infrastructure/virt/vm-prove.sh` | Complete — 2026-05-28 |
| QCOW2 proof disk | `infrastructure/virt/work/ppn-prove.qcow2` | Complete — in .gitignore |
| infrastructure .gitignore | `infrastructure/.gitignore` | Done — 2026-06-29 |
| vm-intelligence systemd unit | `infrastructure/local-vm-intelligence/` | Planned — Part C Step 1 |
| vm-mediakit systemd unit | `infrastructure/local-vm-mediakit/` | Planned — Part C Step 2 |
| os-infrastructure binary | `os-infrastructure/` crate + Microkit 2.2.0 | Active — CLAUDE.md added 2026-06-29 |
| os-network-admin binary | `os-network-admin/` crate | Active — Phase S3 done; S4+daemon pending |
| Per-VM network bridge config | `infrastructure/wireguard/` | Planned — Part C Step 1 |

---

## §9. seL4 Architecture Decision — Updated 2026-06-29

**⚠️ §9 revised: prior assumption (AArch64-only Microkit) was stale.**

Microkit 2.1.0 (November 26, 2025) added x86-64 support (`x86_64_generic` and
`x86_64_generic_vtx` targets). Microkit 2.2.0 (the pinned version) includes this support.
The earlier §9 claim "Microkit 2.2.0 targets AArch64 and RISC-V 64 only. No x86_64" is incorrect.

### Verified configurations as of 2026-06-29

| Architecture | Verification level | Notes |
|---|---|---|
| AArch64 EL2 | Functional correctness + integrity (April 2025) | Confidentiality in progress. Only verified hypervisor-mode config. |
| RISC-V64 | Deepest: binary-level + integrity + confidentiality | Constrained to HiFive Unleashed. |
| x86-64 (`X64_verified.cmake`) | Functional correctness ONLY | No binary proof. No formal security claims permissible. |

**AArch64 EL2 integrity proof (April 2025):** Funded by UK NCSC. Extends the existing
functional correctness proof to include an information-flow integrity invariant at
hypervisor mode. This is what makes the formal security claim possible on AArch64.

### Two-tier architecture

- **x86-64 targets** (`x86_64_generic`, `x86_64_generic_vtx`): Runtime/development targets.
  Boot on Laptop A, foundry-workspace (QEMU/TCG), iMac. No formal verification claims.
  Used for: os-infrastructure test milestones, os-network-admin daemon mode, three-node mesh test.
- **AArch64 EL2 target**: Verified production target. Future GCP C4A Arm instance.
  Formal security claims only on AArch64. Required before making "topology determines security"
  claims in public product copy.

### Three-path architecture (see §19)

Option B is the current active path. Options C and A are documented moonshots.

| Option | seL4 Role | Linux | Formal security |
|---|---|---|---|
| B (current) | seL4 at EL2/VT-x | Guest VM (Debian 12) | AArch64 EL2 only |
| C (moonshot-sel4-vmm) | seL4 PDs own WireGuard + PPN control plane | Guest VM for workloads | AArch64 EL2 only |
| A (moonshot-hypervisor) | Pure seL4 PDs for everything | None | AArch64 EL2 or RISC-V64 |

### Boot chain for x86-64

GRUB2 Multiboot/Multiboot2. No UEFI official support (Neutrality Atoll pushing this).
`x86_64_generic_vtx` requires VT-x (Intel VT-x / AMD-V). Laptop A (Sandy Bridge i5-2400S)
and iMac (2010-2012 Intel) both have VT-x.

### moonshot-toolkit task #14 — unblocked (2026-06-29)

Three decisions recorded in `moonshot-toolkit/src/main.rs` and `NEXT.md`:
1. **Toolchain**: Microkit 2.2.0 SDK; `aarch64-linux-gnu-gcc` for ARM; native `gcc` for x86.
2. **seL4 vendoring**: `vendor-sel4-kernel/` (1,074 files, seL4 15.0.0, already vendored).
3. **Reproducible-build**: `plan_hash` in `plan.rs` + Ed25519 sign images with
   `identity/id_pointsav-administrator`.

---

## §10. project-data Role in vm-mediakit / os-mediakit

`service-fs` (WORM ledger) is project-data's primary contribution. It is production-ready
(v0.1.0+, active on host at `127.0.0.1:9100`) and belongs in vm-mediakit Phase 1 alongside
system-core + system-ledger.

**Host → VM migration:** `local-fs.service` currently runs on host at port 9100.
Inside vm-mediakit: same binary, same port, data dir at `/opt/mediakit/data/service-fs/`.
Host port forward: `19100 → :9100` in `provision-vm-mediakit.sh`.

**Ring 1 surface inside vm-mediakit (migration sequence):**

| Service | Port | Phase | Status |
|---|---|---|---|
| service-fs | 9100 | Phase 1 | Production-ready on host; install in VM |
| service-input | 9106 | Phase 2 | After service-fs stable in VM |
| service-people | 9204 | Phase 2 | After service-fs stable |
| service-email | 9200 | Phase 2 | After service-people stable |

---

## §11. Firecracker x86_64 Alternative Path

**Firecracker + WireGuard** is the pragmatic sovereign-isolation substrate if formal
seL4 verification is deferred and x86_64 production is the constraint.

- Rust microVM manager (Amazon, Apache 2.0)
- KVM-native: requires `/dev/kvm` (real VT-x/AMD-V, not TCG)
- 125ms boot to user space (Lambda-production proven)
- Single-process threat model: no QEMU, no BIOS, no PCI bus emulation
- jailer process for seccomp-BPF + cgroups v2 isolation

**Guarantees vs seL4:** Firecracker is NOT formally verified. "Well-designed isolation,
audited codebase" vs "machine-checked proof of intransitive non-interference."

**Prerequisites:** Real KVM host (Laptop A — VT-x i5-2400S) or GCP with nested
virtualization enabled. GCP workspace VM does not have `/dev/kvm`.

**Decision point:** Choose between Option A (AArch64 seL4) and Option B (Firecracker
x86_64) before Phase 3 provisioning work begins.

---

## §14. os-network-admin as PPN Control Plane — Architecture

### What it is today

`os-network-admin` (`app-network-admin` in the monorepo, deploying as the `F8 Terminal`):

1. `handle_translation()` — HTTP POST to Doorman (`localhost:9080/v1/translate`)
2. `handle_authorization()` — emits a 16-byte binary mesh frame: `op_code u16 BE`
   (ping=0x0001, isolate=0x0002) + `target_node u16 BE` + `timestamp u32 BE` + `reserved [u8;8]`
3. Polls `service-ppn-pairing` at `:9205` for pending join requests
4. **Phase S2 (done 2026-06-14):** UDP socket on `0.0.0.0:9206`; tokio recv loop;
   parses 16-byte frames; PING → PONG reply; `PPN_PEERS` env var; `PPN_MESH_LISTEN_PORT` override

### What it should become (target state)

| Responsibility | Current state | Target state |
|---|---|---|
| WireGuard peer table | Managed manually via `wg set` | os-network-admin writes peer configs programmatically |
| Node join approval | CLI poll + curl to approve | os-network-admin TUI (ratatui) with keyboard approve/deny |
| Mesh frame routing | 16-byte binary frame via UDP (S2) | Real delivery over WireGuard TAP on 10.8.0.0/24 |
| Fleet coordination | Not connected | os-network-admin notifies `service-vm-fleet` when a new node joins |
| Fault isolation | `isolate` op_code stub | os-network-admin issues `wg set` to remove a peer on ISOLATE |

### Phase roadmap

**S1 — done:** 16-byte frames emitted; pairing server polled.
**S2 — done (2026-06-14, commit 3bafaec5):** UDP :9206 listener; PING→PONG.
**S3 — next:** Fleet watch loop + automated peer-table + WORM ledger (see active BRIEF).
**S4 — Genesis Protocol:** Wire `system-network-interface::conduct_pairing_ceremony()` to
UDP server. Test end-to-end on Laptop A bare metal.

---

## §15. PPN Install Model — Any Hardware, Sovereign Compute

Installing os-infrastructure on a machine — bare-metal laptop, leased VPS, or cloud VM —
makes that machine a PPN node. The install creates a WireGuard interface, joins the mesh,
and starts contributing resources to the pool. From the pool's perspective, all node types
are identical: a node_id, a WireGuard IP, an available RAM count, a KVM flag.

| Profile | Hardware owner | KVM | WireGuard bootstrap |
|---|---|---|---|
| Bare-metal (laptop/desktop) | Operator | Likely yes | wg-quick or Genesis Protocol ISO |
| Leased server / VPS | Third party (provider has physical access) | Often yes | Same as bare-metal |
| Cloud VM (GCP/AWS etc.) | Cloud provider | Depends on instance type | Same; GCP node also serves as genesis relay |

**Critical distinction:** In the leased-server and cloud-VM profiles, the hardware owner
can physically access the machine. WireGuard alone does not protect against this. The seL4
microkernel isolation layer (planned/intended — not currently running on bare metal) is the
answer. Until seL4 bare-metal is delivered, the PPN provides network-layer encryption but
not hardware-layer isolation.

**os-network-admin admission gate:** Anyone who generates a valid WireGuard keypair and
learns the genesis endpoint could add themselves as a peer. os-network-admin is the
admission gate: every join request goes through the CPace pairing ceremony and requires
explicit approval from the control plane.

**Mode A — external authority (current):** os-network-admin runs on the GCP relay node.
Simple to bootstrap. Risk: dependency on cloud-provider-hosted node.

**Mode B — self-governing first VM (intended production):** os-network-admin is the first
VM spawned on the genesis node, running inside a seL4 partition. Requires seL4 bare-metal.

**What the 2026-06-12 test proved:**
Three heterogeneous nodes (GCP VM, MacBook Pro, MacBook Air) formed a live WireGuard mesh
and functioning resource pool. Advisory placement selected the correct node without operator
input. VM spawn delegation crossed a node boundary. Old consumer hardware (6-10 year old
MacBooks): valid compute tier at 3 GB available RAM, KVM acceleration.

Total operator time to add Laptop A: approximately 10 minutes of terminal interaction.

**Target install experience:** Single bootable ISO — write to USB, boot, answer 3 questions
(node name, genesis endpoint, pairing code). os-infrastructure is that ISO. Current gap:
os-infrastructure does not yet boot on bare metal with a working Genesis Protocol.
Phase S4 closes the gap.

---

## §16. Customer Virtualization Layer — service-vm-tenant

*Architectural review by claude-fable-5, 2026-06-12.*

`service-vm-tenant` (:9221) is a thin authenticated proxy in front of `service-vm-fleet` (:9203).
Bearer module-id authentication, stateless tenant registry (environment-seeded for MVP),
server-side injection of validated tenant identity. Four routes: create VM, list VMs,
destroy VM, usage/quota status.

```
Customer (Bearer: <opaque-token>)
  → service-vm-tenant :9221   — auth, tenant namespace, quotas
  → service-vm-fleet  :9203   — internal, placement, delegation
  → service-vm-host   :9220   — internal, QEMU spawn per node
```

### Isolation guarantees (shipped as of 2026-06-14)

Three guaranteed:
- **Namespace isolation** — authenticated tenant cannot enumerate or destroy another tenant's VMs.
- **Process isolation** — each VM is a separate QEMU/KVM process.
- **Per-VM network containment** — SLIRP gives each guest a private NAT'd stack; no inbound path.

Two explicitly NOT guaranteed:
- No per-tenant subnet — all VMs egress through the host indistinguishably.
- No isolation from the node operator — root on a physical node can read guest disk/memory.

Additionally: `:9203` and `:9220` must be firewalled to accept traffic only from
service-vm-tenant (loopback or mesh-internal). Otherwise the auth layer is bypassable.

### Design decisions (Fable review, implemented 2026-06-14)

1. **tenant_id persists via heartbeats** — fleet VM record carries tenant_id; vm-host echoes
   it in heartbeats so registry rebuilds correctly after fleet restart. Correctness requirement.
2. **Bearer token is opaque** — TOKEN_MAP env var maps random tokens → tenant_ids (A1, dbf6a528).
3. **Quota enforcement serialized** — `create_lock: Arc<Mutex<()>>` in AppState.
4. **SLIRP hostfwd** — VmRecord carries host_ports (HostPortMapping) for inbound SSH paths (A3, dbf6a528).
5. **Audit trail** — tenant lifecycle ops write to service-fs WORM ledger via fire-and-forget POST (A4, dbf6a528).

### Network-level isolation path

API-layer isolation ships now. Network-layer isolation (per-tenant WireGuard subnets) is
gated on os-network-admin Phase S3 (automated peer-table management) and seL4 Mode B.
Until then: API-level isolation is the shipped capability.

---

## §17. os-network-admin — Control Plane Role (OpenStack/CloudStack Comparison)

*Architectural research by claude-fable-5, 2026-06-12.*

**os-network-admin is the Neutron/Management-Server-equivalent network control plane.**

```
service-vm-fleet       ≈ Nova          (compute placement)
os-network-admin       ≈ Neutron       (peer tables, tenant IPAM, subnet allocation)
app-network-admin      ≈ operator TUI  (F8 Terminal — local operator only)
<planned portal>       ≈ Horizon       (customer-facing thin app; separate Phase 3 item)
```

**What "customers see one IP space" requires:**
1. **Port abstraction + IPAM datastore** — tenant IPs bound to port records owned by
   os-network-admin, never to nodes. IPs stay stable across node changes.
2. **Overlay transport** — VXLAN tunneled over WireGuard underlay, or nested per-tenant
   WireGuard interfaces (WG-in-WG). The latter is simpler at 3 nodes; VXLAN scales better.
3. **Per-node programming agent** — when service-vm-host spawns a VM, it attaches it to
   the tenant overlay on instructions from os-network-admin.
4. **Migration = peer-table update** — stable IP means os-network-admin re-programs
   AllowedIPs/forwarding on hub and hosts when a port re-binds.

**Prerequisites for tenant-visible stable IPs (none exist today):**
1. IPAM datastore with subnet pools and port records (owned by os-network-admin)
2. Ratified overlay decision (VXLAN-over-WG vs nested WG)
3. Spawn-time port allocation from fleet → os-network-admin before boot
4. Per-node network programming agent
5. Hub routing of overlay traffic between spokes

These gate on os-network-admin Phase S3 and, for full isolation, seL4 Mode B.

---

## §18. Placement Policy — Laptop B Reserved

*Implemented 2026-06-12.*

Laptop B (`laptop-b-1`, 10.8.0.1) is reserved for VP use via `VM_RESERVED=true` in
`/etc/default/vm-host` on Laptop B.

**Pass 1 — non-reserved nodes only:** Laptop A (KVM) preferred; GCP for TCG workloads.
**Pass 2 — reserved nodes (last resort):** Laptop B included only when Pass 1 finds no
node with sufficient RAM. WARN log emitted on fallback.

Implementation: `reserved: bool` field added to `NodeHeartbeat` and `NodeRecord`
(serde default false — backward compatible). `service-vm-fleet/src/placement.rs`
uses the `pick()` helper with `reserved_tier` flag.

---

## §19. Three-Path seL4 Architecture — Full Spec (2026-06-29)

### Option B — Current Path (seL4 EL2/VT-x + CAmkES VMM + Linux guest)

**What it is:** seL4 runs at EL2 (AArch64) or VT-x (x86-64). CAmkES VMM component
runs as a seL4 protection domain. CAmkES VMM hosts a Linux (Debian 12) guest VM. All
PPN services (WireGuard, service-vm-fleet, service-vm-host, Doorman, etc.) run inside
the Linux guest. seL4 kernel is the only formally verified component; VMM is unverified
userspace within seL4's capability model.

**Formal security coverage:**
- AArch64: integrity proof (April 2025). seL4 at EL2 ensures Linux guest cannot escape to
  hypervisor layer. Any two VM guests cannot communicate unless seL4 grants a channel.
- x86-64: functional correctness only. No formal escape-proof. Runtime/dev use.

**Gate to ship:** Three-node mesh test (Laptop A + foundry-workspace VM + iMac daemon).
Then upload to software.pointsav.com.

### Option C — Moonshot: seL4 PDs + Linux VM (moonshot-sel4-vmm)

**What it is:** Hybrid architecture. seL4 protection domains own WireGuard and the entire
PPN network control plane (os-network-admin logic ported to a seL4 PD). Linux VM hosts
non-critical workloads (service-vm-fleet, Doorman, etc.). The security boundary is the
WireGuard PD: the Linux VM cannot reach WireGuard state without an explicit seL4 channel.

**Why:** Network control plane is the highest-security component. Moving it to a PD means
the WireGuard kernel module is formally isolated from the rest of the system.

**What changes vs Option B:**
1. os-network-admin's WireGuard management ported from Linux userspace to seL4 PD
2. Linux VM loses network admin capability (no wg0 interface in guest)
3. seL4 IPC used for service-vm-fleet to request peer additions from the WireGuard PD

**Gate:** Option B ships and three-node mesh test passes. Then ≥6 months stability.

**Crate:** `moonshot-sel4-vmm/` (CLAUDE.md added 2026-06-29; no implementation yet).

### Option A — Moonshot: Pure seL4 PDs (moonshot-hypervisor)

**What it is:** No VMs. Every component is a seL4 protection domain. WireGuard ported
to seL4 PD (no Linux socket layer). service-vm-fleet as PD. QEMU itself either becomes
a PD or is removed (unikernel model). Smallest possible TCB.

**Formal verification target:** AArch64 EL2 with confidentiality proof (in progress as
of 2026-06-29). RISC-V64 has the deepest proofs and is an alternate target.

**What must be ported:**
1. WireGuard — IPC-based, no Linux socket layer, no net_device API
2. service-vm-fleet resource tracker
3. service-vm-host QEMU spawner (QEMU becomes a PD or is removed)
4. Minimal libc equivalent (seL4 Foundation's sel4runtime or sel4cp)

**Gate:** Option C (moonshot-sel4-vmm) ships and proves the PD model. Then ≥6 months.

**Crate:** `moonshot-hypervisor/` (CLAUDE.md added 2026-06-29; no implementation yet).

### seL4 Capability Topology — the Security Claim

seL4's security model is based on capability topology: the directed graph of capability
pointers determines what component can call what other component. The invariant
"only connectivity begets connectivity" (Miller 2000) is formally proved. This means:
the topology of the capability graph is an upper bound on information flow.

Use "topology" not "geometry" — "topology" is the established term in the academic
literature (Miller 2000, Drossopoulou 2016, Fuchsia "component topology" as industrial
prior art). No peer-reviewed paper uses "geometry" for this purpose.

**Commercial claim (AArch64 EL2 only):**
"Security is determined by the topology of the capability graph — who can call what.
Components without an explicit capability route cannot exchange information. This is a
formal proof, not a policy."

This claim is only valid for AArch64 EL2 deployments with the integrity proof applied.
Do NOT use this claim for x86-64 deployments (functional correctness only).

---

## §20. Competitor Landscape (2026-06-29)

### Kry10 OS

- seL4 microkernel + BEAM/Erlang application platform
- Founded: New Zealand; $6M raised January 2024 (investors include In-Q-Tel)
- Kent McLeod, VP Engineering — former seL4 Technical Lead at CSIRO/TSC
- Target: critical infrastructure (industrial, defense)
- Proprietary — not open source. No public pricing.
- Relevant as: closest technical competitor (seL4 + managed runtime for services)
- PointSav differential: open-source kernel, public pricing ($1/$19 USDC), WireGuard mesh
  as the network model vs. Kry10's BEAM message passing

### NIO ONVO L60

- SkyOS-M (seL4-based automotive OS) in mass production as of September 2024
- Chinese EV market; NIO (NASDAQ: NIO) subsidiary ONVO brand
- First mass-produced consumer product built on seL4 formally verified kernel
- Relevant as: proof that seL4 is production-viable at consumer scale

### seL4 Foundation

- Non-commercial governance body for seL4 kernel
- AArch64 EL2 integrity proof funded by UK NCSC (April 2025)
- Microkit 2.2.0 is the Foundation's recommended programming model
- No commercial products; Foundation exists to maintain the proofs and tooling

---

## §21. software.pointsav.com Distribution Model (2026-06-29)

### Three artifacts per product per version

| Artifact | Format | Use case | Notes |
|---|---|---|---|
| `.iso` | ISO 9660 + El Torito + GRUB2 multiboot | Bare metal (USB write + boot) | Boot chain: GRUB2 → seL4 → CAmkES VMM → Linux guest |
| `.qcow2` | QCOW2 (QEMU native) | Cloud VM import (GCP raw import, DigitalOcean, etc.) | Same kernel binary; `platform=` boot arg selects behavior (Talos Linux pattern) |
| Daemon AppImage / `.deb` | ELF binary (Linux); later: Windows .exe, macOS .pkg | Existing Linux (daemon mode, no seL4 boot) | `os-network-admin` primary distribution; compiled from same Rust source with `--features daemon` |

### Pricing — BETA (current)

| Product | Price | Notes |
|---|---|---|
| `os-network-admin` | FREE (BETA) | Payment disconnected; public CLI download |
| `os-infrastructure` | FREE (BETA) | Payment disconnected; ISO/QCOW2 when built |

Paid listing ($1 USDC / $19 USDC) enabled by operator after D7 mesh test passes and
operator explicitly approves. Payment reconnect is a deliberate operator action — it does
not happen automatically at D7.

**Source code:** Available free on GitHub (open source). Future paid listing is for the
pre-built Ed25519-signed binary distribution.

### BETA upload policy

Upload binaries to software.pointsav.com as soon as they build and sign — do NOT wait
for the D7 three-node mesh test. The BETA listing is the proof that the software exists.

Rationale: investors, bankers, and technical reviewers must be able to download and inspect
the binary independently. A "coming soon" page is not the same as an actual downloadable
binary. Upload early; upgrade to paid after D7.

project-software listing requirements for BETA:
- Label: "BETA — Free Download" (not a price or payment button)
- Payment: disconnected (no USDC modal, no wallet connection required)
- Download: public URL, no token, no account required
- Version tag: `0.1.0-beta.1` (or current semver + `-beta.N`)

### CLI install — primary download method

Users and reviewers download via `curl`. No browser modal, no payment during BETA.

**os-network-admin daemon (Linux x86-64):**
```bash
curl -fL https://software.pointsav.com/download/os-network-admin/beta/x86_64 \
  -o os-network-admin
chmod +x os-network-admin
sudo ./os-network-admin
```

**Verify Ed25519 signature:**
```bash
curl -fL https://software.pointsav.com/download/os-network-admin/beta/x86_64.sig \
  -o os-network-admin.sig
# Verify with PointSav signing key published at software.pointsav.com/signing-key
```

**os-infrastructure ISO (bare metal, when built):**
```bash
curl -fL https://software.pointsav.com/download/os-infrastructure/beta/x86_64.iso \
  -o os-infrastructure.iso
# Write to USB: sudo dd if=os-infrastructure.iso of=/dev/sdX bs=4M status=progress
```

**Target install experience:** ≤3 operator decisions (node name, genesis endpoint,
pairing code). Complexity beyond those 3 decisions is product scope reduction work.

### project-software mandate — page structure + all-project catalog

project-software is responsible for building out software.pointsav.com to accommodate
binaries from all projects, not just this archive. When project-infrastructure sends
its handoff outbox message, project-software should:

1. **Build the full product catalog page structure** — one product card per binary,
   organized by project. Initial catalog from this archive:
   - `os-network-admin` — PPN mesh control plane daemon (AppImage, Linux x86-64)
   - `os-infrastructure` — PPN node OS (ISO, QCOW2, x86-64) — when built

2. **URL convention** — `software.pointsav.com/download/<product>/<channel>/<artifact>`:
   - `channel`: `beta` (current) → `v1`, `latest` (future)
   - `artifact`: `x86_64`, `x86_64.iso`, `x86_64.qcow2`, `x86_64.sig`

3. **Each product listing page** must include:
   - Product name + one-line description
   - BETA badge (while payment is disconnected)
   - CLI download command (pre-filled `curl` one-liner)
   - Ed25519 signature verification command
   - System requirements (e.g. CAP_NET_ADMIN, WireGuard kernel module)
   - Version + build date
   - Link to GitHub source

4. **Binaries from other archives** — project-software will receive separate outbox
   messages from other archives (e.g. app-workplace-*, app-orchestration-*) as they
   produce distributable binaries. Each message will follow the same format as the
   project-infrastructure handoff (see handoff pipeline below).

5. **No self-service uploads** — project-software does not pull binaries directly from
   foundry-workspace. All uploads are triggered by explicit outbox messages from the
   originating archive.

### project-software handoff pipeline (this archive)

When a binary is ready for software.pointsav.com:

1. **Build + sign** (project-infrastructure, this archive):
   - `cargo build --release --features daemon`
   - Sign with `identity/id_pointsav-administrator` Ed25519 key

2. **Send outbox message to project-software** with:
   - Binary path on foundry-workspace
   - Ed25519 signature file path
   - Version string (e.g. `0.1.0-beta.1`)
   - Explicit instruction: BETA listing, payment disconnected, CLI download URL template
   - System requirements (CAP_NET_ADMIN, WireGuard module, x86-64 Linux)

3. **project-software receives** and:
   - Uploads binary + signature to software.pointsav.com asset storage
   - Publishes BETA listing: "BETA — Free Download", no payment modal
   - Adds product card to the catalog page
   - Confirms CLI URL works: `curl -fL https://software.pointsav.com/download/...`

4. **Later — paid listing (separate operator approval after D7)**:
   - project-infrastructure sends second outbox to project-software
   - project-software enables USDC modal at $1 USDC (os-network-admin) / $19 USDC (os-infrastructure)
   - Operator reviews and approves before enabling — this does NOT happen automatically

### Gate to paid listing (D7 mesh test)

The D7 gate applies to the **paid listing** only. BETA binaries upload before D7.

Three-node mesh test (D7):
1. Laptop A: os-infrastructure ISO boot (VT-x bare metal) → peer registers in fleet
2. foundry-workspace: os-infrastructure QCOW2 under QEMU/TCG → peer registers
3. iMac Linux Mint: os-network-admin daemon → peer registers

All three peers visible in `service-vm-fleet` → operator approves → project-software
enables USDC payment at $1 / $19.

---

## Cross-References

- `BRIEF-totebox-transformation.md` — active work: live fleet state, open items
- `BRIEF-PPN-ARCHITECTURE.md` — hypervisor TCB, seL4 proof, distributed fabric design
- `BRIEF-sovereign-os-family-master-plan.md §R–§W` — governance layer
- `BRIEF-PPN-DEV-BOOTSTRAP.md §11` — Session 7 Q&A
- `infrastructure/virt/vm-prove.sh` — proven hypervisor foundation
- `infrastructure/virt/work/ppn-prove.qcow2` — proof QCOW2 (gitignored)
