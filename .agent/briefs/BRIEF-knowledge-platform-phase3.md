---
artifact: brief
status: active
topic: knowledge-platform Phase 3+4 — claim-layer + DTCG token wiring
archive: project-knowledge
created: 2026-05-22
updated: 2026-05-22
owner: totebox@project-knowledge
---

# BRIEF — Knowledge Platform Phase 3+4 (claim-layer + DTCG token wiring)

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

## Stage 6 status

**12 commits unpromoted on monorepo `main`** — Phase 1 (`8f51ddfc`, `959f8e6f`,
`bf35f38d`, `3d9cd9ec`) + Phase 3 A–E (`7887f8ec`, `c41bf85e`, `77e0d0a8`,
`dbd5d3fa`, `9bc39de4`) + Phase 4 F–H (`bce932b1`, `1ddfca98`, `8406001a`).
Stage 6 promotion + binary rebuild are Command scope; outbox notified.

## Next phase

**Phase 5 — Bilingual /es/ routing.** Self-contained: detect `Accept-Language: es`
+ `/es/{slug}` URL prefix; serve `{slug}.es.md` if present, else fall through to
English. No cross-cluster dependency. Next in plan order.
