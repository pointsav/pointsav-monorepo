# CLAUDE.md — system-core

> **State:** Active  —  **Last updated:** 2026-04-27
> **Version:** 0.1.4  (per `~/Foundry/CLAUDE.md` §7 and DOCTRINE.md §VIII)
> **Registry row:** `pointsav-monorepo/.claude/rules/project-registry.md`

---

## What this project is

The substrate-primitive crate for The Capability Ledger Substrate
(Doctrine claim #33). Defines the [`Capability`] and
[`WitnessRecord`] types every other `system-*` and `moonshot-*` crate
binds against.

Sibling to `system-substrate` (kernel binding), `system-security`
(cryptographic-pairing), `system-verification` (proof-artefact
container), `system-audit` (audit sub-ledger).

## Current state

Phase 1A increment 1 of the project-system cluster brief
(`~/Foundry/clones/project-system/.claude/inbox.md`). The
`Capability`, `WitnessRecord`, `LedgerAnchor`, `CapabilityType`, and
`Right` types are defined per `system-substrate-doctrine.md` §3.1 +
§5.1. Six unit tests cover serialisation round-trips and hash
determinism. No runtime ledger code yet — the kernel-side consultation
simulator (Phase 1A item 3 in the brief) lives in `system-substrate`
or a new `system-capability-ledger` crate; the architecture decision
is open and tracked in `ARCHITECTURE.md`.

`master-relay.rs` at the project root is residual sketch material
(predates this cluster); it is NOT a binary target and is preserved
in place pending defect closure.

## Build and test

```
cargo check -p system-core
cargo test  -p system-core
```

Six tests pass on Rust stable. No external services required; pure-
data primitives.

## File layout

```
system-core/
├── Cargo.toml         # workspace member as of v0.1.20
├── README.md          # bilingual pair (English)
├── README.es.md       # bilingual pair (Spanish overview)
├── CLAUDE.md          # this file
├── AGENTS.md          # vendor-neutral pointer
├── NEXT.md            # open items
├── ARCHITECTURE.md    # Phase 1A architecture intent
├── src/lib.rs         # Capability + WitnessRecord + LedgerAnchor
└── master-relay.rs    # residual sketch — pending defect closure
```

## Hard constraints — do not violate

- The [`Capability`] field set is doctrine-bound. Adding or removing
  fields requires a doctrine MINOR per `system-substrate-doctrine.md`
  §10 ("Doctrine-version pinning"), not an in-crate decision.
- Hash function is SHA-256 (worm-ledger-design.md §3 D3 baseline).
  Algorithm-agility is structural — a future MINOR may add BLAKE3 /
  SHA-3 alongside SHA-256, never instead of.
- Witness record signature namespace is `capability-witness-v1`. Do
  not reuse the commit-signing or verdict-signing namespace tags;
  cross-namespace replay is the attack this convention prevents.
- The crate stays `no_std`-eligible long-term (the kernel may consume
  it). v0.1.x carries `std` for `Vec` + JSON serialisation; future
  MINOR carves the `no_std` path. Do not add std-only dependencies
  (filesystem, network, threads) without surfacing the architectural
  question.

## Dependencies on other projects

- Consumed by: `system-substrate` (kernel binding), `system-security`,
  `system-audit`, `system-verification`, eventually
  `system-capability-ledger` (or extension of `system-substrate`).
- Consumes: nothing in the workspace today; pure data definitions.

## Commit convention

Per `~/Foundry/CLAUDE.md` §8 — staging-tier helper
`bin/commit-as-next.sh` on `cluster/project-system` branch. Commit
messages end with `Version: M.m.P` trailer.

---

## Inherited rules — do not duplicate, do not silently override

- **Repo-level:** `pointsav-monorepo/CLAUDE.md` (when present) —
  prefix taxonomy, canonical names, ADR hard rules (SYS-ADR-07, -10,
  -19), bilingual README rule, BCSC disclosure posture.
- **Repo-level rules dir:**
  `pointsav-monorepo/.claude/rules/{repo-layout,project-registry,
  cleanup-log,handoffs-outbound}.md`.
- **Workspace-level:** `~/Foundry/CLAUDE.md` §3 (commit signing) +
  §6 (rules of engagement) + §11 (Master/Root/Task action matrix) +
  §13 (root-files-discipline) + §14 (TOPIC vs GUIDE).
- **Constitutional charter:** `~/Foundry/DOCTRINE.md` claims #33 +
  #34.
- **Operational spec:**
  `~/Foundry/conventions/system-substrate-doctrine.md`.

If a rule at this level conflicts with an inherited rule, **stop and
surface the conflict** — do not silently override.
