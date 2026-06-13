---
artifact: brief
schema: foundry-draft-v1
title: "os-totebox Phase 1 Boot Milestone — Service Stack Deployment Plan"
status: archived
created: 2026-06-11
updated: 2026-06-12 (smoke test PASSED; commit 92692800)
author: totebox@claude-code (claude-sonnet-4-6)
authored_with: claude-sonnet-4-6
language_protocol: CODE
audience: internal-engineering
bcsc_class: no-disclosure-implication
research_done_count: 5
research_suggested_count: 0
open_questions_count: 2
research_provenance: sub-agent (Fable + 3 background agents)
research_inline: true
cites: []
related_briefs:
  - BRIEF-substrate-phd-thesis-2026-05-27.md
notes_for_editor: |
  Engineering planning BRIEF. Not for editorial pipeline. Self-contained build plan
  for Phase 1 os-totebox boot milestone. Research findings from 2026-06-11 Fable analysis.
contaminated_note: "M-17 contamination — belongs to project-data; archived from project-gis 2026-06-13 by command@claude-code"
---

# os-totebox Phase 1 Boot Milestone

## Goal

Boot a NetBSD 10.1 QCOW2 guest image (os-totebox) under GCP TCG with three services
running and Veriexec strict mode enforced. This image becomes the base for all Totebox
Archives — each project-* cluster gets its own instance.

**Architecture context:** os-totebox is the compat-bottom of the two-bottom design.
seL4 (native bottom) and NetBSD (compat bottom) run as separate QEMU processes on x86_64.
seL4-as-hypervisor-for-NetBSD is Phase 3 (AArch64 hardware required; libvmm x86_64 is
unmerged PR #198 as of 2026-06-11). The parallel-QEMU architecture is the permanent
x86_64 arrangement.

**Uniqueness claim:** the composition of a verification-pedigree microkernel +
NetBSD Veriexec + transparency-log checkpointing in a freely transferable VM capsule
appears previously undemonstrated. See BRIEF-substrate-phd-thesis §7.5.4 for the
complete framing paragraph with required precision hedges.

---

## Phase 1 Service Stack

Three binaries — cross-compiled for `x86_64-unknown-netbsd` — installed by `build-image.sh`:

| Binary | Port / socket | Purpose | Phase 1 state |
|---|---|---|---|
| `system-ledger-server` | `/run/system-ledger/ledger.sock` (Unix socket ONLY — NO HTTP) | WORM capability ledger; stdlib blocking I/O | In image: 502K, 5/5 tests passing |
| `slm-doorman-server` | `:9080/healthz`, `:9080/readyz` | MCP routing; Tier-0-only baseline (COLD) | In image: `SLM_FORCE_BROKER_MODE=true` in rc.d |
| `service-content` | `:9081/healthz` | Entity corpus | NOT in Phase 1 image; `service_content=NO` in rc.conf |

**Excluded from Phase 1:**
- `service-fs` — seL4 PD stub (`#![no_std]`, bare-metal); Phase 3 artifact; NOT a NetBSD service
- `llama-server` — inference weights are deployment-time injection; never in base image

**Doorman "COLD" state** (correct for base image):
- `/healthz` → HTTP 200 (doorman alive)
- `/readyz` → HTTP 503 `{"reason":"no_tier_available"}` — this is the PASSING state

---

## seL4 Signing Oracle (Phase 1 Transferability Demo)

The already-working `system-ledger-pd` (commit `6fabe58e`) runs as a **signing oracle
on a separate host** from the archive being built. This is Option C+E per Fable analysis.

Protocol:
1. NetBSD guest accrues ledger entries in `system-ledger-server`
2. At archive egress, guest calls PPC → seL4 PD signs the ledger snapshot hash
3. Signing key was born inside the PD; it never leaves; host operator cannot reach it
4. Signed checkpoint travels with the QCOW2 as a custody artifact
5. Destination host verifies against PD's public key (obtained via PPC, not disk)

This delivers a genuine Phase 1 isolation property: the archive host's operator cannot
access the signing key because it lives on a separate machine. Phase 3 relocates the
oracle into the seL4 isolation boundary on the same machine — same artifacts, stronger
isolation guarantee. The narrative arc is architecturally honest.

---

## Cross-Compile Setup

Target: `x86_64-unknown-netbsd` (Rust Tier 2).

```bash
rustup target add x86_64-unknown-netbsd
```

Sysroot: download NetBSD 10.1 amd64 `base.tgz` + `comp.tgz`, extract to `build/netbsd-sysroot/`.
`comp.tgz` provides `/usr/include` and `/usr/lib/*.a`.

Cargo config at `os-totebox/.cargo/config.toml`:
```toml
[target.x86_64-unknown-netbsd]
linker = "x86_64--netbsd-gcc"
```

For pure-Rust binaries with no C FFI, `rust-lld` + `--sysroot` also works if no NetBSD
cross-GCC is installed. All three Phase 1 binaries are pure Rust.

Build commands:
```bash
# From monorepo root
cargo build --release --target x86_64-unknown-netbsd -p system-ledger-server
cargo build --release --target x86_64-unknown-netbsd -p service-content

# From service-slm/ (separate workspace)
cargo build --release --target x86_64-unknown-netbsd -p slm-doorman-server
```

---

## rc.d/doorman — DONE (2026-06-12)

`os-totebox/scripts/rc.d/doorman` now exports broker-mode env vars:
```sh
export SLM_FORCE_BROKER_MODE=true
export SLM_BIND_ADDR=0.0.0.0:9080
export SLM_AUDIT_DIR=/var/db/doorman/audit
export FOUNDRY_ROOT=/var/foundry
```
Plus `doorman_precmd()` creates `mkdir -p /var/db/doorman/audit` and `/var/foundry`.

`build-image.sh` rc.conf section: `llama_server=NO`, `service_content=NO`. DONE.

Note: `SLM_FORCE_BROKER_MODE` rename to `SLM_TIER0_BASELINE` proposed to project-intelligence
(outbox 2026-06-11). Use current name until rename confirmed in service-slm source.

---

## Veriexec Policy

`build-image.sh` generates `/etc/signatures` from SHA-256 of installed binaries. The
`system-substrate-netbsd/src/lib.rs` `OS_TOTEBOX_BINARIES` array names the 3 binaries:
- `/usr/bin/system-ledger-server`
- `/usr/bin/slm-doorman-server`
- `/usr/bin/service-content`

`llama-server` is skipped by the existing `[ -f "${BIN_PATH}" ] || continue` guard in
`build-image.sh:122-132`. No code change needed for the Veriexec manifest.

`veriexec=YES` + strict level 1 (IDS mode: mismatch denies exec, writes allowed) is the
Phase 1 enforcement target. Strict level 2 (LEARNING-off mode: mismatch denies + logs)
is Phase 2.

---

## Build Sequence

```bash
# On GCP e2 Ubuntu host, from monorepo root:

# Step 1: cross-compile toolchain
rustup target add x86_64-unknown-netbsd
mkdir -p build/netbsd-sysroot
curl -fSL https://cdn.netbsd.org/pub/NetBSD/NetBSD-10.1/amd64/binary/sets/base.tgz \
  | tar -xz -C build/netbsd-sysroot/
curl -fSL https://cdn.netbsd.org/pub/NetBSD/NetBSD-10.1/amd64/binary/sets/comp.tgz \
  | tar -xz -C build/netbsd-sysroot/

# Step 2: cross-compile 3 binaries
cargo build --release --target x86_64-unknown-netbsd -p system-ledger-server
cargo build --release --target x86_64-unknown-netbsd -p service-content
(cd service-slm && cargo build --release --target x86_64-unknown-netbsd -p slm-doorman-server)

# Step 3: build image
cd os-totebox
TOOLS_DIR=../build/netbsd-tools \
BINARIES_DIR=../../target/x86_64-unknown-netbsd/release \
bash scripts/build-image.sh

# Note: nbmakefs comes from NetBSD cross tools (TOOLS_DIR).
# If build/netbsd-tools is absent, build cross tools first:
#   git clone https://github.com/NetBSD/src netbsd-src
#   cd netbsd-src && ./build.sh -U -T ../os-totebox/build/netbsd-tools -m amd64 tools
```

---

## Smoke Test — PASSED 2026-06-12 [totebox@claude-code, session 26]

**Method:** in-VM console via QEMU monitor sendkey + VGA screendump. SSH from host
times out at banner under heavy TCG load (not blocking; console login confirmed working).

**QEMU 13:** PID 1543740, `-bios /usr/share/ovmf/OVMF.fd -accel tcg,thread=multi -smp 4 -m 1G`
`-netdev user,id=net0,hostfwd=tcp::12222-:22,hostfwd=tcp::18011-:8011,hostfwd=tcp::19080-:9080`

Results:
```
[x] system-ledger-server   PID 931  ttyE0  /usr/bin/system-ledger-server  RUNNING
[x] slm-doorman-server     PID 1142 ttyE0  /usr/bin/slm-doorman-server    RUNNING
[x] sshd                   PID 937  ?      listener 0 of 10-100 startups  RUNNING
[x] /run/system-ledger/ledger.sock   PRESENT (confirmed: cat returns ENOTSUP = is a socket)
[x] /healthz → ok (HTTP 200)         PASS  (ftp -o /tmp/h.txt returns 2 bytes: "ok")
[x] /readyz  → 503 Unavailable       PASS  (COLD mode; ftp returns "503 Service Unavailable")
[~] kern.veriexec.strict = 0         OBSERVE mode (not IDS mode 1 as planned)
                                      Phase 2 item: raise to 1 after adding system binary fingerprints
[~] SSH from host                     times out at banner exchange under TCG; console login works
                                      Phase 2 item: diagnose (DNS reverse lookup? fork latency?)
```

**tools note:** `curl`, `fetch`, `wget` are NOT in NetBSD 10.1 base.tgz. Use `ftp -o <file> <url>`.
`nc` (`/usr/bin/nc`) IS available for raw TCP tests.

**commit:** `92692800` — 4 files: `.cargo/config.toml`, `build-image.sh`, `rc.d/doorman`, `rc.d/system_ledger`

---

## Phase 2 and Phase 3 Roadmap

**Phase 2 — Lightweight MICROVM config + Laptop A KVM:**
- Custom TOTEBOX kernel config (strip GENERIC to virtio-blk + virtio-net + virtio-console + ACPI)
- NetBSD-current MICROVM config (9–15 ms KVM boot) — backport or track -current
- base.tgz pruning via `MKMAN=no MKDOC=no MKHTML=no MKCATPAGES=no MKINFO=no MKNLS=no`
- Per-archive firstboot identity injection from `provision-data-disk.sh` data QCOW2
- Laptop A KVM deployment; WireGuard wg0 peer to GCP

**Phase 3 — seL4 as hypervisor hosting NetBSD (AArch64 hardware required):**
- Trigger: Phase 2 stable ≥1 week AND operator acquires AArch64 hardware
- libvmm AArch64 v0.1.0 → Linux guest first (de-risk VMM) → swap to NetBSD/evbarm GENERIC64
- `system-ledger-pd` as seL4 native PD; guest reaches it only via PPC
- `service-fs` becomes a real seL4 PD (Phase 3 artifact; currently bare-metal stub)
- Relocate signing oracle from separate host into seL4 isolation boundary

---

## Open Questions

1. ~~**nbmakefs availability**~~ — ANSWERED 2026-06-11: host `makefs` used (fallback in
   `build-image.sh`). `build/netbsd-tools/bin/nbmakefs` does NOT exist. Host makefs works.

2. ~~**slm-doorman-server workspace**~~ — ANSWERED 2026-06-11: crate name confirmed
   `slm-doorman-server`; builds at `service-slm/` root.

3. ~~**SSH host key ownership**~~ — RESOLVED 2026-06-12 via `sudo chown -R 0:0 ${OVERLAY}`
   in build-image.sh §9 (before makefs). Root cause: `tar` extraction as uid=1001 set all
   files to uid=1001. PAM's `openpam_check_desc_owner_perms()` blocks login.conf/pam.d/*
   unless owned by root. Blanket chown covers ALL files — SSH host keys, pam.d, login.conf.
   Console login confirmed working (VGA screendump: "ROOT LOGIN (root) on tty constty").

4. **Kernel serial console**: NetBSD GENERIC under QEMU TCG does not output to COM1 after
   kernel jump — UEFI path ignores `consdev com0` in boot.cfg. Workaround used: QEMU monitor
   (`-monitor unix:/tmp/qemu13-monitor.sock`) + VGA screendump for in-VM diagnostics.
   Phase 2 proper fix: custom kernel config with `CONSDEVNAME="com0"`.

5. **kern.veriexec.strict = 0 (not 1)**: Veriexec fingerprints load for our 3 custom
   binaries but strict level is not raised. Phase 2: add system binary fingerprints
   (sshd, sshd-session, /bin/*, /sbin/*, etc.) or explicitly set strict=1 in rc.conf.
   Strict level 1 (IDS mode) is the Phase 1 target but was not achieved in this run.

6. **SSH from host times out at banner**: sshd IS listening (netstat -an, PID 937). TCP
   connects from host (SLIRP hostfwd port 12222→22) but SSH banner times out after >10s.
   Possible causes: (a) TCG fork/exec latency for sshd-session, (b) DNS reverse lookup
   on 10.0.2.2 (SLIRP host) hanging. Phase 2: add `UseDNS no` to sshd_config in image.
