---
schema: foundry-doc-v1
document_version: 0.1.0
component: app-mediakit-knowledge
status: design — Phase 1 in flight
last_updated: 2026-04-26
---

# app-mediakit-knowledge — Architecture

The wiki engine for the PointSav knowledge platform. A single Rust
binary that serves a directory of CommonMark-with-wikilinks files as
a Wikipedia-shaped read-and-edit surface.

This document is the engineering design. The strategic positioning
that motivates the design — substrate substitution for MediaWiki,
structural opposition to hyperscaler knowledge platforms, the
intended role as a continuous-disclosure surface under NI 51-102
and equivalent regimes outside Canada — is briefed separately to
Master via the cluster outbox and is Master scope to fold into
doctrine or a new convention.

## 1. Source-of-truth inversion

Most wiki and IR-tech systems treat a database as canonical and
files as an export artefact. This engine inverts that: **Markdown
files in a Git tree are canonical; every database, index, and cache
is derived state rebuildable from `git checkout && reindex`**.

Concretely:

- Page identity is a path: `<content_dir>/<slug>.md`
- Page revision history is `git log -- <slug>.md`
- Page metadata is YAML frontmatter at the top of the file
- Wikilinks are `[[Page Name]]` (CommonMark + GFM + comrak's
  built-in wikilinks extension)
- Multi-content slots — infoboxes, references, citation tables —
  are sibling files (`Foo.infobox.yaml`, `Foo.references.json`)
  or named frontmatter sections, not separate database tables
- Talk pages are sibling Markdown (`Foo.talk.md`) or a thread-store
  directory under `talk/<slug>/`
- Categories, link graph, watchlists are derived indices kept in
  an embedded KV (redb) and a search index (tantivy), both
  rebuildable on startup or on demand from the file tree

Anything that needs the database can be reconstructed; anything in
the database that conflicts with the files loses.

## 2. Stack

Pinned versions are floors; verify current minor at implementation
time and document chosen version in `Cargo.toml` comments.

| Layer | Crate | Purpose |
|---|---|---|
| HTTP server | `axum` 0.8+ | Tower-based, idiomatic, single-binary |
| Async runtime | `tokio` 1.x (`rt-multi-thread`, `signal`) | Graceful SIGTERM under systemd |
| Middleware | `tower-http` | Compression, CORS, tracing, etag, body limits |
| Markdown | `comrak` 0.29+ (`wikilinks_title_after_pipe`, `syntect`) | CommonMark + GFM + native `[[wikilinks]]` |
| Templating | `maud` | Compile-time HTML macro for chrome |
| Search | `tantivy` 0.24+ | Phase 3 — in-process BM25 + fuzzy + multilingual |
| Git write | `git2` (libgit2-sys) | Phase 4 — production-ready commit-on-edit |
| Git read | `gix` | Phase 4 — pure-Rust history/blame/log |
| Embedded KV | `redb` 4.1+ | Phase 4 — link graph, sessions, hot caches |
| Static assets | `rust-embed` 8.x | Embed CSS/JS/Mermaid/KaTeX in binary |
| Auth | `tower-sessions` + `axum-login` + `argon2` + `openidconnect` | Phase 5 — local + OIDC |
| Content hash | `blake3` 1.8+ | Phase 7 — federation/decentralisation seam |
| Logging | `tracing` + `tracing-subscriber` | Structured logs to stdout for journald |

Single C dependency: libgit2-sys (via git2). Re-evaluate gix-only
write path annually — likely viable 2027–2028 per gix maintainer's
own roadmap.

## 3. Build phases

Each phase ships a working binary. No phase depends on later phases.

### Phase 1 — render one TOPIC (current)

Smallest binary that validates the toolchain.

- `axum` server bound to `127.0.0.1:9090`
- `GET /wiki/:slug` reads `<content_dir>/<slug>.md`, parses with
  comrak (wikilinks + GFM extensions), wraps body in maud chrome
- `GET /` lists files in `<content_dir>/`
- `GET /static/*` serves embedded CSS via rust-embed
- Health endpoint at `/healthz`
- Content directory configured by `--content-dir` CLI flag or
  `WIKI_CONTENT_DIR` env var
- Bind address configured by `--bind` (default `127.0.0.1:9090`)

Target: ~300 lines across `src/main.rs`, `src/server.rs`,
`src/render.rs`, `src/error.rs`. No state beyond CLI args. No KV,
no search, no Git, no auth. Renders one page or a list.

### Phase 2 — edit endpoint

- `GET /edit/:slug` — markdown source in a `<textarea>` with
  CodeMirror 6 (loaded as embedded static asset)
- `POST /edit/:slug` — write to disk, atomically (temp + rename)
- `POST /create` — new page from title

No revision history yet; the next save just overwrites. Phase 4
adds the Git layer that turns each save into a commit.

### Phase 3 — search

- `tantivy` index at `<state_dir>/search/`
- Index rebuilt on startup from a tree walk
- `GET /search?q=` with fuzzy + BM25
- Page edits trigger incremental re-index

### Phase 4 — Git sync + history

- `git2` opens the content dir as a Git repo
- Each `POST /edit` commits with author from session
- `GET /history/:slug` reads `git log -- <slug>.md` via `gix`
- `GET /diff/:slug?a=<sha>&b=<sha>` renders unified diff
- `GET /blame/:slug` annotates lines with last-author + commit
- `redb` index for link graph (`<from_slug>` → `[<to_slug>]`)
  rebuilt on commit; `GET /backlinks/:slug` reads from index

This is where the engine becomes a wiki rather than a static-site
generator with edit. It's also where the source-of-truth inversion
becomes load-bearing — Git is the revision system, not a separate
revisions table.

### Phase 5 — auth

- `tower-sessions` with `redb` session backend
- `axum-login` middleware for request auth context
- Local accounts via `argon2id` password hashing (OWASP params)
- OIDC SSO via `openidconnect` (Google / Microsoft / Okta /
  generic provider)
- Per-page ACLs in frontmatter (`read: public`, `edit: members`)
  — Phase 5.1

### Phase 6 — wikilink resolution + backlinks

Most of this is incidental to Phase 4 once the link graph index
exists. Phase 6 adds:

- Slug normalisation rules (case-insensitive first letter,
  spaces → underscores, MediaWiki-style canonicalisation — chosen
  over Obsidian/Foam/Roam variants for migration compatibility)
- Redirect chain support (frontmatter `redirect_to:`)
- Ambiguity disambiguation (two pages with the same title)
- Backlinks panel rendered in the page chrome

### Phase 7 — federation seam (planned)

Structural seam landing in v0.3.x; the file-format and addressing
discipline starts now so it can light up later without rework.

- `blake3` hash of every TOPIC at every commit, stored in `redb`
  keyed by `(slug, revision_sha)`
- `GET /api/v1/page/:hash` returns page content by content-address
- `GET /api/v1/peer/:url/page/:hash` proxies to another instance
  for federated read
- Git remotes are the federation primitive — no new protocol

Decentralisation futures (TOPIC content-addressing extending to
on-chain anchoring for verifiable immutability) compose on top of
this seam without engine changes. Discipline is in the format
from day one; the on-chain integration is intended for v0.5.0+
and depends on operator decisions out of scope here.

### Phase 8 — disclosure mode (planned)

What turns the wiki into a continuous-disclosure substrate.

- iXBRL extractor for financial-statement frontmatter blocks
  (planned; mapping from a structured YAML block to ESEF-compliant
  iXBRL output for jurisdictions that mandate it)
- NI 51-102 / OSC 51-721 frontmatter linter — required fields,
  forward-looking-information labelling, third-party governance
  language checks (planned)
- OpenTimestamps anchoring on every commit (Bitcoin-anchored, free)
- RFC 3161 TSA support for jurisdictions requiring formal
  trusted-timestamping (planned, configurable TSA URL)
- MediaWiki XML import (`wiki import-mediawiki-xml`) for migration
  from existing wikis (planned)
- MediaWiki Action API compatibility shim for pywikibot and
  similar tooling (planned)

Phase 8 is what makes this engine the BCSC continuous-disclosure
substrate rather than just a wiki. It is also the planned moat:
no surveyed incumbent (MediaWiki, Confluence, Notion, hyperscaler
knowledge bases, Q4 Inc IR-tech) combines Markdown-native
authorship + iXBRL extraction + cryptographic timestamping +
per-jurisdiction export adapters in one substrate.

## 4. Process model

Single binary, runs as a systemd unit. No Docker, no containers
(per `~/Foundry/conventions/zero-container-runtime.md`).

```
[Unit]
Description=PointSav Knowledge Wiki
After=network.target

[Service]
Type=simple
User=local-knowledge
Group=local-knowledge
ExecStart=/usr/local/bin/app-mediakit-knowledge serve \
    --content-dir /var/lib/local-knowledge/content \
    --state-dir /var/lib/local-knowledge/state \
    --bind 127.0.0.1:9090
ReadWritePaths=/var/lib/local-knowledge
ProtectSystem=strict
ProtectHome=true
NoNewPrivileges=true
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

The systemd unit + bootstrap script are workspace-tier artefacts
(Master scope). This crate ships them as catalog reference under
`pointsav-fleet-deployment/media-knowledge-documentation/` for
Master to install.

Loopback bind by default. Public-internet exposure
(`documentation.pointsav.com` and equivalents) is a v0.5.0+
operator decision out of scope here; it lands by terminating TLS
at a reverse proxy that proxies to the loopback unit, not by
binding the wiki to a public address.

## 5. Data layout on disk

```
<content_dir>/                         (Git-tracked; canonical)
├── README.md                          repo-level readme
├── .wiki/                             reserved for engine config
│   └── slugs.toml                     redirect / alias rules
├── glossary-documentation.csv         existing — flat data
├── topic-architecture.md              CommonMark + frontmatter
├── topic-architecture.es.md           bilingual sibling per §6
├── TOPIC-DOCTRINE.md                  uppercase TOPIC convention
├── topic-wiki-engine/                 sub-namespacing for big topics
│   ├── index.md
│   ├── render-pipeline.md
│   └── render-pipeline.infobox.yaml   MCR-style typed slot
└── talk/
    └── topic-wiki-engine/
        └── 2026-04-26-render-question.md

<state_dir>/                           (NOT Git-tracked; derived)
├── search/                            tantivy index
├── kv.redb                            link graph + sessions
└── timestamps/                        OpenTimestamps proofs (Phase 8)
```

The split is load-bearing. `<content_dir>` round-trips through
any Markdown editor. `<state_dir>` is regenerable from
`<content_dir>` plus engine code; backing it up is optional.

## 6. Frontmatter schema

```yaml
---
schema: foundry-doc-v1
document_version: 0.1.0          # semver per CLAUDE.md §7
title: "Page Title"              # display title (filename is canonical)
slug: page-title                 # canonical URL slug
aliases: [old-page-title]        # redirect-from
language: en                     # ISO 639-1
translations:
  es: page-title.es              # bilingual sibling per §6
authors:
  - jwoodfine
  - pwoodfine
last_revised: 2026-04-26
cites:                           # per CLAUDE.md §16
  - ni-51-102
  - constitutional-ai-2212-08073
forward_looking: false           # NI 51-102 FLI label per §6
disclosure_class: narrative      # narrative | financial | governance
acl:
  read: public
  edit: members
---
```

All fields optional except `schema`, `document_version`, `title`.
The linter (Phase 8) checks that pages classified `disclosure_class:
financial` have an iXBRL block; that pages with `forward_looking:
true` carry the cautionary language patterns; that any third-party
governance claims appear with documented `cites:` resolution.

## 7. Compatibility surface

The MediaWiki ecosystem moat (~1,500 extensions, hundreds of
thousands of templates, pywikibot, Wikidata) demands a migration
path. Two compatibility surfaces are planned:

- **`wiki import-mediawiki-xml`** subcommand: walks a `Special:Export`
  XML dump, runs each revision through a wikitext-to-Markdown
  converter (`pandoc` shelled out, with a pluggable template-
  expansion shim), writes one file per page, one commit per
  revision, preserves authorship and timestamp.
- **MediaWiki Action API shim** under `/api.php`: enough of the
  Action API surface that pywikibot and common bots Just Work
  against this engine. Read-side first (queries, page-fetch),
  then a deliberate subset of write-side (edit, move).

Both planned for Phase 8. The single decision that determines
whether a Wikimedia-class migration is conceivable or theoretical
is whether bots keep working unmodified.

## 8. Out of scope (engine)

- Real-time collaborative editing (CRDT/OT) — adjacent useful
  feature; intended for v0.4.x via `yrs` (Yjs Rust port); not in
  Phase 1–7
- Mobile-first editor UX — the desktop editor is the proof; mobile
  follows after v0.3.x
- Multi-tenancy at engine level — one engine instance serves one
  content tree; tenancy is at the deployment layer (multiple
  instances per tenant)
- AI integration of any kind — the engine renders, edits, and
  serves Markdown. Optional-AI capabilities (suggest-as-you-write,
  semantic search) are intended to land via the Doorman
  (`service-slm`) as a Ring 3 adapter against the same Markdown
  corpus, not by coupling the engine to any model

## 9. Testing approach

- Unit tests on the renderer (wikilink parsing, frontmatter
  handling, slug normalisation)
- Golden-file tests on the full HTML output against fixture
  Markdown
- Integration tests against `tower::ServiceExt::oneshot` for
  HTTP routes
- A minimum fixture corpus under `tests/fixtures/content/` —
  enough TOPICs to exercise wikilinks, redirects, frontmatter
  variation
- No reliance on external services at any test phase. Phase 4
  Git tests use `tempdir` repos; Phase 5 auth tests stub OIDC
  via mock provider

## 10. Versioning

Per `~/Foundry/CLAUDE.md` §7: `MAJOR.MINOR.PATCH`. Patch increments
per accepted commit; minor per feature milestone (each Phase
above is a minor); major per breaking change. Phase 1 lands at
crate version `0.1.0`; Phase 2 at `0.2.0`; etc. SSH-signed tags;
Version: trailer on commit messages.

## 11. References

- `~/Foundry/DOCTRINE.md` — constitutional charter
- `~/Foundry/CLAUDE.md` — workspace operational guide
- `~/Foundry/conventions/zero-container-runtime.md` — systemd, no Docker
- `~/Foundry/conventions/citation-substrate.md` — frontmatter discipline
- `~/Foundry/conventions/bcsc-disclosure-posture.md` — Phase 8 source
- `~/Foundry/conventions/knowledge-commons.md` — public-content posture
- `~/Foundry/clones/project-knowledge/.claude/manifest.md` — cluster scope
- Strategic synthesis (substrate substitution, hyperscaler structural
  opposition, continuous-disclosure positioning, decentralisation
  path, Q4 Inc critique) — briefed to Master via cluster outbox
  2026-04-26; Master scope to fold into doctrine or a new convention.
