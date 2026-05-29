---
artifact: brief
name: BRIEF-PPN-DEV-BOOTSTRAP
status: active
created: 2026-05-28
engine: claude-code
session: totebox@project-infrastructure
synthesises:
  - BRIEF-PPN-ARCHITECTURE.md (project-infrastructure)
  - BRIEF-sovereign-os-family-master-plan.md (workspace)
  - BRIEF-totebox-ppn-infrastructure-master-plan.md (workspace; superseded)
  - BRIEF-dev-environment-hardening-2026-05-18.md (workspace)
description: >
  Operational BRIEF for activating the first live PPN ceremony on the existing
  WireGuard mesh. Documents the dogfood principle, current topology, role mapping,
  service deployments, virtualization proof-of-concept, and migration path.
---

# PPN Dev-Environment Bootstrap

**The first PointSav Private Network is the development environment itself.**

---

## 1. The Dogfood Principle

The three machines on the WireGuard mesh Part A-lite are not just connected — they
*are* the first PPN deployment. Building PPN on PPN is intentional: the development
environment is the proving ground. Every piece of software shipped to future customers
runs here first. The iMac runs `os-network-admin`. The GCP VM hosts `service-ppn-pairing`.
Laptop B is the WireGuard routing hub. The first node-join ceremony that runs in this
environment is the proof that the architecture works.

This mirrors the Totebox Orchestration principle: COMMAND runs the same session tooling
that customers will use. The operator using the PPN is the same person building it.

---

## 2. Current Topology — Part A-lite (live as of 2026-05-23)

Ground truth is `infrastructure/wireguard/topology.yaml`. Authoritative peer list:

| Machine | PPN role | WireGuard IP | Target IP (10.42) | Status |
|---|---|---|---|---|
| **Laptop B** | route-network-admin-1 (WireGuard hub) | 10.8.0.1 | 10.42.0.1 | live; 24.86.192.209:51820 |
| **GCP VM** (foundry-workspace) | fleet-infrastructure-cloud-1 | 10.8.0.9 | 10.42.10.1 | live |
| **Laptop A** (iMac / Linux Mint) | station-workplace-mathew-1 | 10.8.0.6 | 10.42.20.2 | live |
| **Jennifer** (macPro) | station-workplace-jennifer-1 | 10.8.0.5 | 10.42.20.1 | pending (key rotated 2026-05-23) |

The three live machines can already reach each other. SSH from Laptop A:
```
ssh mathew@10.8.0.9    # GCP VM
ssh mathew@10.8.0.1    # Laptop B
```

---

## 3. Role Mapping — Dev Environment as PPN Deployment

Two-layer architecture per BRIEF-sovereign-os-family-master-plan.md:

```
OPERATOR SURFACE
────────────────────────────────────────────────────
  station-workplace-mathew-1 [Laptop A / iMac, 10.8.0.6]
    ├── os-network-admin  (Foundation OS layer — PPN control plane)
    │     Zero cryptographic authority. Routing + tunnel integrity.
    │     Polls service-ppn-pairing; operator approves node-join codes.
    └── app-network-admin  (F8 Terminal interface on top of os-network-admin)
          HTTP :8085 (intent → service-slm → authorised command)
          UDP  :8090 (signed 16-byte binary broadcast to PPN peers)

TOTEBOX ORCHESTRATION LAYER (MBA pairings — data plane)
──────────────────────────────────────────────────────
  gateway-orchestration-command-1 [GCP VM — co-tenant with fleet-cloud-1]
    └── cluster-totebox-* / media-* / vault-privategit-* / node-console-*
    Uses PSP (PointSav Protocol) — capability-based binary over TLS.
    Stateless data aggregator. Holds no keys to archives.

POINTSAV PRIVATE NETWORK LAYER (WireGuard — physical transport plane)
──────────────────────────────────────────────────────────────────────
  route-network-admin-1 [Laptop B — WireGuard hub, 10.8.0.1]
    ├── fleet-infrastructure-cloud-1 [GCP VM] (10.8.0.9 → target 10.42.10.1)
    └── station-workplace-mathew-1  [Laptop A] (10.8.0.6 → target 10.42.20.2)

HYPERVISOR LAYER (os-infrastructure — not yet compiled)
───────────────────────────────────────────────────────
  Each physical node manages a pool of VMs.
  Resource pool: virtio_balloon (memory) + vCPU scheduling (cgroups v2 cpu.weight).
  Gives more / less CPU + RAM to each VM on demand. Per-node pool, not cross-node.
```

**`os-network-admin` runs on Laptop A.** Foundation OS layer — PPN control plane.
Zero cryptographic authority. It polls `service-ppn-pairing` on the GCP VM and shows
pending node-join requests. The operator approves new nodes from Laptop A; approved
WireGuard public keys are written to `nodes.jsonl` on the GCP VM.

**`app-network-admin`** is the F8 Terminal interface that runs on top of `os-network-admin`.
It provides an HTTP command surface (port 8085) and UDP mesh broadcast (port 8090) for
translating operator intent into signed mesh commands via `service-slm`.

**`route-network-admin`** is the deployment instance name for the network admin node in the
customer fleet — not a separate codebase.

**Governance gap (from sovereign-os-family master plan §4):** `route-network-admin-1` is
the only sovereign component currently absent from `pairings.yaml`. The WireGuard hub has
no MBA pairing to the orchestration gateway — it is governance-orphaned. This must be
closed before Part A proper.

---

## 4. What service-ppn-pairing Provides

Crate: `service-ppn-pairing/` (committed 2026-05-27, deployed as `local-ppn-pairing.service`)
Default listen: `0.0.0.0:9202` (accessible from all WireGuard peers)
Database: `~/.local/share/ppn/ppn-pairing.db` (SQLite; auto-migrates)
Registry: `~/.local/share/ppn/nodes.jsonl` (append-only; one JSON line per approved node)

API:

| Method | Path | Purpose |
|---|---|---|
| `POST` | `/v1/node-join/request` | Node submits node_id + wireguard_pubkey + bottom + arch; gets back code + expires_at |
| `GET` | `/v1/node-join/pending` | Operator polls for pending approvals |
| `POST` | `/v1/node-join/approve` | Operator approves a code; writes to nodes.jsonl |
| `POST` | `/v1/node-join/deny` | Operator denies a code |
| `GET` | `/v1/node-join/status/{id}` | Node polls its own request state |

Code format: Crockford base32 XXXX-XXXX (40-bit entropy, 600s TTL). Confusable characters
normalised: I→1, L→1, O→0.

When a node is approved, `register_approved_node()` appends to `nodes.jsonl`:
```json
{"node_id":"...","wireguard_pubkey":"...","bottom":"netbsd-compat","arch":"x86_64","request_id":"...","approved_at":"..."}
```
This is the Phase 1 registry. Phase 2 (Genesis Protocol) will read `nodes.jsonl` and
generate WireGuard peer entries automatically.

---

## 5. os-network-admin on Laptop A

Crate: `os-network-admin/` — minimal poll mode (deferred: ratatui TUI with QR display)

Current implementation: polls `$PAIRING_SERVER/v1/node-join/pending` every 5 seconds;
prints pending requests to stdout. Operator approves via curl.

```bash
# On Laptop A, after copying the binary:
PAIRING_SERVER=http://10.8.0.9:9202 ./os-network-admin
```

To approve a pending node-join (from any WireGuard peer):
```bash
curl -s -X POST http://10.8.0.9:9202/v1/node-join/approve \
     -H 'Content-Type: application/json' \
     -d '{"code":"XXXX-XXXX"}'
```

**Deferred (NEXT.md):** ratatui TUI with keyboard approve/deny (`a`/`d`), QR rendering
via `system-pairing-codes::qr_unicode`, expiry countdown. This is the full §9.2 Step 4
of the BRIEF-PPN-ARCHITECTURE.md build order.

---

## 6. Virtualization Proof-of-Concept

The thesis claim (BRIEF-PPN-ARCHITECTURE.md §1.1 Contribution #3): "sub-five-minute SMB
deployment" and "formally-isolated VMs." The development environment proves the concept
using KVM/QEMU (where available) as the compat-bottom stand-in.

### What a VM is in this architecture

Each VM provisioned by the PPN is a **sovereign execution environment** for one
Totebox Archive (`cluster-totebox-corporate-1`, `-personnel-1`, `-property-1`) or one
gateway node (`gateway-orchestration-command-1`). The VM:

- Packages the Totebox Archive as a **bootable disk image** — freely transferable between
  physical nodes, clouds, or bare metal without losing integrity or historical context
- Cannot see the PPN's WireGuard keys, nodes.jsonl, or the node-join ceremony
- Cannot see other VMs on the same physical host
- Presents an MBA keypair to connect to `gateway-orchestration-command-1` via PSP
- Does not know which physical node or hypervisor it runs on

This is the isolation invariant from BRIEF-PPN-ARCHITECTURE.md §1.1 Contribution #2.

### What os-orchestration does (data layer — separate from the VM/PPN layers)

`os-orchestration` is stateless. It aggregates **data access** across Totebox Archives
via the PointSav Protocol (PSP) — a capability-based binary protocol over TLS. It:

- Sends signed capability objects (read permission for a specific record, fixed time window)
- Toteboxes verify and emit only query results — never raw records
- Holds no keys to archives; if compromised, the underlying Toteboxes remain sealed

**os-orchestration does not manage compute resources.** It does not schedule VMs, allocate
CPU, or move RAM between nodes. That is the hypervisor layer's job.

### Resource pool management (PPN hypervisor layer)

The PPN hypervisor (`os-infrastructure`) manages a per-node pool of CPU and RAM:

- **virtio_balloon** — the balloon controller inflates the balloon in a VM (guest gives RAM
  back to the host pool) or deflates it (guest receives more RAM). Per BRIEF §9.4.
- **vCPU scheduling** — cgroups v2 `cpu.weight` per QEMU process; contended vCPUs
  distributed proportionally to weight table in the capability ledger.
- **Scope is per-node** — each physical node manages its own pool. Cross-node workload
  placement is `gateway-orchestration-command-1`'s job (MBA-authorized scheduling).

### TCB delta — what each hypervisor proves

| Hypervisor | Isolation claim | Formal proof | Status |
|---|---|---|---|
| QEMU/KVM (Linux) | Process isolation + EPT | None | **Used for proof today on Laptop A** |
| NetBSD/bhyve | EPT isolation; VeriExec load-time integrity | Argued (not proved) | compat bottom target |
| seL4 (AArch64) | Machine-checked IFC; intransitive non-interference | Murray et al. 2013 | native bottom target (moonshot-kernel) |

### KVM availability

- **Laptop A (iMac / Intel Sandy Bridge)**: VT-x present; KVM available on Linux Mint.
  Use `vm-prove.sh` without `--tcg`.
- **GCP VM (foundry-workspace)**: runs AS a KVM guest; nested virtualisation NOT enabled
  by default. Use `vm-prove.sh --tcg` (QEMU TCG). KVM can be enabled:
  `gcloud compute instances update foundry-workspace --enable-nested-virtualization`.

**Proof script:** `infrastructure/virt/vm-prove.sh`

Includes `-device virtio-balloon` — installs the balloon driver in the guest. From the
QEMU monitor (`Ctrl-A c`): `balloon 128` inflates the balloon (reclaims RAM to host pool);
`info balloon` shows current allocation. This is the first live proof of PPN resource pool
management.

**KVM availability:**
- **Laptop A (iMac / Intel Sandy Bridge)**: VT-x present; KVM available on Linux Mint
  (`/dev/kvm` expected). Use `vm-prove.sh` without `--tcg`.
- **GCP VM (foundry-workspace)**: runs AS a KVM guest; nested virtualisation NOT enabled
  by default on e2 instances. Use `vm-prove.sh --tcg` (QEMU TCG, slower but proves the
  concept). To enable KVM on the GCP VM: stop instance → `gcloud compute instances update
  foundry-workspace --enable-nested-virtualization` → restart.

**Proof script:** `infrastructure/virt/vm-prove.sh`

What it does:
1. Detects KVM availability (`/dev/kvm`)
2. Downloads Alpine Linux virt ISO (~50 MB, single download cached in `work/`)
3. Boots a 256 MB VM with virtio NIC
4. Forwards host port 10202 → VM port 9202
5. Operator installs `service-ppn-pairing` binary in the VM, starts it
6. From Laptop A: `curl http://10.8.0.9:10202/v1/node-join/pending` — proves a
   Totebox service is reachable inside a VM through the PPN mesh

This is "Totebox Orchestration using a VM": the VM IS the cluster-totebox host.
When `os-infrastructure` compiles, KVM is replaced by the bhyve/seL4 hypervisor.
The ceremony and mesh provisioning are unchanged.

---

## 7. Deployment Sequence (unblocked right now)

### Step 1 — Deploy service-ppn-pairing on GCP VM

```bash
# On GCP VM (10.8.0.9):
cd /srv/foundry/clones/project-infrastructure
cargo build --release -p service-ppn-pairing
sudo cp target/release/service-ppn-pairing /usr/local/bin/
sudo cp infrastructure/systemd/local-ppn-pairing.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now local-ppn-pairing
systemctl status local-ppn-pairing
```

### Step 2 — Verify from Laptop A

```bash
# From Laptop A (10.8.0.6) over WireGuard:
curl http://10.8.0.9:9202/v1/node-join/pending
# Expected: {"pending":[]}
```

### Step 3 — Build and copy os-network-admin to Laptop A

```bash
# On GCP VM:
cargo build --release -p os-network-admin
scp target/release/os-network-admin mathew@10.8.0.6:~/bin/
```

### Step 4 — Run os-network-admin on Laptop A

```bash
# On Laptop A:
PAIRING_SERVER=http://10.8.0.9:9202 ~/bin/os-network-admin
```

### Step 5 (optional proof) — Run vm-prove.sh on Laptop A

```bash
# On Laptop A (real hardware, KVM available):
cd /path/to/project-infrastructure
./infrastructure/virt/vm-prove.sh
# Boot Alpine, install service-ppn-pairing in VM, verify port 10202
```

---

## 8. Address Plan Migration Path

The 10.8.0.0/24 range is Part A-lite (manual WireGuard setup, 2026-05-23). The canonical
address plan from BRIEF-sovereign-os-family-master-plan.md §B is **10.42.0.0/16**.

Migration is gated on operator subnet ratification (NEXT.md Q2). When confirmed:

| Step | Change |
|---|---|
| Update topology.yaml | `ppn_ip` → `target_ppn_ip` values; already populated |
| Update Laptop B wg0.conf | New AllowedIPs ranges; regenerate peer configs |
| Update app-network-admin | Replace hardcoded `10.50.0.x` with `10.42.x.x` |
| Update os-network-admin | Replace default `10.8.0.9` with `10.42.10.1` |
| Update INVENTORY.yaml | Fill all node assignments |

The `target_ppn_ip` fields in `topology.yaml` are already correct:
- Laptop B: 10.42.0.1 (routing hub)
- GCP VM: 10.42.10.1 (fleet-cloud)
- Laptop A: 10.42.20.2 (station-workplace-mathew)

---

## 9. Connection to COMMAND's Totebox Transformation

COMMAND is migrating from master/root session roles to Command/Totebox vocabulary (Sessions
8–24, 2026-05-17 to 2026-05-27). The Totebox Orchestration layer (MBA pairings) and the
PPN layer (WireGuard) are the two-layer architecture that governs all production deployments.

**Three open items for COMMAND to close before Part A proper:**

1. **MBA governance gap:** `route-network-admin-1` (Laptop B) is absent from `pairings.yaml`.
   Every other sovereign component has an MBA entry. This must be added as an admin-tier
   commit to `pointsav-monorepo`.

2. **Subnet ratification (Q2):** Confirm 10.42.0.0/16. The 10.8.0.0/24 Part A-lite range
   is a transitional stepping stone — the 10.42.0.0/16 plan is ratified in principle by the
   sovereign-os-family BRIEF; it needs an operator acknowledgement commit.

3. **DNS resolution (Q4):** `network.woodfinegroup.com` DNS record was deleted 2026-05-26.
   The Guide references it. Either restore the record or update the Guide to use a direct IP.

---

## 10. What Remains for Genesis Protocol

This BRIEF describes the "activate what we have" phase. The full Genesis Protocol
(BRIEF-PPN-ARCHITECTURE.md §5) requires:

| Step | Crate | Notes |
|---|---|---|
| 1 | `os-infrastructure` rewrite | blind boot → mDNS → short-code ceremony; **does not compile** |
| 2 | `system-substrate-broadcom` | silicon_ping(); Broadcom 14e4:16b4 PCI detection |
| 3 | `system-network-interface` | WireGuard/mDNS substrate |
| 5 | `app-network-admin` | Replace system-slm subprocess with Doorman HTTP |
| 6 | `app-network-admin` | Replace JSON mesh payloads with 16-byte binary |

Steps 1–3 are gated on Q2 (subnet ratification). Step 5 is gated on Q5 (Doorman deployed).

---

---

## 11. Session 7 Research: Infrastructure Questions + Distributed VM Fabric (2026-05-28)

Three operator questions were researched and a distributed VM fabric architecture was
synthesised. Results recorded here; full editorial output staged as TOPIC/GUIDE drafts.

### Q1 — Do Laptop A and Laptop B need to be rebooted to pool their resources?

**No.** Standard pool operations are dynamic:
- `virtio_balloon` inflation/deflation: the QEMU monitor signals the in-guest driver,
  the driver responds, the pool adjusts — no guest or host reboot.
- `cgroups v2 cpu.weight` changes take effect on running QEMU processes immediately.

**What IS blocking (deployment work, not reboots):** `service-ppn-pairing` not yet
deployed on the GCP VM; `os-network-admin` not yet installed on Laptop A; node-join
ceremony not yet run on real hardware. These are §7 deployment steps, not OS changes.

### Q2 — Do we have a test VM running?

**Proof complete; no persistent VM is running.** `infrastructure/virt/vm-prove.sh` ran
2026-05-28 (commit `04388865`): Alpine Linux 3.20 booted via QEMU TCG in 114 seconds;
`virtio_balloon` inflation/deflation confirmed. The QCOW2 image exists; the VM is a
one-shot proof script, not a persistent systemd service.

### Q3 — Feasibility of one VM per project and one VM per deployment folder

**Not feasible at per-project granularity; per-cluster is the right target.**

| Scope | Count | Min RAM | Verdict |
|---|---|---|---|
| Per source project | 116 | 464 GB | Infeasible (32 GB available) |
| Per deployment instance | 18 | ~72 GB | Right unit for capacity planning |
| Per logical cluster | 9 | ~36–72 GB | Intended next scale tier |

The right unit is the **running deployment instance** (18 today), not the source project
(116, most of which are dormant scaffold-coded crates).

### Q4 — Most sophisticated distributed virtualization we can build; leapfrog 2030

**Four components planned above the proven per-node layer:**

1. **virtio-mem lending over WireGuard** — analogous to CXL 3.0 (PCIe memory
   disaggregation) but works over any encrypted network including WAN. seL4 capability
   model ensures lending node retains no read capability over lent memory blocks.
   Reserved: `moonshot-network/`.

2. **Distributed capability ledger** — HMAC-signed grants keyed to pairing-ceremony
   identity; Merkle DAG gossip across the WireGuard mesh; intended sub-second revocation
   without central authority (vs. IAM 10–60 sec centrally propagated).
   Reserved: `moonshot-protocol/`, `moonshot-database/`.

3. **Cross-node VM scheduler** — deterministic bin-packing; QEMU live migration over
   WireGuard; sovereignty constraint (operator can pin VMs to specific trusted nodes).
   Reserved: `os-orchestration/`.

4. **Sovereign attestation chain** — `dm-verity` root filesystem hash anchored to
   pairing-ceremony key; no TPM vendor, no silicon vendor in chain. Intel TDX / AMD
   SEV-SNP require the silicon vendor's CA; PPN sovereignty makes the operator the
   attestation root.

**What this leapfrogs by 2030:** machine-checked formal isolation proof on SMB hardware
(AWS/Azure/GCP have none); cross-node memory lending over WAN (CXL requires PCIe fabric);
sub-second decentralised capability revocation; operator-sovereign attestation without a
cloud vendor; sub-five-minute SMB deployment. None of these will be shipped by major cloud
providers by 2030 due to backward-compatibility constraints and the cloud-tenant threat
model they are optimised for.

**Editorial output staged:** two new TOPIC draft pairs in `.agent/drafts-outbound/`:
- `topic-ppn-distributed-vm-fabric.draft.md` + `.es` (new full TOPIC)
- `topic-ppn-hypervisor-resource-pool.draft.md` updated: §3 "Planned: cross-node resource
  extension" added
- `topic-ppn-architecture-overview.draft.md` updated: distributed fabric paragraph in
  hypervisor layer section
- `guide-ppn-first-deployment.draft.md` updated: VM capacity planning table added

---

---

## 12. seL4 First-Boot Path — Phase 1C.d Achievement and Gap to os-mediakit

*(Added 2026-05-29 — cross-ref: BRIEF-totebox-transformation §9)*

### What project-system delivered (Phase 1C.d, moonshot-toolkit v0.3.0)

`/srv/foundry/clones/project-system/moonshot-toolkit` produces a bootable AArch64 seL4
system image using a pure-Rust assembler (no Python, no CMake in the critical path):

- **CPIO writer** (`src/cpio.rs`) packs `kernel.elf` + `kernel.dtb` + rootserver ELF into
  a CPIO archive that the seL4 elfloader extracts at boot
- **System specification** (`system-spec.toml`) declares Protection Domains in a
  Microkit-shaped TOML format (≤63 PDs, ≤63 channels/PD, `PPC` vs `Notification` distinction)
- **Boot:** `qemu-system-aarch64 -machine virt,secure=off -cpu cortex-a53 -m 1G -nographic
  -kernel build/system-image.bin` → seL4 boots to user space ("hello from seL4 rootserver")
- **Architecture:** AArch64, `qemu-arm-virt` platform, `KernelDebugBuild=ON`,
  `KernelPrinting=ON`. 35 unit/integration tests pass.

The rootserver is currently `examples/hello.c` — a bare-metal halt loop proving the image
assembly pipeline. It is NOT a Microkit root task (no libmicrokit, no `init`/`notified` entry
points) and NOT using the Microkit Python SDK.

### The gap: AArch64 kernel blob → os-mediakit guest OS

| Gap | Detail |
|---|---|
| Arch mismatch | Phase 1C.d produces AArch64; GCP workspace is x86_64. Cannot replace Debian 12 QCOW2 directly. |
| Rootserver is hello.c | Needs to become os-mediakit Rust binary with real services |
| Rust PD compilation | moonshot-toolkit only compiles `.c` PDs; needs `cargo build --target aarch64-unknown-none` branch |
| system-substrate-sel4 absent | BRIEF §5.3 shim crate (feature flags `["native"]`/`["compat"]`) not yet created |
| os-mediakit scaffold only | `os-mediakit/src/lib.rs` is a 1-function stub in both project-system and project-infrastructure |
| x86_64 QEMU path absent | moonshot-toolkit has no Multiboot2 assembler; GRUB/QEMU x86 path would be new build track |

### Ordered steps for project-system to deliver os-mediakit Phase 1

1. Wire `os-mediakit/` as workspace member with `system-spec.toml` — single PD `mediakit-root`
2. Convert `os-mediakit/src/` to AArch64 `#![no_std] #![no_main]` Rust `_start` — halt loop
   with `SysDebugPutChar` printing "os-mediakit booted" validates the pipeline
3. Extend `moonshot-toolkit::cmd_build` to invoke `cargo build --target aarch64-unknown-none`
   for Rust PDs (currently only `aarch64-linux-gnu-gcc` for `.c`)
4. Boot: `moonshot-toolkit build os-mediakit/system-spec.toml` → verify QEMU output
5. Create `system-substrate-sel4` shim crate — even a stub with `seL4_DebugPutChar` is enough
6. Phase 1C.e: Sigstore cosign on `plan_hash` (already queued in moonshot-toolkit NEXT.md)
7. Handoff artifact to project-infrastructure: `build/system-image.bin` + note that
   `infrastructure/os-infrastructure/forge_iso.sh` is wrong toolchain (GRUB/x86, non-existent
   paths) — replace with moonshot-toolkit AArch64 build path

**Stretch (only if x86_64 is non-negotiable for Phase 3):**
Rebuild vendor-sel4-kernel pc99 with `KernelPrinting=ON`; add `AssembleMultibootImage` variant
to moonshot-toolkit; this enables the Debian 12 QCOW2 replacement path on x86_64 GCP.
Estimated: significant new build track. Recommendation: prefer AArch64 GCP C4A instance.

### Microkit x86_64 status (confirmed by internet research 2026-05-29)

Microkit 2.2.0 supports **AArch64 and RISC-V 64 only**. x86_64 seL4 requires raw seL4 + CAmkES
(not Microkit). No known production seL4 deployments on x86_64 in 2025-2026. Commercial
momentum (Cog/Riverside Research, NASA cFS, Collins flight vehicle) is AArch64-dominant.
The seL4 Foundation advises small teams: incremental cyber-retrofit, AArch64 bare metal first.

---

*End of BRIEF — project-infrastructure / 2026-05-28 (§12 added 2026-05-29)*
*Activating the first ceremony: service-ppn-pairing on GCP VM + os-network-admin on Laptop A*
