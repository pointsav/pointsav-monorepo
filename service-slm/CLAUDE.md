# CLAUDE.md — service-slm

> **State:** Active  —  **Last updated:** 2026-05-04
> **Registry row:** `pointsav-monorepo/.agent/rules/project-registry.md`
>
> When state changes, update this header AND the registry row in the
> same commit. Drift between the two is a documentation defect.

---

## What this project is

service-slm is the single secure boundary between the isolated
Totebox Archive and any external Large Language Model. It implements
the Doorman Protocol: sanitise outbound payloads, route them to
external compute (local if the host has the resources, otherwise the
yo-yo substrate on GCP), receive structured deltas, and rehydrate
them back into the ledger. service-slm does not generate text; it
gates compute. Read-side sibling for long-term semantic memory is
service-content — see `ARCHITECTURE.md` Ring 3a.

## Current state

**Active.** Doorman in production on workspace VM (`local-doorman.service`).
Tier A (llama-server, OLMo 3 7B Q4) live and verified. **157/157 tests.**

As of 2026-05-04 (commit `764636b`):

- **Tier A** — live; GBNF/JsonSchema grammar; Lark rejected pre-flight.
- **Tier B** — code-complete; multi-Yo-Yo `HashMap<String, YoYoTierClient>`;
  named nodes `"default"`, `"trainer"` (L4/OLMo 3 32B-Think), `"graph"`
  (H100/Llama 3.3 70B); `yoyo_label` on `ComputeRequest` selects target;
  deploy gated on D4 image-build pipeline.
- **Tier C** — code-complete; compile-time `ExternalAllowlist`; mock-only
  tests per operator cost guardrail.
- **Grammar substrate (PS.3)** — complete; Doorman-side Lark validation
  via `llguidance`; all three variants routed per tier.
- **Audit substrate (PS.4)** — complete; `POST /v1/audit/proxy` +
  `POST /v1/audit/capture`; contract v0.2.0; four `entry_type`
  discriminators.
- **Apprenticeship (AS-2..AS-7 + §7C brief queue)** — code-complete;
  **disabled on workspace VM** (`SLM_APPRENTICESHIP_ENABLED` unset;
  operator-presence re-enable pending).
- **Graph context (Ring 2→3)** — `GraphContextClient` queries
  `service-content` before every inference; injects `[ENTITY CONTEXT]`
  system message; non-fatal if service-content is down.
- **Mesh discovery** — `MeshRegistry`/`DiscoveryProvider`/`DynamicRegistry`
  scaffolded; `route_async()` is a Phase 1 stub (logs selected node,
  falls through to `route()`); no concrete `DiscoveryProvider` yet.
- **Phase 3** (training threshold detection) — not started; gated on
  operator go-ahead and D4 deployment.

## Build and test

```
cargo check --workspace                # seconds incremental
cargo test  --workspace                # 157 tests (14 slm-core + 92 slm-doorman + 5 audit + 4 queue + 42 http)
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt   --all -- --check
```

End-to-end against a real Tier A endpoint:

```
SLM_LOCAL_ENDPOINT=http://127.0.0.1:8080 \
SLM_BIND_ADDR=127.0.0.1:9080 \
    cargo run -p slm-doorman-server
```

`SLM_YOYO_ENDPOINT` is intentionally unset by default — community-
tier mode. Setting it activates Tier B (currently a stub returning
`NotImplemented` until B2 lands). The legacy `cognitive-forge/`
subcrate still builds in isolation:

```
cargo build --manifest-path cognitive-forge/Cargo.toml
```

## File layout

```
service-slm/
├── README.md                  English README
├── README.es.md               Spanish README
├── CLAUDE.md                  this file
├── NEXT.md                    open items
├── ARCHITECTURE.md            three-ring model, stack, target file tree
├── DEVELOPMENT.md             build/CI policy, migration phases, blockers
├── Cargo.toml                 workspace manifest (B1, 2026-04-25)
├── deny.toml                  licence policy per DEVELOPMENT.md §2.1
├── rust-toolchain.toml        stable channel pin
├── .gitignore                 target/, swap files
├── crates/
│   ├── slm-core/              shared types, moduleId discipline
│   ├── slm-doorman/           three-tier router + audit ledger (lib)
│   └── slm-doorman-server/    axum HTTP entry point (bin)
├── cognitive-bridge.sh        placeholder — defect, queued for scripts/
├── cognitive-forge/           legacy subcrate — workspace `exclude`
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/main.rs            tokio + reqwest + serde_json
└── transient-queues/          runtime payload bleed — defect, triage pending
    └── TX-*_skeleton.txt      (8 files)
```

Still missing relative to `ARCHITECTURE.md` §File tree:
`memory/{kv,adapters}/`, `compute/`, `outbound/`, `inbound/`,
`log/`, `ledger/`, plus the remaining crates (`slm-ledger`,
`slm-compute`, `slm-memory-kv`, `slm-memory-adapters`,
`slm-inference-local`, `slm-inference-remote`, `slm-api`,
`slm-cli`). Phase-1 additions land as the Phase B task list
progresses (B2 fills `slm-doorman/src/tier/yoyo.rs`; B4 fills
`tier/external.rs`).

## Hard constraints — do not violate

- **Do not generate text in this service.** service-slm is an API
  gateway. Generation happens externally (Claude API in Phase 1;
  mistral.rs on the yo-yo node in Phase 2). If code here starts
  producing text directly, it has exceeded its remit.
- **Do not route structured data through the external LLM.**
  SYS-ADR-07 hard rule (workspace `CLAUDE.md` §6). Prose payloads
  cross the boundary outbound; structured facts stay in-Totebox.
- **Do not retire "Cognitive Forge" naming silently.** The rename
  is user-tracked in the monorepo `NEXT.md` rename series. Wait
  for the paired decision with `tool-cognitive-forge`.
- **Do not introduce dependencies outside the permissive-licence
  allow-list** once the Rust workspace is scaffolded.
  `DEVELOPMENT.md` §License policy names the `deny.toml`
  enforcement mechanism.

## Dependencies on other projects

- **service-content** (read side): service-slm writes extraction
  output to `service-content/knowledge-graph/`, which
  `service-content/content-compiler` consumes. Wire-format
  reconciliation pending.
- **service-email, service-people, service-content** as the three
  sovereign ledgers the Doorman compiles outbound context from
  (per README §I).
- **system-slm**: the local SLM engine. The "MISSING CONNECTION
  PHYSICS" comment in `cognitive-bridge.sh` is the documented gap
  blocking the placeholder bridge.

---

## Inherited rules — do not duplicate, do not silently override

- **Repo-level:** `pointsav-monorepo/CLAUDE.md` does not yet exist
  (tracked in workspace `NEXT.md` as a documentation-debt item).
  In its absence, the monorepo's `.agent/rules/` carries local
  conventions: `repo-layout.md`, `project-registry.md`,
  `cleanup-log.md`, `handoffs-outbound.md`.
- **Workspace-level:** `~/Foundry/CLAUDE.md` — identity store,
  commit flow (`tool-commit-as-next.sh`), cluster session pattern
  (§9), ADR hard rules (§6).

If a rule at this level conflicts with an inherited rule, **stop and
surface the conflict** — do not silently override.
