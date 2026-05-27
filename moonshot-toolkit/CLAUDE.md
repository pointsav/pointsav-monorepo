# CLAUDE.md — moonshot-toolkit

> **State:** Active  —  **Last updated:** 2026-05-27
> **Version:** 0.1.3  (per `~/Foundry/CLAUDE.md` §7 and DOCTRINE.md §VIII)
> **Registry row:** `pointsav-monorepo/.claude/rules/project-registry.md`

---

## What this project is

The Rust-only build orchestrator for Foundry's seL4 unikernel
images, replacing Microkit's Python/CMake toolchain (per MEMO §7
"Microkit (Python/CMake) → moonshot-toolkit (Rust-Only Toolchain)").
Reads a Rust-native equivalent of Microkit's system-description
XML from a `system-spec.toml`, generates a deterministic
content-addressed `BuildPlan`, and orchestrates the seL4 + system-* +
os-* compile to produce a bootable image.

Foundational to Phase 1B per the inbox brief: "Without
`moonshot-toolkit`, none of the other `moonshot-*` projects can be
exercised."

## Current state

Phase 1B complete at v0.1.3. All three Phase 1B deliverables shipped:
- `src/spec.rs` (445 lines) — SystemSpec TOML parser; PDs ≤ 63,
  channels, memory regions, IRQ delivery; 12 tests.
- `src/plan.rs` (310 lines) — BuildPlan generator; deterministic
  plan_hash (SHA-256 of canonical JSON); 10 tests.
- `src/main.rs` (241 lines) — clap CLI; `validate` / `plan` / `build`
  subcommands; `build` is a stub pending cross-compile toolchain; 8 tests.

30 tests total (`cargo test --all-targets`). Zero warnings.

Next milestone: seL4 hello-world cross-compile + QEMU AArch64 boot
(task #14). Blocked on Group 3A decisions — see NEXT.md.

## Build and test

```
cargo check -p moonshot-toolkit
cargo test  -p moonshot-toolkit --all-targets   # 30 tests
cargo run   -p moonshot-toolkit -- validate <path/to/system-spec.toml>
cargo run   -p moonshot-toolkit -- plan     <path/to/system-spec.toml>
cargo run   -p moonshot-toolkit -- build    <path/to/system-spec.toml>  # stub in v0.1.x
```

## File layout

```
moonshot-toolkit/
├── Cargo.toml             # workspace member; v0.1.3
├── README.md / README.es.md   # bilingual pair
├── CLAUDE.md / AGENTS.md / NEXT.md / ARCHITECTURE.md / DEVELOPMENT.md
├── build-totebox.sh       # legacy shell sketch — remove after task #14 ships
└── src/
    ├── lib.rs             # re-exports SystemSpec + BuildPlan for library consumers
    ├── spec.rs            # SystemSpec TOML parser (445 lines, 12 tests)
    ├── plan.rs            # BuildPlan generator (310 lines, 10 tests)
    └── main.rs            # clap CLI — validate/plan/build (241 lines, 8 tests)
```

## Hard constraints — do not violate

- **Rust-Only Toolchain mandate** per MEMO §7. Do not introduce
  Python, CMake, shell-as-load-bearing, or other build languages
  into the moonshot-toolkit critical path. The legacy
  `build-totebox.sh` exists ONLY for migration reference and will
  be removed when Phase 1B's Rust replacement is operational.
- **Deterministic plan generation.** Same SystemSpec → same
  BuildPlan bytes → same plan_hash. Reproducible-build harness
  cosignature (Sigstore + customer-apex per convention §6.1)
  depends on this property.
- **Content-addressed inputs.** Every build input (source files,
  config, vendored dependencies) is named by SHA-256 hash. The
  plan_hash is the hash of the canonical bytes of the entire
  BuildPlan including all input hashes.
- **No network on `build`.** Reproducible-build harness must be
  hermetic. Network access happens at `prepare` time (vendoring),
  not at build time.

## Dependencies on other projects

- Will eventually consume `system-core` (for content-addressed
  primitives) and `system-ledger` (for cosigned manifests).
- Consumes (future): `vendor-sel4-kernel` (vendored seL4 source
  for cross-compile).
- Consumed by: every future `os-*` build pipeline (the toolkit
  is the build mechanism, not just a tool — without it, no
  os-* binary ships).

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
  `~/Foundry/conventions/system-substrate-doctrine.md` §6
  (Reproducible-Verification-On-Customer-Metal — moonshot-toolkit
  produces the artefacts that customer re-executes).
- **MEMO context:** §7 "Microkit (Python/CMake) → moonshot-toolkit
  (Rust-Only Toolchain)".

If a rule at this level conflicts with an inherited rule, **stop and
surface the conflict** — do not silently override.
