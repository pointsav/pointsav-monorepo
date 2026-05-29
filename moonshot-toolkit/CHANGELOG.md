# Changelog — moonshot-toolkit

## 0.3.1 — 2026-05-29
- `CompilePd`: add `-O2` flag. seL4 rootserver starts with SP uninitialised;
  without optimisation the compiler emits a stack-frame prologue at `_start`
  that immediately faults (`stp [sp, #-32]`). Required for all bare-metal PDs.
- `examples/hello.c`: wired `SysDebugPutChar` (-9 via AArch64 `svc #0`). Now
  prints "hello from seL4 rootserver" before entering the yield loop.
- `build-totebox.sh` removed (migration reference; Phase 1C.d complete).
- 35 tests total. Zero warnings.

## 0.3.0 — 2026-05-29
- `AssembleImage` fully implemented: pure Rust orchestration produces a
  bootable `build/system-image.bin` (elfloader.elf). No Python, CMake, or
  shell in the critical path (MEMO §7 Rust-Only Toolchain mandate).
- New `src/cpio.rs`: Rust CPIO "newc" writer (`write_archive`). Translates
  `gen_cpio.py` padding formula: align4(header+name) and align4(data); appends
  TRAILER!!! automatically. 4 unit tests.
- `AssembleImage` orchestration: validates well-known paths; generates CPIO
  archive; writes `archive.S` with `.incbin` (absolute path); copies libcpio;
  compiles 44 elfloader C/ASM sources + libcpio.c with full CFLAGS/includes;
  preprocesses linker script; links with `-lgcc -nostdlib -static`; copies
  elfloader.elf to `output_image`.
- `examples/hello-world.toml`: updated paths to project-root-relative; QEMU
  boot command added to comment.
- `lib.rs`: `pub mod cpio` added.
- 35 tests total (26 lib + 9 bin). Zero warnings.
- Verified: `build/system-image.bin` entry 0x40400000; QEMU boots to
  "Bootstrapping kernel" → "Booting all finished, dropped to user space".

## 0.2.0 — 2026-05-27
- `build` subcommand: replace stub with real `std::process::Command` execution.
  `CompilePd` invokes `aarch64-linux-gnu-gcc` (-nostdlib -nostartfiles -ffreestanding
  -static -no-pie -march=armv8-a -mgeneral-regs-only); produces bare-metal AArch64
  static ELF. `AssembleImage` returns actionable error: Phase 1C.d — requires
  Microkit SDK or Rust image assembler.
- Add `examples/hello-world.toml` + `examples/hello.c`: minimal Phase 1C PD spec
  (one PD "hello", bare-metal _start, cross-compiles to build/hello.elf).
- Update `build_command_succeeds_as_stub` → `build_command_errors_without_source_file`
  to reflect real compiler invocation.
- Phase 1C.a complete: `moonshot-toolkit build examples/hello-world.toml` produces
  a valid AArch64 bare-metal static ELF.

## 0.1.3 — 2026-04-27
- Phase 1B complete: `src/spec.rs` (445 lines, 12 tests), `src/plan.rs`
  (310 lines, 10 tests), `src/main.rs` CLI rewrite (241 lines, 8 tests).
  `validate` / `plan` / `build` (stub) subcommands working. 30 tests total.

## 0.1.2 — 2026-04-27
- Framework activation: CLAUDE.md, AGENTS.md, NEXT.md, ARCHITECTURE.md,
  DEVELOPMENT.md created; bilingual READMEs; workspace member; registry Active.

## 0.1.1 — 2026-04-26
- Initial scaffold: lib.rs re-export stubs; Cargo.toml workspace member entry.
