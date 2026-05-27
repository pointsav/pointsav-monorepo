# NEXT.md — moonshot-toolkit

> Last updated: 2026-05-27
> Read at session start. Update before session end.

---

## Right now

- Nothing in progress. Phase 1C.a complete at v0.2.0 (`build` subcommand
  executes real aarch64-linux-gnu-gcc; `examples/hello-world.toml` +
  `examples/hello.c` added; CHANGELOG.md created). Phase 1C.c and 1C.d
  are blocked — see Blocked section below.

## Queue

- **Sigstore Cosign + customer-apex cosignature** — `plan_hash` field is in place; cosignature emission deferred until `moonshot-toolkit build` produces a full bootable image (post-#14 + post-Phase 1C.d).
- **`build-totebox.sh` removal** — still present; remove once moonshot-toolkit build produces a real bootable image end-to-end (Phase 1C.d complete).

## Blocked

- **Phase 1C.c — QEMU boot** — seL4 kernel ELF (`vendor-sel4-kernel/build/
  aarch64-qemu/kernel.elf`, AArch64) is built. But seL4 requires the
  `elfloader` (from `seL4_tools` repo, separate from kernel source) to
  set up the MMU before jumping to the kernel at 0xffffff8040000000.
  Without the elfloader, QEMU loads the ELF but the kernel can't boot.
  Unblocked by: cloning `seL4_tools` repo + building elfloader + linking
  kernel + rootserver into combined bootable image.

- **Phase 1C.d — Image assembly** — `AssembleImage` step is a stub
  returning an actionable error. Requires either:
  1. Microkit SDK (`microkit.py`) — installs as a Python package; acceptable
     as interim tool while Rust assembler is built.
  2. Rust image assembler — new crate in `moonshot-toolkit/src/assemble.rs`
     implementing Microkit image format (ELF packing + system description).
  Unblocked by: either option above.

## Deferred

- `build-totebox.sh` legacy shell sketch — kept as migration reference
  until `moonshot-toolkit build` produces a bootable image end-to-end
  (post-#14). Remove then.
- `no_std` eligibility — not a constraint for the build orchestrator
  (runs on the workspace VM, not in the kernel). No action needed.

## Recently done

- 2026-05-27 (Phase 1C.a / v0.2.0): `build` subcommand — replace stub with
  real `std::process::Command`. `CompilePd`: aarch64-linux-gnu-gcc v13.3.0
  with -nostdlib -nostartfiles -ffreestanding -static -no-pie -march=armv8-a.
  Verified: `moonshot-toolkit build examples/hello-world.toml` produces
  `build/hello.elf` (AArch64 static ELF, entry 0x40010c). CHANGELOG.md
  created. QEMU boot blocked on elfloader (Phase 1C.c). Image assembly
  blocked on Microkit/Rust assembler (Phase 1C.d).
- 2026-05-27 (seL4 kernel / Phase 1C.b): `vendor-sel4-kernel/build/
  aarch64-qemu/kernel.elf` built with aarch64-linux-gnu-gcc v13.3.0,
  KernelPlatform=qemu-arm-virt, KernelSel4Arch=aarch64, KernelPrinting=ON.
  AArch64 static ELF, entry 0xffffff8040000000. Needs elfloader for QEMU boot.
- 2026-05-27: NEXT.md + CLAUDE.md updated to v0.1.3 delivered state;
  Group 3A blockers resolved; decisions recorded in project-system-todo.md.
- 2026-04-27 (Phase 1B): `src/spec.rs` (445 lines, 12 tests) + `src/plan.rs`
  (310 lines, 10 tests) + `src/main.rs` CLI rewrite (241 lines, 8 tests).
  30 tests total (`cargo test --all-targets`). v0.1.3.
