# CLAUDE.md — app-mediakit-knowledge

> **State:** Active  —  **Last updated:** 2026-05-07
> **Registry row:** `pointsav-monorepo/.agent/rules/project-registry.md`
>
> When state changes, update this header AND the registry row in the
> same commit. Drift between the two is a documentation defect.

---

## What this project is

`app-mediakit-knowledge` is the Wikipedia-pattern HTTP knowledge wiki
for `os-mediakit`. Serves `content-wiki-documentation` as a fully
navigable wiki at `documentation.pointsav.com`. Single Rust binary;
no database; no runtime dependencies beyond the compiled binary.

Substrate substitution for MediaWiki per Doctrine claim #29.

## Current state

**Active.** Phases 1, 1.1, 2 (Steps 1-7), 3 (Steps 3.1-3.4) shipped
and promoted to canonical. Phase 4 plan complete (BP1 decision packet
ready); implementation gated on operator BP1 clearance.

Running in production at `documentation.pointsav.com` via
`local-knowledge-documentation.service` (port 9090) and
`local-knowledge-projects.service` (port 9093).

## Build and test

```
cd app-mediakit-knowledge
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
```

Run locally:
```
cargo run -- serve --content-dir <path-to-content-wiki-documentation>
```

## Phase shipping status

| Phase | State | Notes |
|---|---|---|
| 1 — render | Shipped | Route `/wiki/{slug}`, `static/`, `/healthz` |
| 1.1 — Wikipedia chrome | Shipped | Article/Talk/History tabs, TOC, hatnote, language switcher, footer |
| 2 — edit + collab | Shipped (Steps 1-7) | JSON-LD, atomic edit, CodeMirror 6, SAA squiggles, citation autocomplete, collab via yjs |
| 3 — search + feeds | Shipped (Steps 3.1-3.4) | Tantivy BM25, `/feed.atom`, `/feed.json`, `/sitemap.xml`, `/robots.txt`, `/llms.txt`, `/git/{slug}` |
| 4 — Git sync + MCP | Designed; BP1 gated | `docs/PHASE-4-PLAN.md` + `docs/BP1-DECISION-PACKET.md` |
| 5-9 | Designed | See `ARCHITECTURE.md` §0 status snapshot |

## Hard constraints

- **Markdown files in a Git tree are the source of truth.** No schema
  migration ladder. Databases/indexes are derived state only.
- **Substrate-native API surfaces only** per `disclosure-substrate.md`
  §5.1 (no MediaWiki action-API shim).
- **BCSC disclosure posture** on all public-facing content. No
  Sovereign Data Foundation current-tense language. See
  `conventions/bcsc-disclosure-posture.md`.
- **Phase boundaries are scope boundaries.** No Phase N+1 features
  during Phase N implementation.

## Key files

```
app-mediakit-knowledge/
├── CLAUDE.md              this file
├── NEXT.md                open items
├── ARCHITECTURE.md        phase plan, status, conventions
├── docs/
│   ├── BP1-DECISION-PACKET.md   Phase 4 operator review (~15 min)
│   ├── PHASE-4-PLAN.md          8-step Phase 4 design
│   └── STEP-7-COLLAB-SMOKE.md  Phase 2 Step 7 smoke runbook
└── src/
    └── server.rs          main HTTP handler
```

## Inherited rules — do not duplicate, do not silently override

- **Workspace-level:** `~/Foundry/CLAUDE.md`
- **Repo-level:** `pointsav-monorepo/.agent/rules/`

If a rule at this level conflicts with an inherited rule, stop and
surface the conflict.
