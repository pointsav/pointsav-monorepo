---
artifact: editorial-qa-schema
title: Corpus schema
slug: corpus-schema
version: 1.0
status: active
created: 2026-05-21
owner: project-editorial
consumers:
  - editorial-lint.py (Track D / D1 — project-editorial)
  - validate_editorial_standards (app-mediakit-knowledge engine — project-knowledge)
companion: editorial-standard.md
---

# Corpus schema

The machine-readable schema of the Foundry editorial corpus: the document
classes, their frontmatter, the file and directory layout, and the body
structure rules. It consolidates what is otherwise spread across
`content-wiki-documentation/.agent/rules/content-contract.md`,
`naming-convention.md`, and `conventions/draft-research-trail-discipline.md`
into one reference that the linter and the engine validator both read.

`editorial-standard.md` is the companion document — it carries the *prose*
rules (Gate-0, registers, banned vocabulary, failure modes). This file carries
the *structural* schema.

## 1. Document classes

| `schema:` value | Class | Where it lives |
|---|---|---|
| `foundry-doc-v1` | Published wiki article (TOPIC, GUIDE, category landing, home) | a `content-wiki-*` repo |
| `foundry-draft-v1` | Draft in a draft pipeline, pre-publication | `.agent/drafts-outbound/` of any cluster |

A file with neither schema is linted generically (banned vocabulary and
sentence length only) and raises an advisory.

## 2. `foundry-doc-v1` frontmatter

YAML between `---` delimiters at file start.

**Required:** `title`, `slug`, `category`.

**Optional:** `subcategory` (metadata only — being retired),
`last_edited` (`YYYY-MM-DD`), `editor`, `status`
(`stable` | `pre-build` | `draft` | `stub`), `references` (footnote list),
`bcsc_class`, `paired_with`, `cites`, `lang` (on `.es.md` files).

**Rules:** `slug` equals the filename stem. `category` equals the parent
directory name (`root` for `index.md`). Slugs are globally unique and immortal
after publish; a rename is handled with `aliases`, never by mutation.

## 3. `foundry-draft-v1` frontmatter

A draft carries pipeline-routing fields plus the **five research-trail fields**
mandated by `draft-research-trail-discipline.md` (Doctrine claim #39).

**Routing fields:** `schema`, `state`, `originating_cluster`, `target_repo`,
`target_path`, `target_filename`, `audience`, `bcsc_class`,
`language_protocol`, `authored`, `authored_by`, `authored_with`.

**Research-trail fields (all five required):**

| Field | Type | Notes |
|---|---|---|
| `research_done_count` | integer | 0 valid for trivial drafts |
| `research_suggested_count` | integer | 0 valid when nothing to suggest |
| `open_questions_count` | integer | 0 valid when nothing unresolved |
| `research_provenance` | enum | `direct-consultation` \| `sub-agent` \| `citation-registry` \| `mixed` \| `tacit` \| `none` |
| `research_inline` | boolean | `true` when the body carries the research-trail section |

A draft with `research_inline: true` carries a research-trail section in the
body. `research_inline: false` is permitted only with
`research_provenance: none` or when the draft references a paired
sub-agent-results file.

## 4. File and directory layout

- **Filename:** lowercase ASCII, kebab-case, single hyphens, no leading or
  trailing hyphen. The stem is the slug.
- **Bilingual pair:** every TOPIC ships as `X.md` (English) plus `X.es.md`
  (Spanish strategic adaptation). GUIDEs are English-only. Drafts in a draft
  pipeline carry the pair too.
- **Directory depth:** content categories are two levels at most
  (`<category>/<slug>.md`). Subcategory is a frontmatter field, not a directory.
- **Category landing:** `<category>/_index.md`, one per category.
- **Home:** `index.md` at repo root, `category: root`.

## 5. Body structure rules

- **No body H1.** The title is supplied from frontmatter `title:`; the body
  carries no `# H1` line (content-contract §5.2). A `foundry-draft-v1` working
  document is exempt — it carries a document title that is stripped on publish.
- **Headings:** `## H2` and `### H3` enter the table of contents; `####` and
  deeper render but do not.
- **Terminal section order:** where present, the closing sections appear last
  and in this order — *See also*, then *References*, then *External links*.
- **Wikilinks:** `[[slug]]` or `[[slug|Display Text]]`; an unresolved slug
  renders as a red link.
- **Footnotes:** `[^N]` in the body resolves to an `id: N` entry in the
  frontmatter `references` list.

## 6. What the linter gates vs. advises

| Check | Applies to | Gate or advisory |
|---|---|---|
| Frontmatter present | all | gate |
| `foundry-draft-v1` five research-trail fields | drafts | gate |
| `research_provenance` enum valid | drafts | gate |
| `foundry-doc-v1` title / slug / category | published | gate |
| Banned vocabulary | all | gate |
| Body H1 | published only | gate |
| Terminal section order | all | gate |
| Bilingual `.es` pair (non-GUIDE) | published + drafts | gate |
| Sentence length over the ~45-word ceiling | all | advisory |

## Change log

| Version | Date | Change |
|---|---|---|
| 1.0 | 2026-05-21 | Initial schema — consolidates content-contract, naming-convention, and draft-research-trail-discipline. Track D / D4. |
