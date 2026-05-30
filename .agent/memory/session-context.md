## Session context — rolling 3-session summary

---

### 2026-05-30 | totebox@project-console | claude-sonnet-4-6

**Done this session:**
- Phase A complete (prior session): Doorman port 8011→9080 fixed everywhere; Phase 5 marked complete in BRIEFs; `BRIEF-cross-platform-release.md` created; Stage 6 force-push executed (`009b2e04`); outbox sent to Command.
- Phase B complete (this session): Cross-platform release infrastructure.
  - `rust-toolchain.toml` (stable channel pinned at monorepo root)
  - `.github/workflows/release.yml` rewritten: 4-target matrix (Linux musl via cargo-zigbuild on ubuntu-22.04, macOS Intel 10.13+ on macos-13, macOS ARM 11.0+ on macos-14, universal lipo); trigger on `v*.*.*` tag + workflow_dispatch; `softprops/action-gh-release@v2`
  - `reqwest` TLS switched from default (native-tls) to `rustls-tls` in all 4 workspace members: app-console-keys, app-console-content, app-console-input, app-console-system
  - `TerminalCaps` struct added to `app-console-keys/src/chassis.rs`: fields `kitty`, `sixel`, `truecolor`; detected from `Picker::protocol_type()` (ratatui-image 9.0.0) + `COLORTERM` env var; stored on `AppConsoleKeys`; populated in `run_local()` after probe; exposed via `caps()` accessor
  - `cargo check --workspace` exits 0; committed `6f21f580` (Jennifer Woodfine)
- B2 (`.cargo/config.toml`) omitted: `[env]` in Cargo config has no target-conditional syntax; `MACOSX_DEPLOYMENT_TARGET` correctly set per-job in CI workflow.
- `pointsav-monorepo/NEXT.md` updated: Phase B complete; Phase C/D/E added as next items.

**Pending / carry-forward:**
- Stage 6 for Phase B commit (`6f21f580`) + prior Phase A commit (`009b2e04`): need `bin/promote.sh` from Command Session. Outbox sent (prior session).
- **Phase C** — Email cartridge (F3): convert `app-console-email` stub → lib; implement `EmailCartridge` (inbox/read/compose via `service-email` in `project-data`); wire into `os-console`.
- **Phase D** — SLM cartridge (F9): convert `app-console-slm` stub → lib; implement `SlmCartridge` (Doorman health at 9080, Yo-Yo tier display); wire into `os-console`.
- **Phase E** — Orchestration wiring: audit `mba_client.rs`; rename `totebox_host` → `orchestration_host` in `ConsoleConfig`.
- Phase 6: offline mode + Tantivy full-text search (original coding roadmap).
- Tag `v0.1.0` triggers GitHub Actions release (after Stage 6 + canonical promote).

**Operator preferences surfaced:** (none new this session)

---

### 2026-05-29 | totebox@project-console | claude-sonnet-4-6

**Done this session:**
- Editorial routing only. No code changes.
- Inventoried `.agent/drafts-outbound/` (176 files). Identified 8 project-console drafts at `draft-ready-for-language-pass`: 6 TOPICs (editorial-pipeline-three-stages, customer-tier-catalog-pattern, machine-based-authorization, os-console-platform, input-machine, pointsav-private-network) + 2 GUIDEs (mba-pairing-ceremony, os-console-operator).
- Prepended outbox message `project-console-20260529-editorial-route` to `totebox@project-editorial` listing all 8 ready drafts with their types and routing destinations.
- `topic-language-protocol-substrate.md` held back — skeleton only, at `draft-pending-language-pass`.
- JOURNAL manuscripts (J1–J6) noted but not routed — they follow the separate JOURNAL submission workflow; J5 is on explicit HOLD.

**Pending / carry-forward:**
- Stage 6 push: waiting Command authorization for force-push (orphan branch divergence). See inbox `command-20260522-console-stage6-orphan-branch`.
- **Pre-Phase 6 blocker:** doorman port — verify 9080 vs 8011. Check `slm/endpoint.txt` and `pairings.yaml`. Update `app-console-content/src/draft.rs` + ContentCartridge if 9080 is correct.
- Phase 6: offline mode + Tantivy full-text search.
- Archive-level NEXT.md (`/srv/foundry/clones/project-console/NEXT.md`) has project-infrastructure contamination — Command decision pending.
- `topic-language-protocol-substrate.md` needs a substantive writing pass before language gate.

**Operator preferences surfaced:** (none new this session)

---

### 2026-05-28 | totebox@project-console | claude-sonnet-4-6

**Done this session:**
- Startup only. No code changes.
- NOTAM hazard resolved: 17 health alerts (doorman-unreachable, services-down) cascaded from 2026-05-27T00:34Z. Root cause: `slm-doorman-server` killed by SIGTERM after spin-loop on shadow brief `84DEA8VZHK0XNXW0JD1FERH3WX`. Apprenticeship queue was empty; restarted `local-doorman` — service now healthy.
- Doorman port discrepancy discovered: service binds `127.0.0.1:9080` per systemd logs; manifest + Phase 3 code note says `localhost:8011`. `app-console-content` code references 8011. Must resolve before Phase 6 work (offline mode polls doorman healthz). Added to monorepo NEXT.md.
- Archive-level NEXT.md (`/srv/foundry/clones/project-console/NEXT.md`) contains project-infrastructure content ("NEXT.md — project-infrastructure (cluster/project-infrastructure branch)") — contamination noted in outbox for Command.
- Updated monorepo `NEXT.md`: Phase 5 → Complete; Phase 6 → Current (was still showing Phase 5 as Current since `894452c1` shutdown update didn't land in monorepo on rebased main).

**Pending / carry-forward:**
- Stage 6 push: waiting Command authorization for force-push. See outbox `project-console-20260522-stage6-history-divergence`.
- **Pre-Phase 6 blocker:** doorman port — verify which is authoritative (9080 from service log vs 8011 from manifest). Check `slm/endpoint.txt` and `pairings.yaml`. If 9080 is correct, update code references in `app-console-content/src/draft.rs` and `ContentCartridge`.
- Phase 6: offline mode + Tantivy full-text search.
- Pairing-server systemd unit, GCE firewall port 2222, Peter's SSH key — Command/operator.
- Archive-level NEXT.md replacement (currently has project-infrastructure content) — Command decision needed.

**Operator preferences surfaced:** (none new this session)


