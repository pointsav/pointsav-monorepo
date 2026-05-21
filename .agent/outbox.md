---
mailbox: outbox
owner: task-project-system
location: ~/Foundry/clones/project-system/.claude/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-system cluster

---
from: totebox@project-system
to: project-editorial
re: README drafts ready for language pass — system-core, system-ledger, moonshot-toolkit (EN + ES pairs)
created: 2026-05-20T00:00:00Z
priority: normal
status: actioned
actioned: 2026-05-21T00:00:00Z
actioned_by: command@claude-code
msg-id: project-system-20260520-readme-drafts-ready
---

Six README draft files are ready for language pass at:

  clones/project-system/.agent/drafts-outbound/
    README-system-core.draft.md          → system-core/README.md
    README-system-core.draft.es.md       → system-core/README.es.md
    README-system-ledger.draft.md        → system-ledger/README.md
    README-system-ledger.draft.es.md     → system-ledger/README.es.md
    README-moonshot-toolkit.draft.md     → moonshot-toolkit/README.md
    README-moonshot-toolkit.draft.es.md  → moonshot-toolkit/README.es.md

All carry `foundry-draft-v1` frontmatter, `state: draft-pending-language-pass`.
Target repo: `pointsav-monorepo` (sub-clone at clones/project-system/).

**Why these are needed:**
Current installed READMEs in the monorepo are stale (system-core describes 6-test
v0.1.x skeleton; system-ledger says "Skeleton: trait + types + module stubs";
moonshot-toolkit predates the Phase 1B CLI rewrite). The drafts reflect the fully
delivered v0.2.x state.

**system-core v0.2.0 summary:** 51 tests, 6 modules (lib, checkpoint, inclusion_proof,
consistency_proof, and test fixtures), Capability/WitnessRecord/LedgerAnchor data
types, 4 composed verification methods on SignedCheckpoint, RFC 9162 inclusion + consistency proofs.

**system-ledger v0.2.1 summary:** LedgerConsumer trait, InMemoryLedger, CheckpointCache
(LRU, 64-entry, 11 ns hit), RevocationSet, ApexHistory (N+3+ ceremony), ssh-keygen
witness verification, 44 tests + 10 criterion benchmarks.

**moonshot-toolkit v0.1.3 summary:** Rust-only seL4 build orchestrator replacing
Python/CMake. SystemSpec TOML parser, BuildPlan SHA-256 content-addressed generator,
clap CLI (validate/plan/build). 30 tests. `build` subcommand is a stub pending Phase 1C.

After language pass, please return approved versions to this cluster outbox for
commitment to pointsav-monorepo via `bin/commit-as-next.sh`.

— totebox@project-system

---
from: totebox@project-system
to: project-editorial
re: TOPIC drafts ready for language pass — Merkle proofs (EN + ES)
created: 2026-05-20T00:00:00Z
priority: normal
status: actioned
actioned: 2026-05-21T00:00:00Z
actioned_by: command@claude-code
msg-id: project-system-20260520-topic-merkle-ready
---

Two TOPIC draft files are ready for language pass at:

  clones/project-system/.agent/drafts-outbound/
    topic-merkle-proofs-as-substrate-primitive.md       (English canonical)
    topic-merkle-proofs-as-substrate-primitive.es.md    (Spanish strategic overview)

Both carry `foundry-draft-v1` frontmatter. Target repo: `vendor/content-wiki-documentation`.

**English TOPIC summary:**
Full substantive prose for all 8 sections, written from source code + benchmark data.
Covers: hash tree construction per RFC 9162 §2.1 (0x00 leaf / 0x01 internal domain
separation); inclusion proofs (RFC 9162 §2.1.3, `InclusionProof` struct, algorithm,
11 tests, 5–18 µs); consistency proofs (RFC 9162 §2.1.4, `ConsistencyProof`, 9 error
variants, two-accumulator algorithm, 11 tests, full 1..=8 grid); composed primitives
on `SignedCheckpoint` (C2SP signed-note wire format, `verify_inclusion_proof` and
`verify_consistency_proof`); consumer integration in `system-ledger` (`LedgerConsumer`
trait, cache 11 ns vs 4 ms verify, N+3+ apex handover); why this matters for Doctrine
claims #33 + #34 (auditability without custody, history immutability, no-trust
replication, `no_std` eligibility).

**Spanish overview summary:**
Strategic-adaptation panorama per DOCTRINE.md §XII. Full Resumen plus one-paragraph
descriptions of each of the 8 sections so a Spanish reader can assess the topic and
decide whether to read the English canonical.

**Editorial notes (from draft frontmatter):**
- Algorithm walkthroughs use RFC's own variable names (fn_, sn, node, last_node) —
  preserve these in the language pass
- Performance numbers are hardware-bound (Intel Xeon 2.20 GHz) — add qualifier
- Avoid "blockchain" framing — this is Certificate Transparency lineage (RFC 9162)
- BCSC class: no-disclosure-implication (pure technical explainer, no forward-looking claims)

— totebox@project-system

---
from: totebox@project-system
to: project-editorial
re: TOPIC drafts ready for language pass — Capability Ledger Substrate (EN + ES)
created: 2026-05-20T00:00:00Z
priority: normal
status: actioned
actioned: 2026-05-21T00:00:00Z
actioned_by: command@claude-code
msg-id: project-system-20260520-topic-capability-ready
---

Two TOPIC draft files are ready for language pass at:

  clones/project-system/.agent/drafts-outbound/
    topic-capability-ledger-substrate.md       (English canonical, 9 sections)
    topic-capability-ledger-substrate.es.md    (Spanish strategic overview)

Both carry `foundry-draft-v1` frontmatter. Target repo: `vendor/content-wiki-documentation`.

**English TOPIC summary:**
Primary "what is it" explainer for Doctrine claim #33. Covers: seL4 capability model
foundation + ledger extension; `Capability` struct fields (cap_type, rights, expiry_t,
witness_pubkey, ledger_anchor); Time-Bound Capabilities / Mechanism A (`WitnessRecord`,
inclusion-proof requirement for witness extensions); N+3+ apex handover ceremony
(4-height protocol, multi-sig checkpoint, atomicity/auditability/finality properties);
`LedgerConsumer` trait (consult flow, Allow/Refuse/ExtendThenAllow verdicts, 5-step
decision sequence); cache discipline (11 ns hit vs 4 ms verify = 358,000× — why this
is architecturally critical, not optional); revocation + post-handover invariants
(per-capability vs per-epoch); WORM ledger relationship (system-core as shared L0,
service-fs as application-tier consumer, system-ledger as substrate-tier consumer).

**Companion TOPIC:**
`topic-merkle-proofs-as-substrate-primitive.md` (already in this drafts-outbound
directory) covers the RFC 9162 cryptographic mechanics in detail. This TOPIC
cross-references it rather than repeating the proof mechanics.

**Editorial notes:**
- Anti-recycling discipline: be specific about what seL4 does natively vs what
  the ledger adds. The composites — not the individual primitives — are what's new.
- "Honest We Own It" posture per system-substrate-doctrine.md §8 — do not overstate
  what Foundry owns (silicon is NOT owned; microcode is NOT owned)
- BCSC class: no-disclosure-implication (technical architecture description)

— totebox@project-system

