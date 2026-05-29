# NEXT.md — project-infrastructure (cluster/project-infrastructure branch)

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.
> Full TODO with all sections and sequencing: `.agent/plans/project-infrastructure-todo.md`.

Last updated: 2026-05-29 (session 11).

Architecture: VM-* naming mirrors the os-* product lineup exactly. Each `os-*` binary is the
source identity; each `VM-*` is the runtime identity. See `BRIEF-VM-ARCHITECTURE.md`.

---

## VM-MediaKit — Phase 1 COMPLETE (6/6) [2026-05-29]

- [x] Ubuntu 24.04 QEMU provisioned (6 GiB RAM, 2 CPUs, TCG; port-forward NAT on GCP host)
  [2026-05-29 totebox@claude-code]
- [x] 6/6 services migrated: proofreader (9092) · knowledge-documentation (9090) ·
  knowledge-corporate (9095) · knowledge-projects (9093) · marketing-pointsav (9101) ·
  marketing/woodfine (9102). All originals still running on host. No DNS changes.
  [2026-05-29 totebox@claude-code]
- [x] `guide-vm-mediakit-provision` + `guide-vm-mediakit-service-migration` staged (commit 4a53d3af)
  [2026-05-29 totebox@claude-code]
- [x] `topic-os-mediakit` corrected for Ubuntu 24.04 (session 10)
  [2026-05-29 totebox@claude-code]
- [x] systemd units reorganised → `infrastructure/systemd/mediakit/`
  [2026-05-29 totebox@claude-code]
- [ ] Binary-ledger sha256 entries — pending Stage 6 + nightly build rebuild

---

## VM-Totebox — Phase 1 (blocked on Command promoting project-data)

- [ ] **service-fs binary available on host** — blocked: Command must promote project-data (23 commits).
  Outbox sent to project-data with install instructions.
  [2026-05-29 totebox@claude-code]
- [ ] **VM-Totebox QEMU instance provisioned** — `infrastructure/virt/provision-vm-totebox.sh`
  (stub exists; implementation follows service-fs availability)
  [2026-05-29 totebox@claude-code]
- [ ] **service-fs install inside VM-Totebox** — follows promotion
  [2026-05-29 totebox@claude-code]
- [ ] **system-core + system-ledger install** — pending project-system (outbox sent; 95 tests must pass)
  [2026-05-29 totebox@claude-code]

---

## VM-Orchestration — Phase 1

- [ ] **Provision VM-Orchestration** — new QEMU instance, separate from vm-mediakit.
  `infrastructure/virt/provision-vm-orchestration.sh` (stub exists)
  [2026-05-29 totebox@claude-code]
- [ ] **app-orchestration-bim (9096)** — install + smoke test. Was previously mis-scoped to
  VM-MediaKit; correct scope is VM-Orchestration. Depends on VM-Totebox service-fs.
  [2026-05-29 totebox@claude-code]
- [ ] **app-orchestration-gis instance** [2026-05-29 totebox@claude-code]
- [ ] **app-orchestration-slm instance (:9180)** [2026-05-29 totebox@claude-code]

---

## VM-PrivateGit — Phase 1 (future)

- [ ] **Provision VM-PrivateGit** — `infrastructure/virt/provision-vm-privategit.sh` (stub exists)
  [2026-05-29 totebox@claude-code]
- [ ] **app-privategit-source-control install** (Gitea + SSH)
  [2026-05-29 totebox@claude-code]
- [ ] **app-privategit-design-system** (Storybook)
  [2026-05-29 totebox@claude-code]

---

## VM-Infrastructure — fabric bootstrap (3-node trust mesh)

- [x] Alpine Linux TCG proof-of-concept (`vm-prove.sh`, 2026-05-28) — virtio_balloon confirmed
- [ ] **Deploy `service-ppn-pairing` on GCP** — build release binary + install systemd unit
  `infrastructure/systemd/ppn/local-ppn-pairing.service`. Listens on `0.0.0.0:9202`.
  [2026-05-28 totebox@claude-code]
- [ ] **Build + copy `os-network-admin` to Laptop A** — `cargo build --release -p os-network-admin`
  then deploy to iMac with `PAIRING_SERVER=http://10.8.0.9:9202`. [2026-05-28 totebox@claude-code]
- [ ] **provision-vm-infrastructure-cloud.sh** — GCP genesis-seed node (stub exists; implement after Q2–Q6)
  [2026-05-29 totebox@claude-code]
- [ ] **provision-vm-infrastructure-onprem.sh** — Laptop A join (stub exists; implement after Q2–Q6)
  [2026-05-29 totebox@claude-code]
- [ ] **Deferred: os-network-admin ratatui TUI** — keyboard approve/deny; QR; expiry countdown.
  [2026-05-28 totebox@claude-code]

---

## os-mediakit seL4 — Phase 3 (planned — operator decision needed first)

- [ ] **Operator decision: AArch64 GCP C4A vs Firecracker x86_64 on Laptop A**
  Option A: AArch64 GCP C4A Arm instance — Microkit 2.2 native, formal proof (~$50-100/mo)
  Option B: Firecracker + WireGuard on Laptop A — x86_64, KVM-native, pragmatic, free
  Option C: seL4 x86_64 Multiboot2 — years of new toolchain work, not recommended
  See BRIEF-totebox-transformation §9/§11 for full analysis.
  [2026-05-29 totebox@claude-code]

- [ ] **project-system: wire os-mediakit as AArch64 seL4 Microkit rootserver** — 7-step
  build instructions sent to project-system outbox (2026-05-29). Gated on operator decision.
  [2026-05-29 totebox@claude-code]

---

## TOPIC + GUIDE leg — drafts staged, needs editorial pickup

Ten TOPIC draft pairs + 3 GUIDE drafts + 1 new TOPIC pair in `.agent/drafts-outbound/`;
pickup notice sent to project-editorial.

**TOPICs (content-wiki-documentation):**
- [ ] `topic-sovereign-mesh` + `.es` — expands stub at `infrastructure/sovereign-mesh.md` [session 2]
- [ ] `topic-genesis-protocol` + `.es` — new; `architecture/genesis-protocol.md` [session 3]
- [ ] `topic-ppn-command-protocol` + `.es` — new; `architecture/ppn-command-protocol.md` [session 3]
- [ ] `topic-service-pointsav-link` + `.es` — new; `architecture/service-pointsav-link.md` [session 3]
- [ ] `topic-os-network-admin` + `.es` — new; replaces published `systems/os-network-admin.md` [session 5]
- [ ] `topic-ppn-hypervisor-resource-pool` + `.es` — updated; §: Planned cross-node extension [session 7]
- [ ] `topic-totebox-archive` + `.es` — new; `systems/totebox-archive.md` [session 6]
- [ ] `topic-ppn-architecture-overview` + `.es` — updated; distributed fabric paragraph [session 7]
- [ ] `topic-ppn-distributed-vm-fabric` + `.es` — new; full distributed VM fabric [session 7]
- [ ] `topic-os-mediakit` + `.es` — updated session 10: Ubuntu 24.04 fix; Phase 1 service table corrected [session 8, corrected 10]

**GUIDEs (woodfine-fleet-deployment/fleet-infrastructure/) — session 10:**
- [ ] `guide-vm-mediakit-provision` — provision-vm-mediakit.sh runbook; cloud-init; pkg install; TCG notes [session 10]
- [ ] `guide-vm-mediakit-service-migration` — migrate-service-to-vm.sh; per-service instructions; pre-DNS checklist [session 10]

**GUIDEs (woodfine-fleet-deployment/fleet-infrastructure/):**
- [ ] `guide-ppn-first-deployment` — 5-step deployment sequence; VM capacity planning table [session 7]
- [ ] `guide-node-join-ceremony` — approval workflow [session 6]
- [ ] `guide-vm-prove-balloon-demo` — vm-prove.sh + virtio_balloon demo [session 6]

---

## Dev-environment bootstrap (unblocked — activate first ceremony)

- [ ] **Deploy `service-ppn-pairing` on GCP VM** — build release binary + install systemd unit
  `infrastructure/systemd/local-ppn-pairing.service`. Listens on `0.0.0.0:9202`.
  [2026-05-28 totebox@claude-code]

- [ ] **Build + copy `os-network-admin` to Laptop A (iMac)** — `cargo build --release -p os-network-admin`
  then `scp target/release/os-network-admin mathew@10.8.0.6:~/bin/`. Run with
  `PAIRING_SERVER=http://10.8.0.9:9202 ~/bin/os-network-admin`. [2026-05-28 totebox@claude-code]

- [x] **Run `infrastructure/virt/vm-prove.sh` — GCP TCG proof complete 2026-05-28.**
  Alpine Linux 3.20 (kernel 6.6.31-0-virt) booted in 114s via TCG. virtio_balloon
  inflation confirmed: `balloon 128` → `actual=128`; deflation: `balloon 256` →
  `actual=256`. Full KVM proof on Laptop A (hardware VT-x) remains for production
  validation. [2026-05-28 totebox@claude-code]

- [ ] **Deferred: os-network-admin ratatui TUI** — keyboard approve/deny (a/d); QR rendering
  via `system-pairing-codes::qr_unicode`; expiry countdown. Full §9.2 Step 4 UX.
  [2026-05-28 totebox@claude-code]

---

## Blocking — operator decisions needed first

- [x] **Decide: EAPOL-monitor-mode vs Genesis Protocol** — **RESOLVED: Genesis Protocol.**
  BRIEF-PPN-ARCHITECTURE.md (2026-05-27) establishes Genesis Protocol as the canonical
  bootstrap architecture. EAPOL approach superseded entirely. Code rewrite gated on Q2–Q6.
  [2026-05-27 totebox@claude-code]

- [ ] **Q2: Ratify `10.50.0.0/24` as the canonical PPN subnet**
  Code hardcodes `10.50.0.1/2/3`; `guide-lxc-network-admin.md` also uses this range (de
  facto confirmed). Confirm, then update `route-network-admin/guide-mesh-orchestration.md`
  + `INVENTORY.yaml`. [2026-05-27 totebox@claude-code]

- [ ] **Q3: Provide GCP static IP for cloud relay**
  Needed to complete `fleet-infrastructure-cloud/guide-provision-relay.md` and
  `os-network-admin/scripts/mesh_status.sh` (`[ENTER_YOUR_GCP_STATIC_IP_HERE]` placeholder).
  [2026-05-27 totebox@claude-code]

- [ ] **Q4: Confirm Laptop B local IP + `network.woodfinegroup.com` DNS status**
  `guide-deploy-vpn.md` has `<LOCAL_IP_OF_LAPTOP_B>` placeholder.
  `guide-mesh-execution.md` references `https://network.woodfinegroup.com`.
  [2026-05-27 totebox@claude-code]

- [ ] **Q5: Is service-slm Doorman deployed at `localhost:9080`?**
  `app-network-admin/src/main.rs` F8 Gateway still calls subprocess `/opt/pointsav/f8-gateway/system-slm`.
  Must be replaced with HTTP to `localhost:9080` (BRIEF §9.2 Step 5). [2026-05-27 totebox@claude-code]

- [ ] **Q6: Flag stale editorial pickup to Command Session outbox?**
  5 draft pairs in `.agent/drafts-outbound/` — 7 days without pickup. [2026-05-27 totebox@claude-code]

---

## BRIEF — PPN Architecture (gate for all code decisions)

- [x] **`BRIEF-PPN-ARCHITECTURE.md` written** — Yale PhD thesis quality; 385 lines / 39.5 KB;
  57-citation bibliography; Genesis Protocol confirmed; CPace PAKE + SAS short-code pairing;
  CAmkES OS personality; intransitive non-interference invariant. Committed 2026-05-27.
  All code work in §9.2 build order is now unblocked at the architecture level; operator
  decisions Q2–Q6 gate individual implementation steps. [2026-05-27 totebox@claude-code]

---

## Code — implement Genesis Protocol (gated on Q2–Q6)

- [ ] **Rewrite `os-infrastructure/src/main.rs`** — Genesis Protocol boot sequence
  (blind boot → mDNS scan → genesis fork → WebSocket holding pattern → admin claim).
  Replaces broken EAPOL approach. BRIEF §9.2 Step 1. [2026-05-27 totebox@claude-code]

- [ ] **Implement `system-substrate-broadcom/src/lib.rs`** — `silicon_ping() -> bool`,
  Broadcom 14e4:16b4 PCI detection, no_std. BRIEF §9.2 Step 2. [2026-05-27 totebox@claude-code]

- [ ] **Implement `system-network-interface/src/lib.rs`** — WireGuard/mDNS substrate
  (replaces 4-line scaffold). BRIEF §9.2 Step 3. [2026-05-27 totebox@claude-code]

- [ ] **Short-code pairing ceremony for node join** — CPace PAKE + Crockford base32 8-char
  code; mirrors project-console Phases 1–4. BRIEF §9.2 Step 4. [2026-05-27 totebox@claude-code]

- [ ] **Replace F8 Gateway subprocess with HTTP to `localhost:9080`** — Q5 must be
  confirmed first. BRIEF §9.2 Step 5. [2026-05-27 totebox@claude-code]

- [ ] **Replace JSON mesh payloads with 16-byte binary protocol** — BRIEF §9.2 Step 6.
  [2026-05-27 totebox@claude-code]

- [ ] **Add focus crates to root `Cargo.toml` workspace members** — `os-infrastructure`,
  `os-network-admin`, `system-network-interface`, `system-substrate-broadcom`.
  [2026-05-27 totebox@claude-code]

---

## Future milestones — balloon controller + PSP + Phase 4 gateway

- [ ] **Balloon controller in `os-infrastructure`** — the virtio_balloon controller that decides
  when to inflate/deflate each VM's balloon in response to demand signals. Planned milestone.
  Until implemented, operators use QEMU monitor: `balloon 128` / `info balloon`.
  [2026-05-28 totebox@claude-code]

- [ ] **PSP (PointSav Protocol) implementation** — the capability-based binary protocol over TLS
  that `os-orchestration` uses to query Totebox Archives. Stateless aggregator sends signed
  capability objects; Totebox verifies and emits only result rows. Planned milestone.
  [2026-05-28 totebox@claude-code]

- [ ] **Phase 4 gateway: `gateway-orchestration-command-1`** — inbound MBA connections via PSP;
  multi-archive query routing. Gated on PSP protocol implementation.
  [2026-05-28 totebox@claude-code]

---

## GUIDE leg — cross-repo fix (Command Session scope)

- [ ] **`fleet-infrastructure-leased/guide-deploy-vpn.md`** — fix hardcoded path
  `$HOME/Foundry/pointsav-monorepo/` → `/srv/foundry/vendor/pointsav-monorepo/`.
  Edit lives in `customer/woodfine-fleet-deployment` — Command Session admin-tier.
  [2026-05-20 task@claude-code]

---

## Completed this cluster (archived for reference)

- [x] Sweep project-intelligence contamination from archive (2026-05-20 session 1)
- [x] Fix `session-start.md`, `manifest.md` slug mismatch, `NEXT.md`, memory init (session 1)
- [x] Stage `sovereign-mesh.md` + `.es.md` drafts to `drafts-outbound/` (session 2)
- [x] Fix `os-infrastructure/Makefile` script name (session 2)
- [x] Fix `os-infrastructure/forge_iso.sh` hardcoded path (session 2)
- [x] Gitignore build artifacts in `os-infrastructure/` and `os-network-admin/` (session 2)
- [x] Create `app-infrastructure-onprem/`, `-leased/`, `-cloud/` Reserved-folder scaffolds (session 2)
- [x] Split `system-network-interface` → extract F8 Gateway binary to `app-network-admin/` (session 2)
