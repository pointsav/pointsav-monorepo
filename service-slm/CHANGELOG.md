# Changelog

All notable changes to service-slm are recorded here. Format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and semantic
versioning per [SemVer 2.0.0](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial workspace scaffold with ten crates per SLM-STACK §5.1.
- Architecture Decision Records 0001–0004.
- Governance files (GOVERNANCE, MAINTAINERS, CODEOWNERS, CODE_OF_CONDUCT,
  SECURITY, CONTRIBUTING).
- CI workflows for fmt, clippy, test, audit, deny, SBOM.
- Claude Code memory hierarchy (root CLAUDE.md, per-crate CLAUDE.md,
  `.claude/` commands and agents).
- Developer and user documentation trees under `docs/`.

### Security

- Dependency licence allow-list enforced via `cargo-deny` per
  [`deny.toml`](./deny.toml).
- AGPL-3.0-only for all PointSav-authored files, SPDX headers required.

---

## Versioning policy

Until we ship a `1.0.0` release, every minor bump (`0.x.0`) may include
breaking changes. Once we hit `1.0.0`, SemVer discipline applies
strictly: breaking changes require a major bump.

The `CHANGELOG.md` is updated in every PR that merits a user-visible
entry. CI fails if a user-visible PR merges without a changelog update.
