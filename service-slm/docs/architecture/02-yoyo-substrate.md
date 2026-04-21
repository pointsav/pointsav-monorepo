# The yo-yo compute substrate

The yo-yo substrate is the mechanism by which service-slm gets GPU
inference without paying for an always-on GPU. It is specified
authoritatively in [YOYO-COMPUTE.md](../../specs/YOYO-COMPUTE.md);
this document is the narrative summary.

## The problem

Hyperscalers offer two postures for GPU inference:

- **Managed warm endpoint** (Vertex AI, SageMaker). ~$600–3,000/month
  minimum. Zero cold start. Economic only at very high utilisation.
- **Serverless with full cold start** (Cloud Run GPU without
  pre-staging). Scale-to-zero. Full cold start means 60–120 seconds
  to load a 26B model from Hugging Face.

Neither fits the PointSav workload, which is bursty but predictable:
weekly batch jobs plus occasional query-time generation.

## The yo-yo

service-slm keeps four artefacts in cheap cold storage:

1. **Pre-built container** in Artifact Registry.
2. **Pre-downloaded weights** in GCS, mounted via Cloud Storage FUSE.
3. **Cloud Run GPU** with drivers pre-installed.
4. **Warm-pool opt-in** (`min-instances=1`) for known sustained-load
   windows.

A request arrives. `slm-compute` issues a scale-to-one. The container
pulls from the regional mirror (~5s), mounts weights (~10s), boots
`mistralrs-server`. Total warm start: ~15s at zero idle cost.

## The rings

State is partitioned into three rings per YOYO-COMPUTE §1:

- **Ring 1** — bootstrap: container, weights, secrets. Survives
  teardown as artefacts.
- **Ring 2** — working memory: KV cache blocks in Mooncake Store.
  Survives teardown pooled across invocations.
- **Ring 3** — long-term: 3a is the LadybugDB graph (service-content's
  responsibility), 3b is the LoRA adapter stack (ours).

## The ledger

Every yo-yo event writes a row: `BOOT_REQUEST`, `BOOT_COMPLETE`,
`JOB_START`, `JOB_COMPLETE`, `CHECKPOINT`, `TEARDOWN_REQUEST`,
`TEARDOWN_COMPLETE`, `PREEMPTION`, `ADAPTER_LOAD`, `KV_POOL_SYNC`. This
is the SOC3 processing-integrity artefact and is what distinguishes
service-slm from "just a wrapper around the LLM API."

## Where it lives

- Ring 1: `crates/slm-compute/`
- Ring 2: `crates/slm-memory-kv/`
- Ring 3b: `crates/slm-memory-adapters/`
- Ledger: `crates/slm-ledger/`

Authoritative specification: [YOYO-COMPUTE.md](../../specs/YOYO-COMPUTE.md).
