# service-extraction

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

**Vendor:** PointSav Digital Systems
**Standard:** SYS-ADR-07 (Bifurcated Ingestion)
**Tier:** 5 — Service Logic

---

## What it does

`service-extraction` is the deterministic parser for the Totebox Archive. It reads raw `.eml` files from `service-email` or files committed through `service-input`, strips proprietary formatting (MIME multipart, Base64, HTML), and writes structured records to the WORM ledger.

Routing follows SYS-ADR-07: structured data — headers, recipients, signatures, CSV attachments — is parsed deterministically with zero AI dependency. Unstructured body text is handed off to `service-slm` only when that service is installed on the archive node. The base ToteboxOS tier runs this service without any AI component.

## Build

```
./build.sh
```

Produces two binaries: a native debug binary for local testing, and a statically linked MUSL binary for deployment to commodity cloud nodes.

## Run

```
./target/release/service-extraction <input.eml> <totebox_root>
```

Output goes to four locations under `<totebox_root>/service-fs/data/`, following the per-service WORM isolation pattern documented in the User Guide (Part VI):

| Path | Contents |
|---|---|
| `service-extraction/source/` | Raw vaulted `.eml`, SHA-256 sealed |
| `service-extraction/ledger/` | Extracted record index |
| `service-people/source/` | Personnel records (one JSON per person) |
| `service-content/source/` | Clean body text for downstream indexing |

## Test

```
./build.sh
mkdir -p /tmp/test-totebox
./target/release/service-extraction samples/sample.eml /tmp/test-totebox
ls /tmp/test-totebox/service-fs/data/service-people/source/
```

For full corpus validation against the 10-email test set, see `VALIDATION.md`.

## Status

- **v0.2** — current baseline. Regex-based, 571 lines. Validated qualitatively against 10 real `.eml` files (newsletter and business correspondence). Handles multi-language signatures, tracking-URL filtering, and newsletter classification.
- **v0.4** — in active development. Adds Aho-Corasick gazetteer, Cognitive Gravitation Model classifier, Shannon entropy boilerplate filter, and improved signature boundary detection. Target: >90% extraction fidelity on the test corpus.

See `ROADMAP.md` for the full technique inventory and phase plan.

## Integration

`service-extraction` is invoked by `spool-daemon.sh` watching `service-email/maildir/new/` and `service-input/source/`. It is a point-in-time process — one `.eml` in, structured records out, process exits. No persistent daemon, no network calls, no external dependencies.

The extraction engine is the backside of the F12 Input Machine pipeline (SYS-ADR-10). Every record committed through this service is a candidate for the Verification Surveyor workflow in `service-people`.

## Dependencies

All Apache 2.0 or MIT compatible. No GPL. No proprietary SDKs. See `Cargo.toml` for the current dependency set; changes to dependencies are subject to the stop conditions in `ROADMAP.md`.

## Legal

Refer to the `LICENSE` file in the monorepo root. This software is currently under Incubation Phase licensing. All rights reserved by Woodfine Capital Projects Inc.

---

*© 2026 PointSav Digital Systems™.*
