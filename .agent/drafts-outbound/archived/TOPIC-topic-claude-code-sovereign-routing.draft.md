---
schema: foundry-draft-v1
state: draft
language_protocol: PROSE-TOPIC
originating_cluster: project-intelligence
target_repo: vendor/content-wiki-documentation
target_path: topic-claude-code-sovereign-routing.md
audience: internal
bcsc_class: no-disclosure-implication
authored: 2026-05-29
authored_by: project-intelligence Totebox (claude-sonnet-4-6, Sprint 1 closeout)
authored_with: claude-sonnet-4-6
research_done_count: 1
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  Source: service-slm/docs/topic-claude-code-sovereign-routing.md (original research session 2026-05-16)
research_inline: true
notes_for_editor: |
  Claude Code as sovereign coding agent via Doorman gateway — decision rationale, ToS constraints.
  Bloomberg-register pass required. Bilingual ES sibling required for TOPIC artifacts.
  Remove any internal crate paths or infrastructure-specific detail before publication.
  Verify all code references match HEAD (Sprint 1 updated several line numbers).
---

# TOPIC: Claude Code as Sovereign Coding Agent via Doorman Gateway

## 1. Decision

Claude Code — configured with `ANTHROPIC_BASE_URL` pointing at the local Doorman — is the
sovereign coding agent surface. We do not build a competing agentic coding loop.

All strategic value in the sovereign routing plan resides in the Doorman gateway, not in the
client. Tier routing, cost reduction, training data capture, and audit logging are Doorman
responsibilities. The client is a caller; its identity does not affect these properties.

Claude Code has: context compaction, tool sandbox with permission model, hooks system,
MCP client, sub-agents (Task tool), IDE bridges, slash command system, /review, /schedule,
auto-memory, session management, and project CLAUDE.md hierarchy. Reaching 95% parity on
agent-loop quality alone is twelve or more months of sustained engineering work. Anthropic
has a research moat on this layer that it is not productive to race.

## 2. The Three-Tier Routing Model

```
Claude Code request
    │
    ├─ model: claude-haiku-*   → Tier A  local OLMo 2 1B  (:8080)   $0
    ├─ model: claude-sonnet-*  → Tier B  Yo-Yo OLMo 3 32B (:9443)  ~$0.40/hr
    └─ model: claude-opus-*    → Tier C  Anthropic API     (external) pay/token
```

The routing decision is made by the Doorman based on the model name field in the
Anthropic Messages API request body. No change to Claude Code configuration beyond
`ANTHROPIC_BASE_URL` is required to activate tier routing.

## 3. Sprint Sequence

| Sprint | Scope | State |
|---|---|---|
| 0a | Anthropic Messages API shim (`POST /v1/messages` in http.rs:1214) | IMPLEMENTED |
| 0b | Real SSE streaming + on-demand Yo-Yo lazy-start + training capture gate | Planned |
| 1  | Canonical IR (`CanonicalMessage` + `ContentBlock` in slm-core) — unlocks tool-use | Planned |
| 2  | Native Tier C Anthropic outbound + OpenAI Responses API inbound | Planned |
| 3  | `slm-mcp-server` — Foundry tools as MCP for any MCP client | Planned |
| 4a | `app-console-slm` status + admin TUI only | Planned |
| 4b | `app-console-slm` coding agent loop | Deferred |
| 5  | A2A agent card | Planned |

Sprint 4b conditions (all required): team ≥3 active developers; local model competitive
with Sonnet on Foundry-specific tasks; product distribution motive exists.

## 4. Sprint 0a Constraint — Tool-Use Breaks Until Sprint 1

Sprint 0a flattens `tool_use` and `tool_result` content blocks to plain text strings.
Claude Code's agentic loop (tool_use → dispatch tool → tool_result → re-POST) requires
structured content blocks to be preserved through the round-trip. Until Sprint 1 (Canonical
IR) ships, the shim is safe only for:
- Chat and simple Q&A
- Single-turn requests (no multi-step tool chains)
- Moderate code edits where Claude Code does not invoke tools iteratively

Do not set `ANTHROPIC_BASE_URL` globally for coding sessions until Sprint 1 is merged.
Set it per-project in `.claude/settings.json` for controlled testing only.

## 5. Credential Architecture

| Credential | Location | Used for |
|---|---|---|
| `ANTHROPIC_API_KEY` | Doorman server env (`/etc/local-doorman/local-doorman.env`) | Tier C outbound to Anthropic |
| `SLM_GATEWAY_TOKEN` | Doorman server env | Validates inbound shim requests |
| `ANTHROPIC_AUTH_TOKEN` | Claude Code client env or `.claude/settings.json` | Must equal `SLM_GATEWAY_TOKEN` — validated by shim only, never forwarded upstream |

The Claude Pro/Max subscription OAuth token is bound to direct connections to
`api.anthropic.com`. When `ANTHROPIC_BASE_URL` is set, Max billing does not apply.
Tier C passthrough bills to the Commercial API key. Max subscription applies only to
direct Claude Code sessions where `ANTHROPIC_BASE_URL` is absent.

## 6. Per-Project Activation (`.claude/settings.json`)

```json
{
  "env": {
    "ANTHROPIC_BASE_URL": "http://127.0.0.1:9080",
    "ANTHROPIC_AUTH_TOKEN": "<gateway-token>",
    "ANTHROPIC_SMALL_FAST_MODEL": "claude-haiku-4-5-20251001"
  }
}
```

Settings hierarchy (later overrides earlier):
1. `~/.claude/settings.json` — user/global
2. `<project>/.claude/settings.json` — project (commit-tracked)
3. `<project>/.claude/settings.local.json` — project-local (gitignored)
4. Process environment variables — highest priority

Use `.claude/settings.local.json` (gitignored) for development-only activation.
Commit `.claude/settings.json` only when the project team has agreed to route
all sessions through the gateway.

## 7. Sprint 4a — Admin TUI Only

`app-console-slm` Sprint 4a builds the one capability Claude Code does not provide:
a single-pane-of-glass operator TUI for Foundry infrastructure.

```
console-slm status   # one-line health check
console-slm admin    # ratatui TUI: Doorman / Yo-Yo / Corpus / DataGraph panels
```

This replaces: manual curl to /readyz, start-yoyo.sh, SSH to Yo-Yo VM for nvidia-smi,
corpus-stats.sh, manual datagraph-health.json reads.

The coding agent loop (console-slm code) is Sprint 4b and is deferred indefinitely
at current team scale.

## 8. The Compound Flywheel

```
Claude Code session → ANTHROPIC_BASE_URL → Doorman
    Tier A or Tier B handles request
    post-commit hook → /v1/shadow (brief + diff)
    Doorman stores DPO tuple (Tier A/B outputs only — Tier C excluded per ToS)
    Weekly: Unsloth DPO run → new LoRA adapter → Tier B hot-reload
    Local model improves on Foundry patterns → more sessions route locally
    → more training data → compound loop
```

Tier C (Claude) outputs are excluded from the training corpus under Anthropic's
competing-models clause. See `topic-tos-training-constraints.md`.

## 9. Sprint 3 Priority — MCP Before Coding Loop

The MCP server (Sprint 3) ships before `app-console-slm` Sprint 4a for a specific reason:
one `slm-mcp-server` binary makes Foundry capabilities (DataGraph query, corpus stats,
entity extraction, Doorman health) available to every MCP-compatible client — Claude Code,
Cursor, Cline, Continue, and any future tool. This is two weeks of work that delivers to
N clients. Building the coding agent loop first would deliver to one client in six to eight
weeks.

## 10. References

- `service-slm/crates/slm-doorman-server/src/http.rs:1214` — Sprint 0a implementation
- `service-slm/docs/topic-anthropic-gateway-integration.md` — shim mechanics detail
- `service-slm/docs/topic-tos-training-constraints.md` — Anthropic ToS analysis
- `service-slm/docs/guide-activate-anthropic-shim.md` — activation runbook
- `.agent/plans/sovereign-routing-comprehensive.md` — full sprint plan with cost model
