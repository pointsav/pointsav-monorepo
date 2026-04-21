# Pre-commit checklist for the service-slm scaffold

Before committing this scaffold to the `pointsav-monorepo`, verify the
following. Each item is scored by severity: **must-fix** blocks the first
commit; **should-fix** can land as a follow-up PR; **note** is an FYI.

---

## Must-fix before first push

### [1] Peter rewrites ADR-0003 rationale — DONE 2026-04-20

Resolved as part of the AGPL-3.0 alignment work: `docs/adr/0003-agpl3-for-own-code.md`
now reflects Woodfine's actual organisational reasoning (alignment to
`factory-release-engineering/README.md §3`) and the editor's-note block
has been removed.

### [2] Confirm `pwoodfine` and `jwoodfine` GitHub handles are current

`CODEOWNERS` and `MAINTAINERS.md` both reference these two handles. If
either handle has changed since 2026-04-20, the CI auto-assignment
will be broken on the first PR.

**Action:** Verify both handles exist and belong to the expected
people. If a handle is wrong, update `CODEOWNERS` and `MAINTAINERS.md`
in the same commit.

### [3] Placement in the monorepo

The scaffold assumes it lives at `pointsav-monorepo/service-slm/`. The
CI workflow paths, the relative links in docs, and the licence
compatibility between the monorepo's own licence (whatever that is)
and AGPL-3.0-only should all be verified before the first push.

**Action:** Confirm the monorepo's top-level `LICENSE` is permissive.
AGPL-3.0-only is compatible with MIT/Apache-2.0 and other permissive
inbounds (all permitted under `deny.toml`), but a more restrictive
parent licence would complicate distribution. If the monorepo root
uses per-directory licensing per `factory-release-engineering/README.md §3`,
no action needed.

---

## Should-fix before second or third push

### [4] Pin dependency versions

`Cargo.toml` contains indicative minimum versions based on SLM-STACK §11.
`TASKS.md` entry **#3** tracks this.

**Action:** At Phase 2 kickoff (not now), run `cargo update` and pin
each workspace-level dep to the current latest. Commit `Cargo.lock`.

### [5] Verify CI workflows on first push

The CI workflows in `.github/workflows/` are structurally correct but
unexercised. The first push to `main` will be the first time they
run.

**Action:** Watch the first CI run. If any step fails for a
non-scaffold reason, open a follow-up issue.

### [6] `cargo about` template

`release.yml` calls `cargo about generate about.hbs` to produce
`THIRD-PARTY-NOTICES.md`. The `about.hbs` template is not included —
it needs to be authored before the first release tag.

**Action:** Before cutting the first release, add `about.hbs` at the
repo root. A standard template is in the `cargo-about` documentation.

### [7] Sigstore setup

The `release.yml` workflow signs artefacts using Sigstore keyless
signing via the GitHub Actions OIDC issuer. No secrets are required,
but the first release should be run in a way that lets Peter verify
the signing chain before cutting a public release.

**Action:** First release should be a pre-release tag
(`service-slm-v0.1.0-alpha.1`) so any signing issues can be fixed
before a `v0.1.0` appears.

---

## Notes (FYI, no action required now)

### [8] The `slm-cli` binary currently does nothing

Every subcommand returns an `anyhow::bail!` with a pointer to the next
task. This is intentional scaffolding; real behaviour lands per the
`TASKS.md` queue.

### [9] Dependencies marked `# TODO` in `Cargo.toml`

Three workspace dep entries carry `# TODO: confirm at Phase 2 kickoff`
comments: LadybugDB bindings, mistral.rs CUDA features, and the
`google-cloud-run` crate (not yet on crates.io as of writing). These
are known open items and are tracked in `TASKS.md`.

### [10] The `specs/` directory is read-only

The two files in `specs/` are verbatim copies of the master
specifications that live at the monorepo root. `CLAUDE.md` and
`GOVERNANCE.md` both document that edits to `specs/` are a governance
event. Updates flow: change at monorepo root → re-copy here → PR.

### [11] REUSE compliance

`.reuse/dep5` declares licence metadata for files that cannot easily
carry an inline SPDX header (the `LICENSE` file itself, `CODEOWNERS`,
etc.). The `LICENSES/AGPL-3.0-only.txt` file is required by REUSE; it is
a copy of the root `LICENSE` to satisfy the REUSE tool's expected
layout. CI runs `reuse lint` on every PR.

### [12] Claude Code memory drift protection

`CLAUDE.md` (canonical) and `AGENTS.md` (framework-agnostic pointer)
are kept in sync by `cargo xtask verify-agents-parity`, which CI runs.
If someone adds project rules to `AGENTS.md` instead of `CLAUDE.md`,
CI fails.

### [13] The placeholder `__scaffold_placeholder()` function

Every library crate's `lib.rs` exports a `pub fn
__scaffold_placeholder()` that does nothing. This keeps the workspace
compiling and the smoke tests green. Remove it as each crate ships
real items, along with the matching smoke test.

---

## First-commit sequence

Recommended:

```bash
# Drop the scaffold into the monorepo
cd pointsav-monorepo
unzip service-slm-scaffold.zip   # or however you move it in

# Verify structure
ls service-slm/

# Initialise (from the monorepo root, not the service-slm subdir)
git add service-slm/

# Review the diff one section at a time
git diff --cached service-slm/LICENSE
git diff --cached service-slm/docs/adr/0003-agpl3-for-own-code.md
# ^ re-read this one carefully before committing

# Peter edits ADR-0003 rationale, then:
git add service-slm/docs/adr/0003-agpl3-for-own-code.md

# Commit
git commit -m "service-slm: initial scaffold

Ten-crate workspace per SLM-STACK §5.1. AGPL-3.0-only per ADR-0003.
Governance, CI, and Claude Code memory hierarchy installed. No
business logic yet; see service-slm/TASKS.md for the work queue."

# First push
git push origin main
# Watch .github/workflows/ci.yml run; should pass all jobs except
# possibly REUSE (which depends on the monorepo's broader layout).
```
