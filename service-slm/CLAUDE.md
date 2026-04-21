# CLAUDE.md — service-slm

You are working on **service-slm**, a Rust cargo workspace inside the
PointSav monorepo. This file is your project memory. Read it at the start of
every session. It is small on purpose; crate-level detail lives in
`crates/*/CLAUDE.md`.

---

## What this service is, in one paragraph

`service-slm` is the single boundary where PointSav products cross from local
compute to external compute. It owns the **doorman protocol** (sanitise
outbound, send, await, receive, rehydrate), the **yo-yo compute substrate**
(spin up a GCP GPU node, run inference, tear down), and the **three-ring memory
model** (bootstrap / KV cache / long-term adapter stack). It ships as **one
signed Rust binary** that runs as a systemd unit or an os-totebox appliance
service component. Read [`specs/SLM-STACK.md`](./specs/SLM-STACK.md) and
[`specs/YOYO-COMPUTE.md`](./specs/YOYO-COMPUTE.md) before changing architecture.

---

## Hard invariants (do not violate without an ADR)

1. **Licence: AGPL-3.0-only for PointSav code.** Every `.rs`, `.toml`, `.yaml`,
   `.sh` file authored here carries `SPDX-License-Identifier: AGPL-3.0-only` at the
   top. Copyright line reads `Copyright (c) 2026 Woodfine Capital Projects Inc.`
   No exceptions. See [ADR-0003](./docs/adr/0003-agpl3-for-own-code.md).
2. **Dependency licences: permissive only.** MIT, Apache-2.0, BSD-2/3, ISC,
   Unicode-DFS, MPL-2.0 (file-level only), Zlib. Nothing else. Enforced by
   `cargo-deny` per [`deny.toml`](./deny.toml). If `cargo deny check` fails,
   fix the dependency tree, never the policy.
3. **No unsafe Rust without an ADR.** `#![forbid(unsafe_code)]` is in every
   crate's `lib.rs`. If a future integration genuinely requires `unsafe`,
   write an ADR, scope the block, document the invariants it upholds.
4. **Contributor License Agreement required.** Every contributor must have
   a signed CLA on file before any pull request is merged. CLA Assistant
   enforces the check in CI per `factory-release-engineering/` policy for
   AGPLv3 repos.
5. **One binary.** The workspace produces exactly one binary: `slm-cli`. All
   other crates are libraries. Do not add a second binary without an ADR.
6. **No network calls in unit tests.** Integration tests that need a network
   go in `tests/` and gate on a feature flag or an environment variable.
7. **Every inference event writes a ledger row.** This is a SOC3 processing
   integrity guarantee and the commercial differentiator. See
   [YOYO-COMPUTE.md §5](./specs/YOYO-COMPUTE.md).
8. **`moduleId` is threaded through every call.** Bootstrap, KV cache, graph,
   adapters, ledger — all five layers use it. See [YOYO-COMPUTE.md §6](./specs/YOYO-COMPUTE.md).

---

## The phase we are in

**Phase 2 scaffolding.** The repository structure is complete. The ten crates
compile as empty stubs. CI is green. No business logic has been written yet.

Phase 1 (Python trial) runs elsewhere in the monorepo (see
`service-content/` and the phase-1 vLLM/SkyPilot configuration). This
repository does not touch the Phase 1 code path. The goal of Phase 2 is to
rewrite the service-slm functionality in Rust such that the same test suite
passes, then switch over.

Your work almost certainly belongs in **Phase 2**. If you think it belongs
anywhere else, stop and ask the human.

---

## Model selection

Every task in [`TASKS.md`](./TASKS.md) carries a `Model:` field suggesting
which Claude Code model should run it. The convention:

- **`opusplan`** — default for most implementation work. Opus does the
  planning; Sonnet executes the code generation automatically. Best balance
  of reasoning and cost.
- **`opus`** — reserved for tasks where execution itself needs deep
  reasoning, not just the plan (cross-crate refactors, tricky concurrency,
  ADRs that need architectural judgement throughout).
- **`sonnet`** — straight implementation where the plan is already clear
  and code-writing is the main work. Faster and cheaper than `opusplan`
  when no real planning is needed.
- **`haiku`** — mechanical tasks: dependency bumps, CHANGELOG entries, doc
  typo fixes, lockfile commits, boilerplate scaffolding.
- **`human`** — not a Claude Code task. Content a specific person must
  author (organisational rationale, legal language, maintainer
  appointments). Do not attempt these.

At session start, the human selects the model per the task's tag. Do not
override the tag silently. If the tag says `haiku` but the task is turning
out to require reasoning beyond Haiku's capacity, **stop and tell the
human** — this is an escalation, not a signal to keep pushing. The
operator will either switch models or re-tag the task.

Similarly, if a task tagged `sonnet` or higher turns out to be trivially
mechanical, you may note that in the session wrap-up so the tag can be
demoted for future similar work. Cost discipline improves over time through
this feedback loop.

The full guidance, including rationale and operator instructions, is in
the "Model guidance" section at the top of [`TASKS.md`](./TASKS.md).

---

## The session protocol

Every session follows this shape:

1. **Orient.** Read this file. Then read [`STATUS.md`](./STATUS.md) and
   [`TASKS.md`](./TASKS.md) to see what has changed since last session.
2. **Pick.** Select the highest-priority open task from `TASKS.md` that you
   have the context to complete. Read its `Model:` field. If the active
   Claude Code model does not match, pause and tell the human before
   proceeding — they will either switch models or choose a different task.
   If nothing fits, ask the human.
3. **Work in the crate.** Read the `CLAUDE.md` inside the crate you are
   touching. Follow its conventions.
4. **Verify.** Run `./scripts/check-all.sh`. It must pass before you claim
   anything is done.
5. **Update state.** Update `STATUS.md` (crate status) and `TASKS.md` (mark
   the task done, add follow-ups, note any model-tag observations). This is
   part of the definition of done.
6. **Commit.** `git commit -m "scope: short description"`. Conventional
   Commits style, lowercase scope matching the crate name.

If you skip step 5, the next session has no record of what you did and
drift begins. Do not skip step 5.

---

## Where things live

| If you need… | Look here |
|---|---|
| Project overview | `README.md` |
| Architecture decisions | `docs/adr/` |
| Narrative architecture | `docs/architecture/` |
| Developer setup | `docs/dev-guide/getting-started.md` |
| The Rust stack spec | `specs/SLM-STACK.md` |
| The yo-yo substrate spec | `specs/YOYO-COMPUTE.md` |
| Work queue | `TASKS.md` |
| Model-selection guidance | `TASKS.md` (top-of-file section) |
| Crate status | `STATUS.md` |
| Slash commands | `.claude/commands/` |
| Specialised subagents | `.claude/agents/` |
| CI workflows | `.github/workflows/` |
| Licence policy | `deny.toml` |
| Workspace dependencies | `Cargo.toml` (root) |

---

## Voice and prose style (for documentation you author)

The documentation in this repository imitates the voice of `specs/SLM-STACK.md`
and `specs/YOYO-COMPUTE.md` deliberately. When you write documentation:

- **Numbered sections, full sentences.** Not bullet-point soup.
- **Tables for comparisons, prose for reasoning.** Never a table of links or
  a table of vague adjectives. A table earns its place by making a dense
  comparison scannable.
- **Explicit rationale.** If a decision is not obvious, write the rationale
  next to the decision. The pattern is: *Decision. Reasoning. Consequence.*
- **Institutional register.** This is what PointSav's clients and potential
  auditors read. Write as if you were producing a memo for an audit
  committee, not a tweet thread.

If your draft reads like generic open-source-project README output, rewrite
it.

---

## What not to touch

- **`specs/`** is read-only in this repository. The two files are verbatim
  copies of the master specs. If a spec needs to change, change it at the
  monorepo-root source and re-copy. Do not edit in place.
- **`LICENSE`** is the canonical AGPL-3.0 text. Do not edit. If a licence
  change is ever contemplated, write an ADR first.
- **`CODEOWNERS`** is governed by `GOVERNANCE.md`. Do not edit without a
  matching governance change.

---

## Escalation

If you hit any of the following, stop and ask the human before proceeding:

- A task that requires adding a dependency with a non-allow-listed licence.
- A task that requires an `unsafe` block.
- A task that requires adding a second binary to the workspace.
- A test that requires a network connection to anything other than a local
  sidecar (Mooncake master, LMCache mock, a local `httpbin`).
- A change to anything in `specs/` or `LICENSE`.
- A disagreement between `specs/SLM-STACK.md` and an ADR that is not yet
  resolved — flag it; do not interpret.
- The task's `Model:` tag is lower-capability than the task actually
  requires (for example, a `haiku`-tagged task is turning out to need
  real reasoning).

Short, explicit escalation beats long, confident drift.
