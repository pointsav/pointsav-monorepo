# Getting started

This is the walkthrough for a new contributor on a fresh machine.
Budget 30 minutes.

## Prerequisites

You need:

- **A Unix-like host.** Linux or macOS. Windows via WSL2 works;
  native Windows is not tested.
- **Git** configured with your contributor identity (`git config --global user.email
  you@domain`).
- **`rustup`.** If you do not have it: `curl --proto '=https' --tlsv1.2
  -sSf https://sh.rustup.rs | sh`. The toolchain version is read from
  `rust-toolchain.toml`; you do not need to pick one.
- **About 6 GB of free disk** for the Rust toolchain and
  `target/` directory.

## First clone and build

```bash
git clone git@github.com:woodfinegroup/pointsav-monorepo.git
cd pointsav-monorepo/service-slm

# Installs supplementary tools (cargo-audit, cargo-deny, cargo-sbom,
# cargo-about).
./scripts/bootstrap.sh

# Run the full local check suite. Must pass on a clean checkout.
./scripts/check-all.sh
```

If anything fails at this stage, that is a bug. Open an issue.

## The editor experience

We use `rust-analyzer` and `clippy`. The repository ships a
`rustfmt.toml` and `clippy.toml` with project settings; your editor
should pick them up automatically.

Recommended VS Code extensions:

- `rust-lang.rust-analyzer`
- `tamasfe.even-better-toml`
- `vadimcn.vscode-lldb`

## Your first change

1. Read [CONTRIBUTING.md](../../CONTRIBUTING.md), [CLAUDE.md](../../CLAUDE.md),
   and [STATUS.md](../../STATUS.md).
2. Pick an open task from [TASKS.md](../../TASKS.md) — ideally a P1 you
   have context for.
3. Branch: `git checkout -b <scope>/<short-description>`.
4. Work inside a single crate where possible. Read the crate's
   `CLAUDE.md` and `README.md` before touching its code.
5. Commit often with `git commit -m "scope: …"`.
6. Run `./scripts/check-all.sh` before pushing.
7. Open a PR using the template. Reviewers are assigned
   automatically per `CODEOWNERS`.

## Where things are

| Need | Path |
|---|---|
| Project overview | [README.md](../../README.md) |
| Invariants and session protocol | [CLAUDE.md](../../CLAUDE.md) |
| Decisions of record | [docs/adr/](../adr/) |
| Normative specifications | [specs/](../../specs/) |
| Work queue | [TASKS.md](../../TASKS.md) |
| Crate status | [STATUS.md](../../STATUS.md) |

## Asking for help

- Design questions → GitHub Discussions.
- Bugs or concrete feature requests → GitHub Issues.
- Security → [SECURITY.md](../../SECURITY.md).
- Anything else → peter@woodfinegroup.com.
