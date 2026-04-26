---
schema: foundry-doc-v1
document_version: 0.2.0
component: app-mediakit-knowledge
status: design — Phase 1 shipped 2026-04-26 (commit 722ae18); Phase 2 paused pending Master review of session-2 outbox
last_updated: 2026-04-26
session: 2
companion_docs:
  - docs/INVENTIONS.md
upstream_doctrine:
  - DOCTRINE.md claim #29 Substrate Substitution (v0.0.5)
  - DOCTRINE.md claim #30 Project Triad Discipline (v0.0.4)
  - conventions/disclosure-substrate.md
  - conventions/knowledge-commons.md
  - conventions/zero-container-runtime.md
  - conventions/citation-substrate.md
  - conventions/bcsc-disclosure-posture.md
  - conventions/compounding-substrate.md
---

# app-mediakit-knowledge — Architecture

The wiki engine for the PointSav knowledge platform. A single Rust
binary that serves a directory of CommonMark-with-wikilinks files
as a Wikipedia-shaped read-and-edit surface; landing point for the
[`disclosure-substrate`](../../../../conventions/disclosure-substrate.md)
convention; intended substrate substitution for MediaWiki under
[Doctrine claim #29 Substrate Substitution](../../../../DOCTRINE.md).

This document is the engineering design. The strategic positioning
that motivates the design landed in doctrine across workspace
v0.1.9–v0.1.10 — see `conventions/disclosure-substrate.md`,
`conventions/knowledge-commons.md` §3, and DOCTRINE claims #29 +
#30. Five candidate inventions emerging from session-2 research
are documented separately in [`docs/INVENTIONS.md`](docs/INVENTIONS.md).

## 0. Status snapshot

| Phase | State | Cluster |
|---|---|---|
| 1 — render | shipped (722ae18) | project-knowledge |
| 2 — edit | designed; paused pending Master review of session-2 outbox | project-knowledge |
| 3 — search + syndication feeds | designed | project-knowledge |
| 4 — Git sync + MCP server | designed | project-knowledge |
| 5 — auth + webhooks | designed | project-knowledge |
| 6 — wikilink resolution + portable identity | designed | project-knowledge |
| 7 — federation seam (blake3 + ActivityPub) | designed | project-knowledge |
| 8 — disclosure mode | per Master v0.1.10 ratification, **migrates to a future `project-disclosure` cluster** | project-disclosure (TBD) |
| 9 — Constrained-Constitutional Authoring (CCA) | proposed in session-2 outbox to Master; pending doctrine touch | project-disclosure (TBD) |

Session-2 also surfaced a conflict: `conventions/disclosure-substrate.md`
§5 retains the `mediawiki-action-api-shim` as a Phase 8 artefact,
while operator's session-2 direction is to drop the shim entirely
in favour of substrate-native API surfaces (MCP + REST + Atom +
JSON-LD + ActivityPub + Git remote + Markdown bulk export). The
engine ARCHITECTURE.md adopts the substrate-native surface set as
the working assumption (§3 phases below + §11 API surface set);
the convention conflict is briefed to Master via the cluster
outbox for resolution. The engine writes no Action API shim code
in any phase under this ARCHITECTURE.md.

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

| Layer | Crate | Phase |
|---|---|---|
| HTTP server | `axum` 0.8+ | 1 |
| Async runtime | `tokio` 1.x (`rt-multi-thread`, `signal`) | 1 |
| Middleware | `tower-http` | 1 |
| Markdown | `comrak` 0.29+ (wikilinks + GFM + syntect) | 1 |
| Templating | `maud` 0.27+ | 1 |
| Static assets | `rust-embed` 8.x | 1 |
| Logging | `tracing` + `tracing-subscriber` | 1 |
| CLI | `clap` 4 | 1 |
| Frontmatter | `serde` + `serde_yaml` | 1 |
| In-browser editor | CodeMirror 6 (embedded JS asset) | 2 |
| Search | `tantivy` 0.24+ | 3 |
| Feed generation | `atom_syndication` + `serde_json` | 3 |
| JSON-LD serialisation | `serde_json` (typed schema.org structs) | 3 (baseline from 1) |
| MCP server | `rmcp` (official Anthropic Rust SDK) | 4 |
| Git write | `git2` (libgit2-sys) | 4 |
| Git read | `gix` | 4 |
| Embedded KV | `redb` 4.1+ | 4 |
| Webhooks | `tokio` + `reqwest` | 5 |
| Auth | `tower-sessions` + `axum-login` + `argon2` + `openidconnect` | 5 |
| OpenAPI spec | hand-authored `openapi.yaml` 3.1 | 4 |
| AsyncAPI spec | hand-authored `asyncapi.yaml` 3.1 | 5 |
| Content hash | `blake3` 1.8+ | 7 (baseline from 4) |
| ActivityPub | `activitypub_federation` (or rolled) | 7 |
| WebFinger | small custom handler | 7 |
| Portable issuer DID | `did-key` / `did-web` (TBD) | 6 |

Single C dependency: libgit2-sys (via git2). Re-evaluate gix-only
write path annually — likely viable 2027–2028 per the gix
maintainer's published roadmap.

Phase 9 (CCA) brings a new tier of dependencies (constrained-decode
runtime, citation-graph query service, adversarial dual-AI gate,
W3C VC signing); these are scoped to the future `project-disclosure`
cluster and are not specified here. See
[`docs/INVENTIONS.md`](docs/INVENTIONS.md) §5.

## 3. Build phases

Each phase ships a working binary. No phase depends on later phases.
API surface items per phase are summarised here and tabulated in §11.

### Phase 1 — render one TOPIC ✓ (shipped 2026-04-26, commit 722ae18)

Smallest binary that validates the toolchain.

- `axum` server bound to `127.0.0.1:9090`
- `GET /wiki/{slug}` reads `<content_dir>/<slug>.md`, parses with
  comrak (wikilinks + GFM extensions), wraps body in maud chrome
- `GET /` lists files in `<content_dir>/`
- `GET /static/{*path}` serves embedded assets via rust-embed
- `GET /healthz` returns "ok"
- Content directory configured by `--content-dir` CLI flag or
  `WIKI_CONTENT_DIR` env var
- Bind address configured by `--bind` (default `127.0.0.1:9090`)
- 8/8 unit + integration tests passing; end-to-end smoke verified

Shipped: `src/{main,server,render,error,assets}.rs` + `static/style.css`
+ `tests/fixtures/content/topic-hello.md`. Approximately 400 lines.

### Phase 2 — edit endpoint + JSON-LD baseline

- `GET /edit/{slug}` — markdown source in a `<textarea>` upgraded
  by CodeMirror 6 (loaded as embedded static asset; no CDN)
- `POST /edit/{slug}` — atomic write to disk (temp + rename)
- `POST /create` — new page from title
- **JSON-LD `<script type="application/ld+json">` in every
  rendered page from Phase 2 onward.** Schema.org `TechArticle` /
  `DefinedTerm` profile. Cumulative — costs nothing in later
  phases, accumulates AEO-eligibility.
- Path-traversal hardening already in Phase 1 (`..` and nested
  paths rejected); extended for the write side
- New TOPIC fixtures committed alongside: redirect + bilingual
  sibling test; FLI-true rendering test (cautionary banner)

No revision history yet; the next save just overwrites. Phase 4
adds the Git layer that turns each save into a commit.

### Phase 3 — search + syndication feeds + crawler discovery

- `tantivy` index at `<state_dir>/search/`
- Index rebuilt on startup from a tree walk
- `GET /search?q=` with fuzzy + BM25
- Page edits trigger incremental re-index
- `GET /feed.atom` — RFC 4287 Atom feed of recent edits
- `GET /feed.json` — JSON Feed 1.1 equivalent
- `GET /sitemap.xml` — sitemaps.org standard
- `GET /robots.txt` — declared crawlers
- `GET /llms.txt` — emerging convention for LLM-readable site
  manifest (links to canonical TOPIC URLs + recommended JSON-LD
  ingestion paths)
- `GET /git/{slug}.md` — read-only Git-mirror endpoint exposing
  raw markdown source for `git clone`-style ingestion (Phase 4
  upgrades this to a full read-only Git remote)

### Phase 4 — Git sync + history + MCP server + OpenAPI 3.1

- `git2` opens the content dir as a Git repo
- Each `POST /edit` commits with author from session
- `GET /history/{slug}` reads `git log -- <slug>.md` via `gix`
- `GET /diff/{slug}?a={sha}&b={sha}` renders unified diff
- `GET /blame/{slug}` annotates lines with last-author + commit
- `redb` index for link graph (`<from_slug>` → `[<to_slug>]`)
  rebuilt on commit; `GET /backlinks/{slug}` reads from index
- `blake3` hash of every TOPIC at every commit, stored in `redb`
  keyed by `(slug, revision_sha)` — the federation-seam baseline
- **MCP server (`rmcp`) — first-class agent surface.** Resources
  (`wiki://topic/{slug}` per TOPIC), tools
  (`search_topics`, `get_revision`, `create_topic`, `propose_edit`,
  `link_citation`, `list_backlinks`), prompts (`/cite-this-page`,
  `/summarize-topic`, `/draft-related-topic`). Co-designed with
  the Doorman (`service-slm`) for auth and rate-limit policy. Per
  Doctrine §11 every Ring 1 service is an MCP server; the wiki
  is the customer-facing example.
- **Read-only Git remote** — substrate exposes itself as
  `git://wiki.example.com/{tenant}.git` for `git clone` style
  consumption. Pure read; writes go through the auth'd HTTP
  surface.
- **OpenAPI 3.1 spec** published alongside the binary as
  `openapi.yaml`. Includes JSON Schema for every endpoint payload
  and frontmatter shape. Serves as the source of truth from which
  client SDKs can be generated and from which (per Agent-1
  research, arxiv 2507.16044) MCP tool definitions can be derived.

This is where the engine becomes a wiki rather than a static-site
generator with edit. It's also where the source-of-truth inversion
becomes load-bearing — Git is the revision system, not a separate
revisions table.

### Phase 5 — auth + webhooks + AsyncAPI 3.1

- `tower-sessions` with `redb` session backend
- `axum-login` middleware for request auth context
- Local accounts via `argon2id` password hashing (OWASP params)
- OIDC SSO via `openidconnect` (Google / Microsoft / Okta /
  generic provider)
- Per-page ACLs in frontmatter (`read: public`, `edit: members`)
  — Phase 5.1
- **Webhook subscriptions** — customers register endpoints,
  receive POSTs on every commit (page-create, page-edit,
  page-delete, citation-update). Hash-signed payloads for
  verification.
- **AsyncAPI 3.1 spec** published as `asyncapi.yaml` covering
  webhooks + (Phase 7) ActivityPub Inbox/Outbox events.

### Phase 6 — wikilink resolution + backlinks + portable issuer identity

Slug normalisation rules (case-insensitive first letter,
spaces → underscores, MediaWiki-style canonicalisation — chosen
over Obsidian/Foam/Roam variants for migration compatibility).
Redirect chain support (frontmatter `redirect_to:`). Ambiguity
disambiguation. Backlinks panel rendered in the page chrome.

**Portable issuer identity (planned).** Per the AT Protocol /
W3C DID transplant in the session-2 wildcard research, every
substrate instance carries a tenant DID (`did:web:wiki.example.com`
or `did:foundry:<blake3>`). Cited TOPIC URLs survive provider
migration; an issuer can move from PointSav-operated hosting to
self-hosted hardware without breaking external citations.
WebFinger discovery at `/.well-known/webfinger` resolves the DID
to current canonical URLs. This is the substrate-portable-identity
seam; full DID-method selection is a Phase 6 design decision still
to land.

### Phase 7 — federation seam (planned)

- `GET /api/v1/page/{hash}` returns page content by content-address
  (uses the blake3 index from Phase 4)
- `GET /api/v1/peer/{url}/page/{hash}` proxies to another instance
  for federated read
- Git remotes are the federation primitive — no new protocol
- **ActivityPub Inbox/Outbox + WebFinger.** Each TOPIC becomes a
  fediverse `Article` Object; updates fan out via standard
  ActivityPub `Update`/`Create` activities to subscribed followers
  (Mastodon, other wikis, custom monitors). XWiki and Forgejo
  proved this works for wiki/forge content; we follow their lead.
- **Iroh-style content discovery (planned).** Each TOPIC is
  reachable as `iroh://{blake3}` via the BitTorrent mainline DHT
  without trusting a central index. Substrate-as-CDN with
  cryptographic guarantees. The engine ships an opt-in
  `--iroh-enable` flag; the federation primitive remains Git
  remotes.
- **`.well-known/api-catalog`** (RFC 9727) — single discoverable
  manifest pointing at openapi.yaml, asyncapi.yaml,
  mcp-manifest.json, schema.org JSON-LD context, ActivityPub
  endpoints, WebFinger location, sitemap. One URL gets a
  consumer the entire substrate API surface.
- **Claim-granular C2PA seam (Phase 7 planned, Phase 9 active).**
  Frontmatter and per-paragraph rendering carry C2PA-style
  assertion blocks binding text claims to citation registry IDs.
  C2PA's text vocabulary is immature (per session-2 research);
  the seam is in from Phase 7 so Phase 9 / project-disclosure
  cluster work can light it up without engine changes.

Decentralisation futures (TOPIC content-addressing extending to
on-chain anchoring for verifiable immutability) compose on top of
this seam without engine changes. Discipline is in the format
from day one; the on-chain integration is intended for v0.5.0+
and depends on operator decisions out of scope here.

### Phase 8 — disclosure mode → migrates to `project-disclosure` cluster

Per Master v0.1.10 ratification and `conventions/disclosure-substrate.md`
§6, Phase 8 work happens in a future `project-disclosure` cluster
that this crate's Phase 7 hands off to. Engine code in
`app-mediakit-knowledge/` continues to grow during Phase 8 (the
disclosure-mode modules — iXBRL extractor, OpenTimestamps anchor,
per-jurisdiction export adapters — live in this crate's `src/`),
but the cluster running the work is `project-disclosure`, with
its own Task sessions, its own trajectory capture, its own
adapter target. The handoff is purely organisational; the engine
is not split.

In-scope for Phase 8 (per the convention):

- iXBRL extractor for financial-statement frontmatter blocks
  (planned; mapping from a structured YAML block to ESEF-compliant
  iXBRL output for jurisdictions that mandate it)
- NI 51-102 / OSC 51-721 frontmatter linter — required fields,
  forward-looking-information labelling, third-party governance
  language checks (planned)
- OpenTimestamps anchoring on every commit (Bitcoin-anchored,
  free, open-source — research validated maturity, free public
  calendar servers, GitHub Actions integration available)
- RFC 3161 TSA support for jurisdictions requiring formal
  trusted-timestamping (planned, configurable TSA URL — research
  validated DigiCert / Sectigo / Entrust / GlobalTrust availability;
  eIDAS 2 Qualified Electronic Ledgers land EU-wide Dec 2026)
- MediaWiki XML import (`wiki import-mediawiki-xml`) — one-shot
  migration tool, no ongoing surface area. Operator-confirmed.

**Conflict surfaced — MediaWiki Action API shim.** The convention
§5 retains `mediawiki-action-api-shim` as a Phase 8 artefact;
operator's session-2 direction is to drop it. This ARCHITECTURE.md
adopts the operator's direction and **does not specify the shim
in any phase**. Substrate-native API surfaces (REST + MCP +
ActivityPub + JSON-LD + Atom/RSS + Git remote + Markdown bulk
export, all per §11 below) cover the "let others syndicate the
content" use case substantially better than the Action API
without needing to look like MediaWiki. Surfaced to Master via
cluster outbox 2026-04-26 session-2 for convention amendment.

### Phase 9 — Constrained-Constitutional Authoring (CCA) — proposed

The killer invention emerging from session-2 research. See
[`docs/INVENTIONS.md`](docs/INVENTIONS.md) §5 for the full
thinking; engineering summary here.

The substrate's TOPIC frontmatter schema, citation-ID syntax, FLI
label vocabulary, and BCSC structural-positioning rules are
compiled into a context-free grammar that the Doorman
(`service-slm`) injects as a logit constraint at AI decode time.
Constrained decoding is mature enough in 2026 (llguidance,
XGrammar, native structured output) that this is technically
viable. Every emitted TOPIC carries a machine-checkable
proof-of-grounding chain (citation IDs resolved against
`citations.yaml`, source content hashes pinned, adversary-AI
verdict signed as a W3C VC) committed inside the same Git commit
as the TOPIC. **The substrate refuses to render a TOPIC whose
proof chain doesn't verify.**

Engineering seams in this crate that Phase 9 builds on:
- Frontmatter schema (Phase 1, exists)
- Citation registry resolution (Phase 4, planned — wires up
  `citations.yaml` as a query surface)
- C2PA-style claim binding (Phase 7 seam, Phase 9 active)
- W3C VC signing (Phase 9 only — new dep)
- Doorman handshake (Phase 4 MCP server — provides the surface
  the constrained-decoding pipeline calls back through)

CCA depends on `project-disclosure` cluster scope to land
properly; this ARCHITECTURE.md captures the engine-side seams so
Phase 7 doesn't accidentally close them off. Doctrine touch
proposed via session-2 outbox to Master (extension of
disclosure-substrate.md or new claim #31).

## 4. Process model

Single binary, runs as a systemd unit. No Docker, no containers
(per [`conventions/zero-container-runtime.md`](../../../../conventions/zero-container-runtime.md)).

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
│   ├── slugs.toml                     redirect / alias rules
│   └── did.toml                       Phase 6 — tenant DID config
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
├── kv.redb                            link graph + sessions + blake3 index
├── timestamps/                        OpenTimestamps proofs (Phase 8)
└── activitypub/                       Inbox/Outbox queues (Phase 7)
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
# Phase 8+ additions (planned):
published_at: 2026-04-26T03:44:20Z   # Git timestamp + OpenTimestamps anchor
valid_at: 2026-04-26                  # date the information applies to
xbrl_taxonomy: us-gaap-2026           # for disclosure_class: financial
constitution: bcsc-ni-51-102          # Phase 9 — per-jurisdiction rule pack
---
```

All fields optional except `schema`, `document_version`, `title`.
The linter (Phase 8) checks that pages classified `disclosure_class:
financial` have an iXBRL block; that pages with `forward_looking:
true` carry the cautionary language patterns; that any third-party
governance claims appear with documented `cites:` resolution. The
Phase 9 (CCA) constrained-decoding pipeline compiles this schema
into the CFG used to constrain AI authoring.

## 7. Compatibility surface — substrate-native, not shim

The MediaWiki ecosystem moat is real (~1,500 extensions, hundreds
of thousands of templates, pywikibot, Wikidata) but is best
addressed by:

- **`wiki import-mediawiki-xml`** (Phase 8) — one-shot migration
  tool; walks a `Special:Export` XML dump, runs each revision
  through a wikitext-to-Markdown converter (`pandoc` shelled out,
  with a pluggable template-expansion shim), writes one file per
  page, one commit per revision, preserves authorship and
  timestamp. **Operator-confirmed in scope.**

- **No MediaWiki Action API shim.** Operator's session-2 direction
  drops the shim entirely. Pywikibot users who want to migrate
  their automation either (a) rewrite their bots to call our
  REST/MCP surface (which is structurally cleaner — see Agent-1
  session-2 research), or (b) write a thin Python translation
  layer out of tree. The substrate does not pretend to be
  MediaWiki. Conflict with `conventions/disclosure-substrate.md`
  §5 surfaced to Master via cluster outbox 2026-04-26 session-2.

The substrate is **what a wiki engine looks like when it is born
MCP-native, federation-native, AI-citation-native** — not a
MediaWiki successor in the lineage sense.

## 8. Out of scope (engine)

- Real-time collaborative editing (CRDT/OT) — adjacent useful
  feature; intended for v0.4.x via `yrs` (Yjs Rust port); not in
  Phase 1–7. Note: the session-2 wildcard research explicitly
  rejects CRDT-shaped disclosure records (cannot say "issuer
  asserted X at time T" with concurrent edits and no authority
  decision). CRDT may land for *draft co-authoring inside a
  cluster*; never for the disclosure-tier surface.
- Mobile-first editor UX — the desktop editor is the proof; mobile
  follows after v0.3.x
- Multi-tenancy at engine level — one engine instance serves one
  content tree; tenancy is at the deployment layer (multiple
  instances per tenant)
- AI integration of any kind in the engine itself — the engine
  renders, edits, and serves Markdown. Optional-AI capabilities
  (suggest-as-you-write, semantic search, CCA constrained-decode
  authoring) are intended to land via the Doorman (`service-slm`)
  as a Ring 3 adapter against the same Markdown corpus, not by
  coupling the engine to any model. The MCP server (Phase 4) is
  the contract.

## 9. Testing approach

- Unit tests on the renderer (wikilink parsing, frontmatter
  handling, slug normalisation)
- Golden-file tests on the full HTML output against fixture
  Markdown
- Integration tests against `tower::ServiceExt::oneshot` for
  HTTP routes
- A minimum fixture corpus under `tests/fixtures/content/` —
  enough TOPICs to exercise wikilinks, redirects, frontmatter
  variation, FLI rendering, bilingual siblings
- No reliance on external services at any test phase. Phase 4
  Git tests use `tempdir` repos; Phase 5 auth tests stub OIDC
  via mock provider; Phase 7 ActivityPub tests use loopback
  peer instances.

## 10. Versioning

Per `~/Foundry/CLAUDE.md` §7: `MAJOR.MINOR.PATCH`. Patch increments
per accepted commit; minor per feature milestone (each Phase
above is a minor); major per breaking change. Phase 1 lands at
crate version `0.1.0`; Phase 2 at `0.2.0`; etc. SSH-signed tags;
Version: trailer on commit messages.

## 11. API surface set

The full external API surface across all phases. Each surface is
independent — consumers pick what fits. Spec artefacts ship
alongside the binary.

| Surface | Purpose | Audience | Spec artefact | Phase |
|---|---|---|---|---|
| **REST/JSON** (axum) | CRUD + search + admin | Apps, scripts | `openapi.yaml` 3.1 | 1+ |
| **JSON-LD in rendered HTML** | LLM crawler comprehension; AEO | LLM crawlers | schema.org `TechArticle` / `DefinedTerm` | 2+ |
| **Atom + JSON Feed** | Change syndication | Readers, bots, mirror instances | RFC 4287 / JSON Feed 1.1 | 3+ |
| **Sitemap.xml + robots.txt + llms.txt** | Crawler discovery | Crawlers | sitemaps.org + emerging llms.txt | 3+ |
| **Read-only Git remote** | Treat the wiki as a repo | Devs, CI | git protocol v2 | 3+ (basic), 4+ (full) |
| **MCP server** (`rmcp`) | Agent/LLM access to TOPICs | Claude/GPT/Gemini agents, internal Doorman | `mcp-manifest.json` | 4+ |
| **Webhooks** | Push change events to subscribers | Customer integrations | `asyncapi.yaml` 3.1 | 5+ |
| **WebFinger + DID** | Portable issuer identity | Federation peers | RFC 7033 + W3C DID | 6+ |
| **ActivityPub Inbox/Outbox** | Federation of TOPIC objects | Other wikis, Mastodon | W3C ActivityPub | 7+ |
| **Content-addressed read** (`/api/v1/page/{hash}`) | Hash-verified retrieval | Auditors, peers | OpenAPI extension | 7+ |
| **`iroh://{hash}`** (opt-in) | Decentralised peer discovery | Peers | iroh / libp2p | 7+ |
| **Markdown bulk export** | Offline / grep-able snapshot | Operators, audit | filesystem layout doc | 1+ |
| **`.well-known/api-catalog`** | Discoverability of all the above | Anyone | RFC 9727 | 7+ |

Surface set chosen to:

- replace, not shim, the MediaWiki Action API
- expose every consumer pattern (humans, browsers, bots, agents,
  crawlers, peer wikis, regulators, auditors) on its native protocol
- deliver each surface as machine-readable spec from day one
- preserve substrate sovereignty (every surface terminates inside
  the operator-controlled binary; no SaaS index, no vendor cloud
  dependency)

Rationale and prior-art positioning per session-2 research-agent
report on API surfaces (see §13 source ledger).

## 12. Inventions catalogue

Five candidate inventions surfaced from session-2 research,
documented in [`docs/INVENTIONS.md`](docs/INVENTIONS.md):

1. **Substrate-enforced AI grounding** — the substrate refuses to
   render unsourced AI claims; FLI-labelled, citation-resolved,
   constitutionally bound per tenant/jurisdiction
2. **Content-addressed federated AI adapters** — adapters live in
   Git remotes alongside content, blake3-addressed, compose at
   request time per cluster+tenant+role
3. **Two-clock continuous disclosure with cryptographic anchors**
   — published_at (anchored) + valid_at (frontmatter) for forensic
   regulatory time-travel
4. **Disclosure-Diff as Signed Artefact + Subscriber Proof-of-Receipt**
   — diffs and receipts are first-class disclosure objects with
   embedded cryptographic proof
5. **Constrained-Constitutional Authoring (CCA)** — the substrate's
   schema becomes a CFG constraining AI decoding; AI cannot emit
   a TOPIC that fails the schema; every emitted TOPIC carries a
   machine-checkable proof-of-grounding chain. **The killer
   invention.**

INVENTIONS.md carries the prior-art positioning, novelty kernel,
engineering seams, citations, and rationale for each.

## 13. Research source ledger (session-2)

Five parallel research-agent reports informed the design. Cited
URLs are reproduced inline in `docs/INVENTIONS.md`; this is the
high-level provenance.

| Report | Topic | Key finding |
|---|---|---|
| Agent 1 | API surfaces 2026 | MCP is the de-facto 2026 standard (~17,500 indexed servers, 97M monthly SDK downloads); MediaWiki Action API is legacy per upstream; ship REST + MCP + Atom + JSON-LD + ActivityPub + Git + Markdown bulk export as substrate-native surfaces |
| Agent 2 | Substrate-enforced AI grounding | Anthropic Constitutional AI is model-layer; Vectara / Perplexity score citations but don't refuse-to-render; C2PA does asset-level not claim-level; substrate-layer enforcement is structurally novel as a composed system |
| Agent 3 | Federated AI adapters | Hugging Face Xet does content-addressed storage; S-LoRA / Lorax do multi-LoRA serving; CivitAI is an adapter marketplace; the composition (adapters federate over Git remotes, compose at request time, owned by substrate operator) has no published precedent |
| Agent 4 | Two-clock cryptographic disclosure | OpenTimestamps + RFC 3161 TSA + Sigstore Rekor are commodified; no integrated continuous-disclosure substrate ships the composition; SolarWinds enforcement validates the demand for prospective continuous proof |
| Agent 5 | Adjacent inventions wildcard | CCA emerges as the killer composition (substrate-grammar-constrained AI decoding + citation-graph proof-of-grounding committed alongside TOPIC); plus AT Protocol DID transplant for portable identity, Iroh for content discovery, W3C VC 2.0 for credentials, claim-granular C2PA for text |

## 14. References

- [DOCTRINE.md](../../../../DOCTRINE.md) — constitutional charter
- [CLAUDE.md](../../../../CLAUDE.md) — workspace operational guide
- [conventions/disclosure-substrate.md](../../../../conventions/disclosure-substrate.md) — landing point for this engine's strategic positioning (note: §5 retains `mediawiki-action-api-shim`; conflict surfaced to Master 2026-04-26 session-2)
- [conventions/knowledge-commons.md](../../../../conventions/knowledge-commons.md) §3 Three-Tier Contributor Model
- [conventions/zero-container-runtime.md](../../../../conventions/zero-container-runtime.md) — systemd, no Docker
- [conventions/citation-substrate.md](../../../../conventions/citation-substrate.md) — frontmatter discipline
- [conventions/bcsc-disclosure-posture.md](../../../../conventions/bcsc-disclosure-posture.md) — Phase 8 source
- [conventions/compounding-substrate.md](../../../../conventions/compounding-substrate.md) — Three-Ring + Doorman composition
- [conventions/trajectory-substrate.md](../../../../conventions/trajectory-substrate.md) — every commit feeds the cluster adapter
- Cluster manifest: `~/Foundry/clones/project-knowledge/.claude/manifest.md` (triad declaration backfilled per Doctrine v0.0.4)
- Companion design doc: [`docs/INVENTIONS.md`](docs/INVENTIONS.md)
