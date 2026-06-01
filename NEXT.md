# NEXT.md — project-infrastructure (cluster/project-infrastructure branch)

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.
> Architecture: VM-* naming mirrors the os-* product lineup exactly. See `BRIEF-VM-ARCHITECTURE.md`.

Last updated: 2026-06-01 (session 16 — reconstructed from session context; was contaminated
with project-console content since 2026-05-28).

---

## VM-MediaKit — Phase 1 COMPLETE (6/6) [2026-05-29]

- [x] Ubuntu 24.04 QEMU provisioned (6 GiB RAM, 2 CPUs, TCG; port-forward NAT on GCP host)
- [x] 6/6 services migrated: proofreader (9092) · knowledge-documentation (9090) ·
  knowledge-corporate (9095) · knowledge-projects (9093) · marketing-pointsav (9101) ·
  marketing/woodfine (9102). All originals still running on host. No DNS changes.
- [x] `guide-vm-mediakit-provision` + `guide-vm-mediakit-service-migration` staged (commit 4a53d3af)
- [x] `topic-os-mediakit` corrected for Ubuntu 24.04
- [x] systemd units reorganised → `infrastructure/systemd/mediakit/`
- [ ] Binary-ledger sha256 entries — pending Stage 6 + nightly build rebuild

---

## VM-Totebox — Phase 1 (blocked on Command promoting project-data)

- [ ] **service-fs binary available on host** — blocked: Command must promote project-data.
  Outbox sent to project-data with install instructions.
- [ ] **VM-Totebox QEMU instance provisioned** — `infrastructure/virt/provision-vm-totebox.sh`
  (stub exists; implementation follows service-fs availability)
- [ ] **service-fs install inside VM-Totebox** — follows promotion
- [ ] **system-core + system-ledger install** — pending project-system outbox response (95 tests)

---

## VM-Orchestration — Phase 1

- [ ] **Provision VM-Orchestration** — `infrastructure/virt/provision-vm-orchestration.sh` (stub)
- [ ] **app-orchestration-bim (9096)** — depends on VM-Totebox service-fs
- [ ] **app-orchestration-gis instance**
- [ ] **app-orchestration-slm instance (:9180)**

---

## VM-PrivateGit — Phase 1 (future)

- [ ] **Provision VM-PrivateGit** — `infrastructure/virt/provision-vm-privategit.sh` (stub)
- [ ] **app-privategit-source-control install** (Gitea + SSH)
- [ ] **app-privategit-design-system** (Storybook)

---

## VM-Infrastructure — Phase 1 Resource Pool

- [x] `system-vm-fleet-types` scaffolded (4/4 tests pass) [session 12]
- [x] `service-vm-fleet` scaffolded (8/8 tests pass; axum :9203; advisory placement) [session 12]
- [x] `service-vm-host` scaffolded (5/5 tests pass; /proc/meminfo; QEMU monitor) [session 12]
- [x] `kvm_available` field + `prefer_kvm` placement; KVM-first with TCG fallback [session 12]
- [x] `vm_spawn` module: create_blank_disk + spawn_qemu + kill_qemu (14/14 tests) [session 13-14]
- [x] QEMU monitor Phase 2: QMP socket scan → VmState::Running (5/5 tests) [session 13-14]
- [x] GET /v1/nodes endpoint + all_nodes() [session 13-14]
- [x] service-vm-fleet deployed on GCP (:9203, gcp-cloud-1, kvm_available=false) [session 13-14]
- [x] service-vm-host deployed on GCP (heartbeating every 10s) [session 13-14]
- **Node roles:**
  - GCP e2-standard-8: fleet coordinator + TCG-only; `prefer_kvm: false` (e2 hard KVM block)
  - Laptop A (10.8.0.6): primary KVM compute; `prefer_kvm: true`
  - Laptop B (10.8.0.1): primary KVM compute (KVM TBD); `prefer_kvm: true`
- [ ] **GCP nested KVM: NOT available on e2.** Migrate to n2-standard-8 if needed later. Deferred.
- [ ] **Verify Laptop A KVM** — `ls /dev/kvm`; if absent: `sudo modprobe kvm kvm_intel`
- [ ] **Deploy service-vm-host on Laptop A + B** — copy binary + `/etc/default/vm-host`
- [ ] **Binary ledger entries (Command action)** — sha256 values:
  - service-ppn-pairing: `dc29e89ac6b0c12fc01407d4c4c7960477bbcab92efd3849d6b9260d10999137`
  - service-vm-fleet: (Command to measure)
  - service-vm-host: (Command to measure)
- [ ] **Stage 6 — session 15-16 commits** — 4-5 commits ahead of origin/main; promote when ready
- [ ] **software-units.yaml: add ppn-pairing-server :9205** (Command action)

---

## VM-Infrastructure — Phase 1 Genesis Protocol

- [x] Alpine Linux TCG proof-of-concept (`vm-prove.sh`) — virtio_balloon confirmed [session 7]
- [x] `service-ppn-pairing` deployed :9205 [session 13-14]
- [x] `service-ppn-pairing` normalize bug fix deployed (approve/deny working) [session 16]
- [ ] **Build + copy `os-network-admin` to Laptop A** — `cargo build --release -p os-network-admin`
  then `scp target/release/os-network-admin mathew@10.8.0.6:~/bin/`.
  Run with `PAIRING_SERVER=http://10.8.0.9:9205 ~/bin/os-network-admin`.
- [ ] **`provision-vm-infrastructure-cloud.sh --genesis`** — stub exists; implement after Q2–Q6
- [ ] **`provision-vm-infrastructure-onprem.sh`** — stub exists; implement after Q2–Q6
- [ ] **Deferred: os-network-admin ratatui TUI** — keyboard approve/deny; QR; expiry countdown

---

## Leapfrog 2030 — resource targets

Phase 3 targets: os-infrastructure 8 MB disk / 12 MB RAM idle; os-totebox 16 MB / 24 MB;
os-mediakit 24 MB† / 32 MB†; os-orchestration 12 MB / 18 MB; os-privategit 20 MB† / 24 MB†.
(† contingent on retiring MediaWiki/PHP and Gitea/Go respectively.)

- [ ] **Operator decision: retire MediaWiki/PHP for Rust static renderer** — gates os-mediakit P3
- [ ] **Operator decision: retire Gitea/Go for gitoxide-based server** — gates os-privategit P3
- [ ] **AArch64 hardware acquisition** — gates os-infrastructure Phase 3 seL4 Microkit
- [ ] **Phase 2: NetBSD 11.0 provision scripts** — `provision-vm-infrastructure-netbsd.sh`

---

## os-mediakit seL4 — Phase 3 (planned — operator decision needed first)

- [ ] **Operator decision: AArch64 GCP C4A vs Firecracker x86_64 on Laptop A**
  Option A: AArch64 GCP C4A Arm instance — Microkit 2.2 native, formal proof (~$50-100/mo)
  Option B: Firecracker + WireGuard on Laptop A — x86_64, KVM-native, pragmatic, free

---

## TOPIC + GUIDE leg — drafts staged, awaiting project-editorial pickup

12 TOPIC pairs + 4 GUIDEs in `.agent/drafts-outbound/`. Pickup notice sent to project-editorial.

**PROSE-RESEARCH:** v0.2 re-staged to project-editorial (session 15). 6 editorial points applied.
Awaiting editorial review and acceptance.

---

## Code — Genesis Protocol (gated on Q2–Q6)

- [ ] Rewrite `os-infrastructure/src/main.rs` — Genesis Protocol boot sequence (Step 1)
- [ ] Implement `system-substrate-broadcom/src/lib.rs` — silicon_ping() (Step 2)
- [ ] Implement `system-network-interface/src/lib.rs` — WireGuard/mDNS substrate (Step 3)
- [ ] Short-code pairing ceremony for node join (Step 4)
- [ ] Replace F8 Gateway subprocess with HTTP to localhost:9080 (Step 5, gated on Q5)
- [ ] Replace JSON mesh payloads with 16-byte binary protocol (Step 6)
- [ ] Add focus crates to root `Cargo.toml` workspace members (Step 8)

---

## Blocking — operator decisions needed first (Q2–Q6)

- [ ] **Q2: Ratify `10.50.0.0/24` as canonical PPN subnet** (code hardcodes 10.50.0.x)
- [ ] **Q3: Provide GCP static IP for cloud relay** (guide-provision-relay.md placeholder)
- [ ] **Q4: Confirm Laptop B local IP + network.woodfinegroup.com DNS status**
- [ ] **Q5: Is service-slm Doorman deployed at localhost:9080?** (gates Step 5 above)
- [ ] **Q6: Stage 6 + editorial pickup confirmed** (flag to Command Session; TOPIC drafts 7+ days)

---

## GUIDE leg — cross-repo fix (Command Session scope)

- [ ] `fleet-infrastructure-leased/guide-deploy-vpn.md` — fix hardcoded path
  `$HOME/Foundry/pointsav-monorepo/` → `/srv/foundry/vendor/pointsav-monorepo/`
  Lives in `customer/woodfine-fleet-deployment` — Command Session admin-tier.

---

## Completed this cluster (archived for reference)

- [x] Sweep project-intelligence contamination from archive (session 1)
- [x] Fix session-start.md, manifest.md, NEXT.md, memory init (session 1)
- [x] Stage sovereign-mesh.md + .es.md drafts (session 2)
- [x] Fix os-infrastructure/Makefile + forge_iso.sh paths (session 2)
- [x] Gitignore build artifacts in os-infrastructure/ and os-network-admin/ (session 2)
- [x] Create app-infrastructure-onprem/-leased/-cloud/ Reserved-folder scaffolds (session 2)
- [x] PPN architecture: BRIEF-PPN-ARCHITECTURE.md (385 lines, 57 citations) (session 7)
- [x] vm-prove.sh Alpine TCG proof: virtio_balloon confirmed (session 7)
- [x] service-ppn-pairing deployed :9205 (session 13-14)
- [x] service-vm-fleet + service-vm-host deployed on GCP (session 13-14)
- [x] vm_spawn module + QEMU monitor Phase 2 (session 13-14)
- [x] PROSE-RESEARCH v0.2 editorial revision (session 15)
- [x] service-ppn-pairing normalize bug fix + 4 integration tests (session 15)
- [x] service-ppn-pairing fixed binary deployed to :9205 (session 16)
