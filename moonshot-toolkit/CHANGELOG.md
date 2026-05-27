# Changelog — moonshot-toolkit

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
