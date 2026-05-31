---
schema: foundry-draft-v1
state: draft-pending-language-pass
language_protocol: PROSE-GUIDE
originating_cluster: project-design
target_repo: woodfine/woodfine-fleet-deployment
target_path: <tbd-by-project-editorial>
target_filename: guide-design-system-mcp-integration.md
audience: customer-public
bcsc_class: current-fact
authored: 2026-05-08T00:00:00Z
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 3
research_suggested_count: 1
open_questions_count: 1
research_provenance: tacit
research_inline: true
notes_for_editor: |
  Vault stub is live at https://design.pointsav.com/developing/mcp/.
  Open question in research trail: should curl examples use jq piping?
  Language pass: Bloomberg standard. MCP is a registered trademark (Anthropic);
  expand the acronym on first use. The four methods listed are confirmed live
  in main.rs; do not add undeclared methods. Auth state is currently open
  (no token required); this is accurate as of 2026-05-08.
  BCSC note: "future bearer token" is planned/intended language — keep as such.
---

## Research trail

### Done — what informed this draft
- [tacit: vault stub developing/mcp.md] — skeleton content
- [tacit: main.rs /mcp handler + mcp.rs dispatcher] — confirmed four live methods
  and JSON-RPC 2.0 dispatch pattern
- [tacit: app-privategit-design CLAUDE.md] — MCP listed as a declared route

### Suggested — what project-editorial should consult
- [external: modelcontextprotocol.io] — verify MCP spec description one-liner is
  still accurate; update if spec has evolved (medium priority)

### Open questions — for future passes
- Should curl examples use `| jq .` piping? Adds readability but requires jq
  installed. Current stub uses it; consistency with other developing/* guides
  matters. → project-editorial decision.

---

# MCP Integration

The substrate exposes a Model Context Protocol (MCP) server at `POST /mcp`.

MCP is a JSON-RPC 2.0 protocol for exposing structured data to AI agents at
query time. An agent sends a method call; the substrate returns structured data
about its token bundle, component library, and research backplane. The agent
can use this information when generating code or making design decisions.

---

## Endpoint

```
POST /mcp
Content-Type: application/json
```

The endpoint accepts a JSON-RPC 2.0 request object and returns a JSON-RPC 2.0
response object. No authentication is required in the current version.
A bearer token option is planned for deployments that restrict MCP access to
authorised agents.

---

## Methods

### `describe`

Returns substrate metadata: service name, version, tenant identifier, and
counts of loaded tokens, components, and research files.

**Request:**
```json
{"jsonrpc": "2.0", "method": "describe", "id": 1}
```

**Response (example):**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "service": "app-privategit-design",
    "version": "0.1.0",
    "tenant": "pointsav",
    "token_count": 142,
    "component_count": 37,
    "research_count": 8
  }
}
```

### `list_tokens`

Returns all tokens in the active bundle as a flat key-value map, with keys in
dot-path notation (`color.primary-60`, `spacing.5`) and values as resolved strings.

**Request:**
```json
{"jsonrpc": "2.0", "method": "list_tokens", "id": 2}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "color.primary-60": "#234ed8",
    "color.neutral-10": "#f5f6f8",
    "spacing.5": "1.25rem"
  }
}
```

### `list_components`

Returns the names of all components currently loaded in the vault.

**Request:**
```json
{"jsonrpc": "2.0", "method": "list_components", "id": 3}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": ["badge", "breadcrumb", "button", "checkbox", "input-text",
             "link", "navigation-bar", "notification", "select",
             "surface", "switch", "tab"]
}
```

### `list_research`

Returns the slugs of all research files currently loaded in the vault.

**Request:**
```json
{"jsonrpc": "2.0", "method": "list_research", "id": 4}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "result": ["color-system", "typography-scale", "spacing-rationale"]
}
```

---

## Live example

```bash
curl -sS -X POST https://design.pointsav.com/mcp \
  -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","method":"describe","id":1}' | jq .
```

---

## AI agent integration

The MCP endpoint is the primary integration point for AI agents that need to
reason about design decisions, not just generate code. An agent can call
`list_tokens` to understand the full colour system, `list_components` to know
what components are available, and `list_research` to retrieve the rationale
behind specific decisions.

For pure code generation — generating a button or a form — the shadcn registry
at `/r/registry.json` is the simpler entry point. It returns HTML and CSS recipes
directly without requiring a JSON-RPC client. MCP is the richer interface for
agents that need to ask "why" as well as "what".

See the [shadcn registry guide](/developing/shadcn-registry/) for the code
generator integration.
