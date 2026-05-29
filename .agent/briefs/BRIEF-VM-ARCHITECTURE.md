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
| **VM-Infrastructure** (host OS) | Linux + KVM/TCG | NetBSD + bhyve on x86-64 (compat bottom) | seL4 + Microkit 2.2.0 on AArch64 (native bottom; requires hw acquisition) |

**Binding constraint:** Microkit 2.2.0 is AArch64-only as of May 2026. Phase 3 seL4 implicitly requires AArch64 hardware purchase. x86-64 nodes stay on NetBSD/bhyve (compat bottom). Both bottoms share the same capability ledger (system-core).

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
| AArch64 hardware (GCP C4A or Firecracker x86-64 on Laptop A) | Phase 3 seL4 Microkit — AArch64-only |
| Q2: Ratify 10.50.0.0/24 as canonical PPN subnet | Genesis Protocol INVENTORY.yaml; guide-mesh-orchestration.md |
| Q3: GCP static IP for cloud relay | fleet-infrastructure-cloud/guide-provision-relay.md |
| Q4: Laptop B local IP + network.woodfinegroup.com DNS | guide-deploy-vpn.md; guide-mesh-execution.md |
| Q5: Doorman deployed at localhost:9080? | app-network-admin F8 subprocess → HTTP migration |
| `/opt/mediakit/bin/` → `/opt/<vm-name>/bin/` | New VMs use per-VM path; MediaKit Phase 1 path stays as-is (no retroactive migration) |
