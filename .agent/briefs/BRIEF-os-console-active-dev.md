---
artifact: brief
schema: foundry-brief-v1
brief-id: project-console-os-console-active-dev
owner: project-console
title: "os-console Active Development — Phases 8–11"
status: active
created: 2026-06-12
updated: 2026-06-19
authors: [totebox@project-console, claude-sonnet-4-6]
doctrine_anchors: [claim-45, claim-49, claim-54, SYS-ADR-07, SYS-ADR-10, SYS-ADR-19]
companion: BRIEF-project-console-master.md
---

# os-console Active Development — Phases 8–11

> **Read this before any session.** Supersedes `BRIEF-project-console-master.md` for active
> phase tracking. Architecture detail (F-key map, Cartridge trait, MBA topology, platform targets)
> lives in `BRIEF-os-console-platform.md` in the monorepo sub-clone.

---

## §1 — State snapshot (2026-06-19)

| Phase | Status | Key commits |
|---|---|---|
| Phase C — Email cartridge (F3) | ✓ complete | — |
| Phase D — SLM cartridge (F9) | ✓ complete | — |
| Phase E — Orchestration wiring | ✓ complete | — |
| Phase 6 — Offline mode + Tantivy search | ✓ complete | — |
| Phase 7 — PDF viewing | ✓ complete | — |
| Cross-platform Phase A | ✓ complete | `009b2e04` |
| Cross-platform Phase B | ✓ complete; Stage 6 pending | `6f21f580` |
| Phase 8 — Polish | ✓ complete (partial items remain) | `ac7eb500`, `bc95acfa`, `abd8d019` |
| Phase 9 — Operations | in progress (SIGTERM done; firewall + metrics pending) | `d2afebfe`, `78941244` |
| Phase 10 — Sprint (truecolor + watchdog + People) | ✓ complete 2026-06-16 | `bc95acfa`, `abd8d019`, `fc4d0978` |
| Phase 11 — F7 BIM cartridge | blocked | project-bim Phase 1 |

---

## §2 — Stage 6 status

**All Phase 8–10 commits awaiting push to staging mirrors + canonical promote.**

Force-push to staging mirrors **authorized by Command 2026-05-28** (inbox `command-20260528-console-answers`).
Outbox messages sent 2026-06-12 and 2026-06-16 requesting Command action.

**Divergence blocker (high priority):** local `main` and `origin/main` have diverged at `5c36ce66`.
Do NOT force-push until Command Session resolves. See outbox `project-console-20260616-divergence-blocker`.

Full promote range: `6f21f580` through `fc4d0978` (includes Phase B + Phase 8 + Phase 9 + Phase 10).

| SHA | Subject |
|---|---|
| `6f21f580` | feat(release): Phase B — CI matrix, rustls-tls, TerminalCaps |
| `d9261705` | ops(session): Phase B complete |
| `d58960b4` | ops(brief): mark Phase B complete |
| `ac7eb500` | feat(truecolor+people): Phase 8 sprint — Rgb variants + PeopleCartridge scaffold |
| `7259d2a0` | feat(config): people_endpoint default :9091 |
| `d2afebfe` | feat(systemd): local-console.service + local-pairing-server.service |
| `78941244` | feat(sigterm): graceful SIGTERM handling — terminal restored on systemd stop |
| `bc95acfa` | feat(truecolor+session): Rgb color variants across all cartridges + content search persistence |
| `abd8d019` | feat(people): F2 PeopleCartridge scaffold — contact list + CRUD stubs; service-people at :9091 |
| `fc4d0978` | feat(watchdog): chassis auto-reconnect watchdog with exponential backoff |

---

## §3 — Phase 8: Polish

**Gate criterion:** all items below complete; `cargo check --release` clean on all 4 targets.

- [ ] OSC 8 hyperlinks in ContentCartridge (editor navigation)
- [x] Truecolor Rgb variants — all cartridges — `ac7eb500`, `bc95acfa`
- [ ] Multi-tab support (multiple open ContentCartridge documents)
- [x] Content search session persistence — `bc95acfa`
- [x] `/audit` log viewer command — already fully implemented in InputCartridge; confirmed (2026-06-12)
- [x] F2 People cartridge scaffold — `abd8d019`; service-people contract verified 2026-06-19

**UX contract** (enforce per `BRIEF-project-console-master.md` §6 — preserved here):
- `--plain` mode: every cartridge functions fully with no terminal graphics
- Keyboard ergonomics: Esc=cancel, Enter=confirm, Tab=focus-next, F1–F12=chassis-routed
- Semantic colors: red+✗ error; green+✓ success; yellow+⚠ warning; DarkGray inactive

---

## §4 — Phase 9: Operations

**Gate criterion:** `local-console.service` unit deployed on vault-privategit-source-1 and healthy.

Blocked on: GCE firewall port 2222 (operator action); vm-intelligence WireGuard provisioning (project-infrastructure).

- [x] `local-console.service` systemd unit — `infrastructure/systemd/console/local-console.service` — `d2afebfe`
- [x] `pairing-server` systemd unit — `infrastructure/systemd/console/local-pairing-server.service` — `d2afebfe`
- [x] Graceful SIGTERM handling in os-console binary — `78941244`
- [ ] Prometheus metrics exporter on configurable port
- [ ] fail2ban config for port 2222 (SSH brute-force protection)

---

## §5 — Phase 11–13 (deferred)

| Phase | What | Gate |
|---|---|---|
| 11 | F7 BIM cartridge (`app-console-bim`) | project-bim Phase 1 service ready |
| 12 | F10 mesh cartridge (`app-console-mesh`) | PPN mesh operational |
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
- 2026-06-16: Phase 10 sprint complete — truecolor across all cartridges, PeopleCartridge scaffold (F2), chassis watchdog.
- 2026-06-19: service-people HTTP API contract verified — endpoint `http://127.0.0.1:9091/v1/people`, struct fields match, wiring correct. No code changes required.
- 2026-06-19: We Own It principle adopted — all runtime deps must be tier 1 (ours) or tier 2 (vendored trusted; formally verified or BSD/MIT source we build); see conventions/we-own-it-principle.md (pending ratification at Command).

## §8 — Decisions open

- [x] Phase 8 start — proceeding without Stage 6 Phase B promote (local coding unblocked)
- [x] Phase 9 systemd unit design — drafted local-console.service + local-pairing-server.service
- [ ] SSH reconnect session persistence (full state serialization) — Phase 8 item partially addressed by content search persistence; full cross-reconnect state still open

## §9 — Work log

2026-06-19 totebox@claude-code: Cleanup session — inbox cleared, NEXT.md refreshed, BRIEF updated.
  - service-people contract (inbox msg command-20260618): F2 wiring verified clean; endpoint + struct match; archived
  - foundry-prod broadcast (inbox msg command-20260618): awareness noted; archived
  - Phase 11 BIM status: blocked on project-bim Phase 1 (no ETA)
  - drafts-outbound: routed to project-editorial and project-design via outbox
  - artifact-registry.md: updated with Phase 10 artifacts

2026-06-16 totebox@claude-code: Phase 10 sprint — 3 commits.
  - bc95acfa (Jennifer): Rgb color helpers + content search session persistence
  - abd8d019 (Peter): F2 PeopleCartridge scaffold — contact list, detail view, :9091 fetch loop, j/k nav
  - fc4d0978 (Peter): chassis auto-reconnect watchdog (exponential backoff 2s→60s cap)
  - BRIEF and NEXT.md updated; Stage 6 outbox sent to Command

2026-06-13 totebox@claude-code: Phase 8 coding sprint — all items committed.
  - Truecolor Rgb variants: EmailCartridge, SlmCartridge, SystemCartridge, InputCartridge (ac7eb500)
  - F2 PeopleCartridge scaffold: app-console-people/src/{lib,cartridge}.rs; service-people fetch; j/k/Enter/Esc navigation; truecolor-aware; registered at F2 (ac7eb500)
  - people_endpoint added to ProfileConfig default http://127.0.0.1:9091 (prior commit: 7259d2a0)
  - Systemd units: local-console.service + local-pairing-server.service (d2afebfe)
  - TOPIC-os-console-architecture.draft.md + .es stub → drafts-outbound (7d70bb6a)
  - Note: workspace cargo check has pre-existing tantivy-common/time conflict (not our code); targeted crate checks clean

2026-06-12 totebox@claude-code: Created this BRIEF; state files (NEXT.md, manifest.md) repaired; contamination sweep complete.
