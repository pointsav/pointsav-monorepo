<div align="center">

# moonshot-toolkit

[ Leer en Español ](./README.es.md)

</div>

**Entity:** PointSav Digital Systems (The Vendor)
**Taxonomy:** Tier-1 Build Orchestrator — `moonshot-*` family
**Version:** 0.1.3
**Status:** Active — Phase 1B v0.1.x scope closed
**Cluster:** `cluster/project-system` per workspace `PROJECT-CLONES.md`

---

## I. What it is

`moonshot-toolkit` is the Rust-only build orchestrator for Foundry's seL4
unikernel images. It replaces the Microkit Python/CMake toolchain mandated
by MEMO §7 ("Microkit (Python/CMake) → moonshot-toolkit (Rust-Only
Toolchain)") with a single Rust binary that reads a declarative
`system-spec.toml`, produces a deterministic content-addressed `BuildPlan`,
and (in a future increment) drives the cross-compilation pipeline to a
bootable image.

The crate is the load-bearing gateway for the `moonshot-*` family. No other
`moonshot-*` project — kernel, hypervisor, protocol adapters — can produce
a runnable artefact without passing through moonshot-toolkit's plan and
build stages.

---

## II. What it does

### SystemSpec parser

`moonshot-toolkit` reads a `system-spec.toml` that is structurally
equivalent to Microkit 2.2.0's system-description XML
(<https://docs.sel4.systems/projects/microkit/manual/latest/>), but
expressed in TOML and parsed into a typed Rust struct at startup. The spec
declares:

- **Protection Domains (PDs)** — isolated single-threaded components, each
  with a binary path, scheduling priority, and stack size. Hard limit: ≤ 63
  PDs per system (the Microkit architectural constant).
- **Channels** — point-to-point communications between PDs, either
  `ppc` (protected procedure call; synchronous) or `notification`
  (asynchronous signal). Hard limit: ≤ 63 channels per PD.
- **Memory Regions** — physical address ranges with caching policy
  (`cached`, `uncached`, `device-memory`) and read/write/execute
  permissions. Regions must not overlap; the parser enforces this at load
  time.
- **IRQ Delivery** — hardware interrupt bindings from IRQ number to a
  target PD and optional channel ID.

All validation — PD count, channel count per PD, no duplicate PD names,
channel endpoints reference declared PDs, IRQ targets reference declared
PDs, no overlapping memory regions — runs at parse time. A spec that
violates any invariant is rejected with a typed `SpecParseError` before
any build step executes.

### BuildPlan generator

From a validated `SystemSpec`, `moonshot-toolkit` generates a `BuildPlan`:
a deterministic, content-addressed manifest of every step required to
produce the final image. The plan contains:

- **`spec_hash`** — SHA-256 of the canonical TOML serialisation of the
  `SystemSpec`. Whitespace-invariant: the same logical spec always produces
  the same hash regardless of formatting.
- **`steps`** — ordered list of `BuildStep` records. Each step names its
  input and output paths and carries a `BuildCommand` variant:
  - `CompilePd` — one step per protection domain in declaration order;
    cross-compiles the PD's `binary` source to `build/<pd-name>.elf`.
  - `AssembleImage` — final step; takes all PD ELF binaries and the
    spec hash as inputs; produces `build/system-image.bin`.
- **`plan_hash`** — SHA-256 of the canonical JSON serialisation of
  `(spec_hash, steps)`. This is the value a customer-apex cosignature
  commits to per `conventions/system-substrate-doctrine.md` §6.1. Same
  spec on any machine on any date produces the same `plan_hash`.

The `plan_hash` is what makes the build reproducible and auditable. A
customer can verify that the image they received was produced from the plan
they approved by re-running `moonshot-toolkit plan <spec>` and comparing
hashes without trusting the build environment.

### CLI subcommands

The CLI is a single `moonshot-toolkit` binary with three subcommands,
implemented with clap 4:

```
moonshot-toolkit validate <spec.toml>
```
Parse and invariant-check the spec. Exits 0 on success, non-zero with a
message on any parse or validation failure. Prints a summary line
confirming PD count, channel count, memory region count, and IRQ count.
Use this as a pre-commit or CI lint gate.

```
moonshot-toolkit plan <spec.toml> [--format json|pretty-json]
```
Parse the spec and emit the full `BuildPlan` as JSON (default) or
pretty-printed JSON. Suitable for piping into `jq`, storing in a build log,
or feeding to the cosignature step. The `--format pretty-json` flag
produces indented output for human review.

```
moonshot-toolkit build <spec.toml>
```
Parse, plan, then — in v0.1.x — print "would run" for each step instead
of executing. This is an intentional stub. The actual seL4 cross-compile
pipeline is FUTURE work (task #14); see "What's Deferred" below.

---

## III. Status

Version 0.1.3. Phase 1B v0.1.x scope is closed.

The three deliverables of Phase 1B are complete:

1. `src/spec.rs` — `SystemSpec` TOML parser with full invariant
   enforcement (12 tests; commit `045e5cc`).
2. `src/plan.rs` — `BuildPlan` deterministic generator with
   content-addressed spec and plan hashes (10 tests; commit `59d1fc0`).
3. `src/main.rs` — clap CLI rewrite with `validate`, `plan`, and stub
   `build` subcommands (8 tests; commit `af6073f`).

Total test count: 30 (22 library + 8 binary). All tests pass. Zero
compiler warnings.

The `build` subcommand is intentionally incomplete in v0.1.x. It prints
the plan it would execute but performs no compilation. Actual seL4
cross-compile and QEMU AArch64 boot are deferred to task #14, which
requires decisions not yet made on the cross-compile toolchain and seL4
source strategy (see "What's Deferred").

---

## IV. SystemSpec format

A `system-spec.toml` describes the full protection-domain topology of one
seL4 system image. Minimal working example:

```toml
[[protection_domains]]
name = "hello"
binary = "src/hello.rs"
priority = 100
stack_bytes = 8192

[[memory_regions]]
name = "uart"
phys_addr = 0x09000000
size_bytes = 4096
caching = "device-memory"
permissions = ["read", "write"]
```

Key validation rules enforced at parse time:

| Rule | Limit / condition |
|---|---|
| PD count | ≤ 63 |
| PD names | unique within a spec |
| Channels per PD | ≤ 63 (summed across both ends) |
| Channel endpoints | must reference declared PD names |
| IRQ targets | must reference declared PD names |
| Memory regions | must not overlap (checked pairwise) |

`priority` is a `u8` where 0 is the highest seL4 scheduling priority,
matching the Microkit convention. `stack_bytes` defaults to 4096 (4 KiB)
if omitted.

`ChannelKind` values are `ppc` or `notification` (kebab-case). Caching
policy values are `cached` (default), `uncached`, or `device-memory`.
Permissions are `read`, `write`, `execute`.

---

## V. BuildPlan format

A `BuildPlan` serialises to JSON. Example for a single-PD spec:

```json
{
  "spec_hash": [12, 34, ...],
  "steps": [
    {
      "name": "compile-pd-hello",
      "description": "Cross-compile protection domain `hello`",
      "input_paths": ["src/hello.rs"],
      "output_paths": ["build/hello.elf"],
      "command": {
        "compile-pd": {
          "pd_name": "hello",
          "source_path": "src/hello.rs",
          "binary_target": "build/hello.elf"
        }
      }
    },
    {
      "name": "assemble-image",
      "description": "Assemble bootable seL4 image from PD binaries and system spec",
      "input_paths": ["build/hello.elf"],
      "output_paths": ["build/system-image.bin"],
      "command": {
        "assemble-image": {
          "pd_binary_paths": ["build/hello.elf"],
          "spec_hash": [12, 34, ...],
          "output_image": "build/system-image.bin"
        }
      }
    }
  ],
  "plan_hash": [56, 78, ...]
}
```

Steps are always ordered: one `CompilePd` step per protection domain in
declaration order, followed by one `AssembleImage` step. Adding a variant
to `BuildCommand` is a MINOR version change.

`spec_hash` and `plan_hash` are 32-byte arrays (SHA-256). `plan_hash`
covers both the spec hash and all step data, so any change to any input
path, output path, PD name, or ordering produces a different `plan_hash`.

---

## VI. Build and test

```
cargo build -p moonshot-toolkit
cargo test  -p moonshot-toolkit
```

Run subcommands from the workspace root:

```
cargo run -p moonshot-toolkit -- validate path/to/system-spec.toml
cargo run -p moonshot-toolkit -- plan     path/to/system-spec.toml
cargo run -p moonshot-toolkit -- plan     path/to/system-spec.toml --format pretty-json
cargo run -p moonshot-toolkit -- build    path/to/system-spec.toml   # stub in v0.1.x
```

Dependencies: `serde`, `serde_json`, `toml`, `sha2`, `clap 4`. Dev
dependency: `tempfile 3` (CLI tests write temporary spec files).

The crate is a workspace member of `pointsav-monorepo`. Workspace-level
`cargo check` and `cargo test --workspace` cover it alongside the
other declared members.

---

## VII. What's deferred

**Task #14 — actual seL4 cross-compile + QEMU AArch64 boot** is the
next increment. It is a FUTURE task requiring three decisions not yet made:

1. **Cross-compile toolchain strategy.** The `aarch64-linux-gnu-gcc`
   toolchain is a system dependency outside Rust. The Rust-Only Toolchain
   mandate (MEMO §7) applies to the build orchestrator code, not to the
   toolchain it drives. Options include Nix-pinned toolchain, Bazel
   hermetic toolchain, Docker image with fixed toolchain, or operator-
   installed system dependency. Decision gates the reproducible-build
   harness cosignature design.

2. **seL4 source vendoring strategy.** `vendor-sel4-kernel` holds vendored
   seL4 source today. The question is whether moonshot-toolkit drives the
   build against a git submodule, a Cargo `build.rs`-triggered fetch, a
   snapshot in `vendor-sel4-kernel`, or some combination. Each choice has
   different implications for the hermetic-build property and the
   `plan_hash` stability guarantee.

3. **Reproducible-build harness.** The `build` subcommand must be hermetic
   (no network at build time per MEMO §7 hard constraint) and must produce
   the same binary from the same inputs across machines and time. The
   specific mechanism — Nix, Bazel, Cargo build scripts with pre-fetched
   inputs, or a custom hermetic runner — determines what "reproducible"
   means operationally and how the customer re-executes verification.

Until these decisions are made, the `build` subcommand remains a planning
tool only. The `validate` and `plan` subcommands are fully operational.

---

## VIII. Hard constraints

These constraints apply to all future development on this crate and must
not be violated without an explicit doctrine change:

- **Rust-Only Toolchain** (MEMO §7): moonshot-toolkit's own code is
  Rust only. No Python, CMake, or shell scripts in the critical path.
  The legacy `build-totebox.sh` exists for migration reference only and
  will be removed once the Phase 1C implementation is complete.
- **Deterministic plan generation**: same `SystemSpec` → same
  `BuildPlan` bytes → same `plan_hash`. Any non-determinism is a
  correctness defect, not a performance tradeoff.
- **Content-addressed inputs**: every build input is named by SHA-256
  hash. The `plan_hash` covers the full input surface.
- **No network on `build`**: the build step must be hermetic. Network
  access happens at a separate `prepare` or `vendor` phase, not at
  build time.

---

## IX. Cross-references

- **DOCTRINE.md** — claims #33 (The Capability Ledger Substrate) and
  #34 (The Two-Bottoms Sovereign Substrate). The `plan_hash` is the
  artefact that enters the ledger; the customer-apex cosignature on
  that hash is the Reproducible-Verification-On-Customer-Metal claim.
- **`conventions/system-substrate-doctrine.md` §6** — Reproducible-
  Verification-On-Customer-Metal. Specifies the release-artefact format
  that moonshot-toolkit is designed to produce.
- **MEMO §7** — "Microkit (Python/CMake) → moonshot-toolkit
  (Rust-Only Toolchain)". The mandate this crate implements.
- **`system-core`** — the `plan_hash` will eventually be committed to
  a `system-core::LedgerAnchor`; the customer-apex cosignature uses
  `system-core::SignedCheckpoint`. Dependency not yet wired (FUTURE).
- **`vendor-sel4-kernel`** — vendored seL4 source. The cross-compile
  pipeline will consume this directory once task #14 lands.
- **Microkit 2.2.0 reference**:
  <https://docs.sel4.systems/projects/microkit/manual/latest/>

---

## X. Licensing

Inherits the monorepo `LICENSE` at the repo root.
