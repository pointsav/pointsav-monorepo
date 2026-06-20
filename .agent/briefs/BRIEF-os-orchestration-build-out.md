---
artifact: brief
schema: foundry-brief-v1
brief-id: project-data-os-orchestration-build-out
title: "os-orchestration: Stateless Aggregation Layer — Full Build-Out"
status: active
owner: project-data
created: 2026-06-19
updated: 2026-06-19
authors: [totebox@project-data, claude-sonnet-4-6]
doctrine_anchors: [claim-23, claim-34, claim-43, claim-49, claim-52, SYS-ADR-19]
---

# BRIEF — os-orchestration: Stateless Aggregation Layer — Full Build-Out

---

## §1 — Mission

Build out `os-orchestration` as the stateless federation hub in the three-binary
architecture: the layer that aggregates across Totebox archives without holding any
archive keys, writing any data, or owning any WORM ledger.

`os-orchestration` is the commercial boundary. A single Totebox archive running
Rings 1–2 is included in the base product (Doctrine claim #23). Connecting to the
aggregation layer — for multi-archive queries, shared GPU brokering, cross-org
GIS/BIM federation, and marketplace surfaces — is the paid tier.

The build-out has two phases that can proceed in parallel:

1. **Yo-Yo broker** (`app-orchestration-slm`, already scaffolded) — mature to
   production: Phase 2 endpoints, signed membership tokens, integration tests.
2. **Capability-broker PD + remaining app-orchestration-* applications** — implement
   the seL4 protection domain that enforces the "no direct Totebox contact" rule;
   then activate market, exchange, gis, and bim as capability-gated PDs.

The rename from `os-interface` to `os-orchestration` is in flight; use `os-orchestration`
in all new code and documentation. The legacy directory `os-interface/` remains until
the rename commit lands.

---

## §2 — Architecture: Three-Binary Context

The platform has three OS-level binaries, each with a distinct isolation posture:

| Binary | Role | Isolation | Holds keys? |
|---|---|---|---|
| `os-console` | Operator Terminal Surface | Type II VMM on host; TUI; app-console-* cartridges | No |
| `os-totebox` | Sovereign WORM Data Vault | Type I bare metal (target); service-* PDs; no shell, no root, no init | Yes — archive keys stay inside |
| `os-orchestration` | Stateless Aggregation Layer | Federation hub; app-orchestration-* PDs | No — holds no archive keys |

All three share the same substrate:

- `vendor-sel4-kernel` (BSD-2-Clause, formally verified) — microkernel
- `moonshot-sel4-vmm` (~300 LOC PD runtime, fill-in phase) — PD entry points and IPC wiring
- `moonshot-toolkit` (v0.3.1, 35 tests, Phase 1C complete) — system image builder
- `system-core` v1.0.0 + `system-ledger` v1.0.0 — capability type definitions and WORM ledger substrate

`os-orchestration` is distinct from `os-totebox` in one architectural invariant: it is
stateless. Every piece of persistent data it touches lives in a Totebox archive. The
orchestration layer computes over capability-granted views of that data; it never writes
to a Totebox WORM ledger directly.

**VM assignment:** `os-orchestration` runs on `vm-intelligence` alongside project-intelligence
(Doorman + OLMo), project-command, project-bim, project-gis, project-orgcharts, and
project-bookkeeping. It is not a current active VM target — full VM separation is gated on
Part C Step C1 (vm-intelligence launch), which requires WireGuard Part A completion and
`app-orchestration-command` v0.0.1.

---

## §3 — Application Stack (app-orchestration-*)

Five applications are planned for the orchestration surface. Each runs as a separate
seL4 protection domain. None may contact a Totebox service directly — all cross-Totebox
access routes through the capability-broker PD (§4).

| Application | State | Port | Purpose | License |
|---|---|---|---|---|
| `app-orchestration-slm` | Scaffold-coded (MVP wired) | :9180 | Yo-Yo GPU broker; Tier B inference brokering across Totebox archives | Proprietary — paid |
| `app-orchestration-gis` | Reserved-folder | TBD | GIS/mapping federation; deployed as `gateway-orchestration-gis-1` | Proprietary — paid |
| `app-orchestration-bim` | Reserved-folder (research phase) | TBD | BIM aggregation hub; cross-archive IFC clash detection, portfolio queries, BIM model conversion | Proprietary — paid |
| `app-orchestration-market` | Reserved-folder | TBD | Marketplace storefront; Leapfrog 2030 browser surface (Doctrine claim #52) | Proprietary — paid |
| `app-orchestration-exchange` | Reserved-folder | TBD | Ad campaign UI; Leapfrog 2030 browser surface (Doctrine claim #52) | Proprietary — paid |

**Activation sequence (intended):**

1. `app-orchestration-slm` — mature the existing scaffold to production (Phase 2 endpoints, signed tokens).
2. Capability-broker PD — implement the chokepoint before activating any further application PDs.
3. `app-orchestration-gis` — GIS federation is the highest-priority application after the broker.
4. `app-orchestration-bim` — depends on `moonshot-bim-engine` and `app-privategit-bim` reaching Active state.
5. `app-orchestration-market` and `app-orchestration-exchange` — Leapfrog 2030 gating; not on critical path for current milestones.

---

## §4 — seL4 Protection Domain Design (capability-broker PD architecture)

`os-orchestration` uses the same seL4 substrate as `os-totebox` but with a different
PD topology. The defining structural rule: no application PD in `os-orchestration` may
hold a direct seL4 capability to any service in a Totebox archive. All cross-archive
communication is mediated by a single capability-broker PD.

**Planned PD inventory for os-orchestration:**

| PD | Priority | Capability grants | Description |
|---|---|---|---|
| `capability-broker-pd` | 240 | Holds cross-Totebox caps; grants read-only views to app PDs on request | Chokepoint for all inter-archive access; validated by seL4 capability DAG |
| `watchdog-pd` | 250 | heartbeat cap only | System health monitor; highest priority to survive application-PD failure |
| `network-pd` | 180 | smoltcp VirtIO-net | Ingress/egress; TLS termination for external requests |
| `app-orchestration-slm-pd` | 150 | cap granted by capability-broker-pd on authenticated request | Yo-Yo broker; accesses Doorman endpoints via broker-granted caps only |
| `app-orchestration-gis-pd` | 140 | cap granted by capability-broker-pd | GIS federation; no direct Totebox capability |
| `app-orchestration-bim-pd` | 130 | cap granted by capability-broker-pd | BIM aggregation; no direct Totebox capability |
| `app-orchestration-market-pd` | 120 | cap granted by capability-broker-pd | Marketplace; read-only capability grants |
| `app-orchestration-exchange-pd` | 110 | cap granted by capability-broker-pd | Ad campaigns; read-only capability grants |

**Capability-broker PD responsibilities:**

- Holds the set of seL4 capabilities representing cross-Totebox RPC endpoints.
- On startup, reads a signed capability manifest (signed by the Totebox operator key).
- On per-request basis: validates the requesting PD's identity, checks the capability manifest
  for the requested scope, issues a bounded read-only capability to the requesting PD.
- Maintains a local audit record (WORM-compatible append log) of every capability grant.
- Does not forward write capabilities under any circumstances. If a write is needed, it routes
  through the Totebox's own ingress PD (service-fs Ring 1 boundary).

The capability-broker PD is the single place where the capability geometry invariant (§5)
is enforced in code. All other PDs trust only what the broker provides.

---

## §5 — Capability Geometry at the Federation Layer

Geometric protection in `os-orchestration` has two layers:

**Layer 1 — seL4 kernel enforcement (intra-OS):**
No capability path exists from any `app-orchestration-*` PD to a Totebox archive's
internal services unless the capability-broker PD explicitly grants one. This is proved
by the seL4 kernel: a PD cannot invoke a capability it does not hold, and the kernel
enforces this without exception. A compromised `app-orchestration-slm-pd` cannot reach
`service-fs` in any connected Totebox. The proof is structural, not audited.

**Layer 2 — per-org data isolation (inter-Totebox):**
When the capability-broker PD holds capabilities to multiple Totebox archives (representing
different organizations), it enforces per-org scope on every capability grant. A request
from `app-orchestration-market-pd` on behalf of Org A cannot receive a capability grant
scoped to Org B's Totebox, even if both are registered with the same orchestration
instance. The broker's capability manifest explicitly encodes the allowed scope pairs.

This two-layer structure means the capability geometry guarantee for os-orchestration is:

> Per-org data cannot reach another org. A compromised application PD cannot read another
> org's Totebox data. Both claims are enforced by the seL4 capability DAG, not by policy
> logic in application code.

The J2 capability-geometry paper (ASPLOS target) provides the formal substrate proof;
J5 (totebox-orchestration, MLSys target) extends to session-level capability-secured
orchestration. The capability-broker PD design described here is intended as a future
implementation target referenced in J7 §7 (Limitations), where seL4 protection domains
are described as the mechanism that would eliminate residual OS-process-isolation trust.

---

## §6 — "We Own It" Tier Table

| Tier | Component | License | Notes |
|---|---|---|---|
| Tier 1 — ours | `moonshot-toolkit` | LicenseRef-PointSav-Proprietary | System image builder; v0.3.1, 35 tests |
| Tier 1 — ours | `moonshot-hypervisor` | LicenseRef-PointSav-Proprietary | Hypervisor substrate (Scaffold-coded) |
| Tier 1 — ours | `moonshot-sel4-vmm` | LicenseRef-PointSav-Proprietary | ~300 LOC PD runtime (fill-in phase) |
| Tier 1 — ours | `system-core` v1.0.0 | LicenseRef-PointSav-Proprietary | Capability types, Merkle proofs, CBOR serialization |
| Tier 1 — ours | `system-ledger` v1.0.0 | LicenseRef-PointSav-Proprietary | LedgerConsumer trait + InMemoryLedger |
| Tier 1 — ours | all `app-orchestration-*` crates | LicenseRef-PointSav-Proprietary | Application PDs; commercial boundary |
| Tier 1 — ours | all `service-*` crates | LicenseRef-PointSav-Proprietary | Totebox service ring; Apache 2.0 for base tier |
| Tier 2 — vendored | `vendor-sel4-kernel` | BSD-2-Clause | Formally verified; Isabelle/HOL proofs |
| Tier 2 — vendored | `smoltcp` | MIT | Userspace TCP/IP for network-pd |
| **Rejected** | `rust-sel4` | — | Use `moonshot-sel4-vmm` instead; architectural authority stays in-house |
| **Rejected** | nanos | — | Commercial license incompatible |
| **Rejected** | Unikraft | — | External architecture; would import external design decisions |

---

## §7 — Phase Plan

Phases are intended milestones; all forward-looking claims carry "planned/intended/may/target" status.

**Phase O0 — Scaffold stabilisation (current)**
- `app-orchestration-slm` MVP scaffold is wired; 15 tests passing; Stage 6 promotion pending.
- `os-interface` directory represents the intended `os-orchestration` binary target (rename in flight).
- Other `app-orchestration-*` directories are Reserved-folder or Scaffold-coded (research phase for bim).

**Phase O1 — Yo-Yo broker production hardening (intended next)**
- Implement Phase 2 endpoints: `/v1/graph/federated`, `/v1/training/schedule`, `/v1/adapters`, `/v1/audit/rollup`.
- Replace `tier_b_subscribed` self-attestation in `RegistrationRequest` with a signed membership token (Ed25519, issued by `tool-wallet keygen`).
- Integration test: two Doorman instances registering with the chassis, routing inference, and logging per-tenant metering.
- Stage 6 promotion from project-data to canonical.

**Phase O2 — capability-broker PD scaffold (intended)**
- Write `capability-broker-pd` as a `moonshot-sel4-vmm` PD entry point.
- Capability manifest format: CBOR-encoded, operator-signed, loaded at PD startup.
- Stub grant/revoke API for peer PDs.
- Unit tests for per-org scope enforcement logic.

**Phase O3 — os-orchestration base image (intended)**
- `moonshot-toolkit` system spec at `examples/os-orchestration.toml`.
- PD inventory: watchdog-pd + network-pd + capability-broker-pd + app-orchestration-slm-pd.
- QEMU dev boot on foundry-workspace (TCG acceptable for initial validation).
- Confirm stateless invariant: no disk write paths reachable from any application PD.

**Phase O4 — GIS federation activation (intended)**
- `app-orchestration-gis` receives first implementation: federated spatial query across two Totebox archives.
- Capability-broker PD issues read-only GIS data caps to the gis PD.
- Integration: `gateway-orchestration-gis-1` deployment validated.

**Phase O5 — BIM aggregation activation (intended)**
- Gated on `moonshot-bim-engine` and `app-privategit-bim` reaching Active state.
- `app-orchestration-bim-pd` receives IFC clash detection and portfolio query endpoints.

**Phase O6 — Market and Exchange surfaces (intended, Leapfrog 2030)**
- `app-orchestration-market` and `app-orchestration-exchange` activated as read-only PDs.
- Doctrine claim #52 surfaces (browser marketplace + ad campaign UI).

---

## §8 — system-* Prerequisites

The following `system-*` components must reach the stated milestone before os-orchestration
can advance beyond Phase O2:

| Prerequisite | Required state | Gating phase | Current state |
|---|---|---|---|
| `system-core` v1.0.0 | Dual `std`/`sel4` feature flags working; CBOR deterministic hashing | Phase O2 | Complete |
| `system-ledger` v1.0.0 | `LedgerConsumer` trait + `apply_witness_record` (InclusionProof-verified) | Phase O2 | Complete |
| `moonshot-sel4-vmm` | PD runtime (~300 LOC) compilable against `vendor-sel4-kernel` | Phase O2 | Fill-in phase |
| `moonshot-toolkit` | `examples/os-orchestration.toml` system spec accepted by builder | Phase O3 | v0.3.1 ready; spec file not yet written |
| `system-security` | `watchdog-pd` heartbeat protocol defined | Phase O2 | Scaffold-coded |
| `system-gateway-mba` | MBA pairing protocol for `capability-broker-pd` external registration | Phase O4 | Scaffold-coded |
| seL4 Microkit SDK | AArch64 Microkit 2.2.0 available on build host | Phase O3 (bare metal path) | Blocked on operator hardware acquisition decision |

**Architecture decision open (required before Phase O3 bare-metal path):**
Operator must choose Option A (AArch64 GCP, ~$50–100/month) or Option B (Firecracker x86_64
on Laptop A, KVM-native, 125ms boot) as the Phase 3 seL4 target. The x86_64 QEMU TCG path
unblocks Phase O0–O2; the hardware decision gates Phase O3 bare-metal.

---

## §9 — Commercial Model (Doctrine #23)

Doctrine claim #23 defines the product boundary:

> **Solo Totebox (Tier A):** Local OLMo 1B/7B on `hardware` or `accelerated` node. Free.
> **Multi-Totebox via orchestration chassis (Tier B):** Shared Yo-Yo fleet brokered through
> `app-orchestration-slm`. Paid.

The same boundary applies to all `app-orchestration-*` surfaces:

| Layer | Tier | Cost model |
|---|---|---|
| `service-slm` (Doorman, local OLMo) | Tier A | Included in base product; Apache 2.0 |
| `service-content`, `service-people`, `service-fs`, `service-email` | Tier A | Included in base product; Apache 2.0 |
| `app-orchestration-slm` (Yo-Yo broker chassis) | Tier B | Paid; LicenseRef-PointSav-Proprietary |
| `app-orchestration-gis`, `-bim`, `-market`, `-exchange` | Tier B | Paid; LicenseRef-PointSav-Proprietary |
| Capability-broker PD | Infrastructure | Bundled with Tier B subscription; not separately priced |

The Ed25519 license token (issued by `tool-wallet keygen`, verified by `orchestration-slm/src/license.rs`)
is the enforcement mechanism at the chassis layer. Totebox archives connecting without a valid
token receive HTTP 402 on all Yo-Yo proxy endpoints.

Phase 2 of the commercial model (intended): signed membership tokens replace the MVP
`tier_b_subscribed` self-attestation field. The token encodes `module_id`, `issued_to`,
`expiry`, and `product` (`app-orchestration-slm-v1`), signed by the PointSav license key.

SYS-ADR-19 applies: no automated AI publishing to verified ledgers. The orchestration chassis
may route inference requests and log metering, but it does not write to any Totebox WORM
ledger autonomously. All WORM writes remain inside the Totebox's own `service-fs` PD.

---

## §10 — Current State (app-orchestration-slm scaffold; other Reserved-folder apps)

**app-orchestration-slm — MVP scaffold complete:**

- Location: `/srv/foundry/clones/project-data/app-orchestration-slm/`
  (standalone 3-crate workspace; not yet inside `pointsav-monorepo`).
- Crates: `orchestration-slm-core` (wire types), `orchestration-slm` (business logic),
  `orchestration-slm-server` (axum HTTP entry point).
- Bind port: `:9180` (configurable via `ORCHESTRATION_BIND_ADDR`).
- MVP endpoints implemented: `/healthz`, `/readyz`, `/v1/fleet`, `/v1/discovery/register`,
  `/v1/yoyo/proxy`, `/v1/yoyo/trainer`, `/v1/yoyo/graph`.
- Phase 2 endpoints **implemented (session-11):**
  - `POST /v1/graph/federated` — fans out `q` to each registered Doorman's `/v1/query`; 10s timeout per archive; `archives_queried` vs `archives_reachable` summary.
  - `POST /v1/training/schedule` — proxies LoRA training job to Yo-Yo trainer `/v1/training/jobs`; returns `job_id`; 503 if trainer not configured.
  - `GET /v1/adapters` — queries trainer + graph nodes `/v1/adapters`; merges with `node_label`.
- Phase 2 membership tokens **implemented (session-11):**
  - `orchestration-slm/src/membership.rs` — `MembershipKey::generate()` (32-byte OS entropy), `MembershipClaims`, `issue()` (1-hour Ed25519 token), `verify()` (signature + expiry).
  - `POST /v1/discovery/register` now returns `RegistrationResponseV2` with `membership_token: Some(...)`.
  - `fleet.rs` — added `list_full()` returning `Vec<FleetMember>` with `doorman_endpoint` for federation fanout.
- Phase 2 still pending: `/v1/audit/rollup` is wired to route but returns empty rollup (placeholder).
- License: `LicenseRef-PointSav-Proprietary`.
- Stage 6 promotion: pending (included in the batch of commits awaiting Command Session promote).

**Yo-Yo fleet nodes (existing; do not provision new):**

| Label | Node | Hardware | Model |
|---|---|---|---|
| `proxy` | Default (either) | — | General inference |
| `trainer` | Yo-Yo #1 | L4 24GB | OLMo 3 32B-Think |
| `graph` | Yo-Yo #2 | H100 80GB | Llama 3.3 70B grammar-constrained |

**app-orchestration-bim — Research phase:**
- Location: `/srv/foundry/clones/project-data/app-orchestration-bim/`
- Files: `CLAUDE.md`, `RESEARCH.md`, `Cargo.toml`, `src/` — Scaffold-coded.
- Architecture defined in `RESEARCH.md`: stateless, cross-archive IFC clash detection and
  portfolio BIM queries. Commercial boundary follows the platform pattern (service-bim Apache 2.0;
  app-orchestration-bim proprietary).

**app-orchestration-gis — Reserved-folder:**
- Deployed as `gateway-orchestration-gis-1` (deployment exists; app crate implementation pending).

**app-orchestration-market, app-orchestration-exchange — Reserved-folder:**
- READMEs only; Leapfrog 2030 targets; implementation not yet started.

**os-interface (intended os-orchestration):**
- Location: `/srv/foundry/clones/project-data/os-interface/`
- Contains `src/lib.rs`, `README.md`, `README.es.md`, `scripts/` — Scaffold-coded.
- README describes it as "Aggregation Gateway Environment — Status: Active Engineering".
- Rename to `os-orchestration` is in flight per project-registry.md.

---

## §11 — Decisions Locked

1. **os-orchestration is stateless.** It holds no archive keys, writes no WORM ledger,
   and owns no persistent data. Any code path that writes to a Totebox archive from an
   orchestration PD is a bug, not a feature.

2. **Capability-broker PD is mandatory before commercial application PDs are activated.**
   No `app-orchestration-*` PD may hold a direct seL4 capability to a Totebox service.
   All cross-Totebox access routes through the capability-broker PD.

3. **The Yo-Yo fleet nodes are fixed.** Two named nodes: `trainer` (L4 24GB, OLMo 3 32B-Think)
   and `graph` (H100 80GB, Llama 3.3 70B). Do not provision additional Yo-Yo VMs without
   explicit operator decision.

4. **Ed25519 license tokens are the commercial boundary enforcement mechanism.** Phase 2
   replaces the MVP `tier_b_subscribed` self-attestation; no interim soft enforcement is planned.

5. **seL4 Microkit 2.2.0 is AArch64-first.** No x86_64 Microkit target. The x86_64 QEMU TCG
   path is valid for Phase O0–O2 development; Phase O3 bare-metal requires a hardware decision.

6. **`moonshot-sel4-vmm` is the PD runtime.** `rust-sel4` is rejected. All PD entry points
   and IPC wiring use the in-house ~300 LOC PD runtime.

7. **`app-orchestration-slm` lives outside `pointsav-monorepo` as a standalone workspace
   for now.** Unification into the monorepo workspace is a future cleanup item, not a
   current blocker.

---

## §12 — Decisions Open

1. **AArch64 vs x86_64 Firecracker for Phase O3 bare metal.**
   Option A: AArch64 GCP instance (~$50–100/month); seL4 Microkit 2.2.0 native.
   Option B: Firecracker x86_64 on Laptop A (KVM-native, ~125ms boot).
   Operator decision required before Phase O3 begins.

2. **Capability manifest format: CBOR vs signed JSON.**
   `system-core` uses ciborium CBOR for canonical serialization; capability manifests should
   follow the same convention for hash determinism. Confirm before Phase O2 implementation.

3. **Per-org scope enforcement granularity.**
   Does the capability-broker PD enforce scope at the Totebox-archive level (one cap per archive)
   or at the service level within a Totebox (one cap per service-* endpoint)?
   Service-level granularity provides tighter isolation; archive-level is simpler to implement.
   Decision needed before capability-broker PD is implemented.

4. **`app-orchestration-slm` monorepo unification.**
   The crate currently lives in a standalone 3-crate workspace at the project-data root.
   Unification into `pointsav-monorepo` as workspace members would align it with `service-vm-*`
   and other active service crates. Timing and whether this happens before or after Stage 6 is open.

5. **`os-interface` rename commit.**
   The rename from `os-interface` to `os-orchestration` is in flight but the commit has not landed.
   Should this be a standalone rename commit or bundled with the first Phase O1 code change?

6. **GIS federation data contract.**
   `app-orchestration-gis` is deployed as `gateway-orchestration-gis-1` but the crate
   implementation is pending. The wire format for cross-Totebox GIS queries (H3 tiles? GeoJSON
   FeatureCollections? WFS?) is not yet specified.

---

## §13 — JOURNAL Tie-In

os-orchestration build-out directly gates two journal papers:

**J5 — JOURNAL-totebox-orchestration (MLSys, ACM, 22% AR):**
- Current state: v0.3, 2,614 of 9,500 target words.
- Sections 4 (Implementation), 5 (Evaluation), 6 (Discussion), and 8 (Conclusion) are stubs.
- §4 Implementation evidence comes from the os-totebox and os-orchestration reference deployment.
  Each Phase O milestone should be documented in J5 §4 as implementation evidence accumulates.
- §5 Evaluation requires a benchmark harness: concurrent-session isolation test results and
  inference latency measurements through the Yo-Yo broker.
- References section is entirely unpopulated. J2 is the mandatory companion citation
  ([CITATION NEEDED — J2] appears throughout J5 §3.3 and §7).
- J5 cannot be submitted until J2 is published or posted as a citable preprint.
  J2 preprint is already posted (2026-05-28); J2 submission is blocked on Bench #9 re-run.

**J2 — JOURNAL-capability-geometry (ASPLOS, ACM, 19.4% AR):**
- Current state: v0.2, 8,650 of 9,000 target words. All sections complete.
- Blocker: Bench #9 (verify_inclusion_proof, 1,024-leaf, quiet-VM re-run — 22 outliers at ±11% CI).
- J2 §7.7 (Limitations) references seL4 protection domain isolation as a planned future direction;
  the capability-broker PD design in §4 of this BRIEF is the intended concrete implementation.

**Update discipline:**
After each Phase O milestone, update J5 §4 with implementation evidence. Do not leave §4 as
a stub once the first os-orchestration PD is running. Stage the updated manuscript to
`.agent/drafts-outbound/` routed to project-editorial before external submission.

---

## §14 — Work Log

| Date | Session | Work completed |
|---|---|---|
| 2026-06-19 | totebox@project-data | BRIEF created; research gathered from project-infrastructure, project-system, project-data-journals, project-orchestration, and project-data-code subagents; full architecture documented. |

---

## §15 — Carry-Forward

**Actionable in project-data (Totebox scope):**

- [ ] Phase O1: implement Phase 2 endpoints in `app-orchestration-slm`
  (`/v1/graph/federated`, `/v1/training/schedule`, `/v1/adapters`, `/v1/audit/rollup`).
- [ ] Phase O1: replace `tier_b_subscribed` self-attestation with signed Ed25519 membership token.
- [ ] Phase O1: integration test — two Doorman instances registering, routing, and metering.
- [ ] Phase O2: scaffold `capability-broker-pd` as a `moonshot-sel4-vmm` PD entry point.
- [ ] Phase O2: define CBOR capability manifest format; confirm with operator.
- [ ] Phase O3: write `examples/os-orchestration.toml` for `moonshot-toolkit`.
- [ ] J5 §4: update JOURNAL-totebox-orchestration Implementation section after Phase O1 lands.
- [ ] J5 §5/§6/§8: draft Evaluation, Discussion, Conclusion stubs after benchmark harness is available.
- [ ] Rename commit: `os-interface/` → `os-orchestration/` (or confirm Command Session handles this).
- [ ] Update `.agent/briefs/README.md` to include this BRIEF in the active-briefs table.

**Route to Command Session (outbox):**

- [ ] Stage 6 promotion for `app-orchestration-slm` commits (batch with pending project-data commits).
- [ ] Hardware decision: AArch64 GCP vs Firecracker x86_64 for Phase O3 (operator action required).
- [ ] vm-intelligence launch (Part C Step C1) — gated on WireGuard Part A + app-orchestration-command v0.0.1.
- [ ] Project-orchestration archive: resolve manifest merge conflict and NEXT.md contamination
  (M-17 scope; Command Session).
