# CLAUDE.md — moonshot-toolkit

> **State:** Active  —  **Last updated:** 2026-05-29
> **Version:** 0.3.1  (per `~/Foundry/CLAUDE.md` §7 and DOCTRINE.md §VIII)
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

Phase 1C complete at v0.3.0. Full build pipeline from TOML spec to bootable
seL4 system image runs in a single `cargo run` invocation:

```
cargo run -p moonshot-toolkit -- build moonshot-toolkit/examples/hello-world.toml
```

Produces `build/system-image.bin` (elfloader ELF, entry 0x40400000). QEMU
boots to "Booting all finished, dropped to user space".

- `CompilePd`: aarch64-linux-gnu-gcc bare-metal AArch64 ELF.
- `AssembleImage`: pure Rust — CPIO archive (`src/cpio.rs`), elfloader
  compilation (44 C/ASM sources), link with -nostdlib -static -lgcc.
- No Python, CMake, or shell in the critical path (MEMO §7 ✓).

35 tests total (`cargo test --all-targets`; 26 lib + 9 bin). Zero warnings.

## Build and test

```
cargo check -p moonshot-toolkit
cargo test  -p moonshot-toolkit --all-targets   # 35 tests
cargo run   -p moonshot-toolkit -- validate moonshot-toolkit/examples/hello-world.toml
cargo run   -p moonshot-toolkit -- plan     moonshot-toolkit/examples/hello-world.toml
cargo run   -p moonshot-toolkit -- build    moonshot-toolkit/examples/hello-world.toml
```

Note: `build` must be run from the project root (not moonshot-toolkit/ dir) because
AssembleImage resolves vendor/ paths relative to CWD.

## File layout

```
moonshot-toolkit/
├── Cargo.toml             # workspace member; v0.3.0
├── README.md / README.es.md   # bilingual pair
├── CLAUDE.md / AGENTS.md / NEXT.md / ARCHITECTURE.md / DEVELOPMENT.md
├── build-totebox.sh       # legacy shell sketch — remove (Phase 1C.d complete)
└── src/
    ├── lib.rs             # re-exports SystemSpec + BuildPlan; pub mod cpio
    ├── spec.rs            # SystemSpec TOML parser (445 lines, 12 tests)
    ├── plan.rs            # BuildPlan generator (310 lines, 10 tests)
    ├── cpio.rs            # pure Rust CPIO "newc" writer (4 tests)
    └── main.rs            # clap CLI + assemble_image orchestration
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
