# Implementation Plan — Wikipedia Parity + Leapfrog 2030
# project-knowledge Task Claude — auto-edit to-do list

Created: 2026-05-07 by task@project-knowledge
Based on: 3-agent deep research session (MediaWiki architecture + codebase gap analysis + Rust ecosystem)
Current state: ~78% Wikipedia muscle memory
Target: ~95% Wikipedia parity + Leapfrog 2030 layer

Reference: see `drafts-outbound/guide-knowledge-wiki-sprint-roadmap.draft.md` for full implementation details of each item.
All code changes in: `pointsav-monorepo/app-mediakit-knowledge/`
Commit each sprint as a single `bin/commit-as-next.sh` commit after `cargo test` passes.

---

## SPRINT A — Quick wins (no new dependencies, no architecture changes) ✅ COMPLETE — commit 1093186

- [x] **A1** Footnote CSS — added to `static/style.css`; blue `[1]` Wikipedia-style superscripts
- [x] **A2** Footnote hover tooltip — `initFootnoteTooltips()` in `static/wiki.js` (~50 lines JS); tooltip CSS added
- [x] **A3** Enable description lists — `options.extension.description_lists = true` in `src/render.rs` (field name in comrak 0.29 is `description_lists` not `definition_lists`)
- [x] **A4** `/random` route — handler in `src/server.rs`; "Random article" link in left-rail toolbox + mobile nav
- [x] **A5** Redirects — `redirect_to: Option<String>` in `Frontmatter` + 301 redirect in `wiki_page()` before render; return type changed to `Result<Response, WikiError>`
- [x] **A6** Disambiguation page type — `disambig: Option<bool>` in `Frontmatter` + hatnote notice in `wiki_chrome()`
- [x] **A7** Edit summary field — `<input id="saa-summary">` in edit form; saa-init.js reads it; git commit message includes summary when non-empty

---

## SPRINT B — Block types + comrak upgrade ✅ COMPLETE — commit TBD

- [x] **B1** Upgrade comrak `"0.29"` → `"0.52"` in `Cargo.toml`; enable `block_directive`; run `cargo test` to confirm no regressions
- [x] **B2** Infobox block type — AST walk for ` ```infobox ``` ` fenced blocks → YAML parse → `<table class="infobox">` float right (~100 lines Rust + ~60 lines CSS)
- [x] **B3** Navbox block type — AST walk for ` ```navbox ``` ` fenced blocks → YAML parse → collapsible horizontal link table at article bottom (~120 lines Rust + `initNavboxes()` JS + ~60 lines CSS)

---

## SPRINT C — Special pages + Talk namespace ✅ COMPLETE — commit TBD

- [x] **C1** `GET /special/recent-changes` — git log across content_dir → HTML table (date, article, author, summary) (~80 lines)
- [x] **C2** `GET /special/all-pages` — collect all topic files, sort, group by first letter, render alphabetical directory (~50 lines)
- [x] **C3** `GET /special/statistics` — article count, category count, redlink count, most recent edit (~40 lines)
- [x] **C4** Talk namespace — `GET /talk/{*slug}` (serve or stub) + `POST /talk/{*slug}` (append section); update Article/Talk tab active state; store in `<content_dir>/talk/` (~80 lines Rust)

---

## SPRINT D — Visual diff + search improvements ✅ COMPLETE — commit TBD

- [x] **D1** Two-column word-level diff — replace unified diff at `/diff/{slug}` with Wikipedia-style two-column red/green table using `similar::iter_inline_changes()` (~120 lines Rust + ~60 lines CSS)
- [x] **D2** Search autocomplete — add `GET /api/complete?q={prefix}` returning JSON title list; add `initSearchAutocomplete()` in `wiki.js` with debounced dropdown (~60 lines Rust + ~70 lines JS)
- [x] **D3** Edit summary display in history — update `/history/{slug}` table to show the edit summary line from the git commit message (separate from the metadata line) (~20 lines)

---

## SPRINT E — Leapfrog 2030 layer ✅ COMPLETE — commit TBD

- [x] **E1** Citation Authority Ribbon — resolve `cites:` frontmatter against `citations.yaml`; render green/amber/red ribbon below article title based on verification status; requires `verified_date:` and `status:` fields in `citations.yaml` entries (~80 lines Rust + ~30 lines CSS)
- [x] **E2** Research Trail Footer — render `research_trail.*` frontmatter fields as a collapsible `<details>` block at end of article body (~60 lines Rust + ~20 lines CSS)
- [x] **E3** Doorman editor integration — wire `POST /api/doorman/complete` and `POST /api/doorman/instruct` to `WIKI_DOORMAN_URL` env var; 501 fallback when not set (~60 lines Rust)
- [x] **E4** Freshness ribbon — compute age from `last_edited:` frontmatter; render coloured badge (current/recent/aging/stale) in article footer alongside last-edited date (~40 lines Rust + ~20 lines CSS)

---

## After all sprints complete

- [x] Rebuild release binary and deploy to all three wiki instances — binary `a56ca96` (0.3.1) deployed 2026-05-07T06:30Z; all 3 instances at 200 OK
- [x] Write session summary to `.agent/outbox.md` → `master@claude-code` requesting Stage 6 promotion
- [x] Archive this implementation plan to `.agent/implementation-plan-archive.md`
- [x] Stage TOPIC + GUIDE to `drafts-outbound/` (already done 2026-05-07)

---

## Notes

- Do not implement wikitext template transclusion (`{{template}}` macro expansion) — covered by native block types
- Do not migrate from comrak to pulldown-cmark — architecture mismatch, too costly
- Sprint A can be executed first without waiting for B1 (comrak upgrade)
- B1 must precede B2 and B3 (block_directive extension needed for cleanest syntax)
- C and D are independent of B; can be parallelised if two sessions available
- E items depend only on Sprint A CSS being in place
