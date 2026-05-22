---
artifact: brief
status: active
topic: knowledge-platform Phase 3 — claim-layer engine
archive: project-knowledge
created: 2026-05-22
updated: 2026-05-22
owner: totebox@project-knowledge
---

# BRIEF — Knowledge Platform Phase 3 (claim-layer engine)

> Pick-up brief. Tomorrow's session resumes at **Commit D**. Everything
> needed to continue is here.

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

## Done this session (2026-05-22) — Commits A, B, C

All on monorepo `main`, all tested green:

- **A — `7887f8ec`** (§3.1) — `src/claim.rs`: `Claim` struct, `Confidence`
  enum, the `<!--claim …-->` / `<!--/claim-->` marker parser
  (`extract_claims`). **Engine Verification Gate discharged** — `render.rs`
  test `claim_markers_pass_through_inert` proves comrak emits the markers
  inert. `claim.rs` is wired into `lib.rs`.
- **B — `c41bf85e`** (§3.2) — `src/citations.rs`: per-claim citation
  resolution — `resolve()` (id + alias match) and `resolve_claim_cites()`
  → `ClaimCitations { resolved, unresolved }`.
- **C — `77e0d0a8`** (§3.3) — `src/links.rs`: `CLAIM_DEPS` redb table;
  `rebuild_claims_for_slug`, `claim_depends_on`, `claim_dependents`
  (the `depends_on` / `cited_by` graph).

## NEXT — Commit D (start here tomorrow)

**§3.5 two-clock temporality.** `valid_at` (already parsed from the marker,
on `Claim`) + `published_at`.

- **`published_at` — OPERATOR DECISION: Option A (per-span git blame).**
  Compute each claim's `published_at` as the committer timestamp of the
  newest commit touching the claim's own line range — via `gix-blame`
  (already a Cargo dep; see also `src/history.rs` for existing gix/blame
  use). Map the claim's byte-span (the extractor has marker positions) →
  line range → blame → newest commit timestamp. NOT the coarse file-level
  last-commit (Option B was rejected).
- `?asof=<date|sha>` past-revision view on the `wiki_page` route — render a
  TOPIC as of a past git revision (touches `src/server.rs` `wiki_page` +
  git history retrieval).
- Engine produces the temporal data; the **freshness-ribbon visual** is a
  `project-design` component (`component-freshness-ribbon` in the cluster
  manifest) — implement a minimal engine-side render only, note the boundary.

## Then — Commit E

**§3.7** `GET /wiki/{slug}` JSON content-negotiation (`Accept:
application/json` → `{frontmatter, body_md, blake3, revision_sha,
backlinks}`) + **§3.8** enrich `src/jsonld.rs` (`dateModified`,
`description`, `citation` from resolved `cites:`, `version`, `keywords`).
Independent of claims; lower-risk.

## Deferred out of the A–E tranche

- **§3.4 continuous citation verification** — background re-fetch + re-hash
  of cited sources. Needs an HTTP client (`reqwest` was removed in Phase 1 —
  re-add if/when this is built), a background scheduler, drift detection.
  Its own sub-project — schedule a dedicated Phase 3.4 effort.
- **§3.6 claim-record MCP API** (`query_claims(topic, asof)`) — cross-cluster:
  must reconcile with `service-slm`'s `slm-mcp-server`, not duplicate it.
  Outbox sent to project-intelligence 2026-05-22
  (`project-knowledge-20260522-mcp-claims-reconcile`); 3.6 waits on the reply.

## Key files

| File | Role in Phase 3 |
|---|---|
| `src/claim.rs` | claim model + extractor (done; D wires extraction into the render path) |
| `src/citations.rs` | per-claim citation resolution (done) |
| `src/links.rs` | `CLAIM_DEPS` claim graph (done) |
| `src/render.rs` | gate test (done); claim extraction call-site for D |
| `src/history.rs` / `src/git.rs` | gix/git2 — prior art for D's per-span blame |
| `src/server.rs` | `wiki_page` handler — `?asof=` (D) + JSON content-negotiation (E) |
| `src/jsonld.rs` | JSON-LD enrichment (E) |

## Stage 6 status

7 commits unpromoted on monorepo `main` — Phase 1 (`8f51ddfc`, `959f8e6f`,
`bf35f38d`, `3d9cd9ec`) + Phase 3 A/B/C (`7887f8ec`, `c41bf85e`,
`77e0d0a8`). Stage 6 promotion + binary rebuild are Command scope; outbox
notified.
