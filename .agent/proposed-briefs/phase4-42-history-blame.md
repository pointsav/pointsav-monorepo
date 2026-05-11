# Brief 4.2 — GET /history/{slug} + GET /blame/{slug}

**target**: Implement `src/history.rs` with `topic_history` and `topic_blame` using `gix` (read-side); mount `GET /history/{slug}` and `GET /blame/{slug}` routes in `src/server.rs`; add `gix = "0.66"` and `gix-blame = "0.16"` (or current) to `Cargo.toml`; add integration tests in `tests/history_test.rs`.
**target_files**:
- `app-mediakit-knowledge/src/history.rs` (new)
- `app-mediakit-knowledge/tests/history_test.rs` (new)
- `app-mediakit-knowledge/src/server.rs` (modify — mount two new routes)
- `app-mediakit-knowledge/src/lib.rs` (modify — add `pub mod history`)
- `app-mediakit-knowledge/Cargo.toml` (modify — add `gix`, `gix-blame`)
**expected_output**: Both routes render valid HTML pages; `cargo test` passes including `tests/history_test.rs`; manual smoke shows `/history/{slug}` as a table of commit entries and `/blame/{slug}` with per-line author + sha annotations.
**max_response_lines**: 380
**model_tier**: sonnet
**parallelisable**: no (writes)
**confidence_gate_passes**: yes — BP1-Q5 (gix read side) and Q6 (libgit2-dev in PK.3) cleared 2026-04-28. [ready-after-PK.3-libssl-libgit2-install]
**layer_scope**: task
**anti_slop_check**: Must name `src/history.rs`, `tests/history_test.rs`, `gix = "0.66"`, `gix-blame = "0.16"` (or current), the `HistoryEntry` and `BlameLine` struct shapes, and the specific routes `GET /history/{slug}` and `GET /blame/{slug}` as concrete deliverables.
**dependencies**: Step 4.1 must be complete (this step reads from the git repository populated by 4.1; history queries are meaningless without at least one commit; AppState.git established in 4.1 is used here). Requires PK.3 libgit2-dev install for git2 link (transitive from 4.1's git2 dep).

## Specification

Work on feature branch `cluster/project-knowledge`. Run `cargo check` and `cargo test` from inside `app-mediakit-knowledge/` — not from workspace root.

### New module: `src/history.rs`

This module uses `gix` (read-side) per BP1-Q5. The write side (commits) lives in `src/git.rs` (git2); this module is pure read.

Define structs:
```rust
pub struct HistoryEntry {
    pub sha: String,
    pub author: String,
    pub email: String,
    pub timestamp_iso: String,
    pub message: String,
}

pub struct BlameLine {
    pub line_number: usize,
    pub line_text: String,
    pub sha: String,
    pub author: String,
    pub timestamp_iso: String,
}
```

Implement:
```rust
pub fn topic_history(
    content_dir: &Path,
    slug: &str,
    limit: usize,
) -> Result<Vec<HistoryEntry>, WikiError>
```
Opens the `gix` repository at `content_dir`. Walks the `main` branch commit history. Filters to commits that touch `<slug>.md`. Returns up to `limit` entries (most-recent first). If no commits touch the file, returns an empty `Vec` (not an error). Uses `gix::traverse::commit::Simple` or equivalent API to walk.

```rust
pub fn topic_blame(
    content_dir: &Path,
    slug: &str,
) -> Result<Vec<BlameLine>, WikiError>
```
Uses `gix-blame` crate for per-line blame. Returns one `BlameLine` per line of `<slug>.md` at HEAD. If the file has no commits yet, returns an empty `Vec`. The `sha` field should be the short sha (7 chars).

Note on `gix-blame` version: the plan lists `"0.16"` with a "or current" qualifier. Run `cargo search gix-blame` at implementation time to confirm the current crate version and use that. Document the version actually used in a comment in `Cargo.toml`.

### Modify: `src/server.rs`

Mount two new routes:

**`GET /history/{slug}`** — HTML page. Renders `topic_history(content_dir, slug, 50)` as an HTML table with columns: sha (linked to a future `/diff/{slug}?b=<sha>&a=<sha>~`), author, date, message. If no history exists, render a friendly "No revision history yet" placeholder. Re-use the existing page chrome (sidebar, header) from the render module.

**`GET /blame/{slug}`** — HTML page. Renders `topic_blame(content_dir, slug)` as a source-code view with per-line annotation: left column = sha (7 chars) + author name; right column = line text. If no blame data exists, render the raw file content without annotation. Re-use existing page chrome.

Both routes return 404 if the slug does not correspond to an existing TOPIC file.

### Modify: `src/lib.rs`

Add `pub mod history;` to the module declarations.

### `Cargo.toml`

Add to `[dependencies]`:
```toml
gix = "0.66"
gix-blame = "0.16"  # or current — verify with `cargo search gix-blame` at implementation time
```
`gix` is pure Rust (no system lib dependency). `gix-blame` is also pure Rust. These deps do not add to the system-lib surface beyond what git2 already requires.

### New file: `tests/history_test.rs`

Integration test suite covering:

1. **History list**: seed a TOPIC with N=3 edits via POST /create + POST /edit × 2; assert GET /history/{slug} response body contains N table rows (or structured content indicating N commits); assert the most-recent entry is most prominent.
2. **Blame annotation**: after seeding, assert GET /blame/{slug} returns HTML where each line of the TOPIC body has an associated sha annotation (non-empty sha column in the rendered output).
3. **Empty history**: request GET /history/{slug} for a slug with no commits; assert HTTP 200 with a "No revision history" placeholder (not 404, not 500).
4. **Unknown slug**: request GET /history/nonexistent-slug; assert HTTP 404.

Tests use a `tempdir`-backed AppState consistent with the pattern established in Step 4.1.

### `cargo check` discipline

Run from inside `app-mediakit-knowledge/`. The `gix` crate is pure Rust and will compile without system libs. Confirm `cargo check` passes before `cargo test`.

## Acceptance criteria

- `cargo test` (from `app-mediakit-knowledge/`) passes including all tests in `tests/history_test.rs`.
- Manual smoke: GET /history/{slug} on a TOPIC with multiple edits renders a table of commit entries.
- Manual smoke: GET /blame/{slug} renders source with per-line sha + author annotations.
- `src/history.rs` exports `topic_history`, `topic_blame`, `HistoryEntry`, `BlameLine` with the specified signatures.
- `Cargo.toml` contains `gix` and `gix-blame` entries.
- Both routes return 404 for unknown slugs, not 500.
- No `[PENDING-*]` tokens in produced files (record the actual gix-blame version used).
- Commit lands on `cluster/project-knowledge` branch, not `main`.

## Risks

- `gix-blame` API stability: the crate is younger than `git2`; the implementation should wrap any `gix-blame` calls in a thin adapter function so a future API change requires editing only `topic_blame`, not call sites throughout the module.
- `gix` and `git2` both open the same repository directory. This is safe for concurrent read (gix read + git2 write) because both operate on the same on-disk git object store. However, the `gix` open should not hold a file lock that interferes with git2's index lock during commits — gix read paths do not take the index lock, so this is expected to be safe; verify empirically.
- `gix = "0.66"` may not be the latest release at implementation time. Run `cargo search gix` and use the current stable release; update the version in Cargo.toml accordingly. Record the version in a comment.
