---
artifact: guide
schema: foundry-draft-v1
title: "Content Operations — media-knowledge-corporate"
slug: guide-content-operations
draft_id: project-editorial-20260616-mk-corp-content-ops
status: staged
created: 2026-06-16
updated: 2026-06-16
owner: project-editorial
destination: woodfine-fleet-deployment/media-knowledge-corporate/guide-content-operations.md
route_to: command
action: create
language_protocol: PROSE-GUIDE
bcsc_class: customer-internal
research_trail:
  origin: project-editorial
  basis: original authorship; operator-facing content lifecycle for corporate.woodfinegroup.com; adapted from documentation instance template with corporate-specific register notes and BCSC disclosure requirements
  data_sources: media-knowledge-corporate content, bcsc-disclosure-posture.md conventions, editorial workflow conventions
  citations_pending: false
  forbidden_terms_cleared: true
---

## Content Operations — corporate.woodfinegroup.com

This guide covers the ongoing content lifecycle for `corporate.woodfinegroup.com` — the corporate documentation wiki covering the Woodfine family of companies, the investment structure, and the economic model. It covers how articles are added, updated, reviewed, and retired; what quality standards apply; and how changes reach the live wiki.

For initial deployment of the wiki service, see `guide-deployment.md`.

---

## The content lifecycle

1. **Draft** — content is authored or revised in the editorial archive
2. **Staged** — the draft is committed to the staging mirror of `media-knowledge-corporate`
3. **Stage 6** — the staged commit is promoted to the canonical repository by the workspace coordinator
4. **Live** — the content service serves the revision

Content authors are responsible for steps 1 and 2. Stage 6 is handled by the workspace coordinator.

---

## Article types

The corporate wiki carries articles about the legal entities in the Woodfine group, the investment structure (investment units, perpetual equity model, distributions), and the properties or operations those entities conduct. These articles are externally visible and subject to continuous disclosure obligations.

**TOPIC articles** describe the corporate structure, economic model, and investment characteristics. They are written at a level accessible to investors and advisors unfamiliar with the platform technology.

**GUIDE articles** are operational runbooks for managing the wiki deployment. They live in `woodfine-fleet-deployment/media-knowledge-corporate/`, not in the wiki content repository.

---

## Article standards

### Lede

The corporate wiki audience is investors, advisors, and financial professionals. The lede identifies the entity or concept, states its significance to the investment structure, and establishes the context.

Correct order:
1. Identify the subject (which entity, which mechanism, which obligation?)
2. State its role in the investment structure
3. Provide the essential quantitative or structural context (if applicable)

Avoid:
- Opening with internal process vocabulary
- Leading with legal form before explaining function
- Conditional or speculative claims presented as current fact

### Register

The register is formal financial: the level of a corporate disclosure document or offering memorandum. Characteristics:
- Precise and literal — no marketing register, no informal abbreviation
- Passive constructions are acceptable where they add precision
- Numbers: spell out below ten, digits from ten upward; use percentage signs with a space before the `%` sign only in formal tabular context; in prose, write "12 percent" not "12%"
- Distribute no opinion — state facts and structure; avoid characterising the investment as attractive, conservative, or otherwise

### BCSC disclosure posture

All content in this wiki is subject to continuous disclosure obligations. The following rules apply without exception:

- **Sovereign Data Foundation** is referred to in planned or intended terms only. It is not described as an existing equity holder, active governance body, or current counterparty.
- **Forward-looking statements** — any claim about future distributions, returns, performance, or capabilities — must use qualified language: "intended," "planned," "targeted," "expected." Never assert outcomes as achieved before they are.
- **Current-fact claims** must be accurate as of the `last_edited` date. If an article contains a factual claim that has changed, the article must be updated before Stage 6 promotion.

If you are uncertain whether a claim is forward-looking, treat it as forward-looking and qualify it.

### Bilingual requirement

Every article has a Spanish-language pair. Both versions are updated in the same commit. The Spanish version is an editorially adapted equivalent — not a verbatim translation.

The BCSC disclosure posture applies equally to the Spanish pair. Both versions must carry consistent qualified language.

### No body H1

Do not write a `# Title` line in the body. The rendering engine emits the `title:` frontmatter as the page `<h1>`.

### Frontmatter required fields

| Field | Example |
|---|---|
| `schema` | `foundry-doc-v1` |
| `title` | `"Investment Units"` |
| `slug` | `topic-investment-units` |
| `category` | the parent directory name |
| `status` | `active` or `stub` |
| `bcsc_class` | `public-disclosure-safe` |
| `last_edited` | `2026-06-16` |

---

## Adding a new article

1. Confirm the subject is appropriate for the corporate wiki: it must relate to the corporate structure, investment model, or entity relationships — not to platform technology or operations.
2. Choose a slug: lowercase kebab-case, globally unique, descriptive.
3. Draft the English article at `<category>/<slug>.md`.
4. Draft the Spanish pair at `<category>/<slug>.es.md`.
5. Apply the BCSC disclosure posture review (see above).
6. Submit through the editorial channel.

---

## Updating an existing article

1. Edit in place — no `_v2` copies.
2. Update `last_edited` to the revision date.
3. Update the Spanish pair in the same revision.
4. Reapply the BCSC disclosure posture review to any changed section.
5. Submit through the editorial channel.

---

## BCSC compliance review checklist

Before submitting any addition or revision to the editorial channel, confirm:

- [ ] No Sovereign Data Foundation language presents it as a current governance body or equity holder
- [ ] All forward-looking claims use qualified language (intended, planned, targeted)
- [ ] All current-fact claims are accurate as of the revision date
- [ ] No claim implies guaranteed returns or outcomes
- [ ] The Spanish pair carries equivalent qualified language

If any item fails, revise before submitting.

---

## Retiring an article

Articles are not deleted. Mark as superseded by adding `aliases:` to the replacement article, or reduce to a stub with a one-sentence explanation.

---

## Requesting a guide update

Operational guides for this wiki instance live in `woodfine-fleet-deployment/media-knowledge-corporate/`. Route guide additions or updates to the workspace coordinator via the editorial inbox with `route_to: command`.

---

## Verifying content is live

After Stage 6 promotion:

1. Visit the article URL at `corporate.woodfinegroup.com`
2. Confirm `last_edited` matches the revision date
3. Confirm wikilinks resolve
4. Confirm the Spanish pair is current
5. For any article with BCSC-sensitive content, verify the live text matches the approved revision

---

*Copyright © 2026 Woodfine Capital Projects Inc. All rights reserved.*
