# tool-llm-bridge

Tier X — an isolated external-LLM research/code-assist bridge. Runs as its own
process, separate from the core sovereign loop (service-content, service-slm),
so that even if the core loop's process were compromised it could not read the
real upstream API key.

See `BRIEF-os-totebox-platform.md` §7 and §14 #12 for the full design rationale
and its relationship to SYS-ADR-07.

## What it is for

Advisory research and code-writing assistance for this archive's own
development. **Never** used for entity extraction, DataGraph writes, or
training-signal generation — this crate has no DataGraph client dependency at
all, by construction.

## Design

- **Sidecar credential isolation.** Only this process holds the real upstream
  API key (`LLM_BRIDGE_PROVIDER_API_KEY`). Callers authenticate with a
  separate, unrelated local-only token (`LLM_BRIDGE_ACCESS_TOKEN`) and never
  see the real key — it is injected onto the outbound request here and never
  echoed back in any response.
- **Per-label allowlist**, checked before any network attempt — mirrors
  `service-slm`'s existing Tier C design (`slm-doorman/src/tier/external.rs`).
  Empty by default: no label is permitted until explicitly configured.
- **Tag gate.** Every request must carry an explicit `tag`: `local-only` or
  `cloud-allowed`. Only `cloud-allowed` requests may ever reach the upstream
  provider — this is a structural check, not a caller convention.

## Configuration (environment variables)

| Variable | Required | Default | Purpose |
|---|---|---|---|
| `LLM_BRIDGE_BIND_ADDR` | no | `127.0.0.1:9210` | HTTP bind address |
| `LLM_BRIDGE_ACCESS_TOKEN` | **yes** | — | Local-only bearer token callers must present |
| `LLM_BRIDGE_PROVIDER_ENDPOINT` | no | empty (refuses all requests) | Upstream provider URL |
| `LLM_BRIDGE_PROVIDER_API_KEY` | no | empty (refuses all requests) | Real upstream credential |
| `LLM_BRIDGE_ALLOWED_LABELS` | no | empty (refuses all requests) | Comma-separated label allowlist |

## API

`POST /v1/bridge/complete`

```json
{
  "tag": "cloud-allowed",
  "label": "research",
  "payload": { "...": "forwarded verbatim to the upstream provider" }
}
```

`GET /healthz` — liveness check.

## Status

Phase 1 of `BRIEF-os-totebox-platform.md` §16's execution plan. MVP: process +
credential isolation, allowlist, tag gate. Not yet deployed anywhere; not yet
in `.agent/binary-targets.yaml` (declare before shipping — see
`conventions/soft-distribution-pipeline.md` §3, `soft_enabled: false` is
correct until this is production-ready).
