## Session context — rolling 3-session summary

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

### 2026-05-29 | totebox@project-intelligence | claude-sonnet-4-6 (session 6 — continuation)

**Done this session:**
- `docs(brief)`: BRIEF-slm-learning-loop.md §7.2-3 updated with live SSE test result. Commits `df1a5e64` (Peter) + `3fd2dfef` (Jennifer).
- **§7.3 live SSE test completed**: sent `/v1/messages` with `Read` tool to OLMo 7B; got `stop_reason: end_turn` with text response. OLMo 7B is not fine-tuned for tool invocation — describes how to use `cat` instead of invoking the tool. Shim code is correct (no llama-server format errors); model capability is the limit.
- Confirmed Goose v1.36.0 installed at `/usr/local/bin/goose`.
- QEMU vm-mediakit PID updated in NEXT.md: 3949093 → 4039898 (project-infrastructure restarted it).

**Pending / carry-forward:**
- **QEMU vm-mediakit** (PID 4039898, -accel tcg software emulation): system load 17+. Confirm with project-infrastructure owner; `kill 4039898` to unblock inference and enable §7.2 Goose test.
- **§7.2 Goose chat round-trip**: `ANTHROPIC_HOST=http://127.0.0.1:9080 ANTHROPIC_API_KEY=foundry-local GOOSE_MODEL=claude-haiku-4-5-20251001 goose run --text "Say hello"`. Blocked by CPU saturation.
- **§7.3 tool_use in Doorman log**: OLMo 7B does not invoke tools. Options: (a) wait for Yo-Yo VM with OLMo 3 32B-Think, (b) upgrade Tier A to a tool-use-tuned model (e.g. Qwen2.5-7B-Instruct), (c) mark §7.3 as "not achievable with current Tier A model".
- **§7.4 entity extraction**: Yo-Yo VM must be started to close Tier B circuit. Command: `service-slm/scripts/start-yoyo.sh --runtime=2h`.
- **Stage 6 promote**: archive is 32+ commits ahead of origin/main (Command Session scope; prereq rebase per `command-20260520-stage6-rebase-required`).
- **Binary ledger**: `data/binary-ledger/slm-doorman-server.jsonl` in workspace (Command Session scope). SHA256: `9e8542b6...` (slm-doorman-server rebuilt 2026-05-29T02:14Z).

**Operator preferences surfaced:**
- Working autonomously on verification; blocked items documented with clear next-action commands

---

### 2026-05-29 | totebox@project-intelligence | claude-sonnet-4-6 (session 3+4)

**Done this session:**
- Sprint -1 (BRIEF consolidation): 27 contamination BRIEFs archived to `.agent/briefs/archive/`; README rewritten; BRIEF-slm-substrate-master.md corrected (OLMo model name, FORCE_BROKER_MODE rationale); BRIEF-slm-learning-loop.md created. Commit `c5cd4441` (Jennifer)
- Multi-agent research: 5 Opus 4.7 agents for TOPICs/GUIDEs sweep + leapfrog 2030 gap analysis; plan file rewritten
- Sprint 1 (tool_use shim, ~210 LOC): `ComputeRequest.tools` + `ComputeResponse.tool_calls` added to slm-core; local.rs + yoyo.rs propagate tools through to backends and capture tool_calls; anthropic_sse_body emits tool_use SSE blocks; POST /v1/messages/count_tokens + GET /v1/models added. Commit `1b47d3eb` (Jennifer). 51/51 http_test + 102/102 slm-doorman pass.
- Sprint 2 (training pipeline wiring): `git-post-commit-hook.sh` + `claude-session-bridge.py` written. Commit `1d819d7c` (Jennifer)
- Sprint 4 (TOPIC/GUIDE dispatch): 5 TOPICs + 2 GUIDEs from `service-slm/docs/` staged to `.agent/drafts-outbound/`. Commit `d39aea32` (Peter)
- Sprint 0 CONFIRMED COMPLETE by Command: readyz `{"ready":true,"has_local":true,"has_yoyo":true,"has_external":false}` — Tier A + Tier B both live
- `infrastructure/systemd/local-claude-bridge.service` written — completes Sprint 2b wiring (bridge script → CORPUS_WATCH_DIR = jennifer ledgers dir). Needs Command to `sudo cp` + `systemctl enable --now`.

**Pending / carry-forward:**
- **Install claude-bridge service** (Command): `sudo cp infrastructure/systemd/local-claude-bridge.service /etc/systemd/system/ && sudo systemctl daemon-reload && sudo systemctl enable --now local-claude-bridge.service`
- **Install git post-commit hook** — per archive: `cp service-slm/scripts/git-post-commit-hook.sh .git/hooks/post-commit && chmod +x`
- **Yo-Yo nightly cron** — confirm or add: `0 2 * * * .../start-yoyo.sh --runtime=1h` (Tier B currently live; may already be set)
- **Drain 491 poison apprenticeship briefs** from `data/apprenticeship/queue/`
- **Stage 6 promote** — archive is 25+ commits ahead; prerequisite rebase per `command-20260520-stage6-rebase-required`
- **Binary ledger** — update `data/binary-ledger/slm-doorman-server.jsonl` after rebuild
- **Goose install + verification** (Sprint 3, operator): `ANTHROPIC_HOST=http://127.0.0.1:9080 ANTHROPIC_API_KEY=foundry-local GOOSE_MODEL=claude-haiku-4-5-20251001 goose session`

**Operator preferences surfaced:**
- Resumed from context summary; no new preferences this session

---

### 2026-05-28 | totebox@project-intelligence | claude-sonnet-4-6 (session 2)

**Done this session:**
- Multi-agent analysis of CORPUS extraction failures: root causes identified (throughput gap ~5-7x due to missing Flash Attention; slot starvation; grammar silently disabled with thinking; reqwest decode-error misclassification)
- Multi-agent code audit: SC-2/3/5/3d/3e/3f audit items in service-content; SLM-1..SLM-6 in service-slm
- Plan written, approved, executed. 3 commits:
  - `446df43f` (Peter): Tier 2 — deepseek reasoning_content field; reqwest decode→TierBTimeout; Doorman restart after IP update; Packer template adds -fa/deepseek/budget flags
  - `e263d6f0` (Jennifer): Tier 3 — service-content SC-3 health-check, SC-5 logging, SC-2 defer differentiation, SC-3d retry loop, SC-3e write order, SC-3f buffer pool
  - `08896158` (Peter): ops — NEXT.md + BRIEF updated; Stage 6 count updated to 16+
- 111/111 lib tests pass; service-content cargo check clean

**Pending / carry-forward:**
- **Rebuild binaries** — slm-doorman-server and service-content need `cargo build --release` + `systemctl restart` to pick up this session's fixes (commits 446df43f + e263d6f0)
- **Verify CORPUS extraction** after next Yo-Yo start + binary rebuild: `sudo journalctl -u local-content -f | grep -E 'entities extracted|WATCHER|deferred|RETRY'`
- **Packer rebuild** — adds `-fa`, `--reasoning-format deepseek`, `--reasoning-budget 1024` to llama-server.service on the next Yo-Yo image
- Stage 6 promote: archive is 16+ commits ahead of origin/main (Command Session scope); prerequisite rebase per inbox `command-20260520-stage6-rebase-required`
- Binary ledger: `data/binary-ledger/slm-doorman-server.jsonl` + `service-content.jsonl` need fresh sha256 entries after rebuild
- Yo-Yo VM TERMINATED — start with `service-slm/scripts/start-yoyo.sh --runtime=2h` when L4 capacity in europe-west4-a is available

**Operator preferences surfaced:**
- "STARTUP" / "SHUTDOWN" = execute full checklist
- Plan mode with AskUserQuestion for operator decisions before coding
- "All 6 SC-* fixes in a single commit" — batch SC-* fixes together per audit cohort


