# system-ledger-server

Unix socket daemon exposing `InMemoryLedger` consultation on the NetBSD compat bottom.

Shares the `system-ledger-proto` wire format with `system-ledger-pd` (seL4 Microkit PD). Transport changes between the two; protocol and business logic do not.

## Frame format

Each message is a 4-byte LE length prefix followed by a postcard payload. The seL4 PD uses the same header in its shared-memory ring.

## Configuration

| Env var | Default | Purpose |
|---|---|---|
| `LEDGER_SOCK` | `/run/system-ledger/ledger.sock` | Unix socket path |

## NetBSD rc.d

See `scripts/rc.d/system_ledger` in `os-totebox/`.

## Build

```
cargo build --release -p system-ledger-server
```

## Test

```
cargo test -p system-ledger-server
```

## License

AGPL-3.0-or-later. See [LICENSE](../LICENSE).
