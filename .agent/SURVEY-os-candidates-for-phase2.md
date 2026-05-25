---
schema: foundry-survey-v1
created: 2026-04-27
author: Task Claude (project-system cluster, forward-prep sub-agent)
purpose: Phase 2 first-port candidate selection — os-* scaffold audit
---

# Survey — os-* Candidates for Phase 2 (NetBSD Compat-Bottom Prototype)

## 1. Methodology

This survey informs the Phase 2 first-port decision for Doctrine claim #34 (Two-Bottoms
Sovereign Substrate). It is a scaffold-state audit, not a quality audit. Phase 2
deliverable shape: one `os-*` binary that runs on the workspace-VM Linux development
environment AND on a NetBSD AArch64 compat-bottom instance; capability primitives from
`system-core` compose identically across both bottoms via a thin shim layer. The
assessment criteria are (a) scaffold completeness sufficient to reduce Phase 2 bootstrap
cost, (b) existing coupling to `system-core` capability-primitive types, and (c)
customer-demo visibility on both bottoms. Binary artefacts (ISO/IMG) in any scaffold are
treated as informative pre-existing build output, not as blockers or quality signals.

---

## 2. Per-`os-*` Profile

**`os-console` (9 files)**
- Scaffold-coded; NOT a workspace member (`cargo check` fails: "believes it's in a
  workspace when it's not"); `Cargo.toml` declares `edition = "2024"`, no deps, `src/main.rs`
  is `println!("Hello, world!")` — Rust entry point skeleton only.
- Subtree is substantive: `engine/forge_chassis.py` is a working Python HTML/JS generator
  for the operator chassis UI (F-key ribbon, iframe cartridge viewport, MBA auth header);
  `relay/os-console-relay.py` is a live MBA-relay HTTP server on `127.0.0.1:3000` that
  signs outbound payloads to a vault target via Ed25519 (hardcoded iMac target — deployment
  artefact, not portable); `relay/wire-ui-relay.sh` + `engine/state-projector.py` + `scripts/`
  round out the non-Rust layer.
- Capability-primitive exercise potential: MODERATE-HIGH — the relay references Ed25519
  signing and a "vault" target structurally consistent with the Capability Ledger flow, but
  uses a pre-`system-core` Python/shell implementation rather than `system-core` types
  directly. No Rust import of `system_core::Capability` or `SignedCheckpoint` today.
- Surface area: ConsoleOS per Nomenclature Matrix §2 — "lightning-fast State Manager";
  operator-facing chassis that hosts `app-console-*` cartridges via F-key strikes. Claim
  is well-supported by the engine subtree.
- Phase 2 fit: STRONG — the Rust binary (`src/main.rs`) needs Phase 2 content added, and
  the compat-bottom demo would show the same console binary mediating capability handshakes
  on Linux and NetBSD. Console is operator-facing: the demo is tangible to a customer
  watching it live.

**`os-infrastructure` (20 files)**
- Scaffold-coded; NOT a workspace member; `Cargo.toml` depends on `system-substrate-broadcom`
  and `system-network-interface` (both path deps to sibling scaffolds that are also not
  workspace members — transitive workspace-exclusion failure).
- `src/main.rs` is `#![no_std] #![no_main]` with Multiboot2 header, inline assembly
  bootloader, VGA framebuffer pixel-writer, and calls to `silicon_ping()` /
  `enable_monitor_mode()` / `hunt_for_eapol()` — i.e., Broadcom NIC EAPOL-packet extraction
  at bare-metal. `build_iso/` contains pre-built ELF binaries, a compiled ISO (17.9 MB
  tracked), and a shell-based ISO assembly pipeline (`compile_binary.sh`, `linker.ld`).
- Capability-primitive exercise potential: LOW — the scaffold is hardware-specific
  (Broadcom silicon, iMac 12,1 target, x86-only inline asm). No reference to `system-core`
  types; the "substrate" reference is a compiled ELF binary in `build_iso/`, not a Rust
  import. The `no_std` posture means adding `system-core` (which uses `serde_json`, `sha2`)
  would require feature-gating the entire dependency tree — non-trivial.
- Surface area: Nomenclature Matrix §2 describes this as "lightweight universal Bootstrapper
  — prepares host environment to run the Totebox." Actual code is a bare-metal NIC monitor;
  the Bootstrapper claim does not match the EAPOL-extraction implementation.
- Phase 2 fit: WEAK — Broadcom-specific hardware deps and x86 inline ASM are not portable
  to AArch64 NetBSD without a full rewrite. The ISO artefact (17.9 MB tracked, tracking
  status TBD) is a git hygiene blocker that must be resolved before Phase 2 work begins.

**`os-interface` (4 files) — canonical name `os-orchestration`, rename in flight**
- Scaffold-coded; NOT a workspace member; `Cargo.toml` has no deps; `src/lib.rs` is a
  single-function scaffold stub returning a status string. Minimal scaffolding only.
- Capability-primitive exercise potential: NONE today — pure doc-level placeholder.
- Surface area: README calls it "Aggregation Gateway Environment" / "manages connections
  between operator console and multiple underlying data environments." Matches Nomenclature
  Matrix §2 `os-orchestration` role (Totebox aggregation + extended compute). Name in flux
  adds friction: the rename must land before Phase 2 work can commit under the canonical
  name.
- Phase 2 fit: LOW — thinnest scaffold of all eight; pending rename adds a sequencing
  dependency that doesn't affect other candidates.

**`os-mediakit` (4 files)**
- Scaffold-coded; NOT a workspace member; `Cargo.toml` has no deps; `src/lib.rs` is a
  single-function scaffold stub.
- Capability-primitive exercise potential: NONE today.
- Surface area: README claims "autonomous edge node pattern — bundles presentation screen
  and local intelligence into a single vault." Matches Nomenclature Matrix §2 `os-mediakit`
  role (JeOS for corporate disclosure / digital marketing).
- Phase 2 fit: LOW — the edge-delivery use case is customer-visible (MediaKit nodes are
  public-facing), but the Phase 2 compat-bottom demo is most compelling at the
  operator/developer-tool tier where the Capability Ledger interaction is visible. MediaKit
  Phase 2 value comes after the ledger layer is demonstrated elsewhere first.

**`os-network-admin` (12 files)**
- Scaffold-coded; NOT a workspace member; `Cargo.toml` declares no deps (comment says "no
  heavy deps for basic UDP telemetry"); `src/main.rs` is a live UDP socket loop pinging
  `10.0.0.101:5000` with `GET_VITALS` — a hardcoded iMac/LAN target. `scripts/` contains
  `forge_admin_iso.sh` + `mesh_status.sh`; `build_iso/` has an ISO artefact (tracking
  status TBD); `public/` has `index.html` + `mesh-state.json`.
- Capability-primitive exercise potential: LOW — no `system-core` import; UDP telemetry is
  the entire runtime. README is explicit: "Zero Cryptographic Authority — handles packet
  routing and tunnel integrity only."
- Surface area: Nomenclature Matrix §2 names this `os-network-admin` / "Orchestrates VMs,
  handles private network routing, authorizes provisioning." Code is a UDP poll loop; the
  routing claim is not supported by the implementation.
- Phase 2 fit: WEAK — hardcoded LAN target and ISO artefact blocker. Network-admin role is
  infrastructure-tier (operator-invisible to a customer demo). If the demo story is
  "capability ledger secures a network routing decision," this is the right crate — but
  that story requires more Phase 2 implementation lift than a console or totebox demo.

**`os-privategit` (4 files)**
- Scaffold-coded; NOT a workspace member; `Cargo.toml` has no deps; `src/lib.rs` is a
  single-function scaffold stub. Nomenclature Matrix §2: `os-privategit` / `vault-` — "The
  independent code repository and version control."
- Capability-primitive exercise potential: NONE today.
- Phase 2 fit: MODERATE for developer-audience demo — a private git server guarding
  repository access via capability primitives is a credible developer-facing story. But the
  scaffold is at the minimum possible state; Phase 2 lift would be almost entirely net-new.

**`os-totebox` (6 files)**
- Scaffold-coded; NOT a workspace member; `Cargo.toml` declares `edition = "2024"`, no deps;
  `src/lib.rs` is a cargo-new default (the `add(left, right)` stub with the default test).
  `scripts/totebox-launcher.sh` exists; `os-totebox-release.img` (IMG artefact, tracking
  status TBD) is present.
- Capability-primitive exercise potential: LOW today, but HIGH structurally — ToteboxOS is
  described in `system-substrate-doctrine.md` §7 as the specific OS that boots the
  Boot-Anywhere Capability Recovery path (customer enters seed → Totebox Archive ISO →
  ledger replay). The doctrine explicitly names it in the Phase 2 / Mechanism C flow.
- Surface area: README claims "isolated container for organizational ledgers and corporate
  assets — verifiable flat-file ledgers, cryptographic checksums." Claim is consistent with
  the doctrine role; IMG artefact (tracking status TBD) must be addressed.
- Phase 2 fit: HIGH by doctrine position — the Boot-Anywhere Capability Recovery flow
  (Mechanism C, §7 of `system-substrate-doctrine.md`) explicitly boots the Totebox Archive
  ISO on commodity NetBSD hardware. Phase 2 is *the* proving ground for this path. Demo
  would show recovery from paper seed on NetBSD, ledger replay, capability state
  reconstituted — end-user-facing and structurally the capstone of the compat-bottom claim.

**`os-workplace` (4 files)**
- Scaffold-coded; NOT a workspace member; `Cargo.toml` has no deps; `src/lib.rs` is a
  single-function scaffold stub. Nomenclature Matrix §2: "State Projection / Desktop
  Environment — primary rendering agent."
- Capability-primitive exercise potential: NONE today.
- Phase 2 fit: LOW — rendering layer; meaningful only after lower-tier `os-*` primitives
  are working on both bottoms.

---

## 3. Comparison Matrix

| Candidate | Scaffold maturity | Workspace member? | Cargo.toml? | Uses `system-core`? | Phase 2 demo visibility | Recommended? |
|---|---|---|---|---|---|---|
| os-console | Moderate (Python/shell subtree active; Rust is hello-world) | ✗ | ✓ | ✗ (Ed25519 relay in Python) | HIGH — operator-facing | Primary |
| os-infrastructure | Moderate (no_std bare-metal builds; ISO in-tree) | ✗ | ✓ (path deps broken) | ✗ | LOW — hardware-specific; AArch64 rewrite needed | ✗ |
| os-interface (os-orchestration) | Minimal (stub only) | ✗ | ✓ (no deps) | ✗ | ⚠ pending rename | ✗ |
| os-mediakit | Minimal (stub only) | ✗ | ✓ (no deps) | ✗ | ⚠ edge-delivery; post-ledger demo | ✗ |
| os-network-admin | Low (UDP poll loop; hardcoded LAN) | ✗ | ✓ (no deps) | ✗ | LOW — infra-tier, customer-invisible | ✗ |
| os-privategit | Minimal (stub only) | ✗ | ✓ (no deps) | ✗ | ⚠ developer-facing; scaffold too thin | ✗ |
| os-totebox | Minimal (cargo-new stub; IMG in-tree) | ✗ | ✓ (no deps) | ✗ | HIGH — doctrine-named compat-bottom boot | Backup |
| os-workplace | Minimal (stub only) | ✗ | ✓ (no deps) | ✗ | LOW — rendering layer; depends on lower tiers | ✗ |

All eight fail `cargo check` at the workspace level with "believes it's in a workspace when
it's not" — none are declared in the root `Cargo.toml` `[workspace] members` array. This is
a universal Phase 2 precondition: whichever candidate is chosen must be added to workspace
members before Phase 2 work begins.

---

## 4. Recommendation

**Primary: `os-console`**
**Backup: `os-totebox`**

**Primary justification (`os-console`).**

`os-console` has the most credible existing scaffold of the eight candidates. While its Rust
entry point is a hello-world stub, the `engine/` and `relay/` subtrees contain working Python
implementations of the MBA-relay signing protocol and the operator chassis UI — evidence that
the intended architecture (capability-authenticated relay + F-key cartridge system) was
explored and is understood. The relay's Ed25519 signing stub is a direct precursor to
`system-core::SignedCheckpoint` consumption; a Phase 2 port replaces the Python relay with a
Rust binary that calls `system-core` types, running on both Linux dev and NetBSD AArch64. The
demo is operator-facing: an operator can watch the console binary authenticate a capability
handshake across both bottoms in real time. The Nomenclature Matrix §2 and `system-substrate-
doctrine.md` §8 ("Userland / applications: `os-*` / `app-*`") both confirm ConsoleOS as a
first-tier delivery surface. The `os-console` Rust binary has no platform-specific assembly,
no hardware-vendor path deps, and no in-tree binary artefacts — the three disqualifiers that
eliminate `os-infrastructure` and `os-network-admin`.

**Backup justification (`os-totebox`).**

`os-totebox` is the candidate most directly named in the doctrine as the compat-bottom boot
vehicle: `system-substrate-doctrine.md` §7 (Mechanism C — Boot-Anywhere Capability Recovery)
explicitly boots the Totebox Archive ISO on borrowed NetBSD hardware. If the operator's
priority is demonstrating the recovery-from-paper-seed flow rather than the operator-chassis
flow, `os-totebox` is the natural choice. Its scaffold is thinner than `os-console` (cargo-new
default stub vs. working Python subtree), meaning more Phase 2 lift is required on the
implementation side. The IMG artefact (`os-totebox-release.img`, tracking status TBD) must be
resolved before Phase 2 work begins. Despite the thinner scaffold, the doctrine alignment is
strong enough that an operator with a customer-demo preference for the recovery story should
override toward `os-totebox`.

Both candidates require the same Phase 2 precondition: add the chosen crate to workspace
`[members]`, confirm `cargo check` passes on Linux, then begin porting the Rust binary to
build for NetBSD AArch64 (cross-compilation config + `system-core` path dep). The operator
may have preference reasons — audience type, demo venue, which capability primitive flow to
emphasize — that justify choosing `os-totebox` over `os-console`. This survey recommends
`os-console` as primary on the basis of scaffold completeness and demo breadth.

---

## 5. Open Questions for Master / Operator

1. **Customer-demo audience preference.** `os-console` is operator-facing (the administrator
   chassis); `os-totebox` is end-user-facing with the recovery-from-seed story;
   `os-privategit` is developer-facing. Does the operator have a target demo audience or
   scenario (e.g., a specific customer conversation, a regulatory showcase, a developer
   onboarding demo) that should override the scaffold-completeness ranking?

2. **IMG / ISO artefact tracking status.** `os-totebox-release.img`, `os-network-admin`'s ISO
   artefact, and `os-infrastructure`'s two ISOs (one in the crate root, one in `build_iso/`)
   have "tracking status TBD" per the registry. Are these gitignored from the cluster branch
   or tracked? If tracked, are they rebuild-from-source candidates for Phase 2, or are they
   historical scaffolding artefacts to be removed before Phase 2 work begins? The answer
   determines whether `os-totebox` is an unencumbered backup or carries a prerequisite
   cleanup step.

3. **`system-substrate-broadcom` / `system-network-interface` path dep chain.** `os-infrastructure`
   depends on both sibling scaffolds, which are themselves not workspace members. If
   `os-infrastructure` is ever considered for Phase 2 (not recommended in this survey), all
   three crates need workspace declaration and the x86 inline ASM needs AArch64 replacements.
   Is the Broadcom silicon target still a live deployment goal, or is `os-infrastructure`
   effectively superseded by `moonshot-toolkit`'s seL4 build path for the native bottom?

4. **Workspace membership precondition sequencing.** All eight `os-*` crates are excluded from
   the workspace `[members]` array. Phase 2 requires adding at least the chosen candidate.
   Should the Task session add only the chosen candidate, or batch-add all eight `os-*`
   crates (and their transitively-broken path deps) as part of the Phase 0 workspace-
   unification work that precedes Phase 2? The cleanup-log (2026-04-18 Layer 1 audit) flags
   workspace under-declaration as a Critical finding; the scope of the Phase 2 precondition
   step depends on this decision.

---

## 6. References

**Nomenclature Matrix V8** (relevant sections):
- §2 Systemic Wordmarks table: rows for ConsoleOS, InfrastructureOS, MediaKitOS, NetworkOS,
  PrivateGitOS, WorkplaceOS, OrchestrationsOS, ToteboxOS — canonical crate prefixes and roles
- §3 `os-*` taxonomy row: "Foundation — Microkernel boundaries and infrastructure execution.
  Operates strictly below the application layer."
- §3 `system-*` taxonomy row: "Dependency root for every `os-*` module."
- §5 rename note: "`os-interface` → `os-orchestration` in progress"

**system-substrate-doctrine.md** (relevant sections):
- §2 The two-bottoms shape: NetBSD compat-bottom definition; "same `os-*` binaries run on
  either via a thin shim"
- §7 Mechanism C — Boot-Anywhere Capability Recovery: explicit Totebox Archive ISO on NetBSD
  compat-bottom boot sequence
- §8 "We Own It" scoresheet: "Userland / applications: `os-*` / `app-*` / `service-*` under
  Apache 2.0 + BSD compat" — confirms `os-*` as the owned layer
- §11 Implementation order: Phase 2 listed as "NetBSD compat-bottom prototype — install on
  test instance, verify Veriexec, rump kernel exploration"

**project-registry.md** OS section (2026-04-27):
- All eight `os-*` crates: `Scaffold-coded`
- `os-interface` note: "4 files; legacy name — canonical is `os-orchestration` (rename in flight)"
- `os-infrastructure` note: "20 files; ISO artefact in directory — tracking status TBD"
- `os-network-admin` note: "12 files; ISO artefact — tracking status TBD"
- `os-totebox` note: "6 files; IMG artefact — tracking status TBD"
