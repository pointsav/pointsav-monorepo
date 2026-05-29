---
schema: foundry-draft-v1
artifact_type: GUIDE
language_protocol: GUIDE
status: draft
title: "Operating the Local Inference Circuit Without Tier B"
slug: guide-local-circuit-tier-a-only
target_repo: woodfine-fleet-deployment
target_path: cluster-totebox-intelligence/guide-local-circuit-tier-a-only.md
bilingual_required: false
author: totebox@project-intelligence (claude-sonnet-4-6)
created: 2026-05-29
version: "0.1"
grounds_in:
  - service-slm/crates/slm-doorman/src/router.rs
  - service-slm/crates/slm-doorman-server/src/http.rs
  - service-content/src/main.rs
  - TOPIC-topic-doorman-local-inference-circuit
editorial_notes: >
  Bloomberg standard. No forward-looking language for shipped features.
  Sprint 3A/3B are planned/intended — mark as such. Verify step output
  snippets against live service before publication.
open_questions:
  - "Confirm SLM_TIER_A_FIRST env var name matches Sprint 3A implementation before finalising Step 3."
  - "Verify /readyz JSON field names match Sprint 2B implementation when shipped."
---

# Operating the Local Inference Circuit Without Tier B

This guide covers running the local AI gateway with only Tier A (OLMo 7B on CPU)
available. Use this configuration when Tier B (Yo-Yo GPU) is unavailable due to
a capacity stockout, during community deployments where GPU compute is not funded,
or when a local-only data policy prohibits external API calls.

---

## When to use this configuration

Use Tier-A-only mode when:

- Tier B is TERMINATED and europe-west4-a L4 capacity is not available
- This is a community deployment on commodity hardware (no GPU)
- A local-only data policy is in force and Tier C (external API) cannot be used
- You are testing or developing workflows that do not require entity extraction

Do not use this guide as a permanent alternative to Tier B. Shadow capture and
SFT accumulation work in Tier-A-only mode, but CORPUS entity extraction will stall
until Tier B returns (or until the planned WATCHER fallback, Sprint 3B, ships).

---

## What works in Tier-A-only mode

| Capability | Works? | Notes |
|---|---|---|
| Chat completions via Goose | Yes | OLMo 7B handles all complexity levels |
| Shadow capture (apprenticeship queue) | Yes | Briefs enqueued and dispatched when Tier B returns |
| SFT tuple accumulation via git hook | Yes | Independent of Tier B entirely |
| Graph context injection (`/v1/graph/context`) | Yes | Reads existing graph; no new extraction |
| `/v1/extract` (entity extraction) | No | ADR-07 boundary — Tier B only. Returns `deferred`. |
| CORPUS WATCHER entity extraction | No | Defers all CORPUS files until Tier B available |
| Reliable tool invocation (Goose file tools) | No | OLMo 7B is not tool-use fine-tuned |

---

## Step 1 — Verify Tier A is enabled

Tier A runs as `local-slm.service`. Confirm it is active and the Doorman is
routing to it:

```bash
systemctl is-active local-slm
# Expected: active

curl -s http://127.0.0.1:9080/readyz | python3 -m json.tool
# Expected: "has_local": true
```

If `has_local` is false or the service is inactive, check `SLM_FORCE_BROKER_MODE`
in `/etc/local-doorman/local-doorman.env`. It must be `false` (or absent):

```bash
grep FORCE_BROKER /etc/local-doorman/local-doorman.env
# Expected: SLM_FORCE_BROKER_MODE=false  (or no match)
```

If it is set to `true`, change it and restart:

```bash
sudo sed -i 's/SLM_FORCE_BROKER_MODE=true/SLM_FORCE_BROKER_MODE=false/' \
    /etc/local-doorman/local-doorman.env
sudo systemctl restart local-doorman.service
```

---

## Step 2 — Check the circuit breaker state

Inspect Tier B circuit state. The breaker being open is expected in this configuration.

```bash
curl -s http://127.0.0.1:9080/readyz | python3 -m json.tool
```

Expected output when Tier B is unavailable (Sprint 2B output format — planned):

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

Note: `ready: true` is correct even with `circuit: open`. The gateway is
operational on Tier A. The `opened_for_secs` value (seconds since Tier B
circuit opened) is informational.

Before Sprint 2B ships, check the Doorman log directly:

```bash
journalctl -u local-doorman --since "1 hour ago" | grep -i 'circuit\|tier'
```

---

## Step 3 — Set SLM_TIER_A_FIRST (planned)

> **Note:** This env var is planned for Sprint 3A. It is not yet available in
> the current binary. Once Sprint 3A ships, this step becomes the standard
> Tier-A-only configuration knob.

When Sprint 3A is deployed, add the following to `/etc/local-doorman/local-doorman.env`:

```bash
SLM_TIER_A_FIRST=true
```

Then restart:

```bash
sudo systemctl restart local-doorman.service
```

With `SLM_TIER_A_FIRST=true`, all requests route to Tier A regardless of
complexity classification, unless the request carries an explicit Tier B tier
hint AND the Tier B circuit is closed AND Tier B `health_up` is true.
The `/v1/extract` path (ADR-07 boundary) remains exempt — it always requires
Tier B.

---

## Step 4 — Send a test request

Verify the circuit is routing to Tier A:

```bash
curl -s http://127.0.0.1:9080/v1/messages \
  -H "Content-Type: application/json" \
  -H "x-api-key: foundry-local" \
  -d '{
    "model": "claude-haiku-4-5-20251001",
    "max_tokens": 64,
    "messages": [{"role": "user", "content": "Reply with one word: operational."}]
  }' | python3 -m json.tool
```

Confirm in the Doorman log:

```bash
journalctl -u local-doorman -n 20 | grep -E 'tier=|dispatching'
# Expected: tier="local"  or  dispatching ... tier=Local
```

If you see `tier="yoyo"` and a connection error, the circuit breaker has not
opened yet. Wait 30 seconds for the health probe to fail and the breaker to trip.

---

## Step 5 — Verify Goose routes through Tier A

```bash
export ANTHROPIC_HOST=http://127.0.0.1:9080
export ANTHROPIC_API_KEY=foundry-local
export GOOSE_MODEL=claude-haiku-4-5-20251001
goose session
```

At the Goose prompt, ask a simple question. Check the Doorman log confirms
`tier="local"`. Note that Goose file tools (read_file, write_file) will not
invoke as tool_use content blocks — OLMo 7B responds with text instead.
This is a model capability limit, not a configuration error.

---

## Step 6 — Understand performance characteristics

OLMo 7B on CPU (e2-highmem-4, 4 vCPU):

| Metric | Value |
|---|---|
| Inference speed | 1.7–1.95 tokens/second |
| Typical chat response | 90–180 seconds |
| Shadow brief capture | Asynchronous (does not block the response) |
| SFT tuple via git hook | ~2 seconds (async fire-and-forget) |

This throughput is not suitable for interactive latency requirements. For batch
workflows (shadow capture, CORPUS accumulation) it is adequate — these operations
run in the background and are not time-sensitive.

---

## Step 7 — What happens to shadow briefs and entity extraction

**Shadow briefs:** The drain worker dispatches shadow briefs to Tier B. With
the circuit open, dispatches fail immediately and the worker backs off. Briefs
stay in `queue/` and are retried when Tier B returns. This is correct behaviour.

Check the drain log:

```bash
journalctl -u local-doorman --since "1 hour ago" | grep -i 'drain\|shadow\|poison'
```

With Sprint 3C (planned), the drain worker will pause entirely when all
configured Tier B nodes have been circuit-open for more than `SLM_HOLD_THRESHOLD_SECS`
(default: 3600 seconds). This prevents queue-poison accumulation during extended
Tier B outages.

**CORPUS entity extraction:** Service-content routes all CORPUS files through
`/v1/extract` (Tier B-only, ADR-07). When the circuit is open, each file is
marked `deferred` and will not be retried until the service restarts with a
working Tier B connection.

With Sprint 3B (planned), a rate-limited Tier A fallback via `/v1/chat/completions`
will be available for CORPUS files. Enable it with:

```bash
SERVICE_CONTENT_TIER_A_FALLBACK_ENABLED=true
SERVICE_CONTENT_TIER_A_FALLBACK_INTERVAL_SECS=300
```

Until Sprint 3B ships, entity extraction stalls during Tier B outages. The graph
retains all previously extracted entities; no data is lost.

---

## Verification checklist

| Check | Command | Expected |
|---|---|---|
| Tier A active | `systemctl is-active local-slm` | `active` |
| Doorman routing to Tier A | `curl /readyz` → `has_local: true` | `true` |
| Test message routes locally | `journalctl -u local-doorman -n 10 \| grep tier` | `tier="local"` |
| Shadow capture working | Send a message, check `ls data/apprenticeship/queue/` | File count increases |
| Entity graph intact | `curl -s http://127.0.0.1:9081/healthz` | `status: ok` (entity_count once Sprint 2A ships) |

---

## When Tier B returns

When `start-yoyo.sh` exits 0 and the Doorman closes the circuit:

1. The drain worker resumes dispatching shadow briefs automatically.
2. Service-content processes any CORPUS files that were deferred.
3. Entity extraction results appear in the graph within minutes.
4. If `SLM_TIER_A_FIRST=true` (planned Sprint 3A), Tier B will only receive requests
   with an explicit `tier_hint=yoyo` — remove the flag to return to complexity-based routing.

---

## Related documents

- `TOPIC-topic-doorman-local-inference-circuit` — architecture overview and five-defect analysis
- `GUIDE-guide-goose-local-doorman` — Goose setup and usage guide
- `service-slm/scripts/start-yoyo.sh` — Tier B restart procedure and exit codes
- `service-slm/ARCHITECTURE.md` — tier routing specification
