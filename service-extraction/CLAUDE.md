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

- 149-line `src/main.rs`. Single binary, standalone Rust workspace.
- Hardcoded deployment paths (see `BASE_DEPLOYMENT_DIR`, `TOTEBOX_SOURCE_DIR`).
- Not a workspace member — `cargo check` must run inside this directory.
- No tests.

## Build and run

```
cargo build            # inside service-extraction/
cargo build --release  # production build
```

Run (deployment directory must exist):
```
cargo run
```

The binary blocks on a `notify` filesystem watch. Drop `.json` files into
`TOTEBOX_SOURCE_DIR` to trigger processing. Idempotent: already-processed
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
- **Paths are deployment-scoped.** `BASE_DEPLOYMENT_DIR` and
  `TOTEBOX_SOURCE_DIR` are hardcoded — update before deploying to a new
  instance.
