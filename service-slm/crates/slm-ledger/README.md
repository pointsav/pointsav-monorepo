# slm-ledger

The SOC3 processing-integrity artefact for service-slm.

Every yo-yo event — boot request, boot complete, job start, job complete,
checkpoint, teardown, preemption, adapter load, KV pool sync — writes one
row to an append-only CSV. The CSV is mirrored into a SQLite table for
queryable audit. The schema is specified in
[YOYO-COMPUTE §5](../../specs/YOYO-COMPUTE.md).

## Why this is a crate, not a logger call

Ledger rows have legal and commercial weight. They are the per-call audit
trail that ties every inference output back to its source chunks,
adapter versions, cache-hit ratio, GPU seconds, and cost. This is the DARP
I1 compliance chain extended from source-to-graph into source-to-output,
and it is the single biggest structural argument for why PointSav data
commands a premium price.

A `tracing::info!` call is not a ledger row. Do not confuse them.

## What lives here

- The `Event` struct with all 10 `event_type` variants.
- The append-only CSV writer (crash-safe, fsync-on-commit).
- The SQLite mirror with indices on `moduleId`, `timestamp_utc`, `job_id`.
- Export helpers for external audit tooling.

## What does not live here

- The `tracing` subscriber. Operational logs and ledger rows are
  different things.
- Metrics export. That's Prometheus via `metrics-exporter-prometheus` in
  `slm-api`.
