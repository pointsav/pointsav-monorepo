# NEXT.md — app-mediakit-knowledge

> Last updated: 2026-05-07

## Phase 4 — Git sync + MCP server (operator-gated: BP1 clearance)

BP1 decision packet at `docs/BP1-DECISION-PACKET.md` (~15 min read). Seven open questions require operator answers before Phase 4 implementation begins:

1. MCP transport — stdio (Cursor/Claude Desktop) vs SSE (HTTP streaming)?
2. Git remote protocol — smart-HTTP read-only (`/git-http-backend/`) vs dumb-HTTP?
3. `--enable-mcp` default — on or off in production unit?
4. project-slm coordination order — Phase 4 Step 4.6 MCP wiring needs service-slm Tier C available?
5. `gix` vs `git2` split — Phase 4 Step 1 (commit-on-edit) uses git2; Step 2 (history/blame) could use gix. Keep one or split?
6. `libgit2-dev` system-lib install on workspace VM?
7. OpenAPI 3.1 spec — hand-author vs codegen from axum routes?

## Open: activation defect (now closed)

CLAUDE.md + NEXT.md were missing (noted in registry since 2026-04-28). Added 2026-05-07 — defect closed.

## Open: README.es.md out of sync

`README.es.md` is a 4-file scaffold stub; the English README is 8 KB. Refresh pass needed before next public-facing milestone.

## Open: site_title + guide_dir_2 config (production)

`local-knowledge-documentation.service` now supports `--site-title` and `--guide-dir-2` (shipped 2026-05-02). Verify both are set correctly in the active systemd unit on the workspace VM.

## Open: Step 7 collab smoke verification

Manual two-client collab smoke (two editors on the same TOPIC, cursor sync visible) is needed before marking Phase 2 Step 7 fully ratified. See `docs/STEP-7-COLLAB-SMOKE.md`.

## Open: feeds.rs flat walk

`feeds.rs` `collect_recent_items()` still uses a flat `read_dir()` — feeds only surface root-level TOPICs. Deferred from Wave 1 iteration-2. Fix: apply the same recursive-walk pattern as `collect_topic_files()`.

## Deferred / operator-gated

- `libssl-dev` on workspace VM (needed for `cargo build --release`)
- `libgit2-dev` (needed for Phase 4)
- Phase 5-9 implementation — each gated on the preceding phase shipping
