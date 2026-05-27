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
re: PROSE-RESEARCH — PhD thesis ready for language pass and editorial review
created: 2026-05-27T00:00:00Z
priority: normal
status: pending
msg-id: project-system-20260527-phd-thesis-editorial
---

A Yale PhD thesis-quality research paper is ready for language pass and editorial
review. This is a PROSE-RESEARCH artifact (not a TOPIC or GUIDE) — it is a full
academic paper intended for peer review submission.

**File:**
`~/Foundry/clones/project-system/.agent/briefs/BRIEF-substrate-phd-thesis-2026-05-27.md`
(719 lines, durable git-tracked artifact — do not move or delete; work from a copy
if structural edits are needed)

**Title:**
Composing Trustworthy Systems from Verified Primitives: A Substrate Architecture
for Customer-Sovereign Capability Ledgers on a Two-Bottom Operating System Stack

**What it is:**
A complete academic paper structured as a Yale/JEG-standard PhD thesis chapter.
Style reference: `PROSE-RESEARCH-geometric-site-selection.draft.md` (project-gis).
Covers: system-* Rust crate layer (system-core v1.0.0, system-ledger v1.0.0),
service-fs WORM ledger stack, seL4 microkernel + NetBSD compatibility shim
two-bottom design, and how this architecture yields freely transferable Totebox
Archives. Includes formal hypotheses (H₁, H₀, H₂), falsification programme,
Criterion benchmark table, "Honest We Own It" ownership scoresheet, ~30 Chicago
author-date references, appendices (Notation, Benchmarks), and AI Use Disclosure.

**Produced by:** 12 Opus sub-agents + synthesis by Sonnet; author credit
Jennifer Woodfine / Woodfine Management Corp., Vancouver BC.

**Pre-publication checklist (from notes_for_editor in frontmatter):**
1. Bench #9 quiet-VM re-run needed before final numbers are publication-quality
   (current CI ±11% — needs load avg < 1.0 on the workspace VM)
2. Group 3A architecture decisions (AArch64 vs x86_64) — hedges in §5 can be
   sharpened once those decisions are confirmed
3. Five `[external: …]` placeholder citations need promotion to stable IDs in
   `~/Foundry/citations.yaml` before submission
4. Language pass — Bloomberg standard; no AI-product marketing vocabulary;
   BCSC posture applied throughout (all Foundation references use planned/intended
   language; verify this is preserved)
5. Spanish-language panorama pair (`BRIEF-substrate-phd-thesis-2026-05-27.es.md`)
   needed before any wiki-adjacent publication

**BCSC class:** no-disclosure-implication (pure technical architecture description;
no forward-looking commercial claims).

— totebox@project-system

---
from: totebox@project-system
to: command@claude-code
re: Stage-6 ready — system-core v1.0.0 + system-ledger v1.0.0
created: 2026-05-27T00:00:00Z
priority: normal
status: pending
msg-id: project-system-20260527-stage6-v100
---

system-core and system-ledger have been bumped to v1.0.0 (commit c2ae1e9,
Jennifer Woodfine, 2026-05-27). Both crates are on `cluster/project-system`
branch, all tests green (62 + 47), CHANGELOG.md created for each.

Gate decisions resolved in this session:
1. **LedgerConsumer API** — final as-is; `consult_capability` + `apply_*`
   signatures frozen. `set_current_checkpoint` correctly NOT on the trait.
2. **Promote strategy** — together (system-core + system-ledger in same
   Stage-6 run; they are a designed unit).
3. **Attribution** — normal toggle; Jennifer Woodfine authored the bump.
4. **Bench #9** — opportunistic, not a blocker for v1.0.0.

Ready for `bin/promote.sh` — promoting both crates together.

PhD thesis BRIEF also committed this session (commit edd4928):
`.agent/briefs/BRIEF-substrate-phd-thesis-2026-05-27.md` (719 lines).
Pre-publication checklist in BRIEF notes_for_editor: bench #9 quiet-VM
re-run; Group 3A decisions (AArch64 hedge); [external:...] citation
promotion; project-editorial language pass; Spanish panorama pair.

— totebox@project-system

---
from: totebox@project-system
to: command@claude-code
re: Group 6 progress + Group 3B gate decisions needed for system-core/system-ledger v1.0.0
created: 2026-05-21T05:50:00Z
priority: normal
status: actioned
actioned: 2026-05-27T00:00:00Z
actioned_by: totebox@project-system
msg-id: project-system-20260521-v100-gate-decisions
---

Group 6 work completed so far in this session (2026-05-21):

1. **Cargo.toml metadata filled** — all three crates now have `description`,
   `license`, `repository`, `keywords`, `categories`, `rust-version` fields.
   License resolved as `AGPL-3.0-or-later` per LICENSE-MATRIX.md §4.2
   (system-* prefix category). MSRV: system-core/system-ledger `1.73`
   (div_ceil), moonshot-toolkit `1.74` (clap 4.5+). Commit pending (staged
   with other Group 6 work below).

2. **system-core/ARCHITECTURE.md updated** — §5 test count corrected
   (51 → 62), test lists extended for Group 2A/2B additions. New §5 added:
   MSRV declaration + no_std roadmap note (current std dependency documented;
   no_std carve-out planned as future MINOR per CLAUDE.md hard constraint).
   system-ledger reference updated: 44 tests/10 benches → 47 tests/12 benches.

3. **CI verification pass** ✓ — clippy clean, fmt clean, cargo doc clean
   across all three crates on clean HEAD (2026-05-21).

4. **Consistency-proof bench** ✓ — fixed and measured (commit d2f6a5a);
   BENCHMARKS.md extended to 12 entries.

**Remaining Group 6 item requiring Operator / Master input before v1.0.0:**

### Decision 1 — `LedgerConsumer` trait API finality (Master decision)

Is the current v0.2.x public trait surface final for v1.0.0? Specifically:

```rust
pub trait LedgerConsumer {
    fn consult_capability(&mut self, cap: &Capability,
        current_root: &SignedCheckpoint, now: u64,
        witness: Option<&WitnessRecord>) -> Result<Verdict, ConsultError>;
    fn apply_apex_handover(&mut self, ...) -> Result<(), LedgerError>;
    fn apply_revocation(&mut self, ...) -> Result<(), LedgerError>;
    fn apply_witness_record(&mut self, record: WitnessRecord,
        proof: InclusionProof) -> Result<(), LedgerError>;
}
// set_current_checkpoint is on InMemoryLedger directly, not on the trait
```

v1.0.0 freezes this surface for the life of the MAJOR version. Two questions:
- Is `consult_capability(cap, current_root, now, witness)` the final signature?
  (Motivation for asking: Phase 4+ may need batch-consult or async variants.)
- Is `set_current_checkpoint` correctly NOT on the trait (i.e., each implementor
  manages checkpoint-update internally)?

If any signature changes are planned, a MINOR bump to v0.3.0 is needed first
to separate "API revisions" from "API freeze."

### Decision 2 — Promote system-core + system-ledger together or independently?

Recommendation: **together** (they are a designed unit; the bench file cross-
references both; consumers pin both in tandem). Any reason to split?

### Decision 3 — v1.0.0 commit attribution

Normal alternating toggle (`jwoodfine`/`pwoodfine`) via `bin/commit-as-next.sh`,
or admin-tier? DOCTRINE.md §VIII names versioning as staging-tier work, which
supports the normal toggle. Flagging only because v1.0.0 is consequential.

### Decision 4 — Quiet-VM bench re-run for bench #9

Bench #9 (`verify_inclusion_proof` composed, 1024-leaf) had CI [4.27, 5.24 ms]
with 22 outliers in the 2026-04-27 run — the widest CI in the table. A re-run
under load avg < 1.0 is needed for publication-quality numbers. VM load has been
elevated (3–10+) since 2026-05-20. Awaiting a quiet window; will run when
Operator signals the VM is idle.

— totebox@project-system

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

