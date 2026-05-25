---
schema: foundry-doc-v1
document_version: 1.0.0
research_provenance: direct-consultation
research_inline: true
authored: 2026-05-16
authored_by: project-intelligence Totebox (four-Opus parallel research session)
authored_with: claude-sonnet-4-6
cites: []
---

# TOPIC: Anthropic Messages API Gateway Integration

## 1. What the Shim Does

`POST /v1/messages` at `slm-doorman-server/src/http.rs:1214` is the Anthropic Messages
API shim (Sprint 0a). It accepts requests in Anthropic wire format and translates them
into the Doorman's internal `ComputeRequest`, routing to Tier A, B, or C based on the
model name field.

The shim is an inbound adapter only. It does not change the Doorman's internal routing
logic. Tier A and Tier B backends receive OpenAI-format requests (unchanged). Tier C
currently emits OpenAI format to Anthropic (Sprint 2 replaces this with native Anthropic
Messages API to remove the unnecessary round-trip shim on the outbound path).

## 2. `ANTHROPIC_BASE_URL` — Official Anthropic Feature

`ANTHROPIC_BASE_URL` is an officially documented Claude Code environment variable,
used by Anthropic's own Bedrock and Vertex integrations. It redirects all Anthropic
Messages API inference calls to the specified base URL. The CLI appends `/v1/messages`
to the value provided.

Calls that do NOT route through `ANTHROPIC_BASE_URL`:
- Telemetry and error reporting (goes to Anthropic-owned hosts regardless)
- Auth/OAuth handshake
- Update checks

Set `DISABLE_TELEMETRY=1` and `DISABLE_AUTOUPDATER=1` to suppress those if required.

## 3. `.claude/settings.json` Schema

```json
{
  "env": {
    "ANTHROPIC_BASE_URL": "http://127.0.0.1:9080",
    "ANTHROPIC_AUTH_TOKEN": "<gateway-local-token>",
    "ANTHROPIC_SMALL_FAST_MODEL": "claude-haiku-4-5-20251001",
    "CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY": "true"
  },
  "apiKeyHelper": "/path/to/script-that-prints-token",
  "model": "claude-sonnet-4-6",
  "permissions": { "allow": [], "deny": [] },
  "mcpServers": {}
}
```

`ANTHROPIC_SMALL_FAST_MODEL` overrides the haiku-tier model name — set it to a name the
routing table recognises. `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY` causes Claude Code
to query `/v1/models` at startup for available model names rather than using its built-in
list. Verify the exact flag name against current Claude Code documentation before relying
on it.

**Settings hierarchy** (later overrides earlier):
1. `~/.claude/settings.json` — user/global
2. `<project>/.claude/settings.json` — project (commit-tracked)
3. `<project>/.claude/settings.local.json` — project-local (gitignored)
4. Process environment variables — highest priority

## 4. Credential Design

`ANTHROPIC_AUTH_TOKEN` set in the client is validated by the shim only. It is never
forwarded upstream to any backend. It is an arbitrary shared secret between the client
and the local Doorman.

The shim must strip the `Authorization` header from all upstream calls. If `ANTHROPIC_AUTH_TOKEN`
is absent from the client, Claude Code may attach a cached OAuth token from `~/.claude/`
— do not let this forward to Tier C. The gateway strips all inbound authorization headers
before forwarding to backends.

Tier C outbound uses `ANTHROPIC_API_KEY` (a Commercial API key) set in the Doorman server
environment. This is entirely separate from any client credential.

## 5. Pro/Max Subscription Compatibility

Claude Pro/Max subscription uses an OAuth bearer token issued to and bound by
`api.anthropic.com`. When `ANTHROPIC_BASE_URL` redirects traffic to a local gateway,
that token is no longer sent to Anthropic — it is validated by the local shim and
discarded. Max subscription credits do not apply to gateway sessions.

The correct setup for a developer with a Max subscription:
- **Gateway sessions** (ANTHROPIC_BASE_URL set): Tier A at $0, Tier B at ~$0.40/hr,
  Tier C at pay-per-token via Commercial API key on Doorman. Max subscription does
  not bill here.
- **Direct sessions** (ANTHROPIC_BASE_URL absent): Max subscription applies normally.

These are mutually exclusive. One session cannot use both Max billing and gateway routing.

## 6. SSE Streaming Requirements

Claude Code sends `"stream": true` by default. The gateway must:

1. Return `Content-Type: text/event-stream` and disable buffering
2. Emit events in the correct sequence:
   ```
   event: message_start\ndata: {...}\n\n
   event: content_block_start\ndata: {...}\n\n
   event: content_block_delta\ndata: {...}\n\n   (one or more)
   event: content_block_stop\ndata: {...}\n\n
   event: message_delta\ndata: {...}\n\n
   event: message_stop\ndata: {"type":"message_stop"}\n\n
   ```
3. Emit `ping` events approximately every 15 seconds on long responses to prevent
   Claude Code from timing out while waiting for the next token
4. Return correct `stop_reason` values: `"end_turn"` for normal completion,
   `"tool_use"` when the response contains a tool_use block, `"max_tokens"` when
   the token budget is reached. A mismatch causes Claude Code to loop or hang.

Sprint 0a uses buffered fake streaming: the full Doorman response is buffered and all
SSE events are emitted at once. This is protocol-correct but produces no live typing
effect. Sprint 0b replaces this with real token-by-token streaming from llama-server.

## 7. Sprint 0a Known Limitations

| Limitation | Impact | Fixed in |
|---|---|---|
| `tool_use`/`tool_result` blocks flattened to strings | Claude Code agent loop breaks on multi-step tool chains | Sprint 1 |
| No real SSE streaming | No live typing; long responses feel hung | Sprint 0b |
| `cache_control` blocks not preserved | Prompt caching disabled for Tier C (5–10× cost impact) | Sprint 1 |
| `usage.input_tokens` / `output_tokens` return 0 | Budget display broken; function unaffected | Sprint 0b |
| No on-demand Yo-Yo boot | First haiku/sonnet request of the day falls back to Tier A | Sprint 0b |

## 8. Model Routing Table (Sprint 0a)

| Claude Code model | `Complexity` | `yoyo_label` | Routes to |
|---|---|---|---|
| `claude-haiku-*` | Low | — | Tier A (OLMo 2 1B, local, $0) |
| `claude-sonnet-*` | High | `"trainer"` | Tier B Yo-Yo #1 (OLMo 3 32B Think) |
| `claude-opus-*` | High | — | Tier C (Anthropic API, Commercial key) |
| unknown | Medium | `"trainer"` | Tier B or Tier A per circuit state |

The `"graph"` Yo-Yo label (Yo-Yo #2 / Llama 3.3 70B / H100) is not used for Claude Code
routing. That backend is reserved for grammar-constrained DataGraph extraction batch jobs.

## 9. Prompt Caching — Sprint 1 Requirement

Anthropic's prompt caching delivers 5–10× token cost reduction for repeated system-prompt
content. Claude Code uses `cache_control` content block annotations to mark cacheable
prefixes. Sprint 0a strips these annotations. Sprint 1 (Canonical IR) preserves
`cache_control` end-to-end, restoring the cost benefit for Tier C calls.

Until Sprint 1, Tier C costs are higher than necessary by this factor.

## 10. References

- `slm-doorman-server/src/http.rs:1214` — Sprint 0a implementation
- `topic-claude-code-sovereign-routing.md` — decision rationale and sprint sequence
- `topic-tos-training-constraints.md` — credential and terms constraints
- `guide-activate-anthropic-shim.md` — step-by-step activation runbook
- `.agent/plans/universal-ai-gateway.md` — full sprint-level implementation plan
