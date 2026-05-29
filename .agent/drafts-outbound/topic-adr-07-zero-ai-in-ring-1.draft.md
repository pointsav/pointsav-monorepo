---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-data
target_repo: content-wiki-documentation
target_path: ./
target_filename: topic-adr-07-zero-ai-in-ring-1.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
created: 2026-05-29
cites: []
research_trail:
  source_files:
    - service-people/src/acs.rs
    - service-people/src/people_store.rs
    - service-fs/src/ledger.rs
    - conventions/three-ring-architecture.md
  doctrine_refs:
    - DOCTRINE.md (SYS-ADR-07, SYS-ADR-10, claim #16)
  verified_against_commit: "815e11c (feat(service-people): ACS engine absorbed)"
notes_for_editor: >
  Sections 1–6 are current-fact. Section 7 (Ring 3 compositional AI) is
  current-fact for the architectural separation but forward-looking for Ring 3
  deployment — use "intended/planned" language for Ring 3 operational details.
  Spanish skeleton is in topic-adr-07-zero-ai-in-ring-1.es.draft.md.
---

# Topic: SYS-ADR-07 — Zero AI in Ring 1

SYS-ADR-07 is an architectural decision that prohibits AI inference from Ring 1
boundary-ingest services. It is not a preference or a guideline — it is a
hard constraint that all four Ring 1 services implement as code, with no
escape hatch. This topic explains the constraint, the reasoning behind it,
and what "zero AI" means in practice for each service.

---

## 1. What Ring 1 Is

Ring 1 is the outer boundary of the data plane. It is the set of services that
accept raw input from outside the system and write it to durable, immutable
storage. In the current implementation, Ring 1 consists of four services:

| Service | Role |
|---|---|
| `service-fs` | WORM immutable ledger — all Ring 1 writes land here |
| `service-people` | Identity ingest — Person, Anchor, and Claim records |
| `service-email` | Email ingest via EWS (Exchange Web Services SOAP) |
| `service-input` | Document ingest — PDF, Markdown, DOCX, XLSX |

These services are the first point of contact for data entering the system.
Everything they write to `service-fs` becomes part of the permanent, auditable
record. Anything written to the WORM ledger cannot be deleted or modified.

The consequence is that the correctness standard at Ring 1 is different from
the correctness standard elsewhere in the system. Mistakes at Ring 1 are
permanent.

---

## 2. Why Deterministic-Only

Three reasons drive SYS-ADR-07.

**Verifiable audit trail.** The `service-fs` hash chain and Ed25519 checkpoint
signature create a verifiable record of everything written to Ring 1. A third
party can verify the chain from the origin hash to the current tip without
replaying any model state. If Ring 1 operations were AI-derived, this
verification would be incomplete: auditors would need access to the model
weights, sampling configuration, and inference state at the time each record
was written to understand what the system decided and why. Deterministic
operations have no such dependency — the algorithm is the full specification.

**Composability guarantee.** Ring 1 outputs are consumed by Ring 2 services
(`service-extraction`, `service-content`) and Ring 3 intelligence
(`service-slm`). For Ring 2 and Ring 3 to compose Ring 1 data reliably,
they must be able to reason about what Ring 1 produced. An AI-derived Ring 1
output carries implicit uncertainty that propagates through every downstream
computation. A UUID derived deterministically from a lowercase email address
carries no such uncertainty — it is the same value everywhere, always.

**Regulatory posture.** Immutable audit records in regulated industries
(financial services, legal, healthcare) require the ability to explain exactly
what was recorded and exactly how it was derived. Deterministic algorithms
(SHA-256, UUIDv5, regex, Ed25519 signing) have exact, auditable specifications.
AI inference does not.

---

## 3. What "Zero AI" Means in Practice

SYS-ADR-07 prohibits the following at Ring 1:

- LLM inference calls (via API or local inference)
- Embedding-based similarity computations
- Model weights in the process image
- Probabilistic classification or entity extraction

What is allowed:

- Regular expressions (deterministic pattern matching)
- SHA-256 and other cryptographic hash functions
- UUIDv5 (deterministic from a fixed namespace and input)
- Ed25519 signing and verification
- JSON/XML/OOXML parsing (structural, not semantic)
- Explicit confidence scores assigned by rule, not by a model

The distinction is: computation whose output is fully determined by its
input and a fixed algorithm (allowed) versus computation whose output depends
on learned weights or stochastic sampling (prohibited).

---

## 4. Implementation Instances

### service-fs

All operations in `service-fs` are pure cryptography and file I/O:

- **Append** — D4 atomic-write (`.tmp` → fsync → rename → chmod 0o444);
  SHA-256 entry hash chained from previous
- **Checkpoint** — SHA-256 root hash over the chain; Ed25519 signature
  using operator-supplied key; C2SP signed-note wire format
- **Anchor-emitter** — ephemeral Ed25519 keypair per run; Sigstore
  `hashedRekordRequestV002` POST; deterministic hash of the checkpoint payload

No model, no inference, no probability. Every operation has a single
deterministic output for a given input and key.

### service-people

Identity extraction uses a single compiled regular expression:

```
(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}
```

UUID derivation uses UUIDv5:

```
target_uuid = UUIDv5(NAMESPACE_DNS, lowercase(email))
```

Confidence scores are assigned by rule: `1.0` for a regex match. There is no
model that produces a score between 0 and 1 based on learned representations.

Conflict detection compares UUIDs by equality. When two ingest operations
would bind the same email address to different UUIDs, the system surfaces
the conflict to the caller — it does not invoke inference to resolve it.

### service-email

`service-email` reads email from Microsoft Exchange via EWS (Exchange Web
Services) SOAP protocol. The operations are:

- XML string parsing of SOAP envelopes (structural, not semantic)
- Base64 decoding of MIME content
- HTTP bearer authentication using a pre-acquired token (`AZURE_ACCESS_TOKEN`)

No content classification. No entity extraction. No spam scoring. The service
reads what Exchange provides and writes it to `service-fs`. Any intelligence
applied to email content happens downstream, in Ring 2 or Ring 3, reading
from the WORM ledger — never inline during ingest.

### service-input

`service-input` parses documents into text content:

- **PDF** — structural parsing via `oxidize-pdf`; extracts text spans
- **Markdown** — event-stream parsing via `pulldown-cmark`; strips HTML tags
- **DOCX** — ZIP + XML paragraph extraction via `docx-rust`
- **XLSX** — all-sheets tab-delimited extraction via `calamine`

In every case, the parser reads format-defined structure and emits text.
No semantic interpretation, no summarisation, no classification. The raw
extracted text is what lands in `service-fs`.

---

## 5. The Ring 2 Boundary

Intelligence enters the system at Ring 2. `service-extraction` reads from Ring 1
via MCP and applies deterministic parser combinators plus, optionally, Ring 3
inference calls. Its outputs are Ring 2 artifacts — they do not write back
to Ring 1's WORM ledger entries; they create new entries tagged as Ring 2
provenance.

This boundary is structural, not convention-based. Ring 2 services access
Ring 1 via HTTP MCP calls. There is no in-process path from Ring 2 inference
back into Ring 1 storage. A Ring 2 service that wants to annotate a Ring 1
record creates a new Ring 2 record referencing the Ring 1 entry — it does
not modify the Ring 1 entry.

---

## 6. SYS-ADR-10 (F12): Surfacing Ambiguity to the Operator

When deterministic processing encounters a situation that cannot be resolved
algorithmically — for example, a conflict in `service-people` where the same
email address would map to two different UUIDs — the system returns an error
to the caller. It does not:

- Silently pick one of the conflicting records
- Apply a model to guess which is "more likely"
- Merge the records based on heuristic similarity

The F12 requirement (SYS-ADR-10) mandates that ambiguity surfaces to the
operator. Resolving the ambiguity is an explicit operator action, not an
automatic system behaviour. This is a composability guarantee: operators
can reason about what the system decided, because every non-trivial decision
goes through the operator.

---

## 7. Ring 3: Compositional AI

Ring 3 (`service-slm`, the Doorman) is where AI inference is intended to run.
Ring 3 is architecturally optional — Rings 1 and 2 function fully without it.
The design intent is that Ring 3 queries Ring 2, which reads Ring 1, forming
a layered composition.

Ring 3 operating at the session layer is not a Ring 1 exception. A query
from Ring 3 to Ring 1 data (via Ring 2) uses the same deterministic MCP
interface that any other Ring 2 client uses. Ring 3 cannot bypass Ring 1's
write discipline; it reads finished Ring 1 records via the same API as
everything else.

The compositional model is: deterministic ingest (Ring 1) → deterministic
processing (Ring 2) → optional intelligence at query time (Ring 3). Intelligence
is value-add, not load-bearing, and never touches the immutable write path.

---
