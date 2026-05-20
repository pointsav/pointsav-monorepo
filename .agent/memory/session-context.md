# Session Context — Rolling 3-Session Summary

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

---

## 2026-05-18 | Totebox | claude-code (overnight build)

**Done:**
- 12 signed commits, ~3500 LOC across Phases 0–4 of learning-loop-master-plan
- Key modules: `corpus_gate.rs`, `adapter_registry.rs`, `cost_ledger.rs`, `metrics.rs`
- D5 Sprint 1: `CanonicalMessage` + `ContentBlock` replace flat `ChatMessage`
- c67bb284 drain fix; multiple NEXT.md/outbox ops commits

**Pending carry-forward:**
- Phase 4 outboxes to forward to project-editorial
- Tier C auth (Anthropic API key) not yet set in production env
- Yo-Yo #1 packer image rebuild (operator task, from laptop)
