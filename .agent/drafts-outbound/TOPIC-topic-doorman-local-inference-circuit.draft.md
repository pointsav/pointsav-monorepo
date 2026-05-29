---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: PROSE-TOPIC
status: draft
title: "Doorman Protocol — Local Inference Circuit"
slug: topic-doorman-local-inference-circuit
target_repo: content-wiki-documentation
target_path: topics/topic-doorman-local-inference-circuit.md
bilingual_required: true
author: totebox@project-intelligence (claude-sonnet-4-6)
created: 2026-05-29
version: "0.1"
grounds_in:
  - service-slm/ARCHITECTURE.md
  - service-slm/crates/slm-doorman/src/router.rs
  - service-slm/crates/slm-doorman/src/tier/circuit_breaker.rs
  - conventions/apprenticeship-substrate.md
  - conventions/four-tier-slm-substrate.md
  - DOCTRINE.md claims #49, #54
cites: []
bcsc_class: no-disclosure-implication
editorial_notes: >
  Bloomberg standard pass needed. OLMo-only policy is a hard constraint — no softening.
  "Tier A" / "Tier B" / "Tier C" are the correct canonical terms.
  Do not use "sovereign" as a standalone adjective (BCSC posture). Use "customer-controlled"
  or "verifiable" instead. No forward-looking language for shipped features.
---

# Doorman Protocol — Local Inference Circuit

The local inference circuit is the on-premises AI routing layer that connects
coding agents, data pipelines, and operator workflows to machine-learning models
running within the customer's own infrastructure. Three inference tiers are
wired in order; Tier A is the always-on primary.

---

## Circuit architecture

The circuit connects agents (Goose, Claude Code via CORPUS bridge) to one of three
inference back-ends through a single HTTP gateway called the Doorman.

```
Agent (Goose / CORPUS bridge)
    │
    ▼
Doorman  :9080   (service-slm / slm-doorman-server)
    ├── Tier A — OLMo 7B on CPU          :8080   always-on
    ├── Tier B — OLMo 3 32B on GPU       :9443   optional accelerator
    └── Tier C — external commercial API          unconfigured by policy
```

**Tier A** is the baseline. It runs continuously on the workspace VM as
`local-slm.service`, loading OLMo 2 1124 7B Instruct (Q4_K_M quantisation,
4.16 GiB). Every request that the Doorman accepts can reach Tier A.

**Tier B** is the optional accelerator — a Spot GPU VM (`yoyo-tier-b-1`,
europe-west4-a, L4 24GB) running OLMo 3 32B-Think. It improves throughput
and quality for complex requests but is not required for the circuit to function.
The Doorman uses a circuit breaker to track Tier B availability and falls back
to Tier A when the breaker is open.

**Tier C** is intentionally unconfigured. Anthropic ToS §2.c prohibits using
external commercial model outputs as training signal for a competing model.
No API key is set; the configuration slot exists as a future option for
non-training use cases only.

---

## Tier A: the confident primary

Tier A is the correct production tier for community deployments and any
installation where continuous GPU compute is not available or not yet funded.

**What Tier A handles reliably:**

| Capability | Status |
|---|---|
| Chat completions (`/v1/messages`) | Operational |
| Goose round-trips (Anthropic Messages API) | Operational |
| Shadow capture for apprenticeship corpus | Operational |
| SFT tuple accumulation via git post-commit hook | Operational |
| Graph context injection (`GET /v1/graph/context`) | Operational |

**What Tier A cannot do:**

| Capability | Constraint |
|---|---|
| Structured entity extraction (`/v1/extract`) | Tier B-only (ADR-07). OLMo 7B does not produce reliable structured JSON arrays; the boundary is an architectural decision, not a temporary gap. |
| Reliable tool invocation | OLMo 7B Instruct is not fine-tuned for tool use; it responds with text rather than a `tool_use` content block. Tier B (OLMo 3 32B-Think) handles tool calls. |
| CORPUS WATCHER entity extraction | Deferred when Tier B circuit is open. Pending Sprint 3B (rate-limited Tier A fallback via `/v1/chat/completions`). |

**Performance on CPU:**

On an e2-highmem-4 VM (4 vCPU, 32 GiB), OLMo 7B produces approximately
1.7–1.95 tokens/second. A typical chat response completes in 90–180 seconds.
This is not suitable for interactive latency requirements but is sufficient
for batch shadow capture, which runs asynchronously in the background.

---

## OLMo-only model policy

The circuit uses OLMo exclusively for both Tier A and Tier B. No model
substitution (Qwen, Mistral, LLaMA) is permitted at any tier.

**Why:** OLMoTrace provides fully verifiable training provenance. Every inference
from an OLMo model can be traced to a specific record in the published Dolma 3
training dataset. This verifiable chain — from training data through model weights
to inference output — is the legal and commercial foundation for customer-owned
LoRA adapters and for regulated-sector procurement arguments. A model without
published training provenance cannot make this claim.

The policy is recorded in the workspace conventions; sessions that propose
non-OLMo substitution must stop and surface the conflict.

---

## Circuit breaker mechanics

The Doorman maintains a three-state circuit breaker per Tier B endpoint.

```
          failure threshold reached
Closed ────────────────────────────► Open
  ▲                                    │
  │      probe succeeds                │  reset interval elapses
  └──── HalfOpen ◄────────────────────┘
```

| State | Behaviour |
|---|---|
| `closed` | Requests forwarded to Tier B normally |
| `open` | All requests immediately fall back to Tier A (or return `deferred` for extract-only paths) |
| `half_open` | One probe request forwarded; success closes the breaker, failure reopens it |

The breaker records `opened_at` — the timestamp when it transitioned to Open.
This enables `opened_for_secs` reporting in `/readyz`, making duration-aware
decisions possible (e.g., pausing the drain worker after an extended outage).

The current circuit state is exposed at `GET /readyz`:

```json
{
  "ready": true,
  "has_yoyo": true,
  "tier_b": {
    "default": {
      "configured": true,
      "health_up": false,
      "circuit": "open",
      "opened_for_secs": 172800
    }
  }
}
```

---

## Five defects when Tier B is absent

When Tier B has been unavailable for an extended period, five concrete problems
accumulate in the current codebase. These are the defects that the Circuit
Resilience plan (Sprints 1–3) addresses.

**1. Degenerate DPO tuples.**
Shadow briefs route to Tier B for distillation. When the circuit is open and
a brief escalates with `attempt.diff=""`, the write path records an empty
rejected sample — a tuple with no meaningful training signal. With 1,460+
consecutive Tier B failures, 591 such tuples have accumulated in the corpus.

**2. readyz reports false circuit state.**
The `has_yoyo` field in `/readyz` checks only whether a Yo-Yo endpoint is
configured (an env-var check). It does not read the circuit breaker's runtime
state. A Doorman with `has_yoyo: true` may have 1,460 consecutive failures.

**3. entity_count always 0.**
`GET /healthz` on `service-content` returns `{"status":"ok"}` — no entity
count field. External monitors reading `jq .entity_count // 0` always see 0
regardless of how many entities are in the graph.

**4. WATCHER stalls.**
The corpus drain loop routes every CORPUS file through `/v1/extract`, which
is Tier B-only per ADR-07. When the circuit is open, the Doorman returns
`{"deferred":true}` and the WATCHER marks the file `skip-until-restart`.
No new entities enter the graph while Tier B is unavailable.

**5. Drain worker accumulates poison.**
The shadow brief drain worker continues to dequeue and dispatch briefs to
Tier B regardless of circuit state. With Tier B TERMINATED, every dequeued
brief is rejected, increments the failure counter, and is eventually moved
to `queue-poison/`. Over an extended outage, the queue fills with files that
will produce no training signal.

---

## Tier B: optional accelerator

Tier B is available as a Spot GPU VM on GCE (zone: europe-west4-a). When running,
it handles:

- Complex briefs routed by complexity (`High`) or explicit tier hint
- CORPUS entity extraction via `/v1/extract` (ADR-07 boundary — Tier B only)
- Shadow brief distillation for DPO training pairs

The VM stops automatically via two mechanisms: an idle monitor in the Doorman
(configurable `SLM_YOYO_IDLE_MINUTES`) and a dead-man's switch on the VM itself
(`--runtime` flag passed to `start-yoyo.sh`).

When Tier B is available, it improves throughput and quality. When it is not,
the circuit continues — Tier A handles chat and shadow capture; entity extraction
is deferred until the next Tier B window.

---

## Related documents

- `service-slm/ARCHITECTURE.md` — three-ring memory model and tier routing specification
- `service-slm/docs/deploy/deploy-yoyo-tier-b.md` — Yo-Yo provisioning runbook
- `GUIDE-guide-local-circuit-tier-a-only` — operating guide for Tier-A-only deployments
- `GUIDE-guide-goose-local-doorman` — Goose agent setup and usage
- `conventions/apprenticeship-substrate.md` — shadow capture and DPO training pipeline
