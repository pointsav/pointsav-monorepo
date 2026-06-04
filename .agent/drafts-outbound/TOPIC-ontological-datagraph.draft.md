---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC
status: staged-pending-editorial
title: "The organizational knowledge graph — ontological memory for business operations"
slug: ontological-datagraph
target_repo: media-knowledge-documentation
target_path: media-knowledge-documentation/substrate/ontological-datagraph.md
paired_with: ontological-datagraph.es.md
category: substrate
quality: complete
bcsc_class: public-disclosure-safe
research_done_count: 4
research_suggested_count: 0
open_questions_count: 0
research_provenance: "2026-06-04 project-intelligence session — DataGraph architecture consolidation, competing agent analysis, web research on enterprise knowledge graphs"
research_inline: true
---

# The organizational knowledge graph — ontological memory for business operations

An organizational knowledge graph stores what a business knows about itself: who its
people, companies, and projects are; how they relate to one another; what decisions
have been made and by whom; which policies govern which activities. This structured
memory is available to every AI inference request, injected as context before the
model produces its response.

The graph answers a class of question that a vector similarity search cannot: not just
"what documents mention ACME Corp?" but "what is ACME Corp, who do we know there,
what contract governs our relationship, and what decisions have we made about their
invoices?" The traversal follows edges. The answer emerges from structure, not
keyword proximity.

## One graph per deployment node

A deployment node maintains exactly one organizational knowledge graph. All services
running on that node contribute entities to this single store, scoped by a
module identifier that keeps each domain's data isolated within the same physical
database.

This design supports cross-domain reasoning without duplication. When a bookkeeping
service writes "ACME Corp is a vendor with net-30 payment terms" and a document
extraction service writes "ACME Corp is headquartered in Toronto," both facts exist
in the same graph, attached to the same entity. A query about ACME Corp retrieves
both facts in a single traversal.

A separate graph per service would require the inference router to query multiple
sources, merge results, and resolve conflicts — complexity that produces worse
answers at higher operational cost. Industry-scale knowledge graph systems converge
on a unified semantic layer regardless of how many services produce the underlying data.

## What belongs in the graph

The graph stores ontological facts: what entities exist, how they relate, and what
is true about them at a given point in time. It does not store transactional records.

**In the graph:**
- An organization is a vendor (entity with a relationship attribute).
- A contract exists between two parties with specific terms (entities with a relationship carrying properties).
- A decision was made by a named person under a named policy (entities with typed edges).
- A property is owned by a company with a specific classification (entities with a property edge).

**Not in the graph:**
- Individual invoice line items (these are transactional records; they belong in an immutable append-only ledger).
- Journal entries (double-entry accounting records; belong in the bookkeeping ledger).
- Raw document text (belongs in the document store; only the extracted entities from that text belong in the graph).

This distinction matters for auditability. Transactional records are immutable and
must remain in a tamper-evident append-only store. The graph is mutable: facts
are updated as the organization's reality changes.

## Entity types

The graph is configured through ontology files loaded at startup. Each deployment
node can define entity classifications appropriate for its business domain. The
base classifications present in every deployment cover the fundamental organizational
primitives:

- **Person** — named individuals; carries role, contact information, and organizational
  affiliation.
- **Company** — registered organizations; carries classification (vendor, customer,
  partner, regulator), and relationship attributes such as contract terms.
- **Project** — named initiatives; carries status, participants, and governing policies.
- **Account** — financial and service accounts; carries balance class and relationship
  to contract.
- **Location** — geographic places and addresses; carries jurisdiction and physical
  attributes.

Additional classifications are added through ontology CSV files. A legal practice
might add Case, Regulation, and Judgment. A property manager might add Property,
Lease, and Tenant. A manufacturing business might add Equipment, WorkOrder, and
Specification. Each addition extends the graph's reasoning capability for that
domain without modifying code.

## Temporal validity

Every fact in the graph carries a creation timestamp. Facts about entities that
change over time — such as who holds a role, or what terms a contract carries —
can be superseded rather than overwritten. The graph retains the prior fact with
its validity window. A query can ask either "what is true now?" or "what was
true on a given date?"

This temporal property is valuable for auditing and for training. A model trained
on facts that were accurate when the training data was written, but are no longer
current, produces confident incorrect answers. Temporal validity allows the graph
to serve accurate context even as the organization changes.

## Multi-hop traversal

The graph is designed for traversal, not just lookup. A lookup retrieves one entity
by name. A traversal follows edges from that entity to connected entities, and from
those to further connections.

**Example traversal:**

*Query: "What policies govern procurement decisions on this project?"*

1. Start at the Project entity.
2. Follow `governed_by` edges to Policy entities.
3. Follow `defines_exceptions` edges from each Policy to Decision entities.
4. Follow `approved_by` edges from each Decision to Person entities.

The result: the full governance chain, retrieved in a single structured query. A
language model with this context in its prompt can answer questions about governance,
exceptions, and responsible parties without needing to synthesize that knowledge
from unstructured text.

This is the quality advantage that cannot be replicated by a general-purpose AI
service: that service has no knowledge of this organization's structure. The graph
encodes that structure explicitly and makes it available at inference time.

## How entities enter the graph

Entities enter the graph through an extraction pipeline. Documents, emails, meeting
notes, and other prose sources arrive in a watched input directory. The extraction
service reads each source, sends the text to the inference router for structured
entity extraction using a grammar-constrained schema, and writes the resulting
entities to the graph through the router's mutation endpoint.

The extraction quality depends on the inference tier. The local compact model
(Tier A) extracts entities at lower confidence. The burst GPU node (Tier B) extracts
at higher confidence using larger context windows and strict output constraints.
Tier A extraction is useful for rapid coverage; Tier B extraction is used for the
canonical organizational record.

Every extraction is logged with a source reference and a confidence score. Entities
extracted from authoritative sources (executed contracts, filed documents, official
registrations) carry higher confidence than those extracted from informal
correspondence.

## Context injection at inference time

Before the inference router dispatches any request, it queries the organizational
graph for entities relevant to the current request. The query is a substring match
against the last portion of the user's message. Matching entities are formatted as a
structured context block and prepended to the system prompt.

The model receives this context transparently. It does not need to be prompted to
"use the knowledge graph." The context is simply present, as if the model had been
briefed on the relevant organizational relationships before the conversation began.

This injection is non-fatal. If the graph service is unavailable or returns no
matches, the request proceeds without additional context. A circuit breaker on the
graph query path prevents a slow graph service from blocking inference.

## Privacy and sovereignty

The organizational knowledge graph contains sensitive business information: personnel
relationships, contract terms, decision histories, financial account structures. This
data should not leave the organization's control.

The graph runs embedded within the deployment node. Its contents are never sent to
an external inference provider as training data. Context injected into prompts for
external Tier C requests is subject to the organization's data classification policy;
the gateway enforces this through the structured-data boundary rule, which prevents
raw business records from crossing the external inference boundary.

The organization owns the graph database file, the ontology definitions, the entity
data, and the extraction history. These assets are portable: they can be backed up,
migrated, and restored without dependency on any third-party service.
