@~/Foundry/AGENT.md

# pointsav-monorepo — Sub-clone Guide

> **This file is generic.** It applies to every Totebox archive that
> clones `pointsav-monorepo`. Archive-specific context lives in the
> parent archive's `.agent/manifest.md` and `CLAUDE.md`.
> **Workspace AGENT.md takes precedence on conflict.**

---

## Role in the workspace

`pointsav-monorepo` is the canonical vendor source tree. Each Totebox
archive checks out a copy (`clones/<archive>/pointsav-monorepo/`) and
works on a feature branch. Stage 6 promotion (`~/Foundry/bin/promote.sh`,
run from Command Session) merges to `pointsav/pointsav-monorepo` canonical.

## Commit + promote

- Stage changes: `git add <specific files>` (never `git add .` or `-A`)
- Commit: `~/Foundry/bin/commit-as-next.sh "<message>"` from this directory
- Stage 6: write "Stage 6 pending — <archive> — <crate path>" to archive outbox;
  Command Session runs `bin/promote.sh`

## Allowed Cargo commands (no approval needed)

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
cargo check
cargo check -p <crate-name>
```

## Hard rules (workspace-level; do not override here)

- One session per `.git/index` — two sessions on this sub-clone race
- Identity keys `identity/` are 0600 mathew-only — never chmod
- Direct `git commit` is blocked (pre-commit gate); use `commit-as-next.sh`
- No `git add .` or `git add -A`
- Commit author: only `jwoodfine` or `pwoodfine` via `commit-as-next.sh`
