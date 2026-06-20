---
artifact: brief
schema: foundry-brief-v1
brief-id: project-data-os-totebox-build-out
title: "os-totebox: Sovereign WORM Data Vault — Full Build-Out"
status: active
owner: project-data
parent: project-data-os-totebox-ppn-build-out
created: 2026-06-19
updated: 2026-06-19
authors: [totebox@project-data, claude-sonnet-4-6]
doctrine_anchors: [claim-34, claim-43, claim-45, claim-49, SYS-ADR-10, SYS-ADR-19]
---

# BRIEF — os-totebox: Sovereign WORM Data Vault — Full Build-Out

---

## §1 — Mission

os-totebox is the Sovereign WORM Data Vault tier of the three-binary architecture. It
runs as a Type I bare-metal OS (seL4 microkernel) with no shell, no root process, no
init system, and no package manager. Every service that touches durable data runs
inside a seL4 protection domain. The block device capability for WORM storage is held
exclusively by service-fs PD — no other PD holds it, and that exclusion is proved by
the seL4 capability DAG at build time.

This BRIEF supersedes `BRIEF-os-totebox-ppn-build-out.md` (archived 2026-06-12,
status: archived). It extends scope from the initial PPN service-people + JOURNAL work
to the full seL4 PD runtime build-out, startup ordering, and journal completion path.

**Scope of this BRIEF:**
- seL4 PD runtime (moonshot-sel4-vmm, ~300 LOC) for os-totebox
- Service stack startup ordering (Ring 1 → Ring 2)
- QEMU development boot path (Phase H1 first gate)
- J7 (JOURNAL-totebox-orchestration) §4 Implementation fill-in
- Prerequisites from system-* and moonshot-* archives

---

## §2 — Architecture: Three-Binary Context

The deployment tier is defined by three distinct OS surfaces:

| Binary | Role | Isolation model | Inference tier |
|--------|------|-----------------|----------------|
| os-console | Operator Terminal Surface | Type II VMM on host; TUI; app-console-* cartridges | Tier A via Doorman relay |
| **os-totebox** | **Sovereign WORM Data Vault** | **Type I bare metal; seL4 PDs; no shell** | **Tier A only (local OLMo 7B)** |
| os-orchestration | Stateless Aggregation Layer | Federation hub; app-orchestration-* PDs; holds no archive keys | Tier B (Yo-Yo broker :9180) |

os-totebox is the data persistence tier. It is the only surface that holds WORM block
device capabilities and signs ledger checkpoints. os-orchestration coordinates inference
across Tier B GPU nodes but never touches the WORM ledger directly. os-console provides
the operator terminal view but cannot bypass os-totebox capability boundaries.

**VM assignment (intended):** os-totebox workload maps to vm-intelligence on
foundry-workspace. The three-VM layout is: vm-workspace (host, os-privategit),
vm-intelligence (guest 1, 8 GiB, os-totebox + OLMo 7B + Doorman + service-content),
vm-mediakit (guest 2, 6 GiB, os-mediakit + media deployments). The 4-VM hard ceiling
on foundry-workspace applies; vm-console and vm-workplace belong on foundry-prod.

**Two-bottom design:** seL4 (native bottom, Phase 3, AArch64 hardware required) and
NetBSD (compat bottom, Phase 1 complete) run as separate QEMU processes on x86_64.
seL4-as-hypervisor-for-NetBSD is a planned Phase 3 milestone; it requires AArch64
hardware. The parallel-QEMU arrangement is the permanent x86_64 design.

**Shared substrate across all three binaries:**
- `vendor-sel4-kernel` — BSD-2-Clause, formally verified microkernel
- `moonshot-sel4-vmm` — ~300 LOC PD runtime (fill-in phase)
- `moonshot-toolkit` — system image builder, v0.3.1, 35 tests, Phase 1C complete
- `system-core` + `system-ledger` — capability types and WORM audit substrate

---

## §3 — Service Stack (Ring 1 + Ring 2)

Services are divided into two rings based on whether they touch durable storage. Ring 1
must reach healthy status before any Ring 2 service is permitted to start. The startup
script at `os-totebox/scripts/start-stack.sh` implements this ordering for the std/Linux
development path; the seL4 PD scheduler enforces it at the capability level in
production.

### Ring 1 — Boundary Ingest (storage-touching)

| Service | Port | Role |
|---------|------|------|
| service-fs | :9100 | WORM ledger enforcer; holds block device cap; Envelope A std/axum |
| service-input | :9106 | Input Machine; file ingest, migration, calibration; writes via service-fs |
| service-extraction | (no HTTP) | Filesystem watcher; CORPUS emitter to service-content/ledgers/ |
| service-egress | (planned) | Outbound data relay; reads WORM via service-fs only |

service-fs starts before all other Ring 1 services. Its WORM watch drop directory
(`FS_WATCH_DROP_DIR`) is the only sanctioned path through which Ring 2 services may
request persistent writes. Direct block device access is refused at the PD boundary.

### Ring 2 — Deterministic Processing (no direct storage)

| Service | Port | Role |
|---------|------|------|
| service-slm | :9080 | Doorman inference gateway (Tier A only: local OLMo 7B) |
| service-content | :9081 | DataGraph / LadybugDB knowledge graph |
| service-people | :9091 | Personnel ledger HTTP API |
| service-email | (planned) | Cold email ingestion pipeline |

Startup dependency chain (confirmed in `start-stack.sh`):

```
service-slm (:9080 /health)
  → service-content (:9081 /health)
    → service-people (:9091 /v1/people)
      → service-fs (:9100 /healthz)
        → service-extraction (process-alive check)
          → service-input (:9106 /healthz)
```

**Inference note:** service-slm on os-totebox is Tier A only. Tier B GPU brokering
(Yo-Yo, OLMo 3 32B-Think, Llama 3.3 70B grammar, :9180) is the os-orchestration
responsibility, managed by app-orchestration-slm. Tier C (external API) is routed
through os-orchestration's Yo-Yo broker, not through os-totebox.

---

## §4 — seL4 Protection Domain Design

Seven protection domains run on os-totebox in production. Priority values follow the
seL4 Microkit convention (higher number = higher priority). Each PD's capability set is
defined at build time and proved fixed by the seL4 kernel — runtime privilege escalation
is structurally impossible.

| PD | Crate | Priority | Capabilities held | Capabilities denied |
|----|-------|----------|-------------------|---------------------|
| watchdog-pd | system-security | 250 | Heartbeat timer; PD health channels | Block device; network; all other PDs |
| service-fs PD | service-fs | 200 | Block device cap (WORM); FS_WATCH_DROP_DIR write | Network (smoltcp); other PDs |
| network-pd | smoltcp VirtIO-net | 180 | VirtIO-net device; IP stack | Block device; all service PDs except via IPC |
| service-content PD | service-content | 150 | IPC to service-fs (read corpus); HTTP :9081 via network-pd | Block device (direct); service-fs write path |
| service-people PD | service-people | 130 | HTTP :9091 via network-pd; read ledger_personnel.json | Block device; service-fs write path; service-content IPC |
| service-slm PD | slm-doorman-server | 120 | HTTP :9080 via network-pd; IPC to OLMo engine | Block device; service-fs; service-content IPC |
| service-extraction PD | service-extraction | 110 | Write cap to service-fs FS_WATCH_DROP_DIR only; read CORPUS dir | Block device (direct); HTTP surface; other PDs |

**Priority rationale:**
- watchdog-pd at 250: must always preempt any PD to enforce liveness guarantees.
- service-fs PD at 200: storage consistency is higher-priority than query serving.
- network-pd at 180: packet processing must not be starved by application PDs.
- service-content at 150: DataGraph queries are on the critical path for most operations.
- service-people at 130: personnel API is less latency-critical than DataGraph.
- service-slm at 120: inference is best-effort within the Tier A budget.
- service-extraction at 110: background pipeline; may be preempted freely.

**Capability grant summary:** Seven `sel4_cp_grant` declarations in the system
description (`.system` file generated by moonshot-toolkit). The block device cap grant
to service-fs PD is the only block device grant in the image; this is asserted by the
image verifier in moonshot-toolkit at build time.

---

## §5 — Capability Geometry

The seL4 capability DAG provides a structural (geometric) guarantee: a compromised
service-slm PD cannot reach service-fs PD. This is not a policy assertion — it is proved
by the seL4 kernel at runtime through capability type-safety.

**The guarantee, stated precisely:**

service-slm PD holds no capability whose derivation path leads to the block device
endpoint. network-pd holds a VirtIO-net cap and routes HTTP for service-slm, but network-pd
itself holds no block device cap. service-extraction holds a write cap scoped to the
FS_WATCH_DROP_DIR channel, not to the block device directly. The only path from
service-slm to durable storage passes through:

```
service-slm PD
  → (no direct IPC cap to service-fs PD)
  → (no block device cap)
  → BLOCKED by seL4 capability type-safety
```

**Formal basis:** vendor-sel4-kernel (BSD-2-Clause) carries Isabelle/HOL proofs of
capability confinement for all invocation paths. The proof is maintained by the seL4
Foundation and covers AArch64 and RISC-V 64 target architectures (as of Microkit 2.2.0,
March 2026). x86_64 Microkit is not available; the AArch64 proof is the production path.

**What capability geometry does not guarantee:** it does not prevent side-channel
attacks through shared DRAM or shared cache lines if PDs share physical memory regions.
A deployment on hardware with separate physical DRAM regions per PD (e.g., separate DMA
zones) would eliminate that residual. This is noted as a limitation in J7 §7.

---

## §6 — "We Own It" Tier Table

Every dependency is classified by ownership. Tier 1 is code owned and maintained by
PointSav Digital Systems. Tier 2 is vendored external code included under a
permissive license. Rejected entries record alternatives that were evaluated and
declined, with the reason.

| Tier | Component | License | Notes |
|------|-----------|---------|-------|
| Tier 1 | moonshot-toolkit | LicenseRef-PointSav-Proprietary | System image builder; v0.3.1; 35 tests; Phase 1C complete |
| Tier 1 | moonshot-hypervisor | LicenseRef-PointSav-Proprietary | Planned hypervisor substrate; scaffold-coded |
| Tier 1 | moonshot-sel4-vmm | LicenseRef-PointSav-Proprietary | ~300 LOC PD runtime; fill-in phase |
| Tier 1 | system-core | LicenseRef-PointSav-Proprietary | v1.0.0; capability types, Merkle proofs, CBOR serialization |
| Tier 1 | system-ledger | LicenseRef-PointSav-Proprietary | v1.0.0; LedgerConsumer trait; Verdict enum; WORM audit |
| Tier 1 | service-fs | LicenseRef-PointSav-Proprietary | WORM enforcer; Envelope A std/axum; :9100 |
| Tier 1 | service-input | LicenseRef-PointSav-Proprietary | Input Machine; :9106 |
| Tier 1 | service-extraction | LicenseRef-PointSav-Proprietary | Filesystem watcher; CORPUS emitter |
| Tier 1 | service-content | LicenseRef-PointSav-Proprietary | DataGraph / LadybugDB; :9081 |
| Tier 1 | service-people | LicenseRef-PointSav-Proprietary | Personnel ledger HTTP API; :9091 |
| Tier 1 | service-slm (Doorman) | LicenseRef-PointSav-Proprietary | Inference gateway; Tier A local OLMo only on os-totebox; :9080 |
| Tier 1 | app-orchestration-slm | LicenseRef-PointSav-Proprietary | Yo-Yo broker; Tier B; :9180; os-orchestration surface |
| Tier 2 | vendor-sel4-kernel | BSD-2-Clause | Formally verified microkernel; seL4 Foundation |
| Tier 2 | smoltcp | MIT | VirtIO-net IP stack for network-pd |
| REJECTED | rust-sel4 | Apache-2.0 | External crate; moonshot-sel4-vmm is the sovereign replacement |
| REJECTED | nanos | commercial | Commercial unikernel; licensing incompatible with customer deploy model |
| REJECTED | Unikraft | BSD-3-Clause + external | External architecture; would import upstream governance |

---

## §7 — Phase Plan

### H0 — Reference artefact (complete)

- Pre-built `os-totebox-release.img` (50 MB) exists at
  `project-system/pointsav-monorepo/os-totebox/`.
- NetBSD 10.1 QCOW2 guest boots under GCP TCG (QEMU 13, -smp 4 -m 1G).
- `system-ledger-server` (PID 931) and `slm-doorman-server` (PID 1142) running in image.
- `kern.veriexec.strict = 0` (observe mode; Phase H1 item: promote to strict=1).
- SSH from host times out at banner under TCG load (Phase H1 item: add `UseDNS no`).
- This is the compat-bottom (NetBSD) reference. moonshot-toolkit seL4 path replaces it
  at H1.

### H1 — moonshot-sel4-vmm PD runtime (planned)

Gate: moonshot-toolkit Phase 1C already complete (v0.3.1, 35 tests). The seL4 path
requires Microkit SDK download (v2.1.0 or v2.2.0) — this is a blocked operator action
in project-system scope.

Deliverables for H1:
1. `moonshot-sel4-vmm/src/lib.rs` — ~300 LOC PD runtime:
   - PD entry point (`sel4_main!`)
   - IPC channel init for each of the 7 PDs
   - Heartbeat loop registered with watchdog-pd
   - Panic handler writing to virtio-console before halting
2. `moonshot-toolkit/examples/os-totebox.toml` — system spec:
   - 7 PD declarations with priority values from §4
   - Capability grant table (block device to service-fs PD only)
   - Memory region assignments
3. QEMU dev boot: `qemu-system-aarch64 -M virt -cpu cortex-a53 -kernel ...` with
   virtio-blk and virtio-net devices; smoke-tests service-fs /healthz on guest serial.

First milestone within H1: QEMU dev boot. Bare-metal (H2) follows only after dev boot
passes.

### H2 — Bare-metal IMG (planned)

- moonshot-toolkit generates a bootable AArch64 IMG.
- Replaces the Phase H0 NetBSD QCOW2 on the vm-intelligence guest.
- Requires AArch64 hardware decision: Option A (AArch64 GCP, ~$50–100/month) or
  Option B (Firecracker x86_64 on Laptop A, KVM-native, 125ms boot).
- Operator decision required before this phase begins.
- Gates Phase S4 (Genesis Protocol) in project-infrastructure.

### Beyond H2

- Full IPC harness between all 7 PDs.
- seL4 signing oracle: system-ledger-pd on a separate host signs ledger snapshot hashes
  at archive egress; signing key never leaves the PD.
- Benchmark harness for J7 §5 Evaluation: startup overhead, per-inference overhead,
  concurrent-session isolation tests.
- os-totebox as the canonical data layer backing vm-intelligence (Part C Step C1).

---

## §8 — system-* Prerequisites

The following crates must be present and passing tests before the H1 build can proceed.
These are owned by project-system scope; this archive consumes them as dependencies.

| Crate | Version | Status | Dependency path |
|-------|---------|--------|-----------------|
| system-core | v1.0.0 | Complete | Capability types; Merkle proofs; CBOR; dual std/sel4 feature flags |
| system-ledger | v1.0.0 | Complete | LedgerConsumer; Verdict (Allow/Refuse/ExtendThenAllow); InMemoryLedger |
| vendor-sel4-kernel | v15.0.0 | Vendored | BSD-2-Clause; seL4 Foundation Isabelle/HOL proofs |
| moonshot-toolkit | v0.3.1 | Phase 1C complete | System image builder; `.system` file generation; image verifier |
| moonshot-sel4-vmm | fill-in | In progress | ~300 LOC PD runtime; H1 primary deliverable |
| Microkit SDK | v2.1.0 or v2.2.0 | Blocked — operator download | Required for AArch64 PD compilation |

**system-core kernel contract (per `conventions/system-substrate-doctrine.md` §3.1):**
Before the kernel honors any capability invocation, it consults the ledger for:
- Current revocation status of the invoking capability.
- Time-bound expiry (Mechanism A).
- Apex root validity.

**system-ledger Verdict enum:**
- `Allow` — invocation honored; capability current and unexpired.
- `Refuse(RefuseReason)` — refused with structured reason.
- `ExtendThenAllow { new_expiry_t: u64 }` — honored AND witness record appended; future
  invocations see the new expiry. Caller MUST append witness record before honoring.

**Note:** system-core and system-ledger have standalone `[workspace]` in their own
`Cargo.toml` and are NOT members of the pointsav-monorepo root workspace. They are
consumed as path dependencies from the appropriate crate paths.

---

## §9 — Current State

### Built and committed

| Artifact | Commit | Status |
|----------|--------|--------|
| service-people: GET /v1/people, GET /v1/people/{id}, axum :9091, ledger_personnel.json | 997b8d22 | Committed; Stage 6 pending |
| service-extraction: workspace member (Cargo.toml fix; duplicate caseless removed) | 997b8d22 | Committed; Stage 6 pending |
| JOURNAL-totebox-orchestration-v0.1.stub.md (J7): Abstract, §1–§3, §7, §9, §10 (~2,614 words) | 8ab01ff2 | Committed; Stage 6 pending |
| os-totebox/scripts/start-stack.sh | In tree | Ring 1→2 startup ordering; all 6 services; health checks |
| os-totebox/scripts/build-image.sh | In tree | Build script scaffold |
| os-totebox/scripts/provision-data-disk.sh | In tree | Data disk provisioning |
| os-totebox/scripts/totebox-launcher.sh | In tree | Launcher wrapper |
| os-totebox/scripts/rc.d/ (doorman, llama_server, service_content, system_ledger) | In tree | NetBSD rc.d scripts (Phase H0 compat bottom) |
| os-totebox/src/lib.rs | In tree | Scaffold; Rust crate stub |
| os-totebox/Cargo.toml | In tree | v0.1.0; no dependencies yet |
| os-totebox-release.img (50 MB) | project-system | H0 reference artefact; Phase 1 complete |

### Not yet built (gated)

| Item | Gate | Owner |
|------|------|-------|
| moonshot-sel4-vmm PD runtime (~300 LOC) | Microkit SDK operator download | project-system |
| moonshot-toolkit os-totebox.toml system spec | moonshot-sel4-vmm scaffold | project-system |
| QEMU AArch64 dev boot | os-totebox.toml complete | project-system / project-data |
| service-people POST/PATCH endpoints | F2 console cartridge read-only validation first | project-data |
| J7 §4 Implementation | First os-totebox deployment evidence | project-data |
| J7 §5 Evaluation + §6 Discussion + §8 Conclusion | Benchmark harness | project-data |
| J7 References section | All stubs resolved; J2 citable preprint (posted 2026-05-28) | project-data |
| AArch64 hardware decision (Option A GCP or Option B Firecracker) | Operator | Command Session |

### Stage 6 status

At least 25 commits from project-data are pending Stage 6 promotion to canonical
(pointsav-monorepo). Command Session outbox message requesting promote was sent after
Session 1 (2026-06-11). Confirm with Command Session before assuming promotion is
complete; these commits include service-people, service-extraction workspace fix, and J7.

---

## §10 — Decisions Locked

1. **Type I bare metal, seL4 only.** No shell, no root, no init system on os-totebox in
   production. The NetBSD compat bottom (Phase H0) is a transitional artefact; seL4 PDs
   are the target.

2. **service-fs PD holds the only block device cap.** This is proved at build time by
   moonshot-toolkit's image verifier. No other PD receives a block device grant.

3. **Tier A inference only on os-totebox.** service-slm (Doorman) on os-totebox routes
   to local OLMo 7B only. Tier B (Yo-Yo GPU broker) is app-orchestration-slm on
   os-orchestration (:9180). Tier C (external API) is never reached from inside
   os-totebox.

4. **moonshot-sel4-vmm over rust-sel4.** External rust-sel4 crate is rejected. The
   sovereign ~300 LOC PD runtime is the only seL4 Rust binding used.

5. **smoltcp for network-pd.** MIT-licensed; no external libc dependency; suitable for
   `no_std` PD context.

6. **AArch64-first for seL4.** Microkit 2.2.0 targets AArch64 and RISC-V 64 only. No
   x86_64 Microkit path. x86_64 path remains the compat-bottom (NetBSD QCOW2).

7. **system-core + system-ledger as standalone crates.** They are NOT monorepo workspace
   members; they use a standalone `[workspace]` declaration and are path-included by
   consuming crates.

8. **Ring 1 before Ring 2.** service-fs must reach /healthz before any Ring 2 service
   receives a start signal. Enforced by start-stack.sh on the std path; enforced by
   seL4 PD scheduler on the native path.

---

## §11 — Decisions Open

1. **AArch64 hardware acquisition.** Option A: AArch64 GCP instance (~$50–100/month
   operating cost); Option B: Firecracker x86_64 on Laptop A (KVM-native, 125ms boot,
   no monthly cloud cost). Decision required from operator before Phase H2 begins.
   Tracked in project-infrastructure.

2. **service-people CRUD endpoints.** POST /v1/people and PATCH /v1/people/{id} are
   deferred until the F2 console cartridge read-only path (GET) is validated in
   app-console-people. No decision required from this BRIEF; scope is
   project-data-internal.

3. **service-email integration.** Cold email ingestion pipeline is planned for Ring 2
   but no port, schema, or startup ordering is locked. Deferred to a future session after
   the core Ring 1 + Ring 2 stack is live on a VM.

4. **J7 target journal.** Current target: MLSys (ACM, 22% AR). No submission date set.
   Decision to stay at MLSys or shift venue is open and requires operator direction once
   §4 + §5 are drafted.

5. **Benchmark harness design.** J7 §5 Evaluation requires startup overhead,
   per-inference overhead, and concurrent-session isolation tests. Harness design not
   yet specified; will be determined once the first live deployment is available.

6. **veriexec strict mode.** Phase H0 runs `kern.veriexec.strict = 0` (observe). The
   decision to promote to strict=1 on the NetBSD compat bottom is deferred to Phase H1
   preparation. On the seL4 native bottom this is not applicable.

---

## §12 — JOURNAL Tie-In

### J7 — JOURNAL-totebox-orchestration-v0.1.stub.md

State: draft, v0.3, 2,614 of 9,500 target words. Target venue: MLSys (ACM, 22% AR).

**Sections complete:** Abstract, §1 Introduction, §2 Literature Review, §3 Methodology,
§7 Limitations, §9 Formal Hypotheses, §10 Falsification Programme.

**Sections stubbed (gated on this BRIEF):**

| Section | Blocker | This BRIEF contribution |
|---------|---------|------------------------|
| §4 Implementation | First deployment evidence | Update after each session that advances the build |
| §5 Evaluation | Benchmark harness + live deployment | Design harness after H1 QEMU dev boot |
| §6 Discussion | §4 + §5 complete | Synthesize after evaluation data exists |
| §8 Conclusion | §4 + §5 + §6 complete | Final pass |
| References | All citations resolved | J2 preprint posted 2026-05-28 (citable); [CITATION NEEDED — J2] stubs need promotion |

**J7 §7 — seL4 extension note (already written):** J7 §7 explicitly cites a planned
seL4 extension (each session archive in a separate seL4 PD eliminates filesystem
side-channel risk). This BRIEF is the implementation record for that planned extension.
Companion work: J2 (JOURNAL-capability-geometry, v0.2, ASPLOS target, preprint posted
2026-05-28) provides the verified capability ledger substrate that J7 §3 and §7 depend
on.

**Protocol:** After each session that advances the os-totebox build, update J7 §4
Implementation with implementation evidence before closing the session. Do not accumulate
more than two sessions of implementation evidence without writing it to J7.

### J2 — JOURNAL-capability-geometry-v0.2.md

State: near submission-ready. Blocker: Bench #9 (verify_inclusion_proof, 1024-leaf,
quiet-VM re-run — 22 outliers at ±11% CI; requires re-run on load avg < 1.0). Also
required: ORCID IDs for all three authors; citation ID promotions for [external: url]
stubs.

J2 carries the formal proofs (seL4 v15.0.0, Isabelle/HOL) for the capability confinement
that §5 of this BRIEF describes. J7 cannot be submitted until J2 is at minimum posted
as a citable preprint (already done, 2026-05-28). J2 submission is a project-data
milestone that unblocks J7 submission.

### J4 — JOURNAL-private-network-v0.5.1.md

State: all body sections complete; ~2,600 words below the 9,000-word target. Less
directly tied to this BRIEF. WireGuard benchmark data (tunnel establishment n=30,
44±5 ms; re-handshake n=10, 59±20 ms; policy-change propagation 8 ms mean) is relevant
background for the private network topology that os-totebox VMs sit inside.

---

## §13 — Work Log

### Session 1 — 2026-06-11 (jwoodfine, project-data)

- `service-people/src/bin/server.rs` — axum HTTP server, GET /v1/people + GET
  /v1/people/{id}, port :9091, reads `ledger_personnel.json`. Committed 997b8d22.
- `service-extraction/Cargo.toml` — standalone `[workspace]` removed; added to root
  workspace. `caseless` duplicate removed from Cargo.lock. Committed 997b8d22.
- `JOURNAL/JOURNAL-totebox-orchestration-v0.1.stub.md` — J7 HOLD lifted; Abstract,
  §1–§3, §7, §9–§10 written (~2,614 words body). Committed 8ab01ff2.
- Outbox → Command: promote project-data (25 commits ahead of canonical). Sent.
- Outbox → project-gis: service-people contract ACK for project-console F2 relay. Sent.

### Session 2 — 2026-06-19 (automated research sweep, claude-sonnet-4-6)

- Comprehensive BRIEF drafted from coordinator research across project-infrastructure,
  project-system, project-data-journals, project-orchestration, and project-data-code.
- Confirmed start-stack.sh is present and implements Ring 1 → Ring 2 ordering.
- Confirmed rc.d scripts (doorman, llama_server, service_content, system_ledger) are
  present in os-totebox/scripts/rc.d/ for the NetBSD H0 compat bottom.
- No code changes this session; BRIEF-only session.

---

## §14 — Carry-Forward

Items deferred from this session that must be actioned in a future Totebox session
starting in this archive:

- [ ] **service-people POST/PATCH** — after F2 read-only (GET) is validated in
  app-console-people. No blocker on this side; F2 cartridge integration is the gate.
  [2026-06-19 totebox@project-data]

- [ ] **os-totebox startup script — seL4 path** — `os-totebox/src/lib.rs` needs a
  `sel4_main!` entry point wired to the IPC channels defined in the system spec. Gated
  on moonshot-sel4-vmm H1 scaffold from project-system.
  [2026-06-19 totebox@project-data]

- [ ] **J7 §4 Implementation fill-in** — first deployment on QEMU dev boot (H1) provides
  the implementation evidence. Update J7 §4 within two sessions of H1 completion.
  [2026-06-19 totebox@project-data]

- [ ] **J7 §5 Evaluation harness design** — design benchmark harness after H1 QEMU dev
  boot; startup overhead + per-inference overhead + concurrent-session isolation.
  [2026-06-19 totebox@project-data]

- [ ] **J2 Bench #9 re-run** — re-run verify_inclusion_proof (1024-leaf) on quiet VM
  (load avg < 1.0); 22 outliers at ±11% CI must be cleared before J2 submission.
  Project-system scope for the run; project-data scope for the J2 edit.
  [2026-06-19 totebox@project-data]

- [ ] **Stage 6 confirm** — verify Command Session has promoted the 25+ commits from
  project-data (including 997b8d22 and 8ab01ff2). Read Command Session outbox at next
  startup to confirm.
  [2026-06-19 totebox@project-data]

- [ ] **AArch64 hardware decision** — operator must choose Option A (GCP AArch64) or
  Option B (Firecracker x86_64 on Laptop A) before Phase H2 begins. Route request to
  Command Session outbox.
  [2026-06-19 totebox@project-data]

- [ ] **service-email Ring 2 integration** — cold email ingestion pipeline; port, schema,
  startup ordering to be determined once core Ring 1 + Ring 2 stack is live on a VM.
  [2026-06-19 totebox@project-data]
