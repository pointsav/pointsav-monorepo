---
artifact: brief
schema: foundry-brief-v1
brief-id: project-infrastructure-totebox-transformation
title: Totebox Transformation — VM Fabric Live State
status: active
owner: project-infrastructure
created: 2026-05-28
updated: 2026-06-29
---

# Totebox Transformation — VM Fabric Live State

Active work-tracking for the PPN/VM resource pool. Stable architecture decisions
(VM layout, seL4/Firecracker choice, Part C sequencing, service-vm-tenant design,
os-network-admin role definition) are in `BRIEF-ppn-infrastructure-reference.md`.

---

## Live Fleet — 2026-06-29

Three nodes registered and heartbeating (last confirmed 2026-06-12):

| Node | WG IP | RAM avail | KVM | Status |
|---|---|---|---|---|
| laptop-a-1 | 10.8.0.6 | ~3 GB | yes | Active |
| gcp-cloud-1 | 10.8.0.9 | ~3 GB | no | Active — genesis relay |
| laptop-b-1 | 10.8.0.1 | ~2.5 GB | yes | Reserved (VP use; VM_RESERVED=true) |

VM spawn delegation confirmed end-to-end 2026-06-12.
WireGuard: GCP↔Laptop A ✓, GCP↔Laptop B ✓, Laptop A↔Laptop B: peer added; no direct endpoint yet.

Deployed services:
- `service-vm-fleet` :9203 — advisory placement, 14 tests
- `service-vm-host` :9220 (per node) — QEMU spawn, .meta.json sidecar, 7 tests
- `service-vm-tenant` :9221 — auth proxy (opaque TOKEN_MAP, SLIRP host_ports, service-fs audit)
- `app-orchestration-slm` :9180 — commercial Yo-Yo broker; 15 tests; Stage 6 pending

---

## Open Items

### Operator action required (not Totebox-actionable)

- [ ] **Laptop B → Laptop A direct path:** add `endpoint 10.0.0.65:51821` to Laptop B's peer entry + `wg-quick save wg0` (needs Laptop B sudo)
- [ ] **Kill GCP :9299 HTTP server** once Laptop A setup complete; remove UFW rule for :9299
- [ ] **Marketplace listing:** `soft-slm-orchestration` at software.pointsav.com (operator action)
- [ ] **ORCID IDs + Peter email** for JOURNAL drafts (J1, PR1) before submission

### Command scope

- [ ] Stage 6 canonical merge — **promote-queue entry written 2026-06-29**. Cluster branch
  `cluster/project-infrastructure` pushed to both staging mirrors as a named branch
  (staging-j/main has 10 diverged commits from other archives; Command must fetch named branch
  directly and filter `.agent/` before canonical merge). Code commits in scope:
  `59bee7af` (Clippy service-content), `2773f0c3`+`38f98c57` (Clippy service-fs+security),
  `6a43937b` (.mcp.json+CLAUDE.md), `c925c8a4` (CLAUDE.md S125), `d106b46a` (moonshot-toolkit task-14).
- [ ] `pairings.yaml` audit — separate clone directories per archive (contamination)
- [ ] `briefs/README.md` for project-infrastructure

### Architecture decisions locked (2026-06-29 session)

- [x] **seL4 Option B** (current path): seL4 kernel + CAmkES VMM + Linux guest — see §Three-Path Architecture below
- [x] **moonshot-hypervisor** = Option A (pure seL4 PDs, no VMs, moonshot)
- [x] **moonshot-sel4-vmm** = Option C (hybrid: seL4 PDs own WireGuard, Linux VM for workloads, moonshot)
- [x] **Microkit 2.2.0 supports x86-64** (`x86_64_generic` + `x86_64_generic_vtx`, added v2.1.0 Nov 2025)
  — earlier §9 AArch64-only assumption is stale; see updated reference BRIEF §9
- [x] **moonshot-toolkit task #14 unblocked** — decisions recorded in `moonshot-toolkit/src/main.rs`
  (toolchain: Microkit 2.2.0; vendoring: vendor-sel4-kernel/; harness: plan_hash + Ed25519)
- [x] **Three-artifact distribution model per product**: `.iso` (bare metal), `.qcow2` (cloud import),
  daemon AppImage/.deb (Linux). Same Ed25519 key, same USDC license token.
- [x] **"Topology" not "geometry"** — seL4 capability graph model uses "topology" (Miller 2000,
  Fuchsia "component topology" as industrial prior art). All artifacts updated.

### New open items — 2026-06-29

- [ ] **os-infrastructure**: Microkit 2.2.0 system-spec.toml for x86-64 (pc99 target), bootable ISO,
  test on Laptop A (VT-x bare metal) + foundry-workspace (QEMU/TCG)
- [ ] **os-network-admin daemon mode**: `cargo build --features daemon` → standalone Linux binary,
  AppImage packaging; test on iMac 2010-2012 Linux Mint (Intel x86-64)
- [ ] **Three-node mesh test** (D7): Laptop A bare metal + foundry-workspace VM + iMac daemon
  → all three in `service-vm-fleet`; gate for software.pointsav.com listing
- [ ] **os-infrastructure + os-network-admin on software.pointsav.com**: two-click install
  ($19 USDC and $1 USDC respectively)
- [ ] Design sync reply: ACK schema ratification request from project-orchestration
  (invite token pairing protocol — pairings.yaml `user_pairings` + WORM ledger format)
  — pending; see inbox msg-id `command-20260629-design-sync-invite-token-pairing-protoco`

### os-network-admin Phase S3 — DONE (commit 13ef4654)

`fleet_watch.rs` module added. Polls fleet every 30s (or `FLEET_WATCH_INTERVAL_SECS`).
Reads `~/.local/share/ppn/nodes.jsonl` for approved pubkeys. On new node: issues
`wg set <WG_IFACE> peer <pubkey> allowed-ips <wg_ip>/32` then writes WORM event to
service-fs `/v1/append`. Pure `compute_pending_peers()` tested: 8/8 tests. Requires
`CAP_NET_ADMIN` / root for `wg set`. Env vars: `WG_IFACE`, `FLEET_URL`, `SERVICE_FS_URL`,
`NODES_JSONL_PATH`.

### os-network-admin Phase S4 (Genesis Protocol milestone)

Wire `system-network-interface::conduct_pairing_ceremony()` to UDP server (:9206).
Test Genesis Protocol end-to-end on Laptop A bare-metal boot.
Gate: os-infrastructure must boot bare-metal first.

### Editorial backlog

- [ ] TEXT: "Any Hardware, Sovereign Compute" — PPN small-business proposition (~300 words); target: pointsav.com product page; source: reference BRIEF §15
- [ ] TOPIC: Tenant VM Isolation — A1-A4 hardening now stable (dbf6a528); source: reference BRIEF §16
- [ ] TOPIC: os-network-admin as PPN Control Plane — after Phase S3 ships; source: reference BRIEF §14 + §17

---

## 2026-06-14 Session — Completed

| Stream | Work | Commits |
|---|---|---|
| A | service-vm-tenant: opaque bearer (TOKEN_MAP), SLIRP host_ports, service-fs audit route | `dbf6a528` |
| B | os-network-admin Phase S2: tokio UDP :9206 listener; PING→PONG; PPN_PEERS env var | `3bafaec5` |
| C | app-orchestration-slm: runtime license pubkey (ORCHESTRATION_LICENSE_PUBKEY_HEX) | `85e8c60f` |
| D | Wiki leg: TOPIC-ppn-vm-architecture + GUIDE-ppn-fleet-operations staged to drafts-outbound | `2b58b6c5` |
| ops | Manifest fixed; audit rollup endpoint (15 tests); editorial drafts staged | `c46f2d39`, `2b58b6c5` |
| S3 | os-network-admin Phase S3: fleet watch + WireGuard peer-table auto-program + WORM ledger; 8/8 tests | `13ef4654` |

Prior milestones (2026-06-12): fleet RAM enforcement, vm-host .meta.json sidecar, Laptop B
reservation, end-to-end VM spawn delegation confirmed. Full log in reference BRIEF.

---

## § Three-Path seL4 Architecture (2026-06-29)

Three mutually exclusive architecture paths. Only Option B is active development;
C and A are documented moonshots with gate conditions. See reference BRIEF §19 for full spec.

| Option | Name | Status | Gate |
|---|---|---|---|
| B | seL4 → CAmkES VMM → Linux guest | **Current path** (active) | Ships first |
| C | seL4 PDs + Linux VM (moonshot-sel4-vmm) | Moonshot (planned/intended) | After Option B ships ≥6 months |
| A | Pure seL4 PDs, no VMs (moonshot-hypervisor) | Moonshot (planned/intended) | After Option C proves PD model |

**Option B (current):** seL4 at EL2/VT-x. CAmkES VMM hosts Linux (Debian 12) guest. All PPN services
run in Linux guest. AArch64 EL2 has integrity proof (April 2025, UK NCSC). x86-64 has C-level
functional correctness only — no formal security claims on x86.

**Formal verification coverage:**
- AArch64 EL2: functional correctness + integrity (Apr 2025). In progress: confidentiality.
- RISC-V64: deepest (binary-level + all CIA). Constrained to HiFive Unleashed.
- x86-64: functional correctness only. No formal security claims permissible.

**seL4 capability topology:** The directed graph of capability pointers determines what
component can call what other component. Capability isolation is formally proved invariant —
only connectivity begets connectivity (Miller 2000). This is the commercial security claim.
Use "topology" not "geometry" — see Miller 2000, Drossopoulou 2016, Fuchsia "component topology."

---

## Cross-References

- `BRIEF-ppn-infrastructure-reference.md` — stable architecture (extracted 2026-06-14)
- `BRIEF-PPN-ARCHITECTURE.md` — hypervisor TCB, seL4 proof, distributed fabric design
- `BRIEF-sovereign-os-family-master-plan.md §R–§W` — governance layer
- `infrastructure/virt/vm-prove.sh` — proven hypervisor foundation
