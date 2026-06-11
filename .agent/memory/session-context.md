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
- [ ] **yoyo-batch VM STOCKOUT** — us-central1-a L4 exhausted; daily timer retry 02:30 UTC; no zone fallback
- [ ] **local-slm OOM incident review** — M-17 relay from project-system (2026-06-11): two OOM kills Jun-04; fixes applied (zram disabled, cache→2048); action: review service-slm/router/src/*.rs for 4096-token cache assumptions + verify benchmarks
- [ ] **local-content.service Requires→Wants** — Command scope; in outbox; Doorman restart silently kills content service

---

## Session entries

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

