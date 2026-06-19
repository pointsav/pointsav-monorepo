---
artifact: topic
schema: foundry-draft-v1
title: "seL4 Unikernel Substrate for os-console"
lang: en
route: project-editorial
status: draft
created: 2026-06-19
updated: 2026-06-19
brief-id: project-console-os-console-hypervisor
doctrine_anchors: [claim-34, SYS-ADR-10, SYS-ADR-19]
research_trail:
  sources: [BRIEF-os-console-hypervisor.md, moonshot-toolkit-v0.3.1-source, seL4-Microkit-2.2.0-manual, vendor-sel4-kernel-v15.0.0-dev]
  reviewed_by: totebox@project-console
  research_date: 2026-06-19
  session_context: can-we-make-a-bubbly-quasar radical substrate research session
  verification_method: moonshot-toolkit 35 tests passing; seL4 AArch64 boots to user space in QEMU
---

# seL4 Unikernel Substrate for os-console

os-console is intended to run as a seL4 Microkit unikernel image in its final
production form (planned Phase H2). This article explains what that means, what
already works, and what remains to be built.

---

## What Is a Unikernel

A unikernel is an application compiled directly with the operating system primitives
it requires, producing a single bootable binary. There is no general-purpose OS,
no shell, no package manager, no user account system, and no attack surface beyond
the application's own code and the minimal kernel it depends on.

The distinction from a conventional VM:

| Conventional VM | Unikernel |
|---|---|
| Guest OS (Linux/BSD) + application | Application + minimal kernel |
| General-purpose OS: shell, users, packages | Single-purpose: one application only |
| Shared kernel attack surface | No shared kernel; no ambient authority |
| ~500 MB to 2 GB typical footprint | 10–50 MB typical footprint |
| Boot time: 5–30 seconds | Boot time: < 1 second |
| Exploit: OS misconfig, privilege escalation | Exploit: only application-layer bugs |

A unikernel cannot be "rooted" in the conventional sense because there is no root to
escalate to. There is no shell to drop into. There is only the application and its
formally bounded capability set.

---

## seL4 Microkit

seL4 is a formally verified microkernel. The seL4 kernel has been verified using
machine-checked proofs (Isabelle/HOL) establishing correctness of the kernel's
capability model, memory management, and IPC mechanisms. These are not unit tests —
they are mathematical proofs over the kernel's entire implementation.

seL4 Microkit is seL4's minimal operating environment for embedded and unikernel
applications. It defines:

**Protection Domains (PDs):** The unit of isolation. Each PD has its own capability
namespace and cannot read or write another PD's memory. PDs are statically declared
at build time in a TOML system specification.

**Protected Procedure Call (PPC):** Synchronous IPC between PDs. A PD invokes a
PPC endpoint. The kernel switches execution context. The callee returns. The caller
resumes. No shared memory is involved unless explicitly mapped with a capability.

**Channels and Notifications:** Asynchronous communication between PDs via kernel-
mediated notification objects. A PD that has the channel capability can signal; a PD
that has the notification capability can receive.

---

## What Already Works

moonshot-toolkit v0.3.1 (our build orchestrator, 35 tests passing) already produces
bootable seL4 AArch64 images:

```toml
# examples/hello-world.toml — working today
[system]
kernel = "vendor-sel4-kernel/build/aarch64-qemu/kernel.elf"
elfloader = "vendor-sel4-tools/elfloader-tool"

[[pd]]
name = "hello-pd"
binary = "examples/hello.elf"
priority = 100
```

Running `cargo run -- build examples/hello-world.toml` produces an elfloader ELF.
QEMU boots it to: `Booting all finished, dropped to user space`.

vendor-sel4-kernel (v15.0.0-dev, BSD-2-Clause) is vendored in the monorepo and
built from source. vendor-sel4-tools (elfloader, 44 C/ASM sources) is vendored and
compiled by moonshot-toolkit.

The kernel boots. The infrastructure is in place.

---

## The 3-Protection-Domain Design for os-console

The intended seL4 system image for os-console contains three Protection Domains:

```
os-console seL4 system image
┌─────────────────────────────────────────┐
│ os-console PD        priority 100       │
│  Cartridges: F2 F3 F4 F6 F9 F11 F12   │
│  ratatui TUI; no network access direct │
│  Stack: 256 KiB; heap: 1 MiB           │
└──────────┬──────────────────┬───────────┘
           │ PPC (sync IPC)   │ PPC (sync IPC)
           ▼                  ▼
┌──────────────┐   ┌────────────────────┐
│ network-pd   │   │ serial-pd          │  priority 150/180
│ smoltcp      │   │ VirtIO serial      │
│ VirtIO-net   │   │ ratatui output     │
│ HTTP/1.1     │   │ keyboard input     │
└──────────────┘   └────────────────────┘
       ▲
       │ VirtIO-net (DMA capability)
       ▼
moonshot-hypervisor → host network → Totebox services
```

os-console PD makes an HTTP request by calling the network-pd via PPC. The network-pd
holds the VirtIO-net device capability. The os-console PD never directly holds a
network device capability. If the os-console PD is compromised, it cannot exfiltrate
data directly over the network — it can only call the network-pd via its defined
PPC interface.

---

## moonshot-sel4-vmm: The Sovereign PD Runtime

The seL4 Microkit requires a small Rust runtime inside each PD: an entry point,
a heap allocator, and system call wrappers for seL4's ABI. External projects provide
this (rust-sel4 from the seL4 Foundation). The planned Foundry approach is to write
our own in moonshot-sel4-vmm.

The seL4 ABI is small and formally verified — it does not change arbitrarily because
the proofs constrain it. Writing our own bindings (~300 lines of Rust) takes the same
time as integrating an external library but leaves the code under our control.

moonshot-sel4-vmm is intended to provide:
- `_start()` → stack/heap initialization → `pd_main()`
- seL4 system call wrappers (`sel4_call`, `sel4_send`, `sel4_recv`)
- `microkit_msginfo_t` IPC type matching the Microkit ABI
- `notified(ch: u64)` and `protected(ch: u64, msginfo)` callbacks per the Microkit protocol
- `DebugPutChar` for development-time output

This crate is shared across all three OS-family binaries: os-console PDs, os-totebox
service PDs, and os-orchestration app PDs all use the same runtime.

---

## The "We Own It" Stack

The intended runtime dependency chain for os-console as a seL4 unikernel:

| Layer | Component | Status |
|---|---|---|
| Application | os-console cartridge code | Active; ours |
| Build orchestrator | moonshot-toolkit v0.3.1 | Active; ours |
| Host VMM | moonshot-hypervisor | Scaffold; ours — to be filled |
| PD runtime | moonshot-sel4-vmm | Scaffold; ours — Phase H1 fill-in |
| Capability substrate | system-core, system-ledger v1.0.0 | Active; ours |
| Kernel | vendor-sel4-kernel v15.0.0-dev | Vendored BSD-2-Clause; we build it |
| Elfloader | vendor-sel4-tools | Vendored BSD-2-Clause; we compile it |
| Network PD | smoltcp | MIT; vendorable; replaces reqwest |
| Dev boot | QEMU | Dev tool only; not in product image |

Nanos (commercial unikernel) and hermit-os (external mini-OS architecture) are not
used. The path to a working prototype is through moonshot-sel4-vmm (~300 lines), which
takes the same time as integrating an external library and leaves every layer under
Foundry control.

---

## Phase Roadmap

**Phase H0 (current):** Alpine Linux in QEMU — validates the service stack before
investing in the seL4 substrate. No seL4 code required.

**Phase H1 (planned, 4–6 weeks):** Fill in moonshot-sel4-vmm. Boot os-console as a
single seL4 PD. Render the TUI via VirtIO serial. Connect to a test Totebox service
via smoltcp network PD. VirtIO clipboard working (non-optional for SMB operators).

**Phase H2 (planned, 8–16 weeks):** Full 3-PD design. moonshot-hypervisor replaces
QEMU. os-console image built by moonshot-toolkit from `examples/os-console-sel4.toml`.
Boots in under 1 second. 100% sovereign stack; QEMU removed from product path.

**Phase H3 (intended, Leapfrog 2030):** F11 pairing becomes capability minting.
machine-level seL4 capability tokens. system-core Capability types carried in the
PD's CNode. Revocation via system-ledger propagates to kernel level.
