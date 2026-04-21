# Architecture overview

This document is the entry point to the service-slm narrative architecture.
It summarises the design and points to the detailed documents and
specifications that follow.

The authoritative source of truth for every claim on this page is in
[`../../specs/`](../../specs/). If this document and the specs disagree, the
specs win and this document is a bug.

## The one-paragraph version

service-slm is the single boundary in the PointSav platform where local
compute meets external compute. It implements the **doorman protocol**
(sanitise outbound, send, await, receive, rehydrate) and drives the **yo-yo
compute substrate** (spin up a GCP GPU node, run inference, tear down, keep
state across teardowns). It ships as **one signed Rust binary** that runs as
a systemd unit or an os-totebox appliance service component.

## The shape of the service

Three concentric rings of state, as specified in
[YOYO-COMPUTE §1](../../specs/YOYO-COMPUTE.md):

1. **Ring 1 — Bootstrap.** Container image, weights, and secret
   references, staged in cheap cold storage so the GCP node starts in
   under 30 seconds at zero idle cost.
2. **Ring 2 — Working memory.** The LMCache + Mooncake Store KV cache
   pool, which survives teardown and is `moduleId`-namespaced so that
   Project A's cached blocks never serve Project B's requests.
3. **Ring 3 — Long-term memory.** Split into 3a (the LadybugDB graph
   owned by `service-content`) and 3b (the LoRA adapter stack, which is
   this service's responsibility). The adapter stack is the
   cross-project skill library and the commercial moat.

Bridging all three rings is the **ledger** — an append-only CSV with a
SQLite mirror that writes a row for every inference event, every bootstrap,
every teardown, every adapter load. This is the SOC3 processing-integrity
artefact the hyperscalers structurally cannot produce.

## Why Rust and why one binary

Two decisions, both recorded as ADRs:

- [ADR-0001](../adr/0001-rust-end-to-end.md) — Rust end-to-end. Appliance
  fit, low-RAM hosts, predictable memory, one signed artefact.
- [ADR-0004](../adr/0004-flat-binary-no-mesh.md) — One binary, no
  microservice mesh. Inside the trust boundary, inter-module
  communication is a function call. Only external boundaries cross the
  network.

The inference engine choice, [ADR-0002](../adr/0002-mistralrs-over-vllm-phase-2.md),
swaps vLLM for `mistral.rs` in Phase 2 to preserve the single-binary shape.

## Reading order

If you are new to this codebase, read in this order:

1. This file.
2. [01 — The doorman protocol](./01-doorman-protocol.md).
3. [02 — The yo-yo substrate](./02-yoyo-substrate.md).
4. [03 — The Rust stack](./03-rust-stack.md).
5. [04 — The three-ring memory model](./04-three-ring-memory.md).
6. [05 — The ledger schema](./05-ledger-schema.md).
7. [06 — os-totebox integration](./06-os-totebox-integration.md).

Then the specs in [`../../specs/`](../../specs/).

## Where things are implemented

The ten crates in `crates/` map to the architecture as follows:

| Architectural concern | Crate |
|---|---|
| Shared types, `ModuleId` | `slm-core` |
| Doorman protocol | `slm-doorman` |
| Audit ledger | `slm-ledger` |
| Ring 1 (bootstrap, Cloud Run) | `slm-compute` |
| Ring 2 (KV cache) | `slm-memory-kv` |
| Ring 3b (adapters) | `slm-memory-adapters` |
| Local inference | `slm-inference-local` |
| Remote inference | `slm-inference-remote` |
| HTTP API | `slm-api` |
| Binary entry point | `slm-cli` |

Every crate has a `CLAUDE.md` describing its responsibilities, invariants,
and next work units.
