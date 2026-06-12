# Session Context — project-intelligence

> Format spec: `~/Foundry/conventions/session-context-format.md`.
> Keep only 5 most recent entries. Push oldest to `session-context-archive.md`.

---

## Operator preference digest

- Operator routinely authorises direct dispatch without Master ratification queue — confirm in-session
- Velocity > perfection for bulk SLM corpus work; ship and iterate
- Phase 6 training arm: always set SLM_DRAIN_PAUSED=false + arm corpus-threshold.py cron before 17:00 UTC on training day
- Never stop yoyo-batch VM manually mid-training — let corpus-threshold.py handle lifecycle
- LoRA checkpoint naming: `dpo-ckpt-<YYYYMMDD>` under GCS `gs://pointsav-slm-corpus/checkpoints/`

---

## Cross-archive carry-forward

- [x] ~~LoRA pip install on yoyo-batch VM~~ — DONE session 3 (2026-06-09); `~/training-venv` with trl 1.5.1 installed
- [ ] **Stage 6 remaining archives** — 24 archives have doc-only CLAUDE.md changes from Phase C; promote in batch from Command Session
- [ ] **Option B migration** — wire mailbox tools through app-orchestration-command after Phase 3 (arch decision pending)
- [x] ~~service-content DataGraph enrichment~~ — binary b159c9 deployed 2026-06-10; 0-entities fix live
- [ ] **yoyo-batch VM STOCKOUT** — us-central1-a L4 exhausted; **BLOCKER: no DPO pairs + no training without Tier B**; no zone fallback; operator/Command action required
- [ ] **Phase 4b ledger bug** — 400 commits marked swept on 202-queue ACK, not enrichment success; all 400 permanently skipped; fix `yoyo-daily-cycle.sh:171` BEFORE next cycle or the fix is moot
- [ ] **SLM_YOYO_WEIGHTS_GCS_BUCKET unset** — 20 training markers pile up with no consumer; set in `infrastructure/local-yoyo-daily.service` or document as manual-dispatch pre-D4
- [ ] **8080 drain-worker fault + 737-entry quarantine** — `/metrics` + `/v1/chat/completions` fail; blocks apprenticeship shadow-capture; quarantine purge outside Totebox scope → surface to Command
- [ ] **Stage 6 pending — workspace commits** — `c89e78e`, `9341778`, `78ce725` (yoyo Phase 4b + set-e fix + training rsync fix); promote from Command Session
- [ ] **local-slm OOM incident review** — M-17 relay from project-system (2026-06-11): two OOM kills Jun-04; fixes applied (zram disabled, cache→2048); action: review service-slm/router/src/*.rs for 4096-token cache assumptions + verify benchmarks
- [ ] **local-content.service Requires→Wants** — Command scope; in outbox; Doorman restart silently kills content service

---

## Session entries

### 2026-06-12 — Session 8: first full yoyo-batch cycle; Opus audit; Phase 4b + training fixes

**Role:** totebox | **Engine:** claude-code (sonnet-4-6 main loop, opus workflow agents for audit)

**Done this session:**
- Diagnosed Phase 4b not triggering in prior run (old PID loaded pre-commit script; next PID fixed)
- Deleted false training receipt `coding-lora-2026-06-12.ran` (written despite SSH failure)
- **Phase 6 training fixes** (Fable agent, commit `78ce725`): rsync `run-dpo-training.py` + corpus to yoyo-batch before SSH; receipt only written on `training_rc=0`
- **BRIEF §13 and §14 updated** with session-8 as-built + full Opus audit entry (commit `6769b3b0` + this session)
- **First complete yoyo-batch cycle ran** (00:52 stint, PID 1137652): CLEAN end-to-end; +22 entities, 0 DPO pairs, 0 training, $0.804
- **Opus deep audit (8-agent workflow)** of the full cycle; falsified two BRIEF premises:
  - 418 valid DPO pairs DO exist at `feedback/apprenticeship-git-commit-*.jsonl` (NOT `enrichment-*.jsonl` — that prefix doesn't exist)
  - Markers ARE being generated (20 in `training-pending/`, daily since 06-08)
- Root cause verified: Tier B Terminated entire window; Doorman `consecutive_failures=102`; Tier-A-only sweep cannot produce DPO pairs by design
- Two code faults found: Phase 4b ledger records 202-queue ACK not enrichment success (400 commits poisoned); 60s wait-for-pairs meaningless for async cascade
- `SLM_YOYO_WEIGHTS_GCS_BUCKET` verified unset everywhere — 20 markers have no consumer

**Carry-forward:**
- BLOCKER: restore Tier B GPU endpoint (operator/Command)
- Fix Phase 4b ledger bug BEFORE next cycle (`yoyo-daily-cycle.sh:171`)
- Set `SLM_YOYO_WEIGHTS_GCS_BUCKET` or document manual dispatch
- Reserve training-budget floor when adapters reach READY
- Fix stall early-exit to fire at 6/6 (currently 12/6); guard `entities=?` read failure
- Fix 8080 drain-worker + audit 737-entry quarantine (Command scope)
- Expand `SWEEP_REPOS` to glob all 23 `clones/*/.git`

---

### 2026-06-11 — Opus audit + preemption-resilient yoyo rewrite

**Role:** totebox | **Engine:** claude-code

**Done this session:**
- Opus agent deep audit of all yoyo scripts on disk (not from summaries) — found 5 root causes
- Key finding: `yoyo-idle-monitor.timer` ACTIVE (every 5min), racing Phase 4 enrichment; disabled and archived
- Key finding: `start-yoyo.sh` never installed in bin/ — prior "two worlds" framing was wrong
- Key finding: STOCKOUT has no retry — confirmed Jun 11 root cause (0 work all day)
- Rewrote `bin/yoyo-daily-cycle.sh` (commit `53f8765`): day-budget ledger + `run_stint()` + `main()` outer loop + `start_vm_with_retry()` (22h STOCKOUT retry) + Phase 4 stall/preemption detector
- Added `AUTONOMOUS_ENABLED` training gate (replaces per-day tag that was never auto-created)
- Updated `infrastructure/local-yoyo-daily.service`: `YOYO_DAY_BUDGET_MIN=120` + `YOYO_RETRY_DEADLINE_HOURS=22`
- BRIEF updated with session-7 As-Built and Opus audit findings (commit `1cce73ee`)
- Tonight's timer (02:30 UTC Jun 12) will use new rewritten script for first real test

**Carry-forward:**
- STOCKOUT still active — us-central1-a L4; timer fires 02:30 UTC Jun 12; new script will retry all day if needed
- Training gate: AUTONOMOUS_ENABLED not yet created; create when first genuine training run desired
- AUTONOMOUS_ENABLED arm: `echo 'operator-authorized' > /srv/foundry/data/training-approved/AUTONOMOUS_ENABLED`
- Corpus transport gap (Phase 6): SSH passes workspace path; rsync needed before training can read pairs
- Adapter-to-serving link still missing (PEFT→GGUF + llama-server hot-swap)

---

### 2026-06-10 — Inbox clear + BRIEF retrieval + VM start attempts

**Role:** totebox | **Engine:** claude-code

**Done this session:**
- Inbox cleared: ACK'd BRIEF pickup notification (both BRIEFs already present); ACK'd service-content blocker resolved; ACK'd contamination notice
- Sent BLOCKER message to Command re: service-content 0-entities binary rebuild (high priority) — Command actioned same session (binary b159c9 deployed)
- Training pipeline confirmed 100% ready: ML libs installed, Phase 6 wired, approval tag present, service-content fix live
- Retrieved 2 contaminated BRIEFs from project-knowledge archive: BRIEF-project-intelligence-master.md (PRIMARY PLAN OF RECORD, supersedes slm-substrate-master) + BRIEF-project-intelligence-active-work.md; committed
- VM start attempts × 4: yoyo-batch STOCKOUT us-central1-a throughout session; daily timer armed 02:30 UTC

**Problems encountered:**
- `start-yoyo.sh` defaults to instance `yoyo-tier-b-1` / zone `europe-west4-a`; must pass `SLM_YOYO_GCP_INSTANCE=yoyo-batch` explicitly
- us-central1-a L4 capacity exhausted for ~4+ hours despite VM having run earlier same day (09:34–10:31 PDT)

**Pending / carry-forward:**
- yoyo-batch VM STOCKOUT — daily timer retry 02:30 UTC; wait for L4 capacity
- local-slm OOM incident review (M-17 relay, high priority) — review service-slm/router/src/*.rs for 4096-token cache assumptions
- local-content.service Requires→Wants fix (Command scope)

### 2026-06-09 — MCP Sprint 5 (Sessions 1–3) + Stage 6

**Role:** totebox | **Engine:** claude-code

**Done this session:**
- `slm-mcp-server` v0.3.0 promoted to canonical (Stage 6 complete — 33 of 54 local commits landed; 21 `.agent/`-only commits correctly dropped during rebase)
- Sprint 5: `cast_apprenticeship_verdict` + `get_service_status` tools wired and smoke-tested
- Sprint 4: `get_session_brief`, `send_mailbox_message`, `query_mailbox`, `get_doorman_status` — 13 tools total at v0.3.0
- Binary install: `pkill -x slm-mcp-server` required before replacing binary (Text file busy if skipped)
- service-content: `fix(Tier A response parsing)` — Doorman envelope + 180s timeout shipped
- service-content: `fix(EXTRACTION_SYSTEM_PROMPT)` — removed prompt-injection examples; guard empty-rejected DPO pairs
- BRIEF §9c stale claim corrected: graph context injection was NOT broken — live logs confirmed `entity_count=5` working
- BRIEF §13 mutations audit was NOT pending — `http.rs:1215–1234` already implemented it in PS.4 sprint

**Problems encountered (carry-forward institutional memory):**
- `"Text file busy"` error when replacing binary: MCP server holds the binary fd open. Always `pkill -x slm-mcp-server` BEFORE `sudo cp new-binary /usr/local/bin/slm-mcp-server`. Recurs on every deploy.
- BRIEF items can go stale: §9c "graph injection broken" was a 2026-06-05 snapshot, already fixed by deploy time. Pattern: always grep live code/logs before treating a BRIEF claim as open work.
- `.agent/inbox.md` / `.agent/outbox.md` tracked in git (committed before gitignore entry added). During rebase against canonical, all 12 commits touching `.agent/` files caused `modify/delete` conflicts. Resolution: `git rm --cached --ignore-unmatch .agent/...` then skip if no remaining staged content.
- `promote.sh` location guard: run `FOUNDRY_COMMAND_SESSION=1 FOUNDRY_PROMOTE_YES=1 ~/Foundry/bin/promote.sh` when calling from a Totebox clone path.

**Successes (confirmed working):**
- `cast_apprenticeship_verdict` SSH signing flow: `identity/.toggle` → `ssh-keygen -Y sign` → base64 PEM → `POST /v1/verdict` — fully operational
- Graph context injection confirmed working automatically (entity_count=5 in live Doorman logs)
- Post-commit hooks fire `POST /v1/shadow` immediately on `commit-as-next.sh` — confirmed via `journalctl -u local-doorman`
- 13 tools confirmed via direct JSON-RPC smoke test against the binary
- `get_session_brief()` eliminates 3,000–8,000 tokens of manual file reads per Totebox session start

**Pending / carry-forward:**
- LoRA pip install on yoyo-batch VM (operator SSH action)
- Stage 6 batch for remaining 24 doc-only CLAUDE.md archives
- Option B migration (arch decision pending)

