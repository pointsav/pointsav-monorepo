---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: customer/woodfine-fleet-deployment
target_path: project-system/
target_filename: guide-moonshot-toolkit-phase1c-build-setup.md
audience: vendor-internal
bcsc_class: no-disclosure-implication
language_protocol: PROSE-GUIDE
authored: 2026-05-27T02:00:00Z
authored_by: task-project-system (session phase-1c-a)
authored_with: claude-sonnet-4-6
references:
  - clones/project-system/moonshot-toolkit/CHANGELOG.md v0.2.0
  - clones/project-system/moonshot-toolkit/NEXT.md (Blocked section)
  - clones/project-system/moonshot-toolkit/examples/hello-world.toml
  - clones/project-system/moonshot-toolkit/examples/hello.c
  - clones/project-system/moonshot-toolkit/src/main.rs (execute_step)
notes_for_editor: |
  Operational runbook for the Phase 1C.a build environment. Written from the
  actual workspace VM configuration after installation (2026-05-27).

  All commands are exact and have been run on Ubuntu 24.04 with Rust stable
  and aarch64-linux-gnu-gcc v13.3.0. Do not paraphrase commands; preserve them.

  The guide documents BOTH what works (Phase 1C.a: CompilePd produces hello.elf)
  AND what is not yet implemented (Phase 1C.d: AssembleImage blocked). Be honest
  about the gap without dismissive language.

  English-only per CLAUDE.md §14 — GUIDEs are not bilingual. No .es.md pair.

  BCSC posture: Phase 1C.c and 1C.d completion are planned/intended. Current
  state (Phase 1C.a) is current fact.
---

# moonshot-toolkit Phase 1C Build Setup

This guide covers installing the AArch64 cross-compile environment on the Foundry
workspace VM and using the `moonshot-toolkit build` subcommand to cross-compile a
protection domain source file to a bare-metal AArch64 ELF. It reflects the Phase 1C.a
milestone (moonshot-toolkit v0.2.0) and documents the current limitation at
Phase 1C.d (image assembly not yet implemented).

## Prerequisites

### Rust Toolchain

Rust stable is required. Verify with:

```
rustup show
```

The workspace VM has Rust installed at `/home/mathew/.cargo/`. The `moonshot-toolkit`
crate requires `rust-version = "1.74"` or later per `Cargo.toml`.

### AArch64 Cross-Compile Toolchain

Install via apt on Ubuntu 22.04 or 24.04:

```
sudo apt-get install -y \
  gcc-aarch64-linux-gnu \
  binutils-aarch64-linux-gnu \
  qemu-system-aarch64 \
  device-tree-compiler \
  libxml2-utils
```

Verify the cross-compiler:

```
aarch64-linux-gnu-gcc --version
# Expected: aarch64-linux-gnu-gcc (Ubuntu ...) 13.3.0
```

Verify QEMU:

```
qemu-system-aarch64 --version
# Expected: QEMU emulator version 8.2.x
```

### Python Dependencies (for seL4 Kernel Build Only)

The seL4 kernel CMake build uses Python scripts. These are not required for
`moonshot-toolkit build` itself, but are needed if building the seL4 kernel from
source (Phase 1C.b):

```
pip install --break-system-packages pyfdt tempita
```

## Using moonshot-toolkit

All commands are run from the `moonshot-toolkit/` directory within the
`clones/project-system/` archive:

```
cd ~/Foundry/clones/project-system/moonshot-toolkit
```

### Validate a System Specification

The `validate` subcommand parses a `system-spec.toml` and checks all invariants.
It exits 0 on success and prints a one-line summary:

```
cargo run -p moonshot-toolkit -- validate examples/hello-world.toml
```

Expected output:

```
✓ examples/hello-world.toml — 1 protection_domain(s), 0 channel(s), 0 memory_region(s), 0 irq_delivery
```

### Generate a BuildPlan

The `plan` subcommand parses the spec and generates a deterministic `BuildPlan` in
JSON. The `plan_hash` field is the SHA-256 of the canonical JSON of `(spec_hash, steps)`:

```
cargo run -p moonshot-toolkit -- plan examples/hello-world.toml --format pretty-json
```

The same spec always produces the same `plan_hash`. This hash is the value that a
customer-apex cosignature attaches to per `system-substrate-doctrine.md §6.1`.

### Build a Protection Domain (Phase 1C.a)

The `build` subcommand parses the spec, generates the BuildPlan, creates a `build/`
output directory, and executes each step:

```
cargo run -p moonshot-toolkit -- build examples/hello-world.toml
```

Expected output (Phase 1C.a):

```
Building plan (plan_hash = 3280a9dc2943ac63…)
[1/2] compile-pd-hello
  ✓ build/hello.elf
[2/2] assemble-image
error: assemble-image → build/system-image.bin: not yet implemented; requires Microkit SDK or Rust image assembler (Phase 1C.d). PD binaries ready: [build/hello.elf]
```

The exit code is non-zero because the AssembleImage step fails. The CompilePd step
succeeds: `build/hello.elf` has been produced.

### Verify the Compiled Output

Confirm the ELF is a valid bare-metal AArch64 binary:

```
file build/hello.elf
```

Expected output:

```
build/hello.elf: ELF 64-bit LSB executable, ARM aarch64, version 1 (SYSV), statically linked, not stripped
```

For a more detailed header inspection:

```
aarch64-linux-gnu-readelf -h build/hello.elf | grep -E "Type|Machine|Entry"
```

Expected:

```
  Type:                              EXEC (Executable file)
  Machine:                           AArch64
  Entry point address:               0x40010c
```

## Current Limitation

As of moonshot-toolkit v0.2.0, the `AssembleImage` build step is not yet implemented.
This step packs the compiled protection-domain ELFs and the seL4 kernel into a single
bootable image in Microkit's image format. It requires either the seL4 Microkit SDK
(a Python package that provides the `microkit` image-assembly tool) or a Rust-native
image assembler planned for v0.3.0.

The CompilePd step (Phase 1C.a) is complete and produces a verified AArch64
bare-metal static ELF. The full Phase 1C hello-world milestone — a seL4 protection
domain booting in QEMU and producing output — is intended to complete when the
AssembleImage step (Phase 1C.d) and the QEMU boot infrastructure (Phase 1C.c,
requiring the seL4 elfloader from `github.com/seL4/seL4_tools`) are in place.

The AssembleImage error message includes the paths to any already-compiled PD binaries,
so they do not need to be recompiled when Phase 1C.d resolves:

```
PD binaries ready: [build/hello.elf]
```

## Running the Test Suite

```
cargo test -p moonshot-toolkit --all-targets
```

Expected: 30 tests pass (22 lib tests + 8 bin tests). The bin test
`build_command_errors_without_source_file` verifies that the build subcommand
correctly propagates compile errors when the source file referenced in the spec
does not exist.

## See Also

- `topic-moonshot-toolkit-build-orchestrator.md` — architectural background on
  what moonshot-toolkit is and how the BuildPlan model works
- `topic-sel4-aarch64-qemu-substrate-target.md` — the seL4 AArch64 QEMU target
  that the built images run on
