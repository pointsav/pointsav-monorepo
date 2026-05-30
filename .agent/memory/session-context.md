## Session context — rolling 3-session summary

---

### 2026-05-30 | totebox@project-intelligence | claude-sonnet-4-6 (session 9 end — Sprint 3D + poison recovery)

**Done this session (continued from session 9 start):**
- **Sprint 3D — Tier A timeout fix (commits `5166f43b`, `e452abdb`, `526b3735`, `1398522b`):**
  - Root problem: `reqwest::Client::new()` in `local.rs` had no timeout → drain worker blocked indefinitely
  - First fix: added 120s client timeout (`5166f43b`) and 150s drain wrapper (`e452abdb`) — turned out 120s is too short for OLMo 7B (2048 tokens at ~2 tok/s = 1024s theoretical max; observed 17–60 min)
  - Second fix: `SLM_TIER_A_FIRST=true` bypass for drain hold (`526b3735`) — drain hold was holding the queue waiting for Tier B recovery even though Tier A is the primary
  - **Final fix (`1398522b`):** raised client timeout 120s→1800s; drain wrapper 150s→1860s
- **Binary deployed 2026-05-30T21:14:54Z** — sha256=`bd91eafc7c2a232c10e0c449f31474d9d994568df9c4054eb8f591f93ce3360d`; binary ledger updated
- **21 poison briefs recovered** — moved from `queue-poison/` back to `queue/`. Root cause: old binary called `dequeue()` on `ShadowQueueEntry`-format files (JSON has `{"brief":..., "actual_diff":...}` wrapper); `dequeue()` expects bare `ApprenticeshipBrief` → parse failed → poison. New binary uses `dequeue_shadow()` correctly.
- **Drain worker confirmed live**: 23 briefs queued, 1 in-flight (brief 4BA59EC8 dispatched at 21:14Z), 550 done, 0 poison

**Pending / carry-forward:**
- **Stage 6** — now **9 commits ahead** of origin/main; see outbox `project-intelligence-20260530-stage6-sprint3d`
- **Confirm drain completes** — brief 4BA59EC8 should complete (done: 551) within 30 min; if it hits reaper (300s lease) check for "dispatch timed out after 1860s" log
- **Operator installs** — orchestration-slm-server binary + service still pending from session 9 start (see outbox `project-intelligence-20260530-stage6-orchestration-deploy`)
- **stale test fields** — `anthropic_shim_test.rs` likely has stale `AppState` fields (`tier_a_reason`, `idle_monitor`) — add NEXT.md item; low priority
- **Yo-Yo 1h test** — `start-yoyo.sh --wait-ready=120 --runtime=1h` when L4 capacity available

**Operator preferences surfaced:**
- Timeouts must cover actual hardware reality (CPU inference rate, not GPU assumption)
- Want Tier A flow to be reliable 24h/day without manual intervention

---

### 2026-05-30 | totebox@project-intelligence | claude-sonnet-4-6 (session 9 — orchestration-slm + daily smoke)

**Done this session:**
- **Planned** console → orchestration chain (project-console port fix + app-console-slm Sprint 4a spec sent via outbox)
- **Quarantined** 26 remaining poison briefs → `/srv/foundry/data/apprenticeship/quarantine/` (668 total)
- **Confirmed** git post-commit hook already installed and matching source
- **Commit `d445b5ea` (Jennifer):** 6 new infrastructure files:
  - `infrastructure/systemd/local-orchestration-slm.service` — Yo-Yo broker chassis at port 9180
  - `infrastructure/env/local-orchestration-slm.env.template` — operator env template with comments
  - `infrastructure/systemd/foundry-daily-smoke.{service,timer}` — daily Tier A smoke at 02:00
  - `infrastructure/systemd/foundry-weekly-tier-b-smoke.{service,timer}` — Saturday 03:00 Yo-Yo 1h test
- **Commit `82f01343` (Peter):** `start-yoyo.sh` `--runtime=Nh/Nm/Ns` auto-stop flag

**Pending / carry-forward:**
- **Stage 6** — 4 commits ahead of origin/main (shutdown commit `4023b9bf` + session 9 commits); Command scope
- **Operator installs** (see outbox `project-intelligence-20260530-stage6-orchestration-deploy`):
  1. Build + deploy `orchestration-slm-server` binary
  2. Install `/etc/foundry/local-orchestration-slm.env` with ORCHESTRATION_YOYO_BEARER set
  3. `sudo systemctl enable --now local-orchestration-slm.service`
  4. Add `SLM_ORCHESTRATION_ENDPOINT=http://127.0.0.1:9180` etc. to local-doorman.env + restart
  5. Enable daily/weekly smoke timers
- **project-console actions** (see outbox `project-intelligence-20260530-console-wiring`):
  - Port fix: `app-console-content/src/draft.rs` 8011 → 9080
  - Sprint 4a: implement `app-console-slm status` command (spec in outbox)
- **Yo-Yo 1h test** — try `start-yoyo.sh --wait-ready=120 --runtime=1h` when L4 capacity available
- **orchestration-slm Yo-Yo endpoint** — set `ORCHESTRATION_YOYO_*_ENDPOINT` in chassis env once Yo-Yo starts

**Operator preferences surfaced:**
- Want daily hardening testing to keep everything flowing 24h/day
- Yo-Yo must remain manually started (confirmed session 7)

---

### 2026-05-30 | totebox@project-intelligence | claude-sonnet-4-6 (session 8 — circuit resilience complete)

**Done this session:**
- **All five circuit-resilience sprints deployed** (commits `96dcaf2b`→`b08cec3d`):
  - Sprint 3A: `SLM_TIER_A_FIRST=true` threaded through `DoormanConfig`, `ApprenticeshipConfig`, `select_tier()`, `pick_tier_for_brief()`. Startup guard prevents mutual use with `SLM_FORCE_BROKER_MODE`. `route_yoyo_only` (ADR-07) unchanged.
  - Sprint 3B: WATCHER Tier A fallback in `service-content/src/main.rs`. Rate-limited at 300s. `TierAFallbackConfig` + `last_tier_a_attempt`. Calls `/v1/chat/completions` with 5-category system prompt + json-schema grammar; confidence 0.75; upserts entities to LadybugDB.
  - Sprint 3C: Drain worker pause in `slm-doorman-server/src/main.rs`. Before `dequeue_shadow()`, checks `tier_b_status()` — if ALL nodes circuit=open AND `opened_for_secs >= SLM_HOLD_THRESHOLD_SECS` (3600s default), skips cycle and logs.
- **Both binaries rebuilt and deployed** (2026-05-29T19:26Z):
  - `slm-doorman-server` sha256=`81b8629c`; running with `SLM_TIER_A_FIRST=true`, `SLM_HOLD_THRESHOLD_SECS=3600`
  - `service-content` sha256=`2362ea5c`; running with `SERVICE_CONTENT_TIER_A_FALLBACK_ENABLED=true`, 300s interval; entity_count=7,201 live
- **Verification**: `/readyz` → `tier_b` field with per-node circuit state ✓; `/healthz` → `entity_count: 7201` ✓; startup log `SLM_TIER_A_FIRST=true: Tier A is the confident primary` ✓; shadow dispatch `tier="local"` ✓
- **Binary ledger updated** at `/srv/foundry/data/binary-ledger/` (workspace level)
- **BRIEF-slm-substrate-master.md** updated: all 8 sprint checkboxes ✓; live state table updated
- **Outbox to Command**: Stage 6 promotion requested for 9 commits; quarantine 590 poison briefs; binary ledger confirmation

**Pending / carry-forward:**
- **Stage 6 promotion** — 9 commits ahead of origin/main; Command Session required
- **Quarantine 590 poison briefs** — `mv /srv/foundry/data/apprenticeship/queue-poison/* /srv/foundry/data/apprenticeship/quarantine/`
- **Install claude-bridge service** (Command if not done): `sudo cp infrastructure/systemd/local-claude-bridge.service /etc/systemd/system/ && sudo systemctl enable --now local-claude-bridge.service`
- **Install git post-commit hook** — per archive: `cp service-slm/scripts/git-post-commit-hook.sh .git/hooks/post-commit && chmod +x`
- **Verify CORPUS extraction via Tier A fallback** — drop a CORPUS file, confirm `[WATCHER-TIER-A]` log within 300s
- **`.agent/` contamination** — Command Stage-6 rebase pulled project-knowledge/.agent/ content into project-intelligence (outbox.md + NEXT.md at root show project-editorial/knowledge content). Report to Command for cleanup.

**Operator preferences surfaced:**
- No new preferences this session

---


