# NEXT.md — project-infrastructure (cluster/project-infrastructure branch)

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.
> Full TODO with all sections and sequencing: `.agent/plans/project-infrastructure-todo.md`.

Last updated: 2026-05-20.

---

## Blocking — operator decisions needed first

- [ ] **Decide: EAPOL-monitor-mode vs Genesis Protocol** for `os-infrastructure/src/main.rs`
  Current code does WiFi NIC EAPOL capture. Published TOPICs describe a WireGuard-first
  Genesis Protocol (seL4 keypair at boot; holding pattern; admin claim). Which path?
  [2026-05-20 task@claude-code]

- [ ] **Ratify `10.50.0.0/24` as the canonical PPN subnet**
  Code hardcodes `10.50.0.1/2/3`; guides say `10.x.x.x/24` (unspecified). Confirm range,
  then update `route-network-admin/guide-mesh-orchestration.md` + create `INVENTORY.yaml`.
  [2026-05-20 task@claude-code]

- [ ] **Provide GCP static IP for cloud relay**
  Needed to complete `fleet-infrastructure-cloud/guide-provision-relay.md` and
  `os-network-admin/scripts/mesh_status.sh` (currently `[ENTER_YOUR_GCP_STATIC_IP_HERE]`).
  [2026-05-20 task@claude-code]

- [ ] **Confirm Laptop A / Laptop B local IPs and `network.woodfinegroup.com` DNS status**
  `guide-deploy-vpn.md` has `<LOCAL_IP_OF_LAPTOP_B>` placeholder.
  `guide-mesh-execution.md` references `https://network.woodfinegroup.com`.
  [2026-05-20 task@claude-code]

---

## Code — fix broken build (no decisions needed)

- [ ] **Fix `os-infrastructure/Makefile`** — references `forge_infrastructure_iso.sh`;
  actual file is `forge_iso.sh`. One-line fix. [2026-05-20 task@claude-code]

- [ ] **Fix `os-infrastructure/forge_iso.sh`** — hardcoded path
  `$HOME/Foundry/factory-pointsav/pointsav-monorepo`; correct to
  `/srv/foundry/vendor/pointsav-monorepo`. [2026-05-20 task@claude-code]

- [ ] **Gitignore build artifacts** — `build_iso/*.elf`, `build_iso/staging/`, `*.iso`,
  `linker.ld` in `os-infrastructure/` and `os-network-admin/`. [2026-05-20 task@claude-code]

- [ ] **Resolve missing symbols in `os-infrastructure/src/main.rs`** — blocked on EAPOL
  vs Genesis Protocol decision above. [2026-05-20 task@claude-code]

- [ ] **Split `system-network-interface`** — `lib.rs` (bare-metal stub) and `main.rs`
  (F8 Gateway binary) cannot coexist in one crate. Extract gateway to `app-network-admin/`
  or equivalent. [2026-05-20 task@claude-code]

---

## TOPIC leg — one gap

- [ ] **Expand `infrastructure/sovereign-mesh.md` from stub to full topic**
  Currently one sentence (since 2026-05-07). Needs: WireGuard overlay, hub-spoke topology,
  `ppn0` interface, 16-byte command packets on port 8090, Genesis Protocol relationship.
  Stage to `drafts-outbound/` → project-editorial. [2026-05-20 task@claude-code]

---

## GUIDE leg — misaligned files (fix regardless of operator decisions)

- [ ] **`fleet-infrastructure-leased/guide-deploy-vpn.md`** — fix hardcoded path
  `$HOME/Foundry/pointsav-monorepo/` → `/srv/foundry/vendor/pointsav-monorepo/`.
  [2026-05-20 task@claude-code]

---

## App surface scaffolds (low effort, no decisions needed)

- [ ] Create `app-infrastructure-onprem/` — `README.md` + `README.es.md`
- [ ] Create `app-infrastructure-leased/` — `README.md` + `README.es.md`
- [ ] Create `app-infrastructure-cloud/` — `README.md` + `README.es.md`
  [2026-05-20 task@claude-code]
