---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-knowledge
target_repo: content-wiki-documentation
target_path: architecture/   # candidates: architecture/, governance/, applications/ — project-language decides
target_filename: substrate-native-compatibility.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-04-27T16:55:00Z
authored_by: task-project-knowledge (session 619abe3eff24497e)
authored_with: opus-4-7
references:
  - DOCTRINE.md claim #29 (Substrate Substitution)
  - DOCTRINE.md claim #16 (Optional Intelligence Layer)
  - DOCTRINE.md claim #25 (Citation Substrate)
  - DOCTRINE.md claim #34 (Two-Bottoms Sovereign Substrate)
  - conventions/disclosure-substrate.md §5 (substrate-native API surface set)
  - conventions/disclosure-substrate.md §5.1 (compat surface enumerated)
  - conventions/disclosure-substrate.md §6 (continuous-disclosure cadence)
  - conventions/disclosure-substrate.md §8 (Substrate-Enforced AI Grounding, CCA Invention A)
  - conventions/three-ring-architecture.md
  - conventions/citation-substrate.md
  - conventions/bcsc-disclosure-posture.md
  - conventions/knowledge-commons.md (Three-Tier Contributor Model)
  - vendor/pointsav-monorepo/app-mediakit-knowledge/ARCHITECTURE.md §7 (compatibility surface)
  - vendor/pointsav-monorepo/app-mediakit-knowledge/ARCHITECTURE.md §11 (API surface set)
  - https://www.mediawiki.org/wiki/API:Main_page
  - https://www.mediawiki.org/wiki/Help:Export
  - https://en.wikipedia.org/wiki/Help:Wikitext
  - https://commonmark.org/
  - https://datatracker.ietf.org/doc/html/rfc4287   # Atom
  - https://www.jsonfeed.org/version/1.1/
  - https://www.sitemaps.org/protocol.html
  - https://llmstxt.org/
  - https://schema.org/
  - https://c2sp.org/tlog-tiles
  - ni-51-102
  - osc-sn-51-721
  - cloud-act-2018
notes_for_editor: |
  This is doctrine-grade content. Doctrine claim #29 (Substrate Substitution) is the
  load-bearing reference; preserve every claim reference verbatim during refinement.

  The Action API drop is the case study. The pattern (substrate-native > shim) generalises
  to other substrate-substitutions named in the disclosure-substrate convention §6, including
  Q4 Inc disclosure-distribution platforms, MediaWiki itself (already covered here),
  Salesforce-class CRM substitution (project-people territory), and SAP-class corporate-records
  substitution (project-system territory). project-language can decide whether to keep §8
  (generalisation) in this TOPIC or split it to a separate TOPIC.

  §2 (ecosystem-moat reframing) is the audience hook for a financial-community reader who
  associates "compatibility" with "ecosystem reach" rather than with "interface mimicry."
  The reframing matters because compatibility-as-ecosystem-reach is exactly what xml-dump
  and URL conventions provide; compatibility-as-interface-mimicry is what the Action API
  shim would have provided AND the post-launch maintenance and security audit it would
  have demanded.

  §6 (verify://) is forward-looking — the URL scheme is named in UX-DESIGN.md §4.8 but
  not implemented as of v0.1.29. Apply BCSC discipline if expanding the section.

  §7 (disclosure posture as compatibility lens) is substantive doctrine work. The frame is:
  a wiki engine's compatibility surface choices are continuous-disclosure choices in
  disguise. Each surface kept or dropped affects what claims the substrate makes to the
  outside world and what audit obligation it accepts. project-language: this section is
  load-bearing for the BCSC posture; preserve.

  Repetition allowed: §3 and §4 cover overlapping ground (the kept-vs-dropped table); §3
  emphasises what was kept and why, §4 emphasises what was dropped and why. project-language
  picks one or merges per audience.

  target_path candidates listed.
---

# Substrate-native compatibility — why the Action API shim was dropped

## §1 The choice

The PointSav engineering wiki at `documentation.pointsav.com` is a
substrate-native wiki. It accepts a one-shot MediaWiki XML import,
serves URLs in MediaWiki's familiar `/wiki/{slug}` shape, accepts
Wikipedia-style `[[wikilink]]` and `[^footnote]` syntax, and stops
there. It does not implement MediaWiki's Action API, does not
support MediaWiki templates, does not run a bot framework
compatible with pywikibot, and does not provide a write surface
that mimics MediaWiki's REST endpoints.

This is the substrate-native posture per Doctrine claim #29
(Substrate Substitution): when an existing platform's role is
substituted by the substrate, the substrate replicates *structural
compatibility* (the surfaces a reader or external integrator
encounters) without replicating *interface mimicry* (the API
shape an internal-system integration would consume). The
distinction is load-bearing.

The MediaWiki Action API shim was scoped during the wiki engine's
session 1 design pass, drafted in `conventions/disclosure-substrate.md`
§5 at workspace v0.1.10, and dropped at v0.1.14 after the cluster
session 2 conflict surfaced the maintenance burden and the
disclosure-posture risk the shim would have introduced. This TOPIC
covers the rationale.

## §2 Ecosystem moat — what compatibility actually buys

The MediaWiki ecosystem is real. Approximately 1,500 extensions,
hundreds of thousands of templates, a mature bot framework
(pywikibot), Wikidata integration, the Wikimedia Commons asset
library, and a deep archive of operational practice across
Wikipedia, Wiktionary, Wikiquote, and a long tail of self-hosted
installations. A wiki engine that promises MediaWiki compatibility
inherits a portion of that moat by reference.

But the moat is in two parts, and the parts have different costs
to participate in.

The first part is **reader and external-integrator structural
compatibility**: a reader who knows Wikipedia's URL pattern and
markup can navigate and edit the wiki without retraining; an
external system that ingests sitemap.xml or follows wikilinks
discovers the corpus without bespoke adapters. This part is
*low-cost to provide* — `/wiki/{slug}`, `[[wikilink]]`, and
`[^1]` are conventions the substrate adopts because they are
useful, not mimicry.

The second part is **internal-system interface mimicry**:
extensions that read and write through the Action API, bots that
authenticate against MediaWiki's login flow, templates that
expand server-side using the MediaWiki parser. This part is
*high-cost to provide* — every API endpoint mimicked is an
interface that must be maintained against MediaWiki's evolution,
hardened against MediaWiki's known attack surfaces, and audited
under whatever compliance posture the substrate has committed to.

The substrate's choice is to participate in the first part fully
and decline the second part deliberately. The cost calculus
favours decline:

- Reader / integrator ecosystem effects scale via *conventions*,
  which are stable across a decade. The substrate adopts the
  convention once and benefits indefinitely.
- Internal-system extension ecosystem effects scale via *interfaces*,
  which evolve with MediaWiki releases. The substrate would inherit
  a maintenance burden that grows with MediaWiki's ecosystem
  velocity, not the substrate's.
- The disclosure posture (per `conventions/bcsc-disclosure-posture.md`
  and `conventions/disclosure-substrate.md`) makes every interface
  the substrate exposes a continuous-disclosure surface. An Action
  API shim would commit the substrate to disclosure-grounding
  every Action API endpoint's behaviour, which is a moving target.

## §3 What was kept

Four substrate-compatibility surfaces are preserved, all in the
*low-cost-to-provide* category:

**The xml-dump import path.** A future `import-mediawiki-xml` tool
(planned, named in `conventions/disclosure-substrate.md` §5)
consumes Special:Export-style XML dumps and emits the wiki's
markdown + frontmatter shape. The migration is one-shot per
corpus; no live MediaWiki API runs alongside the substrate
afterward. Importing 30 articles, 3,000 articles, or 30,000
articles is the same operational shape — read the dump, write
markdown files, commit. The pattern composes with version control
naturally.

**URL conventions.** `/wiki/{slug}` matches MediaWiki's URL layout.
External sites that link to articles via this URL pattern
continue to resolve without 404s. The route is one line in the
wiki engine's axum router and costs nothing to maintain. Adopting
the convention is a low-cost ecosystem-reach choice.

**Wikilink syntax.** `[[slug]]` and `[[slug|display text]]`
match Wikipedia's markup. Contributors who know Wikipedia
recognise the form. The wiki's renderer parses wikilinks at
render time, resolves them against the in-memory link graph
(the redb-backed graph lights up in Phase 4), and emits HTML
with red-link styling for unresolved targets. The implementation
is one renderer extension and stays stable across releases.

**Footnote syntax.** `[^1]` matches CommonMark's footnote
extension. The bibliography resolves footnotes against the
article's frontmatter `references:` list. The implementation
ships with `comrak` and adds no maintenance burden specific to
the substrate.

These four surfaces give the substrate's wiki the
reader / integrator ecosystem reach a substrate-substituted MediaWiki
deployment expects. None of them require running a MediaWiki
parser, a MediaWiki API, or a MediaWiki extension framework.

## §4 What was dropped

Three surfaces in the *high-cost-to-provide* category were
declined:

**The MediaWiki Action API shim.** The shim was scoped at
workspace v0.1.10 in `conventions/disclosure-substrate.md` §5 as
an interface that would have replicated `?action=parse`,
`?action=edit`, `?action=query`, `?action=login`, and the
remaining Action API surface against the substrate's wiki engine.
At v0.1.14 the shim was removed from scope. The reasoning:

- *Maintenance scales with MediaWiki's velocity.* Every Action API
  endpoint MediaWiki ships, deprecates, or modifies generates a
  shim-side maintenance event. The substrate has no leverage over
  this velocity.
- *Compliance audit scales with the API surface.* The disclosure
  posture per `conventions/bcsc-disclosure-posture.md` requires
  every interface the substrate exposes to the public to be
  disclosure-grounded. The Action API shim would have multiplied
  the audit surface by an order of magnitude with no commensurate
  ecosystem benefit on the substrate's actual customer base.
- *Substrate-native interfaces cover the use cases.* The wiki's
  route surface (`/wiki/{slug}`, JSON-LD, Atom, JSON Feed,
  sitemap, llms.txt, raw markdown via `/git/{slug}`,
  `/search?q=`, `POST /edit/{slug}`) covers what the Action API
  shim would have served, without committing the substrate to
  Wikipedia's API contract.

**MediaWiki templates and parser functions.** The wiki's renderer
is `comrak` (CommonMark) plus PointSav-specific extensions for
wikilinks, footnotes, table of contents, and section anchors. It
is not a MediaWiki parser. Templates do not expand server-side.
The workaround for content that would be a template in MediaWiki
is markdown partials, inlined by the contributor at edit time;
the substrate accepts the duplication cost in exchange for a
deterministic rendering pipeline that can be audited without
parsing a Turing-complete template language at render time.

**The pywikibot ecosystem.** The substrate's automation path is
the workspace's existing tooling — `bin/commit-as-next.sh`, the
trajectory-substrate corpus capture per Doctrine claim #15, the
mailbox protocol per `CLAUDE.md` §11, the new drafts-outbound
input ports per `conventions/cluster-wiki-draft-pipeline.md`. None
of these implement the pywikibot interface. A bot author migrating
from pywikibot to the substrate's tooling re-implements against
the new interfaces; the substrate accepts the migration cost in
exchange for keeping its automation path coherent with the rest
of the workspace's session model.

## §5 The substrate-native API surface set

What the Action API shim would have served, the substrate's
native interfaces serve coherently:

| Action API need | Substrate-native interface |
|---|---|
| `?action=parse` (render markdown to HTML) | `GET /wiki/{slug}` (rendered HTML directly) |
| `?action=raw` (raw wikitext) | `GET /git/{slug}` (raw markdown) |
| `?action=edit` (write articles) | `POST /edit/{slug}` (atomic write with frontmatter validation) |
| `?action=query&list=allpages` (enumerate articles) | `GET /sitemap.xml` (sitemaps.org compliant) |
| `?action=query&prop=links` (link graph) | `GET /backlinks/{slug}` (Phase 4) |
| `?action=query&prop=revisions` (history) | `GET /history/{slug}` (Phase 4) |
| `?action=opensearch` / `?action=query&list=search` | `GET /search?q=` (Tantivy / BM25) |
| `?format=json` (machine-readable per article) | JSON-LD `<script type="application/ld+json">` block in every article's `<head>` |
| RSS / Atom feeds | `GET /feed.atom` + `GET /feed.json` |
| `?action=expandtemplates` (template expansion) | not provided; the substrate's renderer does not expand templates |
| `?action=login` / authentication | not provided over HTTP; authenticated edits use the Mutual Bidirectional Auth (MBA) path per `app-console-help` and `app-mediakit-marketing` (separate substrate, not API-shim-replicated) |

The interfaces compose with the substrate's other invariants. The
JSON-LD is generated from the article's frontmatter by the same
code path that emits the article HTML, so the structured data
cannot drift from the rendered content. The Atom feed shares
its data source with the JSON Feed, the sitemap, and the index
page; a content tree change updates all four atomically. The
search backend (Tantivy) reads the same content tree as the HTML
renderer; no separate write path needs to keep the search index
synchronised with the article store, because the article store is
the file system.

These are properties an Action API shim could not have provided
without major engineering: MediaWiki's Action API is a layer over
a database that stores parsed wikitext; the substrate's interface
set is a layer over a markdown file system. The two have
different consistency models, different audit surfaces, and
different compliance postures.

## §6 The `verify://` URL scheme

A future substrate-specific URL scheme — `verify://{citation-id}`
— resolves a citation reference to its verifiable source through
the substrate's verification path, not through public DNS. The
scheme is named in `UX-DESIGN.md` §4.8 and is *planned* for Phase
7 of the wiki engine; it is not implemented as of v0.1.29.

The motivation: a `[citation-id]` in an article is a structured
reference that the substrate can resolve to an authoritative
source through the citation registry at `~/Foundry/citations.yaml`.
The registry maps the ID to a (title, URL, optional clause
reference) tuple, and a future Information Verifiability Citation
(IVC) machinery (per Doctrine claim future-#) would harden the
resolution to a cryptographically-verifiable proof of provenance.

The `verify://` scheme is the URL surface that lets the IVC
machinery be addressed. A user (or a crawler, or a peer
substrate) follows `verify://ni-51-102` and the substrate returns
the verified provenance record for `ni-51-102`, which the user
can compare to the source the citation registry pinned. The
scheme is substrate-native; no DNS substitute is needed because
the registry is local to the substrate.

This is forward-looking; the planned mechanism. Cautionary
language applies per `ni-51-102` and `osc-sn-51-721`. The
reasonable basis is the citation-registry substrate already
operating at v0.1.29 plus the IVC scaffolding visible in the
wiki engine's masthead band placeholder (Phase 1.1 chrome). The
material assumption is that the IVC machinery ratifies as a
Doctrine claim before Phase 7 implementation begins.

## §7 Disclosure posture as compatibility lens

The deeper reason the substrate-native posture wins is that the
wiki engine's compatibility-surface choices are
continuous-disclosure choices in disguise. Every interface the
substrate exposes to the public commits the substrate to a
disclosure obligation under `conventions/bcsc-disclosure-posture.md`
and the workspace's NI 51-102 + OSC SN 51-721 compliance posture.
What the substrate doesn't expose, it doesn't need to disclose
about.

A few concrete cases:

**An Action API shim that returned `?action=query&prop=revisions`
would have committed the substrate to a continuous-disclosure
representation of every article's revision history.** The
substrate already commits to this through its git history (every
edit is a signed commit, every commit is a published claim). But
a parallel Action API representation would have introduced a
*second* canonical record that needs to stay in sync with the
git history; any drift is a disclosure failure. The substrate
declines the shim and keeps git canonical.

**An Action API shim that returned `?action=parse` server-side
would have committed the substrate to a server-side rendering
contract independent of the markdown source.** A reader could
fetch the API result and assert "this is what the article said at
this moment." If the rendered HTML drifts from the markdown
source (because the renderer has a bug, because the parser
function expanded differently, because of an Action API caching
layer), the substrate has a disclosure conflict. The substrate
declines the shim and keeps the rendered HTML a deterministic
function of the markdown source plus the renderer version.

**An Action API shim that supported `?action=edit` would have
committed the substrate to an authenticated write path with
weaker provenance than the substrate's edit surface.** The
`POST /edit/{slug}` route requires MBA-verified authorship per
SYS-ADR-19 and Doctrine claim #15 trajectory-substrate; a
MediaWiki-style API edit would have permitted token-bearer auth
over a session that does not bind the editor's identity to a
human-verified key chain. The substrate declines the shim and
keeps the trajectory-substrate guarantee.

The pattern: every interface the substrate doesn't replicate is
an obligation it doesn't assume. The substrate keeps the
interfaces it can disclose-ground; declines the rest.

## §8 The pattern generalises

Substrate substitution applies beyond MediaWiki. The
disclosure-substrate convention §6 enumerates additional cases the
substrate addresses with the same posture:

- **Q4 Inc and similar disclosure-distribution platforms.** The
  substrate's continuous-disclosure record (signed git history
  on `documentation.pointsav.com`) is itself the disclosure
  channel; no Q4-style platform integration is required.
  Investors and counterparties retrieve disclosure documents
  directly from the substrate.
- **Salesforce-class CRM platforms.** The project-people cluster
  territory; the substrate's people record is canonical. No
  CRM-style API mimicry is in scope.
- **SAP-class corporate-records platforms.** The project-system
  cluster territory; the substrate's corporate-records ledger
  is canonical via Doctrine claim #34 (Two-Bottoms Sovereign
  Substrate). No SAP-API mimicry.
- **Vendor SaaS for document storage.** The substrate stores
  documents in version-controlled markdown trees; no SaaS
  integration is needed for storage. (Specific tools may
  integrate via the substrate's git-remote interface where
  useful.)

The common thread: the substrate replicates *structural
compatibility* where it pays (a markdown tree readable by humans
and by other tools; a `/wiki/{slug}` URL pattern that resolves
external links; an Atom feed any RSS reader consumes) and
declines *interface mimicry* where the maintenance and disclosure
costs would exceed the ecosystem benefit. Each substitution is
analysed under the same lens: what does the existing platform's
interface obligate the substrate to, and what does the substrate
gain in return.

The Action API drop is the canonical case study. The pattern is
load-bearing across the substrate's broader trajectory.
