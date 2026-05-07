# CLAUDE.md — service-extraction

> **State:** Active  —  **Last updated:** 2026-05-07
> **Registry row:** `.agent/rules/project-registry.md`

---

## What this project is

`service-extraction` is a filesystem-watching Rust binary that ingests
structured JSON payloads and extracts entity records into deployment ledgers.
Each payload (dropped into `TOTEBOX_SOURCE_DIR`) contains a base64-encoded
source file plus pre-computed edge AI entity extractions. The binary decodes
the file, parses origin metadata (email headers via `mailparse`), merges with
the edge AI entities, and writes CRM ledger JSON files into the woodfine
deployment directory tree.

It is not a general-purpose extraction engine. It is a narrow pipeline stage:
incoming JSON file in → ledger JSON files out.

## Current state

- ~195-line `src/main.rs`. Single binary, standalone Rust workspace.
- All paths parameterized via env vars (see Configuration below).
- CORPUS bridge: when `EXTRACTION_EMIT_CORPUS_DIR` is set, emits `CORPUS_*.json`
  alongside `CRM_*.json` for service-content DataGraph ingestion.
- Not a workspace member — `cargo check` must run inside this directory.
- No tests.

## Configuration (env vars)

| Variable | Default | Purpose |
|---|---|---|
| `EXTRACTION_BASE_DIR` | `/home/mathew/deployments/woodfine-fleet-deployment` | Deployment root; used to construct CRM ledger output paths |
| `EXTRACTION_WATCH_DIR` | `${EXTRACTION_BASE_DIR}/cluster-totebox-personnel-1/service-fs/data/service-people/source` | Directory watched for incoming JSON payloads |
| `EXTRACTION_EMIT_CORPUS_DIR` | *(unset — disabled)* | When set, also write `CORPUS_*.json` for service-content DataGraph ingestion |
| `EXTRACTION_CORPUS_MODULE_ID` | *(unset)* | When set with `EXTRACTION_EMIT_CORPUS_DIR`, embeds `module_id` in CORPUS JSON (otherwise service-content uses its own `SERVICE_CONTENT_MODULE_ID`) |

Systemd unit for the jennifer instance: `service-slm/compute/systemd/local-extraction-jennifer.service`.

## Build and run

```
cargo build            # inside service-extraction/
cargo build --release  # production build
```

Run (deployment directory must exist):
```
EXTRACTION_WATCH_DIR=/path/to/source cargo run
```

The binary blocks on a `notify` filesystem watch. Drop `.json` files into
`EXTRACTION_WATCH_DIR` to trigger processing. Idempotent: already-processed
filenames are tracked in-memory for the process lifetime.

## File layout

```
service-extraction/
├── Cargo.toml         standalone workspace; not in monorepo members
├── Cargo.lock
├── README.md
├── CLAUDE.md          this file
└── src/
    └── main.rs        all logic; ~149 lines
```

## Hard constraints

- **Do not add network calls.** Extraction is a local, offline operation.
- **Do not add a database.** Ledger output is flat-file JSON; `service-fs`
  is the persistence layer above.
- **All paths via env vars.** `EXTRACTION_BASE_DIR`, `EXTRACTION_WATCH_DIR`,
  `EXTRACTION_EMIT_CORPUS_DIR` — never hardcode deployment paths in source.
