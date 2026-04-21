# Licence hygiene

The single-sentence rule: **every `.rs`, `.toml`, `.yaml`, `.sh`, and
`.py` file carries an SPDX header; every dependency is on the allow-list
in [`deny.toml`](../../deny.toml).**

## SPDX headers

Every file you author or modify:

```rust
// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.
```

For TOML, YAML, shell, Python: use the comment character for the file
format. For Markdown, use an HTML comment at the top of the file.

CI runs `reuse lint` on every PR; missing headers fail the build.

## The dependency allow-list

Defined in [`deny.toml`](../../deny.toml). The allowed licences are:

- MIT, MIT-0
- Apache-2.0 (with or without LLVM exception)
- BSD-2-Clause, BSD-3-Clause
- ISC
- Unicode-DFS-2016, Unicode-3.0
- MPL-2.0 (file-level copyleft only; acceptable because it does not
  propagate to our binary)
- Zlib
- CC0-1.0, 0BSD, Unlicense (public-domain-equivalent; use sparingly)

## The denied list

These licences are blocked. Any crate under them in the transitive
graph fails the build:

- GPL-2.0, GPL-3.0 (strong copyleft — taints our binary)
- LGPL-2.1, LGPL-3.0 (weak copyleft; dynamic linking exceptions do
  not cleanly apply in Rust)
- AGPL-3.0 (network copyleft)
- BSL (time-delayed OSS; uncertain commercial terms)
- SSPL, Elastic License (not OSI-approved)
- CC-BY-NC (non-commercial)

## Why AGPL-3.0-only for PointSav code but not for dependencies?

PointSav's own code is AGPL-3.0-only per
[ADR-0003](../adr/0003-agpl3-for-own-code.md). AGPL-3.0-only is **not**
allowed for dependencies because:

1. Dependency reciprocity compounds into operational burden at
   acquisition or audit time.
2. Our own copyleft posture is a deliberate choice; a dependency's
   copyleft is an inherited obligation.

In other words: we publish copyleft; we consume permissive. This is a
common, deliberate pattern.

## Adding a new dependency

1. Confirm it is on crates.io (not git). Git deps require an ADR.
2. Check its licence field on crates.io and in the repository. If the
   licence is not on the allow-list, stop — do not propose the dep.
3. Add the dep to the root `[workspace.dependencies]` with a pinned
   minimum version, then reference it as `.workspace = true` in the
   crate that uses it.
4. Run `cargo deny check` locally. If it fails, fix the dependency
   tree, not the policy.
5. Update [`CHANGELOG.md`](../../CHANGELOG.md) under `[Unreleased]`.
6. Open a PR. CI will verify.

## Exceptions

Sometimes a dep you really need carries an oddball licence (a `WITH`
clause, a dual licence, a region-specific variant). The options in
priority order are:

1. **Find an alternative.** Almost always there is one.
2. **Propose a `[[licenses.clarify]]` block in `deny.toml`** if the
   licence is permissive but the metadata is imprecise.
3. **Write an ADR** proposing a narrowly-scoped exception. This is the
   last resort.

Never silently lower the `confidence-threshold` in `deny.toml`. The
threshold is tight on purpose.
