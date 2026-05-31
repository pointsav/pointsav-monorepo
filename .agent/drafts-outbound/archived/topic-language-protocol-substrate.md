---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-proofreader
target_repo: vendor/content-wiki-documentation
target_path: ./
target_filename: topic-language-protocol-substrate.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-04-28T00:30:00Z
authored_by: task-project-proofreader (session d0f5dd46a325f2bf)
authored_with: opus-4-7
references:
  - https://arxiv.org/abs/2310.06992
  - ~/Foundry/conventions/language-protocol-substrate.md
  - ~/Foundry/clones/project-proofreader/pointsav-monorepo/service-proofreader/ARCHITECTURE.md
  - ~/Foundry/conventions/apprenticeship-substrate.md
notes_for_editor: |
  Skeleton stage. Section headings + (draft-pending — substance
  follows in milestone N+1) markers per Tetrad upgrade message.
  Substance to fill in: the Cornell anti-homogenization study finding
  (auto-detection of writing style narrows the space of voices the
  model produces); how explicit per-protocol selection at the request
  boundary preserves voice diversity; how the three-stage pipeline
  (banned-vocab + LanguageTool + Doorman generative) composes; the
  flag-don't-rewrite default + operator-verdict closing the loop;
  the apprenticeship-corpus event-pair shape that flows from this
  pattern (Doctrine claim #35 §7A).

  When refining: apply Bloomberg-article register; resolve URL
  references to citation IDs against ~/Foundry/citations.yaml;
  generate the bilingual .es.md overview per DOCTRINE §XII; keep
  the technical depth — the audience is vendor-public engineers
  evaluating the substrate.

  Suggested length when substance lands: 800-1200 words English.

  Citations to register if not yet present:
  - Cornell-anti-homogenization-2310.06992 (the study driving the
    explicit-protocol-selection design choice)
  - DOCTRINE.md claim #35 (Reverse-Funnel Editorial Pattern; the
    apprenticeship loop the substrate feeds)
---

# The language-protocol substrate

(draft-pending — substance follows in milestone N+1)

## Why explicit protocol selection

(draft-pending — substance follows in milestone N+1)

The substrate's foundational design choice is to **require the
caller to declare a language protocol on every editorial request**
rather than auto-detecting one from the input. Substantive coverage
forthcoming: the Cornell anti-homogenization study's finding that
auto-detection narrows the space of voices a model produces; the
operator-perspective rationale (the operator KNOWS what register
they're writing in — let them say so).

## The four genre families

(draft-pending — substance follows in milestone N+1)

PROSE / COMMS / LEGAL / TRANSLATE — what each family covers, the
banned-vocabulary lists per family, the register-tightening targets.

## Three-stage pipeline composition

(draft-pending — substance follows in milestone N+1)

How `service-proofreader` composes the three stages on every
`/v1/proofread` call:

1. **Banned-vocabulary scan** — rule-driven; deterministic; flags
   only, never rewrites
2. **LanguageTool 6.6 mechanical pass** — spelling / grammar /
   style; runs as a Docker companion; flags + suggestions
3. **Doorman generative pass** — register-tightening rewrite via
   service-slm Doorman; reads the Stage-2 LT findings as inline
   JSON context to avoid re-discovery; produces a whole-text
   rewrite (no per-span suggestions yet — `Severity::Generative`
   variant reserved for that work)

## Flag-don't-rewrite default

(draft-pending — substance follows in milestone N+1)

The `/v1/proofread` response carries a structured `diff` array;
the caller (UI) decides what to apply. The substrate never
silently rewrites the operator's text. The Apply-all UX is opt-in
per request.

## Operator verdict closes the apprenticeship loop

(draft-pending — substance follows in milestone N+1)

After seeing the rewrite, the operator picks one of three
dispositions: `accepted`, `rejected`, or `edited` (with the final
text of their choice). The verdict feeds the apprenticeship-corpus
event-pair (`draft-created` → `draft-refined` → `creative-edited`)
per Doctrine claim #35 §7A. Stage-1 DPO pairs derive directly from
the (input, chosen-rewrite, operator-disposition) tuple.

## Tenant separation in the corpus

(draft-pending — substance follows in milestone N+1)

Per-tenant routing of corpus output: `pointsav` records under the
workspace-tier corpus root; `woodfine` records under the customer-
tenant deployment instance directory. Tenant mixing is structurally
impossible because the path component is the boundary.

## What the substrate does NOT do

(draft-pending — substance follows in milestone N+1)

- Does not auto-detect protocol — the operator picks
- Does not silently rewrite — the operator chooses what to apply
- Does not embed Tier-C API keys in the proofreader — keys live
  with the Doorman
- Does not train on tenant text by default — the no-train contract
  is the substrate

## See also

(draft-pending — substance follows in milestone N+1)

- `conventions/language-protocol-substrate.md` (workspace-tier
  authoritative)
- `conventions/apprenticeship-substrate.md` §7A (the loop closure)
- `conventions/reverse-funnel-editorial-pattern.md` (Doctrine claim
  #35; the editorial pipeline this substrate feeds)
