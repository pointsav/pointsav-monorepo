# NEXT.md — moonshot-toolkit

> Last updated: 2026-04-27
> Read at session start. Update before session end.

---

## Right now

- Framework §9 activation landed (this commit). Next commits in
  this AUTO session: src/spec.rs (#35) + src/plan.rs (#36) +
  src/main.rs CLI rewrite (#37).

## Queue

- Implement `src/spec.rs` per task #35: SystemSpec data model
  (PDs ≤ 63, channels, memory regions, IRQ delivery), serde TOML
  parser, validation rules. Tests for valid round-trip + each
  invariant violation.
- Implement `src/plan.rs` per task #36: BuildPlan generation
  with content-addressed inputs and deterministic plan_hash.
  Tests for determinism + spec → plan output binding.
- Rewrite `src/main.rs` per task #37: clap-based CLI with
  `validate` / `plan` / `build` subcommands. `build` is a stub
  in v0.1.x (prints "would run X" for each step) — actual seL4
  cross-compile + QEMU boot is the future-session milestone (#14).

## Blocked

- Actual seL4 hello-world build (#14) — Blocked on:
  (a) cross-compile toolchain installation (aarch64-linux-gnu-gcc
  or equivalent in workspace); (b) seL4 source vendoring strategy
  decision (git submodule vs Cargo build.rs fetch vs
  vendor-sel4-kernel snapshot); (c) reproducible-build harness
  selection (Nix vs Bazel-hermetic). Surface to operator + Master
  before scheduling that session.

## Deferred

- `build-totebox.sh` legacy shell sketch — Deferred: kept in
  place as migration reference until Phase 1B Rust replacement is
  operational. Remove when `moonshot-toolkit build` produces a
  bootable image end-to-end. Tracked as a task #14 closure
  artefact.
- `src/main.rs` legacy stub (14-line "Forging Managed Substrate"
  print routine) — Deferred to #37 rewrite this session.
- Sigstore Cosign + customer-apex cosignature emission per
  convention §6.1 — Deferred until BuildPlan output is real
  (post-#14). The plan_hash field is in v0.1.x; cosignature
  on top of plan_hash is straightforward when binary outputs
  exist.

## Recently done

- 2026-04-27: framework §9 activation — CLAUDE.md / AGENTS.md /
  NEXT.md / ARCHITECTURE.md / DEVELOPMENT.md created; bilingual
  READMEs updated; workspace member entry added; registry row
  Scaffold-coded → Active.
