# service-slm

**A flat, 100% Rust substrate for the PointSav doorman and yo-yo compute boundary.**

`service-slm` is the single point in the PointSav platform where local insufficiency
meets external compute. It owns the sanitise-send-receive-rehydrate doorman
protocol, the GCP yo-yo node lifecycle, the three-ring memory model
(bootstrap / KV cache / long-term adapter stack), and the SOC3-grade audit
ledger that ties every inference call back to its source material.

One cargo workspace. Ten crates. One signed binary (`slm-cli`). No microservice
mesh. No Python in the hot path. No copyleft in the dependency graph.

---

## About this repository

`service-slm` is developed by **Woodfine Capital Projects Inc.** as part of the
**PointSav Digital Systems** product line. PointSav is the product brand;
Woodfine is the copyright holder and commercial entity that authors this code.
When PointSav Digital Systems incorporates as a separate legal entity, copyright
assignment will transfer per a future ADR.

- **Copyright holder:** Woodfine Capital Projects Inc.
- **Licence:** [AGPL-3.0-only](./LICENSE) (see [ADR-0003](./docs/adr/0003-agpl3-for-own-code.md))
- **Contact:** Peter M. Woodfine, Executive Officer — peter@woodfinegroup.com
- **Security disclosure:** [SECURITY.md](./SECURITY.md)
- **Governance:** [GOVERNANCE.md](./GOVERNANCE.md) / [MAINTAINERS.md](./MAINTAINERS.md)

---

## Status

| Phase | Description | State | Notes |
|-------|-------------|-------|-------|
| 1 | Python trial on GCP (vLLM, SkyPilot, Dagster) | **active elsewhere** | Runs in `service-content`; this repo is the forward path |
| 2 | Rust `service-slm` rewrite (this repo) | **scaffolding** | Ten crates present; no business logic yet |
| 3 | os-totebox integration (appliance target) | not started | Cross-compile, systemd unit, Sigstore release |
| 4 | Optional external contribution / OSS release | not started | AGPL-3.0-only already; open the repo when ready |

Machine-readable crate status lives in [STATUS.md](./STATUS.md). The ordered
work queue is [TASKS.md](./TASKS.md). Phase-keyed roadmap is
[ROADMAP.md](./ROADMAP.md).

---

## Quickstart (development)

Assumes a reasonably current Linux or macOS host with Rust installed via
`rustup`. The pinned toolchain is read automatically from
[`rust-toolchain.toml`](./rust-toolchain.toml).

```bash
# Clone into the monorepo, or stand-alone
git clone git@github.com:woodfinegroup/pointsav-monorepo.git
cd pointsav-monorepo/service-slm

# Verify toolchain + fetch deps
cargo fetch

# Run the full local check suite (fmt, clippy, test, audit, deny)
./scripts/check-all.sh

# Build the binary
cargo build --release --bin slm-cli

# Run the binary (Phase 2 scaffolding — no real behaviour yet)
./target/release/slm-cli --help
```

Full developer setup including sccache, deny-policy tuning, and local
Mooncake-sidecar instructions is in [docs/dev-guide/](./docs/dev-guide/).

---

## What's in this repository

```
service-slm/
├── crates/                  Ten workspace crates (see below)
├── xtask/                   cargo xtask: build, release, sign automation
├── docs/
│   ├── architecture/        Narrative architecture (derived from specs/)
│   ├── adr/                 Architecture Decision Records (MADR format)
│   ├── rfcs/                Forward-looking proposals
│   ├── dev-guide/           For contributors
│   ├── user-guide/          For operators, community, customers
│   └── reference/           CLI flags, HTTP schemas, ledger events
├── specs/                   Normative source documents (SLM-STACK, YOYO-COMPUTE)
├── .claude/                 Claude Code project config (commands, agents)
├── .github/                 CI workflows, issue/PR templates
├── scripts/                 Convenience wrappers over xtask
└── examples/                Runnable examples referenced from docs
```

### The ten crates

| Crate | Role | Ring |
|-------|------|------|
| `slm-core` | Shared types, errors, moduleId discipline | — |
| `slm-doorman` | Sanitise / send / receive / rehydrate protocol | — |
| `slm-ledger` | Append-only CSV + SQLite audit trail | — |
| `slm-compute` | Cloud Run driver, container management | Ring 1 |
| `slm-memory-kv` | LMCache + Mooncake wire protocol client | Ring 2 |
| `slm-memory-adapters` | LoRA adapter registry and loader | Ring 3b |
| `slm-inference-local` | `mistral.rs`-backed local inference | — |
| `slm-inference-remote` | GCP yo-yo driver | — |
| `slm-api` | axum HTTP server (inbound endpoints) | — |
| `slm-cli` | Operator CLI — the binary entry point | — |

The three-ring memory model is specified in
[YOYO-COMPUTE.md §1](./specs/YOYO-COMPUTE.md) and summarised in
[docs/architecture/04-three-ring-memory.md](./docs/architecture/04-three-ring-memory.md).

---

## Authoritative specifications

The two documents in [`specs/`](./specs/) are the normative source of truth for
this service. Narrative documentation in `docs/` explains, cross-references, and
operationalises them but does not override them.

- [`specs/SLM-STACK.md`](./specs/SLM-STACK.md) — the Rust stack, license policy, crate layout, migration roadmap
- [`specs/YOYO-COMPUTE.md`](./specs/YOYO-COMPUTE.md) — the yo-yo substrate, three-ring memory, audit ledger

Where a decision in `specs/` conflicts with a later ADR, the ADR governs and
must explicitly reference the superseded section (see
[ADR-0003](./docs/adr/0003-agpl3-for-own-code.md) for a worked example —
it supersedes SLM-STACK §8 on the licence question).

---

## For contributors

Start with [CONTRIBUTING.md](./CONTRIBUTING.md), then
[docs/dev-guide/getting-started.md](./docs/dev-guide/getting-started.md).

House rules in one line: **CLA signed, SPDX header on every
file, cargo-deny passes, no copyleft dependencies, no unsafe Rust without an
ADR.**

---

## For Claude Code and other AI coding agents

This repository is set up to be driven by Claude Code over many sessions. The
primary memory file is [CLAUDE.md](./CLAUDE.md); per-crate memory lives at
`crates/*/CLAUDE.md`. The slash-commands and subagents in
[.claude/](./.claude/) are the repeatable operations.

If you are an AI coding agent opening this repository for the first time, read
in this order:

1. [CLAUDE.md](./CLAUDE.md) — project-wide invariants and conventions
2. [STATUS.md](./STATUS.md) — what's done, what's in progress, what's open
3. [TASKS.md](./TASKS.md) — the ordered work queue
4. The `CLAUDE.md` inside whichever crate you're working on
5. The relevant spec in [`specs/`](./specs/)

See [docs/dev-guide/claude-code-workflow.md](./docs/dev-guide/claude-code-workflow.md)
for the full workflow.

---

## Licence

Copyright © 2026 Woodfine Capital Projects Inc.

This project is licensed under the GNU Affero General Public License, version 3
(AGPL-3.0-only). See [LICENSE](./LICENSE) for the full licence text and
[NOTICE](./NOTICE) for third-party attributions. The rationale for choosing
AGPL-3.0-only over the Apache-2.0 originally suggested in SLM-STACK §8 is recorded
in [ADR-0003](./docs/adr/0003-agpl3-for-own-code.md).

Dependencies are restricted to permissive OSI licences (MIT, Apache-2.0,
BSD-2/3, ISC, Unicode-DFS, MPL-2.0 file-level, Zlib) and this is enforced in CI
by `cargo-deny` per [`deny.toml`](./deny.toml).
