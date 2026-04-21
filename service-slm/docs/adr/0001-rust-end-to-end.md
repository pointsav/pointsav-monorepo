# ADR-0001: Rust end-to-end for service-slm

- **Status:** accepted
- **Date:** 2026-04-20
- **Deciders:** Peter M. Woodfine, Jennifer Woodfine
- **Supersedes:** none (codifies [`specs/SLM-STACK.md`](../../specs/SLM-STACK.md))

## Context

service-slm sits at the doorman boundary and on the yo-yo compute node. It
is the single point where PointSav-trusted data meets external compute,
which makes it the component with the highest correctness, security, and
predictability bar in the platform. It is also intended to become a
component of os-totebox, the eventual appliance product, where it will run
on hosts with constrained memory (Totebox Laptop-A has ~550 MB headroom
after core services) and where long uptimes without memory leaks matter.

Phase 1 of the PointSav trial runs this logic in Python (vLLM, SkyPilot,
Dagster, FastAPI). The Phase 1 stack was the right choice for rapid
validation of the architecture but is the wrong long-term shape for an
appliance component: Python + PyTorch + vLLM is a 12 GB container with a
GIL-capped runtime that cannot fit Laptop-A's envelope.

## Decision

service-slm is rewritten in Rust as a flat cargo workspace producing a
single signed binary (`slm-cli`). All workspace code is authored in Rust.
Direct dependencies are Rust crates. Two C++ dependencies — the LLM
inference engine (mistral.rs's CUDA kernels) and the LadybugDB graph DB
— remain outside the Rust ecosystem but are accessed behind stable
protocols (local library binding for mistral.rs; wire protocol for
LadybugDB).

This is the "L2 + permissive" position articulated in SLM-STACK §2:
every line in the dependency graph is under a permissive OSI licence,
even though not every line is Rust source.

## Rationale

Rust earns its place against three concrete requirements:

1. **Fit on low-RAM hosts.** Totebox Laptop-A cannot host a Python stack
   and the inference runtime simultaneously. Rust with a quantised
   `mistral.rs` backend fits.
2. **Predictable appliance behaviour.** Static binary, no interpreter
   warm-up, no GC pauses, small attack surface, and memory-safety by
   construction. These are table stakes for a component that will be
   audited by institutional customers.
3. **Signed releases that mean something.** A Sigstore-signed Rust
   binary is a clean chain of custody. A Sigstore-signed container
   image with a Python venv inside is not the same artefact.

### Alternatives considered

- **Stay on Python.** Lower engineering cost in the short term. Fails
  the low-RAM and predictability requirements. Deferred Rust work
  would be more expensive than the Rust rewrite.
- **Go.** Good appliance fit, but ecosystem for GPU inference and
  graph DB clients is thinner, and the PointSav team's existing Rust
  familiarity is an asset.
- **C++ directly.** Lowest level of control. Rejected on
  memory-safety, developer-velocity, and licence-hygiene grounds.

## Consequences

- **Positive.** Predictable appliance component. Clean licence story.
  One binary to sign, install, and operate. A dependency graph that
  is 100% permissive.
- **Negative.** Hiring for "Rust + AI infrastructure" is narrower than
  hiring for Python + ML. Mitigation: the os-totebox product needs
  appliance-engineering discipline anyway, which maps naturally to
  Rust talent.
- **Follow-up.**
  - [ADR-0002](./0002-mistralrs-over-vllm-phase-2.md) selects the
    specific inference runtime.
  - [ADR-0004](./0004-flat-binary-no-mesh.md) records the "one binary,
    no microservice mesh" consequence.

## References

- [`specs/SLM-STACK.md`](../../specs/SLM-STACK.md) §§1–3 and §6 (the
  "We Own It" decision, the three levels of Rust-ness, the os-totebox
  fit argument).
- [`specs/YOYO-COMPUTE.md`](../../specs/YOYO-COMPUTE.md) §1 (the
  three-ring memory model the Rust implementation must support).
