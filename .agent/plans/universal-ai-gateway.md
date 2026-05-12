# Universal AI Gateway — Multi-Week Implementation Plan

> Authored: 2026-05-12 task@project-intelligence  
> Status: Active planning — Sprint 1 ready to begin  
> Companion: ~/.claude/plans/wire-format-leapfrog-2030.md (strategic context)  
>            ~/.claude/plans/sovereign-coding-agent-leapfrog-2030.md

---

## Mission

Transform service-slm from an OpenAI-only gateway into a universal AI gateway that:
1. Accepts ANY client (Claude Code, OpenAI SDK, Anthropic SDK, MCP clients)
2. Routes through sovereign infrastructure (Tier A local → Tier B Yo-Yo → Tier C passthrough)
3. Reduces Claude Code API token spend by 60–70% via local routing
4. Positions as a sovereign node in the emerging A2A agent mesh

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

### Sprint 0 — Anthropic Shim MVP  ✦ DO THIS NOW ✦
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
| **0** | Anthropic shim (http.rs only) | **2–3 days** | **Claude Code routes through Doorman TODAY** |
| 1 | Neutral canonical IR (slm-core) | 1 week | Full content block support; tool-use preserved |
| 2 | Tier C native + Responses API | 1 week | No OpenAI shim for Claude; future-proof inbound |
| 3 | MCP server | 2 weeks | DataGraph + corpus callable from any MCP client |
| 4 | app-console-slm MVP | 3 weeks | No more SSH for ops; coding assistant binary |
| 5 | A2A agent card | 4 weeks | Foundry in the agent mesh |

**Total: ~13 weeks to full stack.** Sprint 0 is this week.

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

## Open Questions Before Sprint 0

- [ ] Should `claude-opus-*` pass through to real Anthropic API (requires Tier C configured)?
      Or map to Tier B and accept quality drop?
- [ ] Token usage tracking: Doorman doesn't count tokens today.
      Add approximate count via `content.split_whitespace().count() * 1.3` placeholder?
- [ ] `ANTHROPIC_BASE_URL` scope: per-project `.claude/settings.json` or workspace-wide?

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
