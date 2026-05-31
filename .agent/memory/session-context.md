## Session context — rolling 3-session summary

---

### 2026-05-31 | totebox@project-intelligence | claude-sonnet-4-6 (session 12 — apprenticeship prompt audit; Fix A + Fix B deployed)

**Done this session:**
- **Apprenticeship prompt audit (full call chain):** Identified two critical gaps making the 554-entry training corpus nearly useless. Both fixed in commit `a0649002` (promoted to canonical by Command).
- **Fix A — `actual_diff: ""` bug:** The post-commit hook used `python3 - <<'PYEOF'` + `sys.stdin.read()`. Heredoc consumes stdin (the script source); `sys.stdin.read()` always returns `""`. Fix: pass diff via `HOOK_DIFF` env var, read via `os.environ.get()`. Applied to `service-slm/scripts/git-post-commit-hook.sh` and workspace `bin/capture-edit.py` (workspace commit `48f23c9` by Command). Verified: 3 newest queue entries have `actual_diff` 2–3.5 KB each.
- **Fix B — 100% escalation rate:** `APPRENTICE_SYSTEM_PROMPT` had Claude-specific jargon ("Doctrine claim #32", "Master/Root/Task Claude"). OLMo wrote preamble before `---`; `extract_frontmatter()` regex requires `\A` → parse fail → `escalate: true`. Rewrote to OLMo-compatible plain instructions with explicit "Do not write any introductory text before the opening `---`".
- **New binary deployed** via `sudo cp` + `sudo systemctl restart local-doorman` at 00:41 UTC.
- **Stage 6 complete (by Command):** commits `a0649002` (Fix A+B), `aef13fd9` (outbox), `b57f9d22` (bonus: Doorman endpoint 8011→9080 in app-console-content + app-console-keys). Archive 0 commits ahead of origin/main.
- **BRIEF-slm-learning-loop.md §8 written:** full audit findings + Fix A/B/C doc.
- **service-slm/NEXT.md updated:** Fix C deferred item + OLMo inference speed note.

**Pending / carry-forward:**
- **Fix C (deferred):** Add GBNF grammar to both `dispatch_shadow()` calls (apprenticeship.rs lines 181, 279). Observe 5–10 drain cycles after Fix B first — if OLMo still preambles, implement. Wiring already exists in `LocalTierClient::complete()`.
- **OLMo inference speed:** ~2 tok/s CPU; `max_tokens=2048` → 17–60 min per brief. Consider reducing to 512–768 for CPU-primary mode (separate config decision, not urgent).
- **project-console Sprint 4a:** implement `app-console-slm status` command (outbox `project-intelligence-20260530-console-wiring`).
- **Yo-Yo 1h test** when europe-west4-a L4 capacity returns.
- **drain-apprenticeship.service/timer files** in `/etc/systemd/system/` — disabled but present; low-priority cleanup.
- **stale shim test fields** — `anthropic_shim_test.rs` `tier_a_reason`/`idle_monitor` stale (NEXT.md).

**Operator preferences surfaced:**
- None new this session.

---

### 2026-05-30 | totebox@project-intelligence | claude-sonnet-4-6 (session 11 — drain-apprenticeship.timer conflict found and killed; flow confirmed)

**Done this session:**
- **Root cause of recurring poison identified:** `drain-apprenticeship.timer` (Phase 3.4 legacy shell drainer) was firing every ~15 min and poisoning ALL queue entries. Script (`/srv/foundry/bin/drain-apprenticeship-queue.sh`) expected flat `ApprenticeshipBrief` JSON but queue contains `ShadowQueueEntry` format (`{"brief": {...}, "actual_diff": ""}`). `prompt` field always empty → poison. Script also bypassed Doorman (called port 8080 directly) — architectural conflict with Rust drain worker.
- **Timer stopped and disabled:** `sudo systemctl stop drain-apprenticeship.timer && sudo systemctl disable drain-apprenticeship.timer`. Definitions left in /etc/systemd/system/ for reference. Rust drain worker in local-doorman.service is the sole drainer.
- **25 briefs recovered:** queue-poison/ → queue/. Queue: 25 pending, 0 poison, 1 active in-flight (0BDB1DF0 dispatched at 22:17:58 UTC by drain-1821781).
- **Flow confirmed:** llama-server busy with 0BDB1DF0 (curl to :8080 timed out = inference in progress). Done count was 550; will reach 551 when OLMo completes 0BDB1DF0 (17-60 min range from dispatch).
- **NEXT.md updated:** drain-apprenticeship.timer root cause documented; system status table updated.

**Pending / carry-forward:**
- **Monitor flow:** 0BDB1DF0 in-flight since 22:17:58 UTC; completion expected 22:35–23:17 UTC. After done count reaches 551, worker picks up next of 25 queued.
- **Orphaned 0B050EFD lease:** From dead PID 1771363; reaper reclaims at 22:17:56 + 2100s = 22:52:56 UTC.
- **drain-apprenticeship.service/timer files:** Still in /etc/systemd/system/. Consider removing or archiving in a cleanup session — they are disabled but misleading.
- **Stage 6** — 9 commits ahead of origin/main (outbox `project-intelligence-20260530-stage6-sprint3d`)
- **Operator installs** (see outbox `project-intelligence-20260530-stage6-orchestration-deploy`):
  1. Build + deploy `orchestration-slm-server` binary
  2. Install `/etc/foundry/local-orchestration-slm.env`
  3. `sudo systemctl enable --now local-orchestration-slm.service`
  4. Add `SLM_ORCHESTRATION_ENDPOINT=http://127.0.0.1:9180` to local-doorman.env + restart
  5. Enable daily/weekly smoke timers
- **project-console actions** (see outbox `project-intelligence-20260530-console-wiring`):
  - Port fix: `app-console-content/src/draft.rs` 8011 → 9080
  - Sprint 4a: implement `app-console-slm status` command
- **Yo-Yo 1h test** when L4 capacity returns
- **stale test fields** — `anthropic_shim_test.rs` `tier_a_reason`/`idle_monitor` fields stale

**Operator preferences surfaced:**
- Flow investigation requires full root cause before "done" — recurring poison needed tracing to drain-apprenticeship.timer, not just recovery

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


