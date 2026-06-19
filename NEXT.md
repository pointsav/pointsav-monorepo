# NEXT.md — project-console

> Totebox Session — starts in `/srv/foundry/clones/project-console`
> Phase 10 complete 2026-06-16. Phase 11 (F7 BIM) blocked on project-bim Phase 1.

---

## Phase 11 — F7 BIM cartridge (blocked)

- [ ] `app-console-bim` activation — blocked on project-bim Phase 1 service (no ETA from project-bim)

---

## Carry-forward diagnostics

These require operator access to iMac / vault-privategit-source-1 to diagnose:

- [ ] os-console exits immediately after MBA error — run binary on MBA, capture full stderr, check binary age vs last known-good build
- [ ] Port 9093 "Address already in use" on iMac — non-blocking; identify which process holds the bind (`lsof -i :9093`) before next os-console launch
- [ ] local-console.service on GCE VM — verify `systemctl status local-console.service` on vault-privategit-source-1; gate: operator must open GCE firewall port 2222 first

---

## Phase H1 — seL4 substrate + VirtIO clipboard (unblocked after H0 Alpine)

- [ ] Fill in `moonshot-sel4-vmm` (~300 lines): `_start()`, seL4 ABI wrappers, `microkit_msginfo_t`,
      `notified()` + `protected()` callbacks — blocks all seL4 unikernel work
- [ ] Boot os-console as single seL4 PD in QEMU: `moonshot-toolkit build examples/os-console-sel4.toml`
- [ ] VirtIO clipboard in `moonshot-hypervisor` (non-optional): arboard host-side + VirtIO clipboard
      protocol guest-side; SMB operators require paste from host apps into cartridges
- [ ] VirtIO serial PD (~200 lines): ratatui output via VirtIO console; keyboard input
- [ ] smoltcp network PD (~400 lines MIT, vendorable): HTTP to test Totebox; replaces reqwest

**Blocked on:** H0 Alpine/QEMU guest validation (see BRIEF-os-console-hypervisor.md §10 Phase H0).
Outbox to project-data sent 2026-06-19 to start parallel os-totebox + os-orchestration seL4 work.

---

## Drafts-outbound

- [ ] project-editorial pickup pending — TOPIC-geometric-protection, TOPIC-os-console-totebox-browser,
      TOPIC-sel4-unikernel-substrate, TOPIC-three-binary-architecture (EN+ES = 8 files) + 2 GUIDEs
      staged to drafts-outbound 2026-06-19
- [ ] project-editorial pickup pending — editorial/research drafts routed 2026-06-19 (outbox sent)
- [ ] project-design pickup pending — DESIGN-* drafts routed 2026-06-19 (outbox sent)
