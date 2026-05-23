---
schema: foundry-auto-todo-v1
created: 2026-05-23
supersedes: AUTO-TODO-phases-0-5 (2026-05-22 version — all phases done)
author: totebox@project-intelligence (claude-sonnet-4-6, session 5)
brief: BRIEF-flow-restructure.md
decision_locked:
  sprint1_done: true       # Source node before Doorman call already at main.rs:338-356
  lbug_option: 1           # Accept ~13.5 MB disk bloat; lbug shared-link. Do NOT revisit.
  latency_class: add       # Approved Phase 6 item — add LatencyClass enum + ComputeRequest field
  backendlifecycle: add    # Approved Phase 6 item — trait wrapper for idle_monitor.rs
  tier_a_model: 1B         # Tier A = OLMo 2 1B specialist (NUC-class only); 7B is wrong. Do NOT revisit.
---

# AUTO TODO — project-intelligence Phase 6

> **Phases 0–5 are DONE.** Start here for Phase 6.
> Work through phases IN ORDER. Each phase has an explicit gate.
> Commit after every discrete unit. Use `~/Foundry/bin/commit-as-next.sh "<msg>"`.
> Do NOT use `git commit` directly (pre-commit gate blocks it).
>
> **Sprint 1 (service-content Ring 2/3 fix) is ALREADY DONE** — Source node write
> before Doorman call is at `service-content/src/main.rs:338-356`. Do not redo it.

---

## Phase 0 — Verify state before touching code

- [ ] **0A.** `cargo check --workspace` from `service-slm/` — must be clean.
- [ ] **0B.** `cargo test --workspace` — must show 260/260 pass (or higher if new tests
  exist from other sessions). If count differs, note the delta before continuing.
- [ ] **0C.** `git status` — confirm no uncommitted changes from prior sessions.
  If dirty: `git diff` to understand, then commit or stash before continuing.
- [ ] **0D.** Confirm Sprint 1 is in code:
  `grep -n "Sprint 1\|Source.*node\|write.*Source" service-content/src/main.rs`
  Expected: lines ~338–356 show Source node write + comment. If absent, Phase 0 fails —
  stop and surface via outbox to Command before proceeding.

**Gate:** all four checks pass. If 0B fails, diagnose before continuing.

---

## Phase 1 — Tier A model drift fixes (~15 min, doc + default only)

Documentation and default-value corrections only. No routing logic changes.
Rationale: `BRIEF-flow-restructure.md §0` (pre-doctrine-audit 7B error corrected 2026-05-22).

- [ ] **1A.** Edit `service-slm/crates/slm-core/src/tier.rs` — update the `Tier::Local`
  doc comment (line ~18):
  ```rust
  // BEFORE:
  /// Tier A — local OLMo 3 7B Q4 (mistral.rs / llama.cpp HTTP on this VM).
  // AFTER:
  /// Tier A — on-device OLMo 2 1B specialist (NUC-class hardware Toteboxes only).
  /// Unavailable on $7/mo e2-micro fleet nodes (DOCTRINE claim #54).
  ```

- [ ] **1B.** Edit `service-slm/crates/slm-doorman-server/src/main.rs` — update
  `SLM_LOCAL_MODEL` default (line ~414):
  ```rust
  // BEFORE:
  .unwrap_or_else(|_| "olmo-3-7b-instruct".to_string()),
  // AFTER:
  .unwrap_or_else(|_| "olmo-2-0425-1b-instruct".to_string()),
  ```
  Env var is not removed — operators can still override. Default now reflects the
  correct Tier A model for NUC-class hardware Toteboxes.

- [ ] **1C.** Edit `service-slm/CLAUDE.md` — three stale items:
  - "177/177 tests" → "260/260 tests" (Phase 5 added 49 tests)
  - Delete: "Tier A upgrade to OLMo 2 1124 7B Instruct pending (see NEXT.md)"
  - In the node-class table, if the NUC row says "7B", correct to "OLMo 2 1B specialist"

- [ ] **1D.** Commit:
  ```
  ~/Foundry/bin/commit-as-next.sh "docs(slm): reconcile Tier A model drift — 1B specialist (NUC) not 7B; test count 260"
  ```

**Gate:** `cargo check --workspace` clean.

---

## Phase 2 — `latency_class` field in ComputeRequest (~50 LOC, ~1 hr)

Corrected W1: routing is node-class-first, then `latency_class`, then `complexity`.
`latency_class` captures the caller's latency contract, not just task complexity —
a nightly extraction can be Low complexity but must route as Batch latency.

### 2A. Add `LatencyClass` to `service-slm/crates/slm-core/src/tier.rs`

After the `Complexity` enum, add:

```rust
/// Caller's latency contract for this request.
/// Used alongside `Complexity` and node-class to select a tier.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatencyClass {
    /// Respond as fast as possible; prefer on-device Tier A when available.
    #[default]
    Interactive,
    /// Can tolerate seconds of queuing; prefer Tier B for quality.
    Background,
    /// Nightly batch work; always routes via Tier B (route_yoyo_only pattern).
    Batch,
}

impl LatencyClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            LatencyClass::Interactive => "interactive",
            LatencyClass::Background => "background",
            LatencyClass::Batch => "batch",
        }
    }
}
```

### 2B. Add to `pub use tier::{...}` in `service-slm/crates/slm-core/src/lib.rs`

```rust
pub use tier::{Complexity, LatencyClass, SpeculationRequest, Tier};
```

### 2C. Add field to `ComputeRequest` in `slm-core/src/lib.rs` (after `complexity` ~line 173)

```rust
#[serde(default)]
pub latency_class: LatencyClass,
```

### 2D. Update `select_tier()` in `slm-doorman/src/router.rs` (line ~196)

Add `latency_class` as the first routing signal (before complexity), consistent
with node-class-first routing (`BRIEF-flow-restructure.md §7`):

```rust
// Batch latency always routes to Tier B — never local, never External without label.
if req.latency_class == LatencyClass::Batch {
    return self.require_tier(Tier::Yoyo, req);
}
// Interactive on a node with on-device Tier A: prefer Tier A unless complexity is High.
// (Node-class gate in build_doorman() already prevents Tier A on Micro nodes.)
```

Read the existing `select_tier()` logic carefully before editing — slot `latency_class`
in as the first check without disrupting the node-class gating already present.

### 2E. Fix `minimal_request()` helper and any ComputeRequest struct literals in tests

Add `latency_class: LatencyClass::default()` to any `ComputeRequest { .. }` struct
literal in tests that fails to compile (exhaustive struct construction).

### 2F. Add targeted tests in `slm-core/src/lib.rs` or `slm-doorman/tests/`

- `batch_latency_routes_yoyo()` — `latency_class: Batch` → result is `Yoyo` (or
  `TierUnavailable` if Yo-Yo not configured; not `Local`)
- `interactive_prefers_local_when_available()` — `latency_class: Interactive`,
  complexity Low, Tier A available → result is `Local`
- Existing tests must pass without modification (`#[serde(default)]` covers missing field)

- [ ] **2G.** Commit:
  ```
  ~/Foundry/bin/commit-as-next.sh "feat(slm): LatencyClass enum + ComputeRequest field (corrected W1)"
  ```

**Gate:** `cargo test --workspace` green. `cargo clippy --workspace -- -D warnings` clean.

---

## Phase 3 — `BackendLifecycle` trait for `idle_monitor.rs` (~80 LOC, ~1 hr)

Quarantine the Yo-Yo idle monitor behind a trait so core routing logic does not
directly reference the concrete monitor struct. This is `BRIEF-flow-restructure.md §8.C`
"Broker discipline" item.

File: `service-slm/crates/slm-doorman-server/src/idle_monitor.rs`

### 3A. Check the pinned Rust toolchain version first

```bash
cat service-slm/rust-toolchain.toml
```

If stable ≥ 1.75: use `impl Future` in trait (RPITIT).
If stable < 1.75: use `Pin<Box<dyn Future<Output = ()> + Send>>` instead.

### 3B. Define `BackendLifecycle` trait

Add to `service-slm/crates/slm-doorman/src/backend_lifecycle.rs` (new file) or
inline in `idle_monitor.rs`:

```rust
// If Rust ≥ 1.75 (RPITIT):
pub trait BackendLifecycle: Send + Sync + 'static {
    fn start(&self) -> impl std::future::Future<Output = ()> + Send;
    fn stop(&self) -> impl std::future::Future<Output = ()> + Send;
    fn is_healthy(&self) -> impl std::future::Future<Output = bool> + Send;
    fn name(&self) -> &'static str;
}

// If Rust < 1.75 (use boxed futures):
pub trait BackendLifecycle: Send + Sync + 'static {
    fn start(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>;
    fn stop(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>;
    fn is_healthy(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool> + Send>>;
    fn name(&self) -> &'static str;
}
```

### 3C. Implement `BackendLifecycle` for the idle monitor handle type

Find the public type exported by `idle_monitor.rs`. Implement the trait:
- `start()` → send a start signal (call `instances.start` or send to monitor task)
- `stop()` → send a stop signal (idle-stop the VM)
- `is_healthy()` → poll the Yo-Yo `/metrics` endpoint once with a short timeout
- `name()` → `"yoyo-idle-monitor"`

### 3D. Update usage site in `slm-doorman-server/src/main.rs`

Replace the direct `IdleMonitorConfig` or handle type with
`Option<Arc<dyn BackendLifecycle>>` in `AppState` (or equivalent). The spawn
logic itself stays the same; only the type held in state changes.

### 3E. Commit:
```
~/Foundry/bin/commit-as-next.sh "refactor(slm): BackendLifecycle trait; quarantine idle_monitor behind it"
```

**Gate:** `cargo test --workspace` green.

---

## Phase 4 — GF-1: async audit writes off hot path (~30 min)

`write_audit()` in `slm-doorman/src/router.rs` (line ~375) is called inside the
async `dispatch()` path.

- [ ] **4A.** Read `write_audit()` — is it `fn` (sync) or `async fn`?
  - **If sync with `std::fs::write` or similar blocking I/O:** wrap the call in
    `tokio::task::spawn_blocking(move || { ... })`. Do NOT await the handle —
    fire-and-forget; log errors via `JoinHandle` if needed.
  - **If already using `tokio::fs` or already spawned:** note "GF-1 already
    addressed" in commit message and close this phase.

- [ ] **4B.** Commit if changed:
  ```
  ~/Foundry/bin/commit-as-next.sh "fix(slm): GF-1 — audit ledger write off async hot path"
  ```

**Gate:** `cargo test --workspace` green.

---

## Phase 5 — GF-2: Tier A inference client timeouts (~20 min)

`LocalTierClient` in `slm-doorman/src/tier/local.rs` (line ~38) uses
`reqwest::Client::new()` for inference — no timeout. The health-check client has
500 ms (line ~58) but inference does not. A hung Tier A call blocks a Doorman
async task indefinitely.

- [ ] **5A.** Find the inference `reqwest::Client` construction (line ~38).
  Replace with a builder that sets timeouts:
  ```rust
  const LOCAL_INFERENCE_TIMEOUT_SECS: u64 = 180;

  reqwest::Client::builder()
      .connect_timeout(std::time::Duration::from_secs(5))
      .timeout(std::time::Duration::from_secs(LOCAL_INFERENCE_TIMEOUT_SECS))
      .build()
      .expect("LocalTierClient inference HTTP client")
  ```
  180 s is a ceiling — the Doorman's outer deadline (60 s socket + 90 s outer)
  is shorter, so this primarily prevents silent hangs after the outer deadline.

- [ ] **5B.** Commit:
  ```
  ~/Foundry/bin/commit-as-next.sh "fix(slm): GF-2 — Tier A inference client connect + read timeouts"
  ```

**Gate:** `cargo test --workspace` green. `cargo clippy --workspace -- -D warnings` clean.

---

## Phase 6 — Shutdown ops

- [ ] **6A.** `cargo test --workspace` — final green run. Note total test count.

- [ ] **6B.** Update `NEXT.md`:
  - Check off Phase 6 items completed (latency_class, BackendLifecycle, GF-1, GF-2, model drift)
  - Note which items were already done (Sprint 1) vs done this session
  - Update Stage 6 carry-forward line with correct commit count

- [ ] **6C.** Update `BRIEF-flow-restructure.md` Status block:
  - Mark Phase 6 items complete in §8.C
  - Update `▶ RESUME HERE` to Phase 7 (W5 remainder — G5/G6/G9/G11–G16/G18)
  - Note commit hash of last Phase 6 commit

- [ ] **6D.** Update `.agent/memory/session-context.md`:
  Prepend new entry per AGENT.md shutdown step §2b. Include done/pending/operator
  preferences columns. Keep only 3 most recent entries; push oldest to
  `session-context-archive.md`.

- [ ] **6E.** Prepend outbox message to `.agent/outbox.md`:
  ```
  re: Phase 6 complete — Stage 6 + Command ops needed
  Phase 6 done: latency_class + BackendLifecycle + GF-1 + GF-2 + model drift docs.
  Local main is now N commits ahead of origin/main (exact count: git log --oneline origin/main..HEAD | wc -l).
  Command needs: (1) Stage 6 promote, (2) build+deploy slm-doorman-server binary,
  (3) stop local-slm + SLM_FORCE_BROKER_MODE=true (see flow-restructure §12).
  ```

- [ ] **6F.** Commit ops files:
  ```
  ~/Foundry/bin/commit-as-next.sh "ops(intelligence): Phase 6 done — NEXT.md + BRIEF + session-context + outbox"
  ```

- [ ] **6G.** Run `git log --oneline -8` and verify the Phase 6 commits are all present.

**Gate:** `git status` clean. All modified files committed. Session lock at
`.agent/engines/claude-code/session.lock` is NOT committed (gitignored — do not stage).

---

## What AUTO does NOT do

These remain **Command Session scope** — surface via outbox, do not attempt:

- `bin/promote.sh` (Stage 6) — hook BLOCKS from Totebox
- `sudo systemctl stop local-slm.service` — requires Command Session
- `sudo cp ... /usr/local/bin/` binary deploy — requires Command Session
- Elastic Compute Packer image rebuild — requires Command Session

Full checklist: `BRIEF-flow-restructure.md §12`.

---

## Deferred to Phase 7 (W5 remainder — Elastic Compute paid tier)

`BRIEF-flow-restructure.md §8.C` items G5/G6/G9/G11–G16/G18.
Not in this session. Open a new AUTO-TODO for Phase 7 when Phase 6 is confirmed promoted.
