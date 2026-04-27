# ARCHITECTURE — moonshot-toolkit

Part of `cluster/project-system` per workspace `PROJECT-CLONES.md`.

The Rust-only build orchestrator for Foundry's seL4 unikernel
images per MEMO §7 ("Microkit (Python/CMake) → moonshot-toolkit
(Rust-Only Toolchain)") and convention `system-substrate-doctrine.md`
§6 (Reproducible-Verification-On-Customer-Metal — moonshot-toolkit
produces the artefacts that customer re-executes).

Activated 2026-04-27 per framework §9 (Master Option A lean +
operator confirmation).

---

## 1. Scope

`moonshot-toolkit` is the build orchestrator. It owns:

- The `SystemSpec` Rust-native equivalent of Microkit 2.2.0's
  system-description XML (`src/spec.rs`)
- The `BuildPlan` generation: deterministic, content-addressed
  manifest derived from a SystemSpec (`src/plan.rs`)
- The CLI surface: `validate` / `plan` / `build` subcommands
  (`src/main.rs`)
- Future: actual seL4 cross-compile orchestration; reproducible-
  build harness integration; Sigstore Cosign output signing

It does NOT own:

- Cross-compile toolchain (aarch64-linux-gnu-gcc; Rust
  cross-targets) — system dependencies, not Rust code.
- seL4 source — vendored into `vendor-sel4-kernel`.
- The actual binary emission — that's the system tool's
  responsibility; moonshot-toolkit orchestrates, doesn't compile.
- Customer-apex cosignature primitive — that's
  `system-core::checkpoint` (cosign integrates with it; doesn't
  re-implement it).

## 2. Why a Rust-only toolchain (vs Microkit's Python/CMake)

Per MEMO §7 the migration is doctrinal. Three reasons:

1. **Determinism.** Python `dict` ordering is implementation-
   defined; CMake's discovery rules vary across versions. Rust's
   serde + content-addressed inputs gives bit-reproducible plan
   bytes.
2. **Audit-friendly.** A single Rust binary auditable end-to-end
   matches the "honest We Own It scoresheet" discipline (convention
   §8). Python + CMake + makefiles + shell wrappers is a five-
   language audit surface.
3. **No-network at build time.** Convention §6 hermetic-build
   property. Python's `pip install` and CMake's `find_package`
   are network surfaces; Rust + vendored deps eliminates them.

## 3. Module layout

### `src/spec.rs` — SystemSpec data model + TOML parser

Rust struct equivalent of Microkit's system-description XML schema.
Fields:

- `protection_domains: Vec<ProtectionDomain>` (max 63 per Microkit)
  - `name`, `entry_points` (init/notified/protected/fault),
    `priority`, `assigned_memory_regions`, `assigned_channels`
- `channels: Vec<Channel>` — point-to-point PPC or notification;
  max 63 per PD
- `memory_regions: Vec<MemoryRegion>` — caching + permissions +
  optional prefill
- `irq_delivery: Vec<IrqDelivery>` — IRQ → PD mapping

TOML on-disk format (deserialised via `toml = "0.8"` + serde).

Validation rules enforced at parse time:
- ≤ 63 protection_domains
- ≤ 63 channels per PD
- No overlapping memory regions (by address range)
- IRQ targets must reference declared PDs
- Channel endpoints must reference declared PDs

### `src/plan.rs` — BuildPlan generation

Given a parsed SystemSpec, derives a `BuildPlan`:

```rust
struct BuildPlan {
    spec_hash: Hash256,         // SHA-256 of canonical SystemSpec bytes
    input_hashes: Vec<Hash256>, // each declared input file
    steps: Vec<BuildStep>,      // ordered (inputs, command, outputs)
    plan_hash: Hash256,         // SHA-256 of all of the above canonical bytes
}
```

Determinism: same SystemSpec → same input_hashes → same steps →
same plan_hash. Tested via duplicate-spec-same-plan-hash assertion.

The plan is the manifest a reproducible-build harness replays.
v0.1.x ships the plan generator only; actual command execution
(future #14) consumes the plan.

### `src/main.rs` — CLI

clap-based subcommands:

- `validate <spec.toml>` — parse the TOML; reject on invariant
  violation; exit 0 on valid.
- `plan <spec.toml>` — parse + generate BuildPlan + print
  canonical TOML / JSON representation.
- `build <spec.toml>` — parse + plan + STUB execute (prints
  "would run: <command>" for each step; exit 0). Actual
  execution lands in future task #14.

## 4. Hash function

SHA-256 baseline per `worm-ledger-design.md` §3 D3 algorithm-
agility discipline. The `plan_hash` is the value a customer-apex
cosignature commits to per convention §6.1; future MINOR may add
BLAKE3 / SHA-3 alongside SHA-256 (algorithm-agile from day one).

## 5. Cross-references

- `~/Foundry/DOCTRINE.md` §II claim #33 + #34 (constitutional
  anchors)
- `~/Foundry/conventions/system-substrate-doctrine.md` §6
  (Reproducible-Verification-On-Customer-Metal)
- `~/Foundry/MEMO-2026-03-30-Development-Overview-V8.md` §7
  (Microkit → moonshot-toolkit transition)
- Microkit 2.2.0 system-description XML reference at
  `https://docs.sel4.systems/projects/microkit/manual/latest/`
- Sibling crates: `system-core` (Hash256 + cosignature primitives;
  consumed when build outputs become real); `vendor-sel4-kernel`
  (vendored kernel source for cross-compile).

## 6. Verification

Activation commit: `cargo check -p moonshot-toolkit` passes
(legacy stub + framework §9 docs only). 0 tests at activation;
tests land alongside each module impl per cluster tasks #35 / #36
/ #37.

## 7. Future work (out of v0.1.x scope)

Per cluster task #14 (FUTURE session):

- Cross-compile toolchain installation (aarch64-linux-gnu-gcc
  or rustc cross targets)
- seL4 source vendoring strategy (git submodule vs Cargo build.rs
  fetch vs vendor-sel4-kernel snapshot)
- Reproducible-build harness selection (Nix-style content-addressed
  inputs vs Bazel-hermetic)
- Build minimal seL4 hello-world Microkit system using
  moonshot-toolkit (not upstream Microkit), boot in QEMU AArch64
- Sigstore Cosign output signing + customer-apex cosignature
  per convention §6.1 release-artefact format

These decisions are operator-trigger or Master-trigger per the
inbox brief; not auto-actionable from a single Task session.
