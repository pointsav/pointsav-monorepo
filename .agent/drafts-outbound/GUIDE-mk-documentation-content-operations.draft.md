---
artifact: guide
schema: foundry-draft-v1
title: "Content Operations — media-knowledge-documentation"
slug: guide-content-operations
draft_id: project-editorial-20260616-mk-doc-content-ops
status: staged
created: 2026-06-16
updated: 2026-06-16
owner: project-editorial
destination: woodfine-fleet-deployment/media-knowledge-documentation/guide-content-operations.md
route_to: command
action: create
language_protocol: PROSE-GUIDE
bcsc_class: customer-internal
research_trail:
  origin: project-editorial
  basis: original authorship; operator-facing content lifecycle for documentation.pointsav.com; no AI pipeline references
  data_sources: content-contract.md, naming-convention.md, editorial workflow conventions
  citations_pending: false
  forbidden_terms_cleared: true
---

## Content Operations — documentation.pointsav.com

This guide covers the ongoing content lifecycle for `documentation.pointsav.com` — the engineering documentation wiki for the PointSav platform. It covers how articles are added, updated, reviewed, and retired; what quality standards apply; and how changes reach the live wiki.

For initial deployment of the wiki service, see `guide-deployment.md`.
For monthly quality review across all wiki articles, see `guide-editorial-content-sweep.md`.

---

## The content lifecycle

An article follows this path from draft to live:

1. **Draft** — content is authored or revised, typically in the editorial archive
2. **Staged** — the draft is committed to the staging mirror of `media-knowledge-documentation`
3. **Stage 6** — the staged commit is promoted to the canonical repository by the Command Session
4. **Live** — the content service detects the updated content and serves the revision

Steps 3 and 4 are handled by the workspace coordinator, not the content author. A content author's responsibility ends at step 2: produce a correct, formatted draft and submit it through the editorial channel.

---

## Article types

The documentation wiki carries two article types:

**TOPIC articles** cover architectural concepts, named services, applications, operating systems, governance decisions, and reference material. They explain what something is and how it relates to other platform components. They do not contain configuration values, internal paths, or operational procedures.

**GUIDE articles** are operational runbooks. They cover how to perform a specific task: deploying a service, diagnosing a failure, provisioning a node. Guides live in `woodfine-fleet-deployment/media-knowledge-documentation/`, not in the wiki content repository. To add or update a guide, see the editorial routing procedure below.

If a topic "is what something is" and a guide is "how to operate it," and your draft does both, split it into two documents.

---

## Article standards

### Lede

The opening paragraph of every article places the reader immediately. The correct order is:

1. Identify the subject (what is it?)
2. State its function or role (what does it do?)
3. Establish why it matters to the reader (so what?)

Avoid:
- "This article describes..." — throat-clearing, rewrite
- Leading with a metric or identifier before the subject is named — inverted order, rewrite
- Technical acronyms in the first sentence without prior definition

### Register

The documentation wiki audience is engineers, architects, and technically literate readers from outside the organisation. The register is technical but accessible:

- Engineers can follow the prose without consulting a glossary
- A technically literate reader unfamiliar with the platform can grasp the intent of any section
- Sentences are complete and precise; no informal register, no casual abbreviation
- Paragraphs are short: two to four sentences each

### Bilingual requirement

Every article in the documentation wiki has a Spanish-language pair. The English article at `architecture/os-totebox.md` has a paired `architecture/os-totebox.es.md`. When authoring or updating an English article, the Spanish pair must be updated in the same commit. An English article without a current Spanish pair is a defect.

Spanish articles follow the same register and structural rules as English articles. They are not machine-translated; they are editorially adapted Spanish equivalents.

### No body H1

Article frontmatter carries a `title:` field. The rendering engine emits the title as the page `<h1>`. Do not write a `# Title` line in the article body — the result is a duplicate H1.

### Frontmatter required fields

Every article must carry:

| Field | Example |
|---|---|
| `schema` | `foundry-doc-v1` |
| `title` | `"OS Console Architecture"` |
| `slug` | `os-console-architecture` (must equal the filename stem) |
| `category` | `architecture` (must equal the parent directory name) |
| `status` | `active` or `stub` |
| `bcsc_class` | `public-disclosure-safe` for public wiki content |
| `last_edited` | `2026-06-16` |

---

## Adding a new article

1. Determine the correct category for the article (see `naming-convention.md §4` for the category taxonomy).
2. Choose a slug following the slug rules: lowercase kebab-case, entity-type prefix if applicable (`service-*`, `os-*`, `app-*`), globally unique.
3. Draft the English article at `<category>/<slug>.md`.
4. Draft the Spanish pair at `<category>/<slug>.es.md`.
5. Verify wikilinks inside the article: every `[[slug]]` must resolve to an existing article or a stub. Red links in committed articles are defects.
6. Submit both files through the editorial channel.

---

## Updating an existing article

1. Edit the article file in place — no `_v2` copies.
2. Update `last_edited` to the revision date.
3. Update the Spanish pair in the same revision.
4. If the revision introduces new wikilinks, verify they resolve.
5. Submit through the editorial channel.

---

## Retiring an article

Articles are not deleted; they are marked as superseded or converted to stubs.

- If the article's topic is covered by a new article: add `aliases: [old-slug]` to the new article's frontmatter. The router redirects `old-slug` to the new article.
- If the article's topic is no longer relevant: change `status: active` to `status: stub` and reduce the body to a one-sentence note explaining the change.
- Published slugs are never removed — inbound wikilinks from other articles would break.

---

## Requesting a guide update

Operational guides for `documentation.pointsav.com` live in `woodfine-fleet-deployment/media-knowledge-documentation/`. They follow a different path than wiki articles: they are placed by the workspace coordinator using the admin-tier commit flow. To request a guide addition or update:

1. Draft the guide or describe the required change in the editorial inbox.
2. Mark it as `route_to: command`.
3. The workspace coordinator handles placement in `woodfine-fleet-deployment/media-knowledge-documentation/`.

Do not stage guide drafts in the wiki content repository.

---

## Verifying content is live

After a revision is promoted through Stage 6:

1. Visit the article URL: `https://documentation.pointsav.com/<category>/<slug>`
2. Confirm the `last_edited` date shown in article metadata matches the revision date
3. Confirm any new wikilinks resolve (click through to verify, or check for red links)
4. Confirm the Spanish pair is also updated

The wiki service rebuilds its search index on content change detection. Allow up to two minutes after promotion for search to reflect the revision.

---

*Copyright © 2026 Woodfine Capital Projects Inc. All rights reserved.*
