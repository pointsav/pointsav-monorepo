# Brief: topic-source-of-truth-inversion.md — bulk draft

**target**: Author a `foundry-draft-v1` bulk draft establishing source-of-truth inversion as a generalised SUBSTRATE pattern across four PointSav applications — wiki engine, service-extraction, app-workplace-presentation, and app-workplace-proforma.
**target_files**:
- Write: `/srv/foundry/clones/project-knowledge/.claude/drafts-outbound/topic-source-of-truth-inversion.draft.md`
- Append JSONL event: `/srv/foundry/data/training-corpus/apprenticeship/prose-edit/pointsav/draft-2026-04-28-topic-source-of-truth-inversion.jsonl`
**expected_output**: One `.draft.md` file with valid `foundry-draft-v1` frontmatter (`state: draft-pending-language-pass`, `audience: vendor-public`, `bcsc_class: no-disclosure-implication`, `language_protocol: PROSE-TOPIC`) and a structured bulk body per §2.2 discipline. No `.es.md` — bilingual pair is project-language's responsibility. One JSONL line with `event: draft-created`.
**max_response_lines**: 220
**model_tier**: sonnet
**parallelisable**: no (writes)
**confidence_gate_passes**: borderline — doctrine-grade structural content (Substrate Substitution claim #29 + Two-Bottoms claim #34 referenced); Sonnet handles well-specified structural exposition; architectural framing is prescribed in this brief and in sibling draft §2; confidence is borderline only because claim #34 applies at the kernel layer whereas the TOPIC is application-layer — the brief instructs the sub-agent to reference #34 at its correct level of abstraction and not over-extend it.
**layer_scope**: task
**anti_slop_check**: this draft satisfies the wiki-leg of the project-knowledge Tetrad (manifest line `topic-source-of-truth-inversion.md` listed as future planned TOPIC) and unblocks project-language pickup of a substantive cross-application pattern TOPIC.
**dependencies**:
- Read: `/srv/foundry/clones/project-knowledge/.claude/drafts-outbound/topic-app-mediakit-knowledge.draft.md` §2 (existing source-of-truth-inversion treatment — generalise beyond wiki)
- Read: `/srv/foundry/clones/project-knowledge/.claude/drafts-outbound/topic-substrate-native-compatibility.draft.md` frontmatter + notes_for_editor (sibling; cross-reference but do not duplicate)
- Read: `/srv/foundry/DOCTRINE.md` claim #29 (Substrate Substitution) and claim #34 (Two-Bottoms Sovereign Substrate) — grep for those claim numbers
- Read: `/srv/foundry/clones/project-knowledge/.claude/manifest.md` (cluster context, confirms target_path candidates)
- Read: `/srv/foundry/conventions/cluster-wiki-draft-pipeline.md` §2 + §2.2 (frontmatter schema)

## Specification

The TOPIC must establish **source-of-truth inversion** as a named Foundry substrate pattern: the principle that in each application, one storage layer is declared canonical (Git for structured text, signed Merkle log for immutable records, CRDT server relay for ephemeral session state), a second layer is a derived view (the running binary's in-memory index, rendered HTML, search index, wikilink graph, in-browser presentation state), and a third layer — when collab is enabled — is session-ephemeral and deliberately does not write back to canonical until a human commits.

For each of four applications, the draft must answer three questions: what is canonical, what is the view, and what is ephemeral. The four applications are:

1. **app-mediakit-knowledge (wiki engine)** — this case already appears in `topic-app-mediakit-knowledge.draft.md` §2; re-use that treatment as the anchor instance without reproducing it verbatim. Canonical: the markdown content tree in Git. View: rendered HTML, Tantivy search index, redb wikilink graph (Phase 4). Ephemeral: CRDT session state on the passthrough relay (Phase 2 Step 7, default-off). This case grounds the pattern with an already-live deployment.

2. **service-extraction (multi-author review pipeline)** — Ring 2 service. Canonical: the extraction event log committed to the immutable WORM ledger (service-fs, per infrastructure/local-fs/). View: the review queue derived from unreviewed ledger entries; per-reviewer verdict summary. Ephemeral: concurrent reviewer annotations before verdict commit. Source-of-truth inversion here means no reviewer's working state can corrupt another's; the ledger enforces total-order; the view re-derives on each query.

3. **app-workplace-presentation (deck collab)** — Canonical: the slide deck source committed to the customer's Git repo (vault pattern per DOCTRINE claim #29 applied to presentation layer). View: rendered slide frames served to browser clients. Ephemeral: CRDT multi-cursor collab state for real-time co-authoring sessions; does not persist between sessions without an explicit commit by a human author.

4. **app-workplace-proforma (table collab)** — Canonical: the proforma table committed as structured data (CSV or structured markdown with schema declaration) in the customer's Git repo. View: rendered table UI with computed fields. Ephemeral: CRDT cell-level collab state during shared editing sessions; same commit-gated persistence model as app-workplace-presentation.

The draft should open with a one-paragraph pattern statement naming the three layers by their structural roles (canonical / view / ephemeral), then work through the four applications in order, then close with a short section naming why this pattern matters for the BCSC continuous-disclosure posture (canonical = the disclosed state; the view is never the record) and for the Two-Bottoms Sovereign Substrate (claim #34 establishes that the same substrate binaries run on both bottoms — the source-of-truth inversion pattern is why: canonical state is kernel-agnostic signed Git; it doesn't matter which OS kernel the view process runs on).

The sub-agent must NOT: attempt register-discipline (no Bloomberg pruning — that is project-language's role), resolve citation IDs (inline URLs and `DOCTRINE.md claim #N` references are correct for bulk stage), generate a Spanish sibling `.es.md`, add structural conclusions about competitive positioning (structural description only per CLAUDE.md §6 "Structural positioning"), or write forward-looking information about planned features without "planned"/"intended" qualifiers.

Section structure proposal: §1 Pattern statement; §2 Application: wiki engine (anchor); §3 Application: service-extraction; §4 Application: app-workplace-presentation; §5 Application: app-workplace-proforma; §6 Why this pattern matters (BCSC posture + claim #34 connection).

## Acceptance criteria

- Valid `foundry-draft-v1` frontmatter with all required fields populated; `state: draft-pending-language-pass`
- `audience: vendor-public`, `bcsc_class: no-disclosure-implication`, `language_protocol: PROSE-TOPIC`
- `references:` list includes DOCTRINE.md claim #29, claim #34, `topic-app-mediakit-knowledge.draft.md`, `topic-substrate-native-compatibility.draft.md`, and `conventions/bcsc-disclosure-posture.md` at minimum
- `notes_for_editor:` block present and non-empty; identifies which sections are load-bearing for claim references and which can be pared for register
- All four applications addressed with explicit canonical / view / ephemeral breakdown
- §2 (wiki engine) cites the sibling draft rather than reproducing verbatim; directs reader to `topic-app-mediakit-knowledge.md` for full treatment
- §6 connects the pattern to BCSC posture (canonical = the disclosed state) and to claim #34 (kernel-agnostic canonical state enables the two-bottoms substrate)
- CRDT/collab references carry "planned" or "Phase N" qualifiers for features not yet live
- No forward-looking statements presented as current fact
- Draft body 80–160 lines; not padded, not thinned below substance
- JSONL `draft-created` event appended to the correct corpus path with `draft_id`, `cluster`, `tenant: pointsav`, `task_type: prose-edit`

## Risks / unknowns

- service-extraction and app-workplace-presentation/proforma may have limited committed architecture detail in the monorepo at this cluster-branch state; the sub-agent should describe the pattern structurally (what WILL be canonical/view/ephemeral per design intent) with "planned" qualifiers rather than treating absence of code as absence of design
- claim #34 (Two-Bottoms Sovereign Substrate) is a kernel-layer claim; the sub-agent must apply it at the correct level of abstraction (canonical storage is kernel-agnostic; that is the connection point) and must not stray into seL4/NetBSD implementation detail that belongs in `topic-substrate-native-compatibility.md`
- the CRDT collab layer (Phase 2 Step 7 for wiki; analogous phases for app-workplace-*) is not yet live as of 2026-04-27; all CRDT references must carry "planned" or "Phase N" qualifiers
- `target_path` for destination in `content-wiki-documentation/` is not yet decided (taxonomy decision pending operator ratification); draft frontmatter should list candidates (`architecture/`, `patterns/`, `applications/`) and delegate final placement to project-language per the existing precedent in sibling drafts
