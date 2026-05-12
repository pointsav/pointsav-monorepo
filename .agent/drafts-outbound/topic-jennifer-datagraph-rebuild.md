---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-intelligence
target_repo: content-wiki-documentation
target_path: ./
target_filename: topic-jennifer-datagraph-rebuild.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-11
authored_by: command@claude-code (session 2026-05-11)
authored_with: claude-sonnet-4-6
references:
  - service-slm/scripts/jennifer-datagraph-rebuild.sh
  - service-slm/CLAUDE.md
  - service-content/ARCHITECTURE.md
  - conventions/three-ring-architecture.md
  - conventions/worm-ledger-design.md
notes_for_editor: |
  This TOPIC covers the jennifer DataGraph rebuild as a nightly integration
  test and property-graph update. The routing parity principle is the
  editorial centrepiece — make sure it reads as a deliberate design decision,
  not an incidental implementation detail.
  The three-consecutive-HEALTHY gate must be clearly framed as a forward
  criterion (planned/intended) since it has not yet been met.
  Cross-reference with topic-yo-yo-lora-training-pipeline.md: the DataGraph
  rebuild is Phase 1 of the same nightly window; the LoRA training TOPIC
  covers Phase 2.
  The jennifer-graph is live at service-content (11 MB LadybugDB file) —
  state this as current fact.

  UPDATE NOTE (2026-05-12): First successful extraction confirmed — 4–11
  entities per document, pipeline end-to-end verified 2026-05-12. State
  pipeline as current-fact (not forward-looking). Three-consecutive-HEALTHY
  gate still forward criterion. Doorman response envelope is `.content`
  (not OpenAI `.choices[]`) — relevant if body text references the wire
  format.
---

# TOPIC — Jennifer DataGraph Rebuild

## Overview

Each night, `jennifer-datagraph-rebuild.sh` processes the jennifer business
data corpus and writes extracted named entities to a property graph stored in
LadybugDB. This property graph — the jennifer DataGraph — is the entity layer
that service-content uses to inject structured business context into inference
requests. The rebuild runs as Phase 1 of the Yo-Yo #1 nightly window, before
the training phase claims the GPU. The jennifer DataGraph is live, with an
11 MB LadybugDB file currently active at service-content.

## What the Jennifer DataGraph Contains

The jennifer DataGraph is a property graph of named business entities
extracted from the jennifer deployment's data corpus. The graph holds five
entity classifications: Person (staff, contacts, counterparties), Company
(vendors, customers, partner organisations), Project (active and historical
engagements), Account (financial accounts and ledger references), and
Location (offices, sites, and operational addresses). These entities are
extracted from three document streams: meeting transcript markdown files
from the minutebook asset directory, research and background YAML and markdown
files from the service-agents directory, and contact source JSON records from
the service-people directory.

## What the Nightly Rebuild Does

For each unprocessed document, the rebuild script calls
`POST :9080/v1/chat/completions` through the Doorman endpoint, passing the
document text with a JSON Schema grammar constraint. The language model —
OLMo 3 32B Think running on Yo-Yo #1 via vLLM — returns a structured JSON
array of entity objects. Each object carries the entity name, classification,
confidence score, and optional role, location, and contact vectors. The script
then calls `POST :9081/v1/graph/mutate` on service-content to write those
entities into LadybugDB. The health probe at the end of the cycle queries
service-content for the current entity count and writes a summary JSON file
at `$FOUNDRY_ROOT/data/datagraph-health.json`.

The script processes three document batches each run: the full minutebook
asset tree, the full service-agents tree, and the 50 most recent unprocessed
service-people JSON files. A randomised inter-document delay (0.3 to 1.5
seconds) prevents the Doorman from receiving a burst of requests that could
interfere with the training phase startup.

## The Routing Parity Principle

The jennifer-datagraph-rebuild.sh script calls only the same two REST API
endpoints that any operator or community member running service-slm and
service-content would call from their own automation:

- `POST :9080/v1/chat/completions` — entity extraction through Doorman
- `POST :9081/v1/graph/mutate` — entity write through service-content

There is no file-watcher shortcut, no internal gRPC bypass, and no direct
database write. This is a deliberate design decision. If the rebuild script
fails, the failure indicates a real defect in service-slm or service-content
that would also affect any operator or customer running the same API surface.
The nightly rebuild functions as a full-stack integration test that runs
against production services on production data every night. Failures are
explicit and immediately actionable rather than hidden in an internal path
that real callers would never exercise.

## Idempotency

The script tracks processed documents using a local ledger at
`$FOUNDRY_ROOT/data/datagraph-processed.txt`. Each document is identified by
a hash of its file content, prefixed with a source tag (`mk-` for minutebook,
`ag-` for service-agents, `sp-` for service-people). Before processing any
document, the script checks whether its identifier appears in the ledger. If
it does, the document is skipped. After a successful `graph/mutate` call, the
identifier is appended to the ledger. This mechanism ensures that documents
are not re-processed across multiple nightly runs, even if the same content
is present in the source directories.

The ledger is append-only and not pruned automatically. If service-content
is restarted and the graph is rebuilt from scratch, the ledger can be cleared
to force a full re-extraction on the next nightly run.

## Graph Context Injection

The jennifer DataGraph is not a static reference store. service-content
queries it before each inference request. When the Doorman receives a
completion request from an operator or application, service-content retrieves
entities relevant to the request context — based on module ID, entity
classification, and confidence thresholds — and injects them into the system
message as a structured entity context block. The language model receives
structured business context (who the relevant people are, what projects are
active, which companies are counterparties) without requiring that structured
data to cross the external model boundary. The graph stays within the
deployment boundary; only the injected prose context leaves it.

## Current Status and Gate Criterion

The jennifer DataGraph is live. Three consecutive nightly runs reporting
HEALTHY status — defined as a non-negative entity count delta and a successful
round trip on both the extraction and mutation endpoints — are the intended
criterion before the DataGraph pattern is extended to larger operational
contexts. That gate has not yet been met; the rebuild pipeline is in its
initial operational period.
