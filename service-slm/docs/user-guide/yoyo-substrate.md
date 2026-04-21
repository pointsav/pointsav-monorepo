# The yo-yo compute substrate — operator view

What the yo-yo does, what it costs, and how to reason about it when
things go sideways. Architecture detail is in
[docs/architecture/02-yoyo-substrate.md](../architecture/02-yoyo-substrate.md);
normative spec is in [specs/YOYO-COMPUTE.md](../../specs/YOYO-COMPUTE.md).

## What it is

The yo-yo is the mechanism by which service-slm runs GPU inference on
a Cloud Run node that only exists while it is serving. The node is
torn down between batches. State that survives teardown is explicitly
modelled in three rings.

## What it costs

Approximate cost model (April 2026 rates, your mileage will vary):

| Phase | Cost item | Order of magnitude |
|---|---|---|
| Per-batch work | Cloud Run GPU seconds | $0.50–$5 per batch |
| Idle | Cloud Run scale-to-zero | $0 |
| Warm pool (opt-in) | min-instances=1 GPU | ~$30/hour during window |
| KV cache | GCS + tiny always-on master | $0.50–$5/month |
| Adapters | GCS + training (one-time) | ~$30 per adapter training |

A month of moderate use (a few batches per week, occasional
query-time generation, no sustained warm pool) is typically
$20–$100 total. Compare to a Vertex AI warm endpoint at
$600–$3,000/month minimum.

## When the yo-yo is doing its job

Signs of health:

- Warm start < 30 seconds.
- Cache hit ratio > 50% on the second full-corpus run.
- Ledger shows matched `BOOT_REQUEST` / `BOOT_COMPLETE` pairs.
- `TEARDOWN_COMPLETE` follows every `TEARDOWN_REQUEST`.

## When the yo-yo is sideways

Symptoms and first-line responses:

- **Cold start > 60 seconds.** Check the container image size; it
  should be ~200 MB for mistral.rs (Phase 2). Check the GCS weight
  mount is still pointing to the regional mirror.
- **Cache hit ratio stuck near zero.** The Mooncake master may be
  unreachable, or the `moduleId` may be changing across invocations
  (which invalidates cache). Run `slm-cli node status` and check the
  Mooncake section.
- **`PREEMPTION` events appearing.** Spot instances are being pulled.
  Either accept the variance (fine for batch) or switch to a non-spot
  machine family in the compute manifest.
- **Adapter load failing.** `slm-cli adapter verify <id>` —
  signatures may have rotated.

## Module isolation

Every request carries a `moduleId`. This propagates through:

- Bootstrap container selection (rare — usually one container per
  deployment).
- KV cache namespace (Project A and Project B never share blocks).
- Graph partition (via service-content).
- Adapter stack selection.
- Ledger event tagging.

If you suspect cross-tenant bleed, the first check is to grep the
ledger for unexpected `moduleId` values on the events you are
investigating.

## Opting into a warm pool

For known sustained-load windows (end-of-quarter batch, scheduled
corpus refresh):

```bash
slm-cli node up --warm-pool --window 3h
```

Warm pool opt-ins write `BOOT_REQUEST` / `BOOT_COMPLETE` immediately,
then the node stays up for the requested window billed as
`min-instances=1`. At window end, an implicit `slm-cli node down`
fires.

## Further reading

- [specs/YOYO-COMPUTE.md](../../specs/YOYO-COMPUTE.md) — authoritative.
- [architecture/02-yoyo-substrate.md](../architecture/02-yoyo-substrate.md) — narrative.
- [architecture/05-ledger-schema.md](../architecture/05-ledger-schema.md) — event semantics.
