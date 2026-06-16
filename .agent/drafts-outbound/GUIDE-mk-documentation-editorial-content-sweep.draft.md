---
artifact: guide
schema: foundry-draft-v1
title: "Monthly Editorial Content Sweep — Documentation Wiki"
slug: guide-editorial-content-sweep
draft_id: project-editorial-20260616-mk-doc-editorial-sweep
status: staged
created: 2026-06-16
updated: 2026-06-16
owner: project-editorial
destination: woodfine-fleet-deployment/media-knowledge-documentation/guide-editorial-content-sweep.md
route_to: command
action: replace
language_protocol: PROSE-GUIDE
bcsc_class: customer-internal
research_trail:
  origin: project-editorial
  basis: full rewrite — removes EGREGIOUS internal vocabulary (DataGraph, Foundry wikis, inference-pipeline jargon, stale internal paths); replaces with operator-facing content
  data_sources: content-contract.md, existing WFD guide structure
  citations_pending: false
  forbidden_terms_cleared: true
---

## Monthly Editorial Content Sweep

This guide covers the monthly quality review of `documentation.pointsav.com`, the platform engineering documentation wiki. The sweep identifies articles needing updates, drafts revisions to current quality standards, and routes approved changes through the editorial commit channel.

The sweep runs in five sequential steps. Do not reorder them.

---

## Overview

Each monthly sweep has two goals:

1. **Freshness** — every article reflects the current state of the platform, not a snapshot from six months ago.
2. **Register** — every article reads at the level expected by its audience. For the documentation wiki, that audience is engineers, architects, and technically literate readers from outside the organisation.

Articles that are stale or written in the wrong register are lower priority than articles that are incorrect. Fix factual errors first; style comes after.

---

## Step 1 — Build the review roster

Visit `documentation.pointsav.com` and compile a list of articles to review this month.

**Sources for the roster:**

- Articles with `last_edited` more than 90 days ago
- Articles linked from the wiki home page (`/`) — highest traffic, highest visibility
- Articles for any service, application, or infrastructure component that received a deployment or significant code change since the last sweep
- Articles where readers have reported errors (surfaced via editorial inbox)

**Priority order (highest first):**

| Priority | Target | Reason |
|---|---|---|
| 1 | Home page article block | Highest external visibility |
| 2 | Architecture articles | Core value proposition; engineer first-read |
| 3 | Services articles | Operational; most likely to go stale |
| 4 | Reference and glossary | Slowest to decay; lowest urgency |

Aim for 10–20 articles per month. A sweep wider than 20 articles loses depth; defer the remainder to the next cycle.

---

## Step 2 — Quality check each article

For each article on the roster, apply the five-point quality check:

**2.1 Lede**

Read the opening paragraph. A correct lede places the reader in context immediately — it identifies the thing, states its function, and tells the reader why it matters. A failing lede does one of the following:

- Opens with "This article describes..." (throat-clearing — rewrite)
- Opens with a score, metric, or rank before the reader knows what the subject is (inverted order — rewrite)
- Opens with internal process vocabulary that a reader outside the organisation would not recognise (vocabulary leak — rewrite)

**2.2 Register**

The documentation wiki register is technical but accessible: engineers can follow the prose; a technically literate reader from outside the organisation can grasp the intent. Red flags:

- Sentences that require knowledge of internal acronyms or project names to parse
- Tables listing configuration values without explaining what they configure or why
- Bullet lists where prose would be clearer

**2.3 Accuracy**

Check any claim that could have changed since `last_edited`:

- Service ports, hostnames, and URL routes
- Feature flags or capability status (active / in development / planned)
- Version numbers or build identifiers
- Team or ownership attributions

**2.4 Bilingual pair**

Every article in `documentation.pointsav.com` has a Spanish-language pair (`.es.md`). Verify the Spanish version exists and carries the same `last_edited` date. If the Spanish version is more than one sweep behind the English version, flag it for translation update.

**2.5 References**

Articles that cite external specifications, standards, or public datasets should carry a `references:` block in their frontmatter. Verify that any reference URL resolves and that the cited version is still current.

---

## Step 3 — Identify gaps

A gap is a topic that belongs in the documentation wiki but has no article. Common gap sources:

- A new service or application deployed since the last sweep with no corresponding wiki article
- A significant architectural decision (ADR) without a companion explanation article
- A concept referenced repeatedly by wikilinks from other articles but whose target is a red link or stub

Log gaps in a session note. If a gap can be filled with a short stub article (one paragraph + planned status), draft it. If a gap requires research or a detailed technical write-up, open a request to the editorial queue.

---

## Step 4 — Draft and submit revisions

Draft revisions for each article that failed the Step 2 check.

**Revision discipline:**

- Correct factual errors first, register second, style last
- One article per revision draft; do not bundle unrelated article edits
- Retain existing article structure; do not add new sections unless the content clearly requires it
- Update `last_edited` to today's date

**Submission:**

Submit completed revision drafts to the editorial channel. Drafts are committed by the designated committer identity via the staging-tier commit flow and promoted to the canonical repository at the next Stage 6 window. Do not attempt to commit directly to the live wiki repository.

---

## Step 5 — Verify the updates

After a revision draft clears the editorial channel and is committed to the canonical repository, verify the updated article at `documentation.pointsav.com`.

**Verification checklist:**

- [ ] Article URL resolves without error
- [ ] Updated `last_edited` date appears in article metadata
- [ ] Wikilinks within the article resolve (no red links introduced by the revision)
- [ ] Spanish pair reflects the revision (if the revision changed substantive content)
- [ ] Any cross-linked articles that referred to the changed content still make sense in context

---

## What not to do

- Do not rewrite articles that are factually correct in order to change their style. Style improves incrementally; accuracy is urgent.
- Do not add technical implementation details (configuration values, internal paths, credential locations) to public-facing articles. Those belong in operational guides, not wiki articles.
- Do not create new articles for topics already covered under a different slug. Search the wiki before drafting a new article.
- Do not update `last_edited` without making a substantive change. Freshness dates signal article currency to readers; inflating them degrades that signal.

---

*Copyright © 2026 Woodfine Capital Projects Inc. All rights reserved.*
