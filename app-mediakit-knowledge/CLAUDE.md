# CLAUDE.md — app-mediakit-knowledge

> **State:** Active  —  **Last updated:** 2026-05-12
> **Registry row:** `pointsav-monorepo/.agent/rules/project-registry.md`
>
> When state changes, update this header AND the registry row in the
> same commit. Drift between the two is a documentation defect.

---

## What this project is

`app-mediakit-knowledge` is the Wikipedia-pattern HTTP knowledge wiki
for `os-mediakit`. Serves `content-wiki-documentation` as a fully
navigable wiki at `documentation.pointsav.com`. Single Rust binary;
optional SQLite auth DB (bundled; no runtime system dependencies).
Substrate substitution for MediaWiki per Doctrine claim #29.

## Current state

**Active.** Phases 1, 1.1, 2 (Steps 1-7), 3 (Steps 3.1-3.4), 4
(Steps 4.1-4.8), and Phase 5 core shipped. Phase 5.1+ (per-page ACLs,
OIDC SSO, webhooks) deferred pending BP5 clearance.

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
| 4 — Git sync + MCP | Shipped (Steps 4.1-4.8) | git2, history/blame/diff, redb wikilink graph, blake3, MCP native JSON-RPC 2.0, git smart-HTTP, OpenAPI 3.1 |
| 5 — auth + edit review | Phase 5 core shipped; 5.1+ deferred | Cookie sessions, argon2id, edit review queue; ACLs/SSO/webhooks gated on BP5 |
| 6-9 | Designed | See `ARCHITECTURE.md` §0 status snapshot |

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
├── openapi.yaml           OpenAPI 3.1 spec (751 lines)
├── src/
│   ├── server.rs          main HTTP handler, routing, AppState
│   ├── auth.rs            Phase 5: cookie sessions, auth extractors
│   ├── pending.rs         Phase 5: edit review queue
│   ├── users.rs           Phase 5: SQLite schema, argon2id
│   ├── mcp.rs             Phase 4.6: MCP JSON-RPC 2.0 server
│   └── git_protocol.rs    Phase 4.7: read-only git smart-HTTP
└── docs/
    └── STEP-7-COLLAB-SMOKE.md  Phase 2 Step 7 smoke runbook
```

## Inherited rules — do not duplicate, do not silently override

- **Workspace-level:** `~/Foundry/CLAUDE.md`
- **Repo-level:** `pointsav-monorepo/.agent/rules/`

If a rule at this level conflicts with an inherited rule, stop and
surface the conflict.
