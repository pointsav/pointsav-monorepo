# NEXT.md — project-infrastructure (cluster/project-infrastructure branch)

> **Scope: this archive only.** Cross-repo and workspace-level items live at `~/Foundry/NEXT.md`.
> Full TODO with all sections and sequencing: `.agent/plans/project-infrastructure-todo.md`.

Last updated: 2026-05-20 (session 2).

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

## Code — fix broken build

- [ ] **Resolve missing symbols in `os-infrastructure/src/main.rs`** — blocked on EAPOL
  vs Genesis Protocol decision above. [2026-05-20 task@claude-code]

---

## TOPIC leg — drafts staged, needs editorial pickup

- [ ] **Pick up `topic-sovereign-mesh.draft.md` + `.es.md` from drafts-outbound**
  Staged at `.agent/drafts-outbound/`. Message sent to project-editorial outbox.
  Full PPN architecture topic — expands the one-sentence stub at
  `infrastructure/sovereign-mesh.md` in content-wiki-documentation.
  [2026-05-20 task@claude-code]

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
