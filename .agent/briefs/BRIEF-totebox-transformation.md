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

## 8. Cross-References

- `BRIEF-sovereign-os-family-master-plan.md §R–§W` — governance layer (which folders move where)
- `BRIEF-PPN-ARCHITECTURE.md` — hypervisor TCB, seL4 proof, distributed fabric design
- `BRIEF-PPN-DEV-BOOTSTRAP.md §11` — Session 7 Q&A: no-reboot confirmation, capacity planning
- `BRIEF-sovereign-os-family-master-plan.md §B` — WireGuard address plan (10.42.0.0/16)
- `BRIEF-sovereign-os-family-master-plan.md §J` — Part A (WireGuard) + Part B (foundry-prod) prerequisites
- `infrastructure/virt/vm-prove.sh` — the proven foundation
- `infrastructure/virt/work/ppn-prove.qcow2` — proof QCOW2 (gitignored; rebuild with vm-prove.sh)
