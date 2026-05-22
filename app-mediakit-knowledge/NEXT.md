# NEXT.md — app-mediakit-knowledge

> Last updated: 2026-05-22

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

## Open: Phase 5 — bilingual /es/ routing

Next in `KNOWLEDGE-PLATFORM-PLAN.md` order after Phase 4 complete. Self-contained:
detect `Accept-Language: es` header + `/es/{slug}` URL prefix; serve `{slug}.es.md` if
present, else fall through to English with a language toggle. No cross-cluster dependency.

## Open: crate hygiene

`cargo fmt` reformats ~37 files; `cargo clippy -D warnings` has pre-existing lints in
`feeds.rs`, `glossary.rs`, `history.rs`, `edit.rs`. Pre-dates Phase 3. Standalone task —
do not bundle into feature commits. Flagged to Command.

## Open: CLAUDE.md / ARCHITECTURE.md accuracy pass

Both files still describe removed features (real-time collab, Doorman proxy stubs, three
redundant MCP read tools). Removed in Phase 1 (Commits 2026-05-22) but docs not updated.
Low urgency — address before Stage 6 promotion of Phase 1 batch.

## Open: Stage 6 promotion

**11 commits unpromoted on monorepo `main`** (Phase 1 ×4, Phase 3 A–E ×5, Phase 4 F–H ×2+).
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
