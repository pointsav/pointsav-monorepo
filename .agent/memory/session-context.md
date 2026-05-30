## Session context — rolling 3-session summary

---

### 2026-05-30 | totebox@project-intelligence | claude-sonnet-4-6 (session 10 — lease expiry fix + flow confirmed)

**Done this session:**
- **Root cause diagnosed for 26 poison entries:** Previous session (ran out of context) manually moved 25 briefs to queue-poison/ at 21:23 during investigation; one more (177F2B11) at 21:38. The drain worker itself did not cause the poison.
- **Root cause diagnosed for drain worker silence:** Reaper's 300s lease expiry < 1860s drain wrapper → reaper reclaimed lease at 300s, drain worker's `release_shadow()` found stale lease and silently returned `Ok()`. After brief was re-queued by reaper and drain woke after 1800s timeout, it worked correctly — but during the previous session's investigation, the briefs had been manually quarantined to poison.
- **Lease expiry fix:** Added `SLM_QUEUE_LEASE_EXPIRY_SEC=2100` to `/etc/local-doorman/local-doorman.env`. 2100s > 1860s drain wrapper; reaper now cannot reclaim a live lease. Restart at 22:17:58 UTC. Confirmed `lease_expiry_secs=2100` in startup log.
- **26 poison entries recovered:** All 26 moved from `queue-poison/` to `queue/`. 0 poison after recovery.
- **Flow confirmed:** New drain worker (PID 1821781) immediately dispatched `0BDB1DF0` at 22:17:58 (tier="local"). llama-server busy with inference. Queue: 24 pending, 1 active in-flight, 0 poison.
- **NEXT.md updated:** System status, poison recovery note, lease fix documented.
- **BRIEF-slm-substrate-master.md updated:** §1 live state table updated with lease fix.

**Pending / carry-forward:**
- **Stage 6** — 9 commits ahead of origin/main (outbox `project-intelligence-20260530-stage6-sprint3d`); `SLM_QUEUE_LEASE_EXPIRY_SEC=2100` is an env-only change (not git-tracked), no new commit needed
- **Operator installs** (see outbox `project-intelligence-20260530-stage6-orchestration-deploy`):
  1. Build + deploy `orchestration-slm-server` binary
  2. Install `/etc/foundry/local-orchestration-slm.env`
  3. `sudo systemctl enable --now local-orchestration-slm.service`
  4. Add `SLM_ORCHESTRATION_ENDPOINT=http://127.0.0.1:9180` etc. to local-doorman.env + restart
  5. Enable daily/weekly smoke timers
- **Monitor drain:** 24 pending briefs + 1 orphaned in-flight (0B050EFD from dead PID 1771363; reaper reclaims at 22:52). Each brief takes 17–60 min at OLMo CPU speed.
- **project-console actions** (see outbox `project-intelligence-20260530-console-wiring`):
  - Port fix: `app-console-content/src/draft.rs` 8011 → 9080
  - Sprint 4a: implement `app-console-slm status` command
- **Yo-Yo 1h test** when L4 capacity returns
- **stale test fields** — `anthropic_shim_test.rs` `tier_a_reason`/`idle_monitor` fields stale

**Operator preferences surfaced:**
- Expects full flow investigation before reporting "done" — the 26 poison entries needed root cause analysis and recovery, not just status report

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


