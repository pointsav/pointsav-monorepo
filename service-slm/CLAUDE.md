# CLAUDE.md — service-slm

> **State:** Active  —  **Last updated:** 2026-04-23
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
`cluster/service-slm` first-live cluster. The only code present is
the nested `cognitive-forge/` subcrate (tokio-based binary that POSTs
payloads to `http://127.0.0.1:8080/v1/chat/completions` and writes
output into `service-content/knowledge-graph/`). There is no
project-root `Cargo.toml` yet — the Rust workspace shape specified in
`ARCHITECTURE.md` is not scaffolded.

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

No build step at project root yet — no `Cargo.toml`. The nested
subcrate builds in isolation:

```
cargo build --manifest-path cognitive-forge/Cargo.toml
```

End-to-end execution requires a running SLM endpoint on
`http://127.0.0.1:8080/v1/chat/completions` and a Totebox root
directory containing `service-slm/transient-queues/` with payload
files. Not run end-to-end from this clone.

## File layout

```
service-slm/
├── README.md                 English README
├── README.es.md              Spanish README
├── CLAUDE.md                 this file
├── NEXT.md                   open items
├── ARCHITECTURE.md           three-ring model, stack, target file tree
├── DEVELOPMENT.md            build/CI policy, migration phases, blockers
├── cognitive-bridge.sh       placeholder — defect, queued for scripts/
├── cognitive-forge/          Rust subcrate — current extraction worker
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/main.rs           tokio + reqwest + serde_json
└── transient-queues/         runtime payload bleed — defect, triage pending
    └── TX-*_skeleton.txt     (8 files)
```

The target layout after Phase 2 — cargo workspace with `crates/`,
`memory/`, `compute/`, `ledger/` — is in `ARCHITECTURE.md` §File
tree. None of that exists yet.

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
