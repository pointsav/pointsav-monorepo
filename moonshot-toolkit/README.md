<div align="center">

# 🚀 MOONSHOT-TOOLKIT

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

**Entity:** PointSav Digital Systems™ (The Vendor)
**Taxonomy:** Tier-1 Build Orchestrator
**Status:** Active (Phase 1B — Rust-only build orchestrator)
**Cluster:** [`cluster/project-system`](../../) per workspace `PROJECT-CLONES.md`

---

## I. PURPOSE

`moonshot-toolkit` is the Rust-only build orchestrator replacing
Microkit's Python/CMake toolchain per MEMO §7 ("Microkit
(Python/CMake) → moonshot-toolkit (Rust-Only Toolchain)"). It
reads a Rust-native `system-spec.toml` (equivalent of Microkit's
system-description XML), generates a deterministic content-
addressed `BuildPlan`, and orchestrates the seL4 + system-* + os-*
compile to produce a bootable image.

Foundational to Phase 1B: without `moonshot-toolkit`, no other
`moonshot-*` project ships.

Constitutional anchors: Doctrine claims #33 + #34. Operational
specification: [`conventions/system-substrate-doctrine.md`](../../../conventions/system-substrate-doctrine.md)
§6 (Reproducible-Verification-On-Customer-Metal).

## II. WHAT IT CONTAINS

After Phase 1B's CLI rewrite (this AUTO session, tasks #35-#37):

- `SystemSpec` Rust struct + TOML parser — equivalent of
  Microkit 2.2.0 system-description XML
- `BuildPlan` generator — deterministic content-addressed
  manifest the customer-apex cosignature commits to
- CLI subcommands: `validate` / `plan` / `build`
  (build-stub in v0.1.x; actual seL4 cross-compile is task #14
  FUTURE)

## III. WHAT IT DOES NOT CONTAIN

- The cross-compile toolchain itself (aarch64-linux-gnu-gcc) —
  system dependency, not Rust code
- seL4 source — vendored into `vendor-sel4-kernel`
- Customer-apex cosignature primitive — that's
  `system-core::checkpoint`; moonshot-toolkit consumes it

## IV. BUILD AND TEST

```
cargo check -p moonshot-toolkit
cargo test  -p moonshot-toolkit
```

After CLI rewrite:

```
cargo run -p moonshot-toolkit -- validate <spec.toml>
cargo run -p moonshot-toolkit -- plan     <spec.toml>
cargo run -p moonshot-toolkit -- build    <spec.toml>   # STUB
```

## V. CURRENT STATE

Activated 2026-04-27 per framework §9 (Master Option A lean +
operator confirmation). Framework §9 docs in place; CLI rewrite
+ SystemSpec + BuildPlan modules land in subsequent commits this
AUTO session. Actual seL4 hello-world cross-compile + QEMU
AArch64 boot is FUTURE work needing operator-trigger or Master-
trigger.

## VI. LICENSING

Inherits the monorepo `LICENSE` at the repo root.

## VII. REFERENCES

- `~/Foundry/DOCTRINE.md` §II claims #33 + #34
- `~/Foundry/conventions/system-substrate-doctrine.md` §6
- `~/Foundry/MEMO-2026-03-30-Development-Overview-V8.md` §7
- This project's `CLAUDE.md`, `ARCHITECTURE.md`, `DEVELOPMENT.md`,
  `NEXT.md`
- Microkit 2.2.0 system-description XML reference at
  `https://docs.sel4.systems/projects/microkit/manual/latest/`
