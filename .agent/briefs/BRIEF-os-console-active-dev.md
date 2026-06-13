---
artifact: brief
schema: foundry-brief-v1
brief-id: project-console-os-console-active-dev
owner: project-console
title: "os-console Active Development — Phases 8–10"
status: active
created: 2026-06-12
updated: 2026-06-12
authors: [totebox@project-console, claude-sonnet-4-6]
doctrine_anchors: [claim-45, claim-49, claim-54, SYS-ADR-07, SYS-ADR-10, SYS-ADR-19]
companion: BRIEF-project-console-master.md
---

# os-console Active Development — Phases 8–10

> **Read this before any session.** Supersedes `BRIEF-project-console-master.md` for active
> phase tracking. Architecture detail (F-key map, Cartridge trait, MBA topology, platform targets)
> lives in `BRIEF-os-console-platform.md` in the monorepo sub-clone.

---

## §1 — State snapshot (2026-06-12)

| Phase | Status | Key commits |
|---|---|---|
| Phase C — Email cartridge (F3) | ✓ complete | — |
| Phase D — SLM cartridge (F9) | ✓ complete | — |
| Phase E — Orchestration wiring | ✓ complete | — |
| Phase 6 — Offline mode + Tantivy search | ✓ complete | — |
| Phase 7 — PDF viewing | ✓ complete | — |
| Cross-platform Phase A | ✓ complete | `009b2e04` |
| Cross-platform Phase B | ✓ complete; Stage 6 pending | `6f21f580` |
| Phase 8 — Polish | not started | — |
| Phase 9 — Operations | not started | — |
| Phase 10–13 | deferred | — |

---

## §2 — Stage 6 status

**Phase B commits awaiting push to staging mirrors + canonical promote:**

| SHA | Subject |
|---|---|
| `6f21f580` | feat(release): Phase B — CI matrix, rustls-tls, TerminalCaps |
| `d9261705` | ops(session): Phase B complete |
| `d58960b4` | ops(brief): mark Phase B complete |

Force-push to staging mirrors **authorized by Command 2026-05-28** (inbox `command-20260528-console-answers`).
Outbox message sent 2026-06-12 requesting Command action (push + promote-queue.jsonl entry).

---

## §3 — Phase 8: Polish

**Gate criterion:** all items below complete; `cargo check --release` clean on all 4 targets.

- [ ] OSC 8 hyperlinks in ContentCartridge (editor navigation)
- [x] Truecolor Rgb variants — email/slm/system/input cartridges (2026-06-12; commit pending `cargo check`)
- [ ] Multi-tab support (multiple open ContentCartridge documents)
- [ ] Session persistence across SSH reconnect (state serialized to local file)
- [x] `/audit` log viewer command — already fully implemented in InputCartridge; confirmed (2026-06-12)
- [x] F2 People cartridge — `app-console-people` scaffold created; service-people fetch + j/k/detail/error states; registered at F2 (2026-06-12; commit pending `cargo check`)

**UX contract** (enforce per `BRIEF-project-console-master.md` §6 — preserved here):
- `--plain` mode: every cartridge functions fully with no terminal graphics
- Keyboard ergonomics: Esc=cancel, Enter=confirm, Tab=focus-next, F1–F12=chassis-routed
- Semantic colors: red+✗ error; green+✓ success; yellow+⚠ warning; DarkGray inactive

---

## §4 — Phase 9: Operations

**Gate criterion:** `local-console.service` unit deployed on vault-privategit-source-1 and healthy.

Blocked on: GCE firewall port 2222 (operator action); vm-intelligence WireGuard provisioning (project-infrastructure).

- [x] `local-console.service` systemd unit — drafted at `infrastructure/systemd/console/local-console.service` (2026-06-12; commit pending `cargo check`)
- [x] `pairing-server` systemd unit — drafted at `infrastructure/systemd/console/local-pairing-server.service` (2026-06-12; commit pending `cargo check`)
- [ ] Prometheus metrics exporter on configurable port
- [ ] fail2ban config for port 2222 (SSH brute-force protection)
- [ ] Graceful SIGTERM handling in os-console binary

---

## §5 — Phase 10–13 (deferred)

| Phase | What | Gate |
|---|---|---|
| 10 | F7 BIM cartridge (`app-console-bim`) | project-bim Phase 1 service ready |
| 11 | F10 mesh cartridge (`app-console-mesh`) | PPN mesh operational |
| 12 | Chassis auto-reconnect watchdog | Phase 9 complete |
| 13 | F2 People cartridge full CRUD | service-people CRUD (project-data) |

---

## §6 — Operator-gated items

- [ ] GCE firewall rule: port 2222 inbound — required for external SSH connections
- [ ] Peter SSH key — generate Ed25519; `proofctl user add peter --tenant woodfine --role editor`
- [ ] Tag `v0.1.0` on pointsav-monorepo — triggers GitHub Actions release build
- [ ] Branch rename `cluster/project-proofreader → cluster/project-console` on GitHub

---

## §7 — Decisions locked

- 2026-05-28: Force-push Phase B to staging mirrors authorized by Command.
- 2026-05-31: Doorman endpoint confirmed at `http://localhost:9080` (not 8011) — fixed in `009b2e04`.
- 2026-05-31: `--plain` mode, keyboard ergonomics contract, semantic color states extracted from Gemini audit.
- 2026-06-12: This BRIEF created; `BRIEF-project-console-master.md` remains archived (historical).

## §8 — Decisions open

- [x] Phase 8 start — proceeding without Stage 6 Phase B promote (local coding unblocked)
- [x] Phase 9 systemd unit design — drafted local-console.service + local-pairing-server.service

## §9 — Work log

2026-06-12 totebox@claude-code: Phase 8 coding sprint begun.
  - Truecolor Rgb variants added to EmailCartridge, SlmCartridge, SystemCartridge, InputCartridge
  - F2 PeopleCartridge scaffold: app-console-people/src/{lib,cartridge}.rs; service-people fetch; j/k/Enter/Esc navigation; truecolor-aware; registered at F2 in os-console/src/main.rs
  - people_endpoint added to ProfileConfig (default http://127.0.0.1:9091)
  - Systemd unit drafts: local-console.service, local-pairing-server.service
  - TOPIC-os-console-architecture.draft.md + .es stub → drafts-outbound
  - All commits pending cargo check clean

2026-06-12 totebox@claude-code: Created this BRIEF; state files (NEXT.md, manifest.md) repaired; contamination sweep complete.
