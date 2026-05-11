---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-knowledge
target_repo: content-wiki-documentation
target_path: applications/   # candidates: applications/, architecture/, infrastructure/ — project-language decides
target_filename: app-mediakit-knowledge.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-04-27T16:45:00Z
authored_by: task-project-knowledge (session 619abe3eff24497e)
authored_with: opus-4-7
references:
  - vendor/pointsav-monorepo/app-mediakit-knowledge/ARCHITECTURE.md
  - vendor/pointsav-monorepo/app-mediakit-knowledge/UX-DESIGN.md
  - vendor/pointsav-monorepo/app-mediakit-knowledge/INVENTIONS.md
  - vendor/pointsav-monorepo/app-mediakit-knowledge/docs/PHASE-2-PLAN.md
  - vendor/pointsav-monorepo/app-mediakit-knowledge/docs/PHASE-4-PLAN.md
  - https://documentation.pointsav.com/
  - https://www.tantivy-search.org/
  - https://www.redb.org/
  - https://github.com/rust-lang/git2-rs
  - https://github.com/Byron/gitoxide
  - https://github.com/yjs/yjs
  - https://codemirror.net/
  - https://commonmark.org/
  - https://github.com/kivikakk/comrak
  - https://schema.org/TechArticle
  - https://schema.org/DefinedTerm
  - https://datatracker.ietf.org/doc/html/rfc4287   # Atom syndication
  - https://www.jsonfeed.org/version/1.1/
  - https://www.sitemaps.org/protocol.html
  - https://llmstxt.org/
  - https://en.wikipedia.org/wiki/Wikipedia:Manual_of_Style/Layout
  - constitutional-ai-2212-08073
  - ni-51-102
  - osc-sn-51-721
  - c2sp-tlog-tiles
  - DOCTRINE.md claim #29 (Substrate Substitution)
  - DOCTRINE.md claim #31 (Constitutional Constrained Author / CCA)
  - DOCTRINE.md claim #16 (Optional Intelligence Layer)
  - DOCTRINE.md claim #21 (Role-Conditioned Cluster Adapters)
  - DOCTRINE.md claim #22 (Adapter Composition Algebra)
  - conventions/disclosure-substrate.md
  - conventions/three-ring-architecture.md
notes_for_editor: |
  This is the headline architecture TOPIC for the wiki engine. It is the most substantive
  draft this cluster authored at v0.1.29 and warrants the best register pass project-language
  can produce.

  The cluster owns the rationale for every design choice in §2 onward. Preserve that
  authority in the refinement pass — pare for register, not for substance. The Doctrine-claim
  references and the invention-catalogue references are load-bearing; project-language
  resolves [citation-id] forms but must not delete the claim references.

  §4 (muscle-memory chrome) is the audience hook. A reader from the financial community
  recognises Wikipedia from the layout; a reader from the engineering community recognises
  it from the layout AND the substrate-native compatibility surface. Both audiences should
  feel at home.

  §8 (compatibility surface) restates Doctrine claim #29 in PROSE-TOPIC voice. There is
  a separate sibling TOPIC draft (substrate-native-compatibility.draft.md) that goes deeper
  on the Action API drop rationale; cross-reference but don't duplicate.

  §9 enumerates inventions. project-language can pare the list for the audience or split
  into a sub-page; recommend keeping the count visible (currently 8 inventions catalogued
  in INVENTIONS.md as of 2026-04-27) because the count is the headline.

  Repetition note: §1 and §2 both cover "git is canonical, the binary is a view." Intentional
  — §1 is the elevator pitch, §2 is the substantive treatment. project-language picks one
  or merges per audience.

  target_path candidates listed; project-language decides per its taxonomy decision (the
  naming-convention.md draft in content-wiki-documentation's .claude/rules/ proposes nine
  categories, awaiting operator ratification).
---

# app-mediakit-knowledge — the wiki engine

## §1 What it is

`app-mediakit-knowledge` is the wiki engine that serves
PointSav's engineering documentation at
`https://documentation.pointsav.com`. The engine is a single-binary
Rust service composed of an `axum` HTTP server, a `comrak` CommonMark
renderer with PointSav-specific extensions for wikilinks +
footnotes + table of contents + section anchors, a `tantivy`
full-text search backend, and a templating layer in `maud` that
ships four article templates and four-or-so static-asset bundles.
The engine reads markdown files from a content directory the
operator names at startup, renders them on demand into HTML, and
returns them with caching headers tuned for a documentation
audience.

The engine is a *view* over a markdown tree, not a content
repository. The markdown tree is canonical; the running binary is
a view that any number of operators can stand up over the same
content tree, or different content trees, with no shared mutable
state on the binary side. This source-of-truth inversion is the
single most important design choice and is treated more
substantively in §2.

The engine's first public deployment went live on 2026-04-27 at
16:25 UTC, serving a four-file placeholder content tree at
`https://documentation.pointsav.com`. The full route surface from
build phases 1, 1.1, 2, and 3 is operational; phases 4 through 8
are planned but not yet implemented.

## §2 Source-of-truth inversion

The substrate's load-bearing design choice: **git is canonical;
the running binary is a view; CRDT (when collab is enabled) is
session-ephemeral**.

Every concrete artefact a reader of the wiki interacts with — the
HTML page, the Atom feed entry, the JSON-LD block, the search-result
hit, the wikilink graph — is derived at request time from the
markdown tree on disk. The disk state is what gets committed,
reviewed, replicated, and disclosed. The HTML is throwaway. The
Tantivy index is throwaway (rebuilt from the markdown tree on
startup). The redb wikilink graph (Phase 4) is throwaway. The
collab CRDT (Phase 2 Step 7, default-off) is throwaway between
sessions.

This inversion is the inverse of MediaWiki's traditional model,
where the database is canonical and the file system is a derived
working copy. The engine's choice flips that: the file system is
canonical, the database (search index, link graph) is a derived
working copy. The motivation is operational simplicity (a
content-tree backup is a `git clone`; a content-tree replication
is a `git pull`; a content-tree audit is a `git log`) and a
substrate-level invariant (every published claim is a signed git
commit; the disclosure record is the git history; the BCSC
continuous-disclosure posture per `conventions/bcsc-disclosure-posture.md`
is enforced by the substrate's structure rather than by policy
alone).

Other patterns follow from the inversion. The wiki has no
preview-then-publish workflow because the canonical state is what
got committed; an edit committed *is* a publication. The wiki has
no scheduled-publish workflow because the same property holds.
The wiki has no server-side draft state because drafts live in
the contributor's git working copy or in the project-knowledge
cluster's drafts-outbound port (per the new
`conventions/cluster-wiki-draft-pipeline.md`), not in a database
the wiki engine owns.

## §3 The route surface

The engine exposes a tight set of HTTP routes. Each is independent;
no route depends on session state or on a database the engine
owns.

| Route | Purpose | Phase |
|---|---|---|
| `/healthz` | Liveness check (returns the literal string `ok`) | 1 |
| `/` | Index page (lists all articles in the served content tree) | 1 |
| `/wiki/{slug}` | Rendered article HTML | 1 |
| `/static/{*path}` | Static assets (CSS, JS, fonts) | 1 |
| `/edit/{slug}` | In-browser editor (CodeMirror 6) | 2 |
| `POST /edit/{slug}` | Atomic-write edit endpoint with squiggle linting | 2 |
| `/search?q=` | Full-text search results (Tantivy / BM25) | 3 |
| `/feed.atom` | RFC 4287 Atom syndication feed | 3 |
| `/feed.json` | JSON Feed 1.1 syndication | 3 |
| `/sitemap.xml` | sitemaps.org compliant sitemap | 3 |
| `/robots.txt` | Crawler discovery | 3 |
| `/llms.txt` | The emerging llmstxt.org convention for LLM crawlers | 3 |
| `/git/{slug}` | Raw markdown source for git-clone-style ingestion | 3 |
| `/ws/collab/{slug}` | WebSocket passthrough relay (default-off behind `--enable-collab`) | 2.7 |

Phase 4 will add `/history/{slug}`, `/blame/{slug}`, `/diff/{a}/{b}`,
`/backlinks/{slug}`, an MCP server route for agent clients, and a
read-only Git remote over smart-HTTP — letting any git client clone
the served content tree as a regular git repository, riding the
existing TLS termination.

The engine emits JSON-LD `TechArticle` and `DefinedTerm` schema in
every rendered article's `<head>` block (Phase 2 Step 1) for
search-engine and LLM-crawler comprehension. The structured data
is generated from the article's frontmatter, not hand-authored
per page; the schema is the same shape across the corpus.

## §4 Wikipedia muscle-memory chrome (Phase 1.1)

The engine ships with a deliberately Wikipedia-recognisable chrome.
A reader of any Wikipedia article will navigate the engine without
prompting, and a reader unfamiliar with Wikipedia will pick up the
patterns in seconds because they are well-documented as conventions.

What was kept (per `UX-DESIGN.md` §1):

- Article / Talk tabs at the top of the page (Talk reserved for a
  future implementation; tab structure present today)
- Read / Edit / View history tabs alongside the Article/Talk pair
- Per-section `[edit]` pencils on every `## H2` and `### H3` heading
- End-of-article ordering: References, See also, Categories, with a
  footer band naming the article's licence and the substrate
- Hatnote band at the top of the article (used for disambiguation
  and "this article is about X; see also Y" cross-references)
- Lead first-sentence convention (the article's subject is the
  bolded subject of the lead sentence)
- Tagline directly under the article title
- Collapsible left-rail TOC (built from `## H2` and `### H3`
  headings; deeper headings render normally but do not enter the TOC)
- Language switcher (currently English / Spanish; structurally ready
  for additional languages without re-templating)

What was added (substrate-specific, not Wikipedia-traditional):

- Citation badges next to inline `[citation-id]` references; hover
  reveals the registry entry from `~/Foundry/citations.yaml`
- Forward-Looking-Information cautionary banner (Phase 8 will harden
  this into a linter check; today the banner renders when an article's
  frontmatter sets `forward_looking: true`)
- BCSC `disclosure_class` field rendered as a small badge in the
  article header (`narrative` / `financial` / `governance`); not
  visible in default chrome but exposed in the JSON-LD
- Information Verifiability Citation (IVC) masthead band placeholder
  (Phase 7 will light up the IVC machinery; the band is a visual
  surface only at this time)
- Reader density toggle (compact / comfortable; settings persist
  client-side)

The chrome is implemented in four `maud` HTML templates
(`article.html`, `category.html`, `search.html`, `editor.html`)
and a CSS bundle that tracks the Wikipedia *Vector* skin's spacing
and typography rather than imitating its colour palette. The
target is muscle memory, not literal mimicry; a reader who knows
Wikipedia recognises the layout, but the visual identity is
distinct.

## §5 Editor surface (Phase 2)

The wiki's editor is a CodeMirror 6 instance vendored into the
binary's static-asset bundle, served at `/edit/{slug}`. It supports
markdown highlighting with line numbers, configurable line wrap,
undo/redo history with a keyboard accelerator, and atomic disk
write via the engine's `POST /edit/{slug}` endpoint.

Three substrate-aware editor features distinguish the implementation:

**Squiggle linting (Phase 2 Step 4).** Seven deterministic rules
flag editorial issues at typing time, each with a cited authority
visible in a hover-card. The rules cover banned vocabulary
("magical", "AI-powered", and the rest of the workspace's
Do-Not-Use list), forward-looking framings without the cautionary
banner pattern, BCSC-discipline checks (Sovereign Data Foundation
referenced in current-tense without a planned/intended qualifier),
and a handful of Bloomberg-article-standard register checks.
The rules are deterministic at edit time; the AS-2 grammar-time
constraint (Doctrine claim #31, Constitutional Constrained Author)
will harden these into compile-time-equivalent guarantees once the
service-slm Doorman ships the `llguidance` integration.

**Citation autocomplete (Phase 2 Step 5).** Pressing `[` triggers
a typeahead populated from the workspace citation registry at
`~/Foundry/citations.yaml`. The contributor types `[ni-51` and the
list narrows to `ni-51-102` (BCSC continuous disclosure) plus any
other matches. Selecting an entry inserts the canonical
`[citation-id]` form and adds the citation to the article's
frontmatter `cites:` list automatically. The pattern keeps
citation discipline cheap to follow and expensive to skip.

**Three-keystroke ladder for Doorman (Phase 2 Step 6 stubs).**
Tab opens a ladder of "ask Doorman" affordances at the cursor
position — find a citation, suggest a hatnote target, generate a
disambiguation link, propose a section heading. The affordances
return 501 stubs in the v0.1.29 binary; Phase 4 wires them up to
the service-slm Doorman per `conventions/three-ring-architecture.md`
Ring 3 routing.

The editor's atomic-write semantics are conservative: the engine
writes the new file content to a temporary path in the same
directory, fsyncs, and renames over the destination. A failed
write is visible to the contributor (the page returns an error
state) and leaves the canonical content untouched. Concurrent
edits from two non-collab sessions race at the rename step; the
last-writer-wins convention is documented.

## §6 Search, feeds, and ingestion (Phase 3)

The engine indexes the content tree on startup and incrementally
on edit. The index is on-disk Tantivy (BM25 by default) at
`<state-dir>/search/`, rebuilt from the content tree if it is
missing. Reindex on edit is triggered from the `POST /edit/{slug}`
flow; the Tantivy `IndexWriter` is held in an `Arc<Mutex<>>` per
the crate's typical pattern and released before reader reload to
avoid the asynchronous-reload race that bit early Phase 3 work.

Three syndication formats render the corpus to crawlers:

- **`/feed.atom`** — RFC 4287 Atom syndication. Each article is a
  feed entry with `title`, `summary`, `published`, `updated`, and
  the article's `cites:` list resolved against the registry.
- **`/feed.json`** — JSON Feed 1.1 syndication. Identical content
  shape to the Atom feed; format differs.
- **`/sitemap.xml`** — sitemaps.org compliant. Lists every
  article URL with its last-modified date.

Two crawler-discovery files round out the surface:

- **`/robots.txt`** — User-agent rules (currently permissive).
- **`/llms.txt`** — The emerging llmstxt.org convention for LLM
  crawlers. Lists the corpus's authoritative source URLs and the
  per-article markdown URLs (`/git/{slug}`) for ingestion.

The `/git/{slug}` route serves raw markdown source. An LLM crawler
or a future federation peer can ingest the content tree by
following `/llms.txt` to discover the article list, then fetching
`/git/{slug}` for each article's source. This is the substrate's
content-addressed federation path; Phase 7 will harden it with
blake3 hashing (Phase 4 Step 4.5 lays the groundwork).

The route accepts an optional `.md` suffix on the slug for the case
of a tool that expects markdown URLs to end in `.md`. Both shapes
return the same content.

## §7 Real-time collaboration (Phase 2 Step 7)

The engine optionally supports real-time collaborative editing via
yjs CRDT. The feature is default-off behind the `--enable-collab`
CLI flag; the production deployment at v0.1.29 does not enable
it.

The implementation follows the substrate's source-of-truth
inversion: **the server is a passthrough WebSocket relay, not a
Yjs server**. Yjs document state never lives on the server. The
relay is a thin `tokio::sync::broadcast` per-slug room with a
256-message lag buffer; clients send Yjs update packets, the
server forwards to other clients in the room, and persistence
flows through the existing `POST /edit/{slug}` save path on
deliberate save (not on every keystroke). When all clients leave
the room, the room closes and any unsaved CRDT state is discarded.

The motivation: the substrate's canonical record is git, not a
Yjs document. A long-lived Yjs document on the server would create
a parallel canonical record that drifts from git, defeats the
disclosure-substrate posture, and complicates audit. The
passthrough relay keeps git canonical and the CRDT
session-ephemeral.

The client lazy-loads `cm-collab.bundle.js` (302 KB; built
out-of-tree and committed to `static/vendor/`) only when the
template's `window.WIKI_COLLAB_ENABLED` flag is set by the
server, so production deploys without `--enable-collab` never
load any of the yjs JavaScript and never expose the
`/ws/collab/{slug}` route.

A manual two-client smoke (two browsers editing the same TOPIC,
seeing each other's cursors) is the current ratification path.
The implementation is wired through unit and integration tests;
the smoke remains because cursor-rendering UX is a visual property
that is awkward to assert programmatically.

## §8 Substrate-native compatibility surface

The engine is a substrate-native wiki, not a MediaWiki shim. This
is deliberate and reflects DOCTRINE claim #29 (Substrate
Substitution) ratified at workspace v0.1.10 and refined at
v0.1.14.

What was kept:

- **`xml-dump` import path.** The engine accepts MediaWiki XML
  dumps for one-time corpus migration. A future `import-mediawiki-xml`
  tool consumes the dump format and emits the engine's markdown +
  frontmatter shape. The migration is one-shot; the live wiki does
  not run a MediaWiki API.
- **URL conventions.** `/wiki/{slug}` matches MediaWiki's URL
  layout, so existing links from external sites continue to
  resolve.
- **Wikilink syntax.** `[[slug]]` and `[[slug|display text]]`
  match Wikipedia's convention; existing contributors recognise
  the form.
- **Footnote syntax.** `[^1]` matches CommonMark; the bibliography
  resolves footnotes against the article's frontmatter
  `references:` list.

What was dropped:

- **MediaWiki Action API shim.** The shim was scoped at workspace
  v0.1.10 and dropped at v0.1.14 per the conflict surfaced in
  cluster session 2. The substrate-native API surface
  (article HTML, JSON-LD, Atom, JSON Feed, sitemap, llms.txt,
  raw markdown via `/git/{slug}`, search via `/search?q=`, edit
  via `POST /edit/{slug}`) covers the use cases the Action API
  shim would have served, without introducing a parallel
  authoritative interface that would need separate maintenance,
  separate hardening, and separate compliance review.
- **MediaWiki templates / parser functions.** The engine's
  rendering path is CommonMark with PointSav-specific extensions,
  not a MediaWiki parser. Templates are not supported. The
  workaround is markdown partials, which the engine does not
  expand server-side; the contributor inlines the partial content
  during edit.
- **Pywikibot / bot ecosystem.** The substrate's automation path
  is the workspace's existing tooling (commit-as-next, bin/
  helpers, the trajectory-substrate corpus capture), not the
  pywikibot framework.

The trade-off is a narrower compatibility surface against a
substrate-coherent posture. A reader migrating from a MediaWiki
deployment loses templates and the Action API; gains source-of-truth
inversion, deterministic rendering, BCSC-grounded disclosure
posture, and a much smaller attack surface. For PointSav's
engineering documentation use case, the trade is favourable.

A separate sibling TOPIC (`substrate-native-compatibility.md`)
goes deeper on the rationale.

## §9 Inventions catalogue

`INVENTIONS.md` at the crate root catalogues eight engine-specific
inventions (current count as of v0.1.29; the catalogue grows as
phases ratify). The inventions are substantive design choices that
distinguish this engine from a generic markdown-served-as-HTML
implementation:

1. **Source-of-truth inversion** — git canonical, binary view,
   CRDT ephemeral; covered §2 above
2. **Substrate-native compatibility** — Doctrine claim #29; covered §8
3. **Constitutional Constrained Author (CCA)** — Doctrine claim
   #31; the editor's squiggle linter at edit time + AS-2 grammar
   constraint at decode time produce text that is structurally
   incapable of violating the BCSC disclosure posture or the
   workspace banned-vocabulary policy
4. **Information Verifiability Citation (IVC)** — Phase 7+; the
   masthead band that surfaces verification status of every
   published claim; today a placeholder, the IVC machinery is
   substantive future work
5. **Substrate-Authored Affordances (SAA)** — the squiggle
   linter's seven deterministic rules; visible at edit time via
   hover-cards with cited authority
6. **`verify://` URL scheme** — Phase 7+; resolves a citation ID
   to its verifiable source via the substrate's verification
   path, not a public DNS lookup
7. **The passthrough WebSocket relay** — covered §7 above; the
   collab implementation that does not introduce a parallel
   authoritative record
8. **The substrate-native API surface set** — the route table in
   §3 above; what the Action API shim would have replicated, done
   coherently with the substrate's other invariants

The catalogue is open. New inventions land in `INVENTIONS.md` with
a brief, the substrate justification, the implementation phase,
and the references that anchor it.

## §10 Build phases trajectory

The engine is shipped through a sequenced build plan documented in
`ARCHITECTURE.md` §3. As of 2026-04-27 the engine is at the end of
Phase 3:

- **Phase 1** — axum server, comrak rendering, four templates,
  static assets, `/healthz` + `/` + `/wiki/{slug}` + `/static/{*path}`
  (shipped, 8 tests)
- **Phase 1.1** — Wikipedia muscle-memory chrome; covered §4
  above (shipped, 19 tests)
- **Phase 2** — JSON-LD baseline + atomic edit endpoint + CodeMirror
  vendoring + squiggle linter + citation autocomplete + Doorman
  ladder stubs + collab passthrough relay (Steps 1–7 all shipped,
  97 tests)
- **Phase 3** — Tantivy search backend + on-disk index +
  `/search?q=` + edit-triggers-reindex + `/feed.atom` +
  `/feed.json` + `/sitemap.xml` + `/robots.txt` + `/llms.txt` +
  `/git/{slug}` (Steps 3.1–3.4 shipped)
- **Phase 4** — git2 commit-on-edit + `/history` + `/blame` via
  gix + `/diff` + redb wikilink graph + `/backlinks` + blake3
  federation seam + MCP server via rmcp + smart-HTTP read-only
  Git remote + OpenAPI 3.1 specification (planned; Phase 4 plan
  document landed at v0.1.29; awaiting operator clearance of
  seven open questions before implementation begins)
- **Phase 5** — image and asset handling
- **Phase 6** — per-tenant shaping for Customer wikis
- **Phase 7** — federation and content-addressed retrieval
  against the blake3 substrate; IVC machinery
- **Phase 8** — disclosure-class linter that hardens the BCSC
  invariants into compile-time-equivalent checks

Phases 4–8 are *planned*; cautionary language applies per
`ni-51-102` and `osc-sn-51-721`. Material changes to the build
plan are recorded in the phase plan documents at
`docs/PHASE-N-PLAN.md` and in the workspace `CHANGELOG.md`.

The trajectory is approachable in standalone Tasks per the
project-knowledge cluster's session pattern. Phase 4 is the
largest remaining phase; Phases 5–8 are smaller in scope but
substantive in invariants (especially Phase 8's linter, which
formalises a substrate-level disclosure-grounding property).
