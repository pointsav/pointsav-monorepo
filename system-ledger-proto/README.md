# system-ledger-proto

Transport-agnostic wire types for Capability Ledger consultation.

`ConsultRequest` and `ConsultResponse` are postcard-serialized identically whether the transport is a Unix socket (NetBSD compat bottom) or a seL4 Microkit PPC shared-memory ring (seL4 native bottom). The same bytes move over both carriers; the business logic in `system-ledger` is invoked identically in both cases.

## Usage

```toml
# std (default)
system-ledger-proto = { path = "../system-ledger-proto" }

# no_std + alloc (seL4 PD)
system-ledger-proto = { path = "../system-ledger-proto", default-features = false, features = ["sel4"] }
```

## Features

| Feature | Use |
|---|---|
| `std` (default) | Standard library — use in `system-ledger-server` and tests |
| `alloc` | `no_std` + alloc — use in embedded/rump contexts |
| `sel4` | `no_std` + alloc + seL4 compat primitives from `system-core` |

## Wire format

`postcard` encoding (little-endian, compact, `no_std`-compatible). Length prefix is the responsibility of the transport layer (Unix socket framing in `system-ledger-server`; Microkit MR length word in `system-ledger-pd`).

## License

AGPL-3.0-or-later. See [LICENSE](../LICENSE).
