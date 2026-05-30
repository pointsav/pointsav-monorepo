## Session context — rolling 3-session summary

---

### 2026-05-30 | totebox@project-console | claude-sonnet-4-6

**Done this session:**
- **Doorman port fix** (8011→9080, authoritative per `local-doorman.service SLM_BIND_ADDR`):
  - `app-console-content/src/cartridge.rs` `ContentCartridge::new()` default SLM endpoint
  - `app-console-keys/src/config.rs` `default_slm_endpoint()` fn
- **BRIEF-leapfrog-2030-coding.md**: Phase 5 marked COMPLETE 2026-05-24 (commits 6422c2a8 + 5118ce77); port fixed; Phase B/C/D/E roadmap entries added; updated date
- **BRIEF-os-console-platform.md**: §10 Doorman correctness table fixed (8011=wrong, 9080=correct); §9 config examples updated; §6 platform table expanded for macOS 10.13+ Intel + universal; §1 updated
- **BRIEF-cross-platform-release.md**: NEW — Phase B spec; macOS 10.13 compat model; build matrix; dependency audit; TerminalCaps design; Phase B checklist
- **BRIEF-tui-pivot-2030.md**: archived notice corrected (9080 is correct)
- `session-start.md`, `NEXT.md` (blocker resolved), drafts-outbound (3 files): all 8011 refs corrected to 9080
- **Outbox**: new message to Command Session (project-console-20260530-phase-a-complete)
- **Committed** `009b2e04` as Peter Woodfine (12 files, 313 insertions / 35 deletions)
- **Stage 6 force-push COMPLETE**: origin-staging-j + origin-staging-p both at `009b2e04` (forced deadd4cf→009b2e04)

**Pending / carry-forward:**
- **Operator decision needed**: release trigger for `.github/workflows/release.yml` (Phase B4). Recommend `v*.*.*` tag push. Noted in BRIEF-cross-platform-release.md + outbox.
- **Phase B** (cross-platform release): rust-toolchain.toml, .cargo/config.toml, reqwest audit, GH Actions matrix, TerminalCaps probe
- **Phase C** (email F3): app-console-email lib crate + EmailCartridge (service-email backend in project-data)
- **Phase D** (SLM F9): app-console-slm lib crate + SlmCartridge (Doorman at 9080)
- **Phase E** (orchestration wiring): mba_client.rs audit; os-orchestration confirmed as command hub
- **Canonical promote**: Command Session runs `bin/promote.sh` after reviewing outbox msg
- Binary rebuild + `systemctl restart` for any consuming services after canonical promote

**Operator preferences surfaced:**
- Continues previous sessions: "STARTUP/SHUTDOWN" = full checklist; auto mode = work without stopping for confirmation

---

### 2026-05-28 | totebox@project-intelligence | claude-sonnet-4-6 (session 2)

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

