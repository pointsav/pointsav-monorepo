# NEXT.md — app-mediakit-knowledge

> Last updated: 2026-05-12

## Phase 4 — COMPLETE (Steps 4.1–4.8 all shipped)

All Phase 4 steps committed on `pointsav-monorepo` main branch. Stage 6
promotion pending (outbox message sent to Master). Release binary built.

| Step | State | Commit |
|---|---|---|
| 4.1 — git2 commit-on-edit | ✓ Shipped | `177813e` |
| 4.2 — /history + /blame | ✓ Shipped | `177813e` |
| 4.3 — /diff | ✓ Shipped | `177813e` |
| 4.4 — redb wikilink graph | ✓ Shipped | `177813e` |
| 4.5 — blake3 hashes | ✓ Shipped | `177813e` |
| 4.6 — MCP server (native, no vendor SDK) | ✓ Shipped | `055b2f8e` |
| 4.7 — git smart-HTTP remote | ✓ Shipped | pre-existing |
| 4.8 — OpenAPI 3.1 spec | ✓ Shipped | `c9db78da` |

**Notes on MCP implementation:** `rmcp` vendor SDK rejected per Doctrine claim #54
("We Own It"). Implemented natively in `src/mcp.rs` (~330 lines) using
`axum` + `serde_json`. Transport: HTTP JSON-RPC 2.0 (standard; no stdio/SSE split
needed). Default off behind `--enable-mcp` / `WIKI_ENABLE_MCP`.

## Open: activation defect (now closed)

CLAUDE.md + NEXT.md were missing (noted in registry since 2026-04-28). Added 2026-05-07 — defect closed.

## Open: README.es.md out of sync

`README.es.md` is a 4-file scaffold stub; the English README is 8 KB. Refresh pass needed before next public-facing milestone.

## Open: site_title + guide_dir_2 config (production)

`local-knowledge-documentation.service` now supports `--site-title` and `--guide-dir-2` (shipped 2026-05-02). Verify both are set correctly in the active systemd unit on the workspace VM.

## Open: Step 7 collab smoke verification

Manual two-client collab smoke (two editors on the same TOPIC, cursor sync visible) is needed before marking Phase 2 Step 7 fully ratified. See `docs/STEP-7-COLLAB-SMOKE.md`.

## Closed: feeds.rs recursive walk

`collect_recent_items()` already implements a two-level walk (root + one category level)
matching the pattern in `collect_topic_files()`. Subdirectory TOPIC coverage verified by
`feeds_include_subdirectory_topics` test added 2026-05-12. NEXT.md note was stale.

## Deferred / operator-gated

- Phase 5-9 implementation — each gated on the preceding phase shipping + operator clearance
- Note: `libssl-dev` and `libgit2-dev` confirmed present on VM (Phase 4 release build succeeded)
