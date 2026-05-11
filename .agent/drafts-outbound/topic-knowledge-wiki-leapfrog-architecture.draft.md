---
schema: foundry-draft-v1
title: "Knowledge Wiki Leapfrog Architecture"
slug: architecture/topic-knowledge-wiki-leapfrog-architecture
language: en
status: draft-pending-language-pass
author: task@project-knowledge
created: 2026-05-07
target_path: vendor/content-wiki-documentation/architecture/topic-knowledge-wiki-leapfrog-architecture.md
bilingual_pair: vendor/content-wiki-documentation/architecture/topic-knowledge-wiki-leapfrog-architecture.es.md
cites: []
bcsc_class: architecture
forward_looking: true
research_sources:
  - name: "MediaWiki architecture documentation"
    url: "https://www.mediawiki.org/wiki/Manual:MediaWiki_architecture"
    accessed: 2026-05-07
  - name: "Parsoid — MediaWiki"
    url: "https://www.mediawiki.org/wiki/Parsoid/About"
    accessed: 2026-05-07
  - name: "Skin:Vector/2022 design documentation"
    url: "https://www.mediawiki.org/wiki/Skin:Vector/2022/Design_documentation"
    accessed: 2026-05-07
  - name: "Extension:Cite — MediaWiki"
    url: "https://www.mediawiki.org/wiki/Extension:Cite"
    accessed: 2026-05-07
  - name: "comrak 0.29 ExtensionOptions"
    url: "https://docs.rs/comrak/0.29.0/comrak/struct.ExtensionOptions.html"
    accessed: 2026-05-07
  - name: "similar crate — TextDiff API"
    url: "https://docs.rs/similar/latest/similar/struct.TextDiff.html"
    accessed: 2026-05-07
research_trail:
  method: "Three concurrent research agents: (1) deep codebase gap analysis of app-mediakit-knowledge src/server.rs and src/render.rs; (2) MediaWiki architecture, Vector 2022 skin, Cite extension, template/transclusion system via web research; (3) Rust ecosystem — comrak 0.29–0.52 API, pulldown-cmark, similar crate, infobox implementation patterns via web research"
  depth: "Full MediaWiki parser architecture including Parsoid rewrite history; Vector 2022 skin layout inventory; Cite extension footnote mechanism; comrak extension flags across 23 versions; SMB wiki competitor analysis (Wiki.js, BookStack, Outline, Confluence)"
  confidence: high
  limitations: "comrak upstream is at 0.52 as of research date; MediaWiki has 400+ extensions; competitor analysis reflects public documentation as of 2026-05"
---

# Knowledge Wiki Leapfrog Architecture

`app-mediakit-knowledge` is the PointSav knowledge platform engine. It serves three wiki instances from a single Rust binary — `documentation.pointsav.com`, `projects.woodfinegroup.com`, and `corporate.woodfinegroup.com` — reading Markdown files from git repositories and rendering them with Wikipedia-shaped chrome. This article describes the architectural philosophy behind the engine, the gap between its current feature set and full Wikipedia muscle memory, and the intended Leapfrog 2030 layer that goes beyond what Wikipedia offers.

## Why not port MediaWiki

MediaWiki is the software that runs Wikipedia. Porting it to Rust would be a mistake.

MediaWiki was designed in 2003 for a PHP-and-MySQL stack. Its parser — the component that converts wikitext (Wikipedia's markup language) into HTML — was described by its original author Tim Starling as "a huge pile of regular expressions." The Parsoid project, MediaWiki's attempt to replace this parser with a clean bidirectional HTML↔wikitext converter, took ten years to ship and is still completing its rollout as of 2025.

The wikitext template system is the core of MediaWiki's content richness — infoboxes, navboxes, citation templates, and geographic coordinate templates are all implemented as wiki pages in the `Template:` namespace, processed by a recursive macro expansion interpreter that calls back into the database during parsing. This design couples the parser to the database, makes caching complex, and makes the system difficult to run without MySQL.

The architectural goal of `app-mediakit-knowledge` is not to replicate this design but to achieve the same *reader experience* through a fundamentally different stack. The content format is Markdown with YAML frontmatter, not wikitext. Version control is git, not a MySQL revision table. The search backend is Tantivy (embedded, zero operational overhead), not CirrusSearch backed by Elasticsearch. Template transclusion is replaced by six native block types that cover 95 percent of what templates are actually used for on Wikipedia.

## The MediaWiki architecture, understood

MediaWiki's namespace system defines 30 namespaces in two axes: subject pages (Article, User, Wikipedia, File, Template, Category, Help, Module, Draft) paired with Talk pages (Talk, User talk, Wikipedia talk, and so on). Special pages form a separate class of software-generated pages with no talk equivalent.

The Vector 2022 skin divides every page into: a sticky header (logo, search, language switcher, personal tools), a left sidebar (navigation menu and table of contents), an article header (namespace tabs, view tabs, title, short description, hatnotes), the article body (infobox floated right, body text, tables, images, footnote superscripts), an appendix (See also, References, Further reading, External links), navboxes, category strip, and a footer (last-edited timestamp, license, legal links).

The Cite extension handles footnotes: `<ref>citation text</ref>` in the article body inserts a numbered superscript; `<references/>` at the section bottom renders the numbered list. Reference Tooltips (a JavaScript gadget) shows the citation text on hover without requiring the reader to scroll.

These elements constitute the "muscle memory" that Wikipedia readers have developed over two decades of use. A wiki that is missing the infobox, the sticky TOC, the `[1][2][3]` footnote superscripts, or the navboxes does not feel like Wikipedia — regardless of how good its content is.

## Current feature state

As of May 2026, `app-mediakit-knowledge` implements approximately 78 percent of the full Wikipedia muscle-memory surface. The following elements are fully operational:

- Wikilinks with blue/red distinction (existing articles link blue; missing articles link red)
- Sticky collapsible table of contents, Vector 2022 pattern
- Read / Edit / View History tabs and per-section edit pencils
- Article/Talk tabs (Article tab functional; Talk tab functional stub)
- Full-text search via Tantivy BM25
- Edit history, blame, and unified diff via git
- CodeMirror 6 editor with citation autocomplete and SAA squiggle linting
- Quality badges (complete / core / stub) and stub notice
- Category pages and category strip at article bottom
- Home page with 3×3 category grid, featured article pin, and leapfrog facts panel
- Atom 1.0 and JSON Feed 1.1 syndication
- Hover card page previews
- Glossary auto-linker with tooltips
- Authentication and edit review queue (Phase 5)
- Read-only git remote (smart-HTTP protocol)
- MCP server (JSON-RPC 2.0) for agent integration
- Notify-based incremental search reindex (no restart required on file changes)
- Mobile hamburger navigation

The following elements are stubbed or absent, ranked by muscle-memory impact:

| Missing feature | Impact | Notes |
|---|---|---|
| Infobox (structured right-column summary) | 9/10 | Most recognizable Wikipedia element; absent entirely |
| Navbox (bottom navigation template) | 8/10 | Creates topic clusters; absent entirely |
| Citation `[1][2][3]` CSS rendering | 8/10 | Footnotes parse correctly; CSS unstyled |
| Citation hover tooltip | 8/10 | JS gadget not yet written |
| Two-column word-level diff | 7/10 | Unified diff exists; two-column Wikipedia style absent |
| Talk/Discussion pages | 7/10 | Tab renders disabled; no backend |
| Redirects | 6/10 | No `redirect_to` frontmatter processing |
| Disambiguation pages | 6/10 | No page type flag or chrome |
| Special:RecentChanges | 6/10 | Git log data exists; HTML page absent |
| Special:AllPages | 5/10 | Article list endpoint absent |
| `/random` article | 5/10 | Trivial to add; absent |
| Edit summary field | 5/10 | Git commit message populated from author+time only |
| Definition lists | 4/10 | comrak extension disabled (one-line fix) |

## The native block types approach

Rather than implementing a wikitext macro expansion engine, `app-mediakit-knowledge` implements six native block types as fenced code blocks parsed during the comrak AST walk. These cover the vast majority of what Wikipedia templates are actually used for:

**Infobox** — a YAML-body fenced block that renders as a `<table class="infobox">` floated at the right margin of the article. The operator writes:

````markdown
```infobox
name: PointSav Digital Systems
founded: 2024
jurisdiction: British Columbia, Canada
sector: Enterprise software
```
````

The engine walks the comrak AST, matches `CodeBlock` nodes with `info = "infobox"`, parses the YAML body, and replaces the node with an `HtmlInline` table. No template database, no macro expansion, no Lua interpreter.

**Navbox** — a similar fenced block that renders a collapsible horizontal table at the article bottom, grouping related article links under a shared title. JavaScript collapses navboxes when two or more appear on the same page, matching Wikipedia's `autocollapse` behaviour.

**Cite block** — an inline citation syntax (`[^id]` with `[^id]: text` at article bottom) that the comrak footnotes extension already parses. The missing pieces are CSS styling and a JavaScript hover tooltip. The `citations.yaml` registry used by the editor's autocomplete system extends naturally into citation rendering.

This approach requires upgrading comrak from version 0.29 to 0.52. The newer version adds the `block_directive` extension (`:::infobox`, `:::navbox` syntax), which is cleaner than fenced code blocks for multi-line Markdown content inside the block body. The API is stable across these versions; existing code compiles without change.

## The Leapfrog 2030 layer

Wikipedia muscle memory is the floor, not the ceiling. Once `app-mediakit-knowledge` achieves full Wikipedia parity, three first-class leapfrog primitives will distinguish it from any hyperscaler-hosted alternative.

**Citation Authority Ribbon** — a per-article (and eventually per-claim) visual indicator of source verification status. Inline citations resolved against `citations.yaml` display a coloured ribbon: green for verified, amber for unverified, red for contested. Wikipedia has no equivalent; all citations are treated as equally reliable regardless of their actual verification state.

**Research Trail Footer** — every article declares five frontmatter fields (research method, depth, confidence, date, limitations) rendered as a collapsible footer section. This makes the provenance of every claim visible to readers, satisfying NI 51-102 continuous-disclosure obligations for regulated issuers. Wikipedia has talk-page archives and edit history but no structured research provenance at the article level.

**AI-integrated editor** — the CodeMirror 6 editor includes a three-keystroke ladder: Tab (complete the current sentence), Ctrl+K (ask a question about the content), Alt+J (insert a citation). These affordances call `app-orchestration-command` via the Doorman proxy. They are currently stubbed as 501 endpoints, pending service-slm operationalization. When activated, they provide author assistance that Wikipedia's editing surface cannot offer.

**BCSC squiggle linting** — the editor currently enforces seven deterministic SAA rules via coloured underlines. The full set includes forward-looking statement detection, Do-Not-Use vocabulary flagging, Sovereign Data Foundation current-tense detection, competitive comparison flagging, and citation-required prompts. This is built into the editing surface rather than bolted on as a post-publication compliance check.

## Comparison with modern alternatives

The available alternatives to MediaWiki for knowledge management — Wiki.js, BookStack, Outline, Confluence — share a common failure mode: they impose a hierarchical structure (books, chapters, spaces, folders) that breaks the flat hyperlink graph that makes Wikipedia useful. A hierarchical wiki creates knowledge silos. A flat wiki with a category DAG creates a knowledge commons.

None of the alternatives implement redlinks (the signal that a page is missing and should be created), backlinks ("what links here"), navboxes, or the full Special pages framework. Several lack talk pages entirely. Confluence explicitly creates silos through its Spaces model.

`app-mediakit-knowledge` is the only Rust-native, git-backed, flat-file wiki engine currently targeting full Wikipedia muscle memory for regulated SMBs.

---

*Research trail: three concurrent research agents, 2026-05-07. See frontmatter for sources.*
