# Brief 4.1 — git2 wiring + commit-on-edit

**target**: Implement `src/git.rs` with `open_or_init`, `commit_topic`, and `ensure_commit_identity_from_env`; wire commit-on-edit into `AppState`, `src/edit.rs`, and `src/main.rs`; add `git2 = "0.20"` to `Cargo.toml`; add integration tests in `tests/git_test.rs`.
**target_files**:
- `app-mediakit-knowledge/src/git.rs` (new)
- `app-mediakit-knowledge/tests/git_test.rs` (new)
- `app-mediakit-knowledge/src/server.rs` (modify — AppState gains `git: Arc<Mutex<Repository>>`)
- `app-mediakit-knowledge/src/edit.rs` (modify — `post_edit` + `post_create` call `git::commit_topic`)
- `app-mediakit-knowledge/src/main.rs` (modify — `serve()` calls `git::open_or_init`; passes repo into AppState)
- `app-mediakit-knowledge/src/lib.rs` (modify — add `pub mod git`)
- `app-mediakit-knowledge/Cargo.toml` (modify — add `git2 = "0.20"`)
**expected_output**: All target files created or modified; `cargo test` passes including the new `tests/git_test.rs` integration tests; manual `git -C <content_dir> log` shows one commit per POST /create and per POST /edit; alternating J/P identity preserved through the git2 commit path.
**max_response_lines**: 400
**model_tier**: sonnet
**parallelisable**: no (writes)
**confidence_gate_passes**: yes — all BP1 decisions cleared 2026-04-28; Q5 (git2 write side) and Q6 (libgit2-dev bundled with libssl-dev in PK.3) answered; no blocking unknowns for this step. [ready-after-PK.3-libssl-libgit2-install]
**layer_scope**: task
**anti_slop_check**: The implementation must name `git2 = "0.20"` in Cargo.toml, `src/git.rs` as the new module, `tests/git_test.rs` as the new integration test file, `Arc<Mutex<Repository>>` as the AppState field type, and J/P identity alternation via `ensure_commit_identity_from_env` as a concrete deliverable — not generic "add git integration" prose.
**dependencies**: none within Phase 4 (this is the root step); requires PK.3 Master pass to have installed `libgit2-dev` alongside `libssl-dev` on the deploy host before `cargo test` with git2 will compile in release mode. Debug binary / `cargo test` works after `apt install libgit2-dev`.

## Specification

Work on feature branch `cluster/project-knowledge`. Run `cargo check` and `cargo test` from inside `app-mediakit-knowledge/` — do not rely on workspace-root `cargo check` (workspace-root pulls `service-content`'s openssl-sys drag per cleanup-log.md 2026-04-18 caveat).

### New module: `src/git.rs`

Implement three public functions:

```rust
pub fn open_or_init(content_dir: &Path) -> Result<git2::Repository, WikiError>
```
Opens the git repository at `content_dir` if one exists, or initialises a new one. Returns `WikiError` on failure. The repo root is `content_dir` itself (the wiki's content directory doubles as the git working tree).

```rust
pub fn commit_topic(
    repo: &Repository,
    slug: &str,
    body: &str,
    author_email: &str,
    author_name: &str,
    message: &str,
) -> Result<git2::Oid, WikiError>
```
Uses `git2`'s `Index::add_path` + `commit` machinery. The file being committed is `<slug>.md` relative to the repo's workdir. Creates or advances the `main` branch. Returns the new commit OID. Commit author and committer are set from the supplied name + email. If the `main` branch does not exist yet (first commit), create it at the new commit.

```rust
pub fn ensure_commit_identity_from_env(repo: &Repository) -> Result<(), WikiError>
```
Falls back to the `bin/commit-as-next.sh` J/P alternation identity when no session author is supplied. Reads the toggle state from `~/Foundry/identity/.toggle` (0 = Jennifer Woodfine / jwoodfine@users.noreply.github.com, 1 = Peter Woodfine / pwoodfine@users.noreply.github.com) and configures the repository's local user.name + user.email accordingly. Phase 5 auth replaces this function when session-based identity lands. If the toggle file is absent or malformed, default to identity 0 (Jennifer) and log a warning rather than erroring.

### Modify: `src/server.rs`

`AppState` gains a new field:
```rust
git: Arc<Mutex<git2::Repository>>,
```
`git2::Repository` is `Send` but mutating operations (index updates, commits) need exclusive access — use the `Arc<Mutex<...>>` pattern, matching the existing `Arc<RwLock<IndexReader>>` tantivy pattern. Update all `AppState` constructors (including those in existing test files: `jsonld_test.rs`, `edit_test.rs`, `squiggle_test.rs`) to supply a test repo opened in a tempdir.

### Modify: `src/edit.rs`

In both `post_edit` and `post_create`, after `atomic_write` succeeds: call `git::commit_topic`. Failures from `commit_topic` are logged at `warn!` level but do NOT roll back the disk write. This is consistent with the existing search-reindex policy (`reindex_topic` failures are also non-fatal to the edit response). The commit message for `post_edit` should be `"edit: {slug}"` and for `post_create` should be `"create: {slug}"`.

Call `ensure_commit_identity_from_env` on the repo before calling `commit_topic` when no explicit author is supplied by the request (Phase 4 has no auth; Phase 5 wires per-session identity).

### Modify: `src/main.rs`

In `serve()`, after `search::build_index` succeeds, call `git::open_or_init(&content_dir)`. Wrap the returned `Repository` in `Arc<Mutex<...>>` and pass into `AppState`. Fail fast (`anyhow::bail!`) if `open_or_init` returns an error — the server should not start with a broken git state.

### Modify: `src/lib.rs`

Add `pub mod git;` to the module declarations.

### `Cargo.toml`

Add to `[dependencies]`:
```toml
git2 = "0.20"
```
`libgit2-sys` is a transitive C dependency. Release build requires `libgit2-dev` system library (bundled with `libssl-dev` in PK.3 Master pass per BP1-Q6). `cargo test` in debug mode works after `apt install libgit2-dev`.

### New file: `tests/git_test.rs`

Integration test suite covering:

1. **Commit per create**: POST /create a new TOPIC slug; assert `git log` on the content_dir shows exactly one commit whose message matches `"create: {slug}"`.
2. **Commit per edit**: POST /edit the same slug; assert `git log` shows a second commit with message `"edit: {slug}"`.
3. **J/P alternation**: create and edit at least two TOPICs in sequence; assert the commit author names alternate between `"Jennifer Woodfine"` and `"Peter Woodfine"` (or both match one identity if the toggle file is stable — acceptable either way; the key assertion is that the author is one of the two, not a blank/default identity).
4. **open_or_init idempotency**: call `open_or_init` twice on the same directory; assert the second call succeeds and returns the same repo (no double-init error).

Tests use a `tempdir`-backed `AppState` (same pattern as `edit_test.rs`). Each test creates a fresh tempdir for isolation.

### `cargo check` discipline

Run `cargo check` from inside `app-mediakit-knowledge/` after implementing. Do not run from `pointsav-monorepo/` root — workspace-root check pulls `service-content`'s openssl-sys which fails without `libssl-dev`. The crate-scoped check is the correct path.

## Acceptance criteria

- `cargo test` (from `app-mediakit-knowledge/`) passes including all tests in `tests/git_test.rs`.
- Manual `git -C <content_dir> log` after a POST /create + POST /edit sequence shows exactly two commits with expected messages.
- `AppState` has the `git: Arc<Mutex<Repository>>` field; all existing tests still compile and pass with updated `AppState` constructors.
- `src/git.rs` exports the three functions with the exact signatures specified above.
- `Cargo.toml` contains `git2 = "0.20"` in `[dependencies]`.
- J/P identity alternation is verified in at least one test assertion.
- No `[PENDING-*]` tokens in the produced files.
- Commit lands on `cluster/project-knowledge` branch, not `main`.

## Risks

- `git2 = "0.20"` requires `libgit2-dev` at link time (C library). In debug mode, `cargo test` may fall back to bundled libgit2 if `libgit2-sys`'s `bundled` feature is enabled — check Cargo.toml after adding the dep and confirm whether bundled build suffices for CI or whether the PK.3 system-lib install is strictly required for `cargo test` to pass.
- The `~/Foundry/identity/.toggle` path is workspace-specific. In tests, `ensure_commit_identity_from_env` should accept an optional path override (or the test can pre-write the toggle file in the tempdir) so the test does not depend on the workspace toggle state.
- `Arc<Mutex<Repository>>` introduces a lock that is held during the `commit_topic` call. Keep the lock scope minimal: acquire, commit, release. Do not hold the lock across the `atomic_write` disk operation.
