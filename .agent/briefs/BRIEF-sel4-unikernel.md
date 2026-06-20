---
artifact: brief
schema: foundry-brief-v1
brief-id: project-console-sel4-unikernel
title: "os-console seL4 unikernel substrate — Phase H roadmap"
status: active
owner: totebox@project-console
created: 2026-06-19
updated: 2026-06-20 (H1/H2a/H2b/H2c/H3/H4/H5 all COMPLETE)
---

# BRIEF — os-console seL4 unikernel substrate

## Context

os-console is the keyboard-native Totebox terminal. The long-range goal (Leapfrog 2030)
is for os-console to run as a seL4 unikernel: each F-key cartridge is a separate seL4
Protection Domain (PD), formally isolated by capability tokens rather than OS processes.
"Geometric Protection" — the access graph between cartridges is a mathematically proven
directed acyclic graph; no ACL list, no ambient authority.

Phase H is the substrate track that builds toward this. It runs parallel to the cartridge
track (Phase 11+ = BIM, etc.) and uses the same monorepo.

Vision document: plan file `can-we-make-a-bubbly-quasar.md` (session 2026-06-19).

## Scope

`moonshot-sel4-vmm` — sovereign seL4 PD runtime (this archive, no_std Rust).
`moonshot-toolkit` — TOML spec → bootable AArch64 elfloader ELF.
`vendor-sel4-kernel` — seL4 v15.0.0-dev vendored source (built, gitignored).
Development target: QEMU `qemu-system-aarch64 -machine virt -cpu cortex-a53 -m 1G`.
Production target: ARM64 bare metal (Phase H3+).

Out of scope until Phase H3: VirtIO clipboard, moonshot-hypervisor, macOS/KVM host VMM.

## Decisions locked

- 2026-06-19: **seL4 is the substrate**. hermit-os rejected (external arch); nanos rejected
  (commercial license). seL4 BSD-2-Clause vendored source = Tier 2 trusted. We own the PD runtime.
- 2026-06-19: **moonshot-sel4-vmm is the sovereign PD runtime** — not rust-sel4. We write
  the seL4 ABI wrappers ourselves (~300 lines). Eliminates external foundation dependency.
- 2026-06-19: **QEMU `-m 1G` required** — vendor-sel4-kernel DTB reports 1 GiB of RAM;
  512M causes kernel Data Abort at physical 0x7FFC0000.
- 2026-06-19: **Phase H1 COMPLETE** — QEMU gate passed: "Hello from os-console seL4 PD"
  on serial. Commit `e2dd8d70`. SysDebugPutChar syscall ABI confirmed working.
- 2026-06-19: **cargo check on host is expected to fail** for moonshot-sel4-vmm — it is a
  bare-metal AArch64 no_std crate; must be compiled with `--target aarch64-unknown-none`.
  Not a blocker; QEMU gate is the ground truth.
- 2026-06-19: **Phase H2a COMPLETE** — QEMU gate passed: "Hello from moonshot-sel4-vmm (Rust)"
  on serial. Pure Rust PD (`console_main.rs`) with no C. `CompileRustPd` build step in
  moonshot-toolkit. QEMU chardev file command: `-chardev file,id=s0,path=/tmp/sel4-serial.log
  -serial chardev:s0 -m 1G`. Rust ABI wrappers confirmed end-to-end. Commits: `0e8cfef5` (BRIEF
  + NEXT), `e25b6ad7` (CompileRustPd + panic handler), H2a completion commit (console_main.rs
  + os-console-rust.toml).

## Decisions open

- H2b IPC: bare seL4 IPC (rootserver-distributes-caps) vs Microkit protocol. Microkit
  is cleaner but requires implementing Microkit bootstrap. Bare seL4 is more direct for H2b.
- H2c VirtIO: MMIO vs PCI. QEMU virt exposes VirtIO over MMIO at 0x0a000000+ (simpler)
  and over PCI. MMIO preferred for Phase H2c (no PCI discovery needed).
- ratatui no_std: ratatui 0.29 requires std. Options: (a) TestBackend → print buffer via
  SysDebugPutChar, (b) write a minimal no_std terminal buffer ourselves. Decision at H2c.
- Capability bootstrap: how rootserver distributes endpoint caps to child PDs. Will decide
  when starting H2b based on seL4 manual §4 (CNode operations).

## Work log

### Phase H0 — Not built
Alpine/QEMU guest validation. Deferred; H1 direct-to-seL4 made H0 redundant.

### Phase H1 — COMPLETE 2026-06-19

**Gate passed:** `Hello from os-console seL4 PD` on QEMU serial output.

Files committed (`e2dd8d70`):
- `moonshot-sel4-vmm/src/syscall.rs` — seL4 AArch64 ABI wrappers (SysDebugPutChar,
  SysYield, SysCall, SysRecv, SysSend); all asm gated `#[cfg(target_arch = "aarch64")]`
- `moonshot-sel4-vmm/src/debug.rs` — putchar / puts / puts_line / spin
- `moonshot-sel4-vmm/src/types.rs` — MsgInfo (label/caps/length) + ChannelId
- `moonshot-toolkit/examples/console_hello.c` — bare-metal C rootserver; SysDebugPutChar
  banner; infinite SysYield loop
- `moonshot-toolkit/examples/os-console-hello.toml` — system spec (single PD)

Key diagnostic: seL4 elfloader CPIO archive (kernel.elf + kernel.dtb + rootserver ELF)
assembled by moonshot-toolkit's pure-Rust AssembleImage. Cargo lock contention bypassed
via `CARGO_TARGET_DIR=/tmp/moonshot-h1-build`.

### Phase H2a — Rust rootserver — COMPLETE 2026-06-19

**Gate passed:** "Hello from moonshot-sel4-vmm (Rust)" on QEMU serial. No C in the PD.

**QEMU boot command:**
```
CARGO_TARGET_DIR=/tmp/moonshot-h2a-build \
  cargo run --manifest-path moonshot-toolkit/Cargo.toml -- \
  build moonshot-toolkit/examples/os-console-rust.toml
qemu-system-aarch64 -machine virt -cpu cortex-a53 -m 1G -display none \
  -chardev file,id=s0,path=/tmp/sel4-serial.log -serial chardev:s0 \
  -kernel build/system-image.bin < /dev/null
cat /tmp/sel4-serial.log
```

**What shipped:**
- `moonshot-toolkit/src/spec.rs` — `rust_bin: Option<String>` field on `ProtectionDomain`;
  for Rust PDs `binary` = crate dir name, `rust_bin` = `--bin` target name
- `moonshot-toolkit/src/plan.rs` — `BuildCommand::CompileRustPd` variant; `from_spec` emits
  it when `pd.rust_bin.is_some()`; 37 tests pass
- `moonshot-toolkit/src/main.rs` — `compile_rust_pd()` invokes `cargo build
  --manifest-path <crate>/Cargo.toml --target aarch64-unknown-none --release --bin <bin>`
- `moonshot-sel4-vmm/src/lib.rs` — `#[panic_handler]` (cfg-gated `aarch64`)
- `moonshot-sel4-vmm/src/bin/console_main.rs` — pure Rust `_start()` → banner →
  `vmm::spin()`; 232-byte BANNER includes gate text
- `moonshot-toolkit/examples/os-console-rust.toml` — `rust_bin = "console_main"` spec

### Phase H2b — Two PDs + seL4 IPC — COMPLETE 2026-06-19

**Gate passed:** "IPC gate: PASSED" + counter value printed. Commit `b399f1f9`.
Serial PD + Console PD string IPC (bordered box) also complete: commit `ea61ad97`.

**Goal:** rootserver boots two PDs; they exchange a message via seL4 endpoint. Proves
the IPC path that all future cartridge isolation depends on.

**Architecture:**
```
rootserver (Rust)
  → creates Endpoint object from untyped memory
  → creates two TCBs + VSpaces + CNodes
  → mints endpoint cap into each child's CNode
  → starts counter-pd + receiver-pd

counter-pd: sends counter value (0..9) to endpoint
receiver-pd: receives + prints "IPC received: N" via SysDebugPutChar
```

**What needs to be written:**
- `moonshot-sel4-vmm/src/bootstrap.rs` — rootserver CSpace/VSpace setup helpers
  (~150 lines). Covers: retype untyped → endpoint, retype untyped → CNode/TCB/VSpace,
  copy caps into child CNodes, configure TCB, resume.
- `moonshot-toolkit/examples/counter_pd.c` or counter_pd.rs — counter sender
- `moonshot-toolkit/examples/receiver_pd.c` or receiver_pd.rs — receiver printer
- `moonshot-toolkit/examples/os-console-ipc.toml` — 3-PD system spec
  (rootserver + counter-pd + receiver-pd). moonshot-toolkit may need channel support.

**Gate:** "IPC received: 5" (or similar) printed via receiver-pd.

**Estimated effort:** 6-10 hours. The CSpace bootstrap is the hard part — requires
reading seL4 manual §4 (CNode invocations) and §6 (TCB invocations) carefully.

**Key references:**
- seL4 manual: `vendor-sel4-kernel/src/manual/` or built PDF
- seL4 initial caps layout: `seL4_BootInfo` struct tells rootserver what untyped
  memory regions it has to work with

### Phase H2c — UART MMIO access from user space — COMPLETE 2026-06-20

**Gate passed:** "UART gate: PASSED\r\n" via direct write_volatile to PL011 UARTDR.
Commit `fae0f517`. Key lesson: AArch64 4-level walk requires TWO ARMPageTableMap
invocations (L1 PUD + L2 PMD) before ARMPageMap can install a SmallPage. One call
leaves lookupPTSlot at ptBitsLeft=21 ≠ 12 → ARMPageMap fails. Device untyped found
via exact paddr match (staircase analysis: 16 MiB untyped starts exactly at 0x09000000).

**Goal:** PD writes to PL011 UART at 0x09000000 directly (not via SysDebugPutChar).
This is the step before VirtIO — validates that MMIO mapping from a seL4 PD works.

### Phase H3 — UART MMIO gate via seL4 device-untyped — COMPLETE 2026-06-20

**Gate passed:** "UART gate: PASSED\r\n" via direct write_volatile to PL011 UARTDR at
virtual 0x40000000. Committed as part of `85367867` (Phase H3+H4 dual-phase commit).

**What shipped:** `bin/uart_main.rs` — device-untyped → SmallPage → two ARMPageTableMap
(L1 PUD + L2 PMD) → ARMPageMap → write_volatile to PL011 UARTDR at 0x09000000.

Key lesson: AArch64 4-level walk requires TWO ARMPageTableMap invocations before ARMPageMap.

### Phase H4 — ANSI cartridge panel via serial PD IPC — COMPLETE 2026-06-20

**Gate passed:** "Panel gate: PASSED" in QEMU serial output. Committed in `85367867`.

**What shipped:** `bin/panel_main.rs` + `src/ansi.rs` — two-TCB system (serial_pd +
console_pd); ANSI clear+home + bordered box + 7 cartridge rows with GREEN [ACTIVE] labels;
IPC protocol: MR[0]=chunk byte count, MR[1..3]=packed LE bytes (24 bytes/message).

### Phase H5 — VirtIO MMIO bus probe via seL4 capability mapping — COMPLETE 2026-06-20

**Gate passed:** "VirtIO-net init gate: PASSED" in QEMU serial output.

**What shipped:** `bin/virtio_net_init.rs` — same two ARMPageTableMap + ARMPageMap chain
applied to VirtIO MMIO bus at 0x0a000000 (virtual 0x40200000); reads MAGIC=0x74726976
(VirtIO MMIO transport present); VERSION=1 (legacy), STATUS progression written.
DEVICE_ID=0 at slot 0: QEMU assigns virtio-net to slot 31 (0x0a003e00); H6 will scan slots.

**QEMU command:** `qemu-system-aarch64 -machine virt -cpu cortex-a53 -m 1G -nographic
  -device virtio-net-device,netdev=n0 -netdev user,id=n0 -kernel build/system-image.bin`

### Phase H6 — VirtIO-net slot scan + virtqueue ring setup (next)

**Goal:** Scan all 32 VirtIO MMIO slots (0x0a000000 to 0x0a003e00, stride 0x200) to find
DEVICE_ID=1 (network). Set up receive virtqueue ring. Gate: "VirtIO-net device_id=1 init
complete" after STATUS progression to DRIVER_OK.

**Key:** Must map 32 consecutive 512-byte regions. Options: (a) map one page covering
0x0a000000-0x0a000fff (8 VirtIO slots × 512 B) and probe with stride, or (b) map only
the target slot once found. Option (a) covers all 8 slots in one SmallPage.

### Phase H7 — smoltcp ICMP ping (future)

**Goal:** Implement virtqueue ring (descriptor table + avail ring + used ring); send an
ICMP echo request via QEMU user-mode network; receive ICMP echo reply. Proves the DMA
path works end to end.

### Phase H8 — HTTP GET to Doorman health endpoint (future)

**Goal:** smoltcp TCP + HTTP/1.1 GET to `10.0.2.2:9080/doorman/health` (QEMU user-mode
NAT puts host at 10.0.2.2). Proves the full network stack to a real Totebox service.

## Carry-forward

- H1–H5 all COMPLETE. Phase H6 next: scan VirtIO MMIO slots for DEVICE_ID=1 + virtqueue ring.
- Stage 6 pending: commits through Phase H5 (commit `85367867` + H5 commit) need `bin/promote.sh`
  from Command Session. Route via outbox.
- M-17 contamination in `.agent/briefs/` — cross-archive BRIEFs present; Command Session sweep
  pending (noted in prior session).
