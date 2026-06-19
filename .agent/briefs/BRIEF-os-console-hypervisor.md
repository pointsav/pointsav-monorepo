---
artifact: brief
schema: foundry-brief-v1
brief-id: project-console-os-console-hypervisor
title: "os-console Type II Hypervisor — Local Deployment Model"
status: active
owner: project-console
parent: project-console-os-console-active-dev
created: 2026-06-16
updated: 2026-06-16 (§10b hybrid TUI/GPU; §11b SSH retirement locked; T0 native-binary testing plan added to §10)
authors: [totebox@project-console, claude-sonnet-4-6]
doctrine_anchors: [claim-45, claim-49, claim-54, SYS-ADR-10, SYS-ADR-19]
---

# os-console Type II Hypervisor — Local Deployment Model

> **Why this BRIEF exists:** The current os-console deployment model requires port 2222 open
> to the internet on the GCE VM. This is the primary security liability. The fix is to invert
> the model: os-console runs locally on the operator's machine, with all Totebox services
> hosted inside a local lightweight VM. No internet-facing port. No open firewall.
>
> This BRIEF records the deep-think research on that architecture, the platform constraints,
> and the phase plan. Companion: [[project-console-os-console-active-dev]].

---

## §1 — The Security Problem

Current deployment model:

```
Operator's Mac/Linux  ──[SSH, port 2222, internet]──▶  GCE VM
                                                        └─ os-console (SSH server mode)
                                                        └─ Doorman :9080
                                                        └─ service-content :9081
                                                        └─ service-proofreader :9092
                                                        └─ service-input :9106
                                                        └─ … all other services
```

**What port 2222 exposes:**
- Any internet host can attempt authentication against the russh server
- fail2ban mitigates brute-force but does not eliminate the attack surface
- The GCE firewall rule is an operator-gated item that has been pending since Phase 9
- Even with fail2ban, an 0-day in russh or the SSH handshake code is a critical path to all
  Totebox services

**Proposed model:**

```
Operator's Mac/Linux  (host)
├─ os-console binary  ────────────────────▶  local TUI (no SSH server feature)
│   └─ ConsoleConfig endpoints:             └─ slm_endpoint: 192.168.64.2:9080
│                                           └─ content_endpoint: 192.168.64.2:9081
│                                           └─ proof_endpoint: 192.168.64.2:9092
│                                           └─ ingest_endpoint: 192.168.64.2:9106
│
└─ embedded VMM  ──────────────────────────▶  guest Linux VM
                                              └─ Doorman :9080
                                              └─ service-content :9081
                                              └─ service-proofreader :9092
                                              └─ service-input :9106
                                              └─ … all Totebox services
```

Zero internet-facing ports. All service communication is host-only on a private vnet.
No russh exposure. No fail2ban needed. The GCE firewall operator item is fully retired.

---

## §2 — Architecture

### Two deployment modes (coexist as compile features)

| Feature flag | Mode | Who uses it | Port exposure |
|---|---|---|---|
| `--features ssh-server` | Remote SSH server | GCE VM operators (sunset — no new features; removed at H4) | port 2222 open |
| default / `--features local-vm` | Local TUI + embedded VMM | Mac/Linux desktop operator (**primary**) | none |
| default (no feature) | Local TUI, no VM | Dev / services running natively | none |

The `local-vm` feature activates the VMM bootstrap code in `os-console/src/main.rs`.
On startup with `local-vm`: boot the guest VM first, wait for services to healthcheck, then
start the TUI event loop. Endpoint config auto-set to the host-only network address.

### What runs in the guest VM

Guest is a minimal Linux (Alpine) with static musl Rust binaries. All services are
the same binaries as the GCE VM deployment — no separate guest-specific build:

| Service | Guest port | os-console cartridge |
|---|---|---|
| `slm-doorman-server` (Doorman) | 9080 | F9 SlmCartridge |
| `service-content` | 9081 | F4 ContentCartridge |
| `service-proofreader` | 9092 | F4 ContentCartridge (proof_endpoint) |
| `service-people` | 9091 | F2 PeopleCartridge |
| `service-input` | 9106 | F12 InputCartridge |
| `service-email` | 9093 | F3 EmailCartridge |
| `pairing-server` | 9201 | F11 SystemCartridge |

Services NOT in the guest (remain on the GCE/host side for shared infrastructure):
- `app-mediakit-knowledge` (wiki service) — multi-tenant, stays on GCE
- `app-privategit-source` / `app-privategit-marketplace` — internet-facing, stays on GCE

---

## §3 — Platform Strategy

### macOS — floor: High Sierra 10.13 Intel; Monterey 11.0 Apple Silicon

**Apple Hypervisor.framework** is the correct host-side VMM layer.

| Fact | Detail |
|---|---|
| Available since | macOS 10.10 (Yosemite) |
| Our floor (Intel) | 10.13 (High Sierra) — existing MACOSX_DEPLOYMENT_TARGET |
| Apple Silicon floor | 11.0 (Big Sur) — existing MACOSX_DEPLOYMENT_TARGET for ARM |
| Entitlement | `com.apple.security.hypervisor` (only needed for sandboxed apps) |
| Root required | No — user-space only |
| API | C API (`hv_vm_create`, `hv_vcpu_create`, etc.) via FFI from Rust |
| Rust bindings | `hypervisor` crate (0.1.x, MIT) or `hvf` (0.4.x) |
| Guest arch on Intel | x86_64 Linux guest |
| Guest arch on Apple Silicon | aarch64 Linux guest |

**What Hypervisor.framework does NOT provide** (we must implement):
- VirtIO device emulation (virtio-blk, virtio-net, virtio-vsock)
- BIOS/UEFI boot shim (use a Linux direct-boot path with vmlinuz + initrd — no BIOS needed)
- MMIO/PCI bus emulation (minimal stubs sufficient for VirtIO-MMIO transport)

**Virtualization.framework** (macOS 11+) is higher-level and would be easier,
but it would break the 10.13 floor requirement. Decision locked: use Hypervisor.framework.

### Linux — floor: kernel 3.10+

**KVM via `/dev/kvm`** is the host-side VMM layer.

| Fact | Detail |
|---|---|
| KVM in kernel since | 2.6.20 (2007) |
| Practical floor | 3.10+ (RHEL 7 / Ubuntu 14.04, 2014) — udev stable, cgroups v1 |
| CPU requirement | Intel VT-x or AMD-V (hardware virtualization) — available on all x86_64 since ~2008 |
| Permission | User must be in `kvm` group (default on most distros with `/dev/kvm` present) |
| Fallback (no KVM) | QEMU TCG (software emulation) — 5–10× slower but functional |
| Rust bindings | `kvm-ioctls` (Firecracker lineage, Apache 2.0) + `vm-memory` |

**ARM Linux hosts (future):** KVM/ARM64 available since 4.7, mature since 5.4. Not in scope
for Phase H1 — focus is Intel Mac and x86_64 Linux first.

### Dependency audit

| Dep | macOS 10.13 safe? | Linux x86_64 safe? | Notes |
|---|---|---|---|
| `hypervisor` / `hvf` crate | ✅ | ❌ macOS-only | `#[cfg(target_os = "macos")]` |
| `kvm-ioctls` | ❌ Linux-only | ✅ | `#[cfg(target_os = "linux")]` |
| `vm-memory` | ✅ | ✅ | Cross-platform anonymous mmap |
| `linux-loader` | ✅ (cross-compile) | ✅ | Reads vmlinuz; no OS dep |
| `virtio-queue` | ✅ | ✅ | Pure Rust VirtIO ring buffer |

The `moonshot-hypervisor` crate will `cfg`-gate the platform-specific paths behind
`#[cfg(target_os)]`. One crate, two backends.

---

## §4 — VMM Options and Recommendation

Three options evaluated:

### Option A — System QEMU (proof-of-concept phase only)

Exec a QEMU binary from `$PATH` with appropriate flags. Simplest path to validate the
guest image and service stack. No VMM code required.

```
qemu-system-x86_64 \
  -kernel vmlinuz -initrd initrd.img \
  -append "console=ttyS0 quiet" \
  -netdev user,id=net0,hostfwd=tcp::9080-:9080,...  \
  -device virtio-net-pci,netdev=net0 \
  -nographic -m 512M
```

**Verdict:** Only for Phase H0 validation. Not shippable (requires QEMU installed, user-mode
networking is slow, no macOS Hypervisor.framework acceleration).

### Option B — Firecracker (Linux only)

Firecracker is a production-grade Rust VMM using KVM. Minimal (no BIOS, no PCI, VirtIO-MMIO
only). Sub-125ms boot times. Used in AWS Lambda.

**Pros:** Production-proven, Rust, Apache 2.0, very fast.
**Cons:** Linux only (no macOS support), requires shipping a separate Firecracker binary or
linking its libraries, API is via a Unix socket HTTP endpoint.

**Verdict:** Use for Linux path in Phase H2+. Not the macOS path.

### Option C — moonshot-hypervisor custom VMM (recommended long-term)

Build a minimal VMM directly in the `moonshot-hypervisor` crate using:
- macOS: `Hypervisor.framework` bindings (`hvf` crate)
- Linux: `kvm-ioctls` + `vm-memory`
- Both: `linux-loader` (ELF/bzImage loader), `virtio-queue` (VirtIO rings)

Devices needed (minimal set for Totebox services):
1. **virtio-blk** (read-only root disk + writable data partition)
2. **virtio-net** (TAP-based host-only networking)
3. **virtio-console** (serial console for guest log capture)
4. **virtio-vsock** (optional; more efficient than TCP for host-guest IPC)

**Verdict:** Phase H2+ on macOS, Phase H3+ cross-platform unified. The moonshot-hypervisor
README already states the seL4 unikernel alignment — this VMM is the substrate for that too.

### Phased recommendation

| Phase | What | Platform | When |
|---|---|---|---|
| H0 | Validate guest image with system QEMU | Linux | Unblocked now |
| H1 | Firecracker + Alpine guest; TAP networking | Linux | After Stage 6 |
| H2 | Hypervisor.framework VMM in moonshot-hypervisor | macOS Intel | After H1 proven |
| H3 | Unified VMM crate; ARM64 guest; vsock IPC | Both | After H2 |
| H4 | os-console binary embeds VMM; single-binary ship | Both | After H3 |

---

## §5 — Guest Image Design

**Base OS:** Alpine Linux (musl-based, matches our Rust binary target, ~5MB base)

**Kernel:** Linux LTS (6.1 or 6.6 recommended). Statically compiled with:
- VirtIO block, net, console, vsock drivers built-in (not modules)
- KVM guest support (`CONFIG_KVM_GUEST=y`, `CONFIG_PARAVIRT=y`)
- No GUI, no display, no USB

**Guest init:** s6-init (supervised process manager, <100KB). Starts each Totebox
service in order: Doorman first (other services depend on it), then remaining services.
Health probe: Doorman `/readyz` returns 200 before os-console TUI starts.

**Disk layout:**
```
/dev/vda  (read-only ext4, ~80MB)
  /bin/        — static musl Rust binaries
  /etc/        — service configs with defaults
  /lib/        — minimal dynamic libs (Alpine musl, openssl)

/dev/vdb  (writable ext4, ~256MB, operator data)
  /data/doorman/      — DataGraph
  /data/content/      — Tantivy index
  /data/proofreader/  — proofreader state
  /data/input/        — ingest audit log (SYS-ADR-10)
  /data/email/        — email state
```

**Boot sequence target:** vmlinuz + initrd → s6-init → all services healthy: **< 3 seconds**
(Firecracker on KVM achieves ~125ms kernel boot; Alpine userspace adds ~500ms; service
startup adds ~1-2s. Total: well under 3s.)

**Image delivery:** bundled alongside the os-console binary in the release artifact.
GitHub Actions CI builds the guest image using Docker multi-stage build (host-toolchain
cross-compiles Rust services for musl target, then packs into Alpine rootfs).

**Image size target:** < 120MB compressed (vmlinuz ~10MB, rootfs ~80MB, data volume separate)

---

## §6 — Host-Guest Networking

### Option A — TAP + host-only bridge (recommended for Phase H1/H2)

```
host: os-console binary
  └─ ConsoleConfig.slm_endpoint = "http://192.168.64.2:9080"

host network:
  └─ tap0: 192.168.64.1/24 (host side of TAP device)
  └─ virtio-net in guest: 192.168.64.2/24

guest: all services bind 0.0.0.0:* (reachable via 192.168.64.2)
```

Setup: `ip tuntap add tap0 mode tap && ip addr add 192.168.64.1/24 dev tap0 && ip link set tap0 up`
This requires root on Linux to create the TAP device. Mitigation: use `TUNSETOWNER`
ioctl to assign the TAP to the operator uid after creation, then drop root.

macOS: `vmnet.framework` provides a user-space network bridge without root since
macOS 10.10. The `hypervisor` crate has vmnet bindings.

### Option B — VirtIO-vsock (recommended for Phase H3+)

virtio-vsock (AF_VSOCK) provides a bidirectional socket channel between host and guest
without going through TCP/IP. Zero port conflicts. Low overhead.

```
guest CID: 3 (assigned at VM creation)
host connects: connect(AF_VSOCK, cid=3, port=9080)
```

os-console endpoints would use a thin vsock-to-TCP proxy on the host side. The
`ConsoleConfig` would remain TCP-URL based; the proxy handles the translation.

Requires: `vhost-vsock` kernel module (Linux); Hypervisor.framework vsock support (macOS 11+).
macOS 10.13 does NOT have vsock support via Hypervisor.framework — TAP is the fallback there.

**Decision for current phase:** TAP + vmnet. vsock upgrade in H3.

---

## §7 — os-console Integration

### New Cargo feature: `local-vm`

```toml
# os-console/Cargo.toml
[features]
ssh-server = ["russh", "russh-keys"]     # existing — GCE deployment
local-vm   = ["moonshot-hypervisor"]     # new — desktop deployment
```

`moonshot-hypervisor` becomes a conditional dep. Compiles out entirely on the `ssh-server`
path. Does not bloat the GCE binary.

### Startup flow with `local-vm`

```rust
// os-console/src/main.rs (local-vm path)
#[cfg(feature = "local-vm")]
fn main() {
    let vm_config = VmConfig::from_bundled_image();   // reads embedded guest image path
    let vm = moonshot_hypervisor::VmHandle::start(vm_config)?;
    vm.wait_for_health("http://192.168.64.2:9080/readyz", Duration::from_secs(10))?;
    let config = ConsoleConfig::for_local_vm(vm.guest_ip());  // overrides all endpoints
    run_local_tui(config);                             // existing local TUI path
    vm.shutdown();
}
```

### ConsoleConfig auto-fill for local-vm mode

`ConsoleConfig::for_local_vm(ip: Ipv4Addr)` returns a config with all endpoints
set to `http://{ip}:{port}`. Operator's `~/.config/os-console/config.toml` is still
read but its endpoint fields are overridden by the VM-assigned address. This prevents
endpoint misconfiguration when the guest network address changes.

### Binary distribution (Phase H4)

Single archive per platform (replaces current 4-artifact release matrix):

| Artifact | Contains |
|---|---|
| `os-console-{VER}-x86_64-unknown-linux-musl.tar.gz` | binary + vmlinuz + initrd.img |
| `os-console-{VER}-x86_64-apple-darwin.tar.gz` | binary + vmlinuz + initrd.img |
| `os-console-{VER}-aarch64-apple-darwin.tar.gz` | binary + aarch64 vmlinuz + initrd.img |
| `os-console-{VER}-universal-apple-darwin.tar.gz` | lipo binary + both guest images |

---

## §8 — Proofreader (F4) and Input Machine (F12) on the Hypervisor Model

### Current state (pre-hypervisor)

**F4 ContentCartridge (Proofreader workflow):**
- Connects to `content_endpoint` (service-content :9081) and `slm_endpoint` (Doorman :9080)
- `proof_endpoint` field exists in `ConsoleConfig` (service-proofreader :9092) — currently
  whether ContentCartridge actually calls this endpoint needs verification in
  `app-console-content/src/`. Noted as open question §11.

**F12 InputCartridge (Input Machine):**
- Connects to `ingest_endpoint` (service-input :9106)
- Port was `:9100` until `a17cfdb0` fixed it. The fix is in the committed code but
  Stage 6 has not yet pushed the binary.
- Audit log (`/audit` view, Ctrl-A) is local to the host (reads from `~/.local/share/os-console/ingest-audit.jsonl`)
- SYS-ADR-10 compliance: all file ingests go through this F12 gate — mandatory in all deployments

### What the hypervisor model changes for these two cartridges

**Nothing in the cartridge code changes.** The Cartridge trait is endpoint-agnostic.
The host-side code calls the same HTTP endpoints; only the IP address changes from
`127.0.0.1` (native) to `192.168.64.2` (VM guest IP).

**The guest VM must include:**
- `service-proofreader` binary (port 9092) — confirmed deployed on GCE VM; needs packaging
- `service-input` binary (port 9106) — confirmed deployed; port fix `a17cfdb0` must be in binary
- Both must start and healthcheck before the TUI opens

**Audit log placement:** The InputCartridge writes its local audit log to the host filesystem
(not inside the VM). This is correct per SYS-ADR-10 — the audit record lives with the
operator, not inside the ephemeral VM. No change needed.

### Getting Proofreader + Input Machine working TODAY (Phase T0 — pre-hypervisor)

**Phase T0 (2026-06-16):** run os-console as a native local binary on Mac Pro (macOS 10.13
Intel) and iMac (Linux Mint x86_64), connecting to GCE VM services via SSH port forwarding.
This does NOT require Stage 6 to land first.

T0 model: `origin/main` build (F4 + F12 present; InputCartridge `:9100` matches GCE VM
service-input on `:9100`) → build locally on each host → SSH tunnel auto-started by launcher
script → double-click desktop launcher to open the console.

Setup artifacts in `os-console/scripts/` in the monorepo sub-clone:
- `install.sh` — one-shot setup (Rust install, build, config, launcher, desktop entry)
- `launch-console.sh` — Linux Mint launcher (starts tunnel + os-console + kills tunnel)
- `launch-console.command` — macOS launcher (same; `.command` = Terminal double-click)
- `config.toml.example` — config template

After T0 is verified on both machines, Stage 6 unblocks the Phase 10 rebuild (F2
PeopleCartridge, InputCartridge `:9106`, reconnect watchdog). Rebuild = `git pull + cargo
build` on each host. The hypervisor phases (H0+) follow after T0 is stable.

---

## §9 — moonshot-hypervisor as VMM Home

Current state: pure scaffold (`pub fn system_status() -> &'static str`).
README correctly identifies the seL4 unikernel alignment — the longer-term direction
is a sovereign VMM that boots seL4 microkernel images. The Totebox-hosting use case is
the near-term motivation that makes this crate Active rather than Reserved-folder.

**Activation path for moonshot-hypervisor:**

Phase H1 (Linux Firecracker path): moonshot-hypervisor wraps Firecracker API calls
(Unix socket HTTP to Firecracker process). Crate provides `VmHandle` with `start()`,
`wait_for_health()`, `shutdown()`. Implementation: ~300 LOC.

Phase H2 (macOS Hypervisor.framework): same `VmHandle` trait, macOS backend uses
`hvf` crate bindings. VirtIO-MMIO device implementations added for blk + net.

Phase H3 (unified cross-platform): single `moonshot_hypervisor::VmHandle::start(config)`
dispatches to the correct backend. os-console just calls the API.

**crate name in Cargo.toml:** `moonshot-hypervisor` (existing). Register as workspace
member when Phase H1 begins.

---

## §10 — Phase Plan

| Phase | Gate | What gets built | Blocker |
|---|---|---|---|
| **T0** | *(current — unblocked)* | Native binary on Mac Pro 10.13 + iMac Linux Mint. Build locally; SSH tunnel to GCE VM services; double-click launcher. F4 Proofreader + F12 Input Machine verified. | None — build from `origin/main` now |
| **H0** | T0 complete + Stage 6 land | Validate guest: boot Alpine + all 7 services in QEMU on the workspace VM. No os-console integration. Just prove the service stack works inside a VM. | T0 verified + Stage 6 (Command) |
| **H1** | H0 passes healthcheck | Firecracker VMM wrapper in moonshot-hypervisor (Linux). `os-console --features local-vm` boots Firecracker, waits for Doorman health, opens TUI. TAP networking. | H0 |
| **H2** | H1 works on Linux | Hypervisor.framework backend in moonshot-hypervisor (macOS Intel 10.13+). Same VmHandle API. vmnet networking. Intel Mac operators can run local-vm. | H1 |
| **H3** | H2 works on macOS | ARM64 guest image. Apple Silicon support (Hypervisor.framework aarch64). vsock upgrade for H3+ (where supported). | H2 |
| **H4** | H3 passes E2E | Bundled binary release: single tar.gz per platform with guest image. GitHub Actions CI builds guest image as part of release workflow. **Remove `--features ssh-server` and russh dependency entirely.** | H3 (SSH retirement decision locked 2026-06-16) |

---

## §11 — Decisions Locked

- Platform floor for macOS: 10.13 Intel, 11.0 ARM — matches existing MACOSX_DEPLOYMENT_TARGET
- Platform floor for Linux: kernel 3.10+ (KVM module required; TCG fallback for CI)
- Guest OS: Alpine Linux (musl; matches our static binary target)
- Guest init: s6 (not systemd — too heavy for a 2s boot target)
- Networking Phase H1/H2: TAP (Linux) + vmnet.framework (macOS) — no root needed on macOS
- `local-vm` as a Cargo feature — does not bloat the ssh-server GCE binary
- Hypervisor.framework (not Virtualization.framework) — preserves 10.13 floor
- InputCartridge audit log stays on host filesystem — SYS-ADR-10 compliance, not inside VM
- **SSH server mode retirement locked (2026-06-16):** `--features ssh-server` is retired
  as a development target. No new features will be developed for it. The binary remains
  buildable for existing GCE VM operators until H4 ships; at H4 the russh dependency
  is removed entirely. See §11b for the full evidence and analysis.
- 2026-06-19: seL4 Microkit unikernel confirmed as Phase H2 target substrate — formally
  verified BSD-2-Clause kernel; moonshot-toolkit v0.3.1 already produces bootable AArch64
  seL4 images (35 tests passing; Phase 1C complete).
- 2026-06-19: moonshot-sel4-vmm is the sovereign PD runtime (~300 lines Rust to fill);
  rust-sel4 external bindings NOT used; we write the seL4 ABI wrappers ourselves.
- 2026-06-19: **Geometric Protection™** defined as PointSav term — seL4 capability DAG
  authorization; not layered ACL security; `system-core::Capability` + `system-ledger::Verdict`
  are the Rust substrate. See §12b for the full definition.
- 2026-06-19: VirtIO clipboard is Phase H1 non-optional deliverable in moonshot-hypervisor —
  arboard host-side bridge; VirtIO clipboard protocol guest-side; required for SMB operator
  UX (paste from host apps into cartridges).
- 2026-06-19: system-ledger::InMemoryLedger is the F12 SYS-ADR-10 WORM audit backend
  — replaces any custom log format planning; WORM + RFC 9162 Merkle proof chain is the
  correct implementation for SYS-ADR-10.
- 2026-06-19: os-console = "browser for Totebox Orchestration" — official design metaphor;
  cartridges = browser tabs (seL4 PDs at H2); F11 machine pairing = certificate store.
- 2026-06-19: We Own It principle adopted — hermit-os REJECTED (external architecture in
  permanent runtime path); nanos REJECTED (commercial license, not sovereign); see
  conventions/we-own-it-principle.md (pending ratification at Command).
- 2026-06-19: Three-binary architecture confirmed — os-console (host/Type II), os-totebox
  (bare metal/Type I), os-orchestration (federation hub); all three use same seL4 substrate,
  moonshot-sel4-vmm PD runtime, and system-core/system-ledger capability substrate.

---

## §11b — SSH Mode Retirement: Evidence and Rationale

> **Decision date:** 2026-06-16. **Decision maker:** operator (jwoodfine session).
> **Evidence base:** workspace-wide BRIEF/TOPIC/GUIDE sweep, same session.

### Why SSH mode was built

SSH server mode (`--features ssh-server`, russh crate, port 2222) was the transport
mechanism for the GCE VM hosted prototype phase. It gave the operator a TUI experience
over an SSH session before local-native binaries existed. It was never the architectural
goal — it was the scaffolding that got os-console to a working state.

### What the workspace-wide documents say

**BRIEF-tui-desktop-architecture.md** (written 2026-05-25 by 10 competing Opus agents;
now archived and synthesised into BRIEF-workplace-desktop-environment.md) is the
authoritative workspace-level classification of TUI Desktop deployment models. §12.1
explicitly defines two models:

> **Model A — Native Packaged App (primary; office-worker-first)**
> Bundle the binary + emulator into a single installable artifact. The user double-clicks
> `PointSav Bookkeeper.app` — they never see a terminal emulator.

> **Model B — Remote Session via WireGuard PPN (secondary; server-operator model)**
> Linux server runs the full stack. Office workers connect via tunnel; client needs only
> a terminal emulator to render sequences over the tunnel.

The SSH server mode is literally Model B — a remote session accessed via a terminal
emulator over a network connection. The document classified it as **secondary** before
this decision was explicitly made in this BRIEF.

**topic-os-console-platform.md** (written 2026-05-20, project-proofreader, draft-ready):
> "An *optional* SSH server mode (compiled with `--features ssh-server`) enables remote
> access over port 2222 for use on a GCE VM."

The word "optional" was chosen deliberately. The document's primary description covers
the local native binary on Linux Mint and macOS.

**guide-os-console-operator.md** (written 2026-05-20, project-proofreader):
Primary instructions cover `os-console` and `os-console --profile offline`. The GCE VM
SSH mode is `--profile gce-native` — a named secondary profile, not the default.

**BRIEF-os-console-platform.md** (archived plan, 2026-05-20):
Platform targets table lists "Linux Mint (primary; local crossterm PTY)" and
"macOS 10.13+" as primary. The GCE VM row (ssh-server feature) appears as a secondary
row alongside Linux static binary.

### The operator's framing (2026-06-16)

> "I'm not sure we need the SSH Mode... we can move right to os-console being the
> binary which gets installed on the host to access either os-totebox or os-orchestration
> ... the SSH version just got us to where we are now."

This is exactly right. SSH mode was the transport for the prototype phase. The target
architecture is the native binary (local-vm or no VM for dev). SSH was the scaffolding,
not the building.

### What this means for the codebase

| Item | Action | When |
|---|---|---|
| `--features ssh-server` | No new features; maintenance-only | Immediately |
| russh + russh-keys deps | Retained but dormant | Until H4 |
| russh + russh-keys deps | Removed from Cargo.toml | H4 |
| `#[cfg(feature = "ssh-server")]` blocks | Removed from src/ | H4 |
| GCE VM operators | Can still build ssh-server binary | Until H4 |
| Port 2222 firewall rule | Can be removed from GCE config | Now (after Command resolves Stage 6) |
| fail2ban for port 2222 | Can be removed | After port rule removed |

### What SSH mode was blocking that local-vm solves

SSH mode forced a pure CrosstermBackend forever — no clipboard, no native window,
no GPU renderer. With local-vm as the primary model, §10b (hybrid TUI/GPU, arboard
clipboard) becomes the default user experience. SSH mode is not compatible with §10b
by design — it stays on CrosstermBackend for its remaining lifespan.

---

## §12 — Decisions Open

- [ ] Does `app-console-content` currently call `proof_endpoint` (service-proofreader :9092)?
  Check `app-console-content/src/` before Phase H0 guest image spec is finalized. If not
  called, service-proofreader may be omitted from Phase H0 guest.
- [ ] Should the `pairing-server` (9201) be inside the guest VM or outside?
  Inside: operator can pair without GCE VM. Outside: pairing ceremony needs a separate host
  binary. Recommendation: inside for Phase H1 simplicity; revisit at H3.
- [ ] Data volume persistence: when the operator upgrades the guest image, how does
  `/dev/vdb` (operator data) survive? Need a migration path before H4 ships.
- [ ] Operator-gated GCE items (port 2222, fail2ban) — can begin decommissioning immediately
  after Stage 6 lands the current binary. Route to Command Session outbox.

---

## §10b — Hybrid TUI/GUI Model (local-vm mode only)

### The problem with pure TUI for business administration

The SSH deployment mode is a pure TUI by necessity — the terminal emulator owns the PTY
and the clipboard. The TUI application has no reliable cross-platform path to the system
clipboard. OSC 52 (the standard escape for clipboard access) works in kitty, iTerm2,
and Windows Terminal but is unreliable or read-only in macOS Terminal.app and most older
Linux terminal emulators.

For business administration workflows — copying a document ID, pasting a file path,
copying an email address from PeopleCartridge, pasting a quote into InputCartridge —
this is a daily friction point that cannot be solved within the pure TUI model.

**The local-vm deployment model removes the constraint entirely.** When os-console runs
on the operator's own machine (not over SSH), native OS APIs are accessible. A hybrid
render model is the correct answer.

### Architecture: ratatui with a GPU backend

`ratatui` is backend-agnostic. Cartridges call `frame.render_widget(...)` — they have
no knowledge of what renders the output. The existing `CrosstermBackend` writes ANSI
escape sequences to a PTY. A `GpuBackend` would instead write cells to a native GPU
surface. **Zero cartridge code changes required.**

```
SSH mode (unchanged, forever valid):
  ratatui → CrosstermBackend → PTY → SSH → operator terminal emulator
  Clipboard: terminal emulator selection + OSC 52 where supported

local-vm mode (new):
  ratatui → GpuBackend → native window (wgpu / softbuffer)
  Clipboard: native OS clipboard API (Cmd+C/V on macOS, Ctrl+C/V on Linux)
  Speed: GPU cell renderer, diff-only redraws, ~1ms frame times
```

The two modes are compile-time features and run-time dispatch — one binary, two render paths.

### What "GPU backend" means in practice

ratatui's `Backend` trait requires four methods: `draw()`, `hide_cursor()`, `show_cursor()`,
`get_cursor()`, `clear()`, `size()`. A `GpuBackend` implementation:

1. Maintains a cell buffer matching the window's character grid
2. On `draw()`: receives `Vec<(u16, u16, Cell)>` — only changed cells (ratatui diffs for us)
3. Maps each cell to a glyph in a font atlas texture
4. Issues one instanced draw call to the GPU per frame
5. `wgpu` (cross-platform, Vulkan/Metal/DX12) or `softbuffer` (CPU fallback, no GPU required)

Reference implementations in the Rust ecosystem: Ghostty, Wezterm, Alacritty, rio all do
exactly this. The difference is they also implement a full VT100 parser — we don't need
that (ratatui gives us cells directly).

**Estimated implementation size:** ~800–1200 LOC for a minimal GpuBackend. The font
rasterization is the heaviest part; `cosmic-text` (used by COSMIC desktop) or `ab_glyph`
(simpler) are suitable.

### Native clipboard integration

macOS: `NSPasteboard` via `objc2` crate or the `copypasta` crate (cross-platform clipboard)
Linux (X11): `arboard` crate (handles X11 + Wayland clipboard)
Linux (Wayland): `wl-clipboard` via `arboard`

The `arboard` crate (MIT) handles macOS + Linux + Windows in one API. This is the
recommended dependency. Paste detection uses `arboard::Clipboard::get_text()` called on
a key event (Cmd+V / Ctrl+V intercepted before crossterm sees it in local mode).

Each cartridge that handles text input gets a `clipboard: Option<Arc<Mutex<Clipboard>>>`
field, populated only in local-vm mode. In SSH mode, `None` — the clipboard field is
absent and the existing behaviour is unchanged.

### Speed: what "unbelievably fast" means in numbers

| Render path | Frame time | Startup | Notes |
|---|---|---|---|
| ratatui + CrosstermBackend (current) | ~2–8ms | ~50ms | Depends on terminal emulator |
| ratatui + GpuBackend (wgpu) | ~0.5–1ms | ~80ms | GPU amortises across frames |
| ratatui + softbuffer (CPU) | ~3–5ms | ~60ms | No GPU required, portable |
| Ghostty / Wezterm reference | ~0.3–0.8ms | ~40ms | Full VT100 stack on top |

For os-console, the cartridges are the bottleneck — not the renderer. The F4 ContentCartridge
does HTTP calls; the F9 SlmCartridge polls every 10s. Render latency is already not the
bottleneck. But a GPU backend eliminates any renderer jank and makes keyboard response
feel instant even on a loaded VM guest.

**Startup time budget (local-vm mode):**
- VMM + guest boot: ~3s (Firecracker target)
- os-console binary launch + font atlas init: ~80ms
- Doorman healthcheck wait: included in the 3s VM boot
- Total operator wait: ~3–4s from launch to interactive TUI

This is the target: operator double-clicks the app, 3–4 seconds, fully interactive.

### Design contract additions for local-vm / GPU mode

Extends the existing UX contract (BRIEF-project-console-master §6):

**4. Clipboard contract (local-vm mode only)**
- Cmd+C (macOS) / Ctrl+C (Linux): copy current selection or focused text field to clipboard
- Cmd+V (macOS) / Ctrl+V (Linux): paste into focused text input field
- Cartridges that contain read-only content (F9 SLM, F11 System) expose a `Y` key to
  "yank" the focused item to clipboard — consistent with the existing vi-key vocabulary
- In SSH mode: no change — terminal emulator handles clipboard; cartridges do not intercept
  Cmd/Ctrl+C (these are not reliably deliverable over SSH anyway)

**5. Window contract (local-vm mode only)**
- Native window title: `os-console — {cartridge title} — {archive name}`
- Minimum window size: 80×24 (same as existing terminal assumption)
- No menu bar, no toolbar — keyboard-native design is preserved in the native window
- Font: operator-configurable in `config.toml` via `font_family` + `font_size` fields
  (new fields; defaults to a monospace system font)
- The window is NOT resizable below 80×24 (ratatui minimum)

### What does NOT change in hybrid mode

- All cartridge `Cartridge` trait implementations — zero changes
- All `handle_event()` key bindings — F-keys, j/k/Esc/Enter/Tab — unchanged
- `set_graphics_caps()` — GPU backend reports `kitty: false, sixel: false, truecolor: true`
  (no Kitty protocol in GPU mode — images render via the GPU path directly instead)
- `--plain` mode — still works; GPU backend renders plain ASCII cells at full speed
- SSH deployment mode — CrosstermBackend path is untouched; GCE VM operators unaffected
- SYS-ADR-10 (F12 mandatory) — unchanged; InputCartridge is required in all modes

### Phase plan addition

| Phase | What | Dependency |
|---|---|---|
| H-GUI-0 | `arboard` clipboard in InputCartridge and PeopleCartridge (local mode only) | Phase H1 (local-vm Cargo feature exists) |
| H-GUI-1 | `softbuffer` GpuBackend — CPU rendering, native window, no wgpu dep | Phase H1 |
| H-GUI-2 | `wgpu` GpuBackend — GPU rendering path; softbuffer kept as fallback | Phase H2 |
| H-GUI-3 | Font configurability (`font_family`, `font_size` in config.toml) | Phase H-GUI-2 |

H-GUI-0 and H-GUI-1 can run in parallel with H1 (Firecracker) — they are independent tracks.
H-GUI-0 in particular (clipboard only, no GPU renderer) is a small self-contained task (~200 LOC).

---

## §12b — Radical Substrate: Geometric Protection, seL4 Unikernel, Browser Frame

> **Date added:** 2026-06-19. Research session: can-we-make-a-bubbly-quasar (6 parallel agents).
> This section documents the architectural synthesis that supersedes vague "seL4 longer-term
> direction" notes in §9. Decisions locked in §11 (2026-06-19 entries).

### The Browser Frame

os-console is the browser for Totebox Orchestration.

| Web browser | os-console |
|---|---|
| Renders HTML from web servers | Renders TUI cartridge views from Totebox services |
| Browser tabs — isolated renderer processes | F-key cartridges — seL4 Protection Domains (Phase H2) |
| Certificate store — trusted server identities | Machine pairing (F11) — host machine as trust anchor |
| HTTP + DNS — universal protocol | Totebox service protocol — cartridge-to-service contract |
| Same-origin policy | seL4 capability boundary between cartridge PDs |
| Bootable browser runtime | os-console as bootable seL4 unikernel image (Phase H2) |

The analogy is structural, not metaphorical. Cartridges and browser tabs solve the same
problem (isolated, protocol-driven views of remote data) with the same solution
(process-level sandboxing with a well-defined communication protocol).

**Key distinction from a browser:** os-console connects to hardware under the operator's
physical control. The Totebox is not a cloud service. No data leaves the operator's premises.

### Geometric Protection™

Geometric Protection™ is a PointSav term for the seL4 capability model applied to Totebox
authorization. It is not a product feature name. It describes a structurally distinct security
model that changes the shape of access control rather than adding layers to an existing model.

The industry default response to security threats is to add layers:
Firewall → WAF → IAM → VPN → TLS → 2FA → SIEM → EDR → CASB. Each layer is a new attack
surface. An adversary who learns the layer can bypass it. The underlying access model
(authenticate, then access) does not change. The geometry stays the same.

Geometric Protection changes the geometry:

1. The set of Totebox resources reachable from a host machine forms a **provably bounded DAG**.
2. Every edge is a `system-core::Capability` token — a kernel object, not a string or flag.
   It cannot be forged, guessed, or escalated into without being explicitly granted.
3. `system-ledger::LedgerConsumer::consult_capability()` is the verdict function.
   Return type: `Verdict::Allow | Refuse | ExtendThenAllow`. Mathematical guarantee.
4. Machine pairing (F11) is the capability minting ceremony — the Totebox authority grants
   `CapabilityType::Endpoint` tokens to the os-console VM instance at pairing time.
5. Revocation calls `apply_revocation()` on the ledger. seL4 formal proofs guarantee
   propagation is complete and instantaneous.

An adversary who fully compromises the F9 SLM cartridge PD can only reach what that PD's
capability set allows. They cannot escalate to F2 People (different PD = different capability
namespace). This is not a firewall. It is a proven mathematical bound.

### "We Own It" Tier Table

| Component | Tier | Notes |
|---|---|---|
| os-console cartridge code | Tier 1 — Ours | Active; all app-console-* crates |
| moonshot-toolkit v0.3.1 | Tier 1 — Ours | Active; 35 tests; Phase 1C complete |
| moonshot-hypervisor | Tier 1 — Ours (H2 fill-in) | Scaffold → H2 replaces QEMU on host |
| moonshot-sel4-vmm | Tier 1 — Ours (H1 fill-in) | Scaffold → ~300 lines = sovereign PD runtime |
| system-core, system-ledger v1.0.0 | Tier 1 — Ours | Active; capability + WORM audit substrate |
| vendor-sel4-kernel v15.0.0-dev | Tier 2 — Vendored trusted | BSD-2-Clause; formally verified; we build |
| vendor-sel4-tools | Tier 2 — Vendored trusted | BSD-2-Clause; elfloader; 44 C/ASM sources |
| smoltcp | Tier 2 — Vendored auditable | MIT; network-pd replacing reqwest; no tokio |
| QEMU | Tier 4 — Tooling only | Dev/PoC; exits the stack at Phase H2 |
| hermit-os | REJECTED | External mini-OS architecture; permanently in stack if adopted |
| rust-sel4 | REJECTED | External bindings; moonshot-sel4-vmm writes the same ~300 lines; we own it |
| nanos / Unikraft | REJECTED | Commercial license (nanos); external architecture (Unikraft) |

### 3-Protection-Domain Design for os-console (Phase H2)

```
os-console seL4 system image — built by moonshot-toolkit
┌──────────────────────────────────────────────┐
│ os-console PD                  priority 100  │
│  Cartridges: F2 F3 F4 F6 F9 F11 F12        │
│  ratatui TUI; calls PDs via PPC IPC only    │
│  Stack 256 KiB; heap 1 MiB                  │
└────────────┬────────────────────┬────────────┘
             │ PPC (sync IPC)     │ PPC (sync IPC)
             ▼                    ▼
┌─────────────────┐   ┌──────────────────────┐
│ network-pd      │   │ serial-pd            │  priority 150/180
│ smoltcp HTTP/1.1│   │ VirtIO serial        │
│ VirtIO-net DMA  │   │ ratatui → terminal   │
│ → Totebox svcs  │   │ keyboard input       │
└─────────────────┘   └──────────────────────┘
       ▲
       │ VirtIO-net
       ▼
moonshot-hypervisor → host network → Totebox services
```

The os-console PD holds NO network device capability. If compromised, it cannot
exfiltrate data without routing through the network-pd's defined PPC interface.

### system-* Dependencies for Phase H2

| Dependency | Purpose in os-console |
|---|---|
| `system-core` | `Capability` + `CapabilityType::Endpoint` tokens held in os-console PD's CNode at Phase H3 |
| `system-ledger` | F12 SYS-ADR-10 WORM audit log; `InMemoryLedger` + RFC 9162 Merkle proof chain |
| `system-substrate::PpcInvoker` | Phase H2: IPC from os-console PD to network-pd via seL4 PPC |
| `system-security` | Watchdog PD in seL4 system image; `notified(ch: u64)` on RESET_CHANNEL |

Phase H2 dependency on `system-core` requires project-system's Stage 6 to be current.
Coordinate via Command Session before H2 begins.

### Three-Binary Architecture Summary

os-console is one of three binaries using the same seL4 substrate:

| Binary | DOCTRINE name | Role | Hosted apps |
|---|---|---|---|
| os-console | Operator Terminal Surface | Host-side TUI; user interface | app-console-* cartridges |
| os-totebox | Sovereign WORM Data Vault | Bare metal; Ring 1+2 services | service-* (21 total) |
| os-orchestration | Stateless Aggregation Layer | Federation hub; vendor-side | app-orchestration-* |

moonshot-sel4-vmm is the shared PD runtime across all three. vendor-sel4-kernel is the
shared kernel build. system-core + system-ledger are the shared capability substrate.
moonshot-toolkit produces all three system images from their respective `.toml` specs.

Geometric Protection applies at each layer with the same seL4 proofs: per-cartridge on
os-console, per-service on os-totebox, per-org on os-orchestration.

---

## §13 — Work Log

2026-06-16 totebox@project-console: SSH mode retirement locked. Evidence sweep across all archives found:
BRIEF-tui-desktop-architecture.md (10-agent synthesis, now archived) classifies SSH mode as
"Model B (secondary; server-operator model)" vs. native packaged app as "Model A (primary)".
topic-os-console-platform.md calls SSH server mode "optional" from first publication.
guide-os-console-operator.md treats local native operation as default; GCE SSH is --profile gce-native.
§11 locked, §11b analysis section added, §12 SSH retirement item removed (was open, now locked),
§2 table updated to mark ssh-server as sunset, §10 H4 blocker updated to reflect locked decision.

2026-06-16 totebox@project-console: §10b added — hybrid TUI/GPU model, native clipboard (arboard), GpuBackend design, speed targets, clipboard + window design contract extensions. SSH mode unchanged for its remaining lifespan. Zero cartridge code changes required for hybrid. H-GUI phase track added.

2026-06-16 totebox@project-console: BRIEF created. Research sources: all project-console
BRIEFs, TOPIC-os-console-architecture.draft.md (archived), app-console-input/src/cartridge.rs,
moonshot-hypervisor README + lib.rs, BRIEF-cross-platform-release.md (platform floors).
DataGraph returned empty for all hypervisor/os-console queries — no entities indexed.
PointSav design side DataGraph also empty. Research is from codebase + BRIEF corpus only.
