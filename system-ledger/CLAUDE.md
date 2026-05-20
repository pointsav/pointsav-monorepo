# CLAUDE.md — system-ledger

> **State:** Active  —  **Last updated:** 2026-05-20
> **Version:** 0.2.1  (per `~/Foundry/CLAUDE.md` §7 and DOCTRINE.md §VIII)
> **Registry row:** `pointsav-monorepo/.claude/rules/project-registry.md`

---

## What this project is

The substrate-tier consumer of the Capability Ledger Substrate
primitives in `system-core`. Owns the kernel-side state machine
that decides whether to honor a capability invocation: checkpoint
cache, revoked-capability set, apex history with post-handover
invariant enforcement, witness-record signature verification.

Created 2026-04-27 per Master Claude directive (Option B
resolution archived in cluster mailbox). Mirrors the `service-fs`
pattern from `worm-ledger-design.md`: same C2SP signed-note format,
substrate-tier vs application-tier consumer, decoupled by layer.

## Current state

Phase 1A structurally complete at v0.2.1. All five modules fully
implemented: `cache.rs` (CheckpointCache LRU 64-entry), `revocation.rs`
(RevocationSet O(1) HashSet + audit sidecar), `apex.rs` (ApexHistory
+ N+3+ post-handover invariant), `witness.rs` (ssh-keygen -Y verify
wrapper, namespace `capability-witness-v1`), `lib.rs` (LedgerConsumer
trait + InMemoryLedger). 44 tests + 10 criterion benchmarks.

`apply_witness_record` takes an `InclusionProof` parameter (since
v0.2.0): witness arrivals are Merkle-inclusion-proof gated, no trust
shortcut.

## Build and test

```
cargo check -p system-ledger
cargo test  -p system-ledger
cargo bench -p system-ledger   # criterion; release profile
```

44 tests pass on Rust stable. `witness.rs` tests require
`/usr/bin/ssh-keygen` in PATH (present on the workspace VM).

## File layout

```
system-ledger/
├── Cargo.toml
├── README.md / README.es.md   # bilingual pair
├── CLAUDE.md / AGENTS.md / NEXT.md / ARCHITECTURE.md
├── BENCHMARKS.md              # published benchmark numbers
├── benches/
│   └── consult.rs             # 10 criterion benchmarks
└── src/
    ├── lib.rs       # LedgerConsumer trait + InMemoryLedger
    ├── cache.rs     # CheckpointCache — LRU 64-entry
    ├── revocation.rs# RevocationSet
    ├── apex.rs      # ApexHistory + N+3+ invariant
    └── witness.rs   # ssh-keygen -Y verify wrapper
```

## Hard constraints — do not violate

- Single-writer assumption. v0.1.x is NOT thread-safe — matches
  the kernel-side single-threaded substrate model. Adding
  multi-threaded support requires explicit doctrine MINOR.
- Witness signature namespace MUST be `capability-witness-v1` (per
  `witness::WITNESS_NAMESPACE`). Cross-namespace replay against
  commit-signing or apprenticeship-verdict signatures is the attack
  this discipline prevents.
- Post-handover invariant per convention §4: signatures from a
  retired apex MUST be refused on checkpoints at or above the
  retirement height. The kernel verifier enforces this; do not
  add escape hatches.
- `tempfile` is a `[dev-dependencies]` entry used by `witness.rs`
  tests. Do not promote it to a regular dependency.
- The crate stays buildable on every commit (`cargo check -p
  system-ledger` and `cargo test -p system-ledger` pass). Never
  push a broken build to the cluster branch.

## Dependencies on other projects

- Consumes: `system-core` (Capability, WitnessRecord, SignedCheckpoint,
  LedgerAnchor, NoteSignature, Hash256).
- Consumed by: future `system-substrate` ledger-consultation hooks
  (Phase 4+ when seL4 CDT integration lands); the broader runtime
  consumes via the `LedgerConsumer` trait.

## Commit convention

Per `~/Foundry/CLAUDE.md` §8 — staging-tier helper
`bin/commit-as-next.sh` on `cluster/project-system` branch. Commit
messages end with `Version: M.m.P` trailer.

---

## Inherited rules — do not duplicate, do not silently override

- **Repo-level:** `pointsav-monorepo/.claude/rules/{repo-layout,
  project-registry,cleanup-log,handoffs-outbound}.md`.
- **Workspace-level:** `~/Foundry/CLAUDE.md` §3 (commit signing) +
  §11 (Master/Root/Task action matrix) + §13 (root-files-discipline)
  + §14 (TOPIC vs GUIDE).
- **Constitutional charter:** `~/Foundry/DOCTRINE.md` claims #33 +
  #34.
- **Operational spec:**
  `~/Foundry/conventions/system-substrate-doctrine.md`.
- **Sibling crate:** `system-core/CLAUDE.md` for the data-primitive
  conventions this crate consumes.
