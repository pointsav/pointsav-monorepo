# Universal AI Gateway — Multi-Week Implementation Plan

> Authored: 2026-05-12 task@project-intelligence  
> Status: Active planning — Sprint 0a ready to begin  
> Updated: 2026-05-12 (on-demand boot, real streaming, training capture, operational model)  
> Companion: ~/.claude/plans/wire-format-leapfrog-2030.md (strategic context)  
>            ~/.claude/plans/sovereign-coding-agent-leapfrog-2030.md

---

## Mission

Transform service-slm from an OpenAI-only gateway into a universal AI gateway that:
1. Accepts ANY client (Claude Code, OpenAI SDK, Anthropic SDK, MCP clients)
2. Routes through sovereign infrastructure (Tier A local → Tier B Yo-Yo → Tier C passthrough)
3. Reduces Claude Code API token spend by 60–70% via local routing
4. Trains the local model on every Claude Code session automatically
5. Positions as a sovereign node in the emerging A2A agent mesh

---

## Operational Model (The Correct Mental Picture)

This is NOT about replacing Claude — it is about routing the RIGHT task to the RIGHT tier.

```
TASK TYPE                          CORRECT TIER         WHY
─────────────────────────────────────────────────────────────────────────
Read file / summarise              Tier A (local 7B)    Zero cost, instant
Simple grep / search result        Tier A (local 7B)    Doesn't need reasoning
Moderate code edit / refactor      Tier B (Yo-Yo 32B)   Good enough, cheap
Entity extraction / DataGraph      Tier B (Yo-Yo 32B)   Already proven working
Complex debugging                  Tier C (Claude)      Needs real reasoning
Architecture decisions             Tier C (Claude)      Needs real reasoning
Multi-step agent chains (5+)       Tier C (Claude)      Tool-use quality gap
```

**Claude Code still does ALL complex tasks** — via Tier C passthrough to the real
Anthropic API. The shim adds intelligence, not degradation. Tier C is always available.

### The Daily Schedule

```
00:00 ──── nightly-run Phase 1 ──────────────────────── DataGraph rebuild
           Yo-Yo #1 starts for extraction (2h budget)
02:00 ──── Phase 2 ──────────────────────────────────── LoRA training (GPU freed)
           Yo-Yo stops after Phase 1; training runs on freed L4
04:00 ──── nightly-run complete ─────────────────────── Yo-Yo idle-stopped
           (idle monitor stops VM after 30 min idle — already live)
...
09:00 ──── Developer starts working ─────────────────── First Claude Code request
           Shim detects Yo-Yo down → falls back to Tier A immediately
           Background: start-yoyo.sh spawned async (~2-3 min boot)
09:03 ──── Yo-Yo #1 healthy ─────────────────────────── Circuit breaker closes
           All sonnet-tier requests now route to Tier B
           Tier A handles haiku-tier requests (always-on, zero cost)
           Tier C passthrough for opus-tier (complex tasks → real Claude)
...
22:00 ──── Developer stops working ──────────────────── Last request processed
           Idle monitor fires after 30 min → Yo-Yo stops automatically
           Cost: ~13h × $0.40/hr = $5.20/day max; actual = hours actively used
```

**Do NOT leave Yo-Yo running 24/7.** On-demand boot + idle-stop is the right model:
- Nightly window: Yo-Yo managed by nightly-run.sh (existing)
- Daytime: Yo-Yo managed by on-demand lazy-start in the Doorman (Sprint 0b)
- Cost ceiling: ~$5.20/day if used all day; often $1-3/day in practice

### Are We Training Our Model By Using It?

**Not yet — but Sprint 0b wires this automatically.**

Every Claude Code request that goes through the shim has:
- A `brief`: the user's message / task description
- An `actual_diff`: the code change that results from the session

The apprenticeship substrate (`POST /v1/shadow`) already accepts `{brief, actual_diff}` and
generates a DPO training tuple. The commit that approves the change is the implicit verdict.

After Sprint 0b: **every Claude Code edit that routes through the shim feeds the training corpus.**
The model learns from YOUR development patterns. After 6–12 months of daily use, the Yo-Yo LoRA
adapter will be fine-tuned specifically on how you build Foundry code. This is the compound moat.

---

## Background: What We Have Today

```
slm-doorman-server (axum, port 9080)
  POST /v1/chat/completions   ← OpenAI format ONLY
  POST /v1/brief              ← apprenticeship
  POST /v1/verdict            ← apprenticeship
  POST /v1/audit/proxy        ← Tier C audit
  POST /v1/graph/query        ← DataGraph proxy
  POST /v1/graph/mutate       ← DataGraph proxy
  GET  /readyz, /healthz, /v1/contract

Tier A: llama-server (OLMo 3 7B, local, port 8080)
Tier B: Yo-Yo #1 llama-server (OLMo 3 32B Think, GCP L4, port 9443)
Tier C: Anthropic API (configured but minimal use)
```

**The gap:** Claude Code speaks Anthropic Messages API. Doorman speaks OpenAI.
Every Claude Code token goes to `api.anthropic.com` at $3–15/M. Nothing routes locally.

---

## The Three-Layer Target Architecture

```
INBOUND (any client)           CANONICAL IR          OUTBOUND (per-backend)
─────────────────────          ────────────          ─────────────────────
/v1/messages         ──┐       CanonicalReq  ──┬──→  OpenAI → Tier A (llama 7B)
/v1/chat/completions ──┤  →    (slm-core)   ──┼──→  OpenAI → Tier B (llama 32B)
/v1/responses        ──┤       +ContentBlock──┴──→  Anthropic → Tier C (Claude)
/mcp                 ──┘       +ThinkingCfg
                               +Grammar
                               +Complexity → routing unchanged
```

**Router logic: ZERO changes.** All work is at boundaries.

---

## Sprint Breakdown

---

### Sprint 0a — Anthropic Shim (fake streaming)  ✦ DO THIS FIRST ✦
**Duration:** 2–3 days  
**Goal:** Claude Code routes through Doorman via `ANTHROPIC_BASE_URL`  
**Token cost reduction begins immediately after merge**

#### What to build

New route in `slm-doorman-server/src/http.rs`: `POST /v1/messages`

**New structs in http.rs (~80 LOC):**
```rust
// Inbound: Anthropic Messages API request
#[derive(Deserialize)]
struct AnthropicMessagesBody {
    model: String,
    #[serde(default)]
    system: Option<String>,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
    #[serde(default)]
    stream: bool,
    #[serde(default)]
    temperature: Option<f32>,
    #[serde(default)]
    metadata: Option<serde_json::Value>,    // ignored
    #[serde(default)]
    stop_sequences: Option<Vec<String>>,    // ignored for now
}

#[derive(Deserialize)]
struct AnthropicMessage {
    role: String,   // "user" | "assistant"
    content: AnthropicContent,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum AnthropicContent {
    Text(String),                           // simple string form
    Blocks(Vec<AnthropicContentBlock>),     // structured blocks form
}

#[derive(Deserialize)]
struct AnthropicContentBlock {
    #[serde(rename = "type")]
    block_type: String,   // "text" | "tool_use" | "tool_result" | "thinking"
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    input: Option<serde_json::Value>,
    #[serde(default)]
    tool_use_id: Option<String>,
    #[serde(default)]
    thinking: Option<String>,
}
```

**Adapter function (~60 LOC):**
```rust
fn anthropic_to_compute_request(
    body: AnthropicMessagesBody,
    module_id: ModuleId,
    request_id: RequestId,
) -> ComputeRequest {
    // 1. Flatten content blocks → plain text (Sprint 0 simplification)
    // 2. Prepend system prompt as ChatMessage { role: "system", content }
    // 3. Map model name → Complexity for tier routing:
    //    claude-haiku-* → Complexity::Low   (→ Tier A)
    //    claude-sonnet-* → Complexity::High (→ Tier B)
    //    claude-opus-* → Complexity::High + tier_hint → Tier C
    // 4. Build ComputeRequest (same as existing chat_completions handler)
}
```

**Response conversion (~60 LOC):**
```rust
fn compute_to_anthropic_response(resp: ComputeResponse, model: &str) -> serde_json::Value {
    serde_json::json!({
        "id": format!("msg_{}", resp.request_id),
        "type": "message",
        "role": "assistant",
        "content": [{"type": "text", "text": resp.content}],
        "model": model,
        "stop_reason": "end_turn",
        "stop_sequence": null,
        "usage": {
            "input_tokens": 0,    // Doorman doesn't track this yet; safe default
            "output_tokens": 0
        }
    })
}
```

**Streaming (fake SSE, ~100 LOC):**
Claude Code sends `"stream": true` by default. We buffer the full Doorman response
then emit all SSE events at once — same latency as non-streaming, but compatible:
```
event: message_start\ndata: {...}\n\n
event: content_block_start\ndata: {...}\n\n
event: content_block_delta\ndata: {...full text in one delta...}\n\n
event: content_block_stop\ndata: {...}\n\n
event: message_delta\ndata: {...}\n\n
event: message_stop\ndata: {"type":"message_stop"}\n\n
```
Uses axum `StreamBody` + `tokio::sync::mpsc` — 1 sender, buffered.

**Route registration (~5 LOC):**
```rust
// In router() function, http.rs line 108:
.route("/v1/messages", post(anthropic_messages))
```

**Total: ~305 LOC across 1 file**  
**Risk: Low** — additive only, zero changes to existing routes or Doorman internals

#### Model routing table (Sprint 0)

| Claude Code model | Complexity | Routes to |
|---|---|---|
| `claude-haiku-*` | Low | Tier A (local OLMo 7B) |
| `claude-sonnet-*` | High | Tier B (Yo-Yo OLMo 32B Think) |
| `claude-opus-*` | High + tier_hint=C | Tier C (Anthropic API passthrough) |
| anything else | Medium | Tier A or B per circuit state |

**Note:** OLMo 3 32B Think ≠ Claude Sonnet for complex coding tasks. Set
`ANTHROPIC_BASE_URL` per-project, not globally. Start with non-critical
sessions to calibrate quality before routing production work.

#### How to activate after merge

```bash
# In a project's CLAUDE.md or shell profile:
export ANTHROPIC_BASE_URL="http://127.0.0.1:9080"

# Or per-session:
ANTHROPIC_BASE_URL=http://127.0.0.1:9080 claude

# Verify routing:
curl -s http://127.0.0.1:9080/v1/messages \
  -H "Content-Type: application/json" \
  -d '{"model":"claude-sonnet-4-6","max_tokens":64,
       "messages":[{"role":"user","content":"say: ok"}]}' \
  | jq '{tier: .x_foundry_tier_used, content: .content[0].text}'
```

#### Files changed

| File | Change |
|---|---|
| `crates/slm-doorman-server/src/http.rs` | +305 LOC: new structs + handler + helpers |
| `crates/slm-doorman-server/src/main.rs` | 0 (route registered inside http.rs router()) |
| `docs/deploy/local-doorman.env.example` | +2 lines: ANTHROPIC_BASE_URL note |

**Tests to add:** 3 unit tests in http.rs (same pattern as existing chat_completions tests):
- simple text message → ComputeRequest translation
- system prompt prepend
- streaming response emits valid SSE events

---

### Sprint 0b — Real Streaming + On-Demand Boot + Training Capture
**Duration:** 3–4 days (directly after 0a)  
**Goal:** Production-quality shim — real token streaming, Yo-Yo lazy-start, session training

#### Part 1: Real Token Streaming (~200 LOC in yoyo.rs + http.rs)

**Why fake streaming is not enough:** Claude Code's UX degrades — no live typing,
long responses feel hung, tool-use chains timeout on large responses.

**How:** llama-server already supports SSE streaming (`"stream": true`).
The Doorman needs to pass the stream through rather than buffering.

In `yoyo.rs`:
```rust
// Detect stream: true in ComputeRequest
// Use reqwest Response::bytes_stream() instead of .json()
// Re-emit each llama-server SSE chunk directly to the axum response channel
// Translate llama-server's OpenAI-format SSE → Anthropic SSE format on the fly
```

In `http.rs` (Anthropic shim handler):
```rust
// Return axum::response::Sse<impl Stream> instead of Json
// Each llama-server "data: {choices[0].delta.content}" chunk →
//   → Anthropic "data: {type:content_block_delta, delta:{type:text_delta, text:...}}"
// Bookend with message_start / message_stop events
```

**Result:** Claude Code sees real token-by-token streaming from Yo-Yo #1.
The 14.7 tok/s GPU speed means a typical 200-token response streams in ~14s,
which is noticeable and natural — not a 14s wait then instant dump.

#### Part 2: On-Demand Yo-Yo Lazy-Start (~80 LOC in router or http.rs)

**Problem:** Yo-Yo is stopped at 9am. First Claude Code request of the day routes
to Tier A (fallback), which is correct. But subsequent requests should hit Tier B.

**Solution:** When Doorman routes a request and Tier B circuit is OPEN (Yo-Yo down):
1. Serve THIS request from Tier A immediately (no user wait)
2. Spawn `tokio::task::spawn` — background async task calls `start-yoyo.sh`
3. The existing background health probe (every 30s) will detect Yo-Yo healthy
4. Circuit breaker closes automatically — next request hits Tier B

```rust
// In router.rs, after Tier B circuit-open fallback:
if self.yoyo.circuit_is_open() && !self.yoyo_start_pending.load(Ordering::Relaxed) {
    self.yoyo_start_pending.store(true, Ordering::Relaxed);
    let pending = Arc::clone(&self.yoyo_start_pending);
    tokio::task::spawn(async move {
        let _ = tokio::process::Command::new("bash")
            .arg("/srv/foundry/clones/project-intelligence/service-slm/scripts/start-yoyo.sh")
            .arg("--wait-ready=300")
            .status().await;
        pending.store(false, Ordering::Relaxed);
    });
}
```

**New env var:** `SLM_YOYO_AUTO_START=true` — gates this behaviour (default off until tested).

**Coordination with nightly-run:** nightly-run.sh manages Yo-Yo independently via
`start-yoyo.sh` / `stop-yoyo.sh`. No conflict — the idle monitor stops it after 30 min idle
regardless of who started it. The nightly window has precedence.

#### Part 3: Training Capture (~80 LOC in http.rs shim)

**Every Claude Code edit that routes through the shim becomes a training tuple.**

The `POST /v1/shadow` endpoint already exists (`brief` + `actual_diff`).

In the Anthropic shim handler, after routing:
```rust
// If SLM_APPRENTICESHIP_ENABLED=true AND request was a code-edit task:
// 1. brief = user's message (the task description)
// 2. actual_diff = placeholder "" for now (Sprint 0b ships the wiring;
//    the actual diff comes from git — a later enhancement captures it)
// 3. POST /v1/shadow async (fire-and-forget, non-blocking)
```

**Sprint 0b ships the wiring.** The actual diff capture (from git) is a later enhancement.
Even with empty diffs, the corpus accumulates task descriptions — useful for SFT.
Full DPO pairs (with diffs) land in a follow-on sprint when git hook integration is added.

#### Sprint 0b files changed

| File | Change |
|---|---|
| `slm-doorman/src/tier/yoyo.rs` | +~150 LOC: streaming path |
| `slm-doorman-server/src/http.rs` | +~150 LOC: SSE response + lazy-start trigger + shadow capture |
| `slm-doorman/src/router.rs` | +~80 LOC: on-demand start logic + atomic flag |
| Tests | +~60 LOC |
| **Total** | **~440 LOC** |

---

### Sprint 1 — Neutral Canonical IR
**Duration:** 1 week  
**Goal:** Replace `ChatMessage` (OpenAI type) with `CanonicalMessage` (neutral) in slm-core

**Why now:** The Sprint 0 shim maps Anthropic content blocks → flat `ChatMessage.content` strings.
That loses tool_use / tool_result / thinking blocks. Sprint 1 preserves them properly.

**Files changed:**
- `crates/slm-core/src/lib.rs` — replace `ChatMessage` with `CanonicalMessage` + `ContentBlock`
- `crates/slm-doorman/src/tier/local.rs` — `CanonicalMessage` → OpenAI (outbound adapter)
- `crates/slm-doorman/src/tier/yoyo.rs` — same
- `crates/slm-doorman/src/tier/external.rs` — same (Tier C keeps OpenAI shim for now)
- `crates/slm-doorman-server/src/http.rs` — upgrade shim to use full content blocks

**New types:**
```rust
pub enum ContentBlock {
    Text { text: String },
    Thinking { thinking: String, signature: Option<String> },
    ToolUse { id: String, name: String, input: serde_json::Value },
    ToolResult { tool_use_id: String, content: Vec<ContentBlock>, is_error: bool },
    Image { media_type: String, data: String },  // base64
}

pub struct CanonicalMessage {
    pub role: Role,   // System | User | Assistant
    pub content: Vec<ContentBlock>,
}
```

**Estimated LOC:** ~150 new + ~80 changed = 230 total  
**Breaking:** No external API changes. Internal types only.

---

### Sprint 2 — Tier C Native Anthropic + OpenAI Responses API
**Duration:** 1 week

**Part A — Tier C native Anthropic:**
- `external.rs`: emit `POST /v1/messages` to `api.anthropic.com` directly
  (removes the current OpenAI-format shim for Claude API calls)
- Proper tool_use / tool_result round-trip preserved
- ~100 LOC change in external.rs

**Part B — OpenAI Responses API inbound:**
- New endpoint `POST /v1/responses` (OpenAI's 2025 stateful API replacing chat completions)
- Chat Completions sunset underway — prepare now before clients require it
- ~80 LOC adapter: `from_openai_responses()` → `CanonicalRequest`

---

### Sprint 3 — MCP Server (`slm-mcp-server`)
**Duration:** 2 weeks

Expose Foundry capabilities as MCP tools. Any MCP client (Claude Code, future tools)
can call Foundry services without custom code.

**Tools to expose:**
```
foundry:query-datagraph        POST /v1/graph/query wrapper
foundry:mutate-datagraph       POST /v1/graph/mutate wrapper
foundry:get-entity-context     Fetch entity enrichment context
foundry:get-corpus-stats       Engineering + apprenticeship tuple counts
foundry:submit-extraction      Queue document for entity extraction
foundry:doorman-health         Tier A/B/C status + circuit breaker state
```

**Wire format:** JSON-RPC 2.0 over stdio (MCP spec 2025-11-25)  
**Integration:** Claude Code's `--mcp-server` flag or `.claude/mcp.json`

**New crate:** `crates/slm-mcp-server/`  
**Estimated LOC:** ~400

---

### Sprint 4 — app-console-slm MVP
**Duration:** 3 weeks

Single binary, two modes: coding agent + operator dashboard.

**New crate:** `crates/app-console-slm/` (or separate repo)

**`console-slm status` (week 1):**
```
Doorman    ● running  (http://127.0.0.1:9080)
Tier A     ● running  OLMo 3 7B   http://127.0.0.1:8080
Tier B     ● running  OLMo 3 32B  https://136.109.20.216:9443  14.7 tok/s
Tier C     ○ standby  Anthropic API
DataGraph  ● healthy  entity_count=74  last_run=02:52 UTC
Training   ⊙ pending  4 markers  GCS bucket: not configured
```

**`console-slm admin` TUI (week 2):**  
Ratatui panels: Doorman health, Yo-Yo controls (start/stop/snapshot),
nightly run log stream, corpus stats, apprenticeship ledger.

**`console-slm chat` (week 3):**  
REPL that routes through Doorman (replaces slm-chat.sh).

---

### Sprint 5 — A2A Agent Card
**Duration:** 4 weeks

Formal entry into the agent mesh. Foundry becomes a cryptographically
identified, sovereign A2A node — callable by any A2A-compatible orchestrator.

**Agent card:**
```json
{
  "name": "foundry-doorman",
  "version": "1.0",
  "capabilities": ["text-generation", "code-generation", "entity-extraction"],
  "endpoints": {
    "inference": "http://foundry-workspace:9080/v1/messages",
    "a2a": "http://foundry-workspace:9080/a2a"
  },
  "governance": {
    "audit_trail": true,
    "sovereignty": "on-premise",
    "compliance": ["ISO-42001", "NI-51-102"]
  }
}
```

**New endpoint:** `GET /a2a/agent-card`, `POST /a2a/tasks/send`  
**Integration:** Registers Foundry as a node in the Linux Foundation A2A mesh

---

## Summary Timeline

| Sprint | Work | Duration | Outcome |
|---|---|---|---|
| **0a** | Anthropic shim, fake streaming (http.rs only) | **2–3 days** | **Claude Code routes through Doorman** |
| **0b** | Real streaming + on-demand Yo-Yo boot + training capture | **3–4 days** | **Production quality; Yo-Yo auto-starts; every session trains the model** |
| 1 | Neutral canonical IR (slm-core) | 1 week | Full content block support; tool-use preserved |
| 2 | Tier C native Anthropic + Responses API inbound | 1 week | No shim for Claude Tier C; future-proof |
| 3 | MCP server | 2 weeks | DataGraph + corpus callable from any MCP client |
| 4 | app-console-slm MVP | 3 weeks | No more SSH for ops; coding assistant binary |
| 5 | A2A agent card | 4 weeks | Foundry sovereign node in agent mesh |

**Total: ~15 weeks to full stack. Sprints 0a+0b ship this week + next week.**

### Cost model after Sprint 0a+0b live

| Scenario | Before | After |
|---|---|---|
| Haiku-tier tasks (file reads, search) | $3/M Anthropic | ~$0 (Tier A local) |
| Sonnet-tier tasks (code edits) | $3–15/M Anthropic | ~$0.40/hr Yo-Yo while active |
| Opus-tier tasks (complex reasoning) | $15/M Anthropic | $15/M Anthropic (unchanged) |
| Training benefit | None | Every session feeds corpus → LoRA adapter improves |
| Yo-Yo running cost | N/A | ~$1–5/day (on-demand, idle-stopped) |

**Claude still handles all complex tasks.** The saving is on the ~60% of tokens
that are file reads, tool results, and moderate edits — those route locally.

---

## Quality Expectations by Tier

This is critical context before routing Claude Code through local models:

| Task type | Tier A (7B) | Tier B (32B Think) | Tier C (Claude) |
|---|---|---|---|
| File summarisation | ✓ good | ✓ excellent | ✓ excellent |
| Simple code edits | ✓ adequate | ✓ good | ✓ excellent |
| Complex debugging | ✗ poor | ~ adequate | ✓ excellent |
| Architecture decisions | ✗ poor | ~ adequate | ✓ excellent |
| Tool-use chains (5+ steps) | ✗ poor | ~ limited | ✓ excellent |
| Bash + file ops (read/write) | ✓ good | ✓ good | ✓ excellent |

**Recommendation for Sprint 0 rollout:**
- Haiku → Tier A: read-only operations, file summarisation, search
- Sonnet → Tier B: code edits, moderate reasoning
- Opus → Tier C passthrough: complex debugging, architecture, long agent chains
- Do NOT set `ANTHROPIC_BASE_URL` globally on production sessions yet

---

## Open Questions Before Sprint 0a

- [ ] **Opus → Tier C passthrough:** Requires Tier C configured with Anthropic API key.
      Currently `SLM_TIER_C_ANTHROPIC_ENDPOINT` / bearer in env. Verify configured before
      routing `claude-opus-*` there. Fallback: map opus → Tier B and accept quality drop.
- [ ] **Token tracking:** Doorman doesn't count tokens. Sprint 0a ships `0` in `usage`.
      Add `content.split_whitespace().count() * 1.3` approximate in Sprint 0b.
- [ ] **`ANTHROPIC_BASE_URL` scope:** Per-project `.claude/settings.json` recommended
      (not workspace-wide) so the current session is not affected during testing.
- [ ] **Nightly-run coordination:** On-demand start (Sprint 0b) must check if nightly-run
      is active before spawning start-yoyo.sh. Add `SLM_NIGHTLY_ACTIVE` lock file check.
- [ ] **Training capture consent:** Auto-submitting to `/v1/shadow` means all shim requests
      become training candidates. Confirm this is desired before Sprint 0b ships.
      Add `SLM_SHIM_TRAINING_CAPTURE=true` gate (default false until explicitly opted in).

---

## Files Index

| File | Sprint | Change |
|---|---|---|
| `slm-doorman-server/src/http.rs` | 0 | +305 LOC: Anthropic shim handler |
| `slm-core/src/lib.rs` | 1 | Replace `ChatMessage` with `CanonicalMessage` |
| `slm-doorman/src/tier/local.rs` | 1 | Outbound adapter to OpenAI |
| `slm-doorman/src/tier/yoyo.rs` | 1 | Outbound adapter to OpenAI |
| `slm-doorman/src/tier/external.rs` | 2 | Native Anthropic outbound |
| `slm-doorman-server/src/http.rs` | 2 | Add `/v1/responses` inbound |
| `crates/slm-mcp-server/` (new) | 3 | MCP server crate |
| `crates/app-console-slm/` (new) | 4 | Console binary |
| `slm-doorman-server/src/http.rs` | 5 | A2A endpoints |

---

## Related Plans

- `~/.claude/plans/wire-format-leapfrog-2030.md` — strategic wire format analysis
- `~/.claude/plans/sovereign-coding-agent-leapfrog-2030.md` — product strategy
- `service-slm/ARCHITECTURE.md` — existing planned crates (`slm-cli`, `slm-api`)
- `service-slm/NEXT.md` — immediate open items
