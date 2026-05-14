# service-slm Architecture Analysis — 2026-05-14

> Authored: 2026-05-14 Opus agent via task@project-intelligence
> Status: Active — Sprint 0a prerequisites and defect list
> Companion: `.agent/plans/universal-ai-gateway.md`

---

## TL;DR

The crate architecture is clean. The routing logic is correct. Sprint 0a is structurally viable. Six prerequisites must be satisfied first — three are already partly done (the two drift fixes from this session), three are new code changes. The "never generate text" rule is clean — no violations found.

---

## 1. Crate Architecture — Sound

- `slm-core`: pure types, no I/O. Correct.
- `slm-doorman`: router + tier clients + ledger + apprenticeship + graph-context. No HTTP.
- `slm-doorman-server`: axum wiring only.

**One coupling concern**: graph-proxy handlers (`graph_query`, `graph_mutate`) in `http.rs:1011-1146` build `ReqwestClient::new()` per request and write audit entries directly. Logic belongs in `slm-doorman`, next to the existing `GraphContextClient`. Not a blocker for Sprint 0a.

---

## 2. Routing Paths

| Method | Trigger | Tier A fallback? |
|---|---|---|
| `route()` | `/v1/chat/completions`, `/v1/brief`, graph-augmented requests | Yes — on Tier B circuit-open or transient |
| `route_async()` | Nothing — orchestrator is `None` everywhere; dead code | N/A |
| `route_yoyo_only()` | `/v1/extract` only | **No** — returns `TierUnavailable(Yoyo)` immediately |
| `dispatch()` | Internal to `route()` | Yes, on Yoyo arm only |

### Circuit breaker (correct)
- 5 consecutive failures → Open; 300s cooldown → HalfOpen; any success → Closed
- `allow_request()` is side-effecting (Open→HalfOpen transition on cooldown expiry)
- `/v1/extract` calls participate in probing HalfOpen state — correct but worth documenting

### Tier A fallback trigger
`router.rs:280-292`: fires on `!client.allow_request()` (circuit open) or `is_transient_tier_b_failure(e)`. Does NOT fire from `route_yoyo_only()`. The SYS-ADR-07 boundary for extraction is enforced.

---

## 3. Audit Ledger — Five Entry Types

| Entry | `entry_type` | Written by |
|---|---|---|
| `AuditEntry` | `"chat-completion"` | `router.rs:304` |
| `AuditProxyStubEntry` | `"audit-proxy-stub"` | `http.rs:751` |
| `AuditProxyEntry` | `"audit-proxy"` | `http.rs:800/832` |
| `AuditCaptureEntry` | `"audit-capture"` | `http.rs:969`, graph proxy handlers |
| `ExtractionAuditEntry` | `"extract"` | `http.rs:586` |

### ExtractionAuditEntry missing fields (ledger.rs:286-309)
- `model: String` — the response carries it (`ExtractionResponse.model`) but is discarded before ledger write
- `cost_usd: f64` — `ComputeResponse.cost_usd` is discarded
- `sanitised_outbound: bool` — always true for extraction but absent
- A `completion_status` enum — the `(extraction_ok, deferred, defer_reason)` triple is non-queryable

### `"graph-query"` event type conflict (http.rs:1072)
Graph proxy handlers write `AuditCaptureEntry` with `event_type: "graph-query"` — but `/v1/audit/capture`'s `AUDIT_CAPTURE_VALID_EVENT_TYPES` does NOT include `"graph-query"`. The Doorman bypasses its own validation. Fix: add to allowed set.

---

## 4. Full HTTP Endpoint Map

| Route | Method | Tier path | Notes |
|---|---|---|---|
| `/healthz` | GET | none | Always 200 |
| `/readyz` | GET | none | Returns `{has_local, has_yoyo, has_external}` |
| `/v1/contract` | GET | none | Doorman+yoyo versions |
| `/v1/chat/completions` | POST | `route()` | OpenAI-compatible; honours `X-Foundry-*` headers |
| `/v1/brief` | POST | `route()` via ApprenticeshipDispatcher | 404 when apprenticeship disabled |
| `/v1/verdict` | POST | verdict pipeline direct | 404 when disabled |
| `/v1/shadow` | POST | enqueue only; drain worker async | 202 ACCEPTED; `actual_diff` always empty |
| `/v1/extract` | POST | `route_yoyo_only("trainer")` | Always HTTP 200; deferred shape on Yo-Yo down |
| `/v1/audit/proxy` | POST | `audit_proxy_client.relay()` | Two-entry ledger; 64 KiB cap |
| `/v1/audit/capture` | POST | direct ledger write | 16 KiB cap |
| `/v1/graph/query` | POST | forward to service-content | Audits as `"graph-query"` |
| `/v1/graph/mutate` | POST | forward to service-content | Audits as `"graph-mutation"` |

---

## 5. Model Drift Impact (now fixed)

`SLM_LOCAL_MODEL` is metadata only — does NOT affect routing decisions. It is passed to llama-server (ignored by llama-server) and recorded in the audit ledger.

The drift (`SLM_LOCAL_MODEL=OLMo-3-1125-7B-Think` while llama-server loads OLMo 2 1B) caused:
1. Every Tier A audit row had wrong `model` field
2. Sprint 0a shim would have returned wrong `"model"` to Claude Code in responses

**Fixed in this session**: `local-doorman.service` updated to `OLMo-2-0425-1B-Instruct-Q4_K_M.gguf`, deployed, restarted.

---

## 6. Sprint 0a Prerequisites

| # | Change | File | Status |
|---|---|---|---|
| 1 | Resolve apprenticeship-enabled drift (clone vs on-VM) | `compute/systemd/slm-doorman.service:37` vs on-VM | **Open** |
| 2 | Resolve Tier A model drift | Fixed in this session | **Done** |
| 3 | Add `graph_context_enabled: Option<bool>` to `ComputeRequest` | `slm-core/src/lib.rs:73`, `router.rs:125-158` | **Open — required** |
| 4 | Decide opus→Tier C path (allowlist label or fall back to Tier B for Sprint 0a) | `external.rs:78-82` | **Open — decision required** |
| 5 | Implement `/v1/messages` handler (~305 LOC) | `slm-doorman-server/src/http.rs` | **The Sprint 0a deliverable** |
| 6 | Update `/readyz` with `lark_validation_active` field | `http.rs:133-138` | Nice-to-have |

### The `graph_context_enabled` field (critical)

Without it, every `/v1/messages` request from Claude Code will have DataGraph entity rows injected as a system message (`router.rs:125-158`). A Claude Code session asking "summarise this file" will receive irrelevant Foundry entity context, degrading model quality. Sprint 0a cannot ship without this gate.

New field in `slm-core/src/lib.rs`:
```rust
pub graph_context_enabled: Option<bool>, // default true for backwards compat; shim sets false
```

Gate in `router.rs:125`: `if req.graph_context_enabled.unwrap_or(true) { ... }`.

### The opus→Tier C path (decision required before Sprint 0a)

Current `FOUNDRY_DEFAULT_ALLOWLIST` (`external.rs:78-82`) contains only:
- `"citation-grounding"`
- `"initial-graph-build"`
- `"entity-disambiguation"`

A Claude Code passthrough has none of these labels. Options:
- **Path A**: Add `"claude-code-passthrough"` to the allowlist (compile-time change), wire Tier C env vars, route opus to Tier C
- **Path B**: Route `claude-opus-*` to Tier B "trainer" in Sprint 0a with a note in deploy docs

Path B is lower risk for Sprint 0a; Path A can land in Sprint 0b.

### Anthropic Messages API conversion (Sprint 0a handler shape)

```
AnthropicMessagesBody → ComputeRequest
  system prompt → ChatMessage{role:"system"} prepended to messages
  model "claude-haiku-*" → Complexity::Low, yoyo_label=None
  model "claude-sonnet-*" → Complexity::High, yoyo_label=Some("trainer")
  model "claude-opus-*" → see opus path decision above
  graph_context_enabled: Some(false) ← critical
```

---

## 7. Key Defects (Prioritised)

| # | Defect | File:line | Severity |
|---|---|---|---|
| 1 | Apprenticeship flag drift (clone says `false`, on-VM says `true`, CLAUDE.md says "unset") | `compute/systemd/slm-doorman.service:37` | High |
| 2 | `ExtractionAuditEntry` missing `model`, `cost_usd`, `sanitised_outbound` | `ledger.rs:286-309`, `http.rs:573-585` | Medium |
| 3 | Graph context injection will corrupt Sprint 0a Claude Code traffic | `router.rs:125-158` | High (blocks Sprint 0a) |
| 4 | Tier C allowlist incompatible with opus → Tier C routing | `external.rs:78-82` | Medium (blocks Path A) |
| 5 | `route_yoyo_only()` has no audit write — footgun for future callers | `router.rs:359-394` | Low |
| 6 | Parse-failure `defer_reason: "yoyo-transient"` is semantically wrong — should be `ModelOutputMalformed` | `http.rs:544-552` | Low |
| 7 | `"graph-query"` not in `AUDIT_CAPTURE_VALID_EVENT_TYPES` but written by graph proxy | `http.rs:1072`, `:859-865` | Medium |
| 8 | `route_async()` is dead code — orchestrator never set in main.rs | `router.rs:75-87`, `main.rs` | Low |

---

## 8. "Never Generate Text" Rule — Clean

Every handler was audited. All text content returned to callers is upstream model output passed through. Error messages are protocol scaffolding. The apprenticeship system prompt is a prompt sent TO the upstream model, not returned to the caller. **No violations.**

The Sprint 0a shim (`compute_to_anthropic_response`) will wrap upstream content in Anthropic SSE frames — protocol scaffolding, not generated content. Compliant by design.
