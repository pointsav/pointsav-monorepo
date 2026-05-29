---
artifact: brief
status: active
created: 2026-05-29
author: totebox@project-infrastructure
---

# BRIEF-VM-ARCHITECTURE — VM-* OS Family and Deployment Model

> Durable planning artifact. Never delete. Supersede by editing `status: archived`.
> Gate document for all VM provisioning, os-* binary work, and unikernel roadmap decisions.

---

## 1. VM-* to os-* Canonical Mapping

Each `VM-*` is the runtime identity. The `os-*` crate name is the source identity. One-to-one correspondence — the VM-* name is what runs; the os-* binary is what compiles.

| VM Type | os-* Source | Services / Apps | Deployment Model |
|---|---|---|---|
| **VM-Totebox** | `os-totebox` | service-fs (WORM) · service-people · service-extraction · Ring 1+2 services | Per-entity; on-prem, leased, or cloud VM |
| **VM-MediaKit** | `os-mediakit` | proofreader · knowledge-docs/corp/projects · marketing-pointsav · marketing-woodfine | GCP host; current vm-mediakit QEMU instance |
| **VM-Orchestration** | `os-orchestration` | app-orchestration-bim · app-orchestration-gis · app-orchestration-slm | Cloud VM; stateless multi-archive aggregator |
| **VM-PrivateGit** | `os-privategit` | app-privategit-source-control · app-privategit-design-system (Gitea + Storybook) | On-prem or cloud |
| **VM-Infrastructure** | `os-infrastructure` | WireGuard PPN fabric · Genesis Protocol · hypervisor + VM placement | 3-node trust mesh (see §3) |

Dogfood alignment: the platform is developed the same way customers experience it.
- PointSav Private Network users → VM-Infrastructure (PPN fabric)
- Totebox Orchestration users → VM-Totebox (data vault)
- Independent Systems users → VM-MediaKit (websites) or VM-PrivateGit (source control)

---

## 2. Placement Principle (Architecture Rule)

> **A service belongs in the VM whose `os-*` namespace owns its data lifecycle and trust boundary — not the VM where its binary first ran.**

Derivations:
- `service-fs` (WORM ledger) → VM-Totebox. It IS the Totebox storage substrate.
- `app-orchestration-bim` → VM-Orchestration. Its name declares it. It is a multi-archive aggregator.
- `app-mediakit-knowledge` → VM-MediaKit. Stateless public web appliance.
- Proofreader (`service-proofreader`) → VM-MediaKit. Stateless transform; its callers are MediaKit-resident. Moving it to VM-Totebox forces every keystroke across the PPN — that is the smell.
- `app-privategit-source-control` → VM-PrivateGit. Sovereign source control is its own trust domain.
- WireGuard / Genesis Protocol / pairing ceremony → VM-Infrastructure. These are fabric concerns.

If a service "needs" loopback co-location with a service in a different VM, that is a design smell. The PPN boundary is where services communicate. Co-location pressure dissolves at the PPN boundary.

---

## 3. VM-Infrastructure: 3-Node Trust Mesh (Not a Scheduler)

VM-Infrastructure is not a resource-pooling cluster scheduler. It is a **trust-meshed host fleet**: three independently-provisioned nodes, each running the same `os-infrastructure` binary, connected by WireGuard PPN.

```
Laptop A (iMac 12,1) — genesis-seed node
  provision-vm-infrastructure-onprem.sh --genesis
  Hosts: VM-Totebox-1 (first Totebox archive instance)
  Constraint: no VT-d → TCG-only (10x slower; acceptable for genesis-seed)

Laptop B             — WireGuard hub/relay
  provision-vm-infrastructure-onprem.sh --join <short-code>
  Hosts: PPN relay + optional VM-Orchestration (if KVM available)

GCP VM               — cloud relay + web-facing endpoint
  provision-vm-infrastructure-cloud.sh --join <short-code>
  Hosts: VM-MediaKit · VM-Orchestration · VM-PrivateGit
```

### Genesis Protocol bootstrap (two-question)

1. Is this the first node? → `--genesis` flag: self-configures WireGuard, seeds key, opens short-code ceremony server.
2. What is the address of the existing network? → `--join <Crockford-base32-short-code>`: CPace PAKE + SAS confirmation.

No single fabric.yaml — each node-class has its own provision script because genesis vs. joiner differ, and cloud vs. on-prem differ in network topology. This is intentional.

### Per-node provision script naming

| Node | Script |
|---|---|
| Laptop A / on-prem genesis seed | `infrastructure/virt/provision-vm-infrastructure-onprem.sh --genesis` |
| Laptop B / on-prem joiner | `infrastructure/virt/provision-vm-infrastructure-onprem.sh --join <code>` |
| GCP cloud joiner | `infrastructure/virt/provision-vm-infrastructure-cloud.sh --join <code>` |
| Future leased/Equinix | `provision-vm-infrastructure-leased.sh` (future stub) |

---

## 4. Unikernel Roadmap

Long-term: each `os-*` becomes a binary — either a lightweight BSD image or a unikernel. Customers deploy the same binary that the development environment runs. This table tracks the evolution path per VM type.

| VM | Phase 1 (now) | Phase 2 | Phase 3 |
|---|---|---|---|
| **VM-Totebox** | Ubuntu 24.04 QEMU (TCG/KVM) | Alpine Linux + musl-static binary | NanoVMs/OPS unikernel or seL4 Microkit (AArch64-only) |
| **VM-MediaKit** | Ubuntu 24.04 QEMU (TCG/KVM) | FreeBSD jails (1 per workload) | seL4 Microvisor 1:1 microVM per workload |
| **VM-Orchestration** | Ubuntu 24.04 QEMU (TCG/KVM) | + gVisor sandboxing for aggregators | NanoVMs for aggregators; fat Linux stays for SLM/GPU inference |
| **VM-PrivateGit** | Ubuntu 24.04 QEMU (TCG/KVM) | FreeBSD jail around Gitea | gVisor or seL4 microVM |
| **VM-Infrastructure** (host OS) | Linux + KVM/TCG | NetBSD/NVMM on x86-64 (compat bottom; QEMU `-accel nvmm`) | seL4 + Microkit 2.2.0 on AArch64 (native bottom; requires hw acquisition) |

**Binding constraints (corrected 2026-05-29):**
- Microkit 2.2.0 includes `x86_64_generic_vtx` (pc99) target — NOT AArch64-only — but
  x86-64 Microkit is capacity-capped: **1 vCPU per guest max**, **Intel VT-x only** (AMD-V
  unsupported). AArch64 is the correct Phase 3 production path.
- Phase 2 compat bottom is **NetBSD/NVMM** (not bhyve — bhyve is FreeBSD's hypervisor).
  NVMM is mainline in NetBSD since 9.0; NetBSD 11.0 adds in-kernel `wg(4)` WireGuard and
  the MICROVM kernel config (NetBSD 11.0 RC4, May 2026).
- Both compat and native bottoms share the same capability ledger (system-core).

---

## 5. Infrastructure Directory Layout (target)

```
infrastructure/
├── local-vm-mediakit/
│   └── vm-mediakit.service              ← host systemd unit for VM-MediaKit QEMU process
├── systemd/
│   ├── mediakit/                        ← guest units for VM-MediaKit services
│   │   ├── local-proofreader.service
│   │   ├── local-knowledge-documentation.service
│   │   ├── local-knowledge-corporate.service
│   │   ├── local-knowledge-projects.service
│   │   ├── local-marketing-pointsav.service
│   │   ├── local-marketing.service
│   │   └── local-fs.service
│   ├── orchestration/                   ← guest units for VM-Orchestration
│   │   └── local-bim-orchestration.service
│   ├── totebox/                         ← guest units for VM-Totebox (future)
│   └── ppn/                             ← PPN fabric units (host-side)
│       └── local-ppn-pairing.service
└── virt/
    ├── lib/                             ← shared shell functions (sourced by provision scripts)
    │   ├── common.sh                    ← SSH wait loop, smoke-test curl, VM health check
    │   └── ppn-join.sh                  ← Genesis Protocol --join ceremony wrapper
    ├── provision-vm-mediakit.sh         ← Phase 1 COMPLETE
    ├── provision-vm-totebox.sh          ← stub (Phase 1 pending VM-Totebox)
    ├── provision-vm-orchestration.sh    ← stub (Phase 1 pending VM-Orchestration)
    ├── provision-vm-privategit.sh       ← stub (future)
    ├── provision-vm-infrastructure-cloud.sh    ← stub (GCP genesis / join)
    ├── provision-vm-infrastructure-onprem.sh   ← stub (Laptop A/B genesis / join)
    ├── vm-prove.sh                      ← Alpine TCG proof-of-concept (keep)
    ├── cloud-init-mediakit/             ← Phase 1 COMPLETE
    │   ├── meta-data
    │   └── user-data
    ├── cloud-init-totebox/              ← stub
    │   ├── meta-data
    │   └── user-data
    ├── cloud-init-orchestration/        ← stub
    │   ├── meta-data
    │   └── user-data
    └── work/                            ← runtime artifacts; gitignored
```

Host-level systemd units (`local-vm-mediakit.service`, future `local-vm-totebox.service`, etc.) live in `infrastructure/local-vm-<type>/`. Guest-level units (running inside each VM) live in `infrastructure/systemd/<type>/`.

---

## 6. Phase Completion Checklist

### VM-MediaKit Phase 1 — COMPLETE (2026-05-29)
- [x] Ubuntu 24.04 QEMU provisioned (6 GiB RAM, 2 CPUs, port-forward NAT)
- [x] 6/6 services migrated: proofreader · knowledge-docs/corp/projects · marketing-pointsav · marketing-woodfine
- [x] guide-vm-mediakit-provision staged (commit 4a53d3af)
- [x] guide-vm-mediakit-service-migration staged (commit 4a53d3af)
- [x] topic-os-mediakit corrected for Ubuntu 24.04 (session 10)
- [x] systemd units in `infrastructure/systemd/mediakit/`
- [ ] Binary-ledger sha256 entries (pending Stage 6 + nightly build rebuild)

### VM-Totebox Phase 1 — BLOCKED (Command must promote project-data)
- [ ] service-fs binary available on host (blocked: project-data 23-commit promotion)
- [ ] VM-Totebox QEMU instance provisioned (`provision-vm-totebox.sh`)
- [ ] service-fs install + smoke test
- [ ] system-core + system-ledger install (pending project-system)

### VM-Orchestration Phase 1 — NOT STARTED
- [ ] VM-Orchestration QEMU instance provisioned (`provision-vm-orchestration.sh`)
- [ ] app-orchestration-bim install + smoke test (depends on VM-Totebox service-fs)
- [ ] app-orchestration-gis instance
- [ ] app-orchestration-slm instance (:9180)

### VM-PrivateGit Phase 1 — NOT STARTED (future)
- [ ] VM-PrivateGit QEMU instance provisioned (`provision-vm-privategit.sh`)
- [ ] Gitea install + SSH
- [ ] app-privategit-design-system (Storybook)

### VM-Infrastructure Phase 1 — IN PROGRESS
- [x] Alpine Linux TCG proof-of-concept (`vm-prove.sh`, 2026-05-28)
- [ ] service-ppn-pairing deployed on GCP (listens 0.0.0.0:9202)
- [ ] os-network-admin built + deployed to Laptop A
- [ ] Genesis Protocol genesis-seed node active (GCP)
- [ ] First node-join ceremony (Laptop A joins GCP)

---

## 7. Open Decisions (gate for Phase 3)

| Decision | Impact |
|---|---|
| AArch64 hardware (GCP C4A or Firecracker x86-64 on Laptop A) | Phase 3 seL4 Microkit — AArch64 preferred; x86-64 limited to 1 vCPU/guest |
| Q2: Ratify 10.50.0.0/24 as canonical PPN subnet | Genesis Protocol INVENTORY.yaml; guide-mesh-orchestration.md |
| Q3: GCP static IP for cloud relay | fleet-infrastructure-cloud/guide-provision-relay.md |
| Q4: Laptop B local IP + network.woodfinegroup.com DNS | guide-deploy-vpn.md; guide-mesh-execution.md |
| Q5: Doorman deployed at localhost:9080? | app-network-admin F8 subprocess → HTTP migration |
| GCP nested KVM enablement | Unblocks KVM-accelerated VM provisioning on GCP; currently all QEMU runs TCG |
| `/opt/mediakit/bin/` → `/opt/<vm-name>/bin/` | New VMs use per-VM path; MediaKit Phase 1 path stays as-is (no retroactive migration) |

---

## §8 — Resource Pooling Layer (added 2026-05-29)

The three-node WireGuard mesh forms a unified VM resource pool. This is a free-tier PPN
primitive — NOT the paid Orchestration tier. Three new crates implement the pool:

```
Laptop A               Laptop B (hub)         GCP VM
service-vm-host        service-vm-host        service-vm-fleet :9203
     │                      │                      │
     └──── WireGuard heartbeat (10s) ──────────────┘
                                                    │
                                              app-network-admin
                                              F9 panel (ratatui)
```

**`service-vm-host`** (one per node, outbound-only):
- Polls `/proc/meminfo` + `/proc/loadavg` every 10s
- Queries QEMU UNIX monitor socket per running VM → `VmRecord`
- POSTs `NodeHeartbeat` to `service-vm-fleet` at `VM_FLEET_ENDPOINT`
- Systemd: `infrastructure/systemd/ppn/local-vm-host.service`

**`service-vm-fleet`** (GCP-resident, :9203):
- Receives heartbeats; evicts nodes silent >30s
- Placement: `ram_available_mb >= request.ram_mb + 512`; sort `ram_available_mb DESC`
- VM-Totebox: `preferred_node` must be caller-specified (WORM data cannot migrate over WireGuard)
- `auto_rebalance: false` — permanent architectural invariant; live migration excluded
- Systemd: `infrastructure/systemd/orchestration/local-vm-fleet.service`

**F12 doctrine (SYS-ADR-10):** "Create VM" is F12-gated. Scheduler's node choice is NOT.

**`system-vm-fleet-types`** (`no_std`-compatible): shared wire types for both services.

---

## §9 — Leapfrog 2030 Resource Targets (added 2026-05-29)

Phase 2 (NetBSD/NVMM host) and Phase 3 (seL4 unikernel) resource targets per os-* image.
See `BRIEF-LEAPFROG-2030.md` for full rationale, engineering constraints, and benchmarks.

| os-* | P2 disk | P2 RAM idle | P3 disk | P3 RAM idle | P3 RAM loaded | P3 OS stack |
|---|---|---|---|---|---|---|
| os-infrastructure | 120 MB | 48 MB | **8 MB** | **12 MB** | 48 MB | seL4+Microkit 2.x (AArch64) |
| os-totebox | 80 MB | 64 MB | **16 MB** | **24 MB** | 96 MB | Nanos/OPS or seL4+Microkit |
| os-mediakit | 600 MB | 180 MB | **24 MB†** | **32 MB†** | 128 MB† | Unikraft+Rust |
| os-orchestration | 96 MB | 96 MB | **12 MB** | **18 MB** | 80 MB | Unikraft+Rust |
| os-privategit | 180 MB | 96 MB | **20 MB†** | **24 MB†** | 96 MB† | Unikraft+Rust |
| seL4 baseline (7 PDs, no services) | 2 MB | 3 MB | — | — | — | seL4 kernel 162 KiB |

† contingent on retiring MediaWiki/PHP (os-mediakit) and Gitea/Go (os-privategit) — application decisions, not OS decisions.

**Positioning:** Phase 3 targets are 4–10× lighter than Lambda 128 MB floor, 20–40× lighter than Cloud Run 512 MiB.

**Key engineering discipline (all new service-* crates):**
```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```
Use `tokio::main(flavor = "current_thread")` for all daemons except service-fs.
