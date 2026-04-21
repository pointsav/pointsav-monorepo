# Roadmap

Phase-keyed roadmap for service-slm. Mirrors
[SLM-STACK §9](./specs/SLM-STACK.md) and
[YOYO-COMPUTE §10](./specs/YOYO-COMPUTE.md).

Each phase is independently valuable; we do not commit to Phase N+1 at
the start of Phase N.

## Phase 1 — Python trial (running elsewhere)

- **Where:** `service-content/` in the monorepo; separate from this
  repository.
- **Stack:** vLLM, SkyPilot, Dagster, FastAPI, Pydantic + instructor.
- **Goal:** validate the architecture end-to-end on the Woodfine corpus.
- **This repository:** not used. The scaffold here is Phase 2
  preparation.

## Phase 2 — Rust service-slm rewrite (this repository)

- **Where:** this repository.
- **Stack:** the Rust stack per [`specs/SLM-STACK.md`](./specs/SLM-STACK.md).
- **Goal:** the Rust `service-slm` passes the same test suite as the
  Phase 1 Python path, then replaces it.
- **Milestones:**
  - [ ] Workspace scaffolded, CI green on `main`. *(nearly complete)*
  - [ ] `slm-core` alpha: `ModuleId`, shared errors.
  - [ ] `slm-ledger` alpha: Event type + CSV writer + SQLite mirror.
  - [ ] `slm-doorman` alpha: sanitise/rehydrate with pass-through policy.
  - [ ] `slm-compute` alpha: Cloud Run driver, `BOOT_*` events.
  - [ ] `slm-inference-remote` alpha: HTTP client, retry policy, full
    ledger integration.
  - [ ] `slm-api` alpha: doorman HTTP surface.
  - [ ] `slm-cli` beta: all subcommands wired to real behaviour.
  - [ ] Parity test suite passing against Phase 1 reference answers.

## Phase 3 — os-totebox integration

- **Goal:** ship service-slm as a signed component of the os-totebox
  appliance.
- **Milestones:**
  - [ ] Cross-compile to `x86_64-unknown-linux-gnu` and
    `aarch64-unknown-linux-gnu`.
  - [ ] systemd unit file.
  - [ ] Sigstore-signed release artefacts.
  - [ ] SLSA provenance attestation.
  - [ ] Fit within Totebox Laptop-A's RAM envelope.

## Phase 4 — Optional open-source release

- **Goal:** make the repository public under AGPL-3.0-only per
  [ADR-0003](./docs/adr/0003-agpl3-for-own-code.md).
- **Milestones:**
  - [ ] REUSE compliance verified.
  - [ ] SBOM published with every release.
  - [ ] Third-party-notices generated and shipped.
  - [ ] Launch post.

No date commitment on Phase 4; the option stays open.
