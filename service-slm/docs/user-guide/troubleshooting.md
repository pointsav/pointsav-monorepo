# Troubleshooting

First-line responses to common failure modes. For anything not
covered here, open an issue with the output of `slm-cli node status`
and the last few `slm-cli ledger tail` entries.

## The service will not start

### "config not found"

The config file path is wrong. Check the search order in
[configuration.md](./configuration.md).

### "ledger path not writable"

The user running `slm-cli` cannot write to `ledger.csv_path`. Fix:
create the directory with the correct owner; do not run as root in
production.

### "port in use"

Another process has the listen port. `ss -tlnp | grep :<port>` finds
the culprit.

## The yo-yo node will not come up

### `BOOT_REQUEST` written but no `BOOT_COMPLETE`

Check:

1. GCP credentials. `gcloud auth list` — service account must have
   `roles/run.developer` and `roles/storage.objectViewer`.
2. The container image. `gcloud artifacts docker images list <path>`
   — must show a current image.
3. The weights. `gsutil ls gs://<bucket>/models/<variant>/` — must
   list model files.
4. The `cold_start_timeout_seconds` config. Default is 60s; some
   regions cold-start slower.

### `PREEMPTION` event repeatedly

Spot instances are being pulled. Either accept it (batch workloads
usually can) or switch to `spot_allowed = false` in the compute
manifest.

## Inference is slow

### Cache hit ratio low

Run `slm-cli node status` and look at the KV section. If
`moduleId` is changing between requests, the cache cannot help — this
is usually a caller misconfiguration. If the Mooncake master is
unreachable, `slm-cli memory kv ping` should tell you.

### High variance in latency

Check `slm_doorman_cycle_duration_seconds` percentiles. If p99 is
significantly worse than p50, it is usually a cold-start issue (see
above) or a preemption.

## Adapters will not load

### Signature verification failed

A signature rotation has happened upstream. Run
`slm-cli adapter verify <id>` for the full error. If the expected
signer has changed, an ADR is needed — do not disable
`verify_signatures` in config.

### Adapter not found

Check the registry path in config and the adapter `id@version` in
the registry file. `slm-cli adapter list` enumerates what the
service thinks is available.

## Ledger irregularities

### `CSV and SQLite disagree`

Run `slm-cli ledger reconcile`. This replays the CSV into SQLite; the
CSV is authoritative.

### Missing events

The ledger is append-only; events should never be missing. If you
are certain an event is missing, there is likely a bug — file it as
a P0 with the surrounding context.

## Shutdown hangs

The shutdown grace period is waiting for:

- An in-flight doorman cycle (wait or cancel; see `server.shutdown_grace_seconds`).
- An in-flight `TEARDOWN_REQUEST` (takes up to 30 seconds; if longer,
  the GCP API may be degraded — check the GCP status page).

`SIGKILL` forces immediate exit but may leave the ledger unflushed.
Use only after confirming the flush has happened.

## Something you cannot explain

- Collect: `slm-cli node status`, last 50 ledger events, logs for
  the past 10 minutes.
- Open a GitHub issue with those attached.
- For security-sensitive situations, email peter@woodfinegroup.com
  per [SECURITY.md](../../SECURITY.md).
