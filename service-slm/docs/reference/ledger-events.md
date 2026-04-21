# Ledger events reference

Authoritative reference for the ten ledger event types. The source of
truth is [YOYO-COMPUTE §5](../../specs/YOYO-COMPUTE.md); this document
is the implementation-facing catalogue.

## Column spec

See [architecture/05-ledger-schema.md](../architecture/05-ledger-schema.md)
for the column list and types. All columns are present on every row;
columns irrelevant to a given event type are empty strings (CSV) or
`null` (SQLite).

## Event types

### `BOOT_REQUEST`

Emitted when `slm-compute` issues a scale-to-one command.

| Column | Value |
|---|---|
| `node_id` | Cloud Run service name |
| `job_id` | Empty (no doorman cycle yet) |
| `adapter_versions` | Intended stack |
| `completion_status` | `pending` |

### `BOOT_COMPLETE`

Emitted when the container is serving and the health check has
passed.

| Column | Value |
|---|---|
| `node_id` | Cloud Run revision id |
| `gpu_seconds` | Elapsed boot time (measures cold start) |
| `completion_status` | `success` or `error` |

### `JOB_START`

Emitted when a doorman cycle begins.

| Column | Value |
|---|---|
| `job_id` | UUIDv7 for this cycle |
| `input_hash` | BLAKE3 of sanitised payload |
| `adapter_versions` | Active adapters |

### `JOB_COMPLETE`

Emitted when the cycle finishes.

| Column | Value |
|---|---|
| `job_id` | Matches the `JOB_START` row |
| `tokens_processed` | Input + output token count |
| `cache_hit_ratio` | 0.0–1.0 |
| `gpu_seconds` | Job-scoped GPU time |
| `cost_usd` | Job-scoped cost |
| `completion_status` | `success` / `error` / `timeout` |

### `CHECKPOINT`

Emitted when a GCS checkpoint is written.

| Column | Value |
|---|---|
| `node_id` | Node that wrote the checkpoint |

### `TEARDOWN_REQUEST`

Emitted when `slm-compute` issues scale-to-zero.

### `TEARDOWN_COMPLETE`

Emitted when the node is confirmed gone.

| Column | Value |
|---|---|
| `gpu_seconds` | Lifetime total for the node |
| `cost_usd` | Lifetime total for the node |

### `PREEMPTION`

Emitted when a spot instance is preempted. Differs from
`TEARDOWN_COMPLETE` in that `completion_status = preempted` and the
node is not expected to come back automatically.

### `ADAPTER_LOAD`

Emitted when a LoRA adapter is activated for a request.

| Column | Value |
|---|---|
| `adapter_versions` | Full `id@version` of the loaded adapter |
| `moduleId` | The module requesting it |

### `KV_POOL_SYNC`

Emitted when Mooncake Store performs a reconciliation.

| Column | Value |
|---|---|
| `cache_hit_ratio` | Instantaneous value at sync time |

## Invariants

- Every `BOOT_REQUEST` eventually pairs with exactly one of
  `BOOT_COMPLETE` (success) or `TEARDOWN_COMPLETE` (if boot failed).
- Every `JOB_START` pairs with exactly one `JOB_COMPLETE`.
- Every `TEARDOWN_REQUEST` pairs with exactly one
  `TEARDOWN_COMPLETE` or a `PREEMPTION`.
- `node_id` is stable within a single boot lifecycle.
- `moduleId` is present on every event except workspace-level events
  where no specific module is in scope (those are rare).

## Auditing

A monthly reconciliation across the ledger should:

- Confirm no unpaired `BOOT_REQUEST` / `JOB_START` /
  `TEARDOWN_REQUEST` rows exist.
- Confirm `cost_usd` sums match the GCP billing export for the
  same time window, within a tolerance of a few percent
  (rounding differences).
- Confirm every `ADAPTER_LOAD` refers to an adapter present in the
  registry at the event timestamp.
