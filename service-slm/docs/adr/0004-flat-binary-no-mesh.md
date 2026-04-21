# ADR-0004: Flat binary, no microservice mesh

- **Status:** accepted
- **Date:** 2026-04-20
- **Deciders:** Peter M. Woodfine, Jennifer Woodfine
- **See also:** [ADR-0001](./0001-rust-end-to-end.md)

## Context

It is tempting, on a modern cloud platform, to decompose service-slm into
a mesh: a doorman microservice, a ledger microservice, a compute
controller, an adapter registry, separate inference proxies for local and
remote, and an HTTP gateway. The operational cost of that shape is not
obvious until you run it.

service-slm has specific characteristics that make the mesh shape wrong:

- It must ship as an os-totebox appliance component, where "one binary
  to install, start, stop, update, sign" is a hard requirement.
- Inter-crate communication within the service is function-call
  latency, not network-hop latency. A mesh would introduce serialisation
  and RPC cost at every boundary for no correctness gain.
- A single trace spanning "doorman → sanitise → send → receive →
  rehydrate → ledger" is trivial inside one process and operationally
  expensive across six processes.

## Decision

The service-slm workspace produces **exactly one binary**: `slm-cli`.
All logical modules — doorman, ledger, compute driver, memory layers,
inference engines, HTTP API — are library crates linked into that
binary. Communication between them is Rust function calls.

External boundaries, and only external boundaries, cross the network:

- The Mooncake Store sidecar (wire protocol per SLM-STACK §4.1).
- The Cloud Run GPU node (HTTP).
- The Claude API or other LLM endpoints (HTTPS).
- LadybugDB (bolt-style wire protocol).
- The inbound axum HTTP surface exposed to `service-content` and
  `os-console`.

Adding a second binary requires a superseding ADR.

## Rationale

- **Appliance fit.** The os-totebox product expects one systemd unit
  per service component. One binary is the correct shape for that
  target.
- **Signing clarity.** One Sigstore-signed artefact, one SLSA
  attestation, one SBOM. This is the chain-of-custody story
  institutional customers ask about.
- **Observability simplicity.** One `tracing` subscriber, one log
  stream, one metrics endpoint. A tower-layered axum server in one
  process is easier to audit than six meshed services.
- **Development loop.** `cargo build` produces the thing you run. No
  service orchestration required to iterate.

### Alternatives considered

- **Per-layer microservices.** Better process isolation, but the
  correctness argument for isolation is weak inside a single-tenant
  trust boundary, and the operational cost is large.
- **Binary per subcommand.** Considered and rejected. A single `slm-cli`
  with subcommands gives the same user experience without the
  release-engineering multiplication.

## Consequences

- **Positive.** One thing to sign, ship, install, operate. Simple
  observability. Fast development loop.
- **Negative.** A bug that panics the process kills every subsystem at
  once. Mitigation: forbid `.unwrap()` / `.expect()` outside tests
  (enforced by `clippy.toml`); use typed errors everywhere.
- **Follow-up.**
  - Binary size and startup time are tracked in CI as regression
    guards.
  - Any proposal for a second binary must cite this ADR and
    articulate why the appliance-fit argument no longer holds.

## References

- [`specs/SLM-STACK.md`](../../specs/SLM-STACK.md) §5 (flat architecture
  design, one binary).
- [`specs/SLM-STACK.md`](../../specs/SLM-STACK.md) §6 (os-totebox
  integration requirements).
