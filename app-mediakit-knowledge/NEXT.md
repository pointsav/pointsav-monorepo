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

## Closed: site_title + guide_dir_2 config (production)

`local-knowledge-documentation.service` now supports `--site-title` and `--guide-dir-2` (shipped 2026-05-02). Verified 2026-05-14: `WIKI_SITE_TITLE=PointSav Documentation Wiki` and `WIKI_GUIDE_DIR_2=/srv/foundry/customer/woodfine-fleet-deployment` both set in the active unit. `local-knowledge-projects.service` and `local-knowledge-corporate.service` confirmed with correct per-instance titles; neither needs `WIKI_GUIDE_DIR_2`.

## Open: Step 7 collab smoke verification

Manual two-client collab smoke (two editors on the same TOPIC, cursor sync visible) is needed before marking Phase 2 Step 7 fully ratified. See `docs/STEP-7-COLLAB-SMOKE.md`.

## Closed: feeds.rs recursive walk

`collect_recent_items()` already implements a two-level walk (root + one category level)
matching the pattern in `collect_topic_files()`. Subdirectory TOPIC coverage verified by
`feeds_include_subdirectory_topics` test added 2026-05-12. NEXT.md note was stale.

## Phase 5 core — shipped

`src/auth.rs` (428 lines), `src/pending.rs` (505 lines), `src/users.rs` (186 lines) —
cookie sessions, argon2id passwords, edit review queue, accept/reject workflow.
Integration tests added 2026-05-12: `tests/auth_test.rs` (5 tests), `tests/pending_test.rs` (4 tests).

Phase 5.1+ not yet implemented: per-page ACLs (`read:`/`edit:` frontmatter), OIDC SSO,
webhook subscriptions, `asyncapi.yaml` 3.1 spec — gated on BP5.

## Phase 6 Part A — shipped (2026-05-13)

Three items implemented and tested:

1. **`inject_wiki_prefixes` trailing-quote fix** (`src/render.rs`) — `raw_slug` previously
   included the closing `"` of the `href` attribute, causing `is_redlink` to always return
   true and wikilink URLs to contain a trailing `"`. Fixed: `trim_end_matches('"')` + slug
   normalisation (decode `%20`, lowercase, spaces→hyphens).

2. **Slug normalisation fallback** (`src/server.rs`) — when a direct file lookup fails,
   tries the lowercase+hyphenated form and returns HTTP 301 to the canonical URL.
   e.g. `/wiki/Compounding-Substrate` → 301 → `/wiki/compounding-substrate`.

3. **Redirect hatnote** (`src/server.rs`, `static/style.css`) — `redirect_to:` 301 now
   includes `?redirectedfrom=<slug>`; `wiki_page` extracts it and passes to `wiki_chrome`;
   `wiki_chrome` renders `.wiki-redirected-from` hatnote at top of article body.

Tests: 4 new tests in `tests/slug_test.rs` — all pass. Full suite: 67 unit + 70+ integration,
all passing.

## Deferred / operator-gated

- Phase 5.1+ — per-page ACLs, OIDC SSO, webhooks, AsyncAPI 3.1 — gated on BP5 + Stage 6
- Phase 6 Part B — portable DID identity (`did:web:` + WebFinger) — needs BP6 design decision
- Phase 7-9 implementation — each gated on the preceding phase shipping + operator clearance
- Note: `libssl-dev` and `libgit2-dev` confirmed present on VM (Phase 4 release build succeeded)
- **Stage 6 + binary rebuild** — now 10 commits ahead of origin on `main`; requires Master session
  (`~/Foundry/bin/promote.sh` + `cargo build --release` + `sudo systemctl restart` all 3 services)
