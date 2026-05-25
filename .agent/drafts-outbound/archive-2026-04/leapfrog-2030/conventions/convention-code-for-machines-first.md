---
schema: foundry-draft-v1
state: refined-pending-master-commit
target_path: conventions/code-for-machines-first.md
audience: foundry-internal + vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-CONVENTION
authored: 2026-04-30
authored_by: master @ /srv/foundry
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
claim: 51
research_done_count: 4
research_suggested_count: 2
open_questions_count: 0
research_provenance: master-direct-from-mcp-substrate-protocol-claim
research_inline: true
---

# Code-for-Machines First

Every Foundry inter-service contract, audit record, configuration, and ontology
is machine-readable as a primary surface. Human-facing surfaces are skins on
machine-first APIs.

This convention codifies Doctrine claim #51 (ratified v0.1.0).

## §1 — The discipline

The data formats are:

- Inter-service communication: MCP (claim #46)
- Audit ledger: JSONL with schema versioning
- Seed taxonomies: JSON
- Configuration: TOML or YAML
- Doctrine and conventions: markdown with structured frontmatter
- Per-tenant configuration: YAML

Every artefact is machine-mutable and machine-introspectable. There is no
"human-only" data surface. The TUI, web UI, and mobile UI all consume the same
MCP servers any other client would.

## §2 — Why this matters

- **Universal observability**: structured data at every layer means audit and
  metrics are consistent across services and tenants
- **Customer extension**: customers write MCP servers in their own languages;
  they do not need to fork Foundry to extend it
- **AI-native composition**: the substrate is consumable by AI agents (cluster
  Tasks, customer agents, partner integrations) without a retrofit step
- **Migration ease**: data export is a routine machine operation, not a
  migration project

## §3 — When human-readable beats machine-first

Two narrow exceptions:

1. **Doctrine and convention prose**: this document is a markdown file intended
   for human reading first. The frontmatter is machine-readable; the body is
   human-first. Operations on the body (refining register, citation resolution)
   are AI-augmented, but the artefact's primary surface is human.

2. **Topic and guide documentation**: these are explicitly human-facing per
   CLAUDE.md §14. The frontmatter is structured; the body prose is for readers.

These exceptions are explicit and narrow. The default is machine-first.

## §4 — Composition

- Claim #46 (MCP-as-Substrate-Protocol): the structural realization of this claim
- Claim #43 (Single-Boundary Compute Discipline): the Doorman boundary exposes
  a machine-first MCP gateway
- Claim #44 (Knowledge-Graph-Grounded Apprenticeship): training tuples are
  JSONL — machine-first at every layer

## Provenance

Research reviewed: MCP 2026 roadmap (machine-first composition pattern);
API-first design literature (industry pattern); JSON Schema as machine-first
contract substrate; ProtoBuf alternatives explored and rejected (binary format
too opaque for audit-ledger purposes).

Suggested next research: (1) JSON Schema files for every Foundry data surface
(per-service schema discovery via MCP); (2) operator UX research — do human
surfaces lose anything by being skins on machine APIs?

## References

- `DOCTRINE.md` claim #51
- Companion: `conventions/mcp-substrate-protocol.md` (claim #46)
