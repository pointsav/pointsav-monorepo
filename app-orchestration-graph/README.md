# app-orchestration-graph

Cross-Totebox DataGraph federation gateway for the PointSav os-orchestration layer.

**Status: Reserved-folder.** The DataGraph federation function currently lives inside
`app-orchestration-slm` (`POST /v1/graph/federated`). This project activates when
that workload grows large enough to warrant extraction into a dedicated service.

**Licence:** Proprietary.

## What it will do

`app-orchestration-graph` is the dedicated gateway through which `os-orchestration`
accesses DataGraphs across multiple Totebox Archives simultaneously. Each Totebox
Archive maintains a sovereign `service-content` DataGraph — entities, relationships,
and corpus metadata specific to that archive's domain. No single Totebox can query
another archive's DataGraph directly. All cross-archive DataGraph access passes
through this gateway.

## Why a separate service

`app-orchestration-slm` is the inference broker — its job is routing inference
requests to the right compute tier (Tier A local, Tier B Yo-Yo, Tier C Anthropic).
It currently also handles DataGraph federation via `POST /v1/graph/federated`, which
fans out to all registered Totebox Doormen and aggregates results.

When the fleet is small (2–5 Toteboxes), collocating federation and inference is fine.
As the fleet grows, DataGraph fanout queries become a separate high-volume workload
with different latency, caching, and connection-pool requirements from inference. At
that point, mixing them degrades both.

`app-orchestration-graph` extracts federation into its own process:
- Maintains persistent connections to all registered Totebox `service-content` endpoints
- Handles fanout, result aggregation, partial-failure tolerance, and result caching
- Relieves `app-orchestration-slm` of DataGraph concern entirely

## What it is NOT

- Not a DataGraph store. It holds no entity data of its own. All data stays sovereign
  in each Totebox's `service-content`.
- Not a sync service. It does not replicate DataGraphs between Toteboxes. Queries are
  pull-on-demand; nothing is pushed to or from the gateway.
- Not `app-orchestration-content`. That name was considered and rejected — it would be
  confused with `service-content` (the per-Totebox store). This service is a graph
  federation gateway, not a content store.

## Relationship to other services

| Service | Role | Relation |
|---|---|---|
| `service-content` | Per-Totebox DataGraph store | Data source — queried by this gateway |
| `app-orchestration-slm` | Inference broker | Currently holds federation logic; hands off on activation |
| `service-slm` (Doorman) | Per-Totebox inference router | Has DataGraph query endpoints proxied by this gateway |

## Port

`:9181` (planned; `:9180` is `app-orchestration-slm`)

## Activation threshold

Extract from `app-orchestration-slm` when any of these are true:
- Fleet exceeds ~10 Totebox Archives with active DataGraphs
- DataGraph fanout queries measurably affect inference broker latency
- A second consumer of cross-Totebox DataGraph access emerges (not just the inference path)

Until then, `POST /v1/graph/federated` on `app-orchestration-slm` covers the use case.
