# The Rust stack

service-slm is a Rust cargo workspace producing a single binary. The
authoritative stack specification is [SLM-STACK.md](../../specs/SLM-STACK.md);
this document summarises the decisions and their consequences.

## The "We Own It" rule

Every transitive dependency carries a permissive OSI licence: MIT,
Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Unicode-DFS, MPL-2.0
(file-level only), or Zlib. No copyleft. No BSL. This is enforced
automatically by `cargo-deny` via [`deny.toml`](../../deny.toml); see
[ADR-0003](../adr/0003-agpl3-for-own-code.md) for why PointSav's own
code is AGPL-3.0-only even though no dependency is.

## Per-layer choices

See [SLM-STACK §3](../../specs/SLM-STACK.md) for the full table. The
decisions worth highlighting here:

- **Inference: mistral.rs** (MIT) for Phase 2. Replaces vLLM. See
  [ADR-0002](../adr/0002-mistralrs-over-vllm-phase-2.md).
- **HTTP: axum + tower + tokio.** Standard Rust web stack.
- **Storage: sqlx (SQLite).** For the ledger mirror; the authoritative
  ledger is CSV.
- **Graph: LadybugDB** via its MIT-licensed Rust API.
- **Document processing: oxidize-pdf, docx-rust, calamine.** All pure
  Rust; the surprise find of 2025/2026 was that the PDF ecosystem
  finally has a production-ready Rust entry.
- **Orchestration: apalis.** Not Dagster. service-slm is job-centric,
  not asset-centric, and apalis fits that shape without a Python
  runtime.

## What is not Rust and why that is fine

Three external services are not Rust:

- **Mooncake Store** (C++, Apache-2.0). Network sidecar. We speak its
  wire protocol; we do not link it.
- **vLLM** (Python, Apache-2.0). Phase 1 only; mistral.rs replaces it
  in Phase 2.
- **SkyPilot** (Python, Apache-2.0). Optional and external.

This matches SLM-STACK §4: three external services behind stable
protocols, all permissively licensed, all forkable if necessary.

## The file layout

See [SLM-STACK §5.1](../../specs/SLM-STACK.md) for the cargo workspace
layout. The ten crates map 1:1 to the architectural concerns; see
[00-overview.md](./00-overview.md) for the mapping.
