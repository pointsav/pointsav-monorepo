# service-extraction

> **State:** Active | **Last updated:** 2026-05-24

## Mission

Filesystem-watching pipeline that ingests email-payload JSON files, extracts CRM
entities (sender header + edge-AI inference), and writes ledger records for
`service-people`.

## Runtime shape

Single binary; no HTTP server; pure filesystem I/O.

- **Watches:** `<BASE_DEPLOYMENT_DIR>/cluster-totebox-personnel-1/service-fs/data/service-people/source/`
- **Input format:** `.json` files with shape:
  ```json
  { "file": { "filename": "…", "data": "<base64 email>" },
    "destination_archive": "cluster-totebox-personnel-1",
    "target_service": "service-people",
    "edge_entities": [{ "entity_name": "…", "classification": "…", "confidence": 0.9 }] }
  ```
- **Output:** `<BASE_DEPLOYMENT_DIR>/<dest_archive>/service-fs/data/<target_service>/ledgers/CRM_<worm_id>.json`
- 250 ms debounce to avoid partial-write races; deduplication via processed-filename list.

## Two extraction passes per payload

1. **Sender anchor** — From header parsed via regex; confidence 1.0; classification `ORIGIN SENDER`
2. **Edge entities** — trusted from `edge_entities` array (WASM AI inference); confidence pass-through

## Open items

- Deployment paths hardcoded (`BASE_DEPLOYMENT_DIR`); move to env var or config file before next activation
- Silent drop on malformed payloads — add structured error logging
- Not a workspace member (has own `[workspace]` in `Cargo.toml`); unify with monorepo workspace when ready

## Constraints

- `~/Foundry/CLAUDE.md` §6 rules apply (Bloomberg standard, BCSC posture, ADR-07/10/19)
- Commits via `~/Foundry/bin/commit-as-next.sh`
