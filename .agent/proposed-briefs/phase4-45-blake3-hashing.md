# Brief 4.5 — blake3 hash storage (federation-seam baseline)

**target**: Extend `src/links.rs` (or new `src/hashing.rs`) with a second redb table `hashes`; wire `record_hash` into `src/edit.rs` after `git::commit_topic`; add `blake3 = "1.8"` to `Cargo.toml`; extend `tests/links_test.rs` with hash-table assertions.
**target_files**:
- `app-mediakit-knowledge/src/links.rs` (modify — add hashes table + `record_hash` + `lookup_by_hash`) OR `app-mediakit-knowledge/src/hashing.rs` (new — if strict separation is preferred; see Specification)
- `app-mediakit-knowledge/tests/links_test.rs` (modify — extend with hash table tests)
- `app-mediakit-knowledge/src/edit.rs` (modify — `post_edit` + `post_create` call `record_hash` after `git::commit_topic` returns the sha)
- `app-mediakit-knowledge/Cargo.toml` (modify — add `blake3 = "1.8"`)
**expected_output**: Every commit records a `blake3` hash of the TOPIC body in redb keyed by `(slug, revision_sha)`; the hash table has one entry per edit; `cargo test` passes including extended `tests/links_test.rs`; redb file at `<state_dir>/links.redb` shows two tables.
**max_response_lines**: 260
**model_tier**: sonnet
**parallelisable**: no (writes)
**confidence_gate_passes**: yes — no blocking unknowns; `blake3` is pure Rust, no system-lib gates; redb schema extension is additive over 4.4.
**layer_scope**: task
**anti_slop_check**: Must name `blake3 = "1.8"`, the `hashes` redb table with key `(slug, revision_sha)` → `[u8; 32]`, `record_hash` and `lookup_by_hash` function signatures, and the extension of `tests/links_test.rs` (not a new test file) as concrete deliverables.
**dependencies**: Step 4.4 must be complete (this step extends the redb database opened in 4.4; the `LinkGraph` struct and `open_or_create` function are the extension point). Step 4.1 must be complete (`record_hash` is called with the revision_sha returned by `git::commit_topic`, which 4.1 establishes).

## Specification

Work on feature branch `cluster/project-knowledge`. Run `cargo check` and `cargo test` from inside `app-mediakit-knowledge/` — not from workspace root.

### Module choice: `src/links.rs` vs `src/hashing.rs`

Two acceptable implementations:

**Option A (extend `src/links.rs`)**: Add the `hashes` table definition and the two new public functions to the existing `links.rs` module. Simpler; keeps all redb logic in one place. Preferred if the agent determines the coupling is not problematic.

**Option B (new `src/hashing.rs`)**: Extract hash-related functions into a separate module. Add `pub mod hashing;` to `src/lib.rs`. Requires reading `src/links.rs` to understand the `LinkGraph` struct, then either (a) accepting a `&LinkGraph` parameter (shares the redb database handle) or (b) opening a separate redb database at `<state_dir>/hashes.redb`. Use a shared database (Option A or Option B with `&LinkGraph`) to avoid two open file handles on the same state directory.

The agent chooses based on code-quality judgment. Either option is acceptable provided the acceptance criteria are met.

### Schema extension

Add a second redb table to the database opened in `links::open_or_create`:

```
TABLE hashes: (slug: &str, revision_sha: &str) → [u8; 32]
```

Key: composite `(slug, revision_sha)`. Value: 32-byte blake3 digest of the TOPIC body at that revision. This is a write-once append table — existing entries are never overwritten.

Add the table definition to the `open_or_create` function alongside the existing `links` table definition. Both tables are defined on the same `redb::Database` instance.

### New functions

```rust
pub fn record_hash(
    graph: &LinkGraph,
    slug: &str,
    revision_sha: &str,
    body: &[u8],
) -> Result<(), WikiError>
```
Computes `blake3::hash(body)` (32 bytes). Inserts `(slug, revision_sha) → digest` into the `hashes` table in a new `WriteTransaction`. No-op if the `(slug, revision_sha)` key already exists (idempotent).

```rust
pub fn lookup_by_hash(
    graph: &LinkGraph,
    hash: &[u8; 32],
) -> Result<Option<(String, String)>, WikiError>
```
Reverse lookup: scans the `hashes` table for an entry whose value matches `hash`; returns `Some((slug, revision_sha))` if found, `None` if not. This is a full-table scan in Phase 4 (acceptable for small wikis). Phase 7 lights up this function as the content-addressed read endpoint at `GET /api/v1/page/{hash}` — it is a no-op stub in Phase 4 in the sense that the route itself is not mounted, but the function is implemented and tested here.

### Modify: `src/edit.rs`

In both `post_edit` and `post_create`, after `git::commit_topic` returns the new `sha: git2::Oid`:
```rust
let sha_str = sha.to_string();
links::record_hash(&state.links, &slug, &sha_str, body.as_bytes())
    .unwrap_or_else(|e| warn!("record_hash failed for {slug}: {e}"));
```
Failures are logged at `warn!` level and do NOT block the response (consistent with the existing non-fatal policy for git commits and search reindexing).

The `body` variable is already in scope at this point in `post_edit` and `post_create` (it is the string written by `atomic_write`).

### `Cargo.toml`

Add to `[dependencies]`:
```toml
blake3 = "1.8"
```
`blake3` is pure Rust; no system-lib dependency.

### Extend `tests/links_test.rs`

Add the following test cases to the existing test file (do not create a new file):

1. **Hash recorded on create**: POST /create a TOPIC; assert the `hashes` table in redb has exactly one entry for `(slug, sha)` where `sha` matches the git log commit sha.
2. **Hash recorded on edit**: POST /edit the same TOPIC; assert the `hashes` table now has two entries for `slug` (one per revision).
3. **lookup_by_hash round-trip**: compute `blake3::hash(body.as_bytes())` locally; call `lookup_by_hash` with that digest; assert it returns `Some((slug, sha))` matching the known values.
4. **Distinct hashes for distinct bodies**: create two TOPICs with different bodies; assert their hashes differ.

### `cargo check` discipline

Run from inside `app-mediakit-knowledge/`. `blake3` is pure Rust and will not introduce new system-lib requirements.

## Acceptance criteria

- `cargo test` (from `app-mediakit-knowledge/`) passes including all extended tests in `tests/links_test.rs`.
- `redb` file at `<state_dir>/links.redb` contains two tables: `links` and `hashes`.
- `record_hash` is called from both `post_edit` and `post_create` and is non-fatal on failure.
- `lookup_by_hash` is implemented (even though no route mounts it in Phase 4) and tested.
- `Cargo.toml` contains `blake3 = "1.8"`.
- No `[PENDING-*]` tokens in produced files.
- Commit lands on `cluster/project-knowledge` branch, not `main`.

## Risks

- `revision_sha` is a `git2::Oid` stringified to 40 hex chars; use the full-length sha as the key (not short sha) for the `hashes` table to ensure uniqueness and consistency with Phase 7's content-addressed retrieval.
- The redb schema now has two tables. If `open_or_create` from Step 4.4 is called on an existing `links.redb` file that only has the `links` table, adding the `hashes` table definition should succeed (redb allows adding new tables to an existing database). Verify this empirically; document in a code comment.
- Phase 7 will mount `GET /api/v1/page/{hash}` using `lookup_by_hash`. The function signature here is the contract that Phase 7 depends on. Do not change it after Phase 4 ships without a corresponding update to Phase 7's brief.
