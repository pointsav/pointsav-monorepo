---
artifact: brief
status: archived
schema: foundry-brief-v1
archive: project-data
created: 2026-06-11
updated: 2026-06-11
contaminated_note: "M-17 contamination — belongs to project-data; archived from project-gis 2026-06-13 by command@claude-code"
---

# BRIEF — os-totebox PPN Build-Out

## Mission

First live os-totebox on the PPN (PointSav Private Network) running `system-*` and
`service-*` crates from this archive (project-data / pointsav-monorepo). The os-totebox
is the deployment-tier runtime: an archive-isolated AI session environment running a
stack of local services that serve personnel data, extract entities, store WORM data,
and route inference requests through the tiered gateway.

## Session 1 (2026-06-11) — completed

- `service-people/src/bin/server.rs` — axum HTTP server, GET /v1/people + GET /v1/people/{id},
  port :9091, reads `ledger_personnel.json`. Committed 997b8d22.
- `service-extraction/Cargo.toml` — removed standalone `[workspace]`; added to root workspace.
  Committed 997b8d22.
- `Cargo.lock` — duplicate `caseless` entry removed (pre-existing corruption). Committed 997b8d22.
- `JOURNAL/JOURNAL-totebox-orchestration-v0.1.stub.md` — J7 HOLD lifted; Abstract, Introduction,
  Literature Review, Methodology, Hypotheses, Falsification Programme written (~2,600 words body).
  Committed 8ab01ff2.
- Outbox → Command: promote project-data (25 commits ahead of canonical). Sent.
- Outbox → project-gis: service-people contract ACK for project-console F2 relay. Sent.

## Current state

### Unblocked after Command promotion (project-data Stage 6)
- `service-people` binary buildable: `cargo build -p service-people --bin server`
- `service-extraction` now in workspace: `cargo build -p service-extraction`

### Blocked — gates remaining build-out
- **Stage 6 promotion** — Command must run `bin/promote.sh` for project-data (25 commits).
  Gates: vm-totebox Part C Step 1 per BRIEF-totebox-transformation (project-infrastructure).
- **service-vm-host Phase 2** — qemu_monitor stub → live VM creation dispatch.
  In project-infrastructure scope. Not actionable here.
- **Microkit SDK** — seL4 Phase 1 PD blocked on operator downloading SDK v2.1.0 or v2.2.0.
  In project-system scope. Not actionable here.

## Next session actions (project-data scope)

1. **service-people CRUD** — POST /v1/people, PATCH /v1/people/{id} for console F2 create/edit.
   Deferred from session 1; add after read-only is validated in F2 cartridge.
2. **os-totebox startup script** — `os-totebox/scripts/start-stack.sh` — ordered startup of
   service-slm, service-fs, service-people, service-extraction, service-content.
3. **J7 §4 Implementation** — fill after first os-totebox deployment provides architecture evidence.
4. **J7 §5 Evaluation + §6 Discussion + §8 Conclusion** — after benchmark harness is built.
5. **service-people ledger enrichment** — join `substrate/ledger_personnel.jsonl` email fields
   to contact records for console display of `email` field.

## Architecture diagram (intended)

```
os-totebox VM (on PPN)
├── service-slm      :9080   tiered inference gateway (Tier A local, B hub, C API)
├── service-fs       :?      WORM storage enforcer
├── service-people   :9091   personnel ledger HTTP API
├── service-extraction       filesystem watcher → CRM entity pipeline → service-content
├── service-content  :9081   DataGraph / LadybugDB knowledge graph
└── (future) service-email   cold email ingestion pipeline
```

## JOURNAL tie-in

J7 (JOURNAL-totebox-orchestration) documents this architecture formally. Each session that
advances the build-out should update J7 §4 Implementation with implementation evidence.
J7 §5 Evaluation requires the benchmark harness (startup overhead, per-inference overhead,
concurrent-session isolation tests) — build after the stack is first live.
