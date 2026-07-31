# service-input

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

Input Machine backend for the `cluster-totebox-jennifer-2` deployment.
Batch-migrates legacy files from the jennifer-1 deployment through the
current ingest pipeline into `service-fs` (or directly into a CORPUS
directory for extraction), and scores extraction output against
hand-curated reference data to track pipeline calibration over time.
Live on port 9106.

## History note

An earlier design for this crate — a generic Ring 1 boundary-ingest
service for multi-format document parsing (PDF/DOCX/XLSX/Markdown) —
was fully built and tested prior to this archive's 2026-06-20 merge,
but that implementation was never carried forward. This crate's real,
current purpose has been under continuous development since
2026-06-14 and is unrelated to that earlier design.

## Endpoints

| Endpoint | Method | Purpose |
|---|---|---|
| `/healthz` | GET | Liveness + queue/done counts |
| `/v1/status` | GET | Phase-1/phase-2 migration progress |
| `/v1/append` | POST | Forward a pre-formed payload to `service-fs` |
| `/v1/migrate` | POST | Batch-migrate legacy jennifer-1 files; resumable |
| `/v1/eval/:stem` | GET | Score one document's extraction output against reference data |
| `/v1/calibration-report` | GET | Aggregate calibration score across all stems |

## State

Active. Live on :9106.

## Licence

Refer to the repo `LICENSE` file. Component-level licence assignment
is governed by `pointsav/factory-release-engineering`'s
`LICENSE-MATRIX.md`.
