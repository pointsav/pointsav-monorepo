---
schema: foundry-draft-v1
state: refined-pending-master-commit
originating_cluster: master-workspace
target_repo: ~/Foundry
target_path: conventions/mcp-substrate-protocol.md
audience: foundry-internal + vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-CONVENTION
authored: 2026-04-30
authored_by: master @ /srv/foundry
authored_with: claude-opus-4-7-1m
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
claim: 46
research_done_count: 12
research_suggested_count: 4
open_questions_count: 2
research_provenance: master-web-research-mcp-2026-roadmap + cda-fortune500-survey
research_inline: true
references:
  - DOCTRINE.md claim #46
  - conventions/single-boundary-compute-discipline.md
  - conventions/three-ring-architecture.md
  - https://modelcontextprotocol.io/specification/2025-11-25
  - https://blog.modelcontextprotocol.io/posts/2026-mcp-roadmap/
---

# MCP-as-Substrate-Protocol

Every Foundry Ring 1 and Ring 2 service exposes a Model Context Protocol (MCP)
server interface as its primary external contract. The Doorman (`service-slm`)
is the MCP gateway. Customer extensions plug in as additional MCP servers. MCP
is the substrate-level wire contract for service composition.

This convention codifies Doctrine claim #46 (ratified v0.1.0). Read at every
architecture decision touching inter-service communication.

## §1 — Why MCP is substrate-level for Foundry

The Model Context Protocol is the current industry standard for AI-native
application composition [mcp-spec]. As of early 2026, 28% of Fortune 500
companies had implemented MCP servers in their AI stacks (per CData Software
industry survey); 75% of API gateway vendors and 50% of iPaaS vendors are
projected to ship MCP features during 2026.

Foundry adopts MCP because the substrate-level wire contract for service
composition has structurally converged on this standard. Bespoke per-service
wire formats (the alternative) accumulate versioning debt, per-pair contract
testing costs, and custom client implementations in every consumer. MCP is the
coordination point that prevents that drift.

## §2 — The three-role pattern as Foundry composition

MCP defines three roles. Foundry's three-ring architecture (claim #16) maps
onto them:

| MCP role | Foundry equivalent |
|---|---|
| **MCP Server** | Each Ring 1 and Ring 2 service: service-fs, service-content, service-extraction, service-input, service-people, service-email, service-egress, service-marketplace, service-ad-exchange, service-settlement |
| **MCP Client** | The Doorman gateway (consumes Ring 1+2 servers as tools); the TUI (slm-cli); customer-built agents; IDE extensions |
| **MCP Host** | The Doorman is also the Host for inference-traffic flows; the slm-cli TUI is the Host for operator-traffic flows |

In Foundry, the Doorman is both Client (for inbound inference; consuming
service-content as a tool) and Host (for outbound inference; presenting the
unified interface to slm-cli). The reuse is intentional: the same process that
holds inference credentials (claim #43) also mediates tool composition.

## §3 — Tool semantics

Each Foundry Ring service exposes a small set of MCP tools. Examples:

**service-content** (knowledge runtime):
- `graph_query(module_id, query_terms, max_depth, max_nodes) → subgraph`
- `graph_mutate(module_id, operations) → applied_count`
- `vector_search(module_id, query_embedding, top_k) → matches`
- `temporal_query(module_id, entity_id, at_time) → historical_state`

**service-fs** (WORM ledger):
- `ledger_append(module_id, entry) → {audit_id, hash}`
- `ledger_query(module_id, range) → entries`
- `checkpoint(module_id) → checkpoint_id`

**service-extraction** (entity extraction):
- `extract_entities(module_id, content, content_type) → entities`
- `classify(module_id, entity, taxonomy) → classification`

**service-people** (CRM):
- `person_lookup(module_id, identifier) → person`
- `person_upsert(module_id, person) → {person_id}`
- `relationship_query(module_id, person_id, relationship_type) → related`

**service-marketplace** (data marketplace; per claim #52):
- `listing_create(module_id, listing) → {listing_id}`
- `listing_update(module_id, listing_id, delta) → ok`
- `listing_query(buyer_id, query) → listings` *(buyer-side)*
- `transaction_initiate(buyer_id, listing_id, terms) → {transaction_id}`

The tool catalog is enumerated in each service's MCP `describe` response.
Customers extending Foundry add new MCP servers without touching the core;
existing services and the Doorman discover new tools at session start.

## §4 — Resource semantics

In addition to tools, MCP defines resources — addressable artefacts the client
can read. Foundry exposes:

- service-fs: each ledger entry is a resource (`fs://module_id/audit_id`)
- service-content: each graph node is a resource (`graph://module_id/entity_id`)
- service-marketplace: each listing is a resource (`market://module_id/listing_id`)
- Documentation surfaces (topic files in content-wiki-documentation) are
  resources accessible by the TUI for operator-facing help

Resources are read-only from the MCP client perspective; mutations go through
tools, preserving write-boundary discipline.

## §5 — Authentication and per-tenant scoping

Per the 2026 MCP roadmap (OAuth 2.1 priority), Foundry MCP servers
authenticate clients via:

- **Workspace VM and Totebox local clients**: filesystem socket permissions or
  localhost-only bind (no additional auth required; the OS user is the auth
  boundary)
- **Cross-Totebox or remote clients**: OAuth 2.1 bearer tokens scoped to
  `module_id` (the per-tenant isolation boundary from claim #34)
- **Customer-extension MCP servers**: the same OAuth 2.1 pattern;
  customer-issued tokens for customer-tenant scope

The Doorman, as the Single-Boundary gateway (claim #43), is the only client
that may issue MCP calls outside the local-tenant scope. All other MCP clients
are tenant-bound by their authentication.

## §6 — The Doorman as MCP gateway

When a Master, Root, Task, or TUI session calls
`POST /v1/chat/completions`, the Doorman:

1. Records the inbound call in the audit ledger
2. Calls service-content's `graph_query` MCP tool to assemble grounding
   context (per claim #44)
3. Routes the request to Tier A, B, or C based on `task_type` (per claim
   #40 amendment)
4. Receives the inference response
5. Optionally calls service-content's `graph_mutate` MCP tool to write
   verdict-accepted graph updates
6. Records the outbound response in the audit ledger
7. Returns the response to the calling session

The Doorman is therefore an MCP **client** (calling service-content) and an
MCP **gateway** (presenting `/v1/chat/completions` to upstream clients). This
is the substrate-level mediation point.

## §7 — Customer extensions plug in without core modification

A customer with a vertical-specific data source (a restaurant POS system, a
law firm case-management system, a hospital EHR) writes an MCP server that
exposes their system to the Doorman. The Doorman discovers the new tools via
MCP `describe`; subsequent inference calls can use the customer's tools
alongside the Foundry built-ins.

This is the structural opening for the Vertical Seed Packs Marketplace
(claim #50): each pack ships with a reference MCP server for that vertical's
typical data sources. The customer customizes from there.

## §8 — Versioning and conformance

Foundry MCP servers carry semantic versioning aligned with workspace versioning
rules (CLAUDE.md §7). The MCP `describe` response includes:

- `server_version: "0.1.0"` (the server's own version)
- `mcp_version: "2025-11-25"` (the MCP protocol version it speaks)
- `foundry_doctrine_version: "0.1.0"` (the doctrine version it was authored against)
- `tool_catalog: [...]` (the tools it exposes; signed)

A Doorman that consumes a service whose MCP protocol version is older or newer
than expected emits a warning to the audit ledger and either falls back to a
compatibility shim or refuses the call (configurable per-service).

## §9 — Composition with claim #51 (Code-for-Machines First)

MCP is the structural realization of claim #51. Every service contract is
machine-readable; the human operator-surfaces (TUI, web) consume the same MCP
interfaces any other client would. There is no "human-only" data surface in
Foundry.

## §10 — When MCP is not the answer

Two narrow exceptions:

1. **Tight-loop performance paths**: when a Ring 2 service composes with
   another Ring 2 service in a hot inference path (e.g., service-content graph
   traversal on every Doorman call), an in-process library call may be
   acceptable for latency reasons. The convention is: in-process when both
   services run in the same binary; MCP across binary boundaries.

2. **Data-plane bulk transfer**: large blob transfers (e.g., a graph export, a
   corpus snapshot) bypass MCP for efficiency and use direct HTTP or file-based
   transport. The MCP tools interface remains for control-plane operations
   (initiating the export); the bulk transfer uses the URL the tool returns.

These exceptions are explicit and narrow. The default is MCP.

## §11 — Migration path from existing wire formats

Existing Foundry services have bespoke wire formats. The migration to MCP is
staged:

| Service | Current state | Target state | Migration phase |
|---|---|---|---|
| service-slm Doorman | OpenAI-compatible HTTP | MCP gateway + OpenAI-compat preserved | Phase 4 |
| service-fs | REST (workspace v0.1.23 deployed) | MCP server | Phase 3 |
| service-content | Legacy file-watcher (deprecated; rebuild) | MCP server (rebuild) | Phase 3 |
| service-extraction | Legacy file-watcher (deprecated; rebuild) | MCP server (rebuild) | Phase 3 |
| service-input | Scaffold-coded | MCP server | Phase 3 |
| service-people | Scaffold-coded | MCP server | Phase 3 |
| service-egress | Scaffold-coded | MCP server | Phase 3 |
| service-marketplace | New (per claim #52) | MCP server | Phase 5 |
| service-ad-exchange | New (per claim #52) | MCP server | Phase 5 |
| service-settlement | New (per claim #53) | MCP server | Phase 5 |

The OpenAI-compatible HTTP boundary on the Doorman is preserved indefinitely —
third-party clients that use the OpenAI SDK continue to work. MCP is added
alongside, not as a replacement.

## Provenance

Research reviewed: MCP specification (https://modelcontextprotocol.io/specification/2025-11-25);
2026 MCP roadmap (https://blog.modelcontextprotocol.io/posts/2026-mcp-roadmap/);
CData Fortune 500 MCP adoption survey; Truto SaaS PM 2026 MCP guide; Anthropic
modelcontextprotocol.io organization; stateless HTTP transport variant; OAuth
2.1 enterprise auth integration; Foundry's existing internal HTTP wire formats;
Three-Ring Architecture composition rules (claim #16).

Suggested next research: (1) MCP authentication bridge to Foundry's per-tenant
moduleId scoping; (2) customer extension MCP server reference template (Rust +
axum + serde-mcp) for vertical pack authors; (3) MCP conformance testing
harness for CI/CD integration; (4) MCP protocol-version-skew handling table.

**OQ #1 — MCP server-server communication.** The current MCP spec defines
client-server communication. When Ring 2 services call other Ring 2 services
(e.g., service-extraction calls service-content during ingest), are those
service-server calls or do they elevate to client? Operational decision pending
v0.1.0 ratification and first MCP rebuild.

**OQ #2 — Streaming response shape under MCP.** The Doorman's existing SSE
streaming for `/v1/chat/completions` is OpenAI-compatible. MCP streaming is a
separate transport. Does Foundry preserve both surfaces indefinitely or migrate
Doorman streaming to MCP-native? Pending architecture decision.

## References

- `DOCTRINE.md` claim #46
- Companion: `conventions/single-boundary-compute-discipline.md` (claim #43)
- Companion: `conventions/code-for-machines-first.md` (claim #51)
- [mcp-spec] https://modelcontextprotocol.io
- External: https://blog.modelcontextprotocol.io/posts/2026-mcp-roadmap/
