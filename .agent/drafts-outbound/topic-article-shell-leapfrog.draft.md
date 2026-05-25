---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-knowledge
target_repo: content-wiki-documentation
target_path: applications/
target_filename: topic-article-shell-leapfrog.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-04-30T01:30:00Z
authored_by: task-project-knowledge (Opus parent synthesis from 4× Sonnet research)
authored_with: claude-opus-4-7
references:
  - clones/project-knowledge/.claude/drafts-outbound/research-wikipedia-leapfrog-2030.draft.md
  - clones/project-knowledge/.claude/drafts-outbound/topic-wikipedia-leapfrog-design.draft.md (existing — reviewed to avoid duplication)
  - external:en.wikipedia.org/wiki/Wikipedia:Manual_of_Style/Lead_section
  - external:en.wikipedia.org/wiki/Wikipedia:Verifiability
  - external:en.wikipedia.org/wiki/Wikipedia:Citing_sources
  - external:en.wikipedia.org/wiki/Wikipedia:Hatnote
  - external:en.wikipedia.org/wiki/Wikipedia:Vector_2022
  - external:meta.wikimedia.org/wiki/Wikimedia_Foundation_Annual_Plan/2025-2026/Product_%26_Technology_OKRs
  - external:enterprise.wikimedia.com/blog/structured-contents-wikipedia-infobox/
  - external:schema.org/TechArticle
  - conventions/draft-research-trail-discipline.md
  - conventions/citation-substrate.md
  - ni-51-102
  - osc-sn-51-721
notes_for_editor: |
  This TOPIC is the deeper companion to the existing
  topic-wikipedia-leapfrog-design.draft.md (which covers Phase 1.1 chrome — the
  visible muscle-memory items already shipped). This new TOPIC covers the
  article-shell LEAPFROG — what extends BEYOND Wikipedia at the article
  reading surface.

  Cross-reference relationship:
  - topic-wikipedia-leapfrog-design covers the WHY of muscle-memory parity
  - this TOPIC covers the WHY of going BEYOND parity

  Both should land. Project-language at gateway-time may consolidate or
  cross-reference; my recommendation is to keep separate since they serve
  different audiences (the parity narrative is for a reader assessing
  whether the substrate respects the muscle-memory contract; this leapfrog
  narrative is for a reader assessing whether the substrate is novel).

  Load-bearing sections:

  §2 — The 10 weakness categories of Wikipedia's article shell. Preserve all
  10. They are the structural argument for why a leapfrog is needed.

  §3 — The 5 leapfrog primitives. Three first-class + two second-class
  prioritization is load-bearing — do not promote a second-class primitive
  to first-class to round out the narrative.

  §4 — The body-register preservation paragraph. This is the "muscle memory
  must hold" guardrail. Without it the leapfrog framing reads as "we replaced
  Wikipedia"; with it the framing is "we extend Wikipedia at primitives it
  cannot ship".

  §5 — Forward-looking. All "planned" / "intended" / "may" per BCSC. Award
  framings as "the substrate is planned to be competitive in" — never as
  "we will win".

  Bilingual: full Spanish strategic-adaptation per DOCTRINE.md §XII.
research_done_count: 14
research_suggested_count: 5
open_questions_count: 1
research_provenance: synthesized-from-research-draft
research_inline: true
---

# The article shell leapfrog — extending Wikipedia at primitives volunteer-governance cannot ship

The PointSav documentation wiki at documentation.pointsav.com inherits its article reading surface from Wikipedia under the Vector 2022 design. Article tabs, edit pencils, hatnotes, lead-section conventions, collapsible Tables of Contents, infobox conventions, references sections, and category breadcrumbs all preserve the structural primitives a Wikipedia reader recognises without conscious thought. The companion TOPIC [[topic-wikipedia-leapfrog-design]] covers what is preserved verbatim — the muscle-memory contract.

This TOPIC covers what extends beyond. Five article-shell primitives that Wikipedia's volunteer-governance model has been structurally unable to ship in fifteen years are introduced here. Three are first-class — the substrate ships them at iteration 1. Two are second-class — the substrate defines their token and engine surfaces and ships when the corpus reaches the scale where they pay off the editorial cost.

## 1. Why the leapfrog is needed

Wikipedia's article shell is the most-imitated reading surface on the public internet. It is the gold standard for general-knowledge encyclopedic depth precisely because its primitives are honed by twenty years of editorial-community refinement. It is also structurally limited in 2026 by ten specific weaknesses that no commercial wiki competitor has shipped a fix for. The Wikimedia Foundation has named some of them in its 2025-2026 Product and Technology OKRs; others are visible in academic literature, Wikipedia's own Talk: discussion, and the structural friction Wikipedia editors describe in long-form discussion threads about the platform's limitations.

The leapfrog primitives in this TOPIC are not novelties. Each one addresses a specific named weakness with a specific named primitive. The substrate ships the fix because it has the engineering surface (Rust + axum + maud + comrak + git2 + Tantivy under `app-mediakit-knowledge`) and the editorial discipline (the project-language gateway with the draft-research-trail-discipline mandate per Doctrine claim #39) to do so. Wikipedia structurally cannot — its codebase is 25 years old, its community-consensus governance has not achieved coalition for substantial Main Page or article-shell change since 2012, and its labour model rewards new articles over revisions to existing structural infrastructure.

## 2. Where Wikipedia's article shell is structurally weak

Ten weakness categories from the 2026 audit:

**(i) References section is a flat numeric list with no source-authority semantics.** A peer-reviewed Nature paper and a personal blog post occupy identical visual registers. The COinS metadata embedded in footnote spans is parseable by Zotero and Mendeley but invisible to human readers. An auditor or analyst who wants to know whether a claim is regulator-backed or industry-press-backed must read each citation in full.

**(ii) Infoboxes are semi-structured but not natively machine-readable.** Wikimedia Enterprise's Structured Contents API parses them, but the API is paywall-gated and external to the reading surface. Wikidata is the canonical machine-readable mirror but synchronisation is volunteer-maintained and inconsistent.

**(iii) Table of Contents is structural-only with no semantic section typing.** "Background", "Method", "Controversy", "Technical implementation" cannot be distinguished by the TOC machinery — readers must read section titles and infer.

**(iv) What-links-here returns a paginated flat list, not a graph.** For a concept article the list may contain thousands of articles. No second-hop neighbours, no cluster grouping by topic domain, no link-context snippets, no machine-readable export at article level.

**(v) No inline-comment surface on the article reading view.** Editorial discussion is on a separate Talk: page; there is no way to see that a particular sentence is contested without leaving the article.

**(vi) No per-section last-edited or authorship granularity.** A 20-section article reports only article-level "last edited" — the freshness illusion.

**(vii) No reading-time or skim-aid.** Article-size guidance is metadata at `?action=info`, not surfaced in the reading view.

**(viii) No citation trail back to the specific cited passage.** Footnote `[4]` resolves to a bibliography entry; the reader must independently navigate to the cited source and locate the relevant passage within it.

**(ix) No live-edit currency indicator.** Articles edited dozens of times per day present no in-session change signal.

**(x) AI-consumption surface is unstructured at section granularity.** Wikipedia's reading-surface HTML provides no per-section semantic hints. The Wikimedia Foundation's 2025-2026 OKRs explicitly note 65% of expensive requests come from scraper bots collecting AI training data — the structure provided does not match the structure AI consumers need.

## 3. The leapfrog primitives

### 3.1 Citation-authority ribbon (first-class)

A small leading badge on each entry in an article's References section indicating source category — academic, regulator, industry, direct-source, news, web-informal. The class is derived from the citation template type and optionally from a `source_authority` frontmatter field. Emitted as an additional `@type` refinement on `citation` entries in the JSON-LD `<head>` block.

A reader can see at a glance whether the article is backed by academic and regulatory sources or by informal ones. An AI consumer pulling structured data gets source authority as a machine-readable field. The substrate component recipe is at the design-system substrate (when project-design refines and commits — see [[design-system]]).

### 3.2 Research-trail footer block (first-class)

A collapsible footer block below the References section, rendered when the article frontmatter declares `research_trail: true`. Three subsections per the existing draft-research-trail-discipline (Doctrine claim #39): Research done (sources consulted with status), Suggested research (next-leg open tasks), Open questions (claims requiring verification). For editors and researchers, not casual readers — collapsed by default.

The trail is emitted as structured JSON-LD `potentialAction` nodes — `SearchAction` for suggested research, `Question` for open questions. LLM consumers identify the article's epistemic frontier without reading prose. This addresses weakness (viii) and (x) above simultaneously: a reader gets a one-glance summary of what the editor knows is open; an AI consumer gets a typed semantic frontier.

The combination of citation-authority ribbon + research-trail footer is what makes the article's epistemological position legible without reading all the footnotes. A financial-community reader, an analyst, a regulator — any reader whose professional training involves source-type assessment — immediately understands what they are looking at. Wikipedia structurally does not ship this; the Talk: namespace is editorial discussion record, not editorial epistemic-frontier signal.

### 3.3 Freshness ribbon — per-section last-content-review (first-class)

An optional small badge on each section heading (right-end of the heading row, after the [edit] pencil) showing the date of the last substantive content change. Three-stop colour scale signals fresh / stale / archived per configurable date thresholds. Surfaces a signal Wikipedia structurally does not — a section-level review date that distinguishes "Background unchanged since 2019" from "Current implementations updated yesterday" — without modifying the article body register.

The substrate emits per-section `dateModified` properties on `WebPageElement` JSON-LD nodes. This addresses weakness (vi) directly and also addresses weakness (x): AI consumers get cheap section-level freshness signals rather than expensive full-article scrapes.

### 3.4 Plain-language toggle backed by curated authored paragraphs (second-class)

A toggle in the reader-preference toolbar. When active, article sections flagged `plain_language: true` in their frontmatter render an alternative lead paragraph written at a lower reading level. The plain-language paragraph appears in a visually distinct block above the technical lead. A reader who toggles back sees the original.

Critical design discipline: plain-language paragraphs are explicitly authored by humans and committed to the article source, with the same citation discipline as the article body. They are not LLM-generated at request time. This preserves the NPOV register and source-based verifiability while extending the entry-point to readers whose reading level or background does not match the technical register.

The substrate ships the toggle and the schema; the corpus chooses which articles to author plain-language paragraphs for. Positioned as second-class because curating the plain-language paragraph at edit time costs editorial labour that scales linearly with corpus size.

### 3.5 Citation-graph mini-map — 3-hop neighbourhood (second-class)

A collapsible section at the article foot showing a small SVG graph: the current article as centre node, 1-hop outbound wikilinks as one ring, 1-hop inbound links as a second ring. Nodes labelled with article titles; edges carry directionality. Interactive — clicking navigates to that article. Sized to a fixed aspect-ratio box; only top-N nodes by link weight shown with an "expand" affordance.

Same link data emitted in JSON-LD `relatedLink` and `mentions` arrays. Downstream knowledge-graph consumers traverse without the visual layer. Positioned as second-class because the wikilink graph must be pre-computed at render time or served from an API. Worth shipping when the article corpus reaches a size where graph traversal is genuinely useful (target: ≥200 articles).

## 4. What the leapfrog deliberately does not change

The body register that makes Wikipedia's prose feel authoritative is the product of explicit editorial discipline codified in its Manual of Style. Every leapfrog primitive in this TOPIC is additive — none modify the body register. The summary style (every section opens with its most important information). The defined subject at the open (bolded subject + copula + definitional clause). The NPOV register (institutional voice; attributing claims rather than asserting). The paragraph length discipline (3–6 sentences typical; visual whitespace separating; scannable rhythm). The link density as navigation density (first-occurrence-only blue links; no decorative links). The MOS lead-section contract (the lead is a summary, not a teaser). The register consistency across section types (Background, Method, Criticism treated identically; no visual-weight implication of importance).

All seven characteristics are preserved verbatim. The leapfrog primitives are subordinate to the body — the citation-authority badges sit in a 1.25em gutter that does not break line-rhythm; the research-trail is collapsed by default; the freshness ribbons can be toggled off entirely by reader preference; the plain-language toggle is opt-in; the citation-graph mini-map is collapsed below the visible article footer. A reader who arrives at the article shell and reads only the body receives the Wikipedia muscle-memory contract intact. A reader who wants the leapfrog primitives finds them where they should be, and they read as additive rather than as replacement.

## 5. What this article shell is planned to compete for

The substrate is planned to be structurally competitive in the same award framings as the home-page composition: Awwwards Site of the Day or Site of the Year; Webby Award in the Reference category; Information is Beautiful Awards in the Interactive / Tools & Services category; Communication Arts Interactive Annual in the Information Design subcategory; the GitNation JavaScript Open Source Awards; the European Open Source Awards; and editorial coverage in MIT Technology Review's annual technology surveys. The article-shell-specific differentiator that wins on its own merits is the citation-authority ribbon plus research-trail footer combination — the article's epistemological position made legible at the reading surface, in a way that no other public knowledge platform ships in 2026.

Whether any specific award lands depends on factors outside the substrate's control. The substrate's structural position is what is claimed; awards are downstream consequence-or-not.

## 6. Open editorial item

Talk-page surface for each article is planned. The article shell may surface a small inline-annotation affordance in a future iteration — a way for a reader to see that a particular sentence is contested, recently fact-checked, or subject to a dispute resolution thread, without leaving the article. The substrate-design and engine surfaces for this are tracked in NEXT.md but not committed to a specific iteration.

## Provenance

Authored 2026-04-30 by a Task Claude session in the project-knowledge cluster, synthesising parallel Sonnet sub-agent research with direct workspace-document consultation. The substrate-internal design research at `clones/project-knowledge/.claude/drafts-outbound/research-wikipedia-leapfrog-2030.draft.md` is the source of truth for the primitive prioritisation. The companion DESIGN-COMPONENT recipes (`component-citation-authority-ribbon`, `component-research-trail-footer`, `component-freshness-ribbon`) define the substrate-canonical visual contract; this TOPIC is the public-facing narrative.

Forward-looking statements per [ni-51-102] and [osc-sn-51-721] continuous-disclosure posture, applied at all times whether or not Foundry-affiliated entities are reporting issuers. Material assumptions: design-system substrate refines token-bundle and component recipes per the companion drafts; editorial labour for the project-language gateway is sustained; the engine work for per-section JSON-LD emission is shipped in a future project-knowledge cluster iteration tracked in NEXT.md.
