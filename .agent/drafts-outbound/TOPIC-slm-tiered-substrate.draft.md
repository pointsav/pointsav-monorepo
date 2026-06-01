---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC
audience: general
bcsc_class: no-disclosure-implication
version: "0.1"
date: 2026-06-01
title: "The Tiered SLM Substrate — Local-First AI Routing"
routes_to: project-editorial
paired_with: pending (ES sibling required before publication — project-editorial to commission)
research_done_count: 3
research_suggested_count: 1
open_questions_count: 1
research_provenance: >
  Live validation 2026-06-01 (Yo-Yo Tier B /v1/extract returned 4 classified entities in 7.2s);
  service-slm/ARCHITECTURE.md; service-content/CLAUDE.md; BRIEF-slm-substrate-master.md §2.8;
  conventions/four-tier-slm-substrate.md; SYS-ADR-07.
research_inline: true
notes_for_editor: >
  Architecture TOPIC drafted by project-intelligence from the engineering record. Needs the
  standard editorial pass: Bloomberg register check, bilingual ES sibling, and verification that
  no forward-looking capability is stated as delivered (training-loop items are planned/intended).
  The wire-format subsection is the durable engineering lesson and is the reason this TOPIC exists
  now. One open question flagged inline for the editor.
---

# The Tiered SLM Substrate — Local-First AI Routing

PointSav runs language-model work through a single gateway that decides, per request, which
tier of compute should answer. The design goal is to keep routine work on hardware the operator
already owns, reach for rented GPU only when a task genuinely needs it, and never let structured
business data cross the AI boundary. This TOPIC describes the tiers, how a request is routed, and
how the gateway constrains model output to a required shape.

## The tiers

The substrate distinguishes three tiers of inference, each suited to a different class of work.

*Tier A — the local model.* A small model (currently OLMo 2 7B, 4-bit quantised) runs on the
operator's own machine and answers immediately for interactive work: code navigation, short
drafting, quick questions. It is always available and costs nothing per request. It is the
*student* of the learning loop, not the generator of training data — a distinction that matters
because a small model asked to critique its own work produces little useful signal.

*Tier B — the burst GPU model.* A larger reasoning model (OLMo 3 32B-Think) runs on a
preemptible cloud GPU that is started on demand and stopped when idle. It handles the heavy,
structured work — entity extraction for the knowledge graph, and the training-data generation
that improves Tier A. Because the GPU is rented by the minute, the gateway brings it up only when
there is batch work to do and a dead-man's switch stops it automatically.

*Tier C — the external API.* A commercial model, deliberately left unconfigured. It exists in the
design as a resilience option but is not enabled, because the substrate's value is that work stays
on customer-controlled compute.

## How a request is routed

Every request carries a complexity hint and an optional tier preference. The gateway's default
posture is *local-first*: a confident local primary answers unless the caller explicitly asks for
the burst tier and that tier's health check is currently passing. A circuit breaker tracks the
burst tier's availability — when the GPU is down, the breaker opens and dependent work *defers*
rather than failing, so a stockout or a stopped VM never produces an error, only a "try again
later" that the caller retries with backoff.

One routing rule is absolute: entity extraction goes only to the burst tier. The small local
model cannot reliably produce structured JSON arrays, so it is never used as a fallback for
extraction. When the GPU is unavailable, extraction simply waits.

## The structured-data boundary

A hard architectural rule (SYS-ADR-07) governs what may cross the AI boundary: prose may go out as
a prompt; structured business records may not. Entity extraction illustrates the discipline. The
text to be analysed is prose. The *schema* that describes the desired output is sent as a
constraint, not as data. The model returns a structured array that the gateway parses and writes
to the graph — but no existing structured record was ever exposed to the model. The boundary is
one-directional by design.

## Constraining output to a required shape

For extraction to be usable, the model's output must match a known JSON shape exactly. The gateway
does this by sending the output schema as a *grammar constraint* alongside the prompt, so the
inference server is forced to emit only conforming tokens. In live testing this turns a request
that would otherwise ramble into prose into a tight, parseable result: a short business sentence
returned four correctly classified entities — company, location, location, person — in about
seven seconds.

*Engineering note — the constraint is engine-specific, and that is a maintenance hazard.* The
mechanism for requesting a grammar constraint differs between inference engines. One family of
servers reads the constraint from a nested request envelope; another reads it from top-level
request fields and silently ignores the envelope. When the substrate's burst tier migrated from
the first engine to the second, one client path was updated and another was not — so for a period
the schema was sent in a form the running server ignored, and the model generated unconstrained
prose that could not be parsed. The lesson is durable: when a structured-output constraint "isn't
working," confirm empirically which request field the *deployed* engine actually honours before
changing anything else, because the failure is silent — the server returns a valid response that
simply ignores the constraint.

## The boundary between interactive and batch

A second durable lesson: a reasoning model is the wrong tool for a structured, mechanical task.
The 32B reasoning model spends a large token budget "thinking" before answering, which is valuable
for hard generation work but pure overhead for extracting named entities against a fixed schema.
The substrate's design intent is therefore to reserve reasoning for the generative path and run
the extraction path with reasoning suppressed — fast, cheap, and deterministic.

> **Open question (for the editor):** whether per-request reasoning control should be exposed at
> the gateway, or whether extraction should run on a dedicated non-reasoning model instance.
> Today the reasoning budget is a server-wide setting; per-path control is a planned, not
> delivered, capability and should be described in planned/intended terms.

## Why this matters

The tiered substrate is what lets a sovereign deployment behave like a managed AI service without
surrendering control of its data or its compute. Routine work is instant and free on local
hardware; heavy work bursts to a GPU only as needed and stops itself; structured records never
leave the operator's boundary; and the model's output is constrained to a shape the system can
trust. The architecture is deliberately boring in operation — which, for infrastructure that sits
between a private data vault and an external model, is the point.
