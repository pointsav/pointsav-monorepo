# CLAUDE.md — service-slm

> **State:** Active  —  **Last updated:** 2026-04-25
> **Registry row:** `pointsav-monorepo/.claude/rules/project-registry.md`
>
> When state changes, update this header AND the registry row in the
> same commit. Drift between the two is a documentation defect.

---

## What this project is

service-slm is the single secure boundary between the isolated
Totebox Archive and any external Large Language Model. It implements
the Doorman Protocol: sanitise outbound payloads, route them to
external compute (local if the host has the resources, otherwise the
yo-yo substrate on GCP), receive structured deltas, and rehydrate
them back into the ledger. service-slm does not generate text; it
gates compute. Read-side sibling for long-term semantic memory is
service-content — see `ARCHITECTURE.md` Ring 3a.

## Current state

**Scaffold-coded → Active on 2026-04-23** as part of the
`cluster/service-slm` first-live cluster (now `cluster/project-slm`
per the v0.0.7 cluster handoff).

**B1 scaffolded on 2026-04-25** (Phase B Doorman task list, inbox
v0.0.7). The standalone-vs-nested workspace question is closed —
service-slm is its own cargo workspace at `service-slm/Cargo.toml`
with three crates under `crates/`:

- `slm-core` — shared types: `ModuleId`, `RequestId` (UUIDv7), `Tier`
  enum (Local / Yoyo / External), `Complexity`, `ComputeRequest`,
  `ComputeResponse`, error types. No async, no I/O.
- `slm-doorman` (lib) — three-tier router skeleton (`router.rs`),
  three tier-client modules (`tier/local.rs`, `tier/yoyo.rs`,
  `tier/external.rs`), append-only JSONL audit ledger
  (`ledger.rs`). Tier A makes real HTTP calls against an
  OpenAI-compatible endpoint; Tier B (`B2`) and Tier C (`B4`) are
  stubs that return `DoormanError::NotImplemented`.
- `slm-doorman-server` (bin) — axum HTTP server. Endpoints:
  `/healthz`, `/readyz`, `/v1/contract`, `POST /v1/chat/completions`.
  Boots cleanly with no Yo-Yo configured (community-tier mode per
  Optional Intelligence; `B5` verification path).

The pre-existing `cognitive-forge/` subcrate remains in place but
is `exclude`d from the workspace; its rename (paired with
`tool-cognitive-forge`) is unchanged.

Three known defects at project root remain unchanged (queued
separately):

Three known defects at project root:

1. `cognitive-bridge.sh` is a placeholder carrying an explicit
   "MISSING CONNECTION PHYSICS: system-slm" block (lines 31–46). It
   does not route payloads to an SLM — the connection vector is
   undefined. Layout-hygiene defect queued in the monorepo `NEXT.md`
   for move to `scripts/`.
2. `transient-queues/` contains eight `TX-*_skeleton.txt` files that
   mirror the `discovery-queue` "Not-a-project" pattern in the
   registry — runtime payload state bleeding into Git. Triage
   pending; see `NEXT.md`.
3. The `cognitive-forge/` subcrate carries the term "Cognitive
   Forge," which is on the project's Do-Not-Use list per the
   cleanup-log. The sibling `tool-cognitive-forge` has a rename
   queued in the monorepo `NEXT.md` rename series; this subcrate is
   not yet flagged but inherits the same concern.

The handoff between `cognitive-forge` (writes markdown) and
`service-content/content-compiler` (parses JSON) is currently
inconsistent — the writer and reader do not interoperate as-is.
Flagged for reconciliation; see `NEXT.md`.

Yo-Yo Compute Phase 1 has not started in code form. `ARCHITECTURE.md`
captures the target shape; `DEVELOPMENT.md` captures the migration
roadmap.

## Build and test

```
cargo check --workspace                # Phase 1: ~1m40s cold, seconds incremental
cargo test  --workspace                # 6 unit tests as of B1
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt   --all -- --check
```

End-to-end against a real Tier A endpoint:

```
SLM_LOCAL_ENDPOINT=http://127.0.0.1:8080 \
SLM_BIND_ADDR=127.0.0.1:9080 \
    cargo run -p slm-doorman-server
```

`SLM_YOYO_ENDPOINT` is intentionally unset by default — community-
tier mode. Setting it activates Tier B (currently a stub returning
`NotImplemented` until B2 lands). The legacy `cognitive-forge/`
subcrate still builds in isolation:

```
cargo build --manifest-path cognitive-forge/Cargo.toml
```

## File layout

```
service-slm/
├── README.md                  English README
├── README.es.md               Spanish README
├── CLAUDE.md                  this file
├── NEXT.md                    open items
├── ARCHITECTURE.md            three-ring model, stack, target file tree
├── DEVELOPMENT.md             build/CI policy, migration phases, blockers
├── Cargo.toml                 workspace manifest (B1, 2026-04-25)
├── deny.toml                  licence policy per DEVELOPMENT.md §2.1
├── rust-toolchain.toml        stable channel pin
├── .gitignore                 target/, swap files
├── crates/
│   ├── slm-core/              shared types, moduleId discipline
│   ├── slm-doorman/           three-tier router + audit ledger (lib)
│   └── slm-doorman-server/    axum HTTP entry point (bin)
├── cognitive-bridge.sh        placeholder — defect, queued for scripts/
├── cognitive-forge/           legacy subcrate — workspace `exclude`
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/main.rs            tokio + reqwest + serde_json
└── transient-queues/          runtime payload bleed — defect, triage pending
    └── TX-*_skeleton.txt      (8 files)
```

Still missing relative to `ARCHITECTURE.md` §File tree:
`memory/{kv,adapters}/`, `compute/`, `outbound/`, `inbound/`,
`log/`, `ledger/`, plus the remaining crates (`slm-ledger`,
`slm-compute`, `slm-memory-kv`, `slm-memory-adapters`,
`slm-inference-local`, `slm-inference-remote`, `slm-api`,
`slm-cli`). Phase-1 additions land as the Phase B task list
progresses (B2 fills `slm-doorman/src/tier/yoyo.rs`; B4 fills
`tier/external.rs`).

## Hard constraints — do not violate

- **Do not generate text in this service.** service-slm is an API
  gateway. Generation happens externally (Claude API in Phase 1;
  mistral.rs on the yo-yo node in Phase 2). If code here starts
  producing text directly, it has exceeded its remit.
- **Do not route structured data through the external LLM.**
  SYS-ADR-07 hard rule (workspace `CLAUDE.md` §6). Prose payloads
  cross the boundary outbound; structured facts stay in-Totebox.
- **Do not retire "Cognitive Forge" naming silently.** The rename
  is user-tracked in the monorepo `NEXT.md` rename series. Wait
  for the paired decision with `tool-cognitive-forge`.
- **Do not introduce dependencies outside the permissive-licence
  allow-list** once the Rust workspace is scaffolded.
  `DEVELOPMENT.md` §License policy names the `deny.toml`
  enforcement mechanism.

## Dependencies on other projects

- **service-content** (read side): service-slm writes extraction
  output to `service-content/knowledge-graph/`, which
  `service-content/content-compiler` consumes. Wire-format
  reconciliation pending.
- **service-email, service-people, service-content** as the three
  sovereign ledgers the Doorman compiles outbound context from
  (per README §I).
- **system-slm**: the local SLM engine. The "MISSING CONNECTION
  PHYSICS" comment in `cognitive-bridge.sh` is the documented gap
  blocking the placeholder bridge.

---

## Inherited rules — do not duplicate, do not silently override

- **Repo-level:** `pointsav-monorepo/CLAUDE.md` does not yet exist
  (tracked in workspace `NEXT.md` as a documentation-debt item).
  In its absence, the monorepo's `.claude/rules/` carries local
  conventions: `repo-layout.md`, `project-registry.md`,
  `cleanup-log.md`, `handoffs-outbound.md`.
- **Workspace-level:** `~/Foundry/CLAUDE.md` — identity store,
  commit flow (`tool-commit-as-next.sh`), cluster session pattern
  (§9), ADR hard rules (§6).

If a rule at this level conflicts with an inherited rule, **stop and
surface the conflict** — do not silently override.
