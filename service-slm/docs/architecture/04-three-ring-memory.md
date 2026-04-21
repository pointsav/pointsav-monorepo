# The three-ring memory model

service-slm holds three distinct kinds of state, each with its own
storage, rebuild cost, and survival semantics. This model is specified
in [YOYO-COMPUTE §1](../../specs/YOYO-COMPUTE.md) and replicated here
as a reading aid.

## Ring 1 — Bootstrap state

| Attribute | Value |
|---|---|
| Storage | Container image + GCS-cached weights + Secret Manager |
| Rebuild cost | 5–15 seconds |
| Survives teardown? | Yes (as artefacts) |
| Governed by | `crates/slm-compute/` |

Ring 1 is the thing that makes the yo-yo fast on first invocation. All
four components (container, weights, drivers, warm-pool toggle) are
pre-staged in cheap storage so bootstrap is a mount-and-start rather
than a download-and-install.

## Ring 2 — Working memory (KV cache)

| Attribute | Value |
|---|---|
| Storage | LMCache + Mooncake Store → object storage |
| Rebuild cost | Near-zero for cache hits |
| Survives teardown? | Yes (pooled) |
| Governed by | `crates/slm-memory-kv/` |

Ring 2 is what makes the yo-yo fast on second invocation. KV cache
blocks are hashed, pooled in Mooncake Store, and shared across vLLM /
mistral.rs instance lifetimes. The `moduleId` namespace ensures that
Project A's cache never serves Project B's requests.

## Ring 3a — Long-term semantic memory (graph)

| Attribute | Value |
|---|---|
| Storage | LadybugDB graph |
| Rebuild cost | None (query-time) |
| Survives teardown? | Yes (authoritative) |
| Governed by | `service-content` (not this service) |

Ring 3a is authoritative data. It lives in LadybugDB, owned by
`service-content`. We read from it at context-assembly time; we write
to it only through the doorman's rehydration path.

## Ring 3b — Long-term skill (LoRA adapters)

| Attribute | Value |
|---|---|
| Storage | LoRA adapter stack, OCI Artifacts |
| Rebuild cost | One-time per project |
| Survives teardown? | Yes (portable) |
| Governed by | `crates/slm-memory-adapters/` |

Ring 3b is the compounding commercial asset. Each adapter is a small
(~50 MB), signed, versioned module that encodes task-specific
behaviour. Base model (commodity) + adapters (our moat) = the answer to
"how does Gemma grow across our projects?"

## What is not memory

Everything that does not fit one of these four ring definitions is
ephemeral and is discarded between invocations by design. If you find
yourself adding a cache that does not fit, stop and ask whether the
design needs updating — caches outside the ring model silently
accumulate cost and leak privacy guarantees.
