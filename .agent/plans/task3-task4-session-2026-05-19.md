# Task 3 + Task 4 — Session 2026-05-19

> **Status:** COMPLETE — both tasks committed, 241 tests pass, Stage 6 pending.
> **Branch:** main — 36 commits ahead of origin.
> **Next session:** update NEXT.md to strike Task 3 + Task 4, then start Sprint 0b or P3-3.2.

---

## What was done

### Task 3 — 503 busy-rejection (Tier A slots_idle=0)

Three commits landed:

| Commit | Description |
|--------|-------------|
| `c38e66de` | Core: `TierABusy` error, `is_busy()` health probe in `local.rs`, `Box::pin` escalation in `router.rs`, `Retry-After: 30` in `http.rs` |
| `e2a93a99` | Fix: `retry_after_secs: None` on 3 direct `ApiError` struct sites (compile error) |
| `160668cd` | Tests: grammar tests updated with health mocks; 3 new busy-probe tests (123 pass) |

**Design decisions:**
- Grammar validation (Lark check) runs BEFORE the `/health` busy probe — invalid input is rejected cheaply before any network I/O
- `is_busy()` returns `false` on any network/parse error — health endpoint misconfiguration never blocks inference
- `Box::pin(self.dispatch(Tier::Yoyo, req)).await` required in `router.rs` because `dispatch` is async + recursive (E0733)
- When Tier A is busy AND Tier B is available, escalates to Tier B silently (logged at info)
- When Tier A is busy AND no Tier B, propagates `TierABusy` → 503 + `Retry-After: 30`

**Files changed:**
- `crates/slm-doorman/src/error.rs` — `TierABusy` variant
- `crates/slm-doorman/src/tier/local.rs` — `LlamaHealthResponse`, `is_busy()`, restructured `complete()`
- `crates/slm-doorman/src/router.rs` — `Box::pin` escalation arm, `classify_error` mapping
- `crates/slm-doorman-server/src/http.rs` — `retry_after_secs` on `ApiError`, `Retry-After` header

---

### Task 4 — Anthropic shim integration tests

One commit landed:

| Commit | Description |
|--------|-------------|
| `93620c1b` | 5 new shim tests (14 total), `doorman_error_to_status` exhaustive match fix, shadow test diff-length fix |

**5 new tests in `tests/anthropic_shim_test.rs`:**
1. `tier_a_busy_returns_503_with_retry_after_header` — end-to-end Task 3 path through HTTP layer
2. `stream_true_with_tier_a_only_emits_fake_sse` — `stream: true` with Tier A only → fake-SSE fallback
3. `non_streaming_response_shape_matches_anthropic_spec` — full Anthropic envelope validated
4. `system_message_is_sent_to_tier_a_backend` — `system` field threads as first downstream message
5. `tool_result_content_block_passes_through_gateway` — multi-turn with `tool_use` + `tool_result`

**2 fixes in `tests/http_test.rs`:**
- Added `TierABusy → 503` and `CorpusGateRejected → 422` to `doorman_error_to_status` helper (was E0004 compile error, blocked 53 tests)
- `shadow_with_local_source_tier_returns_202`: diff was 18 chars vs `MIN_DIFF_CHARS = 20`; padded to 31 chars

---

## Test counts after this session

| Crate / suite | Count |
|---------------|-------|
| slm-core | 18 |
| slm-doorman (unit) | 123 |
| slm-doorman-server (lib) | 29 |
| slm-doorman-server (shim) | 14 |
| slm-doorman-server (audit) | 4 |
| slm-doorman-server (http) | 53 |
| **Total** | **241** |

---

## What's next (in priority order)

### Immediate / no blockers

- [ ] **Strike Task 3 + Task 4 in NEXT.md** (first commit next session)
- [ ] **Sprint 0b** — Replace fake-SSE burst `anthropic_sse_body()` in `http.rs` with real per-token streaming (~60 LOC). No blockers. File: `crates/slm-doorman-server/src/http.rs` — search `anthropic_sse_body`.
- [ ] **P3-3.2** — Canary task set + `bin/canary-run.sh`. Skeleton shipped in `9454bac4`; needs task definitions fleshed out.

### Needs operator sign-off first

- [ ] **P1-1.7** — Tool-use round-trip (~300 LOC). Operator must approve API shape: `tools: Vec<ToolDef>` + `ContentBlock` response.

### Blocked on other work

- [ ] **P2-2.2** RelatedTo edges — needs editorial taxonomy ratification (outbox staged)
- [ ] **P2-2.3** `/v1/editorial/seed` — blocked on P2-2.2 + P2-2.1
- [ ] **P2-2.6** `/v1/editorial/grammar` — blocked on editorial vocab
- [ ] **P3-3.3-followup** adapter A/B dual-dispatch — skeleton only
- [ ] **P3-3.4-followup** Sigstore adapter signing — operator key needed

### Ops (Command Session only)

- [ ] **Stage 6 promote** — 36 commits unpromoted. Run `echo "y" | ~/Foundry/bin/promote.sh` from Command Session (stash `settings.local.json` first).
- [ ] **Rebuild + redeploy Doorman** after Stage 6.
- [ ] **Flip `SLM_APPRENTICESHIP_ENABLED=true`** + drain paused briefs.
- [ ] **`bin/sync-local.sh --all`** after Stage 6.

---

## Key file locations

| What | Path |
|------|------|
| Shim handler | `crates/slm-doorman-server/src/http.rs:1447` |
| Shim conversion fns | `http.rs:1550` (`anthropic_to_compute_request`), `http.rs:1609` (`compute_to_anthropic_response`), `http.rs:1772` (`anthropic_sse_body`) |
| Busy probe | `crates/slm-doorman/src/tier/local.rs` — `is_busy()` |
| Escalation arm | `crates/slm-doorman/src/router.rs:241` — `dispatch()` Tier::Local arm |
| Retry-After header | `crates/slm-doorman-server/src/http.rs` — `IntoResponse for ApiError` |
| Shim tests | `crates/slm-doorman-server/tests/anthropic_shim_test.rs` |
| http integration tests | `crates/slm-doorman-server/tests/http_test.rs` |
| NEXT.md | `service-slm/NEXT.md` |
