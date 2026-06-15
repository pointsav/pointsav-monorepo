---
artifact: topic
schema: foundry-draft-v1
draft-id: project-data-20260614-dual-tier-extraction
title: Dual-Tier Entity Extraction Architecture
language_protocol: project-editorial
route_to: project-editorial
destination: media-knowledge-documentation
created: 2026-06-14
status: staged
research_trail_source: BRIEF-jennifer-2-ingest-pipeline.md sessions 1–6; service-content/src/main.rs source review; 32 DPO enrichment pair analysis (2026-06-14)
research_trail_method: empirical — pair analysis of Tier A vs Tier B entity counts; source code review of flush_tier_a(), ALLOWED_CLASSIFICATIONS, drain-hold predicate
research_trail_data: /var/lib/local-content/graph/entities.lbug (10,831 entities 2026-06-14); training-corpus/feedback/ (32 DPO pairs)
research_trail_claims: Tier B extracted more entities than Tier A in 78% of enrichment pairs (n=32, 2026-06-14, module=jennifer)
research_trail_gaps: drain-hold predicate code fix pending (project-intelligence); LoRA fine-tuning not yet run (ML libs not installed on GPU VM)
bcsc_class: internal
forbidden_terms_cleared: false
---

The Pointsav entity extraction pipeline uses two inference tiers operating in sequence on each document. Tier A provides a baseline extraction on local CPU hardware. Tier B applies a higher-capacity GPU pass when available and records the improvement as a training signal for future model refinement.

## Tier A — Local Inference

Tier A routes document payloads to OLMo 7B running on the workspace VM's CPU via the Doorman's `/v1/chat/completions` endpoint. The `local-slm.service` systemd unit keeps this tier available without external dependencies.

Extraction uses a structured prompt that constrains the model to five entity classifications: Person, Company, Project, Account, and Location. A module-level constant (`ALLOWED_CLASSIFICATIONS`) in the extraction service rejects any output classification outside this vocabulary before the result reaches the graph store, preventing taxonomy drift from uncontrolled model output.

The Location classification carries a restrictive definition — a specific named geographic place or address — with explicit negative examples in the system prompt excluding generic spatial nouns. SPDX licence identifiers and other technical strings are excluded at the prompt level.

## Tier B — GPU Inference

Tier B routes through the Doorman's `/v1/extract` endpoint to OLMo 32B running on an L4 GPU node. When the Tier B circuit is closed and the node is reporting as healthy, the Doorman forwards the same document payload for a second, higher-capacity extraction pass.

Tier B results are compared against the Tier A baseline. The pair is recorded in the DPO (Direct Preference Optimisation) feedback corpus: the Tier B output is the preferred response and the Tier A output is the rejected response. This corpus accumulates across live extraction runs and is used for future model fine-tuning under the Yo-Yo training cycle.

An analysis of 32 DPO enrichment pairs produced on 2026-06-14 found that Tier B extracted more entities than Tier A in 78% of cases, fewer in 6%, and equal counts in 16%.

## Drain-Hold Predicate

The Doorman's apprenticeship drain queue holds pending training jobs. When both tiers are available, extraction and training proceed in parallel. The drain-hold predicate (`SLM_DRAIN_PAUSED`) can pause the apprenticeship queue to prevent the GPU from receiving a depleted queue that has been entirely processed by Tier A alone during a GPU outage.

The predicate is intended to fire automatically when Tier B has been offline for longer than a configured threshold. The current implementation has a known limitation: it does not detect the condition where a GPU node reports `health_up: false` but the Doorman circuit remains closed (because no inference requests have failed to trip the breaker). A `down_for_secs` field on the tier information struct is planned to address this.

## Flush Behaviour When Tier B Is Unavailable

When Tier A completes extraction and the Tier B circuit is open, the document is marked as successfully processed and the Tier A entities are written to the graph store. The document will not be re-processed for a Tier B enrichment pass — the DPO feedback pair is permanently skipped for that document.

For brief GPU unavailability this behaviour is intentional. For extended outages, the drain-hold predicate is intended to preserve documents for future Tier B enrichment by pausing the apprenticeship queue before Tier A processes all pending documents.

## Graph Store

Entities extracted by either tier are written to LadybugDB at `/var/lib/local-content/graph/entities.lbug`. Entities are keyed by module identifier and entity name; duplicates within a module are upserted. The extraction service tracks processed documents in a JSONL ledger at `/var/lib/local-content/graph/processed_ledgers.jsonl` to prevent re-extraction on service restart.

As of 2026-06-14 the graph store holds 10,831 entities across all modules.
