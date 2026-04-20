# STATUS.md — crate status matrix

Machine-readable status of every workspace crate. Updated on every PR that
changes a crate's maturity. Claude Code reads this at session start.

Status levels:

- **scaffold** — crate compiles; `__scaffold_placeholder()` is the only item.
- **alpha** — real code exists; public API shape is unstable.
- **beta** — public API stable; missing coverage or polish.
- **ga** — production-ready; full tests; CI coverage ≥ the workspace target.

| Crate | Status | Last touched | Next milestone |
|---|---|---|---|
| slm-core | scaffold | 2026-04-20 | ModuleId newtype + serde tests |
| slm-doorman | scaffold | 2026-04-20 | SanitisationPolicy type + pass-through |
| slm-ledger | scaffold | 2026-04-20 | Event struct with 10 variants |
| slm-compute | scaffold | 2026-04-20 | Parse compute/manifest.yaml |
| slm-memory-kv | scaffold | 2026-04-20 | Deterministic block hash |
| slm-memory-adapters | scaffold | 2026-04-20 | Registry YAML parser |
| slm-inference-local | scaffold | 2026-04-20 | RAM probe + quantisation selector |
| slm-inference-remote | scaffold | 2026-04-20 | HTTP client + BOOT_* ledger events |
| slm-api | scaffold | 2026-04-20 | Router skeleton + /health |
| slm-cli | scaffold | 2026-04-20 | Wire subcommands to stubbed crate calls |

## Workspace-level checks

| Check | State | Notes |
|---|---|---|
| `cargo build --workspace` | pass | empty crates compile |
| `cargo test --workspace` | pass | one smoke test per crate |
| `cargo clippy -D warnings` | pass | pedantic enabled |
| `cargo fmt --check` | pass | default style |
| `cargo audit` | not installed locally | runs in CI |
| `cargo deny check` | not installed locally | runs in CI |
| CI workflow green | pass | verified on first push to main (2026-04-20) |

Verify the "unverified" row on first push and update this table.

## How to update this file

When you change a crate's maturity, update its row. When you land a
workspace-level change that affects the checks table, update that table.
CI fails if `STATUS.md` has not been modified in a PR that changes
crate-level public API.
