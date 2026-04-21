# Reference

Precise, look-up-oriented documentation for service-slm.

- [CLI](./cli.md) — `slm-cli` subcommands and flags.
- [HTTP API](./http-api.md) — the axum server's endpoints.
- [Ledger events](./ledger-events.md) — the ten event types.
- [Configuration schema](./configuration-schema.md) — TOML keys,
  validation, environment overrides.

Where these documents overlap with prose in
[user-guide/](../user-guide/), the reference is authoritative on the
precise form (column name, flag name, JSON shape) and the user-guide
is authoritative on how to use it.

These pages will eventually be generated from source (clap
definitions, schemars schemas, Event type derives). Until then they
are hand-maintained and CI verifies that the hand-maintained version
does not drift from the code.
