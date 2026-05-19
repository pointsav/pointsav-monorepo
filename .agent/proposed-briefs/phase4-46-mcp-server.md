# Brief 4.6 — MCP server (rmcp) — first-class agent surface

**SEQUENCING NOTE**: This brief is [outbox-first; implementation-gated-on-project-slm-reply]. The first deliverable is an outbox message to the project-slm Task cluster containing a draft MCP auth + rate-limit contract proposal. Implementation of `src/mcp.rs` and the `/mcp` route gates on project-slm's reply to that outbox. Do not begin the Rust implementation until the outbox reply is received and reviewed.

**target** (first deliverable — outbox message): Draft and write an MCP auth + rate-limit contract proposal to `/srv/foundry/clones/project-knowledge/.claude/outbox.md` addressed to the project-slm Task cluster. This is the sole output of the first dispatch of this brief.
**target** (second deliverable — after project-slm reply): Implement `src/mcp.rs` with the 6 tools, 3 prompts, and resource URI scheme; mount `POST /mcp` in `src/server.rs`; add `--enable-mcp` CLI flag (default off) in `src/main.rs`; add `rmcp` to `Cargo.toml`; add integration tests in `tests/mcp_test.rs`.
**target_files** (first dispatch — outbox only):
- `/srv/foundry/clones/project-knowledge/.claude/outbox.md` (append outbox message)
**target_files** (second dispatch — implementation):
- `app-mediakit-knowledge/src/mcp.rs` (new)
- `app-mediakit-knowledge/tests/mcp_test.rs` (new)
- `app-mediakit-knowledge/src/server.rs` (modify — mount `POST /mcp`; add to AppState if needed)
- `app-mediakit-knowledge/src/main.rs` (modify — add `--enable-mcp` CLI flag, default off)
- `app-mediakit-knowledge/src/lib.rs` (modify — add `pub mod mcp`)
- `app-mediakit-knowledge/Cargo.toml` (modify — add `rmcp`)
**expected_output** (first dispatch): Outbox message written with a concrete auth + rate-limit contract draft that project-slm Task can review and reply to. Message identifies: (a) bearer-token auth header shape, (b) per-tenant rate bucket proposal, (c) MCP protocol version to pin, (d) which tools should be accessible without auth (read-only) vs require auth (write). No Rust implementation in the first dispatch.
**expected_output** (second dispatch): `cargo test` passes including `tests/mcp_test.rs`; manual smoke with an MCP client (Claude Desktop config or `mcp inspect` CLI) shows all 6 tools and 3 prompts listed; `/mcp` route is not reachable unless `--enable-mcp` is passed.
**max_response_lines**: 420
**model_tier**: sonnet
**parallelisable**: no (writes)
**confidence_gate_passes**: yes — BP1-Q1 (HTTP on `/mcp`), Q3 (`--enable-mcp` default off), and Q4 (outbox-first) all cleared 2026-04-28.
**layer_scope**: task
**anti_slop_check**: First dispatch must produce a concrete outbox message naming the auth contract proposal (not vague "discuss auth"); second dispatch must name `src/mcp.rs`, `tests/mcp_test.rs`, `POST /mcp`, HTTP transport (not stdio), `--enable-mcp` default-off flag, `rmcp` crate, and all 6 tool names as concrete deliverables.
**dependencies**: Step 4.1 must be complete (AppState shape established; MCP tools reference `AppState.git`, `AppState.links`, `AppState.search`). Outbox-first sequencing (BP1-Q4): this brief has TWO dispatches separated by a project-slm reply cycle (~1 session round-trip).

## Specification

Work on feature branch `cluster/project-knowledge`. Run `cargo check` and `cargo test` from inside `app-mediakit-knowledge/` — not from workspace root.

---

### DISPATCH 1: Outbox contract draft

Before writing any implementation code, write the following outbox message to `/srv/foundry/clones/project-knowledge/.claude/outbox.md`:

```markdown
---
from: task-project-knowledge
to: task-project-slm
re: MCP auth + rate-limit contract proposal — Step 4.6 co-design (BP1-Q4)
created: <ISO 8601 timestamp>
---

## Context

`app-mediakit-knowledge` is shipping an MCP server (Step 4.6) behind
`POST /mcp` (HTTP transport, per BP1-Q1). Per ARCHITECTURE.md §3 Phase 4
and BP1-Q4 (operator cleared 2026-04-28), the MCP server must be co-designed
with the Doorman (`service-slm/router/`) for auth + rate-limit policy before
implementation begins. This message carries the draft contract for review.

## Draft contract — auth

Transport: HTTP/JSON-RPC on `POST /mcp` (same axum server as the wiki
routes; rides existing nginx/TLS termination).

Proposed auth model for Phase 4:
- **Read-only tools** (`search_topics`, `get_revision`, `list_backlinks`,
  `cite-this-page` prompt, `summarize-topic` prompt): accessible without
  auth in Phase 4 demo mode (`--enable-mcp` on, no session). Phase 5 wires
  per-session bearer tokens.
- **Write tools** (`create_topic`, `propose_edit`, `link_citation`,
  `draft-related-topic` prompt): Phase 4 demo mode is open (no auth gate
  while `--enable-mcp` is in demo mode). Phase 5 adds `Authorization:
  Bearer <token>` header check.
- **MCP auth header for Phase 5 contract**: `Authorization: Bearer <token>`
  where `<token>` is the session token from `tower-sessions` / `axum-login`
  (Phase 5). The Doorman should include this header when calling `/mcp` on
  behalf of a session.

## Draft contract — rate limits

Proposed for Phase 4 (open; no enforcement, just logging):
- Per-tenant rate bucket: 60 tool calls / minute / tenant (logged, not
  enforced in Phase 4).
- The `moduleId` header (consistent with `service-fs` Ring 1 convention
  for tenant namespacing): `X-Wiki-Tenant: <tenant>` — Doorman sets this
  when proxying. Phase 4 echoes it back in responses but does not enforce.

## Draft contract — MCP version

Pin to `rmcp` crate's current stable version. MCP spec version: 2024-11-05
(or whatever `rmcp` currently implements — confirm at implementation time).

## Questions for project-slm Task

1. Does the proposed auth model (read-open / write-open in demo mode,
   bearer token in Phase 5) match Doorman's expected client flow?
2. Does the `X-Wiki-Tenant` header approach conflict with any existing
   Doorman routing headers?
3. Any objection to the 60/min/tenant proposal, or a different number
   that matches service-slm's own rate-limit thinking?
4. Is there a preferred MCP spec version Doorman targets? (Doorman speaks
   OpenAI-compatible HTTP per ARCHITECTURE.md; need to confirm MCP spec
   version alignment.)

Reply to `/srv/foundry/clones/project-knowledge/.claude/inbox.md`.
Implementation of Step 4.6 gates on this reply.
```

Stop after writing the outbox message. Do not begin Rust implementation in this dispatch.

---

### DISPATCH 2: Implementation (after project-slm reply)

Read the project-slm reply from `/srv/foundry/clones/project-knowledge/.claude/inbox.md`. Incorporate any contract adjustments. Then implement:

#### `rmcp` crate version

Run `cargo search rmcp` to find the current version. The plan lists `"0.x"` as a placeholder. Use the current stable release. Document the version in `Cargo.toml` with a comment: `# rmcp <version> — MCP SDK; verify current at dispatch time`.

#### New module: `src/mcp.rs`

Implement the MCP server using `rmcp`. Mount it on `POST /mcp` via HTTP/JSON-RPC (BP1-Q1 decision — HTTP transport, not stdio).

**Resources**: `wiki://topic/{slug}` per TOPIC (per-page resource URI). The resource content is the full Markdown body of the TOPIC at HEAD.

**Tools** (6):
1. `search_topics(query: string, limit: int) -> Vec<{slug, title, score}>` — calls `AppState.search` tantivy index
2. `get_revision(slug: string, revision_sha?: string) -> {body, frontmatter, sha, timestamp}` — reads from content_dir; if revision_sha absent, returns HEAD; uses `gix` read path consistent with Step 4.2
3. `create_topic(title: string, body: string, slug?: string) -> {slug}` — calls the existing `post_create` logic; respect demo-mode auth (open in Phase 4)
4. `propose_edit(slug: string, body: string, message: string) -> {revision_sha}` — calls the existing `post_edit` logic; returns the git commit sha
5. `link_citation(slug: string, citation_id: string) -> {ok}` — reads the TOPIC frontmatter, appends to `cites:` list, writes back via `atomic_write`, commits via `git::commit_topic`
6. `list_backlinks(slug: string) -> [string]` — calls `links::backlinks`

**Prompts** (3):
1. `/cite-this-page` — summarises the current TOPIC + suggests citations from `~/Foundry/citations.yaml`; returns a prompt template string
2. `/summarize-topic` — short-form summary of a TOPIC; returns a prompt template string
3. `/draft-related-topic` — generates a draft TOPIC referencing the current one; returns a prompt template string

Prompts are returned as `rmcp::Prompt` objects with the template text. They do not execute inference — they return prompt scaffolds that the MCP client (Doorman or Claude Desktop) sends to its own LLM.

#### Modify: `src/server.rs`

Mount `POST /mcp` behind the `--enable-mcp` flag check. If the flag is false, the route returns 404 (or is not mounted at all — prefer not mounting so the route simply 404s cleanly).

The MCP handler receives the `rmcp` JSON-RPC request, routes to the appropriate tool/resource/prompt handler, and returns the `rmcp` JSON-RPC response.

#### Modify: `src/main.rs`

Add CLI flag:
```
--enable-mcp           default off; mounts POST /mcp; Phase 5 auth gates per-session
```
This matches the existing `--enable-collab` default-off pattern (Phase 2 Step 7). Pass the flag into `AppState` or thread it through the router builder. Do not mount the `/mcp` route unless the flag is set.

#### New file: `tests/mcp_test.rs`

Integration test suite covering:
1. **MCP handshake (initialize)**: POST JSON-RPC `initialize` to `/mcp`; assert response includes `serverInfo` with server name and the tools listing.
2. **`search_topics` round-trip**: create a TOPIC; call `search_topics` via MCP; assert the TOPIC appears in results.
3. **`get_revision` round-trip**: create a TOPIC; call `get_revision` with the slug; assert the returned body matches.
4. **`create_topic` round-trip**: call `create_topic` via MCP; assert the new TOPIC exists on disk.
5. **`propose_edit` round-trip**: create a TOPIC; call `propose_edit` via MCP; assert the edit landed in git log.
6. **Resource read**: send MCP resource read for `wiki://topic/{slug}`; assert the content matches the TOPIC body.
7. **`--enable-mcp` off**: start the server without the flag; assert POST `/mcp` returns 404.

### `cargo check` discipline

Run from inside `app-mediakit-knowledge/`. The `rmcp` crate version may pull additional async deps — confirm `cargo check` passes before `cargo test`.

## Acceptance criteria

**Dispatch 1:**
- Outbox message written to `/srv/foundry/clones/project-knowledge/.claude/outbox.md` with all 4 questions for project-slm.
- No Rust implementation files modified.

**Dispatch 2 (after project-slm reply):**
- `cargo test` (from `app-mediakit-knowledge/`) passes including all tests in `tests/mcp_test.rs`.
- Manual smoke: `mcp inspect` or Claude Desktop config shows all 6 tools and 3 prompts listed.
- `POST /mcp` returns 404 when `--enable-mcp` is not passed.
- `--enable-mcp` flag wired in `src/main.rs` with default off.
- `rmcp` crate version documented in `Cargo.toml` comment (not `"0.x"`).
- Auth contract from project-slm reply incorporated (document any deviation from the draft in a code comment in `src/mcp.rs`).
- No `[PENDING-*]` tokens in produced files.
- Commit lands on `cluster/project-knowledge` branch, not `main`.

## Risks

- `rmcp` crate may not exist under that name on crates.io at implementation time — the Anthropic MCP Rust SDK may have a different name. Run `cargo search rmcp` and `cargo search mcp` to find the current SDK. Document the crate name actually used.
- HTTP/JSON-RPC over axum: the `rmcp` crate may provide a higher-level integration or require manual JSON-RPC dispatch. Check the crate docs at implementation time; do not assume the axum integration API matches what the plan sketches.
- The outbox-first cycle takes approximately 1 session round-trip. Brief 4.7 (read-only Git remote) and Brief 4.8 (OpenAPI) can be dispatched in parallel with the outbox wait — they do not depend on the MCP contract.
- Phase 4 demo mode is deliberately open (no auth on write tools). Document this explicitly in a `// PHASE 4 DEMO MODE` comment in `src/mcp.rs` so Phase 5 implementers know to add auth here.
