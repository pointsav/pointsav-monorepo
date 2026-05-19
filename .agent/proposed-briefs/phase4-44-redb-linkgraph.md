# Brief 4.4 — redb link-graph index + GET /backlinks/{slug}

**target**: Implement `src/links.rs` with the `LinkGraph` struct backed by `redb`; mount `GET /backlinks/{slug}` in `src/server.rs`; wire `rebuild_for_slug` into `src/edit.rs`; initialise `LinkGraph` in `src/main.rs`; add `redb = "4.1"` to `Cargo.toml`; add integration tests in `tests/links_test.rs`.
**target_files**:
- `app-mediakit-knowledge/src/links.rs` (new)
- `app-mediakit-knowledge/tests/links_test.rs` (new)
- `app-mediakit-knowledge/src/server.rs` (modify — mount backlinks route; AppState gains links field)
- `app-mediakit-knowledge/src/edit.rs` (modify — `post_edit` + `post_create` call `links::rebuild_for_slug`)
- `app-mediakit-knowledge/src/main.rs` (modify — build LinkGraph on startup; pass into AppState)
- `app-mediakit-knowledge/src/lib.rs` (modify — add `pub mod links`)
- `app-mediakit-knowledge/Cargo.toml` (modify — add `redb = "4.1"`)
**expected_output**: Backlinks route returns JSON list of slugs; link graph rebuilds on every edit; `cargo test` passes including `tests/links_test.rs`; manual smoke shows backlinks panel on `/wiki/{slug}`.
**max_response_lines**: 380
**model_tier**: sonnet
**parallelisable**: no (writes)
**confidence_gate_passes**: yes — no blocking unknowns; `redb` and `comrak` are pure-Rust deps, no system-lib gates.
**layer_scope**: task
**anti_slop_check**: Must name `src/links.rs`, `tests/links_test.rs`, `LinkGraph` struct, `redb = "4.1"`, `links.redb` as the on-disk file name, the three public functions (`open_or_create`, `rebuild_for_slug`, `backlinks`), and the specific route `GET /backlinks/{slug}` as concrete deliverables.
**dependencies**: Step 4.1 must be complete (the trigger for `rebuild_for_slug` is wired inside `post_edit`/`post_create` alongside the git commit call; the AppState pattern is established by 4.1). Steps 4.2 and 4.3 are not required — 4.4 runs in parallel with the history/diff branch after 4.1 completes.

## Specification

Work on feature branch `cluster/project-knowledge`. Run `cargo check` and `cargo test` from inside `app-mediakit-knowledge/` — not from workspace root.

### New module: `src/links.rs`

```rust
pub struct LinkGraph {
    db: Arc<redb::Database>,
}
```

The `LinkGraph` holds a `redb::Database` wrapped in `Arc` for shared ownership. Phase 4 uses a single table:

```
TABLE links: (from_slug: &str, to_slug: &str) → ()
```

Key design: composite key `(from_slug, to_slug)` in lexicographic order, value is unit. This allows:
- Forward query: scan all keys with prefix `(slug, _)` to find all TOPICs that `slug` links to.
- Reverse query (backlinks): scan all keys with suffix `(_, slug)` — range over all from_slugs for a given to_slug. Because `redb` keys are byte-ordered, a range scan over the second component requires a secondary index or a full-table scan. Use a full-table scan for Phase 4 (the link table is small; Phase 6 adds an inverted index if scale requires it).

Implement:
```rust
pub fn open_or_create(state_dir: &Path) -> Result<LinkGraph, WikiError>
```
Opens `<state_dir>/links.redb` if it exists, creates it if not. Defines the `links` table on first open.

```rust
pub fn rebuild_for_slug(
    graph: &LinkGraph,
    slug: &str,
    body: &str,
) -> Result<(), WikiError>
```
Parses all wikilinks from `body` using the existing `comrak` extension (the same parser already used in Phase 1/2 rendering — reuse the existing `comrak::ComrakExtensionOptions` configuration with `wikilinks_title_after_pipe = true`). Extracts the slug component of each `[[Target]]` or `[[Target|Display]]` wikilink.

Deletes all existing `(slug, *)` rows from the `links` table (to handle removed links on edit). Inserts `(slug, target_slug)` for each parsed wikilink target. All in a single `redb::WriteTransaction`.

```rust
pub fn backlinks(graph: &LinkGraph, slug: &str) -> Result<Vec<String>, WikiError>
```
Scans the full `links` table; returns all `from_slug` values where `to_slug == slug`. Sorted alphabetically for stable output.

### Modify: `src/server.rs`

`AppState` gains a new field:
```rust
links: Arc<LinkGraph>,
```
`LinkGraph` wraps `Arc<redb::Database>` internally; wrap the `LinkGraph` in `Arc<LinkGraph>` for `AppState` sharing across request handlers.

Mount route:

**`GET /backlinks/{slug}`** — returns a JSON array of slug strings: `["slug-a", "slug-b"]`. Content-Type: `application/json`. Additionally, on the existing `GET /wiki/{slug}` page, include a backlinks panel in the page footer area showing the count and links as HTML. The backlinks panel is purely additive — it does not change the existing wiki page structure.

Return 404 if the target slug does not exist as a TOPIC file.

### Modify: `src/edit.rs`

In both `post_edit` and `post_create`, after the disk write and after `git::commit_topic`, call:
```rust
links::rebuild_for_slug(&state.links, &slug, &body)
```
Failures are logged at `warn!` level but do NOT roll back the disk write (consistent with the existing git commit + search reindex policy — non-fatal to the edit response).

Update `AppState` constructors in all existing test files to supply a `LinkGraph` opened in a tempdir. This is the same constructor-update discipline applied in Step 4.1 for the git field.

### Modify: `src/main.rs`

On startup, after `git::open_or_init`, call:
```rust
links::open_or_create(&state_dir)
```
Wrap in `Arc<LinkGraph>` and pass into `AppState`. Fail fast if `open_or_create` returns an error.

Do NOT rebuild the full link graph on startup — the redb file persists across restarts. The graph is rebuilt incrementally on each edit. A future maintenance command (`--rebuild-links`) can force a full rebuild; Phase 4 does not need it.

### Modify: `src/lib.rs`

Add `pub mod links;` to the module declarations.

### `Cargo.toml`

Add to `[dependencies]`:
```toml
redb = "4.1"
```
`redb` is pure Rust; no system lib dependency.

### New file: `tests/links_test.rs`

Integration test suite covering:

1. **Backlinks add on create**: create TOPIC A with body `[[B]]`; assert GET /backlinks/B returns `["a"]`.
2. **Backlinks add on edit**: edit TOPIC A to also include `[[C]]`; assert GET /backlinks/C returns `["a"]`.
3. **Backlinks remove on edit**: edit TOPIC A to remove `[[B]]`; assert GET /backlinks/B returns `[]`.
4. **Multiple inbound links**: create TOPIC A linking to C and TOPIC D linking to C; assert GET /backlinks/C returns `["a", "d"]` (sorted).
5. **JSON content-type**: assert GET /backlinks/{slug} response has `Content-Type: application/json`.
6. **Unknown target slug**: GET /backlinks/nonexistent; assert HTTP 404.

### `cargo check` discipline

Run from inside `app-mediakit-knowledge/`. `redb` is pure Rust and will not introduce new system-lib requirements.

## Acceptance criteria

- `cargo test` (from `app-mediakit-knowledge/`) passes including all tests in `tests/links_test.rs`.
- Manual smoke: create two TOPICs where A links to B; GET /backlinks/B returns `["a"]`; edit A to remove the link; GET /backlinks/B returns `[]`.
- `links.redb` file appears at `<state_dir>/links.redb` after first edit.
- `src/links.rs` exports `open_or_create`, `rebuild_for_slug`, `backlinks`, and `LinkGraph` with the specified signatures.
- `AppState` has the `links: Arc<LinkGraph>` field; all existing tests still compile and pass with updated constructors.
- Backlinks panel appears in the wiki page HTML (additive only, does not break existing chrome).
- No `[PENDING-*]` tokens in produced files.
- Commit lands on `cluster/project-knowledge` branch, not `main`.

## Risks

- `redb = "4.1"` table schema is defined at open time; changing the schema after the file exists requires migration or deletion. Phase 4 defines two tables total (links in 4.4, hashes in 4.5). Design the schema with the hashes table in mind so both can be defined on the same `open_or_create` call — or give each table a separate open function that can be called idempotently. Coordinate with Brief 4.5 which extends `src/links.rs` with the hashes table.
- Full-table scan in `backlinks()` is O(n) in the number of link pairs. Acceptable for Phase 4 (wiki content is small). If the wiki grows to thousands of TOPICs, a secondary inverted index is needed — log this as a Phase 6 item in the NEXT.md.
- Wikilink parsing via `comrak`: confirm the existing `ComrakExtensionOptions` configuration (from Phase 1/2 rendering) includes `wikilinks_title_after_pipe = true`. Read `src/render.rs` to verify before implementing the parser in `links.rs`. Reuse the same parser options to ensure consistency.
