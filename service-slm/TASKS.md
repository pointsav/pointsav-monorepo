# TASKS.md — ordered work queue

The canonical queue of work units for service-slm, ordered by a rough
best-next-thing heuristic. Claude Code reads this at session start and
picks the highest-priority open task it has the context to complete.

Task format:

```
### [N] <short title>
- Status: open | in-progress | done
- Priority: p0 | p1 | p2
- Crate: <crate name> or "workspace"
- Model: opusplan | opus | sonnet | haiku | human
- Context: <one line>
- Acceptance: <one line>
```

Mark a task `done` when it lands on main; do not delete done tasks —
they are the audit trail of how the service got built.

---

## Model guidance

Each task carries a `Model` suggestion to help the operator pick the
right Claude Code model at session start. The guidance reflects cost-
versus-capability trade-offs; it is not enforced by the tool.

- **`opusplan`** — default for most implementation work. Claude Code
  runs planning in Opus, then automatically switches to Sonnet for
  code generation. Best balance of reasoning quality and cost.
  Invoke with `/model opusplan` or `claude --model opusplan`.
- **`opus`** — reserve for tasks where the execution itself requires
  deep reasoning, not just the plan (cross-crate refactors, tricky
  concurrency, ADRs that need architectural judgement throughout).
- **`sonnet`** — straight implementation where the plan is already
  clear and writing the code is the main work. Faster and cheaper
  than `opusplan` when no real planning is needed.
- **`haiku`** — mechanical tasks: dependency bumps, CHANGELOG entries,
  doc typo fixes, lockfile commits, boilerplate scaffolding.
  Significantly cheaper; will struggle on anything requiring design
  judgement.
- **`human`** — not a Claude Code session. Content that a specific
  person must author (organisational rationale, legal language,
  maintainer appointments).

If you're unsure, `opusplan` is almost always the right choice. The
automatic planning-to-execution handoff makes it hard to overspend
and the planning phase also catches misunderstandings before expensive
code generation begins.

To set the default for your session, either run `/model opusplan`
at the start or set `ANTHROPIC_MODEL=claude-opus-4-6` (or the current
opusplan-enabled variant) in your shell before launching `claude`.

---

## P0 — Phase 2 kickoff (must land before any other P1 work)

### [1] Verify workspace builds green on a clean host
- Status: in-progress
- Priority: p0
- Crate: workspace
- Model: sonnet
- Context: First push to origin must prove the scaffold is buildable. Run `./scripts/check-all.sh` on a fresh checkout.
- Acceptance: CI green on `main` for all of fmt, clippy, test, audit, deny.
- Note: Local checks (fmt, clippy, test) pass as of 2026-04-20. cargo-audit and cargo-deny not installed locally; verified in CI. Awaiting first push to main to close.

### [2] Peter rewrites ADR-0003 rationale
- Status: open
- Priority: p0
- Crate: docs
- Model: human
- Context: ADR-0003 ships with placeholder reasoning per the build-session discussion. Rewrite §Rationale to reflect Woodfine's actual organisational rationale. This is content only Peter can author.
- Acceptance: Editor's note removed; rationale reflects real org reasoning; PR merged.

### [3] Pin workspace dependency versions for Phase 2
- Status: done
- Priority: p0
- Crate: workspace
- Model: haiku
- Context: `Cargo.toml` currently carries indicative minimums. Bump to current latest at Phase 2 kickoff and commit `Cargo.lock`.
- Acceptance: `cargo update` produces no diff; lockfile committed.
- Note: Lockfile was already at latest compatible versions on 2026-04-20. Verified with `cargo update` → 0 packages locked; all checks pass. Committed as 6c39ed8.

---

## P1 — First crate implementations

### [10] slm-core: ModuleId newtype
- Status: open
- Priority: p1
- Crate: slm-core
- Model: opusplan
- Context: Foundation for every other crate. Validation per YOYO-COMPUTE §6. Must round-trip through serde and be usable as a HashMap key.
- Acceptance: `ModuleId::new(&str) -> Result<Self, Error>` with validation, `Display`/`FromStr`, serde round-trip, property tests.

### [11] slm-ledger: Event struct with all 10 variants
- Status: open
- Priority: p1
- Crate: slm-ledger
- Model: opusplan
- Context: Schema is specified in YOYO-COMPUTE §5. Build the typed enum first; append-only writer comes next.
- Acceptance: All 10 event_type variants represented as enum; CSV round-trip tested; column order matches spec.

### [12] slm-doorman: SanitisationPolicy type + pass-through impl
- Status: open
- Priority: p1
- Crate: slm-doorman
- Model: opusplan
- Context: The type that defines which fields are stripped. A pass-through implementation (strips nothing) is fine for the first iteration; property test the round-trip.
- Acceptance: `SanitisationPolicy` trait + `NoOp` impl + property test that sanitise ∘ rehydrate = identity.

### [13] slm-ledger: append-only CSV writer with fsync
- Status: open
- Priority: p1
- Crate: slm-ledger
- Model: opusplan
- Context: Follows [11]. Crash-safe writer with fsync on commit. Durability semantics are subtle; `opusplan` ensures the plan phase thinks carefully before code generation begins.
- Acceptance: Writer integration test that kills the process between write and sync and proves no row loss.

### [14] slm-api: router skeleton with /health
- Status: open
- Priority: p1
- Crate: slm-api
- Model: sonnet
- Context: Thinnest possible axum server, to be expanded as other crates ship. Mostly boilerplate; minimal planning needed.
- Acceptance: `/health` returns 200; tower tracing layer installed; unit test via `tower::ServiceExt`.

### [15] slm-compute: parse compute/manifest.yaml
- Status: open
- Priority: p1
- Crate: slm-compute
- Model: opusplan
- Context: Typed `ComputeManifest` with `validator` rules. Input format spec is YOYO-COMPUTE §2.
- Acceptance: YAML round-trip; validator rejects malformed input with clear error messages.

---

## P2 — Second wave

### [20] slm-inference-remote: HTTP client skeleton + BOOT_* events
- Status: open
- Priority: p2
- Crate: slm-inference-remote
- Model: opusplan

### [21] slm-memory-kv: deterministic block hash
- Status: open
- Priority: p2
- Crate: slm-memory-kv
- Model: opusplan
- Context: Hash stability across processes is a subtle invariant. Plan carefully before code generation.

### [22] slm-memory-adapters: Registry YAML parser
- Status: open
- Priority: p2
- Crate: slm-memory-adapters
- Model: sonnet

### [23] slm-inference-local: RAM probe + quantisation selector
- Status: open
- Priority: p2
- Crate: slm-inference-local
- Model: opusplan

### [24] slm-cli: wire `slm-cli ledger tail` to slm-ledger
- Status: open
- Priority: p2
- Crate: slm-cli
- Model: sonnet
- Context: CLI wiring is mechanical once the library crate is alpha-ready.

---

## Done

*(empty — the first commit that lands work here will record it.)*
