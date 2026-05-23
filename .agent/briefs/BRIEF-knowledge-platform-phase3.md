---
artifact: brief
status: active
topic: knowledge-platform Phases 1–5 complete; Phase 6 gated on GitHub rename + Doctrine amendment
archive: project-knowledge
created: 2026-05-22
updated: 2026-05-23
owner: totebox@project-knowledge
---

# BRIEF — Knowledge Platform Phases 1–5 complete

## Mission

Implement Phase 3 of `.agent/plans/KNOWLEDGE-PLATFORM-PLAN.md` §5 — the
claim-layer engine ("the core leapfrog build", Vision §9). The authoring
surface is frozen by `~/Foundry/conventions/claim-authoring-convention.md`
(doctrine claim #54, ratified 2026-05-21). The engine implements the
*extraction and use* of those claims.

## Working context

- **Working branch:** `pointsav-monorepo` `main`. The old `cluster/project-knowledge`
  branch was a stale relic — Command deleted it; `main` is confirmed canonical
  for project-knowledge engine work.
- **Commit path:** `~/Foundry/bin/commit-as-next.sh "<msg>"` from the monorepo
  sub-clone. Each commit must compile (`cargo check --tests`) + pass its
  module tests before committing.
- **Crate:** `pointsav-monorepo/app-mediakit-knowledge/`.

## Done — Phase 4 DTCG token wiring, Commits F–H (2026-05-22)

All on monorepo `main`, all tested green:

- **F — `bce932b1`** (§4.2) — `scripts/dtcg-bundle.json` (vendored canonical DTCG bundle
  from project-design, `knowledge.*` semantic namespace, 148 tokens); `scripts/dtcg-to-css.py`
  (hex→oklch converter + flattener + alias resolver); generates `static/tokens.css` (linked
  first in `<head>` so style.css can override).
- **G — `1ddfca98`** (§4.3+4.4) — `style.css` `:root` reconciled to DTCG semantic var aliases
  (`--bg → var(--surface-background)`, etc.); `static/tokens-woodfine.css` (full Woodfine
  brand override layer, all colors oklch); conditional `<link>` in chrome functions for
  `WIKI_BRAND_THEME=woodfine`.
- **H — `8406001a`** (§4.5) — WCAG 4.5:1 audit: 10 pairs pass, 2 fail (text.tertiary +
  knowledge.editpencil at #878d99, 3.08–3.33:1); both decorative roles, pass 3:1 non-text
  threshold; fix flagged to project-design via outbox.

## Done — Phase 3 Commits A–E (2026-05-22)

All on monorepo `main`, all tested green (94 tests passing after E):

- **A — `7887f8ec`** (§3.1) — `src/claim.rs`: `Claim` struct, `Confidence`
  enum, the `<!--claim …-->` / `<!--/claim-->` marker parser
  (`extract_claims`). Engine Verification Gate discharged.
- **B — `c41bf85e`** (§3.2) — `src/citations.rs`: per-claim citation
  resolution — `resolve()` + `resolve_claim_cites()` → `ClaimCitations`.
- **C — `77e0d0a8`** (§3.3) — `src/links.rs`: `CLAIM_DEPS` redb table;
  `rebuild_claims_for_slug`, `claim_depends_on`, `claim_dependents`.
- **D — `dbd5d3fa`** (§3.5) — two-clock temporality: `line_start`/`line_end`
  on `Claim`, `blame_published_at()` in `history.rs`, `?asof=` past-revision
  view in `wiki_page`, asof notice banner.
- **E — `9bc39de4`** (§3.7+3.8) — JSON content-negotiation (`Accept:
  application/json` → `{frontmatter, body_md, blake3, revision_sha, backlinks,
  claims}`); JSON-LD enriched with `dateModified`, `description`, `version`,
  `keywords`, `citation`. `Frontmatter` derives `Serialize`.

## Deferred out of the A–E tranche

- **§3.4 continuous citation verification** — background re-fetch + re-hash
  of cited sources. Needs `reqwest` (removed Phase 1) + background scheduler.
  Own sub-project; schedule dedicated Phase 3.4 effort.
- **§3.6 claim-record MCP API** (`query_claims(topic, asof)`) — cross-cluster:
  must reconcile with `service-slm`'s `slm-mcp-server`. Outbox sent to
  project-intelligence 2026-05-22
  (`project-knowledge-20260522-mcp-claims-reconcile`); 3.6 waits on reply.

## Key files

| File | Role |
|---|---|
| `src/claim.rs` | claim model + extractor + line tracking (Phase 3 A, D) |
| `src/citations.rs` | per-claim citation resolution (Phase 3 B) |
| `src/links.rs` | `CLAIM_DEPS` claim graph (Phase 3 C) |
| `src/history.rs` | `blame_published_at()`, `get_file_at_rev()` (Phase 3 D) |
| `src/server.rs` | `?asof=` past-revision view + JSON content-negotiation (Phase 3 D, E) |
| `src/jsonld.rs` | JSON-LD enrichment (Phase 3 E) |
| `src/render.rs` | `Frontmatter` derives `Serialize` (Phase 3 E) |
| `scripts/dtcg-bundle.json` | vendored canonical DTCG token bundle (Phase 4 F) |
| `scripts/dtcg-to-css.py` | DTCG → oklch CSS generator (Phase 4 F) |
| `static/tokens.css` | generated token CSS, 148 tokens (Phase 4 F) |
| `static/style.css` | `:root` reconciled to DTCG var aliases (Phase 4 G) |
| `static/tokens-woodfine.css` | Woodfine brand override layer (Phase 4 G) |

## Done — KNOWLEDGE-PLATFORM-PLAN.md Phases 1+5 + pre-build polish (2026-05-22–23)

All on monorepo `main`, full test suite green, `cargo clippy -D warnings` clean:

- **K — `11d482f2`** (Phase 1 PLAN) — crate hygiene: `cargo fmt` + `clippy -D warnings` 24 fixes across 9 files; `RATIFIED_CATEGORIES` → 12 items (added "company" + "help").
- **L — `6180b074`** (Phase 1 PLAN) — docs accuracy: `CLAUDE.md` + `ARCHITECTURE.md` updated (collab removed, Phase 5 shipped, new PLAN phases 1/3/4/5 documented).
- **M — `f2808e57`** — NEXT.md bookkeeping (Stage 6 count → 13).
- **I/J — `98642afb` + `76b501ff`** (Phase 5 PLAN) — bilingual `/es/` routing; 8 integration tests.
- **N — `826d42a5`** — `openapi.yaml` accuracy pass: 15 missing routes added (Phase 5 /es/, auth/pending, `/api/complete`, `/api/preview`, `/category`, `/talk`); category enum corrected; collab flag removed.
- **O — `c2d4010c`** — Accept-Language → `/es/` auto-redirect; `?noredirect=1` suppression; ES home lang-toggle href updated; 4 tests.
- **P — `7a7beb46`** — README.md + README.es.md: Phase 2 collab removed, Phase 5.1 bilingual marked shipped, missing `<div>` fixed.
- **Q — `09992b05`** — NEXT.md bookkeeping (Stage 6 count → 16).

## Stage 6 status

**16 commits unpromoted on monorepo `main`** — ready for tonight's build.
Phase 1 ×4, Phase 3 A–E ×5, Phase 4 F–H ×3, Phase 5 I–J ×2, hygiene K–L ×2, openapi N, Accept-Language O, README P, NEXT Q.
Stage 6 (`~/Foundry/bin/promote.sh`) + binary rebuild + `sudo systemctl restart` all 3 services — Command scope.

## Next phase

**Phase 6 — three-instance deployment split.** Gated on:
1. `content-wiki-*` → `media-knowledge-*` GitHub rename (operator doing manually).
2. MASTER Doctrine amendment for source-of-truth inversion (content repos canonical; GitHub downstream).
Neither gate is Totebox scope. No Totebox work until Command confirms both are clear.
