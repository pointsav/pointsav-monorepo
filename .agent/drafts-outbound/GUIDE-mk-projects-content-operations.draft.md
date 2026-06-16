---
artifact: guide
schema: foundry-draft-v1
title: "Content Operations — media-knowledge-projects"
slug: guide-content-operations
draft_id: project-editorial-20260616-mk-proj-content-ops
status: staged
created: 2026-06-16
updated: 2026-06-16
owner: project-editorial
destination: woodfine-fleet-deployment/media-knowledge-projects/guide-content-operations.md
route_to: command
action: create
language_protocol: PROSE-GUIDE
bcsc_class: customer-internal
research_trail:
  origin: project-editorial
  basis: original authorship; operator-facing content lifecycle for projects.woodfinegroup.com; adapted from documentation instance template with projects-specific register notes
  data_sources: media-knowledge-projects content, editorial workflow conventions
  citations_pending: false
  forbidden_terms_cleared: true
---

## Content Operations — projects.woodfinegroup.com

This guide covers the ongoing content lifecycle for `projects.woodfinegroup.com` — the projects documentation wiki carrying location intelligence research, regional market analysis, and co-location archetype documentation for Woodfine operations. It covers how articles are added, updated, reviewed, and retired; what quality standards apply; and how changes reach the live wiki.

For initial deployment of the wiki service, see `guide-deployment.md`.

---

## The content lifecycle

1. **Draft** — content is authored or revised in the editorial archive
2. **Staged** — the draft is committed to the staging mirror of `media-knowledge-projects`
3. **Stage 6** — the staged commit is promoted to the canonical repository by the workspace coordinator
4. **Live** — the content service serves the revision

Content authors are responsible for steps 1 and 2. Stage 6 is handled by the workspace coordinator.

---

## Article types

The projects wiki carries two article types:

**TOPIC articles** cover research findings, methodologies, regional markets, co-location archetypes, and data systems. They report on what the data shows and what the methodology concludes. They do not contain internal system configuration or operational procedures.

**GUIDE articles** are operational runbooks for managing the projects wiki deployment. They live in `woodfine-fleet-deployment/media-knowledge-projects/`, not in the wiki content repository.

---

## Article standards

### Lede — place before claim

The projects wiki audience is institutional: commercial real estate investors, asset managers, development analysts, and corporate decision-makers. The lede must orient this audience before stating the finding.

Correct order:
1. Identify the subject (what market, archetype, or system is this article about?)
2. State the most significant finding or characteristic
3. Establish the context that makes the finding meaningful (dataset source, methodology basis, geographic scope)

Avoid:
- Leading with methodology or score before naming the subject
- "This article describes..." — throat-clearing
- Unexplained acronyms (T1, DBSCAN, LODES) in the first paragraph

### Register

The register for `projects.woodfinegroup.com` is Bloomberg/Financial Times: precise, institutional, neutral. Characteristics:
- Short declarative sentences (target 15–20 words)
- No informal register, no casual abbreviation
- Numbers are expressed with appropriate precision — do not round a 25.5 score to "about 26"
- Claims are anchored to a dataset or methodology, not asserted without basis
- Forward-looking claims use "intended," "planned," or "targeted" language; never assert outcomes as fact

### Bilingual requirement

Every article has a Spanish-language pair. Both versions are updated in the same commit. Spanish articles are editorially adapted, not translated verbatim.

### No body H1

The rendering engine emits the `title:` frontmatter as the page `<h1>`. Do not write a `# Title` line in the body.

### Frontmatter required fields

| Field | Example |
|---|---|
| `schema` | `foundry-doc-v1` |
| `title` | `"Plano, Texas — Regional Market"` |
| `slug` | `topic-rm-plano-tx` |
| `category` | the parent directory name |
| `status` | `active` or `stub` |
| `bcsc_class` | `public-disclosure-safe` |
| `last_edited` | `2026-06-16` |

---

## Adding a new article

1. Determine the correct category (see the wiki's category taxonomy).
2. Choose a slug: lowercase kebab-case, globally unique, descriptive.
3. Draft the English article at `<category>/<slug>.md`.
4. Draft the Spanish pair at `<category>/<slug>.es.md`.
5. Verify wikilinks inside the article resolve.
6. Submit through the editorial channel.

---

## Updating an existing article

1. Edit the article in place — no `_v2` copies.
2. Update `last_edited` to the revision date.
3. Update the Spanish pair in the same revision.
4. Submit through the editorial channel.

**Data currency:** articles that reference dataset outputs (cluster counts, tier distributions, score values) should reference the dataset phase in the text (e.g., "as of Phase 23+Change B"). When a new dataset phase is produced, update all articles whose cited values differ from the current output.

---

## Retiring an article

Articles are not deleted. Mark as superseded by adding `aliases:` to the replacement article, or reduce to a stub with a one-sentence explanation.

---

## Requesting a guide update

Operational guides for this wiki instance live in `woodfine-fleet-deployment/media-knowledge-projects/`. Route guide additions or updates to the workspace coordinator via the editorial inbox with `route_to: command`.

---

## Verifying content is live

After Stage 6 promotion:

1. Visit the article URL at `projects.woodfinegroup.com`
2. Confirm `last_edited` matches the revision date
3. Confirm wikilinks resolve
4. Confirm the Spanish pair is updated

---

*Copyright © 2026 Woodfine Capital Projects Inc. All rights reserved.*
