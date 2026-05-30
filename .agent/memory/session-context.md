## Session context — rolling 3-session summary

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

### 2026-05-29 | totebox@project-intelligence | claude-sonnet-4-6 (session 7 — Goose verified)

**Done this session:**
- **§7.2 VERIFIED** (2026-05-29T04:10Z): Goose v1.36.0 round-tripped through Doorman → Tier A → OLMo replied "Hello! The result of 2+2 is 4." Doorman log: `dispatching ... tier="local"`.
- **Root cause found and fixed** (`74ba6da0`, Jennifer): Goose sends `system` as array-of-content-blocks `[{"type":"text","text":"..."}]`; Doorman's `AnthropicMessagesBody.system: Option<String>` rejected this with 422. Fixed with `AnthropicSystem` untagged enum that handles both `Text(String)` and `Blocks(Vec<AnthropicContentBlock>)`. 51/51 http_test pass.
- **Goose config written**: `/home/mathew/.config/goose/config.yaml` with `GOOSE_PROVIDER: anthropic`, `ANTHROPIC_BASE_URL: http://127.0.0.1:9080`, `GOOSE_MODEL: claude-haiku-4-5-20251001`.
- **§7.3 live SSE test**: Goose `Read /etc/hostname` tool invocation — OLMo 7B returned `stop_reason: end_turn` with text response (not a tool_use block). Model capability limit confirmed, not a shim bug.
- **Yo-Yo confirmed TERMINATED**: GCP europe-west4-a `yoyo-tier-b-1` is TERMINATED. No file accumulation in Doorman or llama-server queues. No auto-start mechanism exists anywhere (no cron, no timer that starts; `yoyo-idle-monitor.timer` is stop-only).
- **Operator decision**: Yo-Yo must remain manually started. No auto-start until further testing complete.
- **2 GUIDEs staged** to `.agent/drafts-outbound/`: `GUIDE-guide-goose-local-doorman.draft.md` + `GUIDE-guide-post-commit-training-hook.draft.md` (hook payload fixed with Python ShadowWire struct). Commits `4055ad96` + `5fcbd4a3`.
- **BRIEF-slm-learning-loop.md** §7 updated: §7.2 ✅ VERIFIED, §7.3 PARTIAL, blockers summary updated.

**Pending / carry-forward:**
- **§7.3 tool_use**: OLMo 7B does not invoke tools. Requires Tier B (Yo-Yo OLMo 3 32B-Think) or Tier A upgrade to tool-use-tuned model.
- **§7.4 entity extraction + §7.6 training**: Requires manual Yo-Yo start (`service-slm/scripts/start-yoyo.sh --runtime=2h`) after operator testing complete.
- **QEMU vm-mediakit** (PID 4039898): still at ~150% CPU from project-infrastructure; inference was slow (~0.03 tok/s) during test; stopped `local-content` service temporarily to free the llama-server slot. `local-content` restarted after test.
- **Stage 6 promote**: archive is 15+ commits ahead of origin/main (Command Session scope; prereq rebase per `command-20260520-stage6-rebase-required`).
- **Binary ledger**: `data/binary-ledger/slm-doorman-server.jsonl` needs fresh sha256 after `74ba6da0` rebuild (Command Session scope).

**Operator preferences surfaced:**
- Yo-Yo: do NOT auto-start; requires explicit operator invocation after further testing

---

