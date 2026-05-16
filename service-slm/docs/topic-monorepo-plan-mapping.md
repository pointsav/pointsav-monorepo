---
schema: foundry-doc-v1
document_version: 1.0.0
research_provenance: direct-consultation
research_inline: false
authored: 2026-05-16
authored_by: project-intelligence Totebox
authored_with: claude-sonnet-4-6
cites: []
---

# TOPIC: Sovereign Routing Plan — Monorepo Folder Mapping

This document maps every item in the sovereign routing plan to its physical location in
the pointsav-monorepo. It is the single reference for "where does this work live?"

---

## NOW — Immediate Sprint Work (service-* and app-*)

### service-slm — All infrastructure sprints

```
service-slm/
├── crates/
│   ├── slm-core/src/lib.rs
│   │   └── Sprint 1: CanonicalMessage + ContentBlock types (replaces ChatMessage)
│   │       Enables: tool_use/tool_result/thinking preservation end-to-end
│   │
│   ├── slm-doorman/src/
│   │   ├── tier/yoyo.rs
│   │   │   └── Sprint 0b: real SSE token streaming (reqwest bytes_stream)
│   │   ├── tier/external.rs
│   │   │   └── Sprint 2: native Anthropic Messages API outbound (replaces OpenAI shim)
│   │   └── router.rs
│   │       └── Sprint 0b: on-demand Yo-Yo lazy-start (SLM_YOYO_AUTO_START gate)
│   │
│   ├── slm-doorman-server/src/http.rs
│   │   ├── Line 1214: POST /v1/messages — Sprint 0a IMPLEMENTED
│   │   ├── Sprint 0b: SSE response handler + shadow capture gate
│   │   └── Sprint 2: POST /v1/responses (OpenAI Responses API inbound)
│   │
│   └── slm-mcp-server/          ← NEW CRATE (Sprint 3)
│       Create: crates/slm-mcp-server/
│       Tools: foundry:query-datagraph, foundry:get-corpus-stats,
│              foundry:submit-extraction, foundry:doorman-health
│       Wire format: JSON-RPC 2.0 over stdio (MCP spec 2025-11-25)
│       Rust crate: rmcp (modelcontextprotocol/rust-sdk)
│       Claude Code config: .mcp.json at project root
│
└── docs/                         ← TOPIC-* and GUIDE-* files live here
    ├── topic-claude-code-sovereign-routing.md      (NEW — 2026-05-16)
    ├── topic-anthropic-gateway-integration.md      (NEW — 2026-05-16)
    ├── topic-tos-training-constraints.md           (NEW — 2026-05-16)
    ├── guide-activate-anthropic-shim.md            (NEW — 2026-05-16)
    ├── guide-post-commit-training-hook.md          (NEW — 2026-05-16)
    ├── topic-leapfrog-architecture.md              (existing — update Sprint 0a status)
    └── yoyo-training-substrate-*.md                (existing)
```

### service-content — Training data source

```
service-content/
└── No new crates required for Sprint 0a–3.
    Existing change: extraction tuple capture (graph-validated rows only)
    → feeds moonshot-slm extraction-lora corpus
    → Tier C removed from extraction path (already done: POST /v1/extract, route_yoyo_only)
```

### app-console-slm — Sprint 4a operator TUI (NEW CRATE)

```
app-console-slm/               ← NEW (created 2026-05-16, placeholder)
├── Cargo.toml
├── README.md
├── README.es.md
└── src/main.rs
    Sprint 4a (1–2 weeks after Sprint 3):
      console-slm status   — one-line Doorman/Tier/DataGraph/Corpus health
      console-slm admin    — ratatui TUI: Doorman, Yo-Yo, Jobs, Corpus, DataGraph panels
    Sprint 4b (DEFERRED):
      console-slm code     — agentic coding loop (deferred: team ≥3 devs + model quality gate)
      console-slm chat     — REPL (deferred)
```

---

## LONG-TERM — Moonshot Work (moonshot-*)

### moonshot-slm — Sovereign model training pipeline (NEW MOONSHOT)

```
moonshot-slm/                  ← NEW (created 2026-05-16, placeholder)
├── Cargo.toml
├── README.md
├── README.es.md
└── src/main.rs

What this moonshot builds:
  - Automated DPO/SFT pipeline (Unsloth + TRL on OLMo base, Apache 2.0)
  - coding-lora adapter: trained from service-slm /v1/shadow tuples (Tier A/B sessions)
  - extraction-lora adapter: trained from service-content graph-validated extraction rows
  - SLSA-attested LoRA adapter lifecycle: train → evaluate → promote → deploy
  - Evaluation harness: 100-pair held-out corpus; reject on >5% regression
  - Continual learning: 20–30% replay buffer + DPO KL anchor (beta=0.1)
  - Long-term objective: eliminate Anthropic Tier C dependency for Tier B tasks

What replaces:
  External AI API inference dependency (Anthropic Tier C for Tier B-quality tasks)

Implementation trigger:
  ≥1,000 DPO training tuples in service-slm apprenticeship corpus
  (~6–8 weeks after Sprint 0b activation at normal development pace)

Depends on:
  moonshot-gpu (GPU substrate for training runs)
  service-slm (inference gateway + /v1/shadow training data source)
  service-content (extraction-lora training data source)
```

### moonshot-protocol — A2A Agent Mesh (Sprint 5, fits existing moonshot)

```
moonshot-protocol/             ← EXISTING moonshot, extend for Sprint 5

Sprint 5 addition:
  A2A agent card for the Foundry Doorman
  Endpoints: GET /a2a/agent-card, POST /a2a/tasks/send
  Agent card declares: text-generation, code-generation, entity-extraction capabilities
  Governance fields: audit_trail=true, sovereignty=on-premise, compliance=[ISO-42001, NI-51-102]
  Positions Foundry as a sovereign node in the Linux Foundation A2A agent mesh
  (150+ orgs, Azure/AWS/Google/Anthropic all committed to A2A protocol)

This extends moonshot-protocol because A2A is a protocol-level specification,
not a service-layer concern.
```

---

## Sprint Sequence → Folder Map

| Sprint | Duration | Primary folder | State |
|---|---|---|---|
| 0a | Done | `service-slm/crates/slm-doorman-server/src/http.rs` | IMPLEMENTED |
| 0b | 3–4 days | `slm-doorman/src/tier/yoyo.rs` + `http.rs` + `router.rs` | Planned |
| 1 | 1 week | `slm-core/src/lib.rs` + outbound adapters in all tier files | Planned |
| 2 | 1 week | `slm-doorman/src/tier/external.rs` + `http.rs` | Planned |
| 3 | 2 weeks | NEW `service-slm/crates/slm-mcp-server/` | Planned |
| 4a | 1–2 weeks | NEW `app-console-slm/` | Planned |
| 4b | 6–8 weeks | `app-console-slm/` (coding agent loop) | DEFERRED |
| 5 | 4 weeks | `moonshot-protocol/` (A2A agent card) | Planned |
| — | Ongoing | NEW `moonshot-slm/` | Research/placeholder |

---

## TOPIC-* and GUIDE-* Index

All service-slm documentation lives in `service-slm/docs/`:

| File | Type | Purpose |
|---|---|---|
| `topic-claude-code-sovereign-routing.md` | TOPIC | Decision rationale, sprint sequence, credential architecture |
| `topic-anthropic-gateway-integration.md` | TOPIC | ANTHROPIC_BASE_URL mechanics, settings schema, SSE requirements |
| `topic-tos-training-constraints.md` | TOPIC | Anthropic ToS analysis — permitted/prohibited training sources |
| `topic-monorepo-plan-mapping.md` | TOPIC | This document — folder map for the entire plan |
| `guide-activate-anthropic-shim.md` | GUIDE | Step-by-step Sprint 0a activation runbook |
| `guide-post-commit-training-hook.md` | GUIDE | Training hook installation, gate design, LoRA schedule |
| `topic-leapfrog-architecture.md` | TOPIC | Multi-Yo-Yo pipeline (existing; update Sprint 0a status) |

---

## What Does Not Belong in the Monorepo

Planning documents, research transcripts, and session-state files stay in the Totebox
archive's `.agent/plans/` directory and are not promoted to the monorepo:

- `.agent/plans/sovereign-routing-comprehensive.md` — master sprint plan (Totebox only)
- `.agent/plans/universal-ai-gateway.md` — sprint-level implementation detail (Totebox only)
- `~/.claude/plans/sovereign-coding-agent-leapfrog-2030.md` — product strategy (user-level)
- `~/.claude/plans/wire-format-leapfrog-2030.md` — protocol strategy (user-level)

The monorepo receives: doctrine (TOPIC-*), runbooks (GUIDE-*), and code. Plans stay in
the Totebox archive until they are promoted to committed code or documentation.
