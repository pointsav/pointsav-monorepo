# Drift-Detection Audit — moonshot-toolkit ARCHITECTURE.md vs. shipped Phase 1B CLI

**Audit date:** 2026-04-27  
**Auditor:** Task Claude (project-system cluster)

---

## 1. Methodology

This audit compares `moonshot-toolkit/ARCHITECTURE.md` against three
shipped source files committed as part of the Phase 1B CLI rewrite.
Claims in the architecture document are evaluated strictly against what
the code does, not against doctrine or design desirability. This is
drift detection, not architectural critique: a discrepancy is flagged
only when the doc says X and the code does something materially
different.

Files read:

| File | Commit |
|---|---|
| `moonshot-toolkit/ARCHITECTURE.md` | (main HEAD — activation commit `ba34cd8`, current after `af6073f`) |
| `moonshot-toolkit/src/spec.rs` | `045e5cc` |
| `moonshot-toolkit/src/plan.rs` | `59d1fc0` |
| `moonshot-toolkit/src/main.rs` | `af6073f` |

---

## 2. Match list ✓

- ✓ **Three CLI subcommands `validate` / `plan` / `build`** (ARCHITECTURE.md:L24-25, §3 "src/main.rs — CLI"). Confirmed in `main.rs:L38-59` via the `Command` enum with `Validate`, `Plan`, and `Build` variants; semantics (parse-only, generate+print, stub execute) match the doc.

- ✓ **`SystemSpec` fields: `protection_domains`, `channels`, `memory_regions`, `irq_delivery`** (ARCHITECTURE.md:L63-70, §3). All four fields declared in `spec.rs:L29-42` as `Vec<ProtectionDomain>`, `Vec<Channel>`, `Vec<MemoryRegion>`, `Vec<IrqDelivery>`.

- ✓ **Microkit PD limit ≤ 63** (ARCHITECTURE.md:L63). Enforced at `spec.rs:L23` (`MAX_PROTECTION_DOMAINS: usize = 63`) and validated at `spec.rs:L161-165`.

- ✓ **Channels per PD limit ≤ 63** (ARCHITECTURE.md:L64). Enforced at `spec.rs:L25` (`MAX_CHANNELS_PER_PD: usize = 63`) and validated at `spec.rs:L176-188`.

- ✓ **No overlapping memory regions** (ARCHITECTURE.md:L77). Validated at `spec.rs:L223-237`.

- ✓ **IRQ targets must reference declared PDs** (ARCHITECTURE.md:L78). Validated at `spec.rs:L212-220`.

- ✓ **Channel endpoints must reference declared PDs** (ARCHITECTURE.md:L79). Validated at `spec.rs:L192-210`.

- ✓ **TOML on-disk format via `toml = "0.8"` + serde** (ARCHITECTURE.md:L72). `spec.rs:L20` imports `serde::{Deserialize, Serialize}`; `from_toml_str` at `spec.rs:L149` calls `toml::from_str`.

- ✓ **`BuildPlan` struct with `spec_hash`, `steps`, `plan_hash`** (ARCHITECTURE.md:L85-91). Declared at `plan.rs:L26-37`; all three fields present with matching types and semantics.

- ✓ **`plan_hash` = SHA-256 of canonical bytes of (spec_hash, steps)** (ARCHITECTURE.md:L93-95). Implemented at `plan.rs:L132-165`: `compute_plan_hash` serialises `Canonical { spec_hash, steps }` to JSON then SHA-256s the bytes.

- ✓ **Determinism: same SystemSpec → same plan_hash** (ARCHITECTURE.md:L93-95). Tested at `plan.rs:L240-247` (`plan_is_deterministic`).

- ✓ **`build` is a stub printing "would run" per step** (ARCHITECTURE.md:L109-111). Implemented at `main.rs:L118-148`; each step prints `"would run: <cmd_summary>"` (L141).

- ✓ **Actual execution deferred to future task #14** (ARCHITECTURE.md:L111, §7). Explicitly stated in `main.rs:L143-147` stderr note and in module doc at `main.rs:L10-12`.

- ✓ **SHA-256 baseline per `worm-ledger-design.md` §3 D3; algorithm-agility note** (ARCHITECTURE.md:L115-118, §4). `Hash256` type declared at `plan.rs:L22-23` with doc comment citing `worm-ledger-design.md §3 D3` and naming future BLAKE3/SHA-3.

- ✓ **`plan` subcommand prints canonical TOML/JSON representation** (ARCHITECTURE.md:L107-108). `cmd_plan` at `main.rs:L105-116` serialises the plan to JSON and prints to stdout.

- ✓ **`validate` exits 0 on valid; rejects on invariant violation** (ARCHITECTURE.md:L105-106). `cmd_validate` at `main.rs:L92-103` returns `Ok(())` on success; read/parse errors propagate as `Err(String)`, causing `ExitCode::FAILURE` via `dispatch` at `main.rs:L67-76`.

- ✓ **`v0.1.x` ships plan generator only; actual command execution is future #14** (ARCHITECTURE.md:L98-99). Confirmed in `plan.rs:L10-12` module doc.

---

## 3. Drift list ⚠

- ⚠ **`ProtectionDomain` fields listed in ARCHITECTURE.md do not match the shipped struct** (ARCHITECTURE.md:L64-66).

  Doc claims (ARCHITECTURE.md:L65-66):
  > `name`, `entry_points` (init/notified/protected/fault), `priority`, `assigned_memory_regions`, `assigned_channels`

  Shipped code (`spec.rs:L45-56`): `ProtectionDomain` has `name: String`, `binary: String`, `priority: u8`, `stack_bytes: u64`. The doc fields `entry_points`, `assigned_memory_regions`, `assigned_channels` are absent; the code fields `binary` and `stack_bytes` are undocumented. This is a substantive model drift: the doc describes a Microkit-closer schema while the code ships a leaner schema without entry-point callbacks or per-PD region/channel assignment lists.

  **Proposed reconciliation:** Update ARCHITECTURE.md §3 `src/spec.rs` ProtectionDomain fields to: `name`, `binary` (path to PD binary; resolved at build time), `priority` (u8; 0 = highest), `stack_bytes` (u64; default 4 KiB per Microkit). Remove the `entry_points`/`assigned_memory_regions`/`assigned_channels` lines, which were not implemented.

- ⚠ **`BuildPlan` struct in doc includes `input_hashes: Vec<Hash256>` field absent from shipped code** (ARCHITECTURE.md:L86-91).

  Doc claims (ARCHITECTURE.md:L87):
  > `input_hashes: Vec<Hash256>,  // each declared input file`

  Shipped `BuildPlan` struct (`plan.rs:L26-37`) has `spec_hash`, `steps`, and `plan_hash` — no `input_hashes` field. Per-input file hashes are not computed; instead the plan derives per-PD steps whose `input_paths` strings name files but do not hash them.

  **Proposed reconciliation:** Remove the `input_hashes` line from the struct block in ARCHITECTURE.md §3 `src/plan.rs`. The doc block should list the three actual fields: `spec_hash`, `steps`, `plan_hash`.

- ⚠ **`validate` success message not documented; doc implies silent exit 0** (ARCHITECTURE.md:L105-106).

  Doc: "parse the TOML; reject on invariant violation; exit 0 on valid." Shipped `cmd_validate` (`main.rs:L94-103`) prints a confirmation line to stdout (`"✓ <path> — N protection_domain(s), ..."`) on success. This is not drift that misleads, but the doc implies a silent exit while the code is intentionally verbose on success.

  **Proposed reconciliation:** Extend ARCHITECTURE.md:L105-106 `validate` description to note: "on valid, prints a one-line summary to stdout (`✓ <path> — N protection_domain(s), ...`) and exits 0."

- ⚠ **§6 "Verification" states "0 tests at activation; tests land alongside each module impl per cluster tasks #35 / #36 / #37"** (ARCHITECTURE.md:L137-139).

  The Phase 1B work landed 30 tests (22 lib + 8 main) across `spec.rs`, `plan.rs`, and `main.rs`. §6 was written at activation before those tasks completed and was not updated after the CLI rewrite shipped.

  **Proposed reconciliation:** Update ARCHITECTURE.md §6 to: "Phase 1B ships 30 tests: 12 in `src/spec.rs`, 10 in `src/plan.rs`, 8 in `src/main.rs`. `cargo test -p moonshot-toolkit` passes clean."

---

## 4. Undocumented behaviour ➕

- ➕ **Exit-code conventions** (`main.rs:L67-76`). The `main()` function returns `ExitCode::SUCCESS` (0) on all `Ok(())` paths and `ExitCode::FAILURE` (non-zero) on any `Err(String)`. ARCHITECTURE.md §3 mentions "exit 0 on valid" for `validate` but does not document exit codes for `plan` or `build`, nor the general error → non-zero convention. **Proposed addition:** Add a short exit-code table to ARCHITECTURE.md §3 "src/main.rs — CLI": exit 0 on success for all three subcommands; non-zero on any parse, I/O, or plan-generation error.

- ➕ **stdout vs stderr separation** (`main.rs:L70-74`, `L143-147`). Errors are written to stderr via `eprintln!("error: {e}")` (`main.rs:L72`). The `build` stub also writes its scope note to stderr (`main.rs:L143-147`). Plan JSON is written to stdout (`main.rs:L114`). This separation is undocumented. **Proposed addition:** Note in ARCHITECTURE.md §3 that successful output (validate summary, plan JSON) goes to stdout; errors and stub-scope notes go to stderr.

- ➕ **`--format` flag on `plan` subcommand** (`main.rs:L49-51`, `L61-65`). `plan` accepts `--format json` (default) and `--format pretty-json`. ARCHITECTURE.md §3 says only "print canonical TOML / JSON representation" with no mention of a format flag or the two variants. **Proposed addition:** Add a note to the `plan` description in ARCHITECTURE.md §3: accepts `--format json` (default) or `--format pretty-json`; the JSON rendering uses `serde_json`.

- ➕ **`build` subcommand prints `plan_hash` hex prefix** (`main.rs:L121-122`). The stub prints `"Would execute BuildPlan (plan_hash = <first 8 bytes as hex>…)"` using `hex_short()` (`main.rs:L151-158`). The architecture describes the stub only as printing "would run: <command>" per step; the plan_hash output line is undocumented. **Proposed addition:** Extend the `build` description in ARCHITECTURE.md §3 to note that the stub header line prints the plan_hash (first 8 bytes, hex, with `…` suffix).

- ➕ **`tempfile` dev-dependency used in CLI tests** (`main.rs:L163-170`). CLI tests create `NamedTempFile` instances to write spec fixtures. This dependency choice is undocumented. **Proposed addition:** Add a brief note in ARCHITECTURE.md §6 Verification (or a new §3 "Testing approach" sentence): CLI integration tests use `tempfile` (dev-dependency) to write ephemeral spec fixtures; no fixture files are committed.

- ➕ **Duplicate PD name validation** (`spec.rs:L125-143`, `L167-173`). `SpecParseError::DuplicatePdName` is implemented and tested but not listed in ARCHITECTURE.md §3's validation rules (ARCHITECTURE.md:L74-79). **Proposed addition:** Add "No duplicate PD names" to the validation rules list in ARCHITECTURE.md §3 `src/spec.rs`.

- ➕ **`BuildPlan::from_spec` rejects empty spec** (`plan.rs:L87-89`). `PlanGenerationError::EmptySpec` is returned when `protection_domains` is empty. ARCHITECTURE.md §3 `src/plan.rs` does not document this guard. **Proposed addition:** Add a note to the `src/plan.rs` section: "Rejects a spec with no protection domains (`PlanGenerationError::EmptySpec`)."

---

## 5. Build-stub deferral framing

ARCHITECTURE.md explicitly states the v0.1.x scope-boundary at the build-stub. ARCHITECTURE.md:L109-111 reads:

> `build <spec.toml>` — parse + plan + STUB execute (prints
> "would run: <command>" for each step; exit 0). Actual
> execution lands in future task #14.

And §7 (ARCHITECTURE.md:L141-157) explicitly lists all task #14 dependencies: cross-compile toolchain installation, seL4 source vendoring strategy, reproducible-build harness selection, QEMU AArch64 boot, and Sigstore Cosign output signing.

The doc does **not** imply the build subcommand is functional. The stub status and the FUTURE task #14 dependencies are both documented.

**Finding:** No gap. The build-stub deferral framing is already correctly stated in ARCHITECTURE.md.

---

## 6. Recommended ARCHITECTURE.md edits

### High-priority (drift; doc misleads readers about current state)

**Edit 1 — `ProtectionDomain` fields** (ARCHITECTURE.md:L65-66):

Replace lines 64-66:
```
- `protection_domains: Vec<ProtectionDomain>` (max 63 per Microkit)
  - `name`, `entry_points` (init/notified/protected/fault),
    `priority`, `assigned_memory_regions`, `assigned_channels`
```
With:
```
- `protection_domains: Vec<ProtectionDomain>` (max 63 per Microkit)
  - `name` (String), `binary` (path to PD binary; resolved at build
    time), `priority` (u8; 0 = highest; matches Microkit/seL4),
    `stack_bytes` (u64; default 4 KiB per Microkit)
```

**Edit 2 — `BuildPlan` struct block** (ARCHITECTURE.md:L85-91):

Replace lines 85-91:
```
```rust
struct BuildPlan {
    spec_hash: Hash256,         // SHA-256 of canonical SystemSpec bytes
    input_hashes: Vec<Hash256>, // each declared input file
    steps: Vec<BuildStep>,      // ordered (inputs, command, outputs)
    plan_hash: Hash256,         // SHA-256 of all of the above canonical bytes
}
```
```
With:
```
```rust
pub struct BuildPlan {
    pub spec_hash: Hash256,  // SHA-256 of canonical TOML rendering of SystemSpec
    pub steps: Vec<BuildStep>, // ordered compile steps (per-PD) + final assemble
    pub plan_hash: Hash256,  // SHA-256 of canonical JSON of (spec_hash, steps)
}
```
```

**Edit 3 — §6 Verification test counts** (ARCHITECTURE.md:L137-139):

Replace lines 137-139:
```
Activation commit: `cargo check -p moonshot-toolkit` passes
(legacy stub + framework §9 docs only). 0 tests at activation;
tests land alongside each module impl per cluster tasks #35 / #36
/ #37.
```
With:
```
Phase 1B ships 30 tests: 12 in `src/spec.rs` (SystemSpec parse +
invariant coverage), 10 in `src/plan.rs` (determinism, step
generation, hash sensitivity), 8 in `src/main.rs` (CLI integration
via `tempfile` fixtures). `cargo test -p moonshot-toolkit` passes
clean at v0.1.3.
```

### Medium-priority (undocumented behaviour worth adding)

**Edit 4 — Validation rules: add DuplicatePdName** (ARCHITECTURE.md after L79):

After the line "Channel endpoints must reference declared PDs", add:
```
- No duplicate PD names
```

**Edit 5 — `plan` subcommand: document `--format` flag** (ARCHITECTURE.md:L107-108):

Append to the `plan` bullet:
```
Output format controlled by `--format json` (default) or
`--format pretty-json`; rendered via `serde_json`.
```

**Edit 6 — `validate` subcommand: document stdout output** (ARCHITECTURE.md:L105-106):

Append to the `validate` bullet:
```
On valid, prints a one-line summary to stdout:
`✓ <path> — N protection_domain(s), N channel(s), N memory_region(s), N irq_delivery`.
```

**Edit 7 — Add stdout/stderr and exit-code note to `src/main.rs` section** (ARCHITECTURE.md after L111, §3 CLI block):

After the `build` bullet, add:
```
Exit codes: 0 on success for all three subcommands; non-zero on
any I/O, parse, or plan-generation error. Errors written to stderr
(`eprintln!`); plan JSON and validate summary written to stdout.
The `build` stub header line prints the plan_hash (first 8 bytes
hex, `…` suffix) before the per-step "would run" lines.
```

**Edit 8 — `src/plan.rs` section: document EmptySpec guard** (ARCHITECTURE.md:L81-99):

After the determinism sentence (L93-95), add:
```
Rejects a spec with no protection domains
(`PlanGenerationError::EmptySpec`).
```

### Low-priority (cosmetic / clarification)

**Edit 9 — `plan` output format: doc says "TOML / JSON"; code only outputs JSON** (ARCHITECTURE.md:L107-108):

The doc says "print canonical TOML / JSON representation" but `cmd_plan` (`main.rs:L105-116`) only emits JSON (compact or pretty). Remove "TOML /" from the description: "print BuildPlan as JSON (`--format json` or `--format pretty-json`)".

---

## 7. Open questions for Master

No open questions for Master. All discrepancies found are doc-behind-code drift with clear reconciliation paths: the architecture describes an earlier design iteration that the Phase 1B implementation refined (leaner `ProtectionDomain`, no `input_hashes` field). No substantive design conflict was found between doc and code — the code is a coherent, more concrete realisation of the doc's intent.
