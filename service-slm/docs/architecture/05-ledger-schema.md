# The ledger schema

Every yo-yo event writes exactly one row to the append-only audit
ledger. The authoritative schema lives in
[YOYO-COMPUTE §5](../../specs/YOYO-COMPUTE.md); this document is the
implementation-facing summary.

## Columns

```csv
event_id, timestamp_utc, event_type, moduleId, node_id, job_id,
input_hash, adapter_versions, cache_hit_ratio, tokens_processed,
gpu_seconds, cost_usd, completion_status, error_code, operator_id
```

| Column | Type | Notes |
|---|---|---|
| `event_id` | UUIDv7 | Sortable by generation time |
| `timestamp_utc` | RFC 3339 | Nanosecond precision |
| `event_type` | enum | One of 10 variants (see below) |
| `moduleId` | string | Per YOYO-COMPUTE §6 |
| `node_id` | string | Cloud Run revision or Totebox host |
| `job_id` | string | Links events within one doorman cycle |
| `input_hash` | BLAKE3 hex | Stable across re-runs |
| `adapter_versions` | string | Comma-separated `id@version` list |
| `cache_hit_ratio` | float | 0.0–1.0 |
| `tokens_processed` | u64 | Input + output tokens |
| `gpu_seconds` | float | From Cloud Run billing API |
| `cost_usd` | decimal | Matches billing line-item |
| `completion_status` | enum | `success` / `error` / `timeout` / `preempted` |
| `error_code` | string | Empty for success |
| `operator_id` | string | Empty for automated events |

## Event types

The ten `event_type` values are:

1. `BOOT_REQUEST` — a spin-up was requested.
2. `BOOT_COMPLETE` — the node is serving.
3. `JOB_START` — a doorman cycle has begun.
4. `JOB_COMPLETE` — the cycle finished.
5. `CHECKPOINT` — GCS checkpoint written.
6. `TEARDOWN_REQUEST` — explicit tear-down issued.
7. `TEARDOWN_COMPLETE` — node is gone; final cost recorded.
8. `PREEMPTION` — spot instance preempted.
9. `ADAPTER_LOAD` — a LoRA adapter was activated.
10. `KV_POOL_SYNC` — Mooncake Store reconciliation.

## Storage

The authoritative form is an append-only CSV with fsync on commit. A
SQLite table mirrors it for queryable audit — but the CSV wins any
disagreement. Reconciliation is via `slm-cli ledger reconcile`.

## Why this matters

The ledger is the Processing Integrity trust-service criterion of SOC3
made concrete. It ties every model output back to:

- The exact input (via `input_hash`).
- The exact adapter versions at answer time.
- The fraction of the answer that came from cached KV blocks
  (`cache_hit_ratio`) versus fresh prefill.
- The exact cost (`gpu_seconds`, `cost_usd`).
- The exact node that served it (`node_id`).

Vertex AI and equivalent hyperscaler endpoints cannot produce this
information. That is the structural product argument.
