# NEXT.md — moonshot-toolkit

> Last updated: 2026-05-29
> Read at session start. Update before session end.

---

## Right now

- v0.3.1 in progress: `CompilePd -O2` fix + `hello.c SysDebugPutChar` +
  `build-totebox.sh` removal. Rebuilding and QEMU-verifying this session.

## Queue

- **Sigstore Cosign + customer-apex cosignature** — `plan_hash` field in place;
  cosignature emission now unblocked (Phase 1C.d complete). [future MINOR]
- **Configurable kernel/elfloader paths** — `vendor-sel4-kernel/build/aarch64-qemu`
  and `vendor-sel4-tools/elfloader-tool` are well-known hardcoded paths. Consider
  a `[build]` section in system-spec.toml for per-spec overrides.

## Deferred

- `no_std` eligibility — not a constraint for the build orchestrator
  (runs on the workspace VM, not in the kernel). No action needed.

## Blocked

- None at this stage.

## Recently done

- 2026-05-29 (v0.3.1): `CompilePd` gains `-O2` — without it, the compiler
  emits a stack-frame prologue at `_start` that faults immediately (SP
  uninitialised at rootserver entry). `examples/hello.c` wired to
  `SysDebugPutChar` (AArch64 `svc #0`; x7=-9, x0=char); prints "hello from
  seL4 rootserver" before yield loop. `build-totebox.sh` removed.
  NEXT.md + system-core NEXT.md updated.
- 2026-05-27 (Phase 1C.a / v0.2.0): `build` subcommand — replace stub with
  real `std::process::Command`. `CompilePd`: aarch64-linux-gnu-gcc v13.3.0
  with -nostdlib -nostartfiles -ffreestanding -static -no-pie -march=armv8-a.
  Verified: `moonshot-toolkit build examples/hello-world.toml` produces
  `build/hello.elf` (AArch64 static ELF, entry 0x40010c). CHANGELOG.md
  created. QEMU boot blocked on elfloader (Phase 1C.c). Image assembly
  blocked on Microkit/Rust assembler (Phase 1C.d).
- 2026-05-28 (Phase 1C.c): QEMU boot confirmed. "hello from seL4 rootserver"
  output. kernel.elf rebuilt with KernelVerificationBuild=OFF KernelPrinting=ON;
  Python CPIO writer (gen_cpio.py) replaces GNU cpio; QEMU -m 1G required.
  Source committed: `vendor-sel4-project/build-support/qemu-arm-virt/`.
- 2026-05-27 (seL4 kernel / Phase 1C.b): `vendor-sel4-kernel/build/
  aarch64-qemu/kernel.elf` built with aarch64-linux-gnu-gcc v13.3.0,
  KernelPlatform=qemu-arm-virt, KernelSel4Arch=aarch64. Rebuilt with
  KernelPrinting=ON for Phase 1C.c.
- 2026-05-27: NEXT.md + CLAUDE.md updated to v0.1.3 delivered state;
  Group 3A blockers resolved; decisions recorded in project-system-todo.md.
- 2026-04-27 (Phase 1B): `src/spec.rs` (445 lines, 12 tests) + `src/plan.rs`
  (310 lines, 10 tests) + `src/main.rs` CLI rewrite (241 lines, 8 tests).
  30 tests total (`cargo test --all-targets`). v0.1.3.
