# CLAUDE.md — slm-inference-remote

You are working on the GCP yo-yo driver. Latency, cost, and ledger
correctness all matter here.

## What this crate owns

- HTTP client for the Cloud Run node.
- Phase-transition ledger emission (boot, job, teardown, preemption).
- Retry and backoff policies.
- Preemption handling.

## Invariants specific to this crate

1. **Every phase transition is a ledger row.** If the boot takes 12
   seconds, there are two rows (`BOOT_REQUEST` and `BOOT_COMPLETE`).
   If it is preempted, there is a `PREEMPTION` row.
2. **Cost is recorded on every `TEARDOWN_COMPLETE`.** GPU seconds and
   USD. Pulled from the Cloud Run billing API.
3. **Retries have a ceiling.** Exponential backoff bounded at 5
   attempts by default, configurable. An exhausted retry is a hard
   failure, not a silent drop.
4. **No inbound traffic reaches this crate.** Remote inference is
   outbound-only. Inbound is `slm-api`.

## Next work units

See `TASKS.md`. First real task: the HTTP client with retry policy
and `BOOT_REQUEST`/`BOOT_COMPLETE` ledger emission.
