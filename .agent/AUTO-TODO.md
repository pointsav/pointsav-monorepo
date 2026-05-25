---
schema: foundry-auto-todo-v1
created: 2026-05-24
supersedes: AUTO-TODO-phase-6 (2026-05-23 version — Phases 0–6 done; 262/262 tests; 13 commits promoted to origin/main by Command Session)
author: totebox@project-intelligence (claude-sonnet-4-6, session 7)
brief: BRIEF-flow-restructure.md
decision_locked:
  phase_6_done: true            # latency_class + BackendLifecycle + GF-1 + GF-2 + model drift all done
  sprint_1_done: true           # service-content Ring 2/3 fix at b8a70ee; do NOT redo
  lbug_option_order: [B, A, C]  # Try MemoryMax=4G first; LBUG_BUILD_FROM_SOURCE second; pin 0.16.0 last
  sprints_2_5_scope: service-content  # Sprints 2-5 are service-content scope only (graph.rs + main.rs + http.rs)
---

# AUTO TODO — project-intelligence: service-content Sprints + lbug unblock

> **Phases 0–6 (service-slm) are DONE.** 262/262 tests. 13 commits on origin/main.
> This AUTO-TODO covers the next coding run: unblock service-content deployment,
> then Sprints 2–5, then deferred W5 items, then housekeeping.
>
> Work through phases IN ORDER. Each phase has an explicit gate.
> Commit after every discrete unit. Use `~/Foundry/bin/commit-as-next.sh "<msg>"`.
> Do NOT use `git commit` directly (pre-commit gate blocks it).
>
> **service-content sub-clone** is at `./service-content/` relative to this archive.
> It has a separate `.git/` — treat it as an independent repo for staging + commits.

---

## Phase 0 — Verify state (~10 min)

- [ ] **0A.** `cargo test --workspace` in `service-slm/` — confirm 262/262 green.
  If count is higher (other sessions added tests), note the delta; do not fail.
  If count is lower, stop and surface via outbox before continuing.

- [ ] **0B.** Confirm Sprint 1 is in service-content sub-clone:
  ```bash
  git -C service-content log --oneline | grep -i "sprint 1\|ring 2\|ring 3\|b8a70ee"
  ```
  Expected: commit b8a70ee (or its successor) visible in log. If absent, stop —
  Sprint 1 fix is missing; do not proceed to Phase 1 coding.

- [ ] **0C.** Check lbug / service-content deployment state:
  ```bash
  systemctl is-active local-content
  journalctl -u local-content -n 20 --no-pager
  ```
  Note the failure mode: OOM kill, linker error, or service simply not started.
  This determines which Phase 1 option to attempt first.

- [ ] **0D.** Check disk:
  ```bash
  df -h /srv/foundry
  ```
  Must be <90% before building. If >90%, stop — surface to Command via outbox.

**Gate:** 262/262 green; Sprint 1 confirmed; lbug state known; disk OK.

---

## Phase 1 — lbug build blocker (~30 min)

Critical path. service-content binary cannot deploy. Try options in order.

### Option B (try first — fastest): raise MemoryMax to 4G

The LBUG_SHARED=1 binary is already built. The failure is likely OOM at the 2G
systemd MemoryMax limit, not a linker failure.

```bash
sudo systemctl edit local-content.service
```

In the override editor, add:
```ini
[Service]
MemoryMax=4G
```

Then:
```bash
sudo systemctl daemon-reload
sudo systemctl start local-content.service
journalctl -u local-content -f
```

Watch for 30 seconds. If the service stays up and healthz returns 200:
```bash
curl -s http://127.0.0.1:9081/healthz
```
→ Phase 1 done. Proceed to Phase 2.

### Option A (if B fails): LBUG_BUILD_FROM_SOURCE

```bash
cd service-content
LBUG_BUILD_FROM_SOURCE=1 cargo build --release -p service-content 2>&1 | grep -E "error|warning" | head -30
```

Inspect the output. If undefined symbols: check linker search path in build.rs
or Cargo.toml `[package.metadata.lbug]`. Fix the linker flag and rebuild.

After successful build:
```bash
sudo systemctl start local-content.service
curl -s http://127.0.0.1:9081/healthz
```

### Option C (last resort): pin lbug = "=0.16.0"

Only if A fails. This triggers a ~45 min cmake build.

```bash
# In service-content/Cargo.toml, find the lbug dependency and pin it:
# lbug = { version = "=0.16.0", ... }
cargo build --release -p service-content
```

- [ ] **1Z.** Commit whichever option succeeded:
  ```
  ~/Foundry/bin/commit-as-next.sh "fix(service-content): lbug deploy — [option B|A|C used]"
  ```
  Commit goes in the service-content sub-clone's `.git/`, not the archive `.git/`.

**Gate:** `curl http://127.0.0.1:9081/healthz` returns 200 (not 503 warming, not connection refused).
After gate passes, note option used in the Phase 1 commit message.

---

## Phase 2 — service-content Sprint 2: Schema extension (~2 hr, ~150 LOC)

Adds `node_type: String` and `source_worm_id: Option<String>` to the `Entity`
schema (both SQLite GraphStore column and LadybugDB field), plus `RelatedTo` edge
writes.

**File:** `service-content/src/graph.rs`

- [ ] **2A.** Read `graph.rs` in full. Locate:
  - `Entity` struct definition
  - `SqliteGraphStore` impl (CREATE TABLE statement + INSERT)
  - `GraphStore` trait definition
  - Any existing `add_node` / `add_edge` methods

- [ ] **2B.** Add fields to `Entity`:
  ```rust
  pub node_type: String,
  pub source_worm_id: Option<String>,
  ```

- [ ] **2C.** Add `node_type TEXT NOT NULL DEFAULT 'generic'` and
  `source_worm_id TEXT` columns to the CREATE TABLE statement.
  If the table already exists at runtime, add a migration:
  ```sql
  ALTER TABLE entities ADD COLUMN node_type TEXT NOT NULL DEFAULT 'generic';
  ALTER TABLE entities ADD COLUMN source_worm_id TEXT;
  ```
  Guard with `IF NOT EXISTS` semantics (SQLite: wrap in a BEGIN; ignore
  "duplicate column" error, or check `pragma table_info`).

- [ ] **2D.** Add `RelatedTo` edge variant (or a string literal `"related_to"`)
  to the edge-type vocabulary used in INSERT statements.
  Add a `write_related_to(from_id: &str, to_id: &str)` method on `SqliteGraphStore`
  (or equivalent, matching whatever pattern `add_edge` already uses).

- [ ] **2E.** Update all `Entity { .. }` struct literals (in tests + callers) to
  include the two new fields. Use `node_type: "generic".to_string()` and
  `source_worm_id: None` as defaults in existing literals.

- [ ] **2F.** Verify existing round-trip tests still pass:
  ```bash
  cd service-content && cargo test 2>&1 | tail -5
  ```

- [ ] **2G.** Commit in service-content sub-clone:
  ```
  ~/Foundry/bin/commit-as-next.sh "feat(service-content): Sprint 2 — node_type/source_worm_id schema + RelatedTo writes"
  ```

**Gate:** `cargo test -p service-content` green; existing 8+ SqliteGraphStore
round-trip tests still pass.

---

## Phase 3 — service-content Sprint 3: PUSH inversion (~2 hr, net ~-40 LOC)

Inverts graph mutation flow: service-content POSTs mutations to Doorman
(`/v1/graph/mutate`) rather than writing the graph directly (PULL path deleted).
Doorman gains an in-memory mutation queue + the new endpoint.

**Files touched:**
- `service-content/src/main.rs` — delete PULL path
- `service-slm/crates/slm-doorman/src/` — add queue + handler
- `service-slm/crates/slm-doorman-server/src/http.rs` — register route

- [ ] **3A.** Read `service-content/src/main.rs`. Find the watcher loop that writes
  directly to the graph store. This is the PULL path to delete (~120 LOC).
  Also read `service-slm/crates/slm-doorman-server/src/http.rs` to see existing
  route registration pattern.

- [ ] **3B.** In `slm-doorman/src/`, add `graph_queue.rs` (or inline in `router.rs`):
  ```rust
  // In-memory, bounded channel for graph mutation requests.
  // Bounded at 1024 pending mutations; back-pressure on overflow.
  pub struct GraphMutationQueue {
      tx: tokio::sync::mpsc::Sender<GraphMutation>,
      rx: tokio::sync::Mutex<tokio::sync::mpsc::Receiver<GraphMutation>>,
  }
  ```
  Define `GraphMutation` as a simple enum or struct covering the operations
  service-content currently performs: `AddNode { entity: ... }`,
  `AddEdge { from, to, rel_type }`.

- [ ] **3C.** Add `POST /v1/graph/mutate` handler in
  `slm-doorman-server/src/http.rs`. The handler:
  - Deserialises the request body as `GraphMutation`
  - Enqueues via the queue's `tx.send(...).await`
  - Returns `202 Accepted` immediately (fire-and-forget queue)
  - Returns `503 Service Unavailable` if queue is full (channel full)

- [ ] **3D.** Delete the PULL path from `service-content/src/main.rs`.
  Replace with a `POST http://127.0.0.1:9090/v1/graph/mutate` call
  (Doorman's port) using `reqwest`. Use a fire-and-forget pattern
  (spawn a task; log errors; do not block the watcher loop).

- [ ] **3E.** Wire `GraphMutationQueue` into `AppState` in `slm-doorman-server/src/main.rs`.

- [ ] **3F.** `cargo test --workspace` green in service-slm.
  `cargo test -p service-content` green in service-content.

- [ ] **3G.** Two commits — one per sub-clone:
  ```
  # In service-content sub-clone:
  ~/Foundry/bin/commit-as-next.sh "feat(service-content): Sprint 3 — delete PULL graph write; POST to Doorman /v1/graph/mutate"

  # In service-slm sub-clone (this archive's .git/):
  ~/Foundry/bin/commit-as-next.sh "feat(slm): Sprint 3 — /v1/graph/mutate endpoint + GraphMutationQueue in Doorman"
  ```

**Gate:** `cargo test --workspace` green in both sub-clones.

---

## Phase 4 — service-content Sprint 4: /v1/draft/generate migration (~1 hr, ~120 LOC net)

Moves the `/v1/draft/generate` endpoint from service-content to slm-doorman.

**Files touched:**
- `service-content/src/http.rs` — delete handler (~120 LOC deletion)
- `service-slm/crates/slm-doorman-server/src/http.rs` — add equivalent handler

- [ ] **4A.** Read `service-content/src/http.rs`. Find the `/v1/draft/generate`
  handler. Copy its request/response types and logic before deleting.

- [ ] **4B.** Add equivalent handler in `slm-doorman-server/src/http.rs`.
  The handler behaviour is identical; only the binary that serves it changes.
  Port: Doorman listens on 9090.

- [ ] **4C.** Delete the handler from `service-content/src/http.rs`. Remove the
  route registration too. If this leaves `http.rs` empty or near-empty,
  consider whether the file should remain (leave it if other routes exist).

- [ ] **4D.** `cargo test --workspace` green in service-slm.
  `cargo test -p service-content` green.

- [ ] **4E.** Smoke-test the endpoint from Doorman:
  ```bash
  curl -s -X POST http://127.0.0.1:9090/v1/draft/generate \
    -H "Content-Type: application/json" \
    -d '{"prompt": "test"}' | head -5
  ```
  (May return an error body — that's fine. Confirm it reaches the handler.)

- [ ] **4F.** Two commits:
  ```
  # In service-content:
  ~/Foundry/bin/commit-as-next.sh "feat(service-content): Sprint 4 — /v1/draft/generate removed (migrated to Doorman)"

  # In service-slm:
  ~/Foundry/bin/commit-as-next.sh "feat(slm): Sprint 4 — /v1/draft/generate migrated to Doorman"
  ```

**Gate:** `cargo test --workspace` green; endpoint responds from Doorman port 9090.

---

## Phase 5 — service-content Sprint 5: processed_ledgers persistence (~1 hr, net ~+10 LOC)

Replaces the in-RAM `HashSet<String>` processed_ledgers with a graph query.
Eliminates the 114-file retry storm on service restart.

**Files touched:**
- `service-content/src/main.rs`
- `service-content/src/graph.rs` (query method needed)

- [ ] **5A.** Read `service-content/src/main.rs`. Find the `processed_ledgers`
  `HashSet<String>`. Find where entries are added (after a file is processed)
  and where it is checked (before processing a file).

- [ ] **5B.** In `graph.rs`, add a `is_already_processed(worm_id: &str) -> bool`
  query method on `SqliteGraphStore`:
  ```sql
  SELECT 1 FROM entities WHERE source_worm_id = ?1 LIMIT 1
  ```
  Returns `true` if a row with that `source_worm_id` exists.

- [ ] **5C.** In `main.rs`:
  - Remove the `HashSet<String>` processed_ledgers field (and its initialization).
  - Replace the "already processed?" check with a call to `graph.is_already_processed(worm_id)`.
  - The "mark as processed" step is already handled by Sprint 2's `source_worm_id` field
    being written when the node is added. No separate insert needed.

- [ ] **5D.** `cargo test -p service-content` green.

- [ ] **5E.** Deploy and verify the fix:
  ```bash
  sudo systemctl restart local-content.service
  sleep 5
  journalctl -u local-content -n 30 --no-pager
  ```
  Expected: no "processing 114 files" batch in the log. If batch appears,
  the graph query is not finding previously-processed entries — debug.

- [ ] **5F.** Commit in service-content:
  ```
  ~/Foundry/bin/commit-as-next.sh "fix(service-content): Sprint 5 — persistent processed_ledgers via graph query; eliminate 114-file retry storm"
  ```

**Gate:** restart service-content; `journalctl` shows no 114-file batch retry.

---

## Phase 6 — Yo-Yo W5 items — SPEC PASS FIRST (~30 min + coding)

The deferred G5/G6/G9/G11–G16/G18 items are referenced by number only in
`BRIEF-flow-restructure.md §8.C`. Definitions must be recovered before coding.

- [ ] **6A.** Read Phase 0 commit messages to recover G-item definitions:
  ```bash
  git show 35e2dea7 ed63476c a10539c6 --stat --format="%H %s%n%b"
  ```
  (Run in the service-slm sub-clone if these are slm commits.)

- [ ] **6B.** Document each deferred item as a concrete checklist entry here
  (edit this AUTO-TODO in place). Then implement in order.

- [ ] **6C.** Gate per item: `cargo test --workspace` green after each.

- [ ] **6D.** Commit per item:
  ```
  ~/Foundry/bin/commit-as-next.sh "fix(slm): W5 — G<N> <short description>"
  ```

**Gate:** all G-items defined + implemented; `cargo test --workspace` green.

---

## Phase 7 — Housekeeping (~20 min)

- [ ] **7A.** Create `service-content/CLAUDE.md`. Per framework §8, Active-state
  projects require a CLAUDE.md. service-content is Active. Use the template at
  `~/Foundry/templates/project-CLAUDE.md.tmpl`. Populate: project state, mission,
  current phase, key files (main.rs, graph.rs, http.rs), sprint history.

- [ ] **7B.** Commit:
  ```
  ~/Foundry/bin/commit-as-next.sh "docs(service-content): CLAUDE.md — project card (framework §8 requirement)"
  ```

- [ ] **7C.** Check manifest contamination:
  Read `.agent/manifest.md` — if it still contains project-gis content,
  note "manifest contamination unresolved" in outbox. Do NOT attempt to fix it
  from this Totebox session (Command scope).

**Gate:** `service-content/CLAUDE.md` exists; `git status` clean.

---

## Phase 8 — Shutdown ops

- [ ] **8A.** `cargo test --workspace` in service-slm — final green run. Note count.
  `cargo test` in service-content — note count.

- [ ] **8B.** Update `NEXT.md`:
  - Check off completed Sprint items (2, 3, 4, 5, lbug)
  - Note which phases remain (W5 G-items if not done)
  - Update Stage 6 carry-forward line with commit count

- [ ] **8C.** Update `BRIEF-flow-restructure.md` Status block:
  - Mark Sprints 2–5 + lbug done
  - Update `▶ RESUME HERE` to next open phase
  - Note commit hashes of last commits per sprint

- [ ] **8D.** Update `.agent/memory/session-context.md`:
  Prepend new entry per AGENT.md shutdown §2b. Keep only 3 most recent;
  push oldest to `session-context-archive.md`.

- [ ] **8E.** Prepend outbox message to `.agent/outbox.md`:
  Include: sprints done, commit count ahead of origin/main, Command actions needed
  (Stage 6 promote, slm-doorman-server binary rebuild + deploy, service-content binary deploy).

- [ ] **8F.** Commit ops files in archive `.git/`:
  ```
  ~/Foundry/bin/commit-as-next.sh "ops(intelligence): shutdown — NEXT.md + BRIEF + session-context + outbox"
  ```

- [ ] **8G.** `git log --oneline -10` in both sub-clones. Verify all sprint commits present.

- [ ] **8H.** Remove session lock:
  ```bash
  rm -f .agent/engines/claude-code/session.lock
  ```

**Gate:** `git status` clean in both sub-clones. Session lock removed.

---

## What AUTO does NOT do

These remain **Command Session scope** — surface via outbox, do not attempt:

- `~/Foundry/bin/promote.sh` (Stage 6) — hook BLOCKS from Totebox
- `sudo systemctl stop local-slm.service` — requires Command Session
- `sudo cp ... /usr/local/bin/` binary deploy — requires Command Session
- Elastic Compute Packer image rebuild — requires Command Session
- Fixing `.agent/manifest.md` contamination (project-gis content) — Command scope

---

## Deferred (not this session)

- G5/G6/G9/G11–G16/G18 W5 items (Phase 6 above): implement after spec recovery
- `README.es.md` refresh for `app-mediakit-knowledge` (cleanup-log open item)
- `service-parser` removal (cleanup-log open item; requires confirming no callers)
- `vendors-maxmind` → `vendor-maxmind` rename (cleanup-log open item)
