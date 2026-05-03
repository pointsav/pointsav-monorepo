---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-slm
target_repo: content-wiki-documentation
target_path: ./
target_filename: topic-doorman-protocol.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-04-28
authored_by: task-project-slm (session 3620a18e52bc5329)
authored_with: opus-4-7
references:
  - service-slm/ARCHITECTURE.md
  - infrastructure/slm-yoyo/CONTRACT.md
  - conventions/three-ring-architecture.md
  - conventions/llm-substrate-decision.md
  - DOCTRINE.md §I (claim #1 Sovereign Boundary)
notes_for_editor: |
  Skeleton only — substance lands across upcoming service-slm
  milestones (PS.3 AS-2 wire-format adapter; PS.4 A-1 audit endpoints;
  Yo-Yo MIN deploy). The Doorman is structurally sound and
  operationally live as Tier A on the workspace VM (verified B5
  end-to-end 2026-04-26, commit cf4f6ee). Each section's substance
  builds incrementally as the cluster ships milestones; this
  skeleton is the structural placeholder per Tetrad Discipline
  (claim #37) so the wiki leg is no longer "leg-pending — no
  drafts staged".

  Dual consumer note for project-language gateway: this TOPIC and
  topic-apprenticeship-substrate.md cover overlapping ground (the
  Doorman is the routing surface; the Apprenticeship Substrate is
  the production-routing application). Coordinate between them at
  refinement so the published versions don't repeat material.
---

# TOPIC — Doorman Protocol

(draft-pending — substance follows in milestone N+1)

## Why a Doorman

(draft-pending — substance follows in milestone N+1)

The boundary problem: a Totebox holds the customer's authoritative
data; external compute (LLMs) cannot be trusted with structured
facts. Some tasks need compute the local host cannot perform.
Without a boundary, every service in the Totebox grows its own
egress path, every egress path needs its own audit, and the
sanitise/rehydrate discipline (SYS-ADR-07) becomes per-service
discipline rather than substrate discipline.

## Three-tier compute routing

(draft-pending — substance follows in milestone N+1)

Tier A (local) on the host. Tier B (Yo-Yo GPU burst) on a remote
GCE VM with idle-shutdown. Tier C (external API) for narrow-
precision tasks behind a hard-coded allowlist. Cost guardrails
hardwired: Tier B disabled when `SLM_YOYO_ENDPOINT` unset; Tier C
disabled when no provider endpoint set; no silent fallback if
Tier A unreachable.

## The audit ledger

(draft-pending — substance follows in milestone N+1)

Every inference call produces a JSONL audit-ledger entry: timestamp,
request ID, module ID, tier, model, inference_ms, cost_usd,
sanitised_outbound flag, completion_status. Append-only. One file
per calendar day.

## The moduleId discipline

(draft-pending — substance follows in milestone N+1)

Five jobs, one field: `moduleId` selects which systemd unit
ExecStart, namespaces Mooncake KV blocks, scopes LadybugDB graph
traversals, selects which LoRA adapter stack activates, and tags
audit-ledger entries for per-project cost accounting.

## Apprenticeship Substrate routing

(draft-pending — substance follows in milestone N+1; cross-reference
topic-apprenticeship-substrate.md for the corpus + verdict +
promotion-ledger detail)

The Doorman flips polarity for code-shaped work: service-slm
becomes first responder, Master/Root/Task Claude becomes senior
reviewer. Disagreement between them, captured as signed append-only
training tuples, is the highest-quality continued-pretraining signal
Foundry produces.

## Production posture

(draft-pending — substance follows in milestone N+1)

What's live today; what's gated; what scales when.

## References

(draft-pending — citation IDs resolve at project-language refinement)
