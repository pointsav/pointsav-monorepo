---
schema: foundry-doc-v1
document_version: 0.2.0
research_done_count: 0
research_suggested_count: 0
open_questions_count: 0
research_provenance: tacit
research_inline: false
cites: []
---

# Audit Endpoints Contract — v0.2.0

Wire contract for `POST /v1/audit/proxy` and `POST /v1/audit/capture`.
Audience: project-language Task (A-4 editorial gateway adapter loading
via Doorman audit-mediated Tier C); project-data Task (A-5 anchor-emitter
audit-ledger module-id); project-bim (service-codes consumer); any future
cross-cluster consumer that needs to reach the Doorman's audit surface.

**Stability:** v0.2.0. PATCH per clarification / non-breaking field
addition; MINOR per new endpoints, new event_types, or new error
variants; MAJOR per breaking changes (renamed fields, removed endpoints,
status code changes). Versioning follows workspace `CLAUDE.md` §7.

**Layer constraint:** these endpoints live on the `slm-doorman-server`
binary (`service-slm/crates/slm-doorman-server/`). All cross-cluster
callers reach them over HTTP. No caller holds provider API keys; the
Doorman holds them. The endpoints are the sole entry point to
provider-authenticated external calls from inside the ring-2 and ring-1
clusters.

---

## 1. POST /v1/audit/proxy

Doorman-mediated relay to an external provider (Anthropic, Gemini,
OpenAI). The caller submits a structured request; the Doorman
authenticates with the provider, writes two audit ledger entries, and
returns the provider's response.

**Primary consumer:** project-language Task A-4 (editorial gateway
adapter loading — uses `purpose: "editorial-refinement"` or
`"citation-grounding"`).

### 1.1 Request

```
POST /v1/audit/proxy
Content-Type: application/json
```

Body shape (`AuditProxyRequest` in `slm-core::lib.rs`):

| Field | Type | Required | Notes |
|---|---|---|---|
| `module_id` | string | yes | Validated as `ModuleId` — `[a-z0-9-]`, 1–64 chars |
| `purpose` | string | yes | Must be non-empty AND in the purpose allowlist |
| `provider` | string | yes | One of `"anthropic"`, `"gemini"`, `"openai"` (case-insensitive) |
| `model` | string | yes | Provider model identifier, e.g. `"claude-opus-4-7"` |
| `messages` | array | yes | Non-empty; each element: `{"role": string, "content": string}` |
| `max_tokens` | u32 | no | Passed to provider; omitted from wire when absent |
| `temperature` | f32 | no | Passed to provider; omitted from wire when absent |
| `caller_request_id` | string | no | Caller correlation ID; echoed in response and ledger |

Example:

```json
{
  "module_id": "woodfine",
  "purpose": "citation-grounding",
  "provider": "anthropic",
  "model": "claude-opus-4-7",
  "messages": [
    {"role": "user", "content": "Verify this citation: ..."}
  ],
  "max_tokens": 256,
  "caller_request_id": "project-language-req-7f3a"
}
```

### 1.2 Response (success)

```
200 OK
Content-Type: application/json
```

Body shape (`AuditProxyResponse` in `slm-core::lib.rs`):

| Field | Type | Notes |
|---|---|---|
| `audit_id` | string | Doorman-generated UUIDv7; matches the ledger entries |
| `caller_request_id` | string (optional) | Echoed from the request |
| `content` | string | Provider's reply text |
| `usage.prompt_tokens` | u32 | Prompt token count from provider |
| `usage.completion_tokens` | u32 | Completion token count from provider |
| `usage.cost_usd` | f64 | Cost computed from per-provider pricing config |

Example:

```json
{
  "audit_id": "01930a3f-...",
  "caller_request_id": "project-language-req-7f3a",
  "content": "The citation is accurate per ...",
  "usage": {
    "prompt_tokens": 120,
    "completion_tokens": 58,
    "cost_usd": 0.0000375
  }
}
```

### 1.3 Validation order

The handler validates in this order. Each failure is returned
immediately without proceeding to the next step:

1. Parse `module_id` as `ModuleId`; reject 400 on failure.
2. Validate `provider` against `{anthropic, gemini, openai}`; reject 400.
3. Check `purpose` is non-empty; reject 400.
4. Check `purpose` is in the purpose allowlist; reject 403.
5. Check `messages` is non-empty; reject 400.
6. Generate `audit_id` (UUIDv7); write stub ledger entry.
7. If `audit_proxy_client` is `None` (no providers configured), return
   503 with audit_id and "unconfigured" message. Stub entry is still
   written at this step — the paper trail exists regardless.
8. Call relay client; write final ledger entry; return 200 or error.

### 1.4 HTTP status codes

| Code | Condition |
|---|---|
| `200 OK` | Upstream relay succeeded; body is `AuditProxyResponse` |
| `400 BAD_REQUEST` | Invalid `module_id`, unknown `provider`, empty `purpose`, or empty `messages` |
| `403 FORBIDDEN` | `purpose` not in the purpose allowlist (caller-side policy violation) |
| `503 SERVICE_UNAVAILABLE` | No providers configured at Doorman startup (server-side configuration gap), OR upstream returned 5xx |
| `502 BAD_GATEWAY` | Network or parse error from upstream provider |

### 1.5 Purpose allowlist

The default allowlist (`FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST` in
`slm-doorman/src/audit_proxy.rs`) contains four entries:

| Purpose | Consumer |
|---|---|
| `"editorial-refinement"` | project-language gateway refining drafts |
| `"citation-grounding"` | verifying citations against external sources |
| `"entity-disambiguation"` | resolving named entities |
| `"initial-graph-build"` | bootstrapping a fresh service-content graph |

An unenumerated purpose is rejected 403 FORBIDDEN. An empty
allowlist (`AuditProxyPurposeAllowlist::EMPTY`) fails closed: all
purposes are denied. This is the correct posture for deployments
that have not opted into audit_proxy calls.

Operators extend the allowlist by editing
`FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST` in
`slm-doorman/src/audit_proxy.rs` and recompiling. Runtime extension
is not supported; extensions must be visible in code review per
`conventions/llm-substrate-decision.md`.

### 1.6 Two-entry ledger design

Every inbound request that passes validation (through step 5 above)
writes two JSONL entries to the same daily ledger file:

**Entry 1 — stub** (`AuditProxyStubEntry`): written before the
upstream call. Fields include `audit_id`, `inbound_at`, `module_id`,
`purpose`, `provider`, `model`, `caller_request_id`,
`request_messages_count`, and `status: "inbound"`. Guarantees a paper
trail even if the process crashes during the upstream call.

**Entry 2 — final** (`AuditProxyEntry`): written after the upstream
call returns (success or error). Fields include all stub fields plus
`prompt_tokens`, `completion_tokens`, `cost_usd`, `latency_ms`,
`status` (`"ok"` or `"upstream-error"`), and an optional
`error_message`. Both entries share the same `audit_id`, enabling
correlation.

When the relay is unconfigured (step 7 above), only the stub entry is
written.

---

## 2. POST /v1/audit/capture

Inverse direction of audit_proxy: cross-cluster callers push audit
events for work performed locally, without routing through the Doorman.
The Doorman validates, writes a single ledger entry, and returns 200.

**Primary consumers:**
- project-data Task A-5 (anchor-emitter, `event_type: "anchor-event"`)
- project-language Task A-4 (editorial gateway local prose-edit pass,
  `event_type: "prose-edit"`)
- Any Ring 1/2/3 producer that does work the central audit trail should
  record but that did not route through the Doorman.

### 2.1 Request

```
POST /v1/audit/capture
Content-Type: application/json
```

Body shape (`AuditCaptureRequest` in `slm-core::lib.rs`):

| Field | Type | Required | Notes |
|---|---|---|---|
| `audit_id` | string | yes | Caller-generated UUID (UUIDv7 recommended; any UUID accepted). Caller is the source of truth — the work happened locally |
| `module_id` | string | yes | Validated as `ModuleId` — `[a-z0-9-]`, 1–64 chars |
| `event_type` | string | yes | One of the five accepted values (see §2.3) |
| `source` | string | yes | Caller component/cluster identifier, e.g. `"project-language"` or `"project-data:anchor-emitter"`. Must be non-empty |
| `status` | string | yes | Status of the work: `"ok"`, `"policy-denied"`, `"upstream-error"`, or other. Must be non-empty |
| `event_at` | string | yes | RFC 3339 / ISO 8601 timestamp of when the work occurred (caller's clock). Example: `"2026-04-28T14:23:00Z"` |
| `payload` | JSON object | yes | Event-specific payload (untyped). Must be a JSON object; size capped at 16 KiB (see §2.4) |
| `caller_request_id` | string | no | Caller correlation ID; echoed in response and ledger |

Example (project-language editorial gateway):

```json
{
  "audit_id": "01930b12-...",
  "module_id": "woodfine",
  "event_type": "prose-edit",
  "source": "project-language",
  "status": "ok",
  "event_at": "2026-04-28T14:23:00Z",
  "payload": {
    "draft_id": "topic-doorman-protocol",
    "edit_pass": "structural-register",
    "word_delta": 42
  },
  "caller_request_id": "lang-edit-0091"
}
```

Example (project-data anchor-emitter):

```json
{
  "audit_id": "01930c44-...",
  "module_id": "foundry",
  "event_type": "anchor-event",
  "source": "project-data:anchor-emitter",
  "status": "ok",
  "event_at": "2026-04-28T02:00:07Z",
  "payload": {
    "batch_size": 128,
    "tree_root": "sha256:abc123...",
    "rekor_log_id": "..."
  }
}
```

### 2.2 Response (success)

```
200 OK
Content-Type: application/json
```

Body shape (`AuditCaptureResponse` in `slm-core::lib.rs`):

| Field | Type | Notes |
|---|---|---|
| `audit_id` | string | Echoed from the request — confirms the Doorman accepted and wrote |
| `caller_request_id` | string (optional) | Echoed from the request |
| `status` | string | Always `"captured"` on 200 |

Example:

```json
{
  "audit_id": "01930b12-...",
  "caller_request_id": "lang-edit-0091",
  "status": "captured"
}
```

### 2.3 Accepted event_types

Five values are accepted. The wire contract treats `event_type` as a
string with a documented vocabulary; future MINOR versions may extend
the vocabulary by adding entries to
`AUDIT_CAPTURE_VALID_EVENT_TYPES` in
`slm-doorman-server/src/http.rs`.

| `event_type` | Primary producer | Notes |
|---|---|---|
| `"prose-edit"` | project-language editorial gateway | Draft refinement pass via service-language |
| `"design-edit"` | project-design Task | UI component or token edit |
| `"graph-mutation"` | service-content | Knowledge-graph add/update/delete |
| `"anchor-event"` | project-data anchor-emitter | Sigstore Rekor anchoring batch |
| `"verdict-issued"` | Any Ring 3 senior | Apprenticeship-corpus verdict committed |

### 2.4 Validation order

1. Parse `module_id` as `ModuleId`; reject 400 on failure.
2. Validate `event_type` against the five accepted values; reject 400.
3. Validate `source` is non-empty; reject 400.
4. Validate `status` is non-empty; reject 400.
5. Parse `event_at` as RFC 3339; reject 400 on failure.
6. Check payload serialised size ≤ 16 384 bytes (16 KiB); reject 413.
7. Write one `AuditCaptureEntry` to the ledger; return 200.

### 2.5 HTTP status codes

| Code | Condition |
|---|---|
| `200 OK` | Entry accepted and written; body is `AuditCaptureResponse` |
| `400 BAD_REQUEST` | Invalid `module_id`, unknown `event_type`, empty `source`, empty `status`, or unparseable `event_at` |
| `413 PAYLOAD_TOO_LARGE` | `payload` exceeds 16 KiB |
| `500 INTERNAL_SERVER_ERROR` | Ledger write failure |

### 2.6 Single-entry ledger design

Unlike audit_proxy, capture writes exactly one `AuditCaptureEntry` to
the ledger. There is no two-phase commit: the work already happened
locally; the Doorman is a downstream recorder, not an orchestrator.

The entry preserves two timestamps:
- `event_at` — the caller's clock at the time the local work occurred.
- `captured_at` — the Doorman's clock at receipt time.

Both are preserved in the ledger so downstream analysis can detect
clock skew between clusters.

---

## 3. Audit ledger format

The Doorman writes all entries to per-day JSONL files at
`<ledger_base_dir>/<YYYY-MM-DD>.jsonl`. The default base directory is
`$HOME/.service-slm/audit/`; the `SLM_AUDIT_DIR` environment variable
overrides it (see `infrastructure/local-doorman/` for the systemd unit
default).

Each line is one JSON object. Lines from different entry types
interleave in the same file.

### 3.1 Entry types

The JSONL stream contains four entry types. As of contract v0.2.0, all
four carry an explicit `entry_type: string` discriminator field. The
canonical strings are:

| Entry type | `entry_type` value |
|---|---|
| `AuditEntry` (chat-completion routing) | `"chat-completion"` |
| `AuditProxyStubEntry` (proxy inbound stub) | `"audit-proxy-stub"` |
| `AuditProxyEntry` (proxy final outcome) | `"audit-proxy"` |
| `AuditCaptureEntry` (local-work capture) | `"audit-capture"` |

The field is set by `AuditLedger::append_*` at write time regardless of
what the caller placed in the struct. Cross-cluster consumers SHOULD use
the `entry_type` field as the primary discriminator (see §3.2).

**AuditEntry** (`entry_type: "chat-completion"`) — chat-completion
routed via `POST /v1/chat/completions`. Key fields: `entry_type`,
`timestamp_utc`, `request_id`, `module_id`, `tier`, `model`,
`inference_ms`, `cost_usd`, `sanitised_outbound`, `completion_status`.

**AuditProxyStubEntry** (`entry_type: "audit-proxy-stub"`) — inbound
stub for `POST /v1/audit/proxy`, written before the upstream call. Key
fields: `entry_type`, `audit_id`, `inbound_at`, `module_id`, `purpose`,
`provider`, `model`, `caller_request_id`, `request_messages_count`,
`status` (value: `"inbound"`).

**AuditProxyEntry** (`entry_type: "audit-proxy"`) — final outcome for
`POST /v1/audit/proxy`, written after the upstream call. Key fields:
`entry_type`, `audit_id`, `completed_at`, `module_id`, `purpose`,
`provider`, `model`, `caller_request_id`, `prompt_tokens`,
`completion_tokens`, `cost_usd`, `latency_ms`, `status` (`"ok"` or
`"upstream-error"`), `error_message` (optional).

**AuditCaptureEntry** (`entry_type: "audit-capture"`) — local-work
event pushed via `POST /v1/audit/capture`. Key fields: `entry_type`,
`audit_id`, `module_id`, `event_type` (capture vocabulary, e.g.
`"prose-edit"`), `source`, `status`, `event_at`, `captured_at`,
`payload`, `caller_request_id` (optional).

Note: `AuditCaptureEntry` has two string fields whose names are similar
but distinct. `entry_type` (`"audit-capture"`) identifies the JSONL
entry kind; `event_type` (e.g. `"prose-edit"`) identifies the captured
local-work kind and comes from the caller's request.

### 3.2 Distinguishing entry types

#### Canonical path (contract v0.2.0 and later): explicit `entry_type` tag

All four entry types carry an `entry_type: string` field. Read the
`entry_type` field to determine entry kind:

| `entry_type` value | Entry type |
|---|---|
| `"chat-completion"` | `AuditEntry` |
| `"audit-proxy-stub"` | `AuditProxyStubEntry` |
| `"audit-proxy"` | `AuditProxyEntry` (final outcome) |
| `"audit-capture"` | `AuditCaptureEntry` |

**Cross-cluster consumers (project-language A-4, project-data A-5,
project-bim service-codes) SHOULD use `entry_type` as the primary
discriminator.** It is a single field read without combinatorial
logic, stable across PATCH versions, and present in every entry
written by code at or after contract v0.2.0.

**Executable form:** `audit_endpoints_integration.rs` test
`entry_type_tag_discriminates_all_entry_kinds` verifies that all four
entry kinds carry the correct `entry_type` value when written via
`AuditLedger::append_*`.

#### Fallback path: field-presence discrimination (for entries predating v0.2.0)

Entries written by code predating contract v0.2.0 lack the `entry_type`
field. Consumers reading historical JSONL files MUST support field-presence
discrimination as a fallback:

| Discriminating field(s) | Entry type |
|---|---|
| `event_type` field present | `AuditCaptureEntry` |
| `provider` field present AND `status == "inbound"` | `AuditProxyStubEntry` |
| `provider` field present AND `prompt_tokens` field present | `AuditProxyEntry` (final) |
| None of the above | `AuditEntry` (chat-completion) |

Apply the checks in order; the first match wins.

**Executable form:** `audit_endpoints_integration.rs` test
`mixed_entry_types_in_jsonl_stream_distinguishable_by_field_presence`
implements this fallback logic as a passing test. The test still passes
on v0.2.0+ entries: both algorithms produce the same answer when
`entry_type` is present.

#### Recommended consumer implementation

```
if entry.entry_type is present:
    identify by entry.entry_type (canonical strings above)
else:
    identify by field-presence (fallback algorithm above)
```

This two-branch approach handles both old and new entries in a single
reader without requiring a schema migration.

### 3.3 Correlation

Two entries for the same `audit_proxy` call share the same `audit_id`
value. Correlate the stub (presence check: `status == "inbound"` +
`provider` present) with the final entry (`prompt_tokens` present) by
joining on `audit_id`. The stub is always written before the final
entry; timestamps (`inbound_at` / `completed_at`) preserve ordering.

`AuditCaptureEntry` records carry a caller-generated `audit_id`; the
Doorman does not generate it. Cross-system correlation uses the shared
`audit_id` plus `module_id` as the join key.

---

## 4. Error handling

| `DoormanError` variant | HTTP code | Retry-able | Fix |
|---|---|---|---|
| `AuditProxyInvalidProvider { provider }` | 400 | No | Caller: use `anthropic`, `gemini`, or `openai` |
| `AuditProxyPurposeNotAllowlisted { purpose }` | 403 | No | Caller: use a documented purpose from §1.5; operator may extend allowlist |
| `AuditProxyProviderUnavailable { provider }` | 503 | No | Operator: set `SLM_TIER_C_{PROVIDER}_ENDPOINT` + `SLM_TIER_C_{PROVIDER}_API_KEY` at Doorman startup |
| `UpstreamShape(msg)` | 502 | Maybe | Upstream provider error; check message; retry with exponential backoff |
| `AuditCaptureUnknownEventType { event_type }` | 400 | No | Caller: use one of the five documented event_types (§2.3) |
| `AuditCapturePayloadTooLarge { size_bytes, max_bytes }` | 413 | No | Caller: reduce payload to ≤ 16 KiB |
| `AuditCaptureInvalidTimestamp { value }` | 400 | No | Caller: use RFC 3339 format, e.g. `"2026-04-28T14:23:00Z"` |
| `LedgerIo(_)` | 500 | Maybe | Operator: check Doorman disk space / permissions; ledger write failed |
| `LedgerSerde(_)` | 500 | No | Implementation error; open a defect |

Error response body shape (all codes):

```json
{
  "error": {
    "message": "<human-readable description>"
  }
}
```

---

## 5. Versioning and stability

This contract carries version 0.2.0 per workspace `CLAUDE.md` §7:

```
PATCH  +1 per clarification / non-breaking field addition
MINOR  +1 per new endpoint, new event_type, new error variant, or new
         optional field with semantic effect
MAJOR  +1 per breaking change: renamed fields, removed endpoints,
         changed status codes, changed validation order
```

The contract document lives at
`service-slm/docs/audit-endpoints-contract.md` (cluster-internal;
not a wiki TOPIC). Cross-cluster consumers (project-language A-4,
project-data A-5, project-bim service-codes) pin to the MAJOR version
and adapt to MINOR/PATCH changes at their next sprint boundary.

### Version history

**0.2.0** — 2026-04-28. Added explicit `entry_type: string` discriminator
field to all four entry types (`AuditEntry`, `AuditProxyStubEntry`,
`AuditProxyEntry`, `AuditCaptureEntry`). Canonical kebab-case strings:
`"chat-completion"`, `"audit-proxy-stub"`, `"audit-proxy"`,
`"audit-capture"`. Field uses `#[serde(default)]` for backwards
compatibility — old JSONL entries lacking the field still deserialise
correctly. `AuditLedger::append_*` methods force the canonical value at
write time. §3.1 updated with the canonical string table; §3.2 rewritten
with explicit-tag as canonical path and field-presence as fallback.
Closes the future-direction note from v0.1.0 §3.2. Consumer rationale:
project-language A-4, project-data A-5, project-bim service-codes can
now identify entry kind from a single field read.

**0.1.0** — 2026-04-28. First publication, PS.4 step 5. Established the
two-endpoint contract (`audit_proxy` + `audit_capture`), two-entry ledger
design for proxy, single-entry ledger design for capture, purpose
allowlist, event_type vocabulary, and field-presence discrimination
algorithm as the initial discrimination mechanism.
