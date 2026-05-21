# Session Context — Rolling 3-Session Summary

---

## 2026-05-21 | Totebox | claude-code

**Done this session:**
- Fixed `is_busy()` always-true bug — llama-server v1 omits `slots_idle` from `/health`; changed to `Option<u32>` + new test. Committed `6a80c5e3`. 125 slm-doorman tests pass.
- Built and deployed `slm-doorman-server` + `slm-mcp-server` release binaries (from `CARGO_TARGET_DIR=/srv/foundry/cargo-target/mathew`).
- Wired env vars in `/etc/local-doorman/local-doorman.env`: `SERVICE_CONTENT_ENDPOINT`, `SLM_SHIM_TRAINING_CAPTURE=true`, `SLM_AUDIT_DIR=/var/lib/local-doorman/audit`.
- Fixed `ReadWritePaths` in `local-doorman.service` to include `/srv/foundry/data/cost-ledger`.
- Created `/var/lib/local-doorman/audit` directory with correct ownership.
- Verified end-to-end: Doorman `/v1/messages` → OLMo Tier A → Anthropic format response ✅.
- Confirmed training capture active: shadow brief enqueued on each commit.
- Tuned llama-server: `--threads 6` (from 3), `--threads-batch 8`, `CPUQuota=600%`. Applied to `/etc/systemd/system/local-slm.service` (system file).
- Perf comparison: baseline 1.71 tok/s (threads=3), tuned 1.95 tok/s (threads=6). Bottleneck is memory bandwidth, not compute. Plan written: `.agent/plans/olmo-performance-tuning.md`.
- Started `local-content` service (graph loading; 16-min startup).

**Pending / carry-forward:**
- Stage 6 promote (~43 commits) — Command Session task; rebase required first (see inbox message)
- Tier C commercial API key — operator from laptop (deliberate: test local-only first)
- ANTHROPIC_BASE_URL routing for Claude Code — not yet set; discuss with operator
- local-slm.service + local-doorman.service changes are system files; track in `~/Foundry/infrastructure/local-slm/` — not committed in this archive
- `flash-attn on` test — see performance plan
- IQ4_XS quantization — potential ~1.2x speed gain

**Operator preferences surfaced:**
- "local level working 100% before going beyond" — test Tier A thoroughly before adding commercial API key (Tier C). No ANTHROPIC_API_KEY for now.
- Terse responses; keep going on typos.

---

## 2026-05-20 | Totebox | claude-code

**Done this session:**
- Sprint 2a: Tier C switched to native Anthropic Messages API in `external.rs` — `split_system()`, `AnthropicRequest/Response` wire structs, updated `happy_path` test to mock `/v1/messages` + `x-api-key`. 1 commit.
- Sprint 2b: `POST /v1/responses` OpenAI Responses API shim in `http.rs`; `ResponsesApiBody` + `ResponsesInput` structs; 2 tests in `http_test.rs`. 1 commit.
- P1-1.3: `bin/eval-adapter.sh` — F12 holdout scoring gate, difflib SequenceMatcher, `promoted: true` when regression ≤ 5%. 1 commit.
- P1-1.4: `bin/promote-corpus.sh` — operator SSH-signed corpus promotion gate; SYS-ADR-10 closed. 1 commit.
- P1-1.7: Tool-use round-trip — `ToolDef` + `content_blocks: Vec<ContentBlock>`; OAI tool_calls wiring Tier A/B; 2 shim tests. 1 commit.
- Sprint 3: `crates/slm-mcp-server/` — 6 Foundry MCP tools via rmcp 1.7.0 stdio (query-datagraph, mutate-datagraph, get-entity-context, get-corpus-stats, submit-extraction, doorman-health). `.mcp.json` at repo root. `bed0f229`. 250 tests pass.
- NEXT.md cleanup: ticked Sprint 0b / P3-3.2 / P1-1.7 done. `dd9eaa34`.

**Pending / carry-forward:**
- Stage 6 promote (~42 commits now) — Command Session task
- Rebuild + redeploy Doorman (after Stage 6) — Command Session
- Sprint 3 binary deploy: `cargo build --release -p slm-mcp-server && sudo cp target/release/slm-mcp-server /usr/local/bin/`
- P2-2.2 RelatedTo edges — blocked on editorial taxonomy ratification
- P2-2.7, P2-2.8 — deferred (service-content scope / sqlite-vec setup)
- Operator tasks: GCP billing budget, sign eval holdout, sign verdict batch

**Operator preferences surfaced:**
- Terse responses, no trailing summaries.
- Keep going on typos ("just a typo keep going").

---

## 2026-05-19 | Totebox | claude-code

**Done this session:**
- Task 3 (503 busy-rejection): `TierABusy` error variant + `is_busy()` health probe + `Box::pin` escalation + `Retry-After: 30` header. 3 commits (`c38e66de`, `e2a93a99`, `160668cd`). 123 slm-doorman tests pass.
- Task 4 (Anthropic shim integration): 5 new tests in `anthropic_shim_test.rs` (14 total). Fixed latent `doorman_error_to_status` E0004 compile error and `shadow_with_local_source_tier_returns_202` diff-length bug in `http_test.rs`. 1 commit (`93620c1b`). 241 workspace tests pass.
- Struck P3-3.5-followup in NEXT.md (was already done as of `80083e6e`).

**Pending / carry-forward:**
- NEXT.md needs Task 3 + Task 4 struck (done next session)
- Sprint 0b: real per-token streaming — done via yoyo_stream + local_stream
- P3-3.2: canary task set + `bin/canary-run.sh` — done as `77481f74`
- P1-1.7: tool-use round-trip — done as `661909d1`
- Stage 6: 36 commits unpromoted (Command Session task)

**Operator preferences surfaced:**
- No new preferences this session. Existing: terse responses, no trailing summaries.

