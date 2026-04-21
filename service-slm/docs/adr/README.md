# Architecture Decision Records

This directory holds the ADRs for service-slm. Each ADR records a
decision that shapes the project beyond the lifespan of a single
release. ADRs are numbered sequentially and never renumbered. When an
ADR is superseded, a new ADR is added that explicitly references the
superseded one; the superseded ADR is *not* deleted.

Format: [MADR](https://adr.github.io/madr/), lightly adapted. See
[`template.md`](./template.md).

Governance: see [GOVERNANCE §2.2](../../GOVERNANCE.md).

## Index

| # | Title | Status | Date |
|---|-------|--------|------|
| [0001](./0001-rust-end-to-end.md) | Rust end-to-end for service-slm | accepted | 2026-04-20 |
| [0002](./0002-mistralrs-over-vllm-phase-2.md) | mistral.rs replaces vLLM in Phase 2 | accepted | 2026-04-20 |
| [0003](./0003-agpl3-for-own-code.md) | AGPL-3.0 for PointSav-authored code | accepted | 2026-04-20 |
| [0004](./0004-flat-binary-no-mesh.md) | Flat binary, no microservice mesh | accepted | 2026-04-20 |

## When to write a new ADR

Open a new ADR when you are about to make a decision that:

- Will shape the project across more than one release.
- Will cause new contributors to ask "why did we do it this way?" if
  undocumented.
- Overrides or deviates from a spec in [`../../specs/`](../../specs/).
- Adds an exception to the dependency licence policy.
- Introduces `unsafe` code.
- Changes the release process, the signing chain, or the project's
  licence.

Do **not** open an ADR for routine implementation choices. Those belong
in a PR description or an inline doc comment.
