# Release process

How we cut a release. Phase 2 scaffolding note: this is the intended
procedure. It has not yet been exercised because no releases have been
cut. Review this document carefully before the first tag push.

## Versioning

SemVer 2.0.0. Pre-1.0.0 (where we are now) uses minor bumps for
anything user-visible; post-1.0.0, SemVer discipline is strict.

Version numbers:

- `0.x.y` for pre-1.0.0.
- `1.0.0` when the Phase 2 feature set is complete and tested.
- `1.x.y` for SemVer-compatible changes thereafter.
- `2.0.0` only for intentional breaking changes.

## Tag format

Tags are prefixed `service-slm-v` to disambiguate from other services
in the monorepo:

```bash
git tag -s service-slm-v0.2.0 -m "service-slm 0.2.0"
git push origin service-slm-v0.2.0
```

The `-s` is GPG signing. If you do not yet have a GPG key configured
for GitHub, set one up first — release tags must be signed.

## What happens when you push a tag

[`.github/workflows/release.yml`](../../.github/workflows/release.yml)
runs:

1. Cross-compile to `x86_64-unknown-linux-gnu`,
   `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`,
   `aarch64-apple-darwin`.
2. Generate `service-slm-sbom.json` (CycloneDX) via `cargo sbom`.
3. Generate `THIRD-PARTY-NOTICES.md` via `cargo about`.
4. Sign each artefact with `cosign` keyless (OIDC against GitHub
   Actions).
5. Generate SLSA provenance attestation.
6. Create a GitHub Release with all artefacts and auto-generated
   notes.

## Pre-release checklist

Before pushing a release tag:

- [ ] `main` is green on CI.
- [ ] `CHANGELOG.md` `[Unreleased]` section has been renamed to the
  new version and dated.
- [ ] `STATUS.md` reflects the state being released.
- [ ] Any ADRs merged since the last release are referenced in the
  changelog.
- [ ] The version in `[workspace.package]` of root `Cargo.toml`
  matches the tag.
- [ ] `cargo update` has been run and the lockfile committed.
- [ ] `./scripts/check-all.sh` passes on a fresh clone.

## After the release

- Announce in GitHub Discussions if the project is public.
- Update any downstream references (for example, the os-totebox
  project pinning service-slm to a specific version).
- Open a PR adding a new `[Unreleased]` section to `CHANGELOG.md`.

## Security releases

Follow the same process with two additions:

- Coordinate disclosure timing with the reporter per
  [SECURITY.md](../../SECURITY.md).
- Release notes include a GHSA identifier once available.
