---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: vendor/content-wiki-documentation
target_path: ./
target_filename: topic-moonshot-toolkit-build-orchestrator.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-27T02:00:00Z
authored_by: task-project-system (session phase-1c-a)
authored_with: claude-sonnet-4-6
references:
  - MEMO-2026-03-30-Development-Overview-V8.md §7
  - conventions/system-substrate-doctrine.md §6 (Reproducible-Verification-On-Customer-Metal)
  - DOCTRINE.md claim #33
  - clones/project-system/moonshot-toolkit/src/spec.rs
  - clones/project-system/moonshot-toolkit/src/plan.rs
  - clones/project-system/moonshot-toolkit/src/main.rs
  - clones/project-system/moonshot-toolkit/examples/hello-world.toml
  - clones/project-system/moonshot-toolkit/ARCHITECTURE.md
  - clones/project-system/moonshot-toolkit/CHANGELOG.md
  - https://docs.sel4.systems/projects/microkit/manual/latest/
notes_for_editor: |
  Substance pass complete. Written from source code at v0.2.0 (commit 34a1111).
  All technical details verified against moonshot-toolkit/src/*.rs and examples/.

  Key discipline: the TOPIC documents what moonshot-toolkit IS and what it produces.
  It does not document how to run it (that is GUIDE territory). Cross-reference
  guide-moonshot-toolkit-phase1c-build-setup.md for the operational runbook.

  Phase 1C.a is complete: CompilePd produces a verified AArch64 bare-metal static ELF.
  Phase 1C.d (AssembleImage) is blocked on Microkit SDK or Rust image assembler —
  document this gap honestly using planned/intended language per BCSC posture.

  "Honest We Own It" posture: moonshot-toolkit ORCHESTRATES; it does not
  implement the cross-compile toolchain (aarch64-linux-gnu-gcc is a system dependency)
  and does not implement seL4 (that is vendor-sel4-kernel). Be precise.

  Banned-vocab + BCSC discipline + bilingual .es.md generation: project-language enforces.
  Avoid "blockchain" framing. This is a content-addressed build manifest, not a
  distributed ledger.
---

# The moonshot-toolkit Build Orchestrator

moonshot-toolkit is a Rust-only build orchestrator for Foundry's seL4 unikernel images,
replacing the Python and CMake toolchain provided by the seL4 Microkit framework. It
reads a TOML system specification, derives a deterministic content-addressed build
manifest, and orchestrates the cross-compilation of each software component for the
AArch64 bare-metal target.

## 1. Why Rust-Only

The seL4 Microkit framework ships a Python image-assembly script and a CMake build
system. These tools are adequate for general embedded development but present three
problems for a reproducible-build discipline.

First, determinism. Python dictionary ordering is implementation-defined; CMake
dependency discovery varies across versions. A Rust binary built from a fixed source
revision with vendored dependencies produces bit-identical plan bytes across machines
and over time.

Second, auditability. A single Rust binary is an auditable end-to-end artefact.
Python scripts, CMake modules, Makefiles, and shell wrappers compose a multi-language
audit surface that is difficult to reason about formally.

Third, network isolation. Foundry's Reproducible-Verification-On-Customer-Metal
convention (system-substrate-doctrine.md §6) requires that build steps run without
network access. Python's `pip install` and CMake's `find_package` are live network
surfaces. A Rust binary with vendored dependencies eliminates them.

## 2. SystemSpec — The Input

A system specification is a TOML file that describes a seL4 Microkit system. It is
the Rust-native equivalent of Microkit 2.2.0's system-description XML schema.

The specification declares four collections:

**Protection domains** are isolated, single-threaded software components scheduled by
the seL4 kernel. Each domain has a name, a path to its source binary, a scheduling
priority (0 is highest, matching seL4 and Microkit conventions), and a stack size in
bytes (defaulting to 4 KiB per Microkit). The system may contain at most 63 protection
domains, the hard limit imposed by the Microkit framework.

**Channels** are point-to-point communication links between protection domains, using
seL4's Protected Procedure Call or notification mechanisms. Each protection domain
may have at most 63 channels.

**Memory regions** declare physical memory mappings with caching and permission
attributes and an optional prefill from a binary blob. Overlapping regions are
rejected at parse time.

**IRQ delivery** entries bind hardware interrupt lines to specific protection domains.

Validation rules are enforced during parsing: no duplicate domain names, all channel
endpoints and IRQ targets must reference declared domains, and memory regions must
not overlap.

A minimal hello-world specification looks like:

```toml
[[protection_domains]]
name     = "hello"
binary   = "examples/hello.c"
priority = 100
stack_bytes = 65536
```

## 3. BuildPlan — The Manifest

Given a parsed SystemSpec, moonshot-toolkit generates a BuildPlan: a deterministic,
content-addressed build manifest.

The plan contains three fields. The `spec_hash` is the SHA-256 digest of the canonical
TOML rendering of the SystemSpec — it identifies the input. The `steps` field is an
ordered list of build steps: one CompilePd step per protection domain followed by a
single AssembleImage step. The `plan_hash` is the SHA-256 digest of the canonical
JSON rendering of `(spec_hash, steps)` — it is the value the customer-apex cosignature
commits to.

The determinism guarantee is strict: the same SystemSpec always produces the same
`plan_hash`. This property is covered by tests and is foundational to the
Reproducible-Verification-On-Customer-Metal convention: a customer can verify that a
delivered binary corresponds to a known specification by recomputing the plan_hash and
comparing it against a cosigned value.

## 4. Build Commands

A BuildPlan contains two kinds of steps.

**CompilePd** cross-compiles a protection domain's source file to a bare-metal AArch64
ELF binary. The invocation uses the `aarch64-linux-gnu-gcc` cross-compiler with flags
appropriate for seL4 Microkit protection domains:

- `-nostdlib -nostartfiles`: no C standard library or startup files; the protection
  domain provides its own `_start` entry point.
- `-ffreestanding`: no hosted-environment assumptions; no implicit includes.
- `-static -no-pie`: Microkit loads PD binaries at fixed virtual addresses; dynamic
  linking is not available.
- `-march=armv8-a`: target AArch64 ISA.
- `-mgeneral-regs-only`: exclude FPU and SIMD registers; the seL4 kernel does not
  save FPU state by default, and protection domains that need floating-point must
  opt in explicitly.

The output is a statically linked ELF executable targeting the AArch64 architecture.
As of Phase 1C.a (moonshot-toolkit v0.2.0), this step executes and is verified: the
command `moonshot-toolkit build examples/hello-world.toml` produces `build/hello.elf`,
confirmed as an AArch64 bare-metal static ELF with entry point 0x40010c.

**AssembleImage** packs the compiled protection-domain binaries, the seL4 kernel, and
the system specification into a single bootable image in Microkit's image format. This
step requires either the Microkit SDK image-assembly tool or a Rust-native image
assembler. As of v0.2.0, this step is planned — it returns a clear actionable error
identifying the missing dependency and the path to resolve it (Phase 1C.d).

## 5. Reproducibility and Cosignature

The plan_hash ties together the input specification and the full ordered build procedure.
A customer receiving a binary artefact can verify its provenance by:

1. Reconstructing the SystemSpec from the shipped TOML.
2. Running `moonshot-toolkit plan` to derive the BuildPlan.
3. Comparing the computed plan_hash against the value in the vendor's cosigned manifest.
4. Optionally rerunning `moonshot-toolkit build` on their own infrastructure to verify
   the binary byte-for-byte.

This chain satisfies Foundry's Doctrine claim #33 property: cryptographically auditable
access-control decisions anchored to logs the customer controls. The plan_hash is the
point at which the cosignature (Sigstore Cosign with customer-apex key per
system-substrate-doctrine.md §6.1) attaches.

## 6. Phase 1C Status

moonshot-toolkit v0.2.0, released 2026-05-27, completes Phase 1C.a: the CompilePd
step invokes the real AArch64 cross-compiler and produces a verified bare-metal ELF.
The AssembleImage step (Phase 1C.d) is planned, pending availability of the Microkit
SDK or a Rust-native image assembler. The full Phase 1C milestone — a seL4 hello-world
protection domain booting in QEMU AArch64 — is intended to complete when Phase 1C.d
and Phase 1C.c (QEMU boot, requiring the seL4 elfloader from the seL4_tools repository)
are resolved.

## See Also

- `topic-sel4-aarch64-qemu-substrate-target.md` — the seL4 kernel target that
  moonshot-toolkit's images run on
- `topic-capability-ledger-substrate.md` — Doctrine claim #33; what the built images
  enforce at runtime
- `guide-moonshot-toolkit-phase1c-build-setup.md` — operational runbook for setting
  up and using the build environment
