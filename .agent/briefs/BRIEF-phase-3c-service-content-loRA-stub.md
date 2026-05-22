---
title: Phase 3.6 + 3.7 + 3.8 + 3.10 — service-content + LoRA (STUB)
created: 2026-05-18
status: stub-with-plan
scope: project-intelligence + project-infrastructure clusters; multi-day Rust engineering
estimated_effort: 5-7 sessions across 2 weeks
---

# Phase 3C — service-content + LoRA scaffolding

## 3.6 — service-content `POST /v1/draft/generate`

Currently stubbed in `service-content`. Needs:

- Endpoint handler that takes `{module_id, prompt}`, queries LadybugDB
  for relevant entities, assembles ~2K-token grounded prompt, calls
  local-slm via Doorman, returns `{draft, audit_id}`.
- Doorman `/v1/audit/proxy` wiring to forward to this endpoint.
- Integration test with a seeded LadybugDB fixture.

Source: `clones/project-infrastructure/service-content/src/` (or wherever
service-content's source is — verify).

## 3.7 — Citation linkage + provenance

Extend `GraphEntity` schema with:
- `cited_sources: Vec<DocRef>`
- `cites: Vec<EntityRef>`

Add endpoint `GET /v1/graph/provenance?entity_id=&depth=2` that returns
backlinks (which docs cite this entity, which entities does this entity
cite, up to depth N).

Wire into `/v1/draft/generate` so generated drafts include
`[entity_name](#entity_id)` citation links inline.

## 3.8 — Graph-to-apprenticeship corpus wiring

At Doorman inference time, populate `graph_context` field in
shadow-capture JSONL tuples. Each tuple becomes:
```json
{
  "brief": "...",
  "attempt": "...",
  "actual_diff": "...",
  "graph_context": [{"entity_id": "...", "passage": "..."}, ...]
}
```

Wire git post-commit hook (or extend `bin/capture-edit.py`) to capture
accepted graph mutations as positive training signal.

Route tuples to `~/Foundry/data/training-corpus/apprenticeship/graph-grounded/`.

Closes Doctrine claim #44 (Knowledge-Graph-Grounded Apprenticeship)
co-evolution loop.

## 3.10 — corpus-threshold.py + LoRA scheduler

Nightly cron (`/etc/systemd/system/lora-trigger.{service,timer}`) that:
- Counts new training-corpus tuples since last LoRA run
- If `(autonomous_weight * autonomous_count) + (review_weight * review_count) ≥ N tokens`:
  - SSH to Yo-Yo #1 (yoyo-tier-b-1 GCE VM, must be RUNNING)
  - Invoke `lora-training.service` with the new tuples
  - Adapter lands in `data/adapters/<date>.safetensors`
  - Doorman hot-reloads via vLLM Multi-LoRA on next inference

Gated on:
- Yo-Yo VM being able to start on demand (currently TERMINATED)
- vLLM Multi-LoRA support in the Doorman inference path

## Why deferred from 2026-05-18 AUTO sprint

Multi-day Rust engineering across 3 services (service-content,
service-slm/slm-doorman, lora-training). Each item is a feature-branch
PR with its own Stage 6 promotion. Not compatible with single-night
AUTO scope.

## What IS in place from tonight's sprint

- Apprenticeship queue drainer (Phase 3.4) feeds training-corpus
- datagraph-health.json writer (Phase 3.9) surfaces growth
- DPO feedback scanner (Phase 3.11) closes negative-trajectory loop
- Audit-ledger trajectory capture (Phase 2.1) records every Claude session
- Stop hook + permissions allowlist + secret-pattern gate

The compounding substrate IS growing tonight — pure SLM-trained tuples,
trajectory captures, prose-edit pairs. What's blocked is the structured
entity loop (3.6-3.10) which requires the Rust work above.
