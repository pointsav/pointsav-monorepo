---
schema: foundry-auto-todo-v1
created: 2026-05-22
author: task@project-intelligence (claude-sonnet-4-6)
brief: BRIEF-flow-restructure.md
decision_locked:
  lbug_option: 1  # Accept ~13.5 MB disk bloat; lbug compiles in but dormant on Micro nodes.
                  # SqliteGraphStore (Phase 3) solves the 2 GB RAM problem independently.
                  # One binary, all tiers, zero friction. Do NOT revisit this decision.
---

# AUTO TODO — project-intelligence Totebox

Work through phases in order. Each phase has an explicit gate before proceeding.
Commit after every discrete unit of work using `~/Foundry/bin/commit-as-next.sh`.
Do NOT use `git commit` directly (pre-commit gate blocks it).

---

## Phase 0 — Inbox tasks (no decisions needed; do these first)

- [ ] **0-A** Write `.agent/binary-targets.yaml` declaring `slm-doorman-server`:
  ```yaml
  schema: foundry-binary-targets-v1
  cluster: project-intelligence
  targets:
    - product_id: slm-doorman-server
      binary_name: slm-doorman-server
      source_crate: service-slm
      license: Apache-2.0
      license_tier: apache
      class: service-package
      layer: extension
      requires: [os-console]
      platforms: [x86_64-unknown-linux-gnu]
      soft_enabled: true
  ```
  Commit: `ops(binary-targets): declare slm-doorman-server for SOFT- pipeline`
  Mark inbox message `command-20260522-binary-targets-project-intelligence` as actioned.

- [ ] **0-B** Migrate `.agent/plans/*.md` → `.agent/briefs/BRIEF-*.md`:
  1. `git mv .agent/plans/<file>.md .agent/briefs/BRIEF-<file>.md` for each file
     (files: KNOWLEDGE-PLATFORM-PLAN, KNOWLEDGE-PLATFORM-VISION, MASTER-PLAN-2026,
     README, claim-authoring-convention.PROPOSAL, flow-bottleneck-strategic-review-2026-05-21,
     lbug-build-blocker, learning-loop-master-plan-2026-05-18, olmo-performance-tuning,
     service-audit-2026-05-16, service-content-architecture-2026, service-slm-architecture-2026,
     service-slm-hardening-2026-05-18, sovereign-routing-comprehensive, tier-architecture-2026,
     universal-ai-gateway — plus `.agent/plans/archive/` contents)
  2. Add frontmatter `artifact: brief` + `status: active` to each
  3. Pick up 2 briefs from workspace root:
     - `cp ~/Foundry/.agent/briefs/BRIEF-phase-3c-service-content-loRA-stub.md .agent/briefs/`
     - `cp ~/Foundry/.agent/briefs/BRIEF-layer3-compliance-report.md .agent/briefs/`
  4. Create `.agent/briefs/README.md` listing all active briefs
  5. Commit: `ops(briefs): migrate plans/ → briefs/; BRIEF- prefix; pick up 2 workspace briefs`
  Mark inbox message `command-20260521-briefs-migration-project-intelligence` as actioned.

**Gate:** Both inbox messages marked actioned in inbox.md before proceeding to Phase 1.

---

## Phase 1 — Archive alignment (§8.A — do alongside engineering; low effort)

- [ ] **1-A** Log the DOCTRINE claim #49 vs `tier-zero-customer-side-sovereign-specialist.md`
  working-set conflict in `NEXT.md`:
  > Conflict: claim #49 "full substrate at $7/mo" vs convention §1 "2–4 GB working set".
  > Resolution: $7 node = deterministic substrate only (no model, claim #54);
  > "full substrate incl. 1B specialist" = NUC rung. Command to amend convention wording.

- [ ] **1-B** Prepend outbox message to `.agent/outbox.md` — notify Command that the original
  flow-restructure investigation missed ratified conventions #49/#54/four-tier/tier-zero.
  Recommend a doctrine cross-check step for future architecture briefs.

- [ ] **1-C** Update `.agent/manifest.md` `deployment:` leg — target shape is $7/mo e2-micro
  fleet node, not the workspace VM. (Note: current manifest.md contains project-knowledge
  content due to cross-cluster contamination — flag this in the outbox message in 1-B;
  do not rewrite the contaminated manifest, just note it.)

- [ ] **1-D** Update BRIEF-flow-restructure.md §8.A checkboxes as completed.

**Gate:** Commit all Phase 1 items: `docs(archive): §8.A alignment — NEXT.md conflict, outbox, manifest note`

---

## Phase 2 — `foundry-nodeclass` crate (§8.B) — §9 STEP 1

**This is the first dependency. Both service-slm and service-content depend on it.**

Location: `service-slm/crates/foundry-nodeclass/` (new leaf crate, standalone — no workspace
coupling issues).

### 2-A: Crate scaffold
Create `service-slm/crates/foundry-nodeclass/Cargo.toml`:
```toml
[package]
name = "foundry-nodeclass"
version = "0.1.0"
edition = "2021"

[dependencies]
# no external deps — pure stdlib + sysfs reads
```
Add to `service-slm/Cargo.toml` workspace members if present.

### 2-B: Core types (`src/lib.rs`, ~150 LOC)
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeClass { Micro, Hardware, Accelerated }

pub struct Capabilities { pub node_class: NodeClass, pub ram_gib: f64, pub vcpu: f64, pub has_gpu: bool }

impl Capabilities {
    pub fn supports_on_node_ai(&self) -> bool {
        matches!(self.node_class, NodeClass::Hardware | NodeClass::Accelerated)
    }
}
```

### 2-C: `detect()` function
Priority order for RAM: cgroup v2 `memory.max` → cgroup v1 `memory.limit_in_bytes` → `/proc/meminfo` MemTotal. Take the minimum of cgroup limit and physical RAM.
Priority order for vCPU: cgroup v2 `cpu.max` (quota/period) → `nproc`.
GPU probe: `Path::new("/dev/nvidia0").exists() || glob("/dev/dri/renderD*").any()` — no CUDA link.

Classification:
- `has_gpu` → `Accelerated`
- `ram_gib >= 6.0 && vcpu >= 1.5` → `Hardware`
- else → `Micro`

### 2-D: `TOTEBOX_NODE_CLASS` env override
```rust
if let Ok(val) = std::env::var("TOTEBOX_NODE_CLASS") {
    match val.as_str() {
        "micro" => return Capabilities { node_class: NodeClass::Micro, .. },
        "hardware" => ...,
        "accelerated" => ...,
        _ => {} // fall through to probe
    }
}
```

### 2-E: Unit tests
- `test_micro_override()` — `TOTEBOX_NODE_CLASS=micro` → `Micro`
- `test_hardware_override()` — `TOTEBOX_NODE_CLASS=hardware` → `Hardware`
- `test_supports_on_node_ai()` — Micro returns false, Hardware returns true

Commit: `feat(foundry-nodeclass): new leaf crate — NodeClass probe + TOTEBOX_NODE_CLASS override`

**Gate:** `cargo test -p foundry-nodeclass` green before proceeding.

---

## Phase 3 — `SqliteGraphStore` + runtime backend selection (§8.D) — §9 STEP 2

**This is THE blocker. Nothing about the $7-node fleet is real until this is done.**

**lbug decision: LOCKED as Option 1.** lbug C++ stays compiled into the binary.
It is dormant (never called) on Micro nodes. ~13.5 MB disk bloat accepted.

### 3-A: Add `rusqlite` dependency to `service-content/Cargo.toml`
```toml
rusqlite = { version = "0.31", features = ["bundled"] }
```
`bundled` = no system sqlite3 required; compiles SQLite from source. Required for $7-node deploy.

### 3-B: `SqliteGraphStore` implementation (`service-content/src/graph_sqlite.rs`, ~250 LOC)

Schema — two tables mirroring LadybugDB's Entity/RelatedTo shape:
```sql
CREATE TABLE IF NOT EXISTS entities (
    id TEXT PRIMARY KEY,
    module_id TEXT NOT NULL,
    classification TEXT NOT NULL,
    label TEXT NOT NULL,
    summary TEXT,
    location TEXT,
    worm_id TEXT,
    cites_json TEXT,
    created_at TEXT,
    updated_at TEXT
);
CREATE TABLE IF NOT EXISTS relations (
    from_id TEXT NOT NULL,
    to_id TEXT NOT NULL,
    relation_type TEXT NOT NULL,
    PRIMARY KEY (from_id, to_id, relation_type)
);
CREATE INDEX IF NOT EXISTS idx_entities_module ON entities(module_id);
```

Implement every method of the `GraphStore` trait (see `service-content/src/graph.rs`):
- `init_schema()` — run CREATE TABLE IF NOT EXISTS
- `upsert_entities()` — INSERT OR REPLACE
- `query_context()` — FTS or LIKE search on label+summary
- `list_entities()` — SELECT WHERE module_id = ?
- `count_all()` — SELECT COUNT(*)
- `delete_by_classification()` — DELETE WHERE module_id = ? AND classification = ?
- `delete_by_classification_and_location()` — DELETE WHERE module_id = ? AND classification = ? AND location = ?

### 3-C: Runtime backend selection (`service-content/src/main.rs`)

```rust
let caps = foundry_nodeclass::Capabilities::detect();
let graph: Arc<dyn GraphStore> = match caps.node_class {
    NodeClass::Micro => {
        let path = base_dir.join("graph.sqlite");
        Arc::new(SqliteGraphStore::open(&path)?)
    }
    _ => {
        // Hardware / Accelerated — use LadybugDB
        Arc::new(LbugGraphStore::open(&base_dir)?)
    }
};
```

`SERVICE_CONTENT_GRAPH_BACKEND` env override:
- `"sqlite"` → force SqliteGraphStore regardless of node class
- `"ladybug"` → force LbugGraphStore regardless of node class

### 3-D: Background the CORPUS drain
The 16-min synchronous scan on startup must become async:
```rust
let graph_clone = graph.clone();
tokio::spawn(async move {
    graph_clone.drain_corpus().await;
});
```
`/healthz` must return `{"status": "warming"}` until drain completes, then `{"status": "ready"}`.
`/v1/graph/context` returns HTTP 503 with body `{"error": "warming"}` while not ready.
Add a `AtomicBool` ready flag, set to true when drain completes.

### 3-E: Fix legacy hardcoded `base_dir` default (`main.rs`)
Find the hardcoded path (likely something like `/var/lib/service-content` or a dev path)
and replace with a proper CLI arg / env var defaulting to `$STATE_DIR` or `/var/lib/service-content`.

Commit: `feat(service-content): SqliteGraphStore backend + runtime node-class selection (§8.D)`

**Gate:** `cargo test -p service-content` green. Then run manual smoke:
```bash
TOTEBOX_NODE_CLASS=micro SERVICE_CONTENT_GRAPH_BACKEND=sqlite \
  cargo run -p service-content -- --base-dir /tmp/test-content
# In another terminal:
curl http://localhost:<port>/healthz
# Should return {"status":"warming"} then {"status":"ready"} within a few seconds
```

---

## Phase 4 — service-slm Doorman node-class fixes (§8.C) — §9 STEP 3

**Stop the Doorman falsely reporting Tier A exists on a $7 node.**

Add `foundry-nodeclass` dependency to `service-slm/crates/slm-doorman/Cargo.toml`.

### 4-A: `build_doorman()` in `main.rs`
Current: `local` backend is unconditionally `Some(...)` — always claims Tier A exists.
Fix: gate on `caps.supports_on_node_ai()`:
```rust
let caps = foundry_nodeclass::Capabilities::detect();
let local = if caps.supports_on_node_ai() || env_force_local() {
    Some(build_local_backend()?)
} else {
    None  // Micro node: no Tier A
};
```
Thread `caps` / `node_class` into the `Doorman` struct.

### 4-B: `select_tier()` in router
Node-class-first policy. On Micro, never route to `Tier::Local`:
```rust
if request.tier == Tier::Local && self.node_class == NodeClass::Micro {
    return Err(TierError::Unavailable {
        tier: Tier::Local,
        reason: "node-class:micro — no on-node AI; configure Tier B or C".into(),
    });
}
```
Delete the documentation line "Interactive never defaults to Local" — it contradicted doctrine.
Add invariant test: `test_micro_never_routes_local()`.

### 4-C: `/readyz` honest reporting
Report from probe, never from model-load attempt:
```json
{
  "node_class": "micro",
  "tier_a": "unavailable",
  "tier_a_reason": "node-class:micro",
  "tier_b": "configured|unconfigured",
  "tier_c": "configured|unconfigured",
  "ai_available": false
}
```

### 4-D: `slm-doorman.service` systemd unit
Change `Requires=local-slm.service` (if present) to `Wants=local-slm.service`
so the Doorman starts on Micro nodes where local-slm is absent.

### 4-E: `latency_class` field in `slm-core`
Add `latency_class: Option<LatencyClass>` to the request struct (W1 corrected).
`LatencyClass { Interactive, Batch }` — routing hint, not a mandate.

Commit: `feat(service-slm): node-class-first Doorman — no false Tier A on Micro nodes (§8.C)`

**Gate:** `cargo test -p slm-doorman` green. Confirm `TOTEBOX_NODE_CLASS=micro` + `/readyz`
returns `tier_a: unavailable`.

---

## Phase 5 — Base-tier tests + cgroup sandbox (§8.E) — §9 STEP 4

**Stand up early; this is how every later change is verified against the $7-node target.**

### 5-A: Integration tests — `tests/micro_node.rs` in service-slm
```rust
// TOTEBOX_NODE_CLASS=micro — broker has no Tier A, /readyz honest
#[test]
fn micro_node_no_tier_a() { ... }
#[test]
fn micro_node_ai_request_returns_503() { ... }
```

### 5-B: Integration tests — `tests/micro_node.rs` in service-content
```rust
// TOTEBOX_NODE_CLASS=micro — SqliteGraphStore selected, round-trips entities
#[test]
fn micro_node_sqlite_backend_selected() { ... }
#[test]
fn micro_node_graph_roundtrip() { ... }
```

### 5-C: cgroup sandbox script (`scripts/run-micro-sandbox.sh`)
```bash
#!/usr/bin/env bash
# Runs service-content in a 1 GB / 25% CPU cgroup slice (no container required).
# A wrong backend (LadybugDB) will be OOM-killed automatically.
exec systemd-run --user \
  -p MemoryMax=1G \
  -p CPUQuota=25% \
  --wait \
  "$@"
```
Usage: `./scripts/run-micro-sandbox.sh ./target/release/service-content --base-dir /tmp/test`

### 5-D: Node-class CI matrix (`tests/node_class_matrix.rs` in service-content)
Test every (node-class × backend) cell: Micro+SQLite, Hardware+Ladybug.
The deterministic operations suite (upsert, query, count, delete) passes in every row.

Commit: `test(base-tier): micro-node integration tests + cgroup sandbox script (§8.E)`

**Gate:** All new tests green. Run the cgroup sandbox manually — service-content must start
and serve `/healthz` within the 1 GB limit using SQLite backend.

---

## Phase 6 — Small wins (§8.C remainder + content reliability) — §9 STEPS 5-7

These are parallel and independent. Do them in any order.

- [ ] **6-A GF-1** Async audit — move any blocking I/O calls off the hot request path in
  service-slm. Check `idle_monitor.rs` for sync sleeps; wrap in `tokio::task::spawn_blocking`.

- [ ] **6-B GF-2** Tier A client timeouts — add explicit connect + request timeouts to the
  local Tier A client (currently may block indefinitely if local-slm is unresponsive).

- [ ] **6-C** Broker discipline — quarantine `idle_monitor.rs` behind a `BackendLifecycle` trait.
  Tidiness improvement; not a blocker.

- [ ] **6-D** Reconcile Tier A model drift — `env` files reference 7B-Think; correct to OLMo 2 1B
  for the NUC tier. Pin model name. Surface to Command for `permissible-model-substrate.md`.

- [ ] **6-E** Content reliability pass in service-content:
  - `processed_ledgers` → `HashSet` (dedup guard)
  - Remove any `unwrap()`/`expect()` on write paths (replace with logged errors)
  - Persistent deferred-retry queue for failed entity writes

Commit per item: `fix(service-slm): GF-1 async hot path`, `fix(service-slm): GF-2 tier-a timeouts`, etc.

---

## Phase 7 — W5 Yo-Yo paid tier remainder (§8.C W5) — §9 STEP 8

Independent of all above — can be done any time after Phase 4.

Remaining Yo-Yo items from BRIEF §8.C (Phase 0 already shipped G1/G3/G7/G8/G10/G17):
- [ ] G5 — Yo-Yo pre-warm signal
- [ ] G6 — Yo-Yo grace shutdown on budget threshold
- [ ] G9 — Billing event webhook
- [ ] G11–G16 — remaining hardening items (see BRIEF §8.C W5)
- [ ] G18 — final Yo-Yo audit log format

Commit per logical group.

---

## Phase 8 — §8.F gated (do NOT start until gate conditions met)

**GATED. Do not start this phase until:**
1. §8.D done — `service-content` boots on SQLite on a $7 node ✓
2. $7-node fleet verified booting end-to-end ✓
3. A named hardware-Totebox customer exists ✓

Phase 8 is: Tier A on-device 1B specialist — `LocalInferenceBackend` trait, accelerator backends,
model packaging. This is the NUC-rung product. Not now.

---

## Shutdown checklist (run before ending AUTO session)

- [ ] All commits made via `commit-as-next.sh` (never `git commit` directly)
- [ ] Inbox messages for completed items marked `status: actioned`
- [ ] Outbox messages prepended (not appended) to `.agent/outbox.md`
- [ ] BRIEF-flow-restructure.md §8 checkboxes updated to reflect what was completed
- [ ] NEXT.md updated with any new deferred/blocked items
- [ ] `git status` clean (or intentional untracked files explained)
- [ ] Update `.agent/memory/session-context.md` (prepend new entry; keep 3 most recent)
- [ ] Remove session lock: `rm .agent/engines/claude-code/session.lock`
