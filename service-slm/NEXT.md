# NEXT.md — service-slm

> Last updated: 2026-04-23
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now

- **Framework §8 activation commit.** Land `CLAUDE.md`, this file,
  `ARCHITECTURE.md`, `DEVELOPMENT.md`, and the monorepo
  `project-registry.md` row flip (Scaffold-coded → Active) in one
  commit via `tool-commit-as-next.sh`. Hold on push until operator
  approval per workspace `CLAUDE.md` §7.
- **Workspace-root handoff in flight.** This activation commit is
  the Task-scope half of the `SLM-STACK.md` / `YOYO-COMPUTE.md`
  rehoming (see workspace `NEXT.md`). Root-scope half is drafting
  `content-wiki-documentation/topic-service-slm.md` and
  `topic-yoyo-compute.md` in a separate session opened in that
  repo. Master deletes the two workspace-root originals only
  after both halves land.

## Queue

- Move `cognitive-bridge.sh` → `scripts/` — layout-hygiene defect
  queued in monorepo `NEXT.md`. Single `git mv`; script body uses
  positional args only, no caller audit needed.
- Triage `transient-queues/` — mirrors the `discovery-queue`
  "Not-a-project" pattern in the registry. Decide: gitignore and
  relocate live state to `service-fs/data/`, or confirm as
  deliberate fixture. Do not alter until decided.
- Reconcile `cognitive-forge` → `content-compiler` wire format —
  writer emits `.md` files (markdown bullets); reader only
  consumes `.json`. They do not interoperate today. Pick one
  format and land the contract.
- Close "MISSING CONNECTION PHYSICS" — define the concrete wire
  from `cognitive-bridge.sh` to the local SLM. Candidate:
  `POST http://127.0.0.1:8080/v1/chat/completions` to match the
  `cognitive-forge` endpoint (mistral.rs OpenAI-compatible per
  `ARCHITECTURE.md` §Stack). Replace the placeholder
  `RESPONSE="[UNVERIFIED STAGING OVERLAY]..."` with the real call.
- Scaffold project-root `Cargo.toml` — cargo workspace per
  `ARCHITECTURE.md` §File tree. First crate to stub:
  `crates/slm-core` (shared types, `moduleId` discipline). Do not
  port the existing `cognitive-forge/` into `crates/` until after
  the rename decision below.
- Rename the `cognitive-forge/` subcrate — inherits the Do-Not-Use
  "Forge" concern. Pair with the sibling `tool-cognitive-forge`
  rename queued in the monorepo `NEXT.md` rename series so one
  decision covers both.
- Build out `compute/` directory per Ring 1 spec in
  `ARCHITECTURE.md` — `manifest.yaml`, `container/Dockerfile`,
  `weights/registry.yaml`, `sky/*.yaml`, `keys/secret-refs.yaml`.
  Phase 1 scope; no Ring 2 / 3b content yet.
- Build out `ledger/` — append-only `events.csv` plus `schema.md`,
  SOC3 event vocabulary per `ARCHITECTURE.md` §Audit ledger.
  Phase 1 scope.
- Begin Phase 2 doorman-protocol rewrite in Rust — per
  `DEVELOPMENT.md` §Migration. First: `crates/slm-doorman` with
  sanitise / send / receive / rehydrate primitives. Keep the
  existing shell bridge running alongside until parity is proven.
- Land `deny.toml` blocking AGPL / GPL / BSL at workspace level
  per `DEVELOPMENT.md` §License policy — first commit after the
  workspace `Cargo.toml` exists.

## Blocked

- **system-slm connection protocol.** Blocks closing "MISSING
  CONNECTION PHYSICS" in `cognitive-bridge.sh` and the real
  Phase-1 wire. Blocked on: operator decision on SLM endpoint
  shape (OpenAI-compatible HTTP or local CLI binary — see the
  two commented examples in `cognitive-bridge.sh`).
- **Mooncake / LMCache licence audit for Ring 2.** Blocked on:
  operator confirmation at adoption time. `DEVELOPMENT.md`
  §Blockers before Phase 2.
- **Mooncake master hosting.** Blocked on: choice between small
  always-on GCE VM, Totebox co-host, or SkyPilot pool.
- **Secret Manager migration.** Blocks Phase 2 key management —
  currently SSH env vars per Phase 1.
- **Adapter training hardware + evaluation protocol.** Blocks
  Ring 3b build-out.

## Deferred

- CUDA checkpoint/restore integration — deferred until vLLM
  RFC #34303 ships upstream. `ARCHITECTURE.md` has the hook; no
  work needed now.
- C-LoRA single-adapter migration — deferred until the project
  count exceeds ten and dual-adapter management becomes a burden.
- Multi-cloud KV pool — deferred until single-cloud Ring 2
  proves in production.
- FP8 KV-cache quantisation — deferred as Phase-2 polish.
