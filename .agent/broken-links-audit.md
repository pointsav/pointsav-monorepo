# Broken Links Audit — app-mediakit-knowledge

Living document. Updated when stubs are wired or new placeholders are added.
Last audited: 2026-05-07 (Sprint F + Stage 6 complete).

---

## Summary

The engine's route coverage is more complete than the Phase 1.1 file header comment
suggested. The comment at `src/server.rs` line 6 ("Edit + View-history are href='#'
placeholders") is stale — both have been wired since Phase 2 and Phase 4.

As of 2026-05-07 there are **zero `href="#"` stubs** in article rendered HTML.
Remaining non-functional surfaces are infrastructure placeholders gated on later phases.

---

## Route audit

### Fully implemented routes

| Route | Handler | Notes |
|---|---|---|
| `/` | `index` | Home page |
| `/wiki/{*slug}` | `wiki_page` | Article read |
| `/edit/{slug}` | `edit::get_edit` + `edit::post_edit` | CodeMirror editor; Phase 2 |
| `/create` | `edit::post_create` | Create new article |
| `/history/{*slug}` | `history_page` | Git-backed revision history |
| `/blame/{*slug}` | `blame_page` | Line-level blame |
| `/diff/{*slug}` | `diff_page` | Between-revision diff |
| `/talk/{*slug}` | `talk_page` + `talk_post` | Discussion pages (reads talk-files) |
| `/random` | `random_page` | Random article redirect |
| `/wanted` | `wanted_page` | Wanted articles list |
| `/search` | `search_page` | Full-text search (Tantivy BM25) |
| `/category/{name}` | `category_page` | Articles by category |
| `/special/all-pages` | `all_pages_page` | All articles index |
| `/special/recent-changes` | `recent_changes_page` | Recent changes feed |
| `/special/statistics` | `statistics_page` | Wiki stats |
| `/special/whatlinkshere/{slug}` | `what_links_here` | Backlinks |
| `/special/pageinfo/{slug}` | `page_info` | Article metadata |
| `/special/cite/{slug}` | `cite_page` | Citation export (BibTeX etc.) |
| `/special/pending-changes` | `pending::review_queue` | Edit review queue (admin) |
| `/special/login` | `auth::get_login` + `auth::post_login` | Auth |
| `/special/logout` | `auth::post_logout` | Auth |
| `/special/create-account` | `auth::get_create_account` + `auth::post_create_account` | Auth |
| `/git/{slug}` | `git_markdown` | Raw Markdown source |
| `/feed.atom` | `feeds::get_atom` | Atom feed |
| `/feed.json` | `feeds::get_json_feed` | JSON Feed |
| `/sitemap.xml` | `sitemap_xml` | Sitemap |
| `/robots.txt` | `robots_txt` | Robots |
| `/llms.txt` | `llms_txt` | LLM discovery |
| `/api/citations` | `citations::get_citations` | Citation registry |
| `/api/complete` | `search_complete` | Search autocomplete |
| `/api/preview/{*slug}` | `preview_api` | Hover card preview |
| `/mcp` | `mcp_handler` | MCP protocol endpoint |

---

## Stub / placeholder surfaces

### Phase 4 — Doorman integration (501 stubs)

| Endpoint | Handler | Current response | Phase target |
|---|---|---|---|
| `POST /api/doorman/complete` | `doorman_complete` | 501 Not Implemented | Phase 4 |
| `POST /api/doorman/instruct` | `doorman_instruct` | 501 Not Implemented | Phase 4 |

The client-side three-keystroke ladder (Tab / Cmd-K / Cmd-Enter in the editor) is wired
to these endpoints. Currently returns a JSON 501 body explaining the state.

### Phase 7 — IVC / Verification infrastructure

| UI element | Current state | Phase target |
|---|---|---|
| IVC masthead band (`div.wiki-ivc-band`) | Renders placeholder text: "Verification not yet available — Phase 7" | Phase 7 |
| Citation density toggle (Off / Exceptions / All) | Persists to localStorage; no Phase 7 machinery yet | Phase 7 |

### Phase 2 (optional, operator-gated) — Collab WebSocket

| Route | Handler | Current state | To enable |
|---|---|---|---|
| `GET /ws/collab/{slug}` | `collab::ws_collab` | Only mounted when `--enable-collab` flag is set | Run with `--enable-collab`; requires `WIKI_COLLAB_ENABLED=true` in unit |

---

## UX gaps (not broken links, but incomplete muscle memory)

These are not route issues — the routes work. The gap is in the rendered experience
relative to Wikipedia's muscle memory targets.

| Item | Current state | Sprint target |
|---|---|---|
| Left rail hidden on mobile | Left rail rendered below article at ≤768px | Sprint G |
| Sticky header on scroll | Not implemented | Sprint H |
| Active ToC section tracking | Not implemented | Sprint I |
| Mobile collapsible h2 sections | Not implemented | Sprint J |
| ToC content in mobile hamburger drawer | Not implemented | Sprint K |

See `.agent/drafts-outbound/research-wikipedia-toolbar-mobile.draft.md` for implementation
detail on all five items.

---

## How to re-audit

```bash
# Check for href="#" stubs in server.rs (excluding comment lines and anchor fragment links)
grep -n 'href="#"' \
  /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/src/server.rs

# Check which routes are registered
grep -n '\.route(' \
  /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/src/server.rs \
  | head -50

# Check for 501 stub handlers
grep -n '501\|StatusCode::NOT_IMPLEMENTED' \
  /srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/src/server.rs
```
