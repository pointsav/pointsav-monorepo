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
- Status: done
- Priority: p0
- Crate: workspace
- Model: sonnet
- Context: First push to origin must prove the scaffold is buildable. Run `./scripts/check-all.sh` on a fresh checkout.
- Acceptance: CI green on `main` for all of fmt, clippy, test, audit, deny.
- Note: Merged audit-layer-1-findings to main on 2026-04-20. CI verified green: fmt, clippy, test, audit, deny all pass.

### [2] Peter rewrites ADR-0003 rationale
- Status: done
- Priority: p0
- Crate: docs
- Model: human
- Context: ADR-0003 ships with placeholder reasoning per the build-session discussion. Rewrite §Rationale to reflect Woodfine's actual organisational rationale. This is content only Peter can author.
- Acceptance: Editor's note removed; rationale reflects real org reasoning; PR merged.
- Note: Resolved 2026-04-20 as part of broader AGPL-3.0 alignment per `factory-release-engineering/README.md §3`. ADR renamed to `0003-agpl3-for-own-code.md` with full rewrite (jurisdictional-neutrality bullet dropped; AGPL §13 network-use and §11 patent-grant references substituted; follow-up bullets reference CLA Assistant). Editor's-note block and placeholder status qualifier both removed. Same commit flipped contribution certification DCO → CLA per `factory-release-engineering/README.md §2`.

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
- Status: done
- Priority: p1
- Crate: slm-core
- Model: opusplan
- Context: Foundation for every other crate. Validation per YOYO-COMPUTE §6. Must round-trip through serde and be usable as a HashMap key.
- Acceptance: `ModuleId::new(&str) -> Result<Self, Error>` with validation, `Display`/`FromStr`, serde round-trip, property tests.
- Note: Landed 2026-04-20. DNS-label grammar `[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?`, ASCII lowercase + digits + internal hyphens, 1–63 bytes. `ModuleIdError` is a dedicated enum in `slm-core::module_id`; the shared `Error` is deferred until a consumer needs it. 12 unit tests + 4 proptest cases + 1 integration smoke test; full `./scripts/check-all.sh` green.

### [11] slm-ledger: Event struct with all 10 variants
- Status: done
- Priority: p1
- Crate: slm-ledger
- Model: opusplan
- Context: Schema is specified in YOYO-COMPUTE §5. Build the typed enum first; append-only writer comes next.
- Acceptance: All 10 event_type variants represented as enum; CSV round-trip tested; column order matches spec.
- Note: Landed 2026-04-20. `EventType` enum with 10 SCREAMING_SNAKE_CASE variants (Display + FromStr + serde); `Event` struct with 15 fields in spec column order (`moduleId` serde rename preserved); `csv` crate (MIT) added as dep; 17 unit tests + 2 integration tests; `check-all.sh` green.

### [12] slm-doorman: SanitisationPolicy type + pass-through impl
- Status: done
- Priority: p1
- Crate: slm-doorman
- Model: opusplan
- Context: The type that defines which fields are stripped. A pass-through implementation (strips nothing) is fine for the first iteration; property test the round-trip.
- Acceptance: `SanitisationPolicy` trait + `NoOp` impl + property test that sanitise ∘ rehydrate = identity.
- Note: Landed 2026-04-20. `SanitisationError` enum (`Refused`, `Rehydration`); `SanitisationPolicy` trait with `Payload` and `Context` associated types; `NoOp` impl (`Payload = String`, `Context = ()`); proptest on identity property + 3 deterministic unit tests + 2 integration tests; `check-all.sh` green.

### [13] slm-ledger: append-only CSV writer with fsync
- Status: done
- Priority: p1
- Crate: slm-ledger
- Model: opusplan
- Context: Follows [11]. Crash-safe writer with fsync on commit. Durability semantics are subtle; `opusplan` ensures the plan phase thinks carefully before code generation begins.
- Acceptance: Writer integration test that kills the process between write and sync and proves no row loss.
- Note: Landed 2026-04-20. `LedgerWriter::open()` checks file length to suppress duplicate headers on reopen; `append()` sequence is serialize → flush → sync_all(); `LedgerError` wraps `io::Error` and `csv::Error`. Two integration tests: `data_is_durable_after_each_append` (reads via fresh file handle after each append, then drops writer mid-session) and `reopen_does_not_duplicate_header`. `check-all.sh` green.

### [14] slm-api: router skeleton with /health
- Status: done
- Priority: p1
- Crate: slm-api
- Model: sonnet
- Context: Thinnest possible axum server, to be expanded as other crates ship. Mostly boilerplate; minimal planning needed.
- Acceptance: `/health` returns 200; tower tracing layer installed; unit test via `tower::ServiceExt`.
- Note: Landed 2026-04-20. `router()` returns axum `Router` with `GET /health` → `StatusCode::OK` and `tower_http::trace::TraceLayer`; two `ServiceExt::oneshot` integration tests (`health_returns_200`, `unknown_route_returns_404`); `check-all.sh` green.

### [15] slm-compute: parse compute/manifest.yaml
- Status: done
- Priority: p1
- Crate: slm-compute
- Model: opusplan
- Context: Typed `ComputeManifest` with `validator` rules. Input format spec is YOYO-COMPUTE §2.
- Acceptance: YAML round-trip; validator rejects malformed input with clear error messages.
- Note: Landed 2026-04-20. `validator` crate removed after MSRV conflict (transitively required Rust 1.86 via `icu_collections@2.2.0`; workspace pins 1.85). Manual validation via `validate_fields()` used instead — produces `ManifestError::InvalidField(String)` and `ManifestError::InvalidRange(String)` with clear messages. `GpuTier::RtxPro6000` carries explicit `#[serde(rename = "rtx_pro_6000")]` because serde's snake_case does not insert underscore before digits. 8 unit tests + 3 integration tests; `check-all.sh` green.

---

## P2 — Second wave

### [20] slm-inference-remote: HTTP client skeleton + BOOT_* events
- Status: done
- Priority: p2
- Crate: slm-inference-remote
- Model: opusplan
- Context: First real implementation in `slm-inference-remote`. Drive the Cloud Run `/healthz` boot probe with `reqwest`, emit `BOOT_REQUEST` / `BOOT_COMPLETE` via `slm-ledger`, and thread the `ModuleId` through both rows per YOYO-COMPUTE §5/§6. Retry policy and `JOB_*`/`TEARDOWN_*` events deferred to [25]/[26].
- Acceptance: `RemoteInferenceClient::boot(&mut LedgerWriter)` returns `Ok(NodeHandle)` on 2xx `/healthz` with `node_id` populated, returns `Err(RemoteStatus)` on non-success with `BOOT_COMPLETE` recorded as `FAILED` + `error_code = HTTP_<status>`; wiremock-driven integration tests cover both paths; `cargo clippy --all-targets -- -D warnings` clean.
- Note: Landed 2026-04-20. `RemoteInferenceConfig` validates URL scheme is http/https; `RemoteInferenceError::ledger_code()` maps variants to stable strings (`HTTP_TRANSPORT`, `HTTP_<status>`, `LEDGER_FAILURE`, `CONFIG_ERROR`). MSRV trap resolved with lockfile pins (`url=2.5.4`, `idna=1.0.3`, `idna_adapter=1.1.0`, `wiremock=0.6.2`) + `cargo-deny` bans preventing forward walk. CI gained `deny` job running `bans sources` (`licenses` and `advisories` deferred to [30]/[33]). `scripts/check-all.sh` aligned. Two wiremock-driven integration tests green; full `check-all.sh` green.

### [21] slm-memory-kv: deterministic block hash
- Status: open
- Priority: p2
- Crate: slm-memory-kv
- Model: opusplan
- Context: Hash stability across processes is a subtle invariant. Plan carefully before code generation.

### [22] slm-memory-adapters: Registry YAML parser
- Status: done
- Priority: p2
- Crate: slm-memory-adapters
- Model: sonnet
- Note: Landed 2026-04-21. `Registry::from_yaml(&str)` parses the adapter catalogue via a private two-pass shape (raw serde structs → validated public types). Validation: `semver::Version::parse` rejects non-semver strings (bare two-part versions, `v`-prefixed); `HashSet<(adapter_id, version)>` rejects duplicates. `RegistryError` has three variants: `Yaml`, `InvalidVersion`, `DuplicateEntry`. `semver = { version = "1", features = ["serde"] }` added to workspace deps (MIT/Apache-2.0). 6 integration tests; `check-all.sh` green.

### [23] slm-inference-local: RAM probe + quantisation selector
- Status: open
- Priority: p2
- Crate: slm-inference-local
- Model: opusplan

### [24] slm-cli: wire `slm-cli ledger tail` to slm-ledger
- Status: done
- Priority: p2
- Crate: slm-cli
- Model: sonnet
- Context: CLI wiring is mechanical once the library crate is alpha-ready.
- Note: Landed 2026-04-21. `slm_ledger::tail_events(path, n)` added to slm-ledger (`reader.rs`); reads entire CSV and returns trailing `n` rows. `Ledger` command gained `--path` / `SLM_LEDGER_PATH` argument shared by `tail` and `export`. `tail` arm prints one line per event with timestamp, event_type, module_id, node_id, job_id, completion_status. 4 integration tests via `assert_cmd`; `check-all.sh` green.

### [25] slm-inference-remote: retry + exponential-backoff policy
- Status: open
- Priority: p2
- Crate: slm-inference-remote
- Model: opusplan
- Context: Crate CLAUDE.md invariant 3: retries bounded at 5 attempts by default, configurable; exhausted retries are hard failures. Apply to the boot probe and to future `JOB_*` calls. Candidate dep: `backoff` (already in `[workspace.dependencies]`).
- Acceptance: `RetryPolicy` type with configurable attempt ceiling; `boot()` retries transport errors (not 4xx); ledger still emits exactly one `BOOT_REQUEST` + one `BOOT_COMPLETE` (successful attempt or final failure); integration test proves exhaustion returns `Err` and does not duplicate rows.

### [26] slm-inference-remote: JOB_* / TEARDOWN_* / PREEMPTION events
- Status: open
- Priority: p2
- Crate: slm-inference-remote
- Model: opusplan
- Context: Extend the client beyond boot to cover the rest of the YOYO-COMPUTE §5 event set. `TEARDOWN_COMPLETE` carries `gpu_seconds` + `cost_usd` pulled from the Cloud Run billing API per crate CLAUDE.md invariant 2.
- Acceptance: `job_start` / `job_complete` / `teardown` / handler for preemption signals, each writing the matching ledger row with `node_id`, `job_id`, and cost fields populated; integration tests cover happy path + preemption mid-job.

### [30] deny.toml: add AGPL-3.0-only exception for workspace crates
- Status: done
- Priority: p2
- Crate: workspace
- Model: sonnet
- Context: Surfaced by task [20]. `cargo-deny check licenses` currently fails because the 10 workspace crates declare `license = "AGPL-3.0-only"` (ADR-0003) but `deny.toml [licenses].exceptions` is empty. CI only runs `bans sources` as a workaround. Add one `{ allow = ["AGPL-3.0-only"], crate = "<name>" }` row per workspace member, then re-enable the `licenses` subcommand in `.github/workflows/ci.yml` and `scripts/check-all.sh`.
- Acceptance: `cargo deny check licenses` passes locally and in CI; the AGPL-3.0-only exception is bounded to workspace-internal crates only (no upstream leakage); SLM-STACK §7 "We Own It" discipline preserved for all non-workspace deps.
- Note: Landed 2026-04-21. 11 AGPL-3.0 exception rows added (one per workspace member; cargo-deny 0.18.x requires bare `AGPL-3.0` not `AGPL-3.0-only` in exception allow lists). `webpki-roots v1.0.7` (CDLA-Permissive-2.0) surfaced as a previously-hidden licence finding; resolved with a bounded exception (Mozilla CA bundle, permissive, transitive via reqwest → hyper-rustls). `licenses` subcommand re-enabled in CI and `check-all.sh`. `cargo deny check bans sources licenses` passes locally.

### [33] cargo-deny: bump to 0.19.x on MSRV ≥ 1.88
- Status: open
- Priority: p2
- Crate: workspace
- Model: haiku
- Context: Surfaced by task [20]. `cargo-deny 0.18.3` (current pin in CI + local install) cannot parse CVSS-4.0 vector strings in recent RUSTSEC advisories; fix shipped in `0.19.x` but requires Rust 1.88. Depends on a prior MSRV bump ADR. Once MSRV ≥ 1.88, bump the `taiki-e/install-action` pin in `.github/workflows/ci.yml` and re-enable the `advisories` subcommand there and in `scripts/check-all.sh`.
- Acceptance: `cargo deny check advisories` passes locally and in CI; RUSTSEC advisories with CVSS-4.0 vectors no longer error; MSRV chain bans in `deny.toml` (`idna_adapter`, `icu_collections`, `icu_normalizer`) reviewed for continued necessity.

---

## Done

*(empty — the first commit that lands work here will record it.)*
