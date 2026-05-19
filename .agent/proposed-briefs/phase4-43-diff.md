# Brief 4.3 — GET /diff/{slug}?a=&b=

**target**: Implement `src/diff.rs` with `topic_diff` using `gix-diff`; mount `GET /diff/{slug}?a=&b=` in `src/server.rs`; add `gix-diff` to `Cargo.toml`; add integration tests in `tests/diff_test.rs`.
**target_files**:
- `app-mediakit-knowledge/src/diff.rs` (new)
- `app-mediakit-knowledge/tests/diff_test.rs` (new)
- `app-mediakit-knowledge/src/server.rs` (modify — mount one new route)
- `app-mediakit-knowledge/src/lib.rs` (modify — add `pub mod diff`)
- `app-mediakit-knowledge/Cargo.toml` (modify — add `gix-diff`)
**expected_output**: The diff route renders a unified-diff HTML page; `cargo test` passes including `tests/diff_test.rs`; manual smoke shows `+` and `-` lines with syntax-highlighted colouring.
**max_response_lines**: 320
**model_tier**: sonnet
**parallelisable**: no (writes)
**confidence_gate_passes**: yes — BP1-Q5 (gix read side) cleared 2026-04-28; `gix-diff` is the read-side diff crate consistent with the mixed gix/git2 decision.
**layer_scope**: task
**anti_slop_check**: Must name `src/diff.rs`, `tests/diff_test.rs`, `topic_diff` function signature with `sha_a` and `sha_b` parameters, and the specific route `GET /diff/{slug}?a=&b=` with short-sha resolution as concrete deliverables.
**dependencies**: Step 4.2 must be complete (4.2 established the gix read-side dependency pattern; 4.3 extends it with gix-diff; additionally the diff route is surfaced from the history page's commit links which 4.2 builds). Step 4.1 required (repository must contain commits to diff).

## Specification

Work on feature branch `cluster/project-knowledge`. Run `cargo check` and `cargo test` from inside `app-mediakit-knowledge/` — not from workspace root.

### New module: `src/diff.rs`

This module is pure read (gix-side) consistent with the BP1-Q5 mixed strategy.

Implement:
```rust
pub fn topic_diff(
    content_dir: &Path,
    slug: &str,
    sha_a: &str,
    sha_b: &str,
) -> Result<String, WikiError>
```
- Opens the `gix` repository at `content_dir`.
- Resolves `sha_a` and `sha_b` to full OIDs (accept short shas — 7+ chars — and resolve to full). If a sha does not resolve, return `WikiError` with a meaningful message.
- Uses `gix-diff` to produce a unified-diff string between the two tree entries for `<slug>.md`.
- Returns the unified diff as a plain UTF-8 string (not yet HTML — the route handler renders it as HTML with syntax colouring).
- If `<slug>.md` is not present in one tree (e.g., diff against the initial empty state), model the missing side as empty content.

The function does NOT need to handle binary diffs — TOPIC files are always UTF-8 Markdown.

Note on `gix-diff` version: use whatever is bundled with the `gix = "0.66"` (or current) dependency tree; check if `gix-diff` is already a transitive dep of `gix` before adding it separately to Cargo.toml.

### Route handler

In `src/server.rs`, mount:

**`GET /diff/{slug}?a=<sha>&b=<sha>`** — HTML page.

Query parameters `a` and `b` are commit shas (full or short). The route:
1. Validates that both `a` and `b` are present; returns 400 if either is missing.
2. Calls `diff::topic_diff(content_dir, slug, a, b)`.
3. Renders the unified-diff string as an HTML page: lines beginning with `+` are highlighted green (`<span class="diff-add">`), lines beginning with `-` are highlighted red (`<span class="diff-del">`), context lines are unstyled. The `@@` hunk headers are rendered in a muted style.
4. Wraps the result in the existing page chrome (sidebar, header) from the render module.
5. Returns 404 if the slug does not correspond to an existing TOPIC.
6. Returns 400 (with explanation) if either sha cannot be resolved.

Add the minimal CSS classes (`diff-add`, `diff-del`, `diff-hunk`) to `static/style.css` — no new stylesheet needed.

### Modify: `src/lib.rs`

Add `pub mod diff;` to the module declarations.

### `Cargo.toml`

Add to `[dependencies]` if `gix-diff` is not already a transitive dep via `gix`:
```toml
gix-diff = { version = "...", features = [...] }  # verify at implementation time
```
Confirm with `cargo tree | grep gix-diff` after adding `gix`. If it is already transitive, a direct dep may still be needed to access its public API directly — add with `workspace = false` and pin to the same version as gix's transitive.

### New file: `tests/diff_test.rs`

Integration test suite covering:

1. **Two-revision diff**: seed a TOPIC with a known initial body; edit it with a known change; capture the two commit shas from the git log; assert `GET /diff/{slug}?a=<sha1>&b=<sha2>` returns HTTP 200 with HTML containing lines beginning with `+` and `-` that reflect the known change.
2. **Reversed diff**: assert `GET /diff/{slug}?a=<sha2>&b=<sha1>` returns the inverse of the above (what were `+` lines are now `-` lines).
3. **Missing sha**: assert `GET /diff/{slug}?a=deadbeef&b=deadbeef` returns HTTP 400.
4. **Missing query params**: assert `GET /diff/{slug}` (no `a` or `b`) returns HTTP 400.
5. **Unknown slug**: assert `GET /diff/nonexistent?a=abc&b=def` returns HTTP 404.

Tests use `tempdir`-backed AppState consistent with Step 4.1/4.2 pattern.

### `cargo check` discipline

Run from inside `app-mediakit-knowledge/`. Confirm `cargo check` passes before `cargo test`. Pay attention to any API version mismatches between `gix-diff` and `gix` — they share the same release cadence and versions should align.

## Acceptance criteria

- `cargo test` (from `app-mediakit-knowledge/`) passes including all tests in `tests/diff_test.rs`.
- Manual smoke: GET /diff/{slug}?a=&b= on two real commit shas shows `+` lines highlighted green and `-` lines highlighted red.
- `src/diff.rs` exports `topic_diff` with the exact signature specified.
- The route returns 400 for missing/invalid shas and 404 for unknown slugs.
- Short sha resolution (7-char prefix) works correctly — the history page links use short shas.
- CSS classes `diff-add`, `diff-del`, `diff-hunk` exist in `static/style.css`.
- No `[PENDING-*]` tokens in produced files.
- Commit lands on `cluster/project-knowledge` branch, not `main`.

## Risks

- `gix-diff` API surface is evolving; consult the crate's current docs at implementation time rather than relying on the plan's phrasing. The abstraction in `topic_diff` isolates the API surface to one function.
- Short sha resolution: `gix` provides `repo.find_object(sha)` with partial-hash support; verify the exact method call at implementation time.
- The history page (Step 4.2) links to `/diff/{slug}?b=<sha>&a=<sha>~` — the `<sha>~` notation (parent commit) may need special handling in the route. If `gix` supports `sha~` parent syntax, use it; if not, resolve it manually to the parent commit OID. Document the resolution path in a code comment.
