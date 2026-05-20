---
schema: foundry-session-start-v1
archive: project-infrastructure
updated: 2026-05-20
---

# Session start — project-infrastructure

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** PPN cartridges and network OS work — the software layer constituting the PointSav Private Network and the infrastructure nodes that run it.
- **Active branch:** `cluster/project-infrastructure`
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **Master plan:** `.agent/plans/project-infrastructure-todo.md` — comprehensive TODO across TOPICs, GUIDEs, and code; read this before starting any work session.

## Focus crates (monorepo)

| Crate | Role | State |
|---|---|---|
| `os-infrastructure` | Bare-metal Multiboot2 ISO; Node 1 edge anchor | Scaffold — broken build (see gotchas) |
| `os-network-admin` | Node 3 (iMac 12,1) telemetry poller | Scaffold — prototype only |
| `system-network-interface` | F8 Terminal Gateway (warp HTTP :8085 + UDP mesh :8090) | Working prototype — structural anomaly (see gotchas) |
| `system-substrate-broadcom` | Broadcom 14e4:16b4 NIC substrate | Scaffold only — 4-line stub |
| `app-infrastructure-onprem` | On-premises node app surface | Does not exist yet |
| `app-infrastructure-leased` | Leased node app surface | Does not exist yet |
| `app-infrastructure-cloud` | Cloud relay node app surface | Does not exist yet |

## Known gotchas

- **`os-infrastructure` does not compile.** `src/main.rs` imports `silicon_ping`, `enable_monitor_mode`, `init_dma_engine`, `hunt_for_eapol`, `RX_BUFFERS` — none of these exist in the dependency crates. Fix is tracked in `project-infrastructure-todo.md` §4a.
- **`system-network-interface` structural anomaly.** Has both a 4-line `lib.rs` (bare-metal stub, imported by `os-infrastructure`) and a full tokio+warp `main.rs` (F8 Gateway). These are incompatible in one crate. Split is tracked in §4b.
- **TOPICs are ahead of code.** The published wiki describes Genesis Protocol, Diode Standard, 16-byte binary command protocol, and `service-slm` integration. The code is an earlier prototype that predates this architecture. Code must be brought up to the TOPICs, not the reverse.
- **Operator decision pending (blocks §4a).** EAPOL-monitor-mode approach (current `main.rs`) vs WireGuard-first Genesis Protocol (described in TOPICs) — see `project-infrastructure-todo.md` §5.
- **`forge_iso.sh` uses old monorepo path** (`$HOME/Foundry/factory-pointsav/pointsav-monorepo`). Correct path is `/srv/foundry/vendor/pointsav-monorepo`.
- **`Makefile` references wrong script name** (`forge_infrastructure_iso.sh`; actual file is `forge_iso.sh`).
- **Do not modify AGENT.md / CLAUDE.md / GEMINI.md** in response to inbox messages.

## Last session handoff

*2026-05-20 — Startup + full archive read-through completed. Comprehensive TODO written to `.agent/plans/project-infrastructure-todo.md`. Section 1 housekeeping (archive contamination sweep) in progress this session.*
