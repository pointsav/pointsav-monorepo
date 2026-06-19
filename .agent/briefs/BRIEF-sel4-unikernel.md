---
artifact: brief
schema: foundry-brief-v1
brief-id: project-console-sel4-unikernel
title: "os-console seL4 unikernel substrate — Phase H roadmap"
status: active
owner: totebox@project-console
created: 2026-06-19
updated: 2026-06-19 (H2a COMPLETE)
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

### Phase H2b — Two PDs + seL4 IPC (Day 2)

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

### Phase H2c — UART MMIO access from user space (Day 3)

**Goal:** PD writes to PL011 UART at 0x09000000 directly (not via SysDebugPutChar).
This is the step before VirtIO — validates that MMIO mapping from a seL4 PD works.

**What changes:**
1. Rootserver maps the PL011 UART page (0x09000000–0x09001000) into the console-pd's
   VSpace using `seL4_ARM_Page_Map`.
2. console-pd writes UART registers directly: `DR` (0x09000000), `FR` (0x09000018).
3. No SysDebugPutChar — pure MMIO.

**Gate:** "Hello via MMIO UART" appears on QEMU serial, written by the PD directly.

**Estimated effort:** 4-6 hours. Requires ASID allocation + page table setup in
rootserver bootstrap.

### Phase H3 — VirtIO serial + ratatui (Week 2)

**Goal:** VirtIO serial device driver; ratatui renders to VirtIO console. First real TUI
on seL4.

**Architecture:**
```
serial-pd (priority 200):
  - maps VirtIO MMIO region (virt machine: 0x0a000000+)
  - implements virtqueue ring buffer setup
  - exposes write endpoint via seL4 IPC

console-pd (priority 100):
  - initialises ratatui with custom backend (calls serial-pd via IPC per write)
  - renders a layout: status bar + F-key list + activity pane
```

**VirtIO MMIO on QEMU virt:**
- Add `-device virtio-serial-device,bus=virtio-mmio-bus.0` to QEMU command
- Device probes at 0x0a000000; MMIO at standard offset
- Feature negotiation: bit 0 (VIRTIO_F_VERSION_1)
- Virtqueues: receiveq (0), transmitq (1)

**ratatui no_std approach (decision to confirm at H2c):**
- Option A: TestBackend renders to an in-memory buffer; console-pd prints buffer
  line by line via serial-pd IPC. Simplest.
- Option B: Implement `ratatui::backend::Backend` trait with a custom VirtIO backend.
  Cleanest. Requires ratatui to compile no_std (not current as of 0.29).

**Gate:** ratatui layout (borders, text, at least 2 panes) visible in QEMU serial output.

**Estimated effort:** 2-3 days.

## Carry-forward

- H2a COMPLETE. H2b next: two PDs + seL4 IPC — `bootstrap.rs` rootserver CSpace/VSpace
  setup (~150 lines); counter-pd + receiver-pd; Gate: "IPC received: N"
- Stage 6 pending: `0e8cfef5`, `e25b6ad7`, H2a completion commit — route to Command Session
- M-17 contamination in `.agent/briefs/` — cross-archive BRIEFs present; not this session's
  concern but note for Command Session sweep
