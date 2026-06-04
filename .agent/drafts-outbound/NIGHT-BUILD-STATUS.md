# Knowledge Platform Build Status (git-verified) — 2026-06-04

This report is reconstructed from `git log` + filesystem + a live smoke test,
not from any prior narrative file. The previous contents of this file were
fictional (SHAs did not match git; Phases 6/7 were claimed "DEFERRED" while git
shows them committed). Source of truth: the `app-mediakit-knowledge` sub-clone
`.git` HEAD and the shared cargo target dir.

- App repo: `/srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge`
- Build artifact: `/srv/foundry/cargo-target/mathew/release/app-mediakit-knowledge`
- HEAD at report time: `e94bfa9d` (openapi regen after auth removal)

---

## Phase ledger (real commit SHAs, `git log --oneline -15`)

| SHA | Phase / change | Notes |
|---|---|---|
| `920425b6` | /es/ locale-keyed chrome strings (L22) | Spanish home page |
| `3662b11a` | font preload tags — Inter + Source Serif 4 (L23) | 2 preloads |
| `5a715736` | mobile safe-area-inset-bottom (L24) | bottom chrome |
| `d143e10a` | separate title/date in recently-changed (L27) | chrome fix |
| `257559d5` | CodeMirror bundle only on /edit/* (L25) | later superseded by auth removal |
| `48562316` | Phase 1 — modular src/ scaffold; mounts+blueprints (L20,L21,L26) | foundation |
| `5f625ce2` | Phase 2 — render pipeline; comrak+wikilinks; JSON-LD; citations; glossary | render |
| `805faec2` | Phase 3 — chrome modules; mobile-first CSS; scroll-spy TOC; Cmd+K (L21–L27) | chrome; theme-woodfine.css deleted |
| `0da936bc` | Phase 4 — routes wired; Tantivy BM25 search; /search; Atom+JSON feeds | routes |
| `1c81e0ec` | Phase 5 — git2 commit-on-edit; gix history; redb link graph; dead-link gate (L18,L29) | git+links |
| `43739cc4` | Phase 6 — cookie sessions; argon2id; edit workflow; pending queue; editor.js (L25) | auth (later removed) |
| `1a428fa3` | Phase 7 — MCP JSON-RPC 2.0 (5 methods); openapi regenerated | MCP |
| `d72df770` | Phase 8 — token layout vars + knowledge.toml templates (L21) | theming |
| `0184fb16` | refactor(simplify) — remove auth+edit+CodeMirror; git-only (Q1; L25,L26) | **auth removed** |
| `e94bfa9d` | docs(api) — regenerate openapi.yaml after auth removal | **HEAD** |

Phases 1–8 are all committed (not deferred). The auth/edit/CodeMirror stack
added in Phase 6 was subsequently removed in `0184fb16` per operator decision
Q1 (git-only workflow), and `openapi.yaml` regenerated in `e94bfa9d`.

---

## Auth removal (`0184fb16` → git-only workflow)

Verified at the binary and manifest level:

- `app-mediakit-knowledge/Cargo.toml` no longer declares `argon2`, `rusqlite`,
  or `uuid` — all three removed.
- Release binary contains no `argon2` / `rusqlite` / `password_hash` symbols.
  The CLI `--help` text still references `WIKI_ADMIN_USERNAME` /
  `WIKI_ADMIN_PASSWORD_HASH` env vars as override exceptions — these are now
  inert no-ops (cosmetic stale help text only; not a functional regression).
- `openapi.yaml` reduced by 785 deletions / 138 insertions in `e94bfa9d`
  (~923-line drop) — all session/login/edit/pending endpoints stripped.

---

## Theming (`d72df770`, Phase 8)

`static/tokens.css` (+10 layout vars) plus three knowledge.toml templates:
`config/documentation.toml`, `config/projects.toml`, `config/corporate.toml`
(+22 lines each). The matching DESIGN-TOKEN-CHANGE draft still requires
master_cosign from project-design before Stage 6 (pre-deploy item).

CSS file count = 3 (L21): `static/style.css`, `static/tokens.css`,
`static/tokens-woodfine.css`. `theme-woodfine.css` deleted in Phase 3.

---

## OpenAPI regen (`e94bfa9d`)

`openapi.yaml` regenerated after auth removal. 138 insertions, 785 deletions.

---

## Release build

| Item | Result |
|---|---|
| `cargo build --release` | **PASS** |
| Build time | 8m 28s |
| Warnings / errors | 3 warnings (dead_code: `nav_home`, `nav_recent` in `home_handlers.rs`), 0 errors |
| Binary | `/srv/foundry/cargo-target/mathew/release/app-mediakit-knowledge` |
| Binary size | **12M** (12,453,560 bytes) — reduced after rusqlite/argon2/uuid removal |
| `--version` | `app-mediakit-knowledge 0.1.0` |

Note: the workspace cargo target dir is `/srv/foundry/cargo-target/mathew/`
(not a per-clone `target/`). The deploy checklist install path must reference
the resolved target dir, not `app-mediakit-knowledge/target/release/`.

---

## Dead-link gate (`cargo xtask check-content`, L18/L29)

The CI gate exits non-zero on any dead wikilink. Run against each content repo:

| Repo | Dead links | Missing required frontmatter | Gate |
|---|---|---|---|
| content-wiki-documentation | 4,568 | 0 | **FAIL** (dead links present) |
| content-wiki-projects | 396 | 0 | **FAIL** (dead links present) |
| content-wiki-corporate | 290 | 0 | **FAIL** (expected per BRIEF §9 — record, do not block phase) |

All three repos report 0 articles with missing required frontmatter fields.
The dead-link counts are wikilink targets with no matching article (many are
intentional "wanted pages", cross-repo links, or ES-pair links). Corporate is
a known-issue repo per BRIEF §9. Resolving the documentation and projects
gates is a pre-deploy task (see PHASE-9-DEPLOY-CHECKLIST.md).

---

## Smoke test (release binary, port 9099, documentation content)

Started with `WIKI_KNOWLEDGE_TOML` pointing at a temp toml that mounts
`content-wiki-documentation` and binds `127.0.0.1:9099`. The server builds the
Tantivy search index (~11s) before binding the port — a >4s warm-up is
required; first attempt at 5s saw connection-refused, retried after index
ready.

| Check | Expected | Actual | Result |
|---|---|---|---|
| `GET /healthz` | ok | `ok` | PASS |
| `GET /` title | a title tag | `<title>PointSav Documentation</title>` | PASS |
| Font preloads on `/` (L23) | 2 | 2 — `grep -o 'rel="preload"' \| wc -l` = 2 (both tags on one line, so `grep -c` reports 1) | PASS |
| `GET /wiki/slm-tiered-substrate` | 200 | 301 → `/wiki/substrate/slm-tiered-substrate` → 200 | PASS (canonical-path redirect) |
| editor.js on article page (L25) | 0 | 0 | PASS |
| cm-saa on article page | 0 | 0 | PASS |
| Search | results for "substrate" | `/api/search` → **404 (no such route)**; real routes `/search?q=` (HTML, 200, substrate hits) and `/api/complete?q=` (JSON autocomplete, returns results) both work | PASS via real routes |

Search-endpoint correction: the spec's `/api/search` path is not a route in
this build. Routes registered in `src/server/mod.rs`: `/search` (HTML results
page) and `/api/complete` (JSON typeahead). Both return substrate matches.

Article-slug correction: `/wiki/<slug>` 301-redirects to the path-qualified
canonical URL `/wiki/substrate/slm-tiered-substrate`, which serves 200 with no
`editor.js` and no `cm-saa` bundle — confirming the edit/CodeMirror surface is
fully gone after auth removal.

Test server (PID 2249774) was killed after the run; confirmed stopped.

---

## Summary

Release build PASS; binary 12M (reduced post-auth-removal); auth/edit/CodeMirror
fully removed and verified absent from article pages; theming + openapi regen
committed. Dead-link gate FAILS on documentation (4,568) and projects (396) —
must be resolved pre-deploy; corporate (290) is a known-issue repo per BRIEF §9.
