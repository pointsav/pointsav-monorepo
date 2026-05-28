# NEXT.md — project-infrastructure (cluster/project-infrastructure branch)

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.
> Full TODO with all sections and sequencing: `.agent/plans/project-infrastructure-todo.md`.

Last updated: 2026-05-27 (session 4).

---

## Dev-environment bootstrap (unblocked — activate first ceremony)

- [ ] **Deploy `service-ppn-pairing` on GCP VM** — build release binary + install systemd unit
  `infrastructure/systemd/local-ppn-pairing.service`. Listens on `0.0.0.0:9202`.
  [2026-05-28 totebox@claude-code]

- [ ] **Build + copy `os-network-admin` to Laptop A (iMac)** — `cargo build --release -p os-network-admin`
  then `scp target/release/os-network-admin mathew@10.8.0.6:~/bin/`. Run with
  `PAIRING_SERVER=http://10.8.0.9:9202 ~/bin/os-network-admin`. [2026-05-28 totebox@claude-code]

- [ ] **Run `infrastructure/virt/vm-prove.sh` on Laptop A** — boots Alpine Linux VM with
  QEMU/KVM; proves Totebox services can run inside a VM reachable over the PPN mesh.
  Laptop A has real hardware VT-x; GCP VM needs `--tcg` flag (nested virt not enabled).
  [2026-05-28 totebox@claude-code]

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

## TOPIC leg — drafts staged, needs editorial pickup

Five draft pairs in `.agent/drafts-outbound/`; pickup notice sent to project-editorial.

- [ ] `topic-sovereign-mesh` + `.es` — expands stub at `infrastructure/sovereign-mesh.md` [session 2]
- [ ] `topic-genesis-protocol` + `.es` — new; `architecture/genesis-protocol.md` [session 3]
- [ ] `topic-ppn-command-protocol` + `.es` — new; `architecture/ppn-command-protocol.md` [session 3]
- [ ] `topic-service-pointsav-link` + `.es` — new; `architecture/service-pointsav-link.md` [session 3]

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
