# CLI reference

Authoritative reference for `slm-cli` subcommands and flags.

Phase 2 scaffolding note: subcommands are defined but not yet
implemented. This document tracks the intended surface. Generated from
`clap` definitions in `crates/slm-cli/src/main.rs`.

## Global flags

| Flag | Default | Description |
|---|---|---|
| `-v`, `--verbose` | `0` | Repeat for more detail. `-v` = debug, `-vv` = trace. |
| `--json-logs` | off | Emit logs as one-event-per-line JSON. |
| `-c`, `--config <PATH>` | auto | Path to TOML config. See [configuration.md](../user-guide/configuration.md). |
| `--help` | — | Print help. |
| `--version` | — | Print version. |

## Subcommands

### `slm-cli serve`

Start the HTTP API and background workers. The primary operational
mode; what systemd invokes.

### `slm-cli doorman --input <PATH>`

Run one doorman cycle end-to-end against a given payload. For
testing, debugging, and one-off runs.

| Flag | Default | Description |
|---|---|---|
| `-i`, `--input <PATH>` | (required) | Input document to sanitise and send. |

### `slm-cli ledger tail [-n N]`

Print recent ledger events.

| Flag | Default | Description |
|---|---|---|
| `-n`, `--n` | `50` | How many events to show. |

### `slm-cli ledger export --out <PATH>`

Export the ledger in CSV for external audit tooling.

| Flag | Default | Description |
|---|---|---|
| `-o`, `--out <PATH>` | (required) | Destination file. |

### `slm-cli node up | down | status`

Manage the yo-yo compute node.

- `up` — spin up the GCP node.
- `down` — tear down the GCP node.
- `status` — report current node state.

### `slm-cli adapter list | verify <ID>`

Manage LoRA adapters.

- `list` — list registered adapters.
- `verify <ID>` — verify the Sigstore signature of an adapter.

## Exit codes

| Code | Meaning |
|---|---|
| `0` | Success |
| `1` | Generic error |
| `2` | Bad arguments |
| `64` | Configuration error |
| `65` | Input data error |
| `66` | Cannot open input file |
| `69` | Service unavailable (GCP node, Mooncake master) |
| `70` | Internal software error |
| `71` | System error |
| `75` | Temporary failure (retry may succeed) |

These follow the BSD `sysexits.h` conventions where possible.
