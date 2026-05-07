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
  - `name` (String), `binary` (path to PD binary; resolved at build
    time), `priority` (u8; 0 = highest; matches Microkit/seL4),
    `stack_bytes` (u64; default 4 KiB per Microkit)
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
- No duplicate PD names

### `src/plan.rs` — BuildPlan generation

Given a parsed SystemSpec, derives a `BuildPlan`:

```rust
pub struct BuildPlan {
    pub spec_hash: Hash256,   // SHA-256 of canonical TOML rendering of SystemSpec
    pub steps: Vec<BuildStep>, // ordered compile steps (per-PD) + final assemble
    pub plan_hash: Hash256,   // SHA-256 of canonical JSON of (spec_hash, steps)
}
```

Determinism: same SystemSpec → same steps → same plan_hash.
Tested via duplicate-spec-same-plan-hash assertion.

Rejects a spec with no protection domains
(`PlanGenerationError::EmptySpec`).

The plan is the manifest a reproducible-build harness replays.
v0.1.x ships the plan generator only; actual command execution
(future #14) consumes the plan.

### `src/main.rs` — CLI

clap-based subcommands:

- `validate <spec.toml>` — parse the TOML; reject on invariant
  violation; exit 0 on valid. On valid, prints a one-line summary
  to stdout: `✓ <path> — N protection_domain(s), N channel(s),
  N memory_region(s), N irq_delivery`.
- `plan <spec.toml>` — parse + generate BuildPlan + print
  BuildPlan as JSON. Output format controlled by `--format json`
  (default) or `--format pretty-json`; rendered via `serde_json`.
- `build <spec.toml>` — parse + plan + STUB execute (prints
  "would run: <command>" for each step; exit 0). The stub header
  line prints the plan_hash (first 8 bytes hex, `…` suffix) before
  the per-step lines. Actual execution lands in future task #14.

Exit codes: 0 on success for all three subcommands; non-zero on
any I/O, parse, or plan-generation error. Successful output
(validate summary, plan JSON) goes to stdout; errors and the
build stub scope note go to stderr (`eprintln!`).

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

Phase 1B ships 30 tests: 12 in `src/spec.rs` (SystemSpec parse +
invariant coverage), 10 in `src/plan.rs` (determinism, step
generation, hash sensitivity), 8 in `src/main.rs` (CLI integration
via `tempfile` fixtures). `cargo test -p moonshot-toolkit` passes
clean at v0.1.3. CLI integration tests use `tempfile` (dev-
dependency) to write ephemeral spec fixtures; no fixture files are
committed.

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
