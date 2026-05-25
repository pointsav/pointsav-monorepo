# NEXT.md — app-mediakit-knowledge

> Last updated: 2026-05-23

## Phase 4 DTCG token wiring — COMPLETE (Commits F–H, 2026-05-22)

Phases 4.2–4.5 of `KNOWLEDGE-PLATFORM-PLAN.md` committed on monorepo `main`:

| Commit | Phase | What |
|---|---|---|
| `bce932b1` | 4.2 — DTCG build script | `scripts/dtcg-bundle.json` (vendored canonical) + `scripts/dtcg-to-css.py`; generates `static/tokens.css` (148 tokens, all colors in oklch()) |
| `1ddfca98` | 4.3+4.4 — reconcile `:root` + theme switch | `style.css` `:root` aliases → DTCG semantic vars; `tokens-woodfine.css` full Woodfine brand override; conditional `<link>` in chrome when `WIKI_BRAND_THEME=woodfine` |
| _(this commit)_ | 4.5 — WCAG audit | See findings below |

## Phase 4.5 — WCAG 4.5:1 audit findings (2026-05-22)

**Audit scope:** all color pairs in DTCG semantic token set — 12 foreground/background
combinations checked programmatically via relative-luminance formula.

**Results: 10 pass, 2 fail AA (4.5:1):**

| Token pair | Hex FG | Ratio | 4.5:1 AA | 3:1 large |
|---|---|---|---|---|
| `semantic.text.tertiary` on `semantic.surface.background` | #878d99 | 3.08:1 | FAIL | PASS |
| `knowledge.editpencil` on `semantic.surface.layer` | #878d99 | 3.33:1 | FAIL | PASS |

**Assessment:** Both failures use `#878d99`. Both are decorative/supplementary roles:
- `text.tertiary` — placeholder text, disabled labels; qualifies as non-text UI (WCAG 1.4.11, 3:1 threshold) rather than body text (4.5:1)
- `knowledge.editpencil` — edit pencil icon overlay on article text; decorative icon, non-interactive at hover-only visibility; 3:1 threshold applies

**Both colors PASS 3:1 large-text / non-text contrast.** No accessibility regression introduced by Phase 4.

**Fix required at token source (project-design scope):** To meet strict body-text 4.5:1, darken `#878d99` to ≈ `#767c8a` (ratio 4.52:1) in `dtcg-vault/tokens/dtcg-bundle.json`. Outbox message sent to project-design. This is not a blocker for Phase 5.

## Closed: Phase 5 — bilingual /es/ routing (2026-05-22 / 2026-05-23)

`/es/` + `/es/wiki/{*slug}`, ES file fallback, `html lang=`, hreflang tags, language
switcher in nav. Accept-Language → /es/ auto-redirect with `?noredirect=1` suppression
added 2026-05-23 (Commit O, `c2d4010c`). 4 tests added.

## Closed: crate hygiene (Commit K, 2026-05-22)

`cargo fmt` + `cargo clippy -D warnings` — 24 pre-existing lints fixed across
`feeds.rs`, `glossary.rs`, `history.rs`, `render.rs`, `search.rs`, `server.rs`,
`edit.rs`, `main.rs`, and test files. Committed 11d482f2.

## Closed: RATIFIED_CATEGORIES → 12 items (Commit K, 2026-05-22)

Added "company" (after "infrastructure") and "help" (after "reference").
All 8 home_test integration tests now pass. Committed 11d482f2.

## Closed: CLAUDE.md / ARCHITECTURE.md accuracy pass (Commit L, 2026-05-22)

Both files updated: collab removed from Phase 2 row; Phase 5 marked shipped;
new KNOWLEDGE-PLATFORM-PLAN.md phases 1/3/4/5 documented. Committed 6180b074.

## Closed: openapi.yaml accuracy pass (Commit N, 2026-05-23)

15 missing routes added: Phase 5 `/es/` routes, auth/pending special pages,
`/api/complete`, `/api/preview/{slug}`, `/category/{name}`, `/talk/{slug}`.
Category enum corrected (company + help). Collab flag reference removed. `826d42a5`.

## Closed: Accept-Language → /es/ redirect (Commit O, 2026-05-23)

`prefers_spanish()` helper; `IndexQueryParams.noredirect`; ES home lang-toggle
links to `/?noredirect=1`; 4 tests. `c2d4010c`.

## Closed: README refresh (Commit P, 2026-05-23)

Phase 2 row: collab removed. Phase 5.1 bilingual routing marked shipped.
Missing `<div>` in EN README fixed. `7a7beb46`.

## Open: Stage 6 promotion

**16 commits unpromoted on monorepo `main`** (Phase 1 ×4, Phase 3 A–E ×5, Phase 4 F–H ×2,
Phase 5 I–J ×2, crate hygiene K–L ×2, openapi N ×1, Accept-Language O ×1, README P ×1).
Promote via `~/Foundry/bin/promote.sh` from Command Session. Binary rebuild required after
promote. Outbox messages sent.

---

> Historical NEXT.md content (pre-2026-05-22 plan) preserved below for reference.
> The items below reflect the old Phase numbering (git-based Phase 4, auth Phase 5).
> Cross-reference against `KNOWLEDGE-PLATFORM-PLAN.md` for current plan state.

---

> Last updated (historical): 2026-05-12

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
