---
artifact: brief
name: BRIEF-LEAPFROG-2030
status: active
created: 2026-05-29
engine: claude-code
session: totebox@project-infrastructure
synthesises:
  - BRIEF-VM-ARCHITECTURE.md
  - BRIEF-PPN-ARCHITECTURE.md
  - internet research (Opus agents, 2026-05-29)
description: >
  Leapfrog 2030 resource targets for all five os-* images. Phase 2 (NetBSD/NVMM host)
  and Phase 3 (seL4/unikernel) disk and RAM targets. Engineering constraints, Rust
  binary size discipline, and competitive positioning vs cloud provider minimums.
---

# Leapfrog 2030 — os-* Resource Targets

**Mission:** Make PPN nodes so resource-efficient that any operator's spare hardware
outruns any cloud provider's minimum VM tier. Phase 3 targets are 4–10× lighter than
Lambda 128 MB, 20–40× lighter than Cloud Run 512 MiB.

---

## 1. Target Table

Phase 2 = NetBSD/NVMM host (compat bottom, 2027–2028 target).
Phase 3 = seL4/unikernel native bottom (2029–2030 target, AArch64).

| os-* | P2 disk | P2 RAM idle | P3 disk | P3 RAM idle | P3 RAM loaded | P3 OS stack |
|---|---|---|---|---|---|---|
| os-infrastructure | 120 MB | 48 MB | **8 MB** | **12 MB** | 48 MB | seL4+Microkit 2.x / NetBSD+NVMM |
| os-totebox | 80 MB | 64 MB | **16 MB** | **24 MB** | 96 MB | Nanos/OPS or seL4+Microkit |
| os-mediakit | 600 MB | 180 MB | **24 MB†** | **32 MB†** | 128 MB† | Unikraft+Rust |
| os-orchestration | 96 MB | 96 MB | **12 MB** | **18 MB** | 80 MB | Unikraft+Rust |
| os-privategit | 180 MB | 96 MB | **20 MB†** | **24 MB†** | 96 MB† | Unikraft+Rust |
| **seL4 baseline** (7 PDs, no services) | 2 MB | 3 MB | — | — | — | seL4 kernel 162 KiB |

† contingent on application-layer retirement: os-mediakit requires retiring MediaWiki/PHP
for a Rust static renderer; os-privategit requires retiring Gitea/Go for a gitoxide-based
server. These are application decisions that gate the P3 targets — not OS decisions.

---

## 2. Competitive Positioning vs 2030 Benchmarks

| Comparison | RAM floor | Our Phase 3 | Factor |
|---|---|---|---|
| AWS Lambda minimum | 128 MB | 12–24 MB | 5–10× lighter |
| GCP Cloud Run minimum | 512 MiB | 12–24 MB | 20–40× lighter |
| Kubernetes pod overhead | ~100 MB | 12–24 MB | 4–8× lighter |
| Unikraft nginx (proven) | 2–6 MB | — | our services have real state; above this floor |
| NanoVMs unikernel demos | 4–16 MB | — | precedent for sub-20 MB production services |

**Implication:** A Raspberry Pi 4 (4 GB RAM) could host 40+ PPN nodes at Phase 3 targets.
A modern consumer laptop (32 GB RAM) could host 300+.

---

## 3. Engineering Constraints That Gate Targets

### 3.1 Tokio multi-threaded runtime: ~30–40 MB floor

Tokio's multi-threaded scheduler spawns a thread per vCPU plus work-stealing queues.
On a typical host with 4–8 cores, this drives 30–40 MB RSS before any business logic runs.

**Rule:** Use `tokio::main(flavor = "current_thread")` for ALL new system-* and service-*
daemons except `service-fs`. service-fs is the only exception — it requires concurrent
write serialization. Already applied in service-vm-fleet and service-vm-host (Part B crates).

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() { ... }
```

### 3.2 rustls TLS: ~10 MB RSS per active pool

Every TLS connection pool adds ~10 MB RSS. Counted in "P3 RAM loaded" numbers above.
WireGuard eliminates the need for TLS on intra-PPN traffic — service-vm-host and
service-vm-fleet communicate over WireGuard plaintext within the mesh.

### 3.3 MediaWiki/PHP — os-mediakit P3 gate

PHP interpreter alone: ~30 MB RSS. MediaWiki on PHP 8.3: ~120–160 MB RSS at idle.
To hit the 32 MB idle target, MediaWiki/PHP must be replaced by a Rust static site
renderer. This is an application-layer decision — not gated on OS work, but it blocks
the Phase 3 disk and RAM targets for os-mediakit.

### 3.4 Gitea/Go — os-privategit P3 gate

Go runtime: ~20 MB RSS baseline. Gitea at idle: ~80–96 MB RSS.
To hit the 24 MB idle target, Gitea must be replaced by a gitoxide-based Rust server.
Again, an application decision — gated separately from os-privategit OS work.

### 3.5 seL4 x86-64 Microkit: 1 vCPU/VM limit

Microkit 2.2.0 `x86_64_generic_vtx` (pc99) exists but caps each guest at **1 vCPU**.
Multi-threaded service workloads (axum, hyper) can still run on a single vCPU but with
reduced throughput. AArch64 acquisition is required for production Phase 3 with multi-vCPU guests.

---

## 4. Rust Binary Size Discipline

Apply to ALL new system-* and service-* crates. Adds to workspace `Cargo.toml` or
per-crate `Cargo.toml` (whichever applies):

```toml
[profile.release]
opt-level = "z"          # optimize for binary size, not speed
lto = true               # link-time optimization across crates
codegen-units = 1        # single codegen unit for maximum LTO
panic = "abort"          # removes unwinding machinery (~100 KB)
strip = true             # strip debug symbols from output binary
```

**Typical result:** axum+tokio service: 4.2 MB → 2.0 MB. `no_std` minimal: 13–20 KiB.

Apply this profile at the first commit of any new crate — not retrofitted. Profile blocks
are cheap to add early; retrofitting after code is written causes spurious build churn.

---

## 5. OS Stack Choices Per Phase and os-* Type

### os-infrastructure

- **Phase 1 (now):** Ubuntu 24.04 host; QEMU/TCG (GCP) or QEMU/KVM (Laptop A); in-kernel WireGuard
- **Phase 2 (planned):** NetBSD 11.0; in-kernel `wg(4)`; NVMM hypervisor (`qemu -accel nvmm`);
  `securelevel=2` read-only kernel; VeriExec load-time integrity; MICROVM kernel config
  (NetBSD 11.0 RC4+). Disk: 120 MB. RAM idle: 48 MB.
- **Phase 3 (intended):** seL4 v15 + Microkit 2.2.0 on AArch64; 7-PD architecture:
  pd-genesis (reaped after pairing), pd-ledger, pd-wireguard (BoringTun `no_std`),
  pd-net-driver, pd-vmm (libsel4vm), pd-fleet, pd-network-admin. Disk: 8 MB. RAM idle: 12 MB.

### os-totebox

- **Phase 1:** Ubuntu 24.04; service-fs, service-people, service-email; WORM flat-file archive
- **Phase 2:** NetBSD 11.0; FreeBSD-style jail per service; VeriExec; in-kernel `wg(4)`
- **Phase 3 (intended):** Nanos/OPS unikernel or seL4+Microkit; one unikernel image per service;
  WORM archive stays on host filesystem (not inside unikernel). Disk: 16 MB. RAM idle: 24 MB.

### os-mediakit

- **Phase 1:** Ubuntu 24.04; MediaWiki/PHP + Rust services (proofreader, knowledge, marketing)
- **Phase 2:** FreeBSD + jails (one per workload); PHP runtime stays but jailed
- **Phase 3 (intended, gated on app decision):** Unikraft+Rust; requires retiring MediaWiki/PHP.
  If MediaWiki is retained, P3 target is unachievable (PHP ~30 MB baseline alone).
  Disk: 24 MB. RAM idle: 32 MB. (contingent on Rust static renderer)

### os-orchestration

- **Phase 1:** Ubuntu 24.04; app-orchestration-slm (:9180), app-orchestration-bim
- **Phase 2:** NetBSD 11.0 + NVMM; gVisor sandboxing for untrusted aggregator workloads
- **Phase 3 (intended):** Unikraft+Rust; NanoVMs for SLM/GPU inference (fat Linux VM retained
  for GPU workloads — GPU drivers exclude from unikernel path). Disk: 12 MB. RAM idle: 18 MB.

### os-privategit

- **Phase 1:** Ubuntu 24.04; Gitea; SSH forwarding
- **Phase 2:** FreeBSD + jail around Gitea; VeriExec
- **Phase 3 (intended, gated on app decision):** Unikraft+Rust; requires gitoxide-based server
  to replace Gitea/Go. If Gitea is retained, P3 target is unachievable (Go runtime ~20 MB).
  Disk: 20 MB. RAM idle: 24 MB. (contingent on gitoxide-based server)

---

## 6. seL4 Kernel Size Reference

seL4 v15 verified ELF: **162 KiB**. This is the machine-checked kernel itself.
7-PD baseline (kernel + 7 minimal protection domains): **~2 MB disk / ~3 MB RAM total**.
Each additional PD with a service binary adds ~1–4 MB depending on the Rust binary.

The os-infrastructure Phase 3 target (8 MB disk, 12 MB RAM idle) accounts for:
- seL4 kernel: 162 KiB
- 7 PD ELF images: ~3–5 MB combined (post-strip, opt-level="z")
- CPIO initrd + device tree: ~1–2 MB
- Runtime overhead (IPC, scheduling): ~1–2 MB RAM

This is tight but achievable based on the Unikraft and MirageOS precedent of 2–6 MB
production images for services with real state.

---

## 7. Dependencies and Gates

| Target | Gate | Status |
|---|---|---|
| Phase 2 NetBSD/NVMM deployment | NetBSD 11.0 stable release | RC4 available May 2026; stable expected 2026 |
| Phase 3 AArch64 seL4 | AArch64 hardware acquisition | Operator decision pending |
| os-mediakit P3 32 MB target | Rust static renderer replaces MediaWiki/PHP | Application decision — not scheduled |
| os-privategit P3 24 MB target | gitoxide-based server replaces Gitea/Go | Application decision — not scheduled |
| service-vm-fleet :9203 production | service-vm-fleet + service-vm-host crates scaffolded | IN PROGRESS (session 12) |
| bench #9 J2 JOURNAL re-run | Quiet VM, load avg < 1.0 | Blocked on GCP nested KVM enablement |

---

*Created: 2026-05-29 | project-infrastructure session 12 | claude-code (Sonnet 4.6)*
*Cross-references: BRIEF-VM-ARCHITECTURE.md §9, BRIEF-PPN-ARCHITECTURE.md §12, BRIEF-OS-FAMILY.md*
