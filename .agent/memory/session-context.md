# Session Context — project-intelligence archive

Rolling 3-session summary. Newest on top. Keep only 3 entries; push oldest to `session-context-archive.md`.

⚠️ Prior entries in this file were project-knowledge content (cross-cluster contamination
from Command Stage-6 rebase 2026-05-22 — flagged for Command; see outbox).

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

## Session: 2026-05-22 session 3 | Role: totebox | Engine: claude-code

### Done this session
- **Phase 4 (service-slm Doorman node-class gating)**:
  - `foundry-nodeclass/src/lib.rs`: added `NodeClass::as_str()` method
  - `slm-doorman-server/Cargo.toml`: added `foundry-nodeclass = { workspace = true }`
  - `slm-doorman-server/src/main.rs`: reworked `build_doorman()` → `DoormanBoot`; detects node class via `foundry_nodeclass::detect()`; gates `local` client on `caps.supports_on_node_ai() && !SLM_FORCE_BROKER_MODE`; returns `node_class: &'static str` + `tier_a_reason: &'static str`
  - `slm-doorman-server/src/http.rs`: added `node_class: &'static str` + `tier_a_reason: &'static str` to `AppState`; `readyz` now emits `node_class`, `tier_a`, `tier_a_reason`, `ai_available`; ~19 AppState construction sites updated in lib.rs + test files
  - `slm-doorman/src/router.rs`: added `micro_class_no_local_tier_unavailable` invariant test
  - `infrastructure/local-doorman/local-doorman.service`: `Requires=local-slm.service` → `Wants=` (soft dep); workspace git, Command must commit
  - `BRIEF-flow-restructure.md`: Phases 1–4 marked done, Phase 5 resume point set
  - `NEXT.md`: Phase 4 complete note + Stage 6 promote reminder
  - Outbox: added message to Command re: infrastructure change needing workspace commit
- `cargo check --workspace` clean; `cargo test --workspace` running (in progress at session end)

### Pending / carry-forward
- **`cargo test --workspace`** running in background — verify all 241+ tests green before committing
- **Commit Phase 4** via `commit-as-next.sh` — 11 files modified in project-intelligence archive
- **Phase 5**: `TOTEBOX_NODE_CLASS=micro` integration tests + cgroup sandbox (`tests/micro_node.rs`)
- **Workspace commit** (Command scope): `infrastructure/local-doorman/local-doorman.service` needs staging + commit from `/srv/foundry/`
- **Stage 6 promote**: 10+ commits ahead of origin/main; needs `git rebase origin/main` first (Command scope)
- `service-content/CLAUDE.md` doesn't exist — low priority
- Inbox `project-editorial-20260521-e4-triage-naming-blockers` — 3 drafts need rename before re-staging

### Operator preferences surfaced
- No new preferences surfaced this session

