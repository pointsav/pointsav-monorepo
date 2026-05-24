---
artifact: design-research
status: draft
type: DESIGN-RESEARCH-MARKET-POSITIONING
topic: app-mediakit-knowledge — strategic positioning vs MediaWiki, Hugo, Q4 Inc; leapfrog 2030 direction
created: 2026-05-24
owner: totebox@project-knowledge
destination: project-design
foundry-draft-v1: true
research-trail:
  commissioned-by: operator (2026-05-24 session)
  method: agent-web-research
  sources-cited: 11
  bcsc-review: required before public use
---

# Market Positioning Research — app-mediakit-knowledge

## 1. Hugo Gap — What It Does Well, What It Cannot Do

Hugo excels at the things that matter for static documentation: sub-second build times, deterministic
output, zero runtime attack surface, Git-native content, and mature themes (Docsy, Doks) that give
documentation sites clear typographic hierarchy, versioned page trees, and working search via Lunr
or Algolia offload. The result feels complete to a reader.

**What Hugo fundamentally cannot provide (6 structural gaps):**

1. **Server-side authentication and role-gated pages.** Hugo has no concept of a logged-in user.
   Every workaround (Netlify Identity, OAuth proxies) is an external dependency.
2. **Live full-text search over dynamic content.** Lunr indexes are rebuilt at build time. Content
   added between builds is invisible to search. Tantivy BM25 served by the engine is live by definition.
3. **Edit-in-place workflow.** Authors must clone, edit, commit, push, wait for CI rebuild. No
   browser-based edit queue, review state, or diff preview.
4. **Revision history surfaced to readers.** Git log exists but Hugo does not expose per-page change
   history to site visitors without bespoke templating and a CI pipeline.
5. **Structured citation and claim linking.** No native mechanism to cross-reference a claim to a
   source record. Claim-layer citations require an application layer.
6. **Bilingual routing with content parity tracking.** Hugo supports multilingual builds but cannot
   tell you which `/es/` pages are out of sync with their English counterparts at runtime.
7. **MCP/API surface for AI agents.** Hugo output is HTML files. No JSON-RPC endpoint, no structured
   document retrieval, no way for an agent to fetch a canonical claim without scraping.

**The "never feels complete" diagnosis:** Hugo is static. Static = no server-side capability.
`app-mediakit-knowledge` provides all seven capabilities without giving up the flat-file + Git-native
content model. That is the differentiation worth communicating.

---

## 2. MediaWiki — Why It Is Not the Future

**Technical debt:** MediaWiki was architected in 2002 around PHP 4 and MySQL. The parser is a bespoke
Wikitext compiler with 20 years of edge cases. Replacing it (the Parsoid project) took a decade and
is still incomplete in extension coverage.

**Operational burden:** A production MediaWiki installation requires PHP-FPM, a relational database,
Memcached or Redis, Elasticsearch or CirrusSearch, and Parsoid as a separate service. Five runtime
processes for a documentation site.

**Regulated-data liabilities:** MediaWiki's revision table stores all historical content in the
database; purging a revision to comply with a legal hold requires direct database surgery. No native
audit export, no structured content API that returns a content hash, no signing of revisions. For
BCSC continuous-disclosure posture, the absence of a tamper-evident revision chain is a structural
disqualifier.

**Strategic:** Wikimedia Foundation is the only organisation that benefits at the scale MediaWiki was
designed for. Wiki.js and XWiki are capturing the enterprise migration; both still require a database
server and have no Git-native content model.

---

## 3. Q4 Inc — The Gap a Sovereign Wiki Fills

Q4's Capital Connect platform covers the public-facing IR surface well: WCAG 2.2 AA IR websites,
SOC 2 Type II infrastructure, MNPI-aware disclosure workflows, earnings event tooling, and investor CRM.
Its differentiation is speed-to-disclose and capital-markets analytics.

**What Q4 does not fill:**

- **Internal regulatory knowledge base.** Q4 manages what a company publishes to investors; it does
  not manage the internal documentation of *how disclosure decisions are made* — the policies,
  precedents, board resolutions, and legal opinions that sit behind a press release.
- **Continuous-disclosure knowledge layer.** NI 51-102 / OSC SN 51-721 require a durable record of
  material fact documentation. Q4 has no citation-linked wiki that connects a public claim to its
  internal evidentiary basis.
- **Multilingual regulatory content parity tracking.** Q4 IR websites support multiple languages but
  content parity (English claim updated, French page stale) is not tracked programmatically.
- **Machine-readable claim export.** Q4 exposes analytics but not a structured JSON endpoint that
  returns the provenance chain for a specific claim on the IR site.

**The complement play:** `app-mediakit-knowledge` serves as the internal evidence layer that supports
what Q4 publishes externally. The two are not competitors on the same page; they are adjacent in the
disclosure stack. A Woodfine/PointSav pitch to a public-company client positions the wiki as the
*behind-the-firewall compliance knowledge base* that Q4 does not provide.

---

## 4. Leapfrog 2030 — Capabilities That Create Lock-In

The current stack is already ahead of MediaWiki on every axis. The capabilities that convert "ahead"
into "lock-in for regulated clients":

1. **Tamper-evident revision signing.** Each committed revision gets a deterministic content hash
   stored in an append-only table. A regulator can independently verify that the published version
   of any claim has not been altered since a given timestamp. No current wiki engine provides this
   without a blockchain overlay.

2. **Citation graph export (SBOM-equivalent for claims).** A structured export of every claim, its
   citation dependencies, and the last-verified timestamp — machine-readable, diff-able between
   releases. Documentation equivalent of a software bill of materials.

3. **MCP-native AI integration without data egress.** The engine already speaks MCP. Lock-in comes
   from training client IR teams to use AI agents that query the local MCP endpoint rather than
   sending content to a cloud LLM. Once workflows depend on the local MCP surface, migration cost
   is high.

4. **Regulatory diff alerting.** When a source document (a cited regulation, an OSC notice) changes,
   the engine flags every internal page that cites it. No current wiki does this; requires a citation
   graph plus an outbound feed watcher. (§3.4 in KNOWLEDGE-PLATFORM-PLAN.md — currently deferred.)

5. **Offline-capable deployment.** A single Rust binary with embedded SQLite and Tantivy index,
   deployable to an air-gapped environment. Decisive for regulated clients in financial services and
   government who cannot allow SaaS-hosted documentation tools.

---

## 5. Google Cloud Console — The UX Anti-Pattern to Avoid

Google Cloud Console has 120+ products. Specific friction patterns:

- **Horizontal sprawl, no hierarchy.** Left navigation presents all services as a flat alphabetical
  list. No task-oriented grouping ("I want to deploy a container") — only product-oriented grouping.
  Users must know the product name before they can find it.
- **Context loss on project switch.** Switching projects reloads the console and drops navigation
  state. Three levels deep in a configuration → start over.
- **Settings buried under product-specific paths.** IAM for a specific resource is under that
  resource's own settings page, not a global IAM view.
- **Error messages that require a second console to interpret.** Permission denied errors emit a GCP
  error code requiring cross-reference to IAM documentation or Cloud Logging.
- **Search finds products, not settings.** Console search finds which product to navigate to but does
  not surface specific configuration pages, quotas, or resource-level settings.

**The core anti-pattern: product-as-primary-navigation instead of task-as-primary-navigation.**

A wiki engine avoids this by organising around content categories and reader intent, not around the
internal service boundaries of the platform. The Enterprise Learn design (Prototype C) addresses this
directly with audience-routed tiles ("For auditors / For engineers / For operators").

---

## Positioning statement (draft)

`app-mediakit-knowledge` is not a wiki. It is a **sovereign knowledge layer** for regulated
organisations — the internal evidence base that sits behind every public disclosure, every compliance
filing, and every investor communication. It combines the read experience of a great magazine, the
audit trail of a blockchain without the infrastructure, and the AI integration of an MCP endpoint
without the data egress risk. Hugo gives you a brochure. MediaWiki gives you a 2002 database. This
gives you a compliance-grade knowledge system that a regulator can independently verify.

---

## Sources

- Hugo discourse: Authorization and user state functionality
- CloudCannon: Top five static site generators for 2025
- Ask HN: Best self-hosted wiki solution in 2025?
- Nuclino: Best MediaWiki Alternatives in 2026
- Q4 Inc. platform overview (q4inc.com)
- Q4 IR Websites Reviews (G2)
- Docsie: On-Premise Wiki Alternative 2026
- GCP has the worst UI, except for all the others (Hacker News, 27936091)
- Google Cloud Console Reviews 2026 (G2)
- Google Cloud console search experience (Google Cloud Blog)
- Alcove MCP server with Tantivy BM25 (GitHub)
