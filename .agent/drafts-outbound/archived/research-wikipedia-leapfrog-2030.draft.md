---
schema: foundry-draft-v1
state: draft-pending-design-pass
originating_cluster: project-knowledge
target_repo: pointsav-design-system
target_path: research/
target_filename: wikipedia-leapfrog-2030.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-RESEARCH
authored: 2026-04-30T00:55:00Z
authored_by: task-project-knowledge (Opus parent + 4 Sonnet sub-agents in parallel for primary research)
authored_with: claude-opus-4-7 + 4× claude-sonnet-4-6
references:
  - external:en.wikipedia.org/wiki/Main_Page
  - external:en.wikipedia.org/wiki/Wikipedia:Today%27s_featured_article
  - external:en.wikipedia.org/wiki/Wikipedia:Did_you_know
  - external:en.wikipedia.org/wiki/Wikipedia:In_the_news
  - external:en.wikipedia.org/wiki/Wikipedia:Picture_of_the_day
  - external:en.wikipedia.org/wiki/Wikipedia:Manual_of_Style/Lead_section
  - external:en.wikipedia.org/wiki/Wikipedia:Vector_2022
  - external:mediawiki.org/wiki/Skin:Vector/2022/Design_documentation
  - external:meta.wikimedia.org/wiki/Wikimedia_Foundation_Annual_Plan/2025-2026/Product_%26_Technology_OKRs
  - external:enterprise.wikimedia.com/blog/structured-contents-wikipedia-infobox/
  - external:mdpi.com/2227-9709/12/3/97
  - external:designtokens.org/tr/drafts/format/
  - external:w3.org/community/design-tokens/2025/10/28/design-tokens-specification-reaches-first-stable-version/
  - external:doc.wikimedia.org/codex/latest/style-guide/typography.html
  - external:v10.carbondesignsystem.com/guidelines/themes/overview/
  - clones/project-knowledge/.claude/drafts-outbound/component-home-grid.draft.md
  - clones/project-knowledge/.claude/drafts-outbound/topic-wikipedia-leapfrog-design.draft.md
  - content-wiki-documentation/.claude/rules/naming-convention.md
  - conventions/cluster-design-draft-pipeline.md
  - conventions/draft-research-trail-discipline.md
notes_for_designer: |
  This research consolidates four parallel Sonnet sub-agent reports into a single
  design-research record for the substrate. It is the canonical "what we learned" for
  every downstream DESIGN-COMPONENT and DESIGN-TOKEN-CHANGE draft staged in this
  pickup batch (research-trail-footer, citation-authority-ribbon, freshness-ribbon,
  knowledge-wiki-baseline tokens).

  This draft is research, not chrome — it is meant to live in
  `pointsav-design-system/research/wikipedia-leapfrog-2030.md` as a permanent
  reference. The component drafts and token-change draft cite this document.

  Load-bearing decisions for substrate-side refinement:

  - The five "Award-winning leapfrog primitives" in §6 are NOT a long wishlist; they
    are the deliberate three-plus-two set the substrate should design for first.
    Substrate refinement should preserve the prioritization.

  - The Wikipedia Main Page primitive inventory (§2) is structural, not aspirational —
    every named slot has a role and a refresh cadence. The substrate may rename, but
    must not silently drop primitives.

  - The competitor-landscape failure-mode table (§4) is what makes the leapfrog claim
    survivable in a Communication Arts / Webby jury room. The substrate should not
    soften the comparison register; the operator's framing is "win an award for
    cleaning up Wikipedia" and the research backs that with a market-gap analysis.

  - The 6 open questions for project-design ratification (§7) are real decisions, not
    placeholder questions. Resolving them is gateway-time scope.
research_done_count: 32
research_suggested_count: 9
open_questions_count: 6
research_provenance: web-fetch + web-search + workspace-direct-consultation
research_inline: true
master_cosign_required: |
  This is DESIGN-RESEARCH (no Master co-sign required by convention §1). The
  companion DESIGN-TOKEN-CHANGE draft (token-knowledge-wiki-baseline.draft.md)
  DOES require Master co-sign per project-tetrad-discipline.md / cluster-design-
  draft-pipeline.md §3 (brand identity is governance scope). This research draft
  is staged independently and may be refined ahead of token-change ratification.
---

# Wikipedia leapfrog 2030 — design-research substrate for documentation.pointsav.com

A consolidated research record on what the encyclopedic-knowledge home-page and article-shell substrate should be in 2030, derived from a primitive-level audit of Wikipedia's English Main Page (https://en.wikipedia.org/wiki/Main_Page), a structural anatomy of the Wikipedia article shell under Vector 2022, a 25-provider competitive-landscape audit across collaborative knowledge bases, public wiki engines, developer documentation site generators, and personal/networked-thought tools, and a DTCG (Design Tokens Community Group, 2025.10 stable specification) token-vocabulary proposal anchored to Carbon Design System v10 conventions and Wikimedia Codex precedent.

The operator's framing is unambiguous: "we need our wiki to have the gold standard from Wikipedia ... we need to win an award for cleaning up Wikipedia." This research backs that framing with the structural reasons no provider in the last decade has shipped a Wikipedia-class encyclopedic surface, and names the specific primitives a 2030 successor adds without breaking the muscle-memory contract that gives Wikipedia its authority register.

## 1. Why this research was commissioned

`documentation.pointsav.com` shipped its first Wikipedia-Main-Page-shaped chrome at workspace v0.1.70 (2026-04-29), implementing 9 muscle-memory items at the article shell (Article/Talk tabs, Read/Edit/View-history tabs, per-section [edit] pencils, hatnote, lead first-sentence convention, tagline, collapsible left-rail TOC, language switcher, footer ordering) and a home-page composition of lede + featured-pin panel + 3×3 category grid + recent-additions feed. The iteration-1 ship was deliberate visible-operational pacing — show that the surface is operational, even if not yet the gold standard.

Iteration 2 needs to push beyond visible-operational into the leapfrog claim. The cluster's authority on this question is the substrate engine (`app-mediakit-knowledge` in `pointsav-monorepo`); the substrate-design authority is `pointsav-design-system`. This research is the bridge: it documents what the substrate should design *for* before any new component recipe lands.

## 2. Wikipedia Main Page — primitive-level inventory

The English Wikipedia Main Page is structurally composed of ten slots, each with a defined role, refresh cadence, and editorial maintainer. Removing any one breaks the contract. Format invariants are enforced by character count, not word count — this discipline is what prevents both padding and truncation across daily rotation cycles.

| Slot | Role | Refresh cadence | Maintained by | Format invariant |
|---|---|---|---|---|
| Welcome banner | Orientation; declares scope and signals community scale through statistics, not adjectives | Continuous (live counter) | MediaWiki software | Single sentence + two stats; no marketing copy |
| Today's Featured Article (TFA) | Flagship editorial showcase from the 0.1% of articles that pass FA review | Daily at 00:00 UTC | Three named coordinators | 909–1,009 character paraphrase; bold linked title; "(Full article...)" closer; recently-featured archive |
| Did You Know (DYK) | New-content discovery via single striking hook fact | Daily (twice daily when backlog >120) | Community reviewers + bot rotation | "Did you know that…" rhetorical frame; 15–25 word hook; nine hooks per rotation; cited inline |
| In the News (ITN) | News-anchor function; current-events bridge to encyclopedic depth | Non-uniform (consensus-driven) | Community nomination + admin posting | 4–6 blurbs + Ongoing sub-section + Recent deaths sub-section; one sentence per blurb |
| On This Day (OTD) | Historical depth via temporal anchoring to the current date | Daily (pre-authored full year) | Wikipedia:Selected anniversaries project | 5–8 bullet events in roughly reverse chronological order; one sentence per bullet |
| Today's Featured Picture (TFP) | Visual anchor; standalone discovery surface | Daily (queue-driven) | Featured Pictures regulars | One image + ~100 word encyclopedic caption + photographer credit |
| Other Areas | Reader → editor onramp | Static (community-infrastructure-scale) | Community | 7 linked items with 10–15 word descriptions |
| Sister Projects | Ecosystem disclosure across 12 Wikimedia projects | Static | Wikimedia Foundation | Logo + name + 20–30 word description; no implied hierarchy |
| Languages bar | Global scope signal across 346 language editions | Static (re-tiered as counts cross thresholds) | Wikidata interlanguage links | Three tiers by article count |
| Footer | License, privacy, contact | Static | Wikimedia Foundation | License notice + last-updated timestamp; no advertising; no newsletter signup |

The two-column asymmetry (left 65%, right 35%) is not a stylistic choice — it is the same spatial grammar as a newspaper broadsheet, embedded in Western reading cognition. The blue link colour is treated by Wikimedia's Vector 2022 design documentation as "a critical part of the Wikipedia reading experience" (https://www.mediawiki.org/wiki/Skin:Vector/2022/Design_documentation), not subject to redesign. The "(Full article...)" parenthetical is a deliberate non-CTA — it reads as encyclopedic prose continuation, not as a marketing button.

Density without clutter is a measurable property: a single Main Page screen at 1080p contains 1 featured blurb + 9 DYK hooks + 4–6 news items + 5–8 OTD bullets + 1 featured picture caption ≈ 20–25 independently navigable units above the fold, with no whitespace padding gratuitously separating sections.

## 3. Wikipedia article-shell anatomy under Vector 2022

The article-shell substrate is the single most-imitated reading surface on the public internet. Its primitives in 2026:

**Page-level chrome** (persistent across scroll) — sticky header (introduced Vector 2022), Article/Talk tab pair, Read/Edit/View-history tab triplet, watch star, page-protection icon, Tools menu (right sidebar; reorganised January 2023), language bar (button near title; 8-language modal popover).

**Article-body primitives** (in source order) — hatnote (italic disambiguation note above lead body, before infobox), infobox (right-float key-value table; less common for engineering articles), lead section (no explicit heading; 100–400 words; opening sentence bolds the subject and provides a one-clause definition; summarises the entire article body; stands alone), Table of Contents (Vector 2022 made it sticky-left-rail, collapsible, structural-only with no semantic typing), section headings with per-section [edit] pencils (right-end of heading row; no section-level last-edited indicator), body text in summary-style register (most important information first; NPOV; plain English; 17px since 2023; line-height 1.5+; ~960px column maximum), inline citation superscripts (COinS metadata embedded; no visual differentiation by source type), navboxes (collapsible foot-of-article thematic clusters), sister-project links.

**Article-foot primitives** (Wikipedia MOS Layout-mandated ordering) — See also → Notes → References → Further reading → External links → Categories.

**What-links-here and page-information tools** — accessible via the Tools menu; flat-list output with no graph visualisation, no link-context snippets, no machine-readable export at article level.

The article-shell weaknesses in 2026, from the perspective of an encyclopedic-grade reading surface:

1. **References section is a flat numeric list with no source-authority semantics.** A peer-reviewed Nature paper and a personal blog post occupy identical visual registers. The COinS metadata is parseable by Zotero/Mendeley but invisible to human readers. An auditor or analyst who wants to know whether a claim is regulator-backed or industry-press-backed must read each citation in full.

2. **Infoboxes are semi-structured but not natively machine-readable.** Wikimedia Enterprise's Structured Contents API parses them, but the API is paywall-gated and external to the reading surface. Wikidata is the canonical machine-readable mirror but synchronisation is volunteer-maintained and inconsistent — divergence between an infobox and its Wikidata equivalent has no signal to readers.

3. **TOC is structural-only, with no semantic section typing.** A "Background" / "Method" / "Controversy" / "Technical implementation" cannot be distinguished by the TOC machinery — readers must read section titles and infer. AI consumers cannot navigate by content kind without reading section prose first to classify it.

4. **What-links-here returns a paginated flat list, not a graph.** For a concept article like capability-based security, this list may contain thousands of articles. No second-hop neighbours, no cluster grouping by topic domain, no link-context snippets, no machine-readable export at article level.

5. **No inline-comment surface on the article reading view.** The Talk Pages Project (2019–2022) added Reply tool and section subscriptions, but the editorial discussion is still a separate page — there is no way to see that a particular sentence is contested without leaving the article.

6. **No per-section last-edited or authorship granularity.** A 20-section article with one updated section yesterday and 19 unchanged since 2019 reports only "edited yesterday" at the article level. Freshness illusion.

7. **No reading-time or skim-aid.** Article-size guidance is metadata at `?action=info`, not surfaced in the reading view.

8. **No citation trail back to the specific cited passage.** Footnote `[4]` resolves to a bibliography entry; the reader must independently navigate to the cited source and locate the relevant passage within it.

9. **No live-edit currency indicator.** Articles edited dozens of times per day present no in-session change signal.

10. **AI-consumption surface is unstructured at section granularity.** Wikipedia provides Atom/RSS feeds, REST API, and Wikimedia Enterprise structured content. The reading-surface HTML provides no per-section semantic hints. The Wikimedia Foundation's 2025-2026 OKRs explicitly note 65% of expensive requests come from scraper bots collecting AI training data — the structure provided does not match the structure AI consumers need, leading to undifferentiated full-article scraping.

The body-register characteristics that any leapfrog must preserve (these are what make Wikipedia's prose feel authoritative):

- **Summary style** — every section opens with its most important information.
- **Defined subject at the open** — bolded subject + copula + definitional clause as an inviolable opening pattern.
- **NPOV register** — institutional, attributing claims rather than asserting; the prose anchor of authority.
- **Paragraph length discipline** — 3–6 sentences typical; visual whitespace separating; scannable rhythm.
- **Link density as navigation density** — first-occurrence-only blue links by MOS; no decorative links.
- **MOS lead-section contract** — the lead is a summary, not a teaser; reading only the lead returns accurate-if-incomplete information.
- **Register consistency across section types** — Background, Method, Criticism treated identically; no visual-weight implication of importance.

## 4. Competitive landscape — why no provider has replaced Wikipedia in 2026

Twenty-five providers were audited across four groups: collaborative knowledge bases (Notion, Confluence, Coda, ClickUp Docs); public-facing wiki engines (Wiki.js, BookStack, Outline, MediaWiki, Fandom, Wikidot, DokuWiki, TiddlyWiki); developer documentation site generators (Docusaurus, MkDocs Material, VitePress, Nextra, Fumadocs, Astro Starlight, GitBook, Read the Docs); and personal/networked-thought tools (Obsidian Publish, Roam Research, Logseq, Capacities, Quartz v4).

The cross-cutting structural reasons no provider has replaced Wikipedia for general encyclopedic knowledge are eight:

**(i) Audience mismatch.** Notion, Confluence, Coda, ClickUp, Outline, BookStack were built for private organizational knowledge management. The access-control model, pricing model, and UX all assume a known trusted team. Public-encyclopedic publishing requires the opposite — anonymous editors, verifiable sourcing, reader-first navigation. These products cannot pivot to that use case without dismantling their commercial model.

**(ii) No editorial constitution.** Wikipedia's NPOV policy, Notability standards, Reliable Sources policy, No Original Research rule, and Manual of Style constitute a multi-decade-refined editorial constitution. No provider in this audit ships an equivalent. The absence is not a missing feature — it is a missing governance organization. MediaWiki the software exists; the Wikimedia community governance on top of it is what produces Wikipedia.

**(iii) Information density floor.** Docusaurus, MkDocs, VitePress, GitBook, Obsidian Publish optimize for prose elegance, developer aesthetics, clean typography. This is the wrong optimization for encyclopedic reference. Wikipedia articles are deliberately dense — infoboxes, hatnotes, references sections with 100+ footnotes, navboxes linking to related articles, stub tags, disambiguation pages. No documentation site generator ships this density model because its target users actively want the opposite.

**(iv) Navigation primitive missing.** The Wikipedia navigation stack — `[[wikilink]]` with red-link signaling, Special:Random, Special:WhatLinksHere, category graph navigation, disambiguation pages, navbox templates, sister-project interlinking — exists complete in MediaWiki and at most one or two members elsewhere. Obsidian Publish and Quartz have graph views; Capacities has typed-object surfaces. No other provider ships even the red-link mechanism, which is structural to Wikipedia's growth model — it makes missing knowledge visible.

**(v) Citations are decorative, not load-bearing.** Wikipedia's footnote system (cite, reflist, sfn templates) makes claims verifiable at the statement level. Across Group A, C, and D providers, citations are absent entirely, implemented as inline hyperlinks with no formal structure, or supported as page-level frontmatter rather than claim-level. BookStack, DokuWiki, Wiki.js support external links but have no footnote infrastructure.

**(vi) No Talk-page substrate.** Each Wikipedia article has a Talk: page that is the public record of every editorial dispute, consensus negotiation, and source debate. This is the epistemic provenance layer. Confluence and Notion have inline comments — not archived public editorial debate.

**(vii) Structural brittleness.** Notion's block format, Coda's pack structure, ClickUp's embedded docs are proprietary serialization formats — content created in 2020 is at vendor-lock-in risk by 2026. Wikipedia's wikitext is plain text that can be exported, archived, and mirrored.

**(viii) Template homogenization.** Every Docusaurus, Starlight, VitePress, MkDocs site looks structurally identical — two-column layout, left-sidebar nav tree, top search bar, dark mode toggle. This is the documentation aesthetic every engineering team knows. It is also what a Wikipedia reader does *not* associate with encyclopedic authority.

The genuine advantages each provider has over Wikipedia are real and worth identifying — they are the candidates a 2030 leapfrog should steal. Three stand out as integrable into a Wikipedia-class chrome without breaking the muscle-memory contract:

- **MkDocs Material's instant client-side search** with offline support and zero external dependencies — the fastest search-to-result experience in the documentation space.
- **Capacities' typed-object relationship surface** rendered as navigable article metadata — explicit semantic types (concept / os / service / app / adr / person / organization / governance / financial-disclosure / reference / help) with relationship discovery.
- **Obsidian Publish's hover-preview popover on `[[wikilinks]]`** — read-without-clicking affordance for the reader who wants to know if the linked article is worth opening.

## 5. The market-gap analysis — why this leapfrog is structurally available

The gap exists because closing it requires simultaneously building governance software, a navigation primitive set, and an editorial culture, and no commercial incentive in the last decade has pointed all three directions at once.

**Commercial incentive misalignment.** Notion, Confluence, GitBook, Coda, ClickUp make money by selling seat licenses to organizations managing internal knowledge. Their roadmaps are driven by enterprise IT buyers, not by researchers, public-knowledge contributors, or encyclopedic readers. NPOV enforcement, Talk-page infrastructure, red-link discovery do not convert to enterprise seat revenue. The commercial incentive actively points away from the Wikipedia structural model.

**The editorial-labour problem cannot be automated.** Wikipedia's structural authority is twenty years of accumulated editorial labor by a self-organized volunteer community enforcing policies no product manager wrote. AI can generate encyclopedia-style content quickly but cannot replicate the transparent editorial process, source verification standards, or community governance that make Wikipedia trusted by search engines, AI models, and researchers. Any competitor seeking to replicate the credibility surface must replicate the governance — and no commercial entity has bootstrapped that from a product launch.

**Open-source coordination cost.** MediaWiki's codebase is 25 years old, carries enormous legacy compatibility surface, and requires Wikimedia Foundation resources to maintain. No independent open-source project has shipped a "MediaWiki v2 with modern UX" because the coordination cost is prohibitive. Wiki.js tried and reached v2.5 without achieving Wikipedia-class structural primitives.

**Scope creep on one side, narrow scope on the other.** Notion / Confluence have expanded into "everything platforms"; their knowledge-base features compete for attention with AI agents, project management, enterprise integrations. Docusaurus / MkDocs / VitePress are deliberately minimal static-site generators — no collaborative editing model by design.

**The "Wikipedia muscle memory" gap.** No competitor has invested in replicating the specific reader-navigation UX billions of Wikipedia users know by reflex: infobox top-right, TOC in the first scroll, hatnote disambiguation, blue-link density in the first paragraph, References section as credibility signal, category breadcrumbs as exit navigation. This is an information-architecture commitment, not a CSS problem. Documentation sites ship sidebars because their readers navigate a product's API. Encyclopedia readers arrive from search, orient via the infobox, follow blue links sideways, exit via categories. No product in 2026 has designed for that journey.

This is the leapfrog opportunity. PointSav's substrate-sovereignty posture (per `conventions/compounding-substrate.md`), three-tier compute routing (Doctrine §III row 18), apprenticeship corpus capture (Doctrine claim #32), and editorial-pipeline infrastructure (Doctrine claim #35 — reverse-funnel editorial pattern with project-language as gateway) give the substrate exactly the three preconditions no commercial competitor can simultaneously match. The wiki engine becomes the customer-installable demonstration of that substrate.

## 6. Award-winning leapfrog primitives — the prioritized set

Three primitives are first-class, two are second-class. All five are additive to the existing Wikipedia muscle-memory chrome — none modify the body register or the navigation primitive contract.

### 6.1 Citation-authority ribbon (FIRST)

A small coloured left-border or badge on each entry in the References section, indicating source category: academic / peer-reviewed (blue), government / regulator (dark green), industry / trade (orange), direct primary source (teal), news (grey), web / informal (light grey). The class is derived from the citation template type and optionally from a `source_authority` frontmatter field. Emitted as an additional `@type` refinement on `citation` entries in the JSON-LD `<head>` block.

A reader can see at a glance whether the article is backed by academic and regulatory sources or by informal ones. An AI consumer pulling structured data gets source authority as a machine-readable field. This component is what makes the citation substrate visible at the reading surface — directly expressing the BCSC continuous-disclosure posture (workspace §6 rule 6: citations are part of the substrate, machine-readable, audit-traceable).

Visual treatment is subordinate to body register — colour is never the sole differentiator (each badge carries an `aria-label` text). Substrate refinement should harmonize the badge typography with Carbon's tag/badge conventions while preserving the colour-encoded source-authority semantic.

### 6.2 Research-trail footer block (FIRST)

A collapsible footer block below the References section, rendered when the article frontmatter declares `research_trail: true`. Three subsections per the existing `draft-research-trail-discipline` convention (Doctrine claim #39): Research done (sources consulted with status), Suggested research (next-leg open tasks), Open questions (claims requiring verification). For editors and researchers, not casual readers — collapsed by default.

Emitted as structured JSON-LD `potentialAction` nodes — `SearchAction` for suggested research, `Question` for open questions. Enables LLM consumers to identify the article's epistemic frontier without reading prose. The collapsible `<details>/<summary>` pattern with proper `aria-expanded` state makes it accessible; when collapsed the summary line reads "Research trail — N done, N suggested, N open questions" so screen-reader users see the count without expansion.

This combination (citation-authority ribbon + research-trail footer) makes the article's epistemological position legible without reading all the footnotes. A financial-community reader, an analyst, a regulator — any reader whose professional training involves source-type assessment — immediately understands what they are looking at. This is the highest-leverage leapfrog primitive: it addresses the single biggest practical problem with Wikipedia-style reference sections (flat undifferentiated list) in a way that is additive, does not alter the body reading experience, and produces structured data that is valuable to downstream consumers.

### 6.3 Freshness ribbon — per-section last-content-review date (FIRST)

An optional small badge on each section heading (right of the `[edit]` pencil, not replacing it) showing the date of the last substantive content change. "Last reviewed: 2024-03" in muted type. Derived from Git blame at section level — the most recent commit that touched content lines within the section's boundary. Separate from the article-level last-edited timestamp.

The distinction between "section reviewed" and "section edited" matters: a cosmetic formatting change should not update the freshness signal. The frontmatter can declare `content_reviewed_on` per section to allow editorial human override of the automated git-blame date.

Section-level review dates emitted as `dateModified` properties on per-section `WebPageElement` JSON-LD nodes. Consumers can filter "show me only sections reviewed after 2025-01-01" — directly addressing the Wikimedia Foundation's acknowledged problem that 65% of their expensive requests are AI scrapers that cannot do better than full-article pulls when section-level freshness is undeclared.

This feature wins on two independent dimensions: it is demonstrably useful to every reader of technical documentation (not a power-user feature), and it creates structured JSON-LD output that makes the corpus more valuable to AI consumers.

### 6.4 Plain-language toggle backed by curated authored paragraphs (SECOND)

A toggle in the reader-preference toolbar (alongside the density toggle from the existing leapfrog draft). When active, article sections flagged `plain_language: true` in their frontmatter render an alternative lead paragraph written at a lower reading level. The plain-language paragraph appears in a visually distinct block above the technical lead. A reader who toggles back sees the original.

Critical design discipline: plain-language paragraphs are explicitly authored by humans and committed to the article source, with the same citation discipline as the article body. They are not LLM-generated at request time. This preserves NPOV register and source-based verifiability while extending the entry-point to readers whose reading level or background does not match the technical register.

Plain-language content is emitted as `disambiguatingDescription` on the article's Schema.org `TechArticle` node — the short plain-language summary search engines and AI assistants consume preferentially for rich-snippet and AI-summary outputs. The toggle is for direct readers; the structured-data emission is the silent distribution mechanism for the indirect audience that arrives via search and AI summary.

This is positioned as second-class because curating the plain-language paragraph at edit time costs editorial labour that scales linearly with corpus size. The substrate ships the toggle and the schema; the corpus chooses which articles to author plain-language paragraphs for.

### 6.5 Citation-graph mini-map — 3-hop neighbourhood (SECOND)

A collapsible section at the article foot (between External links and Categories) showing a small SVG graph: the current article as centre node, 1-hop outbound wikilinks as one ring, 1-hop inbound links as a second ring. Nodes labelled with article titles; edges carry directionality. Interactive — clicking navigates to that article. Sized to a fixed aspect-ratio box; only top-N nodes by link weight shown with an "expand" affordance.

Same link data emitted in JSON-LD `relatedLink` and `mentions` arrays. Downstream knowledge-graph consumers can traverse without the visual layer. Screen readers encounter the article-prose links in standard body order; the mini-map carries `aria-label` describing its role and an equivalent plain-text list of linked articles in a hidden `<details>` element.

Positioned as second-class because the wikilink graph must be pre-computed at render time or served from an API — high effort relative to the other primitives. Worth shipping when the article corpus reaches a size where graph traversal is genuinely useful (≥200 articles).

## 7. Open questions for project-design ratification

These are real decisions, not placeholder questions. Resolving them is gateway-time scope; the substrate cannot ship coherent component recipes without answers.

**(a) Light/dark theme switching pattern.** The proposed token taxonomy locates all dark-mode substitutions at the semantic tier (a separate JSON file overriding semantic token values), matching Wikimedia Codex's `theme-wikimedia-ui-mode-dark.css` pattern (https://www.npmjs.com/package/@wikimedia/codex-design-tokens) and DTCG `$extends` group inheritance. The question: does the PointSav build pipeline use Style Dictionary's built-in `@value` override mechanism, a DTCG `$extends` override file, or a CSS custom property `:root[data-theme="dark"]` selector? The choice has implications for runtime bundle size and tenant-specific theme stacking.

**(b) `wiki.*` prefix conflict with the `.ps-*` substrate convention.** Current `app-mediakit-knowledge` CSS uses `.wiki-` BEM prefix. The proposed token taxonomy introduces a `wiki.*` semantic namespace. Project-design must rule on whether wiki-specific semantic tokens belong under `ps-wiki.*` (consistent with PointSav substrate naming) or whether `wiki.*` is the tenant-facing public API and `ps.*` tokens sit beneath it. The answer determines how Woodfine and future tenants override wiki chrome without touching PointSav substrate tokens.

**(c) Variable-font loading vs system-stack discipline.** The proposed `font.family.heading` includes Linux Libertine Display and Charter as licensed OFL/system fonts. A variable-font version of a high-quality serif (Source Serif 4 Variable, Alegreya Variable) would enable continuous font-weight scaling. Workspace CLAUDE.md §6 emphasises sovereignty over external CDN dependency. The decision: load one variable serif from a self-hosted CDN path, or keep the system-stack fallback chain and accept visual variance across OS/browser combinations.

**(d) Density toggle as token vs component state.** The proposal defines `density.*` as primitive tokens with multipliers (0.75/1.0/1.5) that component tokens reference. The alternative is a component-level `data-density` attribute activating variant CSS classes, keeping density out of the token file. The token approach is more portable across rendering environments (native, PDF export, screen-reader CSS) but requires every component token participating in density to carry three referenced values.

**(e) FLI-banner colour register — amber vs neutral.** The `status.fli-banner-bg` token currently resolves to `color.status.warn.bg` (amber). The BCSC forward-looking-information disclosure requirement is a *process-discipline* requirement, not an error condition. Amber risks miscommunication: readers familiar with warning patterns may interpret an FLI banner as a content-quality warning. Project-design should consider whether a distinct `color.brand.teal.*` treatment or a neutral `surface.layer-accent` with a specific icon is more register-accurate. Once `status.fli-banner-bg` resolves to warn, every context using it inherits "caution" semantics.

**(f) Research-trail visual weight — footer chrome vs body content.** The `article.research-trail.*` tokens currently treat the trail as styled chrome (background, border, muted text). The draft-research-trail-discipline requires the trail on every draft entering the editorial pipeline. At article-shell render time, it appears as a bottom-of-article section. The question: visually subordinate (footer chrome — low contrast, small type, collapsed by default) or first-class body section (same vertical rhythm as a normal section, with its own heading hierarchy)? The token proposal supports both. Currently `trail-bg` references `surface.layer-accent` (chrome treatment); if the ruling is body-content, `trail-bg` should reference `surface.layer` and receive only a `trail-border` left-rule analogous to a blockquote.

## 8. Award criteria — what this leapfrog targets

A wiki that ships Wikipedia muscle memory plus the five primitives in §6 is competitive in seven realistically reachable awards:

- **Awwwards Site of the Day / Site of the Year** — Design 30% / Usability 30% / Creativity 15% / Content 15% / Mobile 10%. Awwwards rewards craft in information architecture; documentation sites typically fail "Design" because template homogenization is obvious. A genuinely novel article-shell layout that borrows the encyclopedia form factor at modern type standards is competitive.
- **Webby Award — Websites & Mobile Sites: Reference** — Wikipedia itself has won Webby awards in this category. Judging: content, structure, navigation, visual design, functionality, interactivity, overall experience. No Reference category winner in recent years has shipped typed-object semantic structure + red-link discovery + Talk-page-equivalent provenance.
- **Information is Beautiful Awards — Interactive / Tools & Services** — beauty, storytelling, impact, innovation. The category-graph rendered as explorable force-directed visualization with typed nodes, hover-preview summaries, and red-link gaps visible as unrealized nodes is directly competitive.
- **Communication Arts Interactive Annual** — Information Design subcategory rewards editorial and information-architecture decision-making, not just visual surface.
- **Open Source Awards — JavaScript Open Source (GitNation)** — Most Exciting Use of Technology / Most Impactful Open Source Project. The MediaWiki alternative space has no credible 2026 entrant.
- **European Open Source Awards 2026** — Outstanding Achievement in Skills and Education. European provenance, GDPR-native architecture, bilingual English/Spanish publishing positively received.
- **MIT Technology Review's Breakthrough Technologies List** — does not take nominations; surfaces technologies through editorial coverage. The structural claim that attracts MIT TR attention: a wiki engine that closes the loop between human editorial governance and verifiable machine-readable knowledge graph output.

## 9. Substrate-side scope — what project-design refines from this research

This research draft is the source-of-truth context for four downstream drafts staged in the same pickup batch:

1. `component-citation-authority-ribbon.draft.md` — DESIGN-COMPONENT recipe for §6.1
2. `component-research-trail-footer.draft.md` — DESIGN-COMPONENT recipe for §6.2
3. `component-freshness-ribbon.draft.md` — DESIGN-COMPONENT recipe for §6.3
4. `token-knowledge-wiki-baseline.draft.md` — DESIGN-TOKEN-CHANGE proposing the three-tier DTCG bundle (requires Master co-sign per cluster-design-draft-pipeline.md §3 — brand identity is governance scope)

Each downstream draft cites this research at its `references:` field. project-design refines all five at gateway time, and routes the refined component recipes + token bundle into `pointsav-design-system/components/`, `pointsav-design-system/research/`, and `pointsav-design-system/tokens/dtcg-bundle.json` respectively, via the standard design-system Root commit handoff per `cluster-design-draft-pipeline.md` §6.

The two second-class primitives (§6.4 plain-language toggle + §6.5 citation-graph mini-map) are not staged as DESIGN-COMPONENT drafts in this batch; they are surfaced as future-iteration candidates. The substrate may pre-emptively define their token-vocabulary slots so the engine work can land independently when the corpus reaches the size where they pay off.

## Research trail

### Done — what informed this draft

- [Wikipedia Main Page primitive audit, sub-agent A] — full primitive inventory with format invariants
- [Wikipedia article-shell anatomy under Vector 2022, sub-agent B] — full primitive inventory with weakness analysis
- [Competitive landscape audit (25 providers across 4 groups), sub-agent C] — failure-mode taxonomy + per-provider one-pagers
- [DTCG token-vocabulary proposal (DTCG 2025.10 stable, Carbon v10, Wikimedia Codex), sub-agent D] — three-tier taxonomy with full token inventory
- [Existing draft topic-wikipedia-leapfrog-design.draft.md] — reviewed to avoid duplication; this draft is genuinely additive
- [Existing draft component-home-grid.draft.md] — reviewed; this draft extends without overlap
- [naming-convention.md §2 §3 §4 §6 §10] — repo-rule design intent + 9-category set + frontmatter schema proposal
- [content-contract.md §1-§9] — engine-side rendering contract
- [conventions/cluster-design-draft-pipeline.md §1 §3 §6] — DESIGN-* schema and master-cosign requirement for TOKEN-CHANGE
- [conventions/draft-research-trail-discipline.md] — Doctrine claim #39 mandate for research trails on every draft
- [conventions/compounding-substrate.md §1 §2 §3] — three-ring architecture and the structural advantage that makes the leapfrog credible
- [DOCTRINE.md §III row 18] — the substrate sovereignty / optional intelligence / three-tier compute routing pattern
- [tacit] — Carbon Design System v10 as the substrate baseline; PointSav is a Carbon-derivative substrate

### Suggested — what project-design should consult before refinement

- [external:carbondesignsystem.com/elements/themes/tokens] — the Carbon v11 token taxonomy (priority: high) — confirm whether this proposal's three-tier model aligns with the v11 conventions or whether substrate refinement should re-anchor to the Carbon v10 model in use today
- [external:doc.wikimedia.org/codex/v1.9.0/design-tokens/definition-and-structure.html] — the Codex token taxonomy for cross-validation (priority: high) — Wikimedia Codex is the closest published precedent for a wiki-substrate token system
- [external:phabricator.wikimedia.org/T363849] — the Codex link-blue token rationale (priority: medium) — confirms `#3366cc` / `#36c` as the encyclopedic-standard blue
- [external:webbyawards.com/categories/websites-and-mobile-sites/general/reference] — the actual Webby Reference category criteria (priority: medium) — confirm award framing for the substrate's design narrative
- [pointsav-design-system/research/] — verify there is no existing token-system research that conflicts with this proposal (priority: high) — substrate sovereignty over its own conventions takes precedence
- [external:designtokens.org/tr/drafts/format/] — DTCG 2025.10 stable specification (priority: medium) — verify the alias syntax (`{path.to.token}`) and `$extends` group-inheritance pattern used in §5 of this draft are the canonical 2026 form
- [external:meta.wikimedia.org/wiki/Wikimedia_Foundation_Annual_Plan/2025-2026/Product_%26_Technology_OKRs] — re-fetch (priority: medium) — the AI-scraper-65%-of-expensive-requests claim is load-bearing for §6.3's award framing
- [external:enterprise.wikimedia.com/blog/structured-contents-wikipedia-infobox/] — verify the Structured Contents API limitations claim (priority: medium) — used in §3 weakness analysis
- [pointsav-design-system/tokens/dtcg-bundle.json] — read current state (priority: high) — the TOKEN-CHANGE draft must specify additions, not replacements, to whatever is canonical today

### Open questions — for project-design or operator

- (a) Light/dark theme switching pattern — see §7(a). Affects every component recipe.
- (b) `wiki.*` vs `ps-wiki.*` namespace prefix — see §7(b). Affects naming across all four downstream drafts.
- (c) Variable-font loading vs system-stack discipline — see §7(c). Sovereignty / dependency tradeoff.
- (d) Density toggle as token vs component state — see §7(d). Implementation strategy.
- (e) FLI-banner colour register (amber vs neutral) — see §7(e). BCSC posture interpretation.
- (f) Research-trail visual weight (chrome vs body content) — see §7(f). Editorial weight signalling.

## Provenance

This research draft was authored by a Task Claude session in the project-knowledge cluster on 2026-04-30. Primary research was conducted by four parallel Sonnet sub-agents dispatched under operator-override authorization ("do reashc with sonnet ... cross check what the other 'wiki' providers are doing") consistent with the v0.1.30 Sub-agent dispatch pattern (memory: `feedback_subagent_dispatch_pattern.md`) and v0.1.36 operator-override precedent (memory: `feedback_operator_override_sonnet_dispatch.md`). Master ratifies post-hoc.

Authorship is the Opus parent's synthesis of the four agent reports plus direct workspace-document consultation (CLAUDE.md, naming-convention.md, content-contract.md, conventions/cluster-design-draft-pipeline.md, conventions/draft-research-trail-discipline.md, DOCTRINE.md). No claim in this draft is unattributed to either an external URL, an in-cluster source document, or a sub-agent report.

The four sub-agent reports are preserved in this Task session's chat trace; if a follow-up session needs them in fuller detail than the synthesis preserves, they are available via session-trace recovery. The draft-created JSONL event for this draft is emitted to `~/Foundry/data/training-corpus/apprenticeship/prose-edit/pointsav/draft-2026-04-30-research-wikipedia-leapfrog-2030.jsonl` per cluster-wiki-draft-pipeline.md §7 and apprenticeship-substrate.md §7A.

---

## Research Update — 2026-05-15 — Hover Previews + Appearance System + Full-Width Toggle

This update extends the research substrate with three Wikipedia muscle-memory primitives that were declared in scope at iteration-2 planning but are not yet shipped in the running `app-mediakit-knowledge` binary as of 2026-05-15 (cluster `cluster/project-knowledge`, post-`ea4ad77`). The three features here are *additive Vector 2022 reader conveniences* — none requires substrate token re-anchoring, none disturbs the existing three-column layout, and all three respect the engine's no-bundler discipline (vanilla `wiki.js`, vanilla `style.css`).

Methodology: WebFetch against four canonical sources (en.wikipedia.org/wiki/Main_Page, en.wikipedia.org/wiki/Rust_(programming_language), mediawiki.org/wiki/Extension:Popups, mediawiki.org/wiki/Skin:Vector/2022, mediawiki.org/wiki/Page_Previews) plus direct inspection of the live wiki at `http://localhost:9090` (HTML markup of an article page, `static/wiki.js`, `static/style.css`, and the `/api/preview/{slug}` JSON endpoint). The WebFetch summaries returned thin technical detail for several of these pages — the Extension:Popups and Skin:Vector/2022 articles are mostly product-marketing copy with implementation specifics living in Phabricator and gerrit. Where MediaWiki documentation was non-load-bearing, the recommendations below cite the running MediaWiki implementation observed in production (Wikipedia rendering Vector 2022 today, May 2026) as the muscle-memory contract; engine implementers may verify by inspecting page source on en.wikipedia.org directly.

### 1. Hover Previews (Page Previews / ext.popups)

**Wikipedia behaviour (Vector 2022, Extension:Popups, observed May 2026).** When a reader hovers a wikilink in article body content (NOT chrome links, NOT footnote markers, NOT external links — only the main namespace article links inside `.mw-parser-output`), a card appears after an administrator-configurable dwell threshold (default ~500 ms in Extension:Popups; Wikipedia desktop runs closer to 300–500 ms; the abandon-delay before close is ~300 ms after the reader's pointer leaves both the link and the card). The card is built from the REST API endpoint `https://en.wikipedia.org/api/rest_v1/page/summary/{title}`, which returns JSON with `title`, `description` (Wikidata short description), `extract` (plain-text first ~200 chars of lead), `extract_html`, `thumbnail.source` (~320 px width), `originalimage`, `lang`, `dir`, `timestamp`, `content_urls.desktop.page`. Popup container DOM is appended to `<body>`; outer wrapper is `.mwe-popups` (root) with modifiers `.mwe-popups-fade-in-up` / `.mwe-popups-fade-in-down` for the directional reveal, and modifier `.mwe-popups-type-page` (vs `.mwe-popups-type-reference` for footnote previews). Inner structure: `.mwe-popups-container` → `.mwe-popups-discreet` → optional `<a class="mwe-popups-extract"><span class="mwe-popups-thumbnail"><img></span><h3>title</h3><p>extract</p></a>` → `.mwe-popups-settings` (cog icon, opens preference dialog). The card carries `role="tooltip"` with `aria-hidden` flipping on visibility. Keyboard users get focus-triggered previews (the `mouseenter`/`focusin` and `mouseleave`/`focusout` pair, with Escape dismissing). Screen readers are intentionally not exposed to the preview content — `aria-hidden="true"` is set, because the underlying link already carries the same `title` attribute.

Critically: Wikipedia avoids preview-storms by debouncing. Hovering five wikilinks in quick succession does NOT issue five API requests — the implementation cancels in-flight fetches when the hover target changes before completion, and once a preview is rendered for a slug it is cached in memory for the session.

Disable controls: Wikipedia exposes Page Previews on/off via the settings cog on the card itself, AND via Special:Preferences → Appearance → Reading preferences → "Show previews of page contents when hovering over links" (the underlying user preference is `popups`).

**Leapfrog 2030 state (as of 2026-05-15).** `static/wiki.js` already contains a working `initHoverCards()` implementation: a singleton `_hoverCard` div with class `.wiki-hover-card` is appended to body; `_hoverCache` memoises per-slug responses; mouseenter on any `a[data-wikilink="true"]:not(.wiki-redlink)` fires `fetch('/api/preview/' + slug)`; the response is shaped `{title, snippet, image_url, slug}` and rendered into the card. CSS is present at `style.css:1483–1521` — fixed 320 px width, serif title, sans body, box-shadow, optional 150 px object-fit cover image. The `/api/preview/{slug}` server endpoint is live and returns the documented shape.

**Gap analysis vs Wikipedia muscle-memory.** The current implementation is functional but pre-muscle-memory in three respects:

1. **No dwell delay.** The current `mouseenter` handler fires immediately. Wikipedia uses a ~300–500 ms dwell to suppress drive-through previews. Required: add a `setTimeout(showCard, 300)` on mouseenter; clear it on mouseleave before the timeout fires.
2. **No fetch cancellation on hover-target change.** A reader sweeping the cursor across five wikilinks issues five fetches; only the last writes to `_hoverCache`, but the in-flight requests still hit the server. Required: capture the fetch as an `AbortController` ref scoped to the most recent hover, abort on target change.
3. **No "Read more" affordance, no settings cog, no directional reveal.** The current card renders `<strong>title</strong><p>snippet</p>` with no closing link to the article and no off-toggle. Required: append `<a class="wiki-hover-readmore" href="/wiki/{slug}">Read more →</a>` and a small `.wiki-hover-settings` gear button that opens a localStorage-backed preference dialog. The directional reveal (`fade-in-up` vs `fade-in-down`) requires viewport-edge detection and is second-class polish; suggest deferring.
4. **No keyboard / focus parity.** Wikipedia activates previews on `focusin` as well as `mouseenter`. Required: bind the same handler to focus events; bind Escape to dismiss.
5. **No abandon-delay graceful re-entry.** Current implementation hides the card 200 ms after mouseleave. A reader who slides the cursor from link → card should see the card stay open (this works today because the card has its own mouseenter clearTimeout). Verify the same gesture works when transitioning from card → link (it currently does not, because the link's mouseleave fires the hide-timer; the card-mouseenter clears it, but mouseenter on the link does not clear it). Required: add `_hoverTimer` clear on link mouseenter (already present, but verify).

**Implementation notes (specific files).**

- `static/wiki.js` §4 (existing `initHoverCards()`): wrap the `mouseenter` body in a `setTimeout(..., HOVER_DELAY)` with `HOVER_DELAY = 300`; store the timeout handle alongside `_hoverTimer` as `_showTimer` so mouseleave can cancel both. Add `link.addEventListener('focusin', ...)` and `link.addEventListener('focusout', ...)` mirroring the mouse handlers. Add `document.addEventListener('keydown', e => { if (e.key === 'Escape' && _hoverCard) _hoverCard.style.display = 'none'; })`. Replace inline `_hoverCard.innerHTML` construction with a `renderHoverCard(data)` that includes the "Read more" affordance.
- `static/style.css` `.wiki-hover-card`: add `.wiki-hover-readmore { display: block; margin-top: 0.5rem; font-family: var(--sans); font-size: 0.8125rem; color: var(--link); text-decoration: none; }`; add CSS transition `transition: opacity 120ms ease-out, transform 120ms ease-out` and toggle between `opacity: 0; transform: translateY(4px)` (hidden) and `opacity: 1; transform: translateY(0)` (visible).
- `src/server.rs` `/api/preview/{slug}` handler: confirmed working today. No server changes required. Optional Phase 2 polish — extend the response with `last_edited` (so the card can carry a freshness micro-indicator consistent with §6.3 freshness-ribbon design), and `description` (Wikidata-equivalent: pulled from `Frontmatter.short_description` already in render.rs).
- ARIA: set `role="tooltip"` and `aria-hidden="true"` on the hover card root; flip `aria-hidden="false"` on display. Set `aria-describedby` on the underlying link to the card's id when shown.
- localStorage key for the disable preference: `wiki-hover-previews-enabled` (default `'1'`). When `'0'`, `initHoverCards()` early-returns. Settings cog opens a small dialog with two radio buttons.

**Citation density interaction.** The IVC verification band ships Off / Exceptions / All citation density (already in place). Hover previews should respect citation density: when `wiki-citation-density === 'off'`, the preview card should NOT show citation marks inline in the snippet. Verify rendering pipeline strips IVC-mark spans from the preview snippet at API time (server-side strip is cleaner than client-side regex).

### 2. Appearance / Theme System (Day / Night / OS mode)

**Wikipedia behaviour (Vector 2022 Appearance panel, observed May 2026).** Vector 2022 exposes an **Appearance** panel in the right-rail page-tools column (default position; user can pin or unpin from the rail via the `vector-appearance-pinned` user-preference / cookie). The pinnable header is `<div class="vector-appearance" id="vector-appearance">` containing a `<h3>Appearance</h3>`, a pin/unpin toggle button with `data-event-name="ui.sidebar-appearance"`, and three feature groups:

1. **Text** (font size): Small / Standard / Large — implemented as radio inputs `name="skin-client-pref-vector-font-size-group"` with `value="0"` / `"1"` / `"2"`. The selected value writes to localStorage key `mwclientpreferences` (a JSON blob containing all client-prefs) AND to a cookie `mwclientpreferences` for cross-tab consistency, AND sets a class on `<html>` matching the pattern `vector-feature-custom-font-size-clientpref-{0|1|2}`. CSS reads the class and adjusts `--font-size-base` / line-height scale.
2. **Width** (column width): Standard / Wide — radio `name="skin-client-pref-vector-feature-limited-width-group"` with `value="1"` / `"0"` (1 = Standard / limited; 0 = Wide / full-width). The selected value sets `<html class="vector-feature-limited-width-clientpref-{0|1}">`. See §3 below for the full discussion of width.
3. **Color** (theme): Automatic / Light / Dark — radio `name="skin-client-pref-skin-theme-group"` with values `"os"` / `"day"` / `"night"`. The selected value sets `<html class="skin-theme-clientpref-{os|day|night}">`. When `os`, a CSS `@media (prefers-color-scheme: dark)` rule activates the dark palette; when `day`, light is forced; when `night`, dark is forced.

Persistence: anonymous users get the cookie `mwclientpreferences` (max-age 365 days); logged-in users get the value persisted to their account `Special:Preferences` and the cookie acts as a session cache. The localStorage key `mwclientpreferences` is read on page load by `mediawiki.skinning.clientPreferences` to apply the class BEFORE first paint (inline `<script>` in `<head>` runs the read-and-apply so there is no flash of unstyled theme).

CSS implementation: Vector 2022 uses ~600 CSS custom properties from Codex (e.g., `--color-base`, `--color-base--emphasized`, `--background-color-base`, `--background-color-neutral`, `--border-color-base`, `--color-link`, `--color-progressive`, etc). The light-theme defaults sit on `:root` or `html`; the dark overrides sit on `html.skin-theme-clientpref-night`, AND under `@media (prefers-color-scheme: dark)` scoped to `html.skin-theme-clientpref-os`. Codex specifically inverts the *base* palette, the *link* palette, and the *border* palette; image rendering is dimmed with a `filter: brightness(0.85) contrast(1.1)` rule on `img:not([class*="invert"]):not([class*="skin-invert"])` in night mode, with an opt-in/opt-out per-image class system (`class="skin-invert"` to flip a black-on-white SVG, `class="skin-invert-image"` for raster images, `class="notpageimage"` to skip).

**Leapfrog 2030 state (as of 2026-05-15).** Zero theme code today. `static/wiki.js` has no theme handler. `static/style.css` defines a single light palette on `:root` (`--bg: #ffffff`, `--fg: #202122`, `--link: #3366cc`, `--link-visited: #6b4ba1`, `--border: #a2a9b1`, plus the Codex aliases `--mw-color-link`, `--mw-color-base-10`, `--mw-color-base-20`, `--mw-color-link-redlink`) — there is no dark override, no Appearance panel in the right rail, no localStorage key for theme.

**Recommended implementation.**

*CSS substrate.* The existing `:root` block at `style.css:9–35` IS the substrate; the recommended pattern is to keep the light palette on `:root` and add a `html.skin-theme-clientpref-night` selector (and a `@media (prefers-color-scheme: dark) { html.skin-theme-clientpref-os { ... } }` block) that overrides every variable. Concrete dark palette (Codex-derived, validated for WCAG AA contrast against the existing typography stack):

```css
html.skin-theme-clientpref-night {
  --bg: #101418;
  --bg-chrome: #1c2128;
  --bg-aside: #2c241a;
  --fg: #eaecf0;
  --fg-muted: #a2a9b1;
  --border: #54595d;
  --link: #88a9ff;
  --link-visited: #c8b3ff;
  --mw-color-link-redlink: #ff6e6e;
  --mw-color-base-10: var(--bg-chrome);
  --mw-color-base-20: #2a2f37;
  --mw-color-base-50: var(--border);
  --ivc-band-bg: #15212e;
  --ivc-band-border: #243446;
  --toc-bg: var(--mw-color-base-10);
}
@media (prefers-color-scheme: dark) {
  html.skin-theme-clientpref-os {
    /* duplicate of the night block above */
  }
}
```

Images: add a single rule `html.skin-theme-clientpref-night img:not(.skin-invert-image):not([class*="notpageimage"]) { filter: brightness(0.92) contrast(1.05); }` to match the Wikipedia dimming behaviour without inverting raster content.

*Toggle UI.* Vector 2022 places the Appearance panel in the right rail. Leapfrog already has a right-rail page-tools column (see §1 of this research draft's existing §3 article-shell analysis). The toggle should render as a new portlet between "Page tools" and the IVC band. Concrete DOM (rendered server-side in `src/server.rs`'s right-rail builder):

```html
<nav class="vector-appearance" id="vector-appearance" aria-label="Appearance">
  <h3 class="wiki-portlet-heading">Appearance</h3>
  <fieldset class="wiki-clientpref-group">
    <legend>Color</legend>
    <label><input type="radio" name="wiki-theme" value="os"> Automatic</label>
    <label><input type="radio" name="wiki-theme" value="day"> Light</label>
    <label><input type="radio" name="wiki-theme" value="night"> Dark</label>
  </fieldset>
  <fieldset class="wiki-clientpref-group">
    <legend>Width</legend>
    <label><input type="radio" name="wiki-width" value="standard"> Standard</label>
    <label><input type="radio" name="wiki-width" value="wide"> Wide</label>
  </fieldset>
</nav>
```

A compact alternative for tight viewports (and Wikipedia's actual mobile pattern): a single sun/moon icon button in the header that cycles `day → night → os → day` on click, with the radio panel reserved for desktop right-rail. Recommend both — header icon for fast-toggle muscle memory (most Wikipedia readers in 2026 use the header icon, not the panel), panel for explicit preferences. The header icon lives next to the language switcher (🌐) at `:root .wiki-lang-switcher`; add a sibling `.wiki-appearance-quick-toggle` button.

*JavaScript handler.* New section in `static/wiki.js`:

```javascript
/* ------------------------------------------------------------------ *
 * 16. Appearance / Theme System                                       *
 * ------------------------------------------------------------------ */

var STORAGE_KEY_THEME = 'wiki-theme';
var STORAGE_KEY_WIDTH = 'wiki-width';
var THEME_DEFAULT = 'os';
var WIDTH_DEFAULT = 'standard';

function applyTheme(value) {
  var html = document.documentElement;
  html.classList.remove(
    'skin-theme-clientpref-os',
    'skin-theme-clientpref-day',
    'skin-theme-clientpref-night'
  );
  html.classList.add('skin-theme-clientpref-' + value);
}

function applyWidth(value) {
  var html = document.documentElement;
  html.classList.remove(
    'wiki-width-clientpref-standard',
    'wiki-width-clientpref-wide'
  );
  html.classList.add('wiki-width-clientpref-' + value);
}

function initAppearance() {
  var savedTheme = localStorage.getItem(STORAGE_KEY_THEME) || THEME_DEFAULT;
  var savedWidth = localStorage.getItem(STORAGE_KEY_WIDTH) || WIDTH_DEFAULT;
  applyTheme(savedTheme);
  applyWidth(savedWidth);

  /* wire the radios */
  document.querySelectorAll('input[name="wiki-theme"]').forEach(function (r) {
    r.checked = (r.value === savedTheme);
    r.addEventListener('change', function () {
      localStorage.setItem(STORAGE_KEY_THEME, r.value);
      applyTheme(r.value);
    });
  });
  document.querySelectorAll('input[name="wiki-width"]').forEach(function (r) {
    r.checked = (r.value === savedWidth);
    r.addEventListener('change', function () {
      localStorage.setItem(STORAGE_KEY_WIDTH, r.value);
      applyWidth(r.value);
    });
  });

  /* header quick-toggle (cycles day → night → os) */
  var quick = document.getElementById('wiki-appearance-quick-toggle');
  if (quick) {
    quick.addEventListener('click', function () {
      var current = localStorage.getItem(STORAGE_KEY_THEME) || THEME_DEFAULT;
      var next = current === 'day' ? 'night'
               : current === 'night' ? 'os'
               : 'day';
      localStorage.setItem(STORAGE_KEY_THEME, next);
      applyTheme(next);
      var radio = document.querySelector('input[name="wiki-theme"][value="' + next + '"]');
      if (radio) radio.checked = true;
    });
  }
}
```

**Flash-of-unstyled-theme (FOUT) suppression.** Wikipedia inlines a tiny `<script>` in `<head>` that reads `localStorage.mwclientpreferences` and sets the html class BEFORE the body renders. Leapfrog must do the same — `wiki.js` runs `defer`, which fires AFTER the body parses, so a deferred theme apply causes a visible flash. Recommend: render an inline `<script>` block in the document head (`src/render.rs` or wherever `<head>` is composed) with the minimal read-and-apply logic:

```html
<script>
(function() {
  try {
    var t = localStorage.getItem('wiki-theme') || 'os';
    var w = localStorage.getItem('wiki-width') || 'standard';
    document.documentElement.classList.add('skin-theme-clientpref-' + t);
    document.documentElement.classList.add('wiki-width-clientpref-' + w);
  } catch (e) {}
})();
</script>
```

This is the single inline script allowed by the engine's progressive-enhancement discipline — it is two operations, no external dependencies, and is the standard pattern across every Vector 2022 implementation.

### 3. Full-Width / Limited-Width Toggle

**Wikipedia behaviour.** Vector 2022 introduced a width-toggle in 2023. The default is "Standard" — content column capped at ~960 px (Codex `--width-breakpoint-desktop-wide` / `1600px` viewport, content `~960px`). Reader can opt in to "Wide" mode, which lifts the cap and spans the full viewport (with a 1.25 rem gutter on each side). Two surfaces expose the toggle:

1. **Appearance panel** in the right rail — the Width radio group described in §2 above.
2. **Inline expand button** at the right edge of the article body, rendered as a small button with an arrow icon (`<span class="vector-toolbox-expand">⇄</span>` or similar; the exact glyph is a horizontal double-arrow, sometimes rendered as `↔`). Positioned floating at the bottom-right corner of the content column when in Standard width; clicking toggles to Wide. In Wide mode, the same button shows a "collapse" affordance (the icon reverses direction).

CSS: `html.vector-feature-limited-width-clientpref-1` (limited / Standard) sets `--mw-page-container-max-width: 960px` or the equivalent grid-template-columns constraint; `html.vector-feature-limited-width-clientpref-0` (wide) sets it to `none` / `100%`. The Codex breakpoint is implemented via a CSS custom property that the page-container grid reads — not via a media query — because the override is a user preference, not a viewport size.

Persistence: same cookie + localStorage pattern as theme (`mwclientpreferences` JSON blob). Logged-in users persist to account.

Importantly: width and theme are independent. Reader can run Wide + Day, or Standard + Night, in any combination.

**Leapfrog 2030 state.** Zero width-toggle code today. Current layout (`.wiki-layout`) is `grid-template-columns: 220px 1fr 200px; max-width: var(--max-content-width)` where `--max-content-width: 76em` (~1216 px) — already wider than Wikipedia's default Standard. The content column has no explicit max-width separate from the page container.

**Recommended implementation.**

*CSS substrate.* Introduce two new custom properties scoped per-mode:

```css
:root {
  --wiki-page-max-width-standard: 76em;   /* current default */
  --wiki-page-max-width-wide: none;
}

html.wiki-width-clientpref-standard .wiki-layout {
  max-width: var(--wiki-page-max-width-standard);
}
html.wiki-width-clientpref-wide .wiki-layout {
  max-width: var(--wiki-page-max-width-wide);
  /* gutters: */
  padding-left: 1.25rem;
  padding-right: 1.25rem;
}
```

Default `--wiki-page-max-width-standard` is the existing `76em`. Setting `wide` lifts the cap; the existing inner grid (`220px 1fr 200px`) naturally absorbs the extra space into the `1fr` column. Verify the rail columns don't bloat — if they do, add `grid-template-columns: 220px minmax(0, 1fr) 200px` to constrain.

*Inline expand button.* Render in the right-rail OR floating at the article body's bottom-right corner. Wikipedia uses the latter (a floating button at the content-column edge); recommend the floating pattern for muscle-memory parity:

```html
<button class="wiki-width-expand-btn"
        id="wiki-width-expand-btn"
        aria-label="Expand to full width"
        title="Toggle reading width">
  <span aria-hidden="true">⇄</span>
</button>
```

```css
.wiki-width-expand-btn {
  position: fixed;
  bottom: 1.5rem;
  right: 1.5rem;
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 50%;
  border: 1px solid var(--border);
  background: var(--bg-chrome);
  color: var(--fg);
  font-size: 1.25rem;
  cursor: pointer;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
  z-index: 100;
}
.wiki-width-expand-btn:hover { background: var(--mw-color-base-20); }
html.wiki-width-clientpref-wide .wiki-width-expand-btn span {
  transform: rotate(180deg);
  display: inline-block;
}
@media (max-width: 960px) {
  .wiki-width-expand-btn { display: none; } /* width toggle is meaningless on narrow viewports */
}
```

The button's click handler is wired by the same `initAppearance()` from §2:

```javascript
var expandBtn = document.getElementById('wiki-width-expand-btn');
if (expandBtn) {
  expandBtn.addEventListener('click', function () {
    var current = localStorage.getItem(STORAGE_KEY_WIDTH) || WIDTH_DEFAULT;
    var next = current === 'standard' ? 'wide' : 'standard';
    localStorage.setItem(STORAGE_KEY_WIDTH, next);
    applyWidth(next);
    var radio = document.querySelector('input[name="wiki-width"][value="' + next + '"]');
    if (radio) radio.checked = true;
    expandBtn.setAttribute('aria-label',
      next === 'wide' ? 'Collapse to standard width' : 'Expand to full width');
  });
}
```

*FOUT suppression.* Same inline `<script>` in `<head>` from §2 already sets `wiki-width-clientpref-{standard|wide}` before first paint. No additional inline script needed.

**Open implementation question — does this interact with the 200 px right-rail?** Wikipedia's Wide mode keeps the right rail; it just lifts the content max-width. Leapfrog's `.wiki-layout` has the same shape — `220px 1fr 200px` — so the recommendation is to lift the *grid container's* max-width and let the `1fr` column absorb the gain. An alternative is to *collapse* the right rail in Wide mode (giving the content 100% of the lifted width, no rails). This is more aggressive and not what Wikipedia does — recommend matching Wikipedia (keep rails, widen middle).

### 4. Open Questions for Project-Design

Resolving these is gateway-time scope. The three features above can ship without these answered, but the answers determine substrate-level token additions for `pointsav-design-system`:

**(g) Hover preview: snippet length and image policy.** Wikipedia's REST `summary` endpoint truncates `extract` to ~200 chars; the existing `/api/preview/{slug}` endpoint uses an ellipsis-truncated snippet of similar size. Does the substrate ratify 200 chars as the canonical preview-snippet length, or should it be operator-tunable per article? And: when a TOPIC has no image, does the card collapse to text-only (current behaviour), or always reserve a 60×60 placeholder block (a Wikipedia variant pattern)? Recommend text-only collapse for typography integrity, but flag for substrate review.

**(h) Dark-mode link colours — sRGB or P3 wide-gamut?** The recommended `--link: #88a9ff` (Codex-derived) is an sRGB value. Codex 2025 supports P3 wide-gamut variants (e.g., `color(display-p3 0.53 0.66 1)`) for displays that support them. Project-design ruling needed: do we ship sRGB-only (universal compatibility), sRGB + P3 with `@supports (color: color(display-p3 0 0 0))` (modern displays get richer link colour), or hold for substrate-wide P3 audit? Affects ~12 colour tokens across the dark palette.

**(i) Width-toggle: does Wide preserve the IVC band as a centered chrome strip, or stretch it to full width?** The IVC verification band currently spans the full content column. In Wide mode, the band stretches to ~1800 px on a typical 1920 px viewport, which is much larger than the band's typography is designed for. Recommend: cap the IVC band at a `max-width: 76em; margin: 0 auto;` independent of the page container max-width, so the band typography stays consistent. Or: allow the band to stretch and re-design the band typography for wider compositions. Project-design ruling needed.

**(j) Should the right rail collapse on narrow viewports independently of the width toggle?** The current layout's `220px 1fr 200px` grid wraps poorly under 960 px. Wikipedia's mobile Vector 2022 collapses the right rail entirely into a hamburger menu under 720 px. Leapfrog's mobile drawer pattern already supports this; verify the right-rail Appearance panel migrates into the mobile drawer alongside the existing TOC / nav drawers. Implementation detail, not a token question, but worth surfacing.

**(k) Quick-toggle icon: sun/moon, or contrast (◐) glyph?** Wikipedia uses a sun/moon icon (Codex-icon `theme-light` / `theme-dark`). Codex ships these as SVG sprites. Leapfrog's text-icon discipline (the existing 🌐, ☰, § icons in the header) suggests a Unicode glyph rather than an SVG sprite — recommend `◐` (half-circle, theme-agnostic) cycling through, or `☀` (day) / `☾` (night) / `◐` (os). Substrate ruling on whether Leapfrog uses Codex SVG icons or stays on Unicode glyphs for chrome.

**(l) Persistence: localStorage vs server-side cookie.** Wikipedia uses both — localStorage for fast read on next page load, cookie for cross-tab consistency and (for logged-in users) sync to Special:Preferences. Leapfrog currently has no user-account substrate. Recommend localStorage-only for v1; cookie sync deferred to whenever a user account substrate ships. Document this so a future session does not unilaterally add cookies.

### 5. Substrate-side scope summary (additive to §9 of original research)

The three features above add work to two of the four downstream drafts already staged, and a new draft is recommended:

- **`token-knowledge-wiki-baseline.draft.md`** (DESIGN-TOKEN-CHANGE, Master co-sign) — extend with: dark-palette token bundle (15 token overrides under `wiki.theme.night.*` semantic tier), width tokens (`wiki.layout.width.standard` / `.wide`), hover-card tokens (`wiki.hover.bg` / `.border` / `.shadow` / `.delay-ms`). These are not new design surfaces — they are dark-mode and width variants of existing tokens.
- **NEW: `component-appearance-panel.draft.md`** (DESIGN-COMPONENT) — recipe for the right-rail Appearance portlet (radio-group fieldset; icon header; pin/unpin affordance). Stage to drafts-outbound for project-design pickup.
- **NEW: `component-hover-preview-card.draft.md`** (DESIGN-COMPONENT) — recipe for the hover-card primitive. Specifies the 320 px width, image aspect ratio, typography (serif title + sans body), shadow + border, settings cog, Read more link, the 300 ms dwell, the focus-parity behaviour, the AbortController fetch cancellation.
- **`component-research-trail-footer.draft.md`** (existing) — extend with dark-palette `--research-trail-bg-night` value. No structural change.

No new TOPIC drafts are needed for this iteration — the Wikipedia muscle-memory contract is already documented in this research file and in `topic-wikipedia-leapfrog-design.draft.md`. Engine implementation can proceed in parallel with substrate refinement; the engine sets the localStorage keys and CSS class names recommended above, and substrate refinement may rename them at gateway time without breaking the engine (the engine's class names are not load-bearing across the gateway — only the *behaviour contract* is).

### 6. Sequencing recommendation for the engine implementer

1. **Inline FOUT-suppression script in `<head>`** — small, must land first; everything else assumes it is present.
2. **Theme CSS substrate** — dark-palette block in `style.css`; no JS yet, theme can be tested by manually setting `html.skin-theme-clientpref-night` via DevTools.
3. **Theme quick-toggle in header** — single icon button next to the language switcher; cycles day → night → os.
4. **Width CSS substrate** — `wiki-width-clientpref-{standard|wide}` classes with the max-width override; no JS yet, testable via DevTools.
5. **Floating expand button** — bottom-right of viewport; wires width toggle.
6. **Appearance right-rail panel** — full radio panel with both theme and width groups; pin/unpin behaviour.
7. **Hover preview polish** — dwell delay, AbortController, Read more link, focus parity, Escape dismiss, settings cog.

Each step is independently shippable and independently testable. Steps 1–5 are pure CSS + minimal JS; step 6 is the substantial new chrome; step 7 polishes an existing feature. Total engine LoC estimate: ~250 lines wiki.js, ~180 lines style.css, ~30 lines render.rs / server.rs for the right-rail panel and head-script emission.

The Foundry Doctrine 2030 has 39 ratified claims as of v0.0.10. This research operationalises three of them at the wiki-engine surface: claim #21 (Role-Conditioned Cluster Adapters — the cluster-project-knowledge adapter trains from this corpus), claim #35 (The Reverse-Funnel Editorial Pattern — this draft enters the project-language gateway via the standard pickup), and claim #39 (Draft Research Trail Discipline — this draft itself complies with the research-trail mandate, demonstrating the structure it proposes the substrate adopt at article scale).
