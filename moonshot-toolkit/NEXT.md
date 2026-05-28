# NEXT.md — moonshot-toolkit

> Last updated: 2026-05-27
> Read at session start. Update before session end.

---

## Right now

- Nothing in progress. Phase 1C.c complete (`d550217`): seL4 qemu-arm-virt
  AArch64 QEMU boot confirmed — "hello from seL4 rootserver" output observed.
  Phase 1C.d still blocked — see Blocked section below.

## Queue

- **Sigstore Cosign + customer-apex cosignature** — `plan_hash` field is in place; cosignature emission deferred until `moonshot-toolkit build` produces a full bootable image (post-#14 + post-Phase 1C.d).
- **`build-totebox.sh` removal** — still present; remove once moonshot-toolkit build produces a real bootable image end-to-end (Phase 1C.d complete).

## Blocked

- ~~**Phase 1C.c — QEMU boot**~~ COMPLETE (`d550217`, 2026-05-28).
  Boot: elfloader → seL4 kernel → hello-rootserver → "hello from seL4 rootserver".
  Three root causes resolved: KernelVerificationBuild=ON disabled CONFIG_PRINTING;
  GNU cpio padding mismatch (replaced with Python CPIO writer); QEMU -m 512M <
  kernel DTB range (boot with -m 1G). Elfloader entry 0x40400000, kernel at
  0xffffff8040000000, rootserver at 0x400000.

- **Phase 1C.d — Image assembly** — `AssembleImage` step returns an actionable
  error. Requires either:
  1. Microkit SDK tarball — available from `github.com/seL4/microkit/releases`
     as a pre-built release (e.g. `microkit-sdk-1.4.0-linux-x86-64.tar.gz`).
     Provides `bin/microkit` CLI: `microkit <system.xml> --board qemu-arm-virt
     --config debug --search-path build/ --output build/system.img`.
     Note: the `microkit` PyPI package is an unrelated Flask helper — do not install.
  2. Rust image assembler — `moonshot-toolkit/src/assemble.rs` implementing the
     Microkit image format (ELF packing + manifest). Preferred path per MEMO §7
     Rust-Only mandate; requires documenting the Microkit image format spec first.
  Unblocked by: either option above. Rust path is preferred long-term.

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
