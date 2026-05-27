---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: vendor/content-wiki-documentation
target_path: ./
target_filename: topic-sel4-aarch64-qemu-substrate-target.md
audience: vendor-internal
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-27T02:00:00Z
authored_by: task-project-system (session phase-1c-b)
authored_with: claude-sonnet-4-6
references:
  - vendor-sel4-kernel/src/VERSION (v15.0.0-dev)
  - vendor-sel4-kernel/src/src/plat/qemu-arm-virt/config.cmake
  - vendor-sel4-kernel/src/gcc.cmake
  - vendor-sel4-kernel/src/src/plat/qemu-arm-virt/overlay-qemu-arm-virt.dts
  - DOCTRINE.md §IV.a
  - project-system-todo.md §179 (Group 3A decisions) + §221 (Group 3D decisions)
  - Phase 1C.b result: kernel.elf (AArch64 static ELF, entry 0xffffff8040000000)
  - https://github.com/seL4/seL4 (seL4 kernel source)
  - https://github.com/seL4/seL4_tools (elfloader; separate repo)
notes_for_editor: |
  Substance pass complete. Written from source code in vendor-sel4-kernel/src/
  at seL4 v15.0.0-dev, plus verified build output from Phase 1C.b (2026-05-27).
  Audience is vendor-internal: assumes the reader knows what a microkernel is but
  not specifically seL4 or AArch64 bare-metal boot.

  Key discipline: this TOPIC is about the TARGET (seL4 on AArch64 QEMU), not about
  the build orchestrator (that is topic-moonshot-toolkit-build-orchestrator.md).
  Focus on: what seL4 is, why AArch64, how the kernel is configured, why the elfloader
  gap exists, what Phase 1C.c requires to close it.

  "Honest We Own It" posture per system-substrate-doctrine.md §8: be precise about
  what is Foundry-owned vs. what seL4.systems provides. seL4 is a third-party
  formally verified kernel; Foundry builds on it, does not own it.

  BCSC posture: Phase 1C.c and 1C.d completion are planned/intended. The kernel
  build (Phase 1C.b) is current fact; QEMU boot (Phase 1C.c) is intended.

  Banned-vocab + BCSC discipline + bilingual .es.md generation: project-language enforces.
---

# The seL4 AArch64 QEMU Substrate Target

Foundry's unikernel images run on the seL4 microkernel targeting the AArch64
instruction set architecture, with QEMU's `virt` machine model as the primary
emulation environment for development, testing, and CI. This target was selected
through a set of architecture decisions (Group 3A and Group 3D) made in May 2026
and is the hardware foundation for all Phase 1C and Phase 2 work.

## 1. seL4 as the Microkernel Foundation

seL4 is a formally verified L4-family microkernel developed by CSIRO's Data61 and
maintained by the seL4 Foundation. Its defining property is a machine-checked proof
of functional correctness: the kernel implementation is proven to match a formal
specification at the Isabelle/HOL level. This proof covers the AArch64 target.

seL4 uses a capability-based access control model. Every kernel resource — memory,
threads, IPC endpoints, interrupt handlers — is accessible only through a capability,
a typed unforgeable token held in a kernel-managed capability space. Sharing a
resource means delegating a capability; revocation removes it. This model is the
foundation on which the Capability Ledger Substrate (Doctrine claim #33) is built:
the ledger extends seL4's kernel-enforced access control with a cryptographically
auditable record of every capability decision.

The Foundry substrate does not modify the seL4 kernel. It builds on the kernel's
published API (libsel4) and the Microkit framework's protection-domain model.

## 2. AArch64 First

The AArch64 (ARMv8-A 64-bit) architecture was selected as the primary target for
two reasons.

First, the proof portfolio. The seL4 formal proof covers AArch64 as a first-class
target, alongside x86-64 and RISC-V 64. AArch64 has the longest track record of
continuous proof maintenance in the seL4 project and is used in the seL4 foundation's
own continuous-integration pipelines.

Second, the hardware trajectory. AArch64 server processors (Ampere Altra, AWS
Graviton, Neoverse N1/V1) are available in the cloud providers relevant to Foundry's
deployment targets. The QEMU `virt` machine model provides a faithful AArch64
emulation environment that maps directly to these physical targets.

x86-64 is not excluded — seL4 supports it — but AArch64-first means that Foundry's
toolchain investment, build infrastructure, and test matrix are calibrated to AArch64
from the outset. x86-64 support, if needed, inherits from the same build pipeline.

## 3. The QEMU virt Machine Model

QEMU's `virt` machine model for AArch64 (`-machine virt`) is a synthetic platform
with no fixed hardware correspondence. It is designed specifically for software
development and emulation. Key characteristics relevant to seL4:

**CPU:** The default target is Cortex-A53 (`-cpu cortex-a53`), an ARMv8-A
implementation with hardware virtualisation extensions. seL4 on AArch64 runs at
Exception Level 1 (EL1) in non-virtualisation mode for the kernel, with user
processes at EL0.

**Interrupt controller:** GIC version 2 by default (`QEMU_GIC_VERSION=2`).
seL4's AArch64 port uses the ARM Generic Interrupt Controller; the QEMU virt
platform provides a software GIC compatible with the seL4 driver.

**UART:** PL011 serial controller at physical address `0x09000000`. seL4's
`KernelPrinting` and `KernelDebugBuild` options route kernel debug output to this
UART, which QEMU maps to the emulator's standard output when `-nographic` is used.

**Device tree:** The seL4 CMake build system extracts the machine's device tree
binary from QEMU at configure time, then converts it to DTS and compiles it into
the kernel. This is why `qemu-system-aarch64` must be present before the seL4 kernel
can be configured.

## 4. Kernel Build Configuration

The seL4 kernel is built from source at `vendor-sel4-kernel/src/` using CMake with
a GCC cross-compile toolchain. The critical configuration options for the QEMU
AArch64 target are:

```
-DCMAKE_C_COMPILER=aarch64-linux-gnu-gcc
-DCMAKE_ASM_COMPILER=aarch64-linux-gnu-gcc
-DCROSS_COMPILER_PREFIX=aarch64-linux-gnu-
-DKernelPlatform=qemu-arm-virt
-DKernelArch=arm
-DKernelSel4Arch=aarch64
-DKernelPrinting=ON
-DKernelDebugBuild=ON
```

`KernelPrinting=ON` enables the kernel's serial output via the PL011 UART.
`KernelDebugBuild=ON` enables debug assertions and additional diagnostic output.
Both are appropriate for development and testing; a production image would use
`CMAKE_BUILD_TYPE=Release` without these options.

The build produces `kernel.elf`, a statically linked AArch64 ELF executable. As
of Phase 1C.b (2026-05-27), this build succeeds from the seL4 v15.0.0-dev source
tree with the `aarch64-linux-gnu-gcc` v13.3.0 toolchain on Ubuntu 24.04.

## 5. The Elfloader Gap

The seL4 kernel ELF for AArch64 has an entry point at virtual address
`0xffffff8040000000`. This is the kernel's intended virtual address once the
AArch64 MMU is configured and the kernel's own page tables are in place.

QEMU, however, loads ELF files at their stated virtual addresses in physical memory.
When QEMU is told to `-kernel kernel.elf`, it loads the binary at address
`0xffffff8040000000` in physical address space. This address does not exist on the
QEMU `virt` machine, which provides physical memory starting at `0x40000000`. The
kernel cannot execute.

The standard seL4 boot flow resolves this through the **elfloader**, a small
bootstrap program provided by the `seL4_tools` repository (a separate repository
from the seL4 kernel source at `github.com/seL4/seL4_tools`). The elfloader:

1. Runs from a low physical address where QEMU can load it directly.
2. Unpacks the seL4 kernel ELF from a cpio archive embedded in the loader.
3. Configures the AArch64 MMU page tables to map the kernel's virtual address
   to the correct physical location.
4. Unpacks the initial user-level thread (rootserver) from the same archive.
5. Jumps to the kernel entry point, now reachable through the MMU mapping.

The combined elfloader + kernel + rootserver image is what QEMU actually boots.
Without the elfloader, the kernel.elf file alone cannot be executed.

## 6. Phase 1C.b and the Path to QEMU Boot

Phase 1C.b (completed 2026-05-27) demonstrates that the seL4 kernel for AArch64
QEMU can be built from source on the Foundry workspace VM. The resulting
`kernel.elf` is a valid AArch64 static ELF with entry point 0xffffff8040000000,
confirmed with `aarch64-linux-gnu-readelf`.

Phase 1C.c — a full seL4 hello-world booting in QEMU with kernel output visible —
is intended to follow once the `seL4_tools` elfloader is available and a minimal
rootserver is written. The rootserver for this milestone is a C program that uses
seL4's `seL4_DebugPutChar` interface to print to the PL011 UART. The combined
elfloader + kernel + rootserver image is then loaded by QEMU using:

```
qemu-system-aarch64 -machine virt,secure=off -cpu cortex-a53 \
  -m 512M -nographic -kernel <combined-image>
```

## See Also

- `topic-moonshot-toolkit-build-orchestrator.md` — the Rust build orchestrator that
  cross-compiles protection domains for this target
- `topic-capability-ledger-substrate.md` — what the unikernel images enforce at runtime
- `guide-moonshot-toolkit-phase1c-build-setup.md` — operational runbook for the
  cross-compile and build environment
