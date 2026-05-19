# Brief 4.7 — read-only Git remote (smart-HTTP)

**target**: Implement `src/git_protocol.rs` with the git smart-HTTP upload-pack flow; mount `GET /git-server/{tenant}/info/refs` and `POST /git-server/{tenant}/git-upload-pack` in `src/server.rs`; add `--git-tenant <name>` CLI flag (default `pointsav`) in `src/main.rs`; add integration tests in `tests/git_protocol_test.rs`.
**target_files**:
- `app-mediakit-knowledge/src/git_protocol.rs` (new)
- `app-mediakit-knowledge/tests/git_protocol_test.rs` (new)
- `app-mediakit-knowledge/src/server.rs` (modify — mount two git smart-HTTP routes)
- `app-mediakit-knowledge/src/main.rs` (modify — add `--git-tenant <name>` CLI flag)
- `app-mediakit-knowledge/src/lib.rs` (modify — add `pub mod git_protocol`)
- `app-mediakit-knowledge/Cargo.toml` (modify — add any required deps for git smart-HTTP; verify if git2 provides server primitives or a separate crate is needed)
**expected_output**: `cargo test` passes including `tests/git_protocol_test.rs`; manual `git clone http://localhost:9090/git-server/pointsav.git /tmp/clone` succeeds and the cloned content matches what is on disk.
**max_response_lines**: 360
**model_tier**: sonnet
**parallelisable**: no (writes)
**confidence_gate_passes**: yes — BP1-Q2 (smart-HTTP via axum server) cleared 2026-04-28; no blocking unknowns.
**layer_scope**: task
**anti_slop_check**: Must name `src/git_protocol.rs`, `tests/git_protocol_test.rs`, the two specific routes `GET /git-server/{tenant}/info/refs` and `POST /git-server/{tenant}/git-upload-pack`, `--git-tenant` CLI flag with default `pointsav`, and the git smart-HTTP protocol (not git daemon TCP) as concrete deliverables.
**dependencies**: Step 4.1 must be complete (the git repository populated by 4.1 is what the remote serves; `AppState.git` and `content_dir` are the data source). Steps 4.2, 4.3, 4.4, 4.5, and 4.6 are not required — 4.7 is parallel with the history/diff and MCP branches after 4.1 completes.

## Specification

Work on feature branch `cluster/project-knowledge`. Run `cargo check` and `cargo test` from inside `app-mediakit-knowledge/` — not from workspace root.

### Protocol choice (BP1-Q2 confirmed)

smart-HTTP via the same axum server. The git smart-HTTP protocol serves two endpoints:
- `GET /git-server/{tenant}/info/refs?service=git-upload-pack` — advertises refs (branches, tags)
- `POST /git-server/{tenant}/git-upload-pack` — pack negotiation + pack delivery

This is a **read-only** remote: only `git-upload-pack` (clone/fetch) is served. `git-receive-pack` (push) is explicitly not implemented in Phase 4. Writes go through the auth'd HTTP edit surface (Phase 5).

### New module: `src/git_protocol.rs`

#### Implementation strategy

The git smart-HTTP protocol is a binary protocol layered over HTTP. Two implementation paths are available:

**Path A — use `git2` or `gix` server primitives**: Both crates expose server-side primitives for upload-pack. Check if `git2::Repository::uploadpack` or `gix`'s server primitives exist and are stable at implementation time. If available, use them.

**Path B — shell out to `git upload-pack`**: If no suitable Rust-native API is available, implement the route handlers to spawn `git upload-pack --stateless-rpc <content_dir>` as a subprocess and pipe its stdin/stdout through the HTTP request/response body. This is the standard approach used by git hosting services before native Rust implementations matured. It requires `git` to be installed on the VM (it is, per workspace operational tooling).

Check Path A first. If the API is stable and well-documented in the version of git2/gix already in Cargo.toml, use it. If not, use Path B. Document the choice in a comment at the top of `src/git_protocol.rs`.

#### Handler shapes

```rust
/// GET /git-server/{tenant}/info/refs?service=git-upload-pack
pub async fn info_refs(
    State(state): State<AppState>,
    Path(tenant): Path<String>,
    Query(params): Query<InfoRefsQuery>,
) -> impl IntoResponse
```
Returns the git upload-pack advertisement. Content-Type: `application/x-git-upload-pack-advertisement`.
- Validates that `params.service == "git-upload-pack"`. Returns 400 if not.
- Validates that `tenant` matches the configured `--git-tenant` value. Returns 404 if not (prevents serving unexpected tenants).
- Returns the pkt-line encoded capabilities + refs listing.

```rust
/// POST /git-server/{tenant}/git-upload-pack
pub async fn upload_pack(
    State(state): State<AppState>,
    Path(tenant): Path<String>,
    body: Bytes,
) -> impl IntoResponse
```
Processes the upload-pack negotiation. Content-Type: `application/x-git-upload-pack-result`.
- Validates `tenant` as above.
- Processes the pkt-line encoded client request (wants/haves) and returns the pack data.
- This is pure read; do not accept `git-receive-pack` at any endpoint.

#### Security note

The git smart-HTTP endpoint is read-only and serves the same content that `GET /git/{slug}` already serves (raw TOPIC Markdown). No new information is disclosed. The tenant validation prevents serving repos for unexpected tenant names. Phase 5 can add bearer-token auth to the git remote if needed.

### Modify: `src/server.rs`

Mount the two routes. Both routes share `AppState` with the rest of the server — they read `content_dir` from state to locate the git repository.

The tenant path segment (`{tenant}`) allows a future multi-tenant git remote surface. In Phase 4, only the configured `--git-tenant` name is accepted; requests for other tenant names return 404.

### Modify: `src/main.rs`

Add CLI flag:
```
--git-tenant <name>    default pointsav; used in /git-server/{tenant}/... paths
```
This is always-on (no off switch like `--enable-mcp`). The git remote is always mounted; the tenant name controls which path prefix is served.

Store `git_tenant: String` in `AppState` or pass it through the router state. The route handlers read it to validate the `{tenant}` path segment.

### Modify: `src/lib.rs`

Add `pub mod git_protocol;` to the module declarations.

### `Cargo.toml`

If Path B (subprocess) is used: no new Cargo.toml deps required (the subprocess approach uses `std::process::Command`).

If Path A (git2/gix server primitives) is used: add any required feature flags or sub-crates.

Document the approach in a Cargo.toml comment.

### New file: `tests/git_protocol_test.rs`

Integration test covering:

1. **clone round-trip**: Start the test server with the content_dir containing at least one committed TOPIC (from the Step 4.1 integration). Call `git clone http://localhost:{port}/git-server/pointsav.git /tmp/test_clone_{uuid}`. Assert the clone succeeds (zero exit code). Assert the cloned directory contains the committed TOPIC file. Cleanup tempdir after test.
2. **info/refs validation**: GET /git-server/pointsav/info/refs?service=git-upload-pack returns HTTP 200 with Content-Type `application/x-git-upload-pack-advertisement`.
3. **wrong service rejected**: GET /git-server/pointsav/info/refs?service=git-receive-pack returns HTTP 400.
4. **unknown tenant rejected**: GET /git-server/wrong-tenant/info/refs?service=git-upload-pack returns HTTP 404.

The clone round-trip test requires `git` to be available in the test environment (it is, on the foundry-workspace VM). If running in CI without git, skip this test with `#[ignore]` and a comment explaining the requirement.

### `cargo check` discipline

Run from inside `app-mediakit-knowledge/`. If Path B (subprocess) is used, `cargo check` will pass without new deps. If Path A requires new deps, confirm `cargo check` passes before `cargo test`.

## Acceptance criteria

- `cargo test` (from `app-mediakit-knowledge/`) passes including all tests in `tests/git_protocol_test.rs`.
- Manual smoke: `git clone http://localhost:9090/git-server/pointsav.git /tmp/clone` succeeds and the cloned TOPIC files match the content_dir.
- GET /git-server/pointsav/info/refs returns the correct Content-Type and pkt-line data.
- POST /git-server/pointsav/git-upload-pack responds to a `git fetch` request.
- `--git-tenant` CLI flag is wired with default `pointsav`.
- Wrong tenant returns 404; wrong service returns 400.
- No push/receive-pack endpoint exists.
- Implementation approach (Path A or Path B) documented in a code comment.
- No `[PENDING-*]` tokens in produced files.
- Commit lands on `cluster/project-knowledge` branch, not `main`.

## Risks

- The git smart-HTTP protocol has subtle pkt-line encoding requirements; Path B (subprocess) is safer and battle-tested. Prefer Path B unless Path A has a clearly stable API.
- Path B requires `git` to be on `$PATH` at runtime. The foundry-workspace VM has `git` installed. The production deployment runbook (GUIDE in `woodfine-fleet-deployment`) should note this dependency. Add a comment in `src/git_protocol.rs` noting the runtime dependency.
- The integration test that runs `git clone` spawns a subprocess from the test process. Use a random port for the test server to avoid conflicts with the main server or other test runs. The `tokio::net::TcpListener::bind("127.0.0.1:0")` pattern assigns a random available port.
- Multi-tenant: the `{tenant}` path segment is validated against `--git-tenant`. Future multi-tenancy may need a list of allowed tenants. Phase 4 is single-tenant; document the extension point in a code comment.
