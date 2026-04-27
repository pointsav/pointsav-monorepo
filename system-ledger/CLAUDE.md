# CLAUDE.md — system-ledger

> **State:** Active  —  **Last updated:** 2026-04-27
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

**Skeleton commit landed.** All four modules (`cache`, `revocation`,
`apex`, `witness`) plus `lib.rs` with `LedgerConsumer` trait + types
exist as compilable stubs. Public API surface defined per Master's
proposed layout. NO functional behaviour yet — module impls land in
subsequent commits per cluster task list (#18 cache, #19 revocation,
#11 apex, #12 witness, #20 LedgerConsumer impl, #21 benchmarks).

## Build and test

```
cargo check -p system-ledger
cargo test  -p system-ledger
```

Skeleton has zero tests today; tests land alongside each module
implementation.

## File layout

```
system-ledger/
├── Cargo.toml             # workspace member as of v0.1.21
├── README.md              # bilingual pair (English)
├── README.es.md           # bilingual pair (Spanish overview)
├── CLAUDE.md              # this file
├── AGENTS.md              # vendor-neutral pointer
├── NEXT.md                # open items
├── ARCHITECTURE.md        # Phase 1A increment 3 architecture
└── src/
    ├── lib.rs             # LedgerConsumer trait + Verdict / RefuseReason / errors / InMemoryLedger
    ├── cache.rs           # CheckpointCache (skeleton; fill per #18)
    ├── revocation.rs      # RevocationSet + RevocationEvent (skeleton; fill per #19)
    ├── apex.rs            # ApexHistory + ApexEntry (skeleton; fill per #11)
    └── witness.rs         # ssh-keygen -Y verify wrapper (skeleton; fill per #12)
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
- The crate stays buildable on every commit (`cargo check -p
  system-ledger` passes). Skeleton stubs return `NotImplemented`
  errors where impls are pending; never broken builds.

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
