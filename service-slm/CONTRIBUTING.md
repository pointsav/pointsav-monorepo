# Contributing to service-slm

Thank you for considering a contribution. This document tells you what the
house rules are, how to propose changes, and how to get your work merged.

If anything below is unclear, open a discussion on GitHub or email
peter@woodfinegroup.com. Asking early is cheaper than rework.

---

## Before you start

1. **Read [CLAUDE.md](./CLAUDE.md) or [AGENTS.md](./AGENTS.md).** They
   contain the hard invariants this project does not negotiate. Human
   contributors get the same rules as AI coding agents; the split exists only
   because different agent frameworks read different filenames.
2. **Check [STATUS.md](./STATUS.md) and [TASKS.md](./TASKS.md).** Pick up an
   open task or propose a new one on the issue tracker before writing code.
   This prevents two people accidentally starting the same work.
3. **Read the relevant spec in [`specs/`](./specs/).** `SLM-STACK.md` is the
   Rust stack; `YOYO-COMPUTE.md` is the yo-yo substrate. These are normative.

---

## Developer setup

You need:

- Rust toolchain per [`rust-toolchain.toml`](./rust-toolchain.toml) (installed
  automatically if you use `rustup`).
- `cargo-audit`, `cargo-deny`, `cargo-sbom` (`cargo install cargo-audit
  cargo-deny cargo-sbom`).
- `git` configured with your contributor identity (`git config user.email your@address`).

Full walkthrough in [`docs/dev-guide/getting-started.md`](./docs/dev-guide/getting-started.md).

---

## The contribution workflow

1. **Fork and branch.** Branch name is `scope/short-description`, where scope
   matches a crate name or a doc area. Examples: `slm-doorman/add-sanitiser`,
   `docs/user-guide-operating`.

2. **Write the code or docs.** Follow the voice and style rules below. Keep
   diffs small. A 200-line PR gets reviewed; a 2,000-line PR gets pushed to
   the bottom of someone's queue.

3. **Run local checks.** `./scripts/check-all.sh` must pass end-to-end. This
   runs `cargo fmt --check`, `cargo clippy --all-targets --all-features -D
   warnings`, `cargo test --workspace`, `cargo audit`, and `cargo deny check`.
   CI runs the same checks; the script is here so you catch failures locally.

4. **Update state files.** If you touched a crate, update `STATUS.md`. If you
   completed a task, mark it done in `TASKS.md` and add any follow-ups you
   spotted. If you added a new ADR, bump the number and update the index.

5. **Commit.** Use Conventional Commits with a crate-scoped type:

   ```
   git commit -m "slm-doorman: add outbound sanitiser skeleton"
   ```

   **CLA required.** Before a pull request can be merged, every contributor
   must have a signed Contributor License Agreement on file. CLA Assistant
   enforces this automatically — on first contribution the bot will
   prompt you to sign the individual or corporate CLA through a GitHub
   comment. Signatures are retained across subsequent contributions. This
   is consistent with `factory-release-engineering/` policy for AGPLv3
   repos.

6. **Open a PR against `main`.** Fill in the PR template. Link the issue or
   task from `TASKS.md`. Assign reviewers per `CODEOWNERS`.

7. **Respond to review.** Reviewer comments are not optional. Push follow-up
   commits rather than force-pushing; a clean squash happens at merge time.

---

## Commit message convention

We use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
with crate-scoped types. Format:

```
<scope>: <short imperative description>

<optional longer body>

<optional footer: BREAKING CHANGE, Refs, Closes, etc.>
```

Valid scopes:

- A crate name: `slm-core`, `slm-doorman`, `slm-ledger`, `slm-compute`,
  `slm-memory-kv`, `slm-memory-adapters`, `slm-inference-local`,
  `slm-inference-remote`, `slm-api`, `slm-cli`.
- An infrastructure area: `xtask`, `ci`, `docs`, `adr`, `deps`, `release`.

Examples of acceptable commit subjects:

- `slm-doorman: add outbound sanitiser skeleton`
- `docs: add getting-started walkthrough for Linux`
- `deps: bump tokio to 1.41 (fixes cargo audit warning RUSTSEC-2025-…)`
- `adr: add 0005 on Mooncake master hosting`

---

## Coding standards

### Rust

- **`#![forbid(unsafe_code)]`** at the top of every `lib.rs`. If a specific
  feature genuinely needs unsafe, open an ADR first, then add a
  narrowly-scoped `#[allow(unsafe_code)]` with a comment explaining the
  invariants the block upholds.
- **No `.unwrap()` or `.expect()` outside tests.** Use `?` with a typed error
  from the crate's `error.rs`. Use `thiserror` for error enums, `anyhow` only
  in `slm-cli` where end-user-facing error messages are fine.
- **`tracing` for all observability.** `log` macros are banned. Every public
  function that does I/O carries a `#[tracing::instrument]` attribute with
  `skip(large_args)` for payloads.
- **Public API surface gets `#[deny(missing_docs)]`.** Yes, even on
  internal-sounding names. Documentation is a first-class deliverable.
- **Feature flags are additive.** Never remove behaviour behind a default
  feature flag without a breaking-change note.

### Formatting and linting

- `cargo fmt --all` before every commit. Our `rustfmt.toml` is intentionally
  minimal; we follow the default community style with small, justified
  deviations.
- `cargo clippy --all-targets --all-features -- -D warnings`. We treat all
  clippy warnings as errors. If a specific lint needs suppression, use the
  narrowest possible scope (`#[allow(lint_name)]` on the specific item) and
  leave a comment explaining why.

### Documentation

- Crate-level doc comment (`//!`) at the top of every `lib.rs`. Minimum
  content: one paragraph on what the crate does, one paragraph on its
  boundaries (what it is *not* responsible for), a link to the relevant
  section of `specs/` or `docs/architecture/`.
- Public items get `///` rustdoc with at least: one-line summary, one
  paragraph of context, one example (unless the item is genuinely trivial).
- Prose documentation (anything in `docs/`) follows the voice guide in
  [CLAUDE.md §Voice and prose style](./CLAUDE.md). Short version: numbered
  sections, full sentences, explicit rationale, tables for dense
  comparisons only.

---

## Licence discipline

**SPDX header at the top of every file you author or edit:**

```rust
// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.
```

For TOML, YAML, shell: use the comment character for the file type. The
header is required by the REUSE Specification, which CI verifies.

**Never add a dependency with a non-allow-listed licence.** The allow-list
lives in [`deny.toml`](./deny.toml) and matches SLM-STACK §7. If you
genuinely need a crate under a forbidden licence, open an ADR proposing a
narrowly-scoped exception — but in almost every case there is a permissive
alternative, so look hard before you propose an exception.

---

## What happens after you submit

1. CI runs fmt, clippy, test, audit, deny, SBOM, REUSE check. If any fail,
   the PR is blocked.
2. A `CODEOWNERS`-assigned reviewer reads the diff. Expect comments.
3. Once the reviewer is satisfied, a maintainer approves and merges. We use
   squash-merge; your branch history becomes one commit on `main` with a
   descriptive message.
4. Your change lands in the next release, which follows [Semantic
   Versioning](https://semver.org/) and appears in [CHANGELOG.md](./CHANGELOG.md).

---

## Reporting security vulnerabilities

Do **not** open a public issue. See [SECURITY.md](./SECURITY.md) for the
disclosure process.

---

## Questions?

- Open a GitHub Discussion for design questions.
- Open a GitHub Issue for bugs and concrete feature requests.
- Email peter@woodfinegroup.com for anything else.

Thank you for contributing. Your work helps make service-slm the kind of
service that institutions can trust.
