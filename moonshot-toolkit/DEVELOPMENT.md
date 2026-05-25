# DEVELOPMENT — moonshot-toolkit

Build / run / test instructions for the moonshot-toolkit Rust-only
build orchestrator.

---

## v0.1.x — pure-Rust development (current scope)

```
cargo check  -p moonshot-toolkit
cargo test   -p moonshot-toolkit
cargo build  -p moonshot-toolkit          # produces target/debug/moonshot-toolkit
```

After the CLI rewrite lands (cluster tasks #35-#37 in this AUTO
session):

```
# Validate a system-spec.toml without building
cargo run -p moonshot-toolkit -- validate path/to/system-spec.toml

# Generate + print a BuildPlan
cargo run -p moonshot-toolkit -- plan path/to/system-spec.toml

# Stubbed: prints "would run: <command>" for each step
cargo run -p moonshot-toolkit -- build path/to/system-spec.toml
```

Sample `system-spec.toml`s land in `tests/fixtures/` alongside the
parser tests.

## Future: real seL4 cross-compile (task #14, FUTURE session)

The actual seL4 hello-world cross-compile + QEMU AArch64 boot is
not part of v0.1.x. When that work scheduled:

### Prerequisites

- `aarch64-linux-gnu-gcc` toolchain (or Rust cross-target
  `aarch64-unknown-none` for fully-Rust builds)
- `qemu-system-aarch64` for boot testing
- seL4 source — vendoring strategy TBD per `NEXT.md` Blocked
  section. Three candidate sources:
  - Git submodule of seL4/seL4@15.0.0
  - Cargo `build.rs` fetch (network at build time — violates
    convention §6 hermetic property)
  - `vendor-sel4-kernel` snapshot (workspace-tier, audit-trail
    friendly)
- Reproducible-build harness — Nix-style content-addressed inputs
  vs Bazel-hermetic; decision pending operator + Master direction.

### Operator setup

These steps land in a future session that explicitly schedules
the cross-compile work; they need root or sudo for system
package installation.

```
# (future) install cross-compile toolchain
sudo apt-get install gcc-aarch64-linux-gnu qemu-system-aarch64

# (future) vendor seL4 source
# decision pending — see NEXT.md
```

## Repository conventions

- Per `~/Foundry/CLAUDE.md` §8: commits via
  `~/Foundry/bin/commit-as-next.sh` on `cluster/project-system`
  branch; commit messages end with `Version: M.m.P` trailer.
- Per `~/Foundry/conventions/root-files-discipline.md` Tier 2:
  CLAUDE.md / AGENTS.md / NEXT.md required at active state;
  bilingual README pair required at all states.

## Test discipline

- Parser tests use TOML fixtures under `tests/fixtures/`.
- Plan-generation tests assert determinism (same input → same
  plan_hash) and spec-output binding.
- CLI integration tests via `assert_cmd` or
  `Command::cargo_bin`.

## Cross-references

- `CLAUDE.md` for operational rules
- `ARCHITECTURE.md` for design intent
- `NEXT.md` for current and queued work
- `~/Foundry/conventions/system-substrate-doctrine.md` §6 for
  Reproducible-Verification-On-Customer-Metal context
