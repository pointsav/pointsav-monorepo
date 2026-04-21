# HTTP API reference

Authoritative reference for the service-slm HTTP API exposed by the
`slm-api` crate. Phase 2 scaffolding note: endpoints are planned but
not yet implemented. The OpenAPI schema will be generated from the
`schemars`-derived types at build time once handlers ship.

## Base URL

Default: `http://127.0.0.1:8080`. Override with `server.listen` in the
config file.

## Authentication

In Phase 2, the API is expected to run behind an internal network
boundary. Token-based authentication is planned for a later ADR.

## Endpoints

### `GET /health`

Liveness probe. Always returns `200 OK` if the process is running.

### `GET /ready`

Readiness probe. Returns `200 OK` only if all subsystems are ready:

- Ledger opened and fsync-capable.
- HTTP listener bound.
- Adapter registry watch active.
- (Optional) GCP compute driver reachable.

Returns `503 Service Unavailable` with a JSON body listing which
subsystem is not ready, if any.

### `GET /metrics`

Prometheus scrape endpoint. See [operating.md](../user-guide/operating.md#metrics).

### `POST /doorman/cycle`

Run one doorman cycle. Sanitises, sends, awaits, receives,
rehydrates.

Request body (`application/json`):

```json
{
  "moduleId": "woodfine-v1",
  "payload": { ... },
  "policy": "default"
}
```

Response body:

```json
{
  "job_id": "01HK...",
  "result": { ... },
  "ledger_event_ids": ["01HK...", "01HK..."]
}
```

### `GET /ledger/events`

Query the ledger. Query parameters:

| Param | Type | Description |
|---|---|---|
| `moduleId` | string | Filter by module. |
| `event_type` | string | Filter by event type. |
| `since` | RFC 3339 | Events at or after this timestamp. |
| `until` | RFC 3339 | Events at or before this timestamp. |
| `limit` | int | Max events (default 100, max 1000). |

Response: array of events per the schema in
[reference/ledger-events.md](./ledger-events.md).

### `GET /node/status`

Return the current yo-yo node state.

### `POST /node/up`, `POST /node/down`

Imperative node control. Equivalent to the CLI subcommands.

### `GET /adapters`, `GET /adapters/{id}`

List and inspect registered adapters.

## Errors

Standard HTTP status codes. Error responses use
[RFC 7807 Problem Details](https://www.rfc-editor.org/rfc/rfc7807):

```json
{
  "type": "https://slm.pointsav.io/errors/ledger-fsync-failed",
  "title": "Ledger fsync failed",
  "status": 500,
  "detail": "underlying OS error: EIO",
  "instance": "/doorman/cycle/01HK..."
}
```
