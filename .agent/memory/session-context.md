# Session Context — project-intelligence archive

Rolling 3-session summary. Newest on top. Keep only 3 entries; push oldest to `session-context-archive.md`.

⚠️ Prior entries in this file were project-knowledge content (cross-cluster contamination
from Command Stage-6 rebase 2026-05-22 — flagged for Command; see outbox).

---

## Session: 2026-05-24 session 16 | Role: totebox | Engine: claude-sonnet-4-6

### Done this session
- **Phase 1 (lbug deploy) complete**: raised `MemoryMax=4G` / `MemoryHigh=3800M` in
  `/etc/systemd/system/local-content.service.d/memory.conf`; `local-content.service`
  now returns HTTP 200 from `/healthz` with 1,529+ entities in LadybugDB.
- **Sprint 2 (service-content)**: `node_type: String` added to `GraphEntity`; `write_related_to()`
  added to `GraphStore` trait + both backends; `related_to` table in SQLite; 22/22 tests. Commit `14b8c1ef`.
- **Sprint 5 (service-content)**: `is_already_processed(source_worm_id)` added to `GraphStore` trait
  + both backends; corpus drain loop checks graph instead of JSONL file; eliminates restart retry storm;
  23/23 tests. Commit `89ff3dbc`.
- **service-content/CLAUDE.md**: project card created. Commit `c5dd8446`.
- **Phase 6 (W5 G-items)**: BRIEF-flow-restructure.md no longer exists; G5/G6/G9/G11-G16/G18
  unrecoverable. Flagged to Command via outbox. Treated as superseded by 2026-05-23 session.

### Pending / carry-forward
- **Stage 6 promote** — service-content sub-clone has 3 new commits (Sprints 2/5 + CLAUDE.md)
  plus project-intelligence archive has outbox/session-context updates to commit.
  Command must rebase + promote. Prior session's stage6-rebase-required still applies.
- **Infrastructure tracking** (Command scope): `memory.conf` + `crash-loop-guard.conf` in
  `/etc/systemd/system/local-content.service.d/` not tracked in `infrastructure/`. Outbox flagged.
- **Sprint 3 (PUSH inversion)**: deferred — Doorman already has `/v1/graph/mutate` proxy;
  in-memory queue design requires service-slm changes; next coding session.
- **Sprint 4 (/v1/draft/generate migration)**: deferred.
- **G-items (G5/G6/G9/G11-G16/G18)**: unrecoverable; Command to confirm disposition.
- **`is_already_processed` LbugGraphStore test**: only SQLite backend is tested (LbugGraphStore
  requires a live lbug db file; test infrastructure is SQLite-only).

### Operator preferences surfaced
- AUTO mode confirmed — no interruptions through phases; operator directs by plan only.

---

## Session: 2026-05-23 session 6 | Role: totebox | Engine: claude-sonnet-4-6

### Done this session
- **Phase 6 of AUTO-TODO complete** — all four deferred items from §8.F shipped (262/262 tests):
  - `LatencyClass` enum (`Interactive`/`Background`/`Batch`) added to slm-core; `select_tier()` routes Batch→Yoyo first; 2 new routing tests. Commit `b0a... / (latency_class)`.
  - `BackendLifecycle` object-safe trait in `idle_monitor.rs`; `IdleMonitorHandle` wraps spawn; `AppState.idle_monitor: Option<Arc<dyn BackendLifecycle>>`; all AppState struct literals patched across lib.rs + 4 test files (micro_node, http_test, audit_endpoints, anthropic_shim).
  - **GF-1**: `AuditLedger` made `Clone` (Arc-wrapped mutex); `write_audit()` fires append into `tokio::task::spawn_blocking` fire-and-forget.
  - **GF-2**: `LocalTierClient` `reqwest::Client::builder()` with `connect_timeout(5s)` + `timeout(180s)`.
  - Model drift: `SLM_LOCAL_MODEL` default → `"olmo-2-0425-1b-instruct"`; `Tier::Local` doc corrected.
- Commits: `21281703` (BackendLifecycle), `a689ec1e` (GF-1), `28f666bf` (GF-2), + ops commit.
- BRIEF-flow-restructure Phase 6 marked done; NEXT.md updated; session-context rotated.

### Pending / carry-forward
- **Stage 6 promote** — Command Session scope; local main ~11 commits ahead of origin/main; rebase required per inbox `command-20260520-stage6-rebase-required` before promote.
- **After promote**: `bin/sync-local.sh --all` + rebuild + redeploy `slm-doorman-server` on workspace VM; update `local-doorman.service` env `SLM_LOCAL_MODEL=olmo-2-0425-1b-instruct`.
- **service-content Ring 2/3 fix** (~30 LOC, main.rs:198) — write Source node before Doorman call.
- Phases 7–8 of AUTO-TODO (Yo-Yo W5 remainder, Packer rebuild) — Command scope.

### Operator preferences surfaced
- AUTO mode with no interruptions — operator lets all phases run through to shutdown.

---

## Session: 2026-05-23 session 4 | Role: totebox | Engine: claude-sonnet-4-6

### Done this session
- **Phase 5 complete** (commit `32213020`, jwoodfine):
  - `service-slm/crates/slm-doorman-server/tests/micro_node.rs` — 5 integration tests: readyz, 503 clean rejection, healthz, force-broker-mode, hardware sanity
  - `service-content/src/graph.rs` — 8 `SqliteGraphStore` round-trip tests (already landed in fmt pass `4b20c3e3`)
  - `scripts/run-micro-sandbox.sh` — `systemd-run --user -p MemoryMax=1G -p CPUQuota=25%` cgroup sandbox
  - `CLAUDE.md` archive guide added (was untracked since prior session)
  - 260/260 tests pass across service-slm workspace
- **Ops commit** (`dcdd2a58`, pwoodfine): BRIEF-flow-restructure Phase 5 DONE; NEXT.md updated; Phase 6 deferred items noted
- **VM resource diagnosis** (via Opus systems-engineer agent): llama-server swap/CPU issue traced to wrong model (7B on CPU, neither Tier A nor Tier B); actual swap cause is 15+ idle Claude shells; llama-server ctx-size reduced 4096→2048 as hotpatch; service-content restart attempted (still loading)
- **Architecture cross-check**: workspace VM is e2-standard-8 (Hardware-class); 7B model wrong for both tiers; §8.F gates NUC on-device AI behind named customer — not now; correct path is route AI through Yo-Yo or Tier C
- **BRIEF-vm-hardening-and-consolidation.md created** (commit `b1b51c91`, pwoodfine): 4-section todo — remove 7B model, single-binary all-tier deploy, BRIEF consolidation, artifacts
- **BRIEF consolidation audit complete** (Explore agent, 17 BRIEFs reviewed):
  - 2 conflicts surfaced and resolved (lbug build path, Ring 2/3 coupling) — both approved by operator
  - 4 BRIEFs archived: MASTER-PLAN-2026, olmo-performance-tuning, service-audit, service-slm-architecture
  - 1 BRIEF archived (self-declared): flow-bottleneck-strategic-review
  - flow-restructure §4.1 updated with Ring 2/3 coupling defect note (approved)
  - README audit classification: EXTENDS/ABSORBED/CONFLICT/UNRELATED for all active BRIEFs
- **Outbox messages sent to Command** (commit `2268a770`, jwoodfine):
  - Build session todo (6 steps, operator-approved)
  - lbug 0.16.1 packaging bug — request upstream contact
- **Queue model clarified**: Yo-Yo queue is file-backed on disk (`/srv/foundry/data/apprenticeship/queue/`); briefs accumulate from commit hooks; drain worker dispatches at 02:00 UTC nightly; service-content graph context fetched at dispatch time (non-fatal if down)

### Pending / carry-forward
- **Tonight's build session (Command scope)**: Stage 6 promote → remove 7B model (SLM_FORCE_BROKER_MODE) → service-content code fixes (~150 LOC) → service-slm audit fix (~10 LOC) → build + deploy both binaries → Yo-Yo Packer rebuild. Full list in outbox + BRIEF-vm-hardening-and-consolidation.md
- **service-content Ring 2/3 fix** (~30 LOC, main.rs:198) — write Source node before Doorman call; top priority for next code session
- **Yo-Yo env var fix** — start-yoyo.sh sed silently fails on IP change; all 3 endpoint vars must update reliably
- **project-editorial inbox**: `project-editorial-20260521-e4-triage-naming-blockers` — 3 drafts need rename before re-staging
- **Stage 6**: 2 commits ahead of origin/main post-session (BRIEF consolidation commits); Command must promote

### Operator preferences surfaced
- Routes AI through Yo-Yo (no local model on workspace VM) — confirmed understanding of the tier model
- Wants single binary that adapts to all tiers at runtime — already ratified in BRIEF
- Prefers Opus adversarial agents for systems investigation (VM resource diagnosis pattern)
- Approves proposed wording for BRIEF edits before applying — review one diff at a time

---
