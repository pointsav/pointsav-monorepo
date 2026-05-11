---
schema: foundry-draft-v1
state: refined-pending-master-commit
target_path: conventions/knowledge-graph-grounded-apprenticeship.md
audience: foundry-internal + vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-CONVENTION
authored: 2026-04-30
authored_by: master @ /srv/foundry
refined_by: task-project-language (sub-agent 2026-04-30)
refined_date: 2026-04-30
doctrine_version: 0.1.0
claim: 44
research_done_count: 6
research_suggested_count: 3
open_questions_count: 1
research_provenance: cluster-research-iter-24-graphrag + master-validation
research_inline: true
---

# Knowledge-Graph-Grounded Apprenticeship

service-slm consults the service-content per-tenant knowledge graph before
routing every substantive inference request. The atomic training tuple becomes
(query, graph-context, response, verdict). The datagraph and the adapter
co-evolve.

This convention codifies Doctrine claim #44 (ratified v0.1.0). It extends
`conventions/apprenticeship-substrate.md` with a graph-grounding layer.

## §1 — Pre-inference grounding

Before the Doorman dispatches a request to Tier A, B, or C, it calls
service-content's `graph_query` MCP tool to assemble a 2-hop subgraph around
the query terms. The subgraph is rendered as a system-prompt prefix:

```
Known context (graph-grounded):
  Entity: ARTHUR_PENDELTON (Person, Executive archetype)
  Relations: works-for COMP_001 (Woodfine Capital Projects);
             owns-50pct PROP_4B (Building 3, Unit 4B);
             references DOC_INVESTOR_RELATIONS_2026Q1
  Domain: Corporate Strategy
  Theme: THM-04 Q3 Capital Procurement (active)

User query: <original prompt>
```

The Tier A or Tier B model receives this as additional context. The
`grounded_entity_ids` field is recorded in the audit ledger for citation
verification.

## §2 — Post-inference graph mutation

When the model's response includes structured outputs validated against JSON
Schema (the Doorman's existing structured-outputs path per v0.1.33 Q2), the
verdict-acceptance flow proceeds as follows:

1. The senior verdict marks the response `accept` (or `refine` with correction)
2. The Doorman extracts proposed graph mutations (new entities, new
   relationships, updated properties)
3. The Doorman calls service-content's `graph_mutate` MCP tool
4. service-content applies the mutations atomically, per-tenant
5. The audit ledger records the graph_mutation event

The graph grows with each accepted call. New entities discovered during
inference become grounding context for the next inference. The loop is closed.

## §3 — Training tuple shape with graph_context

The apprenticeship corpus tuple gains a `graph_context` field:

```json
{
  "tuple_type": "shadow-capture",
  "task_type": "...",
  "stage_at_capture": "review",
  "brief": { ... },
  "graph_context": {
    "module_id": "woodfine",
    "subgraph": { "nodes": [...], "edges": [...] },
    "context_summary": "..."
  },
  "attempt": "...",
  "verdict": null,
  "actual_diff": "...",
  "doctrine_version": "0.1.0",
  "tenant": "pointsav|woodfine",
  "created": "2026-04-30T..."
}
```

DPO training treats verdict-signed tuples with `graph_context` populated as
higher-weight examples. SFT training over unsigned tuples uses `graph_context`
as additional input signal. The graph and the adapter co-evolve.

## §4 — Graph-coherence as a quality metric

Each model response can be evaluated for graph-coherence on three dimensions:

- **Citation rate**: percentage of named entities in the response that exist in
  the graph
- **Relationship accuracy**: percentage of stated relationships that match graph
  edges
- **Hallucination rate**: percentage of named entities NOT in the graph (the
  primary failure mode)

These metrics feed the verdict-signing process. A response with low citation
rate is a candidate for refinement; one with high hallucination rate is a
candidate for rejection.

## §5 — Per-tenant graph isolation

The `module_id` field enforces tenant isolation. service-content's `graph_query`
and `graph_mutate` tools require a `module_id` and refuse cross-tenant
traversal. The Woodfine adapter trains on Woodfine graph context; the PointSav
adapter trains on PointSav graph context. There is no cross-tenant leakage at
training time.

## §6 — When graph context is absent

Some inference requests have no relevant graph context — for example, a generic
sysadmin question such as "what does `systemctl restart` do". The Doorman calls
`graph_query`, receives an empty subgraph, and proceeds without grounding. The
audit ledger records `graph_context: null`. These tuples remain valid training
data; they are simply ungrounded. The model learns that some questions do not
require graph context.

## §7 — Composition with claim #43 (Single-Boundary)

Graph-grounded apprenticeship depends on the Single-Boundary Compute Discipline.
If inference can bypass the Doorman, it bypasses graph grounding. The two claims
compose: without #43, #44 cannot be structurally enforced.

## Provenance

Research reviewed: Microsoft GraphRAG (2024) — 30–40% hallucination reduction;
Glean and Perplexity workspace-context patterns; Foundry's existing
service-content seed taxonomy (5 Archetypes, 4 COA profiles, 3 Domains, 4
Themes; gravity-keyword classification); Apprenticeship Substrate v0.0.13 §7B
(capture-on-completion semantics); §7C Brief Queue Substrate (queue, drain,
reaper operational since v0.1.85); KuzuDB → LadybugDB migration (graph DB
substrate for Phase 2 rebuild).

Suggested next research: (1) citation-rate measurement instrumentation (audit
ledger field addition); (2) graph-coherence DPO weighting algorithm
(training-pipeline scope); (3) per-tenant subgraph cache layer for
high-frequency queries.

**OQ #1 — Subgraph extraction algorithm.** What is the right algorithm for
assembling a 2-hop subgraph around query terms? Options include keyword-anchor
expansion, embedding-similarity neighborhood, and learned attention. Pending
project-data Task scope decision during service-content rebuild.

## References

- `DOCTRINE.md` claim #44
- Companion: `conventions/single-boundary-compute-discipline.md` (claim #43)
- Companion: `conventions/apprenticeship-substrate.md` (claim #32; §8 amendment)
- Companion: `conventions/seed-taxonomy-as-smb-bootstrap.md` (claim #47)
