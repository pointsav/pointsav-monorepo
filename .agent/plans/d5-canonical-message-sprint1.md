# D5 — Sprint 1: CanonicalMessage + ContentBlock

**Status:** Research complete, implementation NOT started.
**Plan source:** `.agent/plans/declarative-sparking-storm.md` (audit item 11, ~230 LOC)

---

## Goal

Replace `ChatMessage { role: String, content: String }` in `slm-core` with a
proper content-block model that preserves tool_use / tool_result round-trips
through the gateway. This unlocks Claude Code's full agentic loop.

---

## New types to add to `service-slm/crates/slm-core/src/lib.rs`

```rust
/// Role of a participant in a conversation.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::System => "system",
            Role::Tool => "tool",
        }
    }
}

/// A typed content block in Anthropic Messages API format.
/// This is the canonical internal representation used by all tiers.
/// Tier clients are responsible for translating to their native wire format.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    /// Plain text. Serialises as `{"type":"text","text":"..."}`.
    Text { text: String },
    /// Tool invocation from the assistant. Serialises as Anthropic tool_use block.
    ToolUse { id: String, name: String, input: serde_json::Value },
    /// Tool result from the user turn. Serialises as Anthropic tool_result block.
    ToolResult { tool_use_id: String, content: String },
    /// Extended thinking block. Serialises as `{"type":"thinking","thinking":"..."}`.
    Thinking { thinking: String },
}

/// A message in the canonical neutral format.
/// All tier clients translate FROM this format TO their native wire format.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanonicalMessage {
    pub role: Role,
    pub content: Vec<ContentBlock>,
}

impl CanonicalMessage {
    /// Convenience constructor for simple single-text messages.
    pub fn text(role: impl Into<String>, text: impl Into<String>) -> Self {
        let role_str: String = role.into();
        let role = match role_str.as_str() {
            "user" => Role::User,
            "assistant" => Role::Assistant,
            "system" => Role::System,
            "tool" => Role::Tool,
            _ => Role::User,
        };
        Self {
            role,
            content: vec![ContentBlock::Text { text: text.into() }],
        }
    }

    /// Extract plain text content (concatenates all text + thinking blocks).
    /// Used by apprenticeship and graph-context injection where only text matters.
    pub fn text_content(&self) -> String {
        self.content.iter().filter_map(|b| match b {
            ContentBlock::Text { text } => Some(text.as_str()),
            ContentBlock::Thinking { thinking } => Some(thinking.as_str()),
            _ => None,
        }).collect::<Vec<_>>().join("\n")
    }
}
```

---

## Change to `ComputeRequest`

In `slm-core/src/lib.rs` line 77:
```rust
// OLD:
pub messages: Vec<ChatMessage>,

// NEW:
pub messages: Vec<CanonicalMessage>,
```

Keep `ChatMessage` in slm-core — it is still used by `AuditProxyRequest.messages` (line 288).

---

## Tier client wire format translation

### Tier A (local.rs) + Tier B (yoyo.rs) — OpenAI wire format

Both backends use OpenAI `/v1/chat/completions`. OpenAI wire differs from Anthropic:
- Text → `{"role": "...", "content": "text"}` (string, not array)
- ToolUse → `{"role": "assistant", "content": null, "tool_calls": [{"id":...,"type":"function","function":{"name":...,"arguments":"<json-str>"}}]}`
- ToolResult → `{"role": "tool", "content": "...", "tool_call_id": "..."}`
- Thinking → fold into text (llama-server/vLLM don't support native thinking)

Define in local.rs and yoyo.rs (duplicate is fine; they're in different crates):

```rust
#[derive(Serialize)]
#[serde(untagged)]
enum OaiWireMessage {
    Text    { role: String, content: String },
    ToolCall { role: String, content: serde_json::Value, tool_calls: Vec<OaiToolCall> },
    ToolResult { role: String, content: String, tool_call_id: String },
}

#[derive(Serialize)]
struct OaiToolCall {
    id: String,
    #[serde(rename = "type")]
    kind: &'static str, // "function"
    function: OaiFunction,
}

#[derive(Serialize)]
struct OaiFunction {
    name: String,
    arguments: String, // JSON-encoded string (serde_json::to_string(&input))
}

fn canonical_to_oai(msgs: &[CanonicalMessage]) -> Vec<OaiWireMessage> {
    let mut out = Vec::new();
    for msg in msgs {
        let role = msg.role.as_str().to_string();
        // Partition blocks by type
        let texts: Vec<&str> = msg.content.iter().filter_map(|b| match b {
            ContentBlock::Text { text } => Some(text.as_str()),
            ContentBlock::Thinking { thinking } => Some(thinking.as_str()),
            _ => None,
        }).collect();
        let tool_uses: Vec<_> = msg.content.iter().filter_map(|b| match b {
            ContentBlock::ToolUse { id, name, input } => Some((id, name, input)),
            _ => None,
        }).collect();
        let tool_results: Vec<_> = msg.content.iter().filter_map(|b| match b {
            ContentBlock::ToolResult { tool_use_id, content } => Some((tool_use_id, content)),
            _ => None,
        }).collect();

        if !tool_uses.is_empty() {
            out.push(OaiWireMessage::ToolCall {
                role,
                content: serde_json::Value::Null,
                tool_calls: tool_uses.into_iter().map(|(id, name, input)| OaiToolCall {
                    id: id.clone(),
                    kind: "function",
                    function: OaiFunction {
                        name: name.clone(),
                        arguments: serde_json::to_string(input).unwrap_or_default(),
                    },
                }).collect(),
            });
        } else if !tool_results.is_empty() {
            for (tool_use_id, content) in tool_results {
                out.push(OaiWireMessage::ToolResult {
                    role: "tool".to_string(),
                    content: content.clone(),
                    tool_call_id: tool_use_id.clone(),
                });
            }
        } else {
            out.push(OaiWireMessage::Text {
                role,
                content: texts.join("\n"),
            });
        }
    }
    out
}
```

Change `OpenAiChatRequest.messages: Vec<ChatMessage>` → `Vec<OaiWireMessage>` in local.rs and yoyo.rs.
In the `complete()` / `start_stream()` methods, replace `req.messages.clone()` with `canonical_to_oai(&req.messages)`.

The response `OpenAiChatChoice.message` can keep `message: ChatMessage` for now — responses are text-only in Sprint 1.

### Tier C (external.rs) — Anthropic wire format

`CanonicalMessage` IS the Anthropic wire format. The Anthropic tier can forward the blocks directly.
Change `ExternalChatRequest.messages: Vec<ChatMessage>` → `Vec<CanonicalMessage>`.
The OpenAI-compatible providers (Gemini, OpenAI) need the same `canonical_to_oai` translation — wire this in `ExternalTierClient::complete()` per provider.

---

## http.rs changes

### Drop `flatten_anthropic_content`, add `canonical_content`

```rust
// Replace the flatten function with this:
fn canonical_content(content: AnthropicContent) -> Vec<ContentBlock> {
    match content {
        AnthropicContent::Text(s) => vec![ContentBlock::Text { text: s }],
        AnthropicContent::Blocks(blocks) => blocks.into_iter().filter_map(|b| {
            match b.block_type.as_str() {
                "text" => b.text.map(|t| ContentBlock::Text { text: t }),
                "thinking" => b.thinking.map(|t| ContentBlock::Thinking { thinking: t }),
                "tool_use" => {
                    let id = b.id.as_ref()?.as_str()?.to_string();
                    let name = b.name.as_ref()?.as_str()?.to_string();
                    let input = b.input.unwrap_or(serde_json::Value::Null);
                    Some(ContentBlock::ToolUse { id, name, input })
                }
                "tool_result" => {
                    let tool_use_id = b.tool_use_id.as_ref()?.as_str()?.to_string();
                    let content = match b.content? {
                        serde_json::Value::String(s) => s,
                        v => v.to_string(),
                    };
                    Some(ContentBlock::ToolResult { tool_use_id, content })
                }
                _ => None,
            }
        }).collect(),
    }
}
```

### Update `anthropic_to_compute_request`

```rust
// OLD:
let mut messages: Vec<ChatMessage> = Vec::new();
if let Some(system) = body.system {
    if !system.is_empty() {
        messages.push(ChatMessage { role: "system".to_string(), content: system });
    }
}
for msg in body.messages {
    messages.push(ChatMessage { role: msg.role, content: flatten_anthropic_content(msg.content) });
}

// NEW:
let mut messages: Vec<CanonicalMessage> = Vec::new();
if let Some(system) = body.system {
    if !system.is_empty() {
        messages.push(CanonicalMessage::text("system", system));
    }
}
for msg in body.messages {
    messages.push(CanonicalMessage {
        role: match msg.role.as_str() {
            "user" => Role::User,
            "assistant" => Role::Assistant,
            "system" => Role::System,
            _ => Role::User,
        },
        content: canonical_content(msg.content),
    });
}
```

### ChatCompletionsBody (line 193) — local shim

The `/v1/messages` ↔ `ComputeRequest` conversion in `ChatCompletionsBody` also uses `Vec<ChatMessage>`.
Change to `Vec<CanonicalMessage>` and update the two constructions at lines 531, 535:
```rust
// OLD:
ChatMessage { role: "system".to_string(), content: system_msg }
ChatMessage { role: msg.role, content: ... }

// NEW:
CanonicalMessage::text("system", system_msg)
CanonicalMessage { role: Role::from_str(&msg.role), content: canonical_content(msg.content) }
```

---

## Files changed summary

| File | Change |
|---|---|
| `slm-core/src/lib.rs` | Add `Role`, `ContentBlock`, `CanonicalMessage`; change `ComputeRequest.messages` type; update tests |
| `slm-doorman/src/tier/local.rs` | Add `OaiWireMessage` types + `canonical_to_oai`; update `OpenAiChatRequest.messages`; update tests |
| `slm-doorman/src/tier/yoyo.rs` | Same as local.rs |
| `slm-doorman/src/tier/external.rs` | Change `ExternalChatRequest.messages`; add OAI translation for non-Anthropic providers |
| `slm-doorman/src/apprenticeship.rs` | 4× `ChatMessage {..}` → `CanonicalMessage::text(...)` |
| `slm-doorman/src/mesh.rs` | 2× `ChatMessage {..}` → `CanonicalMessage::text(...)` |
| `slm-doorman/src/router.rs` | 2× `ChatMessage {..}` → `CanonicalMessage::text(...)` (1 prod + 1 test) |
| `slm-doorman-server/src/http.rs` | Drop `flatten_anthropic_content`, add `canonical_content`; update `anthropic_to_compute_request`; update ChatCompletionsBody; update 2 constructions |
| `slm-doorman-server/tests/http_test.rs` | 2-3× `ChatMessage {..}` → `CanonicalMessage::text(...)` |

**NOT changed:**
- `slm-core/src/lib.rs` `AuditProxyRequest.messages: Vec<ChatMessage>` — audit proxy uses OpenAI simple format; keep as-is
- `slm-doorman/src/audit_proxy.rs` — uses `AuditProxyRequest.messages`, not `ComputeRequest.messages`; keep as-is

---

## Key imports needed

```rust
// In all tier clients + http.rs:
use slm_core::{CanonicalMessage, ContentBlock, Role, ComputeRequest, ...};
// Drop: ChatMessage (except audit_proxy.rs)
```

---

## Test updates

All `ComputeRequest` test builders that use:
```rust
messages: vec![ChatMessage { role: "user".into(), content: "ping".into() }],
```
become:
```rust
messages: vec![CanonicalMessage::text("user", "ping")],
```

The slm-core tests also add round-trip serialization tests for `ContentBlock` and `CanonicalMessage`.

---

## Verification

```bash
cargo check --workspace      # should compile clean
cargo test  --workspace      # 177+ tests must pass
cargo clippy --workspace --all-targets -- -D warnings
```

The `tool_use` integration test in `anthropic_shim_test.rs` currently `#[ignore]`'d.
After D5, remove the `#[ignore]` and assert the tool_use block survives the round-trip
(the test already has the correct assertion structure; just gated by the ignore).
