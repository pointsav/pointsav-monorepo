---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-knowledge
target_repo: content-wiki-documentation
target_path: patterns/   # candidates: patterns/, architecture/, applications/ — project-language decides; taxonomy decision pending operator ratification per sibling-draft precedent
target_filename: source-of-truth-inversion.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-04-28T04:30:00Z
authored_by: task-project-knowledge (sub-agent, brief 03)
authored_with: sonnet-4-6
references:
  - DOCTRINE.md claim #29 (Substrate Substitution)
  - DOCTRINE.md claim #34 (Two-Bottoms Sovereign Substrate)
  - clones/project-knowledge/.claude/drafts-outbound/topic-app-mediakit-knowledge.draft.md
  - clones/project-knowledge/.claude/drafts-outbound/topic-substrate-native-compatibility.draft.md
  - conventions/bcsc-disclosure-posture.md
  - conventions/cluster-wiki-draft-pipeline.md §2 + §2.2
  - vendor/pointsav-monorepo/app-mediakit-knowledge/ARCHITECTURE.md §2 (source-of-truth inversion)
  - vendor/pointsav-monorepo/app-mediakit-knowledge/docs/PHASE-2-PLAN.md (collab passthrough relay)
  - infrastructure/local-fs/ (service-fs WORM ledger, live since v0.1.23)
  - conventions/three-ring-architecture.md (Ring 2 service-extraction)
  - conventions/disclosure-substrate.md
  - ni-51-102
  - osc-sn-51-721
notes_for_editor: |
  Load-bearing references:
  - §1 (pattern statement): the three-layer taxonomy (canonical / view / ephemeral) is the
    structural claim this TOPIC exists to name. Do not soften or collapse.
  - §2 (wiki engine anchor): cites topic-app-mediakit-knowledge.draft.md §2 for authority;
    preserve that cross-reference — this section should remain SHORT because the full treatment
    lives in the sibling TOPIC. Pruning to 3-4 sentences is correct here.
  - §3 (service-extraction): the WORM ledger = canonical claim is the load-bearing sentence;
    maps directly to Doctrine claim #29 applied to Ring 2. Preserve "total-order enforcement"
    language — that is the structural property.
  - §4 and §5 (app-workplace-*): source material is thin here (no committed architecture files
    for these projects as of 2026-04-27). All claims carry "planned" qualifier — check that
    qualifiers survive the register pass.
  - §6 (why it matters): the BCSC connection (canonical = the disclosed state) is load-bearing
    for the disclosure-substrate posture. The claim #34 connection (kernel-agnostic canonical
    = same os-* binary on either bottom) should be kept SHORT — claim #34 is a kernel-layer
    claim, not an application-layer claim; one sentence is the correct depth here. See Risks
    section of brief 03 for rationale.
  - DO NOT expand §6 into seL4/NetBSD implementation detail — that belongs in
    topic-substrate-native-compatibility.md.
  - "Planned" qualifiers are required on all CRDT/collab references and all app-workplace-*
    material; verify none were dropped during register pass.
  - target_path is a candidate list; do not resolve final placement — delegate to project-language.
---

# Source-of-truth inversion

## §1 Pattern statement

Source-of-truth inversion is a named Foundry substrate pattern. In each
PointSav application, one storage layer is declared canonical — the
authoritative record that is committed, signed, replicated, and disclosed.
A second layer is a derived view — the running process's in-memory index,
rendered output, or computed summary — rebuilt deterministically from the
canonical record on demand, discardable without loss. A third layer, when
collaborative editing is enabled, is session-ephemeral: it exists for the
duration of a shared editing session and does not write back to canonical
until a human author makes a deliberate commit. The pattern recurs across
the wiki engine, the Ring 2 extraction pipeline, and the planned
app-workplace-presentation and app-workplace-proforma applications. In each
case, the choice of what is canonical follows the same structural logic:
the layer with the longest durability requirement, the strongest audit
obligation, and the cleanest replication story is canonical. Everything
else is derived.

## §2 Application: wiki engine (anchor instance)

The full treatment of source-of-truth inversion in the wiki engine appears
in `topic-app-mediakit-knowledge.md` (see §2 of that TOPIC). The summary
here: **git is canonical; the running binary is a view; the CRDT passthrough
relay (Phase 2 Step 7, planned, default-off) is session-ephemeral**.

The Tantivy full-text search index, the redb wikilink graph (planned, Phase
4), and all rendered HTML are derived on demand from the markdown tree on
disk. Any of those derived artefacts can be discarded and rebuilt; none of
them is backed up; none is disclosed. The git tree is what is backed up,
replicated, signed, and disclosed. The binary cannot accumulate state that
the git tree does not have.

This case grounds the pattern with a live deployment:
`https://documentation.pointsav.com` has served this substrate since
2026-04-27.

## §3 Application: service-extraction (Ring 2 multi-author review pipeline)

`service-extraction` is the Ring 2 service that runs the multi-author
document review pipeline. The source-of-truth mapping here is:

**Canonical**: the extraction event log committed to the WORM immutable
ledger managed by `service-fs` (live on the workspace VM since v0.1.23,
binding `127.0.0.1:9100`, ledger root at `/var/lib/local-fs/ledger/`). An
extraction event is durably sequenced the moment it is appended to the
ledger; the ledger enforces total order over all events. Ledger entries are
not modifiable after the fact — that is what WORM (Write Once Read Many)
means structurally, not just operationally. The WORM ledger as canonical
storage is an instance of DOCTRINE claim #29 (Substrate Substitution)
applied to Ring 2: instead of a mutable relational database as the authority
for review state, the substrate is an append-only signed log.

**View**: the review queue shown to each reviewer is derived from the set
of ledger entries that have not yet received a verdict commit. The
per-reviewer verdict summary is derived similarly. Neither the queue nor
the summary is stored separately — both re-derive on each query from the
ledger. The derivation is deterministic: the same ledger produces the same
queue and summary every time it is queried, because the ledger is immutable
and total-ordered.

**Ephemeral**: reviewer annotations made before a verdict commit are
session-ephemeral. One reviewer's working annotations cannot see or corrupt
another reviewer's working annotations, because those annotations have not
yet been committed to the canonical ledger. Concurrent reviewers work
against their own in-process state; the ledger reconciles when a verdict
commit lands. The total-order enforcement of the ledger is the substrate
mechanism that makes concurrent review safe without coordination locks.

## §4 Application: app-workplace-presentation (deck collaboration)

`app-workplace-presentation` is a planned application for collaborative
slide-deck authoring. The intended source-of-truth mapping follows the same
pattern:

**Canonical**: the slide deck source, intended to be committed to the
customer's Git repository under the vault pattern (DOCTRINE claim #29
applied to the presentation layer — the customer's Git repository is the
canonical authority for presentation content, not a proprietary document
server). A commit to the deck's Git repository is the disclosure event; the
deck's git history is the audit record.

**View**: rendered slide frames served to browser clients, computed from
the committed deck source on demand. The rendered frames are not stored
persistently; they are rebuilt from the committed source on each request.

**Ephemeral**: CRDT multi-cursor collaboration state for real-time
co-authoring sessions is planned as session-ephemeral. Participating
authors see each other's edits in real time via a passthrough relay
analogous to the wiki engine's Phase 2 Step 7 relay design. That
session-state does not persist between sessions without an explicit commit
by a human author. When all authors leave the session, the ephemeral state
is discarded; the canonical record in Git is unchanged. This matches the
wiki engine's collab design: the relay is passthrough, not a persistent
document server. Note: the CRDT collab layer for app-workplace-presentation
is planned; it is not yet implemented as of 2026-04-27.

## §5 Application: app-workplace-proforma (table collaboration)

`app-workplace-proforma` is a planned application for collaborative
structured-data editing — proforma tables, financial schedules, and
structured documents used in regulated business contexts. The source-of-truth
mapping:

**Canonical**: the proforma table, intended to be committed as structured
data (CSV or structured markdown with a schema declaration) in the
customer's Git repository. The schema declaration travels with the data,
making the committed artefact self-describing. The customer's Git repository
is the canonical authority; commit signing follows the same vault pattern
as app-workplace-presentation.

**View**: the rendered table UI with computed fields — totals, cross-row
references, conditional formatting — derived from the canonical structured
data on each render. Computed fields are not stored in the canonical record;
they re-derive. This is important for proforma contexts where a formula
change must produce consistent derived values everywhere the formula is
referenced, with no cached stale values possible because the cache is not
canonical.

**Ephemeral**: CRDT cell-level collaboration state during shared editing
sessions is planned as session-ephemeral, following the same commit-gated
persistence model as app-workplace-presentation. A collaborating author
holds uncommitted cell edits in CRDT session state; other authors see those
edits in real time via the passthrough relay; nothing persists to the
canonical record until an explicit commit. Note: the CRDT collab layer for
app-workplace-proforma is planned; it is not yet implemented as of 2026-04-27.

The proforma application is particularly sensitive to the distinction
between canonical and derived. In regulated document contexts, the
authoritative number is the one committed and signed in Git — not the
rendered value in a browser tab that may reflect unsaved session edits.
The pattern enforces that distinction structurally: the only way to change
the canonical record is a commit.

## §6 Why this pattern matters

**BCSC continuous-disclosure posture.** In each application, canonical is
the disclosed state. Per `conventions/bcsc-disclosure-posture.md` and NI
51-102 continuous-disclosure requirements, the record that is disclosed
is the record that is signed, committed, and replicated — not the
rendered view, not the search index, not the session-ephemeral CRDT buffer.
Source-of-truth inversion enforces this by construction: the substrate
cannot accidentally disclose a view-layer artefact as authoritative because
the view is explicitly not the record. The audit trail for any disclosed
claim is a `git log`; the claim lives in a signed commit. This
property is not achieved by policy — it is a structural consequence of
the storage layer designation.

**DOCTRINE claim #34 (Two-Bottoms Sovereign Substrate).** Claim #34
establishes that the same `os-*` binaries run on both substrate bottoms
(native seL4 and compatibility NetBSD) via a thin shim. The application-layer
consequence is that canonical storage must be kernel-agnostic: a signed
Git tree and a signed WORM ledger entry are valid records regardless of
which OS kernel the view process runs on. Source-of-truth inversion
achieves this — by keeping the canonical record as signed structured data
(git commits, ledger entries) and the view as a derived process, the
substrate binaries can move between bottoms without the canonical record
changing its identity or requiring re-derivation of the disclosed state.
The deeper seL4/NetBSD design is treated in `topic-substrate-native-compatibility.md`;
the connection here is only the kernel-agnostic canonical storage claim.

The pattern is not application-specific. It recurs because the same
structural logic applies wherever a substrate needs a clear audit record,
clean replication, and collab that does not corrupt the canonical state.
The four applications above are four instances of one pattern, not four
independent design decisions.
