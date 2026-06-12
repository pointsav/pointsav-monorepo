# system-ledger-pd

seL4 Microkit Protection Domain wrapping `InMemoryLedger`. Receives `ConsultRequest`
via PPC shared-memory ring on channel 1; returns `ConsultResponse` on the same ring.

Same wire format as `system-ledger-server` (Unix socket compat bottom): 4-byte LE
length-prefix + postcard payload. Transport changes; protocol does not.

## Build

Requires Microkit SDK v2.1.0 (x86\_64\_generic board):

```sh
SDK_PATH=~/microkit-sdk-2.1.0 make
```

CI check (no SDK required — verifies no std-creep in upstream crates):

```sh
cargo build --no-default-features --features sel4 --target x86_64-unknown-none
```

## Run

```sh
qemu-system-x86_64 -kernel build/final_image.elf -nographic -m 512M
```

Expected output: `LEDGER PD: online` then `VERDICT: Allow` from the test client.

## Architecture

```
client_pd  (priority 200)
  │  writes ConsultRequest to cap_request_mr (0x4001000, 16 KiB)
  │  issues microkit_ppcall(1, msginfo)
  ↓
system_ledger  (priority 254)
  │  reads ConsultRequest from cap_request_mr
  │  calls InMemoryLedger::consult_capability
  │  writes ConsultResponse to cap_response_mr (0x4005000, 4 KiB)
  │  returns from protected()
  ↓
client_pd
  │  reads ConsultResponse from cap_response_mr
```

See `ledger.system` for the Microkit system description.
