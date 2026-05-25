---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-proofreader
target_repo: vendor/content-wiki-documentation
target_path: ./
target_filename: topic-editorial-pipeline-three-stages.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-04-28T01:30:00Z
authored_by: task-project-proofreader (brief-1-sonnet-subagent)
authored_with: sonnet-4-6
references:
  - ~/Foundry/conventions/language-protocol-substrate.md
  - ~/Foundry/clones/project-proofreader/pointsav-monorepo/service-proofreader/ARCHITECTURE.md
  - ~/Foundry/clones/project-proofreader/pointsav-monorepo/service-proofreader/src/pipeline/
notes_for_editor: |
  Skeleton stage. Section headings + (draft-pending — substance
  follows in milestone N+1) markers per Tetrad upgrade message.

  This TOPIC describes the technical architecture of service-proofreader's
  three-stage pipeline. The audience is vendor-public engineers evaluating
  the pattern — they need enough detail to understand how the stages compose,
  why the ordering is deliberate, and how graceful degradation works when
  individual stages are unavailable.

  When refining:
  - Apply Bloomberg-article register throughout
  - Resolve inline crate paths (e.g., src/pipeline/) to citation IDs
    against ~/Foundry/citations.yaml where possible; register new entries
    for ARCHITECTURE.md if not yet present
  - Generate the bilingual .es.md overview per DOCTRINE §XII (Spanish
    overview, not 1:1 translation)
  - Pare to approximately 800-1200 words in the English version
  - The inline JSON flow between Stage 2 and Stage 3 is a key technical
    detail — ensure it survives the register pass with sufficient precision
    for an engineer to implement against
  - Check that no banned-vocabulary terms have crept in during drafting

  Citations to register if not yet present:
  - DOCTRINE.md claim #37 (Tetrad wiki leg; the substrate this TOPIC
    is part of)
  - DOCTRINE.md claim #35 (Reverse-Funnel Editorial Pattern; operator
    verdict closes the apprenticeship loop referenced in the final section)
  - service-proofreader ARCHITECTURE.md (internal reference; register
    with a workspace-local citation ID)

  Suggested length when substance lands: 800-1200 words English.
---

# The three-stage editorial pipeline

(draft-pending — substance follows in milestone N+1)

`service-proofreader` composes every `/v1/proofread` request through
three discrete stages executed in order. Substantive coverage forthcoming:
the design rationale for a pipeline over a single monolithic pass; how
each stage's output feeds the next; and the wire contract callers rely on.

## Why three stages, in this order

(draft-pending — substance follows in milestone N+1)

The ordering is deliberate: cheap deterministic rules run first so that
expensive model calls never see input that a rule-pass could have
handled. Substantive coverage forthcoming: the cost-discipline argument
(Stage 1 terminates the request early on clear violations); the
correctness argument (Stage 2 mechanical findings prevent Stage 3 from
re-discovering grammar errors the model would handle inconsistently);
the audit argument (each stage emits a structured finding set that is
independently verifiable by the operator).

## Stage 1 — Banned-vocabulary scan (deterministic, rule-driven)

(draft-pending — substance follows in milestone N+1)

Stage 1 is a pure text scan against the workspace's banned-vocabulary
lists. It is deterministic, has no external dependency, and completes
in sub-millisecond time regardless of input length. Substantive coverage
forthcoming: the per-family vocabulary lists (PROSE / COMMS / LEGAL /
TRANSLATE); how a match produces a `Severity::Banned` flag with span
and replacement-hint; the design choice to flag only — never to
silently rewrite.

## Stage 2 — LanguageTool 6.6 mechanical pass (Docker companion)

(draft-pending — substance follows in milestone N+1)

Stage 2 calls the LanguageTool 6.6 HTTP API running as a Docker
companion service. It handles spelling, grammar, and style rules that
are too numerous to enumerate as a static vocabulary list. Substantive
coverage forthcoming: the Docker companion deployment model
(co-located, not remote); the response-normalisation step that maps
LanguageTool rule IDs to the proofreader's internal `Finding` type;
how Stage 2 findings are serialised as inline JSON for Stage 3
consumption.

## Stage 3 — Doorman generative pass (service-SLM via Tier A/B/C)

(draft-pending — substance follows in milestone N+1)

Stage 3 routes through the Doorman (service-slm) to one of three
compute tiers: Tier A (OLMo 3 7B Q4 local), Tier B (OLMo 3.1 32B
Think on multi-cloud GPU burst), or Tier C (external API). Substantive
coverage forthcoming: how the Doorman selects the tier based on request
shape and budget cap; the system-prompt structure that embeds the
Stage-2 findings so the model reads them as editorial context rather
than re-discovering them; the whole-text rewrite output shape.

## How Stage 2 findings flow into Stage 3 as inline JSON

(draft-pending — substance follows in milestone N+1)

The Stage-2 finding set is serialised and embedded in the Stage-3
Doorman prompt as a structured annotation block. Substantive coverage
forthcoming: the exact JSON schema of the annotation block; why inline
embedding is preferred over a separate context-injection endpoint; how
the model's rewrite output is expected to address (or consciously
ignore) each flagged span; the audit implications of this design
(the finding set is part of the signed request record).

## Per-flag severity on the wire

(draft-pending — substance follows in milestone N+1)

The `/v1/proofread` response carries a `findings` array where each
entry has a `severity` discriminant. Substantive coverage forthcoming:
the current severity variants (`Banned`, `Mechanical`, `Generative`);
how callers use severity to drive UI decisions (e.g., auto-highlighting
`Banned` findings vs. collapsing `Mechanical` findings in a review
pane); the `Generative` variant reserved for per-span model suggestions
in a future milestone.

## Operator verdict closes the apprenticeship loop

(draft-pending — substance follows in milestone N+1)

After reviewing the Stage-3 rewrite, the operator records one of three
dispositions: `accepted`, `rejected`, or `edited` (with the operator's
final text). Substantive coverage forthcoming: how the verdict is
submitted back to the proofreader via `/v1/verdict`; the event-pair
tuple (`draft-created` → `draft-refined` → `creative-edited`) that
feeds the apprenticeship corpus per Doctrine claim #35 §7A; how Stage-1
DPO pairs are derived from the `(input, chosen-rewrite, disposition)`
tuple; tenant separation in the corpus output.

## When stages degrade gracefully

(draft-pending — substance follows in milestone N+1)

Each stage has a defined degradation path when its dependency is
unavailable. Substantive coverage forthcoming: Stage 2 degradation
when the LanguageTool Docker companion is unreachable (pipeline
continues at Stage 3 with an empty mechanical findings set, flagged
in the response metadata); Stage 3 degradation when the Doorman
cannot reach any compute tier within the timeout (pipeline returns
Stage-1 and Stage-2 findings only, with `generative_pass: false` in
the response); how the operator UI surfaces degraded responses
without silently omitting findings.

## See also

(draft-pending — substance follows in milestone N+1)

- `conventions/language-protocol-substrate.md` (workspace-tier
  authoritative; the four genre families and per-family vocabulary
  lists Stage 1 draws from)
- `conventions/apprenticeship-substrate.md` §7A (the loop closure
  the operator verdict feeds)
- `topic-language-protocol-substrate.md` (the substrate this pipeline
  implements; companion TOPIC in the same Tetrad wiki leg)
- `topic-customer-tier-catalog-pattern.md` (how the proofreader
  deployment instance is provisioned; companion TOPIC)
- `service-proofreader/ARCHITECTURE.md` (internal crate-level
  reference; vendor-internal)
