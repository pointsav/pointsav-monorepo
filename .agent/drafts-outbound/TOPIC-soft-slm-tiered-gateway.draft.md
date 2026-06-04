---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC
status: staged-pending-editorial
title: "The tiered inference gateway — local-first AI routing"
slug: soft-slm-tiered-gateway
target_repo: media-knowledge-documentation
target_path: media-knowledge-documentation/substrate/soft-slm-tiered-gateway.md
paired_with: soft-slm-tiered-gateway.es.md
category: substrate
quality: complete
bcsc_class: public-disclosure-safe
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: "2026-06-04 project-intelligence session — architecture consolidation, competing agent review, adversarial validation"
research_inline: true
supersedes: slm-tiered-substrate.md
---

# The tiered inference gateway — local-first AI routing

A tiered inference gateway routes every AI request through a hierarchy of compute
tiers, selecting the least expensive capable tier for each request. Routine work runs
on hardware the organization owns. Burst capacity on rented GPU handles work that
exceeds local capability. An external commercial API provides a final fallback. Each
tier degrades gracefully to the one below it; no tier is a single point of failure.

## Why local-first matters

Routing all inference to an external service is simple to operate but has structural
costs. Every request crosses an organizational boundary, exposing the content of
prompts and responses to a third-party provider. Cost is proportional to usage with
no amortization. The organization has no way to adapt the model to its own vocabulary,
processes, or accumulated knowledge.

A local-first gateway eliminates these costs for the majority of work. The local model
handles requests that fall within its capability. External resources handle what it
cannot. Over time, the local model improves through usage-derived training, narrowing
the set of requests that require external compute.

## The three tiers

### Tier A — local inference

Tier A is an inference server running on the organization's own hardware. It is
always running, produces responses in seconds, and costs nothing per request beyond
the amortized hardware. It is the default destination for all requests.

The local model is purpose-trained or fine-tuned for the organization's domain.
It is smaller and faster than models at higher tiers. It answers the majority of
routine requests competently: summarization, classification, entity extraction from
known document types, code generation in known patterns.

Tier A does not handle grammar-constrained structured output well at small model
sizes. Requests requiring precise JSON schema enforcement are routed to Tier B.

### Tier B — burst GPU node

Tier B is one or more remote inference nodes running a larger, more capable model
on dedicated GPU hardware. Nodes start on demand and stop when idle, so the cost is
proportional to actual use rather than availability.

The gateway maintains a per-node circuit breaker and a VM lifecycle state machine.
When a Tier B request arrives and the target node is stopped, the gateway starts it
automatically. The caller receives a 202 Accepted response with a polling endpoint
while the node boots. Once the node is ready, the request is served.

When the node is running, requests are dispatched immediately. When the circuit
breaker opens — after consecutive health probe failures — requests fall back to
Tier A or queue until the node recovers.

Tier B nodes are organized by label. A `batch` label handles background work: corpus
extraction, training data processing, scheduled maintenance. An `express` label handles
time-sensitive work that cannot wait for a cold start.

### Tier C — external inference provider

Tier C is an optional connection to a commercial language model API. It serves as a
final fallback when both Tier A and Tier B are unavailable, and as a direct route for
tasks the organization has explicitly designated as external.

Tier C is never used as a source of training data. An organization that routes
inference to an external provider and then fine-tunes on those responses is
building a dependency on a third party's output quality and terms of service.
The boundary is enforced in the gateway's routing logic.

Tier C requires an explicit API key to activate. Without the key, requests that
reach Tier C fall back to Tier A.

## Request routing

Every request carries a complexity hint and, optionally, a tier label. The gateway
selects the tier using this decision sequence:

1. If a kill switch is closed for the requested tier, the request is rejected or
   falls to the next tier depending on configuration.
2. If an explicit tier label is present, the request is routed to that tier.
3. If no label is present, the routing policy applies:
   - `balanced`: low and medium complexity → Tier A; high complexity → Tier B.
   - `drain-batch`: all non-express work routes to the batch node.
   - `drain-express`: all work routes to the express node to clear a backlog.
   - `local-only`: all work routes to Tier A regardless of complexity.
4. If the selected tier is unavailable, the request falls to the next tier unless
   tier affinity is required (for example, structured extraction requires Tier B and
   does not fall back to Tier A).

The routing policy is configurable at runtime without restarting the gateway:

```
POST /v1/flow/policy  { "policy": "balanced" }
```

## The kill switch

Every tier has an independent kill switch. Closing a kill switch stops all new
dispatching to that tier immediately. In-flight requests complete; no new requests
start. Queued work accumulates and drains when the kill switch is reopened.

The kill switch is the operator's billing control. Closing the express node switch
stops the A100 from starting; the cost drops to zero. Closing the global switch
stops all Tier B and Tier C spending while allowing Tier A to continue serving.

The express lane — which bypasses the file-backed queue for time-sensitive work —
still checks the kill switch. Nothing bypasses the kill switch.

## The priority queue

Background work — apprenticeship briefs, corpus extraction, training corpus generation
— is processed through a file-backed priority queue with three levels:

- **P0** routes exclusively to the local model for lightweight classification.
- **P1** routes to the batch GPU node for extraction work requiring structured output.
- **P2** routes to the batch GPU node for training corpus generation and similar
  long-running background tasks.

The queue drain worker processes one item from each level per cycle, in P0 → P1 → P2
order, then repeats. This prevents a large batch of P2 work from blocking P1
extraction tasks for an extended period.

## Organizational memory context

Before dispatching any request to any tier, the gateway queries the organizational
knowledge graph for entities relevant to the current request. Matching entities are
injected into the system prompt as structured context. The model sees the
organization's known relationships, decisions, and policies without those facts
needing to be re-derived from inference.

This context injection is non-fatal: if the graph service is unavailable, the request
proceeds without context. A circuit breaker on the graph query path prevents a
slow graph service from blocking inference.

## The MCP server

The gateway exposes an organizational memory interface via the Model Context Protocol
at a second port. Any MCP-capable AI client can connect to this interface using its
built-in subscription — no separate API key is required. The client's reasoning
capability combines with the gateway's organizational knowledge graph to produce
responses that are grounded in the organization's actual data.

This is the primary path for interactive use by operators who already have a
subscription to an MCP-capable client. The gateway handles memory; the client
handles reasoning.
