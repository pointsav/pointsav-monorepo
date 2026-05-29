---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-data
target_repo: content-wiki-documentation
target_path: ./
target_filename: topic-identity-ledger-schema.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
created: 2026-05-29
cites: []
research_trail:
  source_files:
    - service-people/src/person.rs
    - service-people/src/acs.rs
    - service-people/src/people_store.rs
    - service-people/src/mcp.rs
    - service-people/src/fs_client.rs
  verified_against_commit: "815e11c (feat(service-people): ACS engine absorbed)"
notes_for_editor: >
  Sections 1–7 are current-fact (schema implemented and tested in session 4,
  31 tests pass). Section 8 (forward-looking) uses "planned/intended" language
  per BCSC posture. Do not promote forward-looking material to present tense.
  The Spanish skeleton is in topic-identity-ledger-schema.es.draft.md.
---

# Topic: Identity Ledger Schema

The identity ledger is the deterministic identity primitive at the centre of
Ring 1 data ingest. It defines three record types — Person, Anchor, and Claim
— that together represent who is known to the system, how that identity was
observed, and what attributes have been recorded about it. All three write
through `service-fs`, producing an immutable, append-only audit trail with
no AI involvement at any stage.

---

## 1. The Identity Primitive

Every identity in Ring 1 is a UUID version 5 derived from a lowercase-normalised
email address:

```
id = UUIDv5(NAMESPACE_DNS, lowercase(primary_email))
```

This derivation is deterministic: the same email address always produces the
same UUID on any machine, in any language, at any time. There is no random seed,
no AI classification, no lookup against an external service. Two systems that
independently ingest the same email address will arrive at the same UUID.

This property is the foundation of the identity primitive's composability
guarantee: a UUID produced by `service-people` can be referenced by
`service-email`, `service-input`, or any Ring 2 component without a shared
identity registry. The derivation is the registry.

---

## 2. The Person Record

The Person record is the primary identity object. It is created by the
`identity.append` MCP tool and stored in the `service-people` in-process
`PeopleStore` and written through to `service-fs`.

```
Person {
    id:              UUIDv5(NAMESPACE_DNS, lowercase(primary_email))
    name:            String
    primary_email:   String  // always lowercase-normalised
    email_aliases:   Vec<String>
    organisation:    Option<String>
    created_at:      DateTime<UTC>  // RFC3339
    updated_at:      DateTime<UTC>  // RFC3339
}
```

The `id` field is always derived; it is never assigned by the caller. The
`primary_email` field is always stored in lowercase regardless of how it was
supplied. These invariants are enforced in `Person::new()` — there is no
constructor that bypasses them.

---

## 3. The Anchor Record

An Anchor is an immutable observation that an email address was seen in a
specific context. It is produced automatically by the `identity.scan_text`
MCP tool when an email regex match is found in a block of text.

```
Anchor {
    target_uuid:   String  // UUIDv5(NAMESPACE_DNS, lowercase(email))
    anchor_source: String  // the email address as observed
    timestamp:     String  // RFC3339
}
```

An Anchor does not assert that the email belongs to a named individual. It
records the observation of an email address and derives the UUID that would
correspond to that address. Anchors are strictly append-only: the system never
modifies or retracts an Anchor once written.

---

## 4. The Claim Record

A Claim is an attribute observation with provenance. It is produced by
`identity.scan_text` alongside each Anchor and records what attribute was
inferred and from what source.

```
Claim {
    claim_id:         String  // UUIDv4 — unique per invocation
    target_uuid:      String  // the identity being annotated
    attribute:        String  // e.g. "email"
    value:            String  // the observed value
    confidence_score: f32     // 1.0 for regex-verified; <1.0 reserved for future
    source_id:        String  // caller-supplied provenance identifier
    timestamp:        String  // RFC3339
}
```

The `claim_id` is a UUIDv4 (random, not derived) — it is unique per Claim
invocation. The `confidence_score` is `1.0` for all Claims produced by the
current implementation because the only extraction method in use is a
deterministic regular expression. A score below `1.0` is reserved for
future extraction methods operating under SYS-ADR-07 constraints.

---

## 5. Conflict Detection

When a Person record is appended to `PeopleStore`, the store checks whether
any email in the record (primary or alias) is already bound to a different
UUID. If so, the store returns a `ConflictingIdentity` error rather than
silently merging or overwriting.

```
PeopleStoreError::ConflictingIdentity {
    email:       String  // the email that triggered the conflict
    existing_id: Uuid    // the UUID already bound to that email
    new_id:      Uuid    // the UUID that the new record would assign
}
```

Conflicts are surfaced to the caller — they are never automatically resolved.
This implements the F12 requirement (SYS-ADR-10): when deterministic matching
surfaces ambiguity, the system surfaces it to the operator rather than
resolving it with heuristics or AI inference.

---

## 6. SYS-ADR-07 Compliance: Zero AI in Ring 1

The identity ledger schema is designed so that every operation is
deterministic and verifiable without model state:

- Email extraction uses a single compiled regular expression
  (`(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}`).
- UUID derivation uses the UUIDv5 algorithm (SHA-1 with a fixed namespace
  UUID — deterministic, not learned).
- Confidence scores are assigned by rule (`1.0` for regex match), not by
  a model.
- Conflict detection compares UUIDs by equality — no fuzzy matching,
  no embedding similarity, no threshold tuning.

No model weights are present in `service-people`. No network call is made to
an inference endpoint during identity ingest. These are hard constraints of
SYS-ADR-07, not implementation preferences.

---

## 7. WORM Persistence

All three record types — Person, Anchor, and Claim — are persisted by writing
through to `service-fs` via `FsClient`. The write path uses:

- HTTP POST to `/v1/append` on the service-fs instance
- `X-Foundry-Module-ID` header set to the value of `PEOPLE_MODULE_ID`
  (environment variable, e.g. `foundry-workspace`)
- JSON-serialised record body

This means every identity observation is written to the WORM ledger at
the moment it is ingested. The records cannot be deleted or modified after
the fact. The append-only guarantee comes from service-fs's D4
atomic-write discipline and linear SHA-256 hash chain.

Three write paths exist:

1. `identity.append` (MCP tool) — writes a Person record
2. `identity.scan_text` (MCP tool) — writes one Anchor and one or more
   Claims per email address found in the input text
3. (no direct write path for Anchor or Claim in isolation — they are
   always produced as a pair by `scan_text`)

---

## 8. Forward-looking: Ring 2 Identity Extension

The schema described above is the Ring 1 deterministic baseline. Planned Ring 2
extensions include:

- **Cross-tenant identity sharing** — a query interface that looks up
  identities across module boundaries, intended for use by Ring 2 components
  (`service-extraction` et al.) that need to correlate entities observed in
  different ingestion contexts.
- **Embedding-based fuzzy matching** — an optional similarity layer, intended
  to run in Ring 2, that may suggest candidate identity merges for operator
  review. This is gated by SYS-ADR-07: any inference-based suggestion must
  surface to the operator for confirmation; it may not write to the Ring 1
  WORM ledger without explicit operator action.

These extensions are planned and intended; they are not present in the current
implementation.

---
