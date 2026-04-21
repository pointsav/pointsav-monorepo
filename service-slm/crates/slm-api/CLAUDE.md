# CLAUDE.md — slm-api

You are working on the inbound HTTP surface. Handlers stay thin; real
work happens in the libraries they call.

## What this crate owns

- axum router and handler signatures.
- Tower middleware stack.
- OpenAPI schema.
- Health, readiness, metrics endpoints.

## Invariants specific to this crate

1. **Handlers are thin.** A handler parses input, calls one service,
   and formats the response. Anything else is a sign that logic is
   leaking into the wrong layer.
2. **Every request carries a trace id.** Via `tracing::Span` on the
   tower layer.
3. **Validation happens at the edge.** Use `validator` on request
   bodies. Reject before reaching domain code.
4. **Metrics are Prometheus.** Via `metrics` crate. Every handler
   emits a latency histogram and a counter.

## Next work units

See `TASKS.md`. First real task: the router skeleton with a `/health`
handler and a `#[tracing::instrument]` wrapper.
