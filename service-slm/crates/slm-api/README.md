# slm-api

The axum-based HTTP server. Inbound surface of service-slm.

Per SLM-STACK §3.2: axum for HTTP, tower for middleware, tokio for the
runtime, hyper underneath. tonic is available if we add gRPC later.

## What lives here

- The axum router and handlers.
- Tower middleware: tracing, metrics, rate limiting, auth.
- OpenAPI schema generation (via `schemars`).
- The health and readiness endpoints.
- Prometheus metrics exposition.

## What does not live here

- Any inference logic. Handlers call into `slm-doorman` and
  `slm-inference-{local,remote}`.
- Any ledger writes. Those happen inside the services the handlers call.
- The CLI entry point — that's `slm-cli`.
