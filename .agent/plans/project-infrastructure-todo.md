---
title: project-infrastructure — Comprehensive TODO
created: 2026-05-20
updated: 2026-05-20 (session 3)
status: active
author: task@claude-code
---

# project-infrastructure — Comprehensive TODO

Full audit completed 2026-05-20. Items are ordered within each section: fix-first
(blocking or misaligned), then incomplete, then new work.

Source: read-through of all focus crates, content-wiki-documentation/systems+infrastructure+architecture,
and woodfine-fleet-deployment fleet-infrastructure-* + route-network-admin clusters.

---

## Section 1 — Archive housekeeping

These items should be done first. None require operator decision.

- [x] **Commit `CLAUDE.md` + `.agent/manifest.md` together** — done 2026-05-20 session 1
- [x] **Fix `session-start.md`** — done 2026-05-20 session 1
- [x] **Delete project-intelligence plans from `.agent/plans/`** — done 2026-05-20 session 1
- [x] **Delete project-intelligence drafts from `.agent/drafts-outbound/`** — done 2026-05-20 session 1
- [x] **Replace `NEXT.md`** — done 2026-05-20 session 1; updated again session 2
- [x] **Create `.agent/memory/` directory and seed `session-context.md`** — done 2026-05-20 session 1
- [x] **Fix manifest `planned_topics` slugs** — done 2026-05-20 session 1

---

## Section 2 — TOPIC leg (content-wiki-documentation)

Target repo: `vendor/content-wiki-documentation` — work staged here via `drafts-outbound/`,
promoted via project-editorial, committed by Command Session admin-tier.

### 2a — Fix: misaligned / outdated

- [x] **`infrastructure/sovereign-mesh.md` + `.es.md` — expand from stub to full topic**
  Done 2026-05-20 session 2. Drafts staged at `.agent/drafts-outbound/topic-sovereign-mesh.draft.md`
  + `topic-sovereign-mesh.es.draft.md`. Outbox message sent to project-editorial. Pending editorial
  pickup → commit to content-wiki-documentation.
  ~~Original description:~~
  Current state: one sentence. This is the gap that corresponds to `topic-ppn-architecture.md`
  in the manifest. Required content:
  - WireGuard overlay — all fleet nodes are mesh peers; no central broker
  - Hub-spoke topology for the Woodfine fleet (cloud relay at `fleet-infrastructure-cloud` as hub;
    on-premises and leased nodes as spokes)
  - `ppn0` interface naming convention
  - 16-byte binary command packets broadcast on port 8090
  - Relationship to `os-network-admin` (mesh policy owner) and `os-infrastructure` (mesh peers)
  - How the PPN connects to the Genesis Protocol described in `infrastructure-os.md`
  - See-also links to `infrastructure-os`, `os-network-admin`, `diode-standard`, `machine-based-auth`
  English + Spanish pair required.

### 2b — New TOPICs

- [x] **`topic-genesis-protocol.md` + `.es.md`** — done 2026-05-20 session 3.
  Staged at `.agent/drafts-outbound/topic-genesis-protocol.draft.md` + `.es.draft.md`.
  Covers: sequencing-dependency problem; 5-step sequence (blind boot, scan, genesis fork,
  holding pattern, claim); deferred fleet assembly; relationship to machine-based-auth.
  One noted open question (EAPOL vs intended arch — no correction needed to topic).

- [x] **`topic-ppn-command-protocol.md` + `.es.md`** — done 2026-05-20 session 3.
  Staged at `.agent/drafts-outbound/topic-ppn-command-protocol.draft.md` + `.es.draft.md`.
  Covers: design constraints (no broker, no plaintext, no verbosity); 16-byte packet format;
  4-step dispatch sequence; simultaneous broadcast rationale; Diode Standard relationship.

- [x] **`topic-service-pointsav-link.md` + `.es.md`** — done 2026-05-20 session 3.
  Staged at `.agent/drafts-outbound/topic-service-pointsav-link.draft.md` + `.es.draft.md`.
  Covers: four properties (default off, hot-plug activation, clean severance, policy in adapter);
  default state invariant; activation sequence; failure mode; Universal Standard.

### 2c — Stub expansion needed (from audit; not yet read fully)

- [ ] **Verify `architecture/three-layer-architecture.md`** — referenced by other topics;
  check if infrastructure layer is adequately covered there

---

## Section 3 — GUIDE leg (woodfine-fleet-deployment)

Target repo: `customer/woodfine-fleet-deployment` — work via Command Session admin-tier.
Cluster directories: `fleet-infrastructure-cloud/`, `fleet-infrastructure-leased/`,
`fleet-infrastructure-onprem/`, `route-network-admin/`.

### 3a — Fix: misaligned / stale

- [ ] **`fleet-infrastructure-leased/guide-deploy-vpn.md` — fix hardcoded path**
  Line 28: `scp /home/mathew/Foundry/pointsav-monorepo/service-vpn/provision_wireguard_hub.sh ...`
  Correct monorepo path is `/srv/foundry/vendor/pointsav-monorepo/`. Also replace
  `user@<LOCAL_IP_OF_LAPTOP_B>` with the ratified IP or a named placeholder that matches
  `guide-mesh-orchestration.md`.

- [ ] **`route-network-admin/guide-mesh-orchestration.md` — reconcile IP range**
  Guide says `10.x.x.x/24`. Code in `system-network-interface/src/main.rs` has peers hardcoded
  as `10.50.0.1`, `10.50.0.2`, `10.50.0.3`. These need to match. Either ratify `10.50.0.0/24`
  as the canonical range and update the guide, or update the code to match a different ratified range.

- [ ] **`guide-mesh-execution.md` — clarify `system-slm` vs service-slm Doorman**
  Guide references `system-slm` semantic router. The current code calls a binary at
  `/opt/pointsav/f8-gateway/system-slm`. The target architecture uses the real `service-slm`
  Doorman at `http://localhost:9080`. The guide should reflect the target architecture, not
  the prototype binary path. Update when the real wiring is in place (see §4c below).

### 3b — Complete: partially written guides

- [ ] **`fleet-infrastructure-cloud/guide-provision-relay.md` — fill in steps**
  Prerequisites are written. Needs:
  - Exact GCP project + zone + machine type recommendation
  - Static IP reservation steps (`gcloud compute addresses create`)
  - WireGuard installation + `wg0.conf` template for the hub role
  - Systemd `wg-quick@wg0` enable sequence
  - Firewall rule via `gcloud compute firewall-rules create`
  - Post-provision smoke: `wg show` + ping from a spoke
  Blocked on: operator ratifying the GCP project name and static IP.

- [ ] **`route-network-admin/guide-mesh-orchestration.md` — fill in steps**
  Structure is written. Needs:
  - Ratified subnet (`10.50.0.0/24` candidate)
  - Node-to-IP assignment table (cloud relay = .1, Laptop A = .2, iMac = .3 per code)
  - Full `wg0.conf` template per node role (hub vs spoke)
  - `INVENTORY.yaml` template at repo root (guide references this but file doesn't exist)
  Blocked on: operator confirming the `10.50.0.0/24` range and node IP assignments.

- [ ] **`fleet-infrastructure-leased/guide-provision-standalone.md` — fill in steps**
  Prerequisites written. Needs: SSH access steps, WireGuard install on Linux/macOS,
  config placement, `wg-quick up wg0`, smoke verification.

- [ ] **`fleet-infrastructure-onprem/guide-provision-onprem.md` — fill in steps**
  (Not read; likely scaffold. Fill in when iMac provisioning sequence is ratified.)

### 3c — New guides needed

- [ ] **`fleet-infrastructure-onprem/guide-genesis-protocol.md`** — operator runbook for
  bootstrapping a new `os-infrastructure` node via the Genesis Protocol:
  - Boot the node from ISO
  - Confirm holding pattern (WebSocket endpoint active)
  - Present admin fiduciary key from `os-network-admin`
  - Verify node joins mesh (`wg show peers`; telemetry heartbeat appears)
  Depends on: Genesis Protocol code being implemented (§4a).

- [ ] **`route-network-admin/guide-key-management.md`** — fiduciary keypair lifecycle:
  - First-boot keypair generation
  - Admin key custody (physically on Laptop A; never delegated to cloud — per manifest notes)
  - Claim key ceremony
  - Revocation procedure

- [ ] **Add `INVENTORY.yaml` at `woodfine-fleet-deployment/` root** — referenced by
  `guide-mesh-orchestration.md` as the node-to-IP registry. Does not exist. Create with
  the three known nodes (cloud relay, Laptop A/B, iMac) once IPs are ratified.

---

## Section 4 — Code (pointsav-monorepo, cluster/project-infrastructure branch)

Work done in this archive; committed via `commit-as-next.sh`; promoted via Stage 6 from Command Session.

### 4a — Fix: broken / won't compile

- [ ] **`os-infrastructure/src/main.rs` — resolve missing symbols**
  Imports `silicon_ping`, `enable_monitor_mode`, `init_dma_engine`, `hunt_for_eapol`, `RX_BUFFERS`
  from `system-substrate-broadcom` and `system-network-interface`. Neither crate exports
  these functions — both `lib.rs` files are 4-line scaffolds. The binary **cannot compile**.
  Three options (needs operator decision):
  - **Option A**: Implement the real bare-metal NIC driver functions in the dependency crates
    (aligns with the EAPOL-monitor-mode bootstrap approach in the current main.rs)
  - **Option B**: Replace `main.rs` with the Genesis Protocol architecture described in the TOPICs
    (WireGuard-first; seL4 keypair generation; holding pattern WebSocket — different approach)
  - **Option C**: Stub the functions to return safe no-op values so the binary compiles,
    then implement properly in a subsequent milestone

- [x] **`os-infrastructure/forge_iso.sh` — fix hardcoded monorepo path** — done 2026-05-20 session 2

- [x] **`os-infrastructure/Makefile` — fix wrong script name** — done 2026-05-20 session 2

- [x] **Gitignore build artifacts in `os-infrastructure/` and `os-network-admin/`** — done 2026-05-20 session 2; 14 tracked binaries removed from index
  ~~The following are tracked but should not be:
  `build_iso/boot/final_image.elf`, `build_iso/boot/kernel.elf`, `build_iso/boot/grub/`,~~
  `build_iso/os-infrastructure.elf`, `build_iso/system-substrate`, `build_iso/staging/`,
  `pointsav-os-infrastructure.iso`, `linker.ld` (if auto-generated), `.gitkeep` (once real
  files exist). Add to `os-infrastructure/.gitignore`.
  Same pattern applies to `os-network-admin/build_iso/` and `pointsav-os-network-admin.iso`.

### 4b — Fix: structural anomaly

- [x] **`system-network-interface` — split lib.rs from main.rs** — done 2026-05-20 session 2
  F8 Gateway binary moved to `app-network-admin/`. `system-network-interface` is now a pure
  lib crate (no std deps, `[workspace]` kept for standalone isolation). Both compile clean.

- [ ] **`os-network-admin/src/main.rs` — clarify role**
  Currently a simple UDP telemetry poller connecting to `10.0.0.101:5000` (Laptop B vitals).
  This is not the pairing registry or mesh routing policy described in the TOPICs. Decide:
  is this a useful operational utility to keep, or is it superseded by the F8 gateway binary?

- [ ] **`os-network-admin/scripts/mesh_status.sh` — fill in GCP relay IP**
  `GCP_RELAY_IP="[ENTER_YOUR_GCP_STATIC_IP_HERE]"` — placeholder not filled.
  Blocked on: cloud relay provisioning (§3b above).

### 4c — New code: align with TOPIC architecture

These are the implementation gaps between the TOPICs (target) and the code (prototype).
Order reflects dependency chain — each item unblocks the next.

- [ ] **Implement `system-substrate-broadcom/src/lib.rs` — real Broadcom NIC functions**
  Replace the 4-line scaffold with:
  - `silicon_ping() -> bool` — detect Broadcom 14e4:16b4 NIC presence via PCI device scan
  - Hardware target: iMac 12,1 (CPU: Sandy Bridge i5-2400S; NIC: Broadcom 14e4:16b4)
  This is a `no_std` bare-metal library; uses memory-mapped I/O via inline assembly.

- [ ] **Implement `system-network-interface/src/lib.rs` — bare-metal NIC driver functions**
  Replace the 4-line scaffold with the functions `os-infrastructure/src/main.rs` imports:
  - `enable_monitor_mode() -> ()` — put NIC into 802.11 monitor mode
  - `init_dma_engine() -> bool` — initialize DMA ring buffers
  - `hunt_for_eapol() -> Option<usize>` — scan RX buffers for EAPOL (802.1X auth) frames
  - `RX_BUFFERS: [[u8; 1514]; N]` — receive buffer array
  Note: these are `no_std` bare-metal functions. The `[workspace]` and std deps in
  `system-network-interface/Cargo.toml` must be removed (or the split in §4b done first).

- [ ] **Implement Genesis Protocol in `os-infrastructure/src/main.rs`**
  Per `infrastructure-os.md` §2:
  - Blind boot: skip DHCP/DNS; generate seL4 fiduciary keypair at first boot
  - Scan for `os-network-admin` beacon on local mesh
  - Genesis fork if no beacon: create single-node PPN; seal external ports except one
  - Holding pattern: open one hardened WebSocket endpoint waiting for admin claim
  - Claim handler: verify admin fiduciary key; bind to fleet; join WireGuard mesh
  Depends on: §4a Option B decision, seL4 keypair generation substrate.

- [ ] **Wire F8 terminal to real `service-slm` Doorman**
  `system-network-interface/src/main.rs` `handle_translation()` currently calls
  `/opt/pointsav/f8-gateway/system-slm` via `Command::new()` — a hardcoded binary path.
  Replace with an HTTP call to `http://localhost:9080/v1/messages` (service-slm Doorman
  endpoint, Sprint 0a shim). The `NODE_ID` env var is already wired. The HITL verification
  step is already in the `handle_authorization` flow — preserve it.
  Depends on: service-slm Sprint 0a shipped and deployed (project-intelligence scope).

- [ ] **Replace UDP broadcast JSON with 16-byte binary command protocol**
  `handle_authorization()` currently serializes a JSON `MeshPayload` struct and sends it.
  Per `os-network-admin.md`, the mesh sees 16-byte binary packets only — never JSON.
  Define the 16-byte layout:
  - Bytes 0–1: 2-byte operation code (e.g., PING=0x0001, ISOLATE=0x0002, etc.)
  - Bytes 2–3: target node ID (or 0xFFFF for broadcast)
  - Bytes 4–7: timestamp (Unix seconds, big-endian)
  - Bytes 8–15: reserved / payload extension
  Replace `serde_json::to_string(&payload)` with the binary serialization.

- [ ] **Implement `service-pointsav-link` adapter stub**
  Per `diode-standard.md`:
  - Default state: not installed; Subject cannot phone home
  - Activation: hot-pluggable single command
  - Failure mode: clean link severance on crash
  At minimum, scaffold the crate at `system-core/` or a new `service-pointsav-link/`
  directory (check the project-registry if a Reserved-folder already exists).

### 4d — App surfaces: create missing Reserved-folders

These appear in the manifest tetrad `focus` list but no directories exist in the monorepo:

- [x] **Create `app-infrastructure-onprem/`** — done 2026-05-20 session 2
- [x] **Create `app-infrastructure-leased/`** — done 2026-05-20 session 2
- [x] **Create `app-infrastructure-cloud/`** — done 2026-05-20 session 2

---

## Section 5 — Cross-cutting / operator decisions needed

These items cannot proceed until the operator makes a call.

- [ ] **Decide: Genesis Protocol vs EAPOL-monitor bootstrap**
  The current `os-infrastructure/src/main.rs` is doing EAPOL packet capture (WiFi NIC
  monitor mode). The TOPICs describe a WireGuard-first Genesis Protocol. These are two
  different approaches to fleet bootstrapping. Which is the intended implementation path?
  - **EAPOL approach**: node uses WiFi monitor mode to observe authentication traffic and
    derive bootstrap material. Unusual; specific to the Broadcom 14e4:16b4 hardware.
  - **Genesis Protocol approach**: WireGuard-first; seL4 keypair at boot; WebSocket hold;
    explicit admin claim. Aligns with the published TOPICs.

- [ ] **Ratify the `10.50.0.0/24` subnet for the PPN mesh**
  Code has `10.50.0.1/2/3` hardcoded. Guides say `10.x.x.x/24` (unspecified).
  Confirm `10.50.0.0/24` as canonical, then update `guide-mesh-orchestration.md` and
  add an `INVENTORY.yaml` to `woodfine-fleet-deployment/`.

- [ ] **Provide GCP static IP for cloud relay**
  Needed to complete `guide-provision-relay.md` and `mesh_status.sh`.

- [ ] **Confirm Laptop A / Laptop B IP assignments**
  `guide-deploy-vpn.md` has `<LOCAL_IP_OF_LAPTOP_B>` as a placeholder.
  `guide-mesh-execution.md` references `https://network.woodfinegroup.com` — DNS status?

---

## Sequencing recommendation

1. **Section 1** (archive housekeeping) — do now; no decisions needed; unblocks clean work
2. **Section 2a** (expand `sovereign-mesh.md` stub) — high impact; TOPIC leg's only gap
3. **Section 5 decisions** (ask operator for subnet + IP ratification) — unblocks §3b + §4c
4. **Section 4a** (fix broken build: resolve missing symbols + forge_iso.sh + Makefile) — code hygiene
5. **Section 4b** (structural split: `system-network-interface`) — needed before §4c
6. **Section 3a** (fix misaligned guides: path, IP range) — small fixes, do alongside §4a
7. **Section 3b** (complete partially-written guides) — once §5 decisions land
8. **Section 4c** (new code: Genesis Protocol, F8/service-slm wiring, binary protocol) — main feature work
9. **Section 2b** (new TOPICs: genesis-protocol, ppn-command-protocol, service-pointsav-link) — after code milestones so TOPICs reflect what shipped
10. **Section 3c** (new guides: genesis-protocol runbook, key-management) — after §4c ships
11. **Section 4d** (app surface Reserved-folders) — low effort; can do any time
