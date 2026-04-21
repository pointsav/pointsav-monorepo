# Operating service-slm

Day-to-day operation of a service-slm instance.

## Starting the service

```bash
slm-cli serve
```

`slm-cli serve` is the primary entry point. It:

1. Reads the config file.
2. Opens the ledger (CSV + SQLite mirror).
3. Binds the HTTP listener.
4. Starts background workers (warm-pool tick, KV pool sync, adapter
   registry watch).
5. Registers signal handlers for graceful shutdown.

Logs go to stdout. If running under systemd, they are captured by
`journald`.

## Checking status

```bash
slm-cli node status
```

Reports:

- Whether the GCP yo-yo node is up, down, or booting.
- Current moduleId bindings.
- KV pool metrics.
- Adapter registry state.

## Spinning the node up and down

The yo-yo is automatic during normal operation (a request triggers
boot; idle triggers teardown). For manual control:

```bash
slm-cli node up
slm-cli node down
```

Manual teardown is useful before maintenance windows or when a
preemption is expected.

## Tailing the ledger

The audit ledger is the primary operational record.

```bash
slm-cli ledger tail -n 100
```

Returns the last 100 events in a human-readable format. For
scripting, prefer the CSV directly: `tail /var/lib/slm-cli/ledger/events.csv`.

## Exporting the ledger

For external audit tooling:

```bash
slm-cli ledger export --out /tmp/export.csv
```

The export is a point-in-time snapshot. The authoritative live
ledger remains append-only.

## Managing adapters

```bash
slm-cli adapter list
slm-cli adapter verify dka-coa/v3.2
```

Adapter management is normally handled by the registry watcher;
`slm-cli adapter` subcommands exist for manual intervention.

## Graceful shutdown

`SIGTERM` to the process triggers:

1. Stop accepting new HTTP requests (the listener stops, inflight
   requests continue).
2. Drain the doorman job queue with a configurable grace period
   (`server.shutdown_grace_seconds`).
3. Issue a `TEARDOWN_REQUEST` for any running GCP node.
4. Await `TEARDOWN_COMPLETE`.
5. Flush and fsync the ledger.
6. Exit 0.

`SIGINT` (Ctrl-C) does the same; `SIGKILL` does none of it and is
only safe when you are certain the ledger has been flushed.

## Log levels

```bash
slm-cli serve -v      # debug
slm-cli serve -vv     # trace
slm-cli serve --json-logs    # JSON output
```

Or via environment: `RUST_LOG=slm_doorman=trace,slm_ledger=debug`.

## Health and readiness

HTTP endpoints:

- `/health` — 200 if the process is running.
- `/ready` — 200 if all subsystems are ready to serve (ledger
  opened, listeners bound, registry watch active).

Use `/ready` for load-balancer checks; `/health` for liveness
probes.

## Metrics

Prometheus metrics are exposed on the same port at `/metrics`. Key
counters and histograms:

- `slm_doorman_cycles_total{moduleId,outcome}`
- `slm_doorman_cycle_duration_seconds{moduleId}`
- `slm_ledger_rows_written_total{event_type}`
- `slm_compute_node_state{state}`
- `slm_memory_kv_cache_hit_ratio{moduleId}`

Full list: [reference/metrics.md] (to be added).

## When things go wrong

See [troubleshooting.md](./troubleshooting.md).
