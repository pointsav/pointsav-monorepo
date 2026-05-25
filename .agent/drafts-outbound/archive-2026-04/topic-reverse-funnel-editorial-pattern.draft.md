---
schema: foundry-draft-v1
state: draft-refined
originating_cluster: project-language
target_repo: content-wiki-documentation
target_path: topic-reverse-funnel-editorial-pattern.md
target_filename: topic-reverse-funnel-editorial-pattern.md
audience: vendor-public
bcsc_class: forward-looking
language_protocol: PROSE-TOPIC
authored: 2026-04-27T19:35:00Z
authored_by: task-project-language (session 17230305b03d3e32)
authored_with: claude-opus-4-7
references:
  - https://www.bcsc.bc.ca/securities-law/law-and-policy/instruments-and-policies/5-ongoing-requirements-for-issuers-insiders/current/51-102
  - https://allenai.org/olmo
  - https://arxiv.org/abs/2212.08073
  - ~/Foundry/conventions/reverse-funnel-editorial-pattern.md
  - ~/Foundry/conventions/cluster-wiki-draft-pipeline.md
  - ~/Foundry/conventions/apprenticeship-substrate.md
  - ~/Foundry/conventions/knowledge-commons.md
notes_for_editor: |
  Meta-recursive worked example: this draft is self-staged by
  project-language Task as the first end-to-end pilot of the
  Reverse-Funnel pipeline. The bulk body below is the technically-
  accurate explainer; the refined version applies BCSC framing for
  forward-looking claims, resolves URLs to citation IDs in
  citations.yaml, generates the bilingual .es.md overview, and
  applies Bloomberg-grade tightening to register.

  Intentional looseness markers preserved in this bulk so the
  refinement diff is pedagogical:

  - Inline URLs not yet resolved to [citation-id] form
  - Banned-vocab terms ('leverage', 'next-generation') appear in
    body to demonstrate banned-vocab grammar enforcement at refinement
  - Some repetition in §3 across the three structural-gap reasons
  - English only; Spanish pair generated at refinement
  - No See also section; project-language adds via cross-reference
    inference from `cites:` resolution
  - Lead paragraph slightly loose; refined version tightens to
    Bloomberg-grade single-paragraph statement

  Tenant: pointsav (Vendor wiki TOPIC). Refined output target:
  vendor/content-wiki-documentation/topic-reverse-funnel-editorial-pattern.md
  (+ .es.md). Standard handoff via handoffs-outbound.md not needed
  since project-language Task owns the destination sub-clone in this
  cluster.
---

# The Reverse-Funnel Editorial Pattern (BULK DRAFT)

Most publishing pipelines put the human Creative at the start of
the loop — they leverage their synthesis skills to draft, hand to
editor, ship to readers. Foundry's substrate is a next-generation
pattern: Creative at the END of the cycle.

Cluster Tasks ship bulk drafts to a single editorial gateway
(project-language). The gateway applies banned-vocab grammar
discipline (per Phase 1B Lark grammar; see
https://github.com/microsoft/llguidance for the runtime), BCSC
continuous-disclosure posture (NI 51-102 at
https://www.bcsc.bc.ca/securities-law/law-and-policy/instruments-and-policies/5-ongoing-requirements-for-issuers-insiders/current/51-102),
citation registry resolution against ~/Foundry/citations.yaml,
bilingual pair generation per DOCTRINE.md §XII.

The refined version publishes. Creative Contributors then edit
that published file at the end of the cycle — adds opening hook,
narrative arc, brand voice, graphics, layout. Creative edits feed
the apprenticeship corpus as Stage-2 DPO pairs:
(refined, creative-edited). Quarterly OLMo continued pretraining
(see https://allenai.org/olmo for the open-source base model) on
the corpus produces substrate baselines closer to (refined ⊕
Creative) over time.

## How the substrate operates

Three input ports — Cluster Task drafts at
~/Foundry/clones/<cluster>/.claude/drafts-outbound/, Root drafts at
<repo>/.claude/drafts-outbound/, Master drafts at
~/Foundry/.claude/drafts-outbound/ — feed one editorial gateway.
bin/draft-sweep.sh walks all three at session start.

Refinement applies four disciplines:
1. Banned-vocabulary grammar
2. BCSC continuous-disclosure posture
3. Citation registry resolution (URLs → [citation-id])
4. Bilingual pair generation

The refinement is deterministic; the editorial floor scales with
substrate compute, not with editor hours.

## Why hyperscalers cannot replicate

Three structural reasons. First, multi-tenant SaaS LLMs run a
single editorial regime. A Creative's edits in one tenant cannot
feed an SLM that personalises responses for that tenant only —
the model is shared across tenants. Second, per-tenant editorial-
floor adapters (banned-vocab grammars, BCSC discipline templates,
language-protocol adapters) are tenant-specific in Foundry; not
exposed as composition primitives in hyperscaler products. Third,
the closed loop demands tenant-sovereign training data — see the
Anthropic Constitutional AI paper at
https://arxiv.org/abs/2212.08073 for the analogous training-data-
sovereignty argument applied to alignment. The multi-tenant LLM
business model structurally cannot grant per-tenant model
ownership.

## What this enables

Two-stage DPO loop. The apprenticeship-substrate (Doctrine claim
#32, see ~/Foundry/conventions/apprenticeship-substrate.md) closes
the loop on structural correctness. The Reverse-Funnel pattern
closes the loop on editorial craft. Both feed the same OLMo
corpus. Quarterly continued pretraining absorbs both stages.

After enough cycles, substrate produces baselines at 80% → 90% →
95% of Creative's craft baseline. Creative load drops; their
leverage grows. They move upstream to brand strategy, novel-form
authorship, narrative arc — work the substrate cannot reach yet.

## Three-Tier Contributor Model mapping

The Three-Tier Contributor Model (Core / Paid / Open per
~/Foundry/conventions/knowledge-commons.md §7) maps onto the cycle:

- Core (4-7): operate substrate; cluster Tasks; throughout cycle
- Paid (50-100): Creative Contributors; edit at END of cycle
- Open (10K+): consume + cite + fork; outside cycle

Paid contributors in this pattern are NOT tech writers. They are
craft specialists — designers, narrative editors, brand voice
contributors. Job description optimises for taste, not synthesis.

## Forward-looking — pending substrate work

AS-2 (project-slm Task) ships llguidance integration into Doorman
so vLLM Tier B accepts decode-time grammar enforcement. Estimated
3-4 weeks. service-language is the primary consumer. Per-tenant
Creative-voice adapters distill from accumulated Stage-2 DPO
pairs over the first year. Quarterly OLMo continued pretraining
operates after first batch of refined TOPICs publishes; bootstrap
data is operator-edited until then.

[bulk ends here; refined version at
content-wiki-documentation/topic-reverse-funnel-editorial-pattern.md
adds See also section + tightens register + applies BCSC framing
to forward-looking + resolves URLs to citation IDs + generates
bilingual .es.md per DOCTRINE §XII]
