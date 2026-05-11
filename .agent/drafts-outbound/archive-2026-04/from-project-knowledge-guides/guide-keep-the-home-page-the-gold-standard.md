---
schema: foundry-guide-v1
title: "Keep the documentation.pointsav.com home page the gold standard"
status: refined
target_repo: pointsav-fleet-deployment
target_path: media-knowledge-documentation/guide-keep-the-home-page-the-gold-standard.md
audience: vendor-internal
bcsc_class: no-disclosure-implication
language: en
last_edited: 2026-04-30
originated_from: project-knowledge cluster, Opus parent synthesis, 2026-04-30
refined_by: project-language, 2026-04-30
research_done_count: 6
research_suggested_count: 2
open_questions_count: 1
research_provenance: synthesized-from-research-draft + workspace-direct-consultation
research_inline: false
---

This guide is for an operator — Master Claude, a future Root Claude in `content-wiki-documentation`, or a manually-acting administrator — who needs to maintain the home page of `documentation.pointsav.com` as a Wikipedia-class encyclopedic surface as the corpus grows. It covers the format invariants the home page must preserve, the featured-topic rotation procedure, and the anti-patterns that degrade the encyclopedic register.

It is the operational complement to [[topic-knowledge-wiki-home-page-design]] (the public-facing narrative) and `clones/project-knowledge/.claude/drafts-outbound/research-wikipedia-leapfrog-2030.draft.md` (the substrate-internal design research).

## Where the home page lives in the substrate

The home page is rendered by the `app-mediakit-knowledge` crate's `index()` handler at `pointsav-monorepo/app-mediakit-knowledge/src/server.rs:641` (calling `home_chrome()` at line 466). The handler reads three sources at request time:

1. `<content-dir>/index.md` — the home lede (rendered Markdown body)
2. `<content-dir>/featured-topic.yaml` — the featured-pin slug + optional `since:` and `note:` fields
3. The category subdirectories — populated by `bucket_topics_by_category()` from `last_edited:` frontmatter on each article

The handler composes: site header → lede (rendered from `index.md`) → featured-pin panel (when `featured-topic.yaml` resolves a valid slug) → "Browse by category" 3×3 grid (all 9 ratified categories) → "Recent additions" feed (top 5 by `last_edited` desc) → site footer.

The deployment instance at `~/Foundry/deployments/media-knowledge-documentation-1/` runs this binary as a systemd service (`local-knowledge.service`) bound to `127.0.0.1:9090`, fronted by an nginx vhost serving `documentation.pointsav.com` over HTTPS.

## Format invariants — hard rules

| Element | Rule | Source |
|---|---|---|
| Welcome banner | One sentence + structural statistics. No adjectives of self-description. | Wikipedia Main Page; preserved verbatim |
| Featured-pin lead | Body-register paraphrase, not advertising copy; closes with "→ Read" or "(Full article...)" — never "Click here", "Learn more", or button-styled CTAs | Wikipedia TFA blurb format |
| Featured-pin length | Target 200–280 character paraphrase of the article lead | Wikipedia TFA enforces 909–1009; documentation register tightens |
| Category cards | Always render all 9 categories, even when empty. Empty cards show "0 articles — in preparation" — not hidden, not collapsed | Operator-ratified per naming-convention.md §10 Q5-A |
| Category order | architecture / services / systems / applications / governance / infrastructure / company / reference / help. Never alphabetised, never reordered | Operator-ratified |
| Recent additions | Top 5 by `last_edited` desc. Never more, never fewer. Always rendered when corpus has ≥1 article | Wikipedia DYK 9-hook density adapted to documentation scale |
| Footer | License + GitHub source link + ARCHITECTURE.md link. No advertising, no newsletter signup, no social buttons | Wikipedia footer convention |

Any violation is a regression. The reader who has internalised the muscle memory will notice; the reader who has not will receive a degraded encyclopedic register.

## Featured-pin rotation procedure

The featured-pin is rotated by editing `featured-topic.yaml` at `content-wiki-documentation/` repo root. This is content-wiki-documentation Root scope (not Task, not Master). A future Root Claude in that repo, or the manually-acting administrator, executes:

1. **Identify the candidate article.** A slug from any of the 9 ratified categories. The candidate must have a substantive lead paragraph that paraphrases cleanly (the engine's lead extraction reads the first 200–300 characters of the body after the H1; if the lead is thin or the article opens with a TOC, the rendered featured panel will be visually degraded).

2. **Verify the slug resolves.** From `content-wiki-documentation/`, run:
   ```
   find . -name "<candidate-slug>.md" -not -path "*/.*"
   ```
   The file must exist in one of the 9 category subdirectories or at repo root. If the result is empty, the engine's render will defensively suppress the featured panel.

3. **Edit `featured-topic.yaml`.** Replace the `slug:` field. Optionally update `since:` to today's date. The `note:` field is engine-ignored but useful for future Root Claudes reading the file.

4. **Commit via the staging-tier helper.** From `content-wiki-documentation/`:
   ```
   git add featured-topic.yaml
   ~/Foundry/bin/commit-as-next.sh "rotate featured pin: <slug> — <one-line rationale>"
   ```
   Author identity alternates between Jennifer and Peter per the workspace commit-toggle pattern.

5. **Master executes Stage-6 promotion** to push the commit to canonical `pointsav/content-wiki-documentation`. The deployed instance polls or is reloaded to pick up the new content; the next visit to `documentation.pointsav.com` renders the new pin.

Cadence: weekly is the current target. Daily is aspirational but requires sustained editorial labour at a level the cluster does not yet have. Skipping a week is acceptable; rotating mid-week to react to a substantive new article is acceptable.

## Anti-patterns

### Adding a "Get started" CTA to the home page

A well-meaning operator may notice that `documentation.pointsav.com` lacks an obvious onramp for a first-time reader and propose adding a "Get started" panel above the lede. Do not. This is the documentation aesthetic of SaaS product marketing. The reader-to-contributor onramp is the "Other areas" footer block linking to GitHub repositories and the contribution-onboarding articles. That is the encyclopedia-register equivalent and it is sufficient.

### Reducing the category grid to "active" categories only

A well-meaning operator may notice that `infrastructure/`, `company/`, and `help/` have few articles and propose hiding empty categories. Do not. The visible "0 articles — in preparation" placeholder is editorially load-bearing — it signals the platform's intended scope at launch and gives contributors a visible target. Hiding empty categories collapses the platform into "what we have" rather than "what we plan to have", which is the wrong scope-signal for both engineering and financial-community audiences.

### Adding promotional copy to the welcome banner

A well-meaning operator may propose extending the welcome banner from "PointSav's platform documentation covers..." to "The leading platform documentation for the substrate sovereignty era." Do not. Adjectives of self-description destroy encyclopedic register on contact. Wikipedia's banner is one sentence + two statistics, no adjectives, and that absence is load-bearing.

### Reordering the category grid by article count

A well-meaning operator may propose ordering the category grid descending by article count, so that the largest categories surface first. Do not. The 9-category order is operator-ratified and the order encodes a structural argument — architecture is foundational, services build on architecture, systems are the OS layer, applications are the user-facing surface, governance is cross-cutting, infrastructure is operational substrate, company is the BCSC continuous-disclosure surface, reference is dictionary-grade, help is participatory onramp. Reordering by count loses that argument.

### Featuring an article from the same category for consecutive weeks

A well-meaning operator may rotate the featured-pin from `compounding-substrate` (architecture) to `apprenticeship-substrate` (architecture) to `language-protocol-substrate` (architecture) over three weeks. The result is a home page that signals "we have one thing to talk about and it's architecture". Do not. Diversity across the 9 categories is editorial signal that the corpus has breadth. Acceptable rotation: architecture → governance → services → reference → architecture. Three consecutive picks from any single category is unacceptable.

### Removing the "Recent additions" section when the corpus is empty

If the corpus has fewer than 5 articles, the engine's `recent_topics_by_last_edited()` returns however many it has. The handler renders whatever is returned, even if 1 or 2 articles. Do not add a special case to suppress the section — the engine's behaviour is correct. The section signals "this is what was added recently"; if there are 2 articles, the section says "2 articles were added recently" and that is accurate.

## When the operator's discretion ends — ratification asks

These changes are not the operator's call. Each requires explicit ratification.

| Change | Whose ratification |
|---|---|
| Modify the 9-category set (add a 10th, remove one, rename one) | Master Claude + operator presence |
| Add a new home-page slot (e.g., "Today's word", "Random article", "Open editorial questions") | Master Claude + operator presence |
| Change the featured-pin format (e.g., add an image, change the closer text) | Master Claude (substrate scope) |
| Modify the welcome banner to include new statistics or framing | Master Claude (BCSC scope; statistics that are forward-looking trigger ni-51-102 considerations) |
| Switch the home page to dark-mode-default | project-design ratifies the substrate convention; Master ratifies the deployment-instance default |
| Change the recent-additions ordering (recency vs ratification gate) | Master Claude (architectural decision; affects the engine handler) |

## What does not require ratification

The operator can:

- Rotate the featured-pin to any existing article slug at any cadence (per the procedure above)
- Rewrite the lede text in `index.md` for editorial improvement (within the format invariants)
- Update the `since:` and `note:` fields in `featured-topic.yaml`
- Add or improve articles in any category — the home page picks up the count and the recent-additions feed automatically
- Edit the footer text for accuracy (license year, GitHub repo paths if reorganised) within the format invariants

These are routine operational changes. The operator commits, Master Stage-6 promotes, the deployment picks up.

## Open editorial item

The recent-additions section ranks by `last_edited`. A future iteration may switch to `content_reviewed_on` (a separate frontmatter field denoting last editorial review). The substrate-design trade-off is named in the research draft §6.3 and the engine integration is tracked in the project-knowledge cluster's NEXT.md. Until that switch, recent-additions ranks by recency. A cosmetic-only edit will surface to recent-additions; this is acceptable degradation pending the engine update.

## Provenance

Authored 2026-04-30 alongside the home-page-design TOPIC and the design-research draft. The format invariants derive from the Wikipedia Main Page primitive audit (research draft §2). The anti-patterns are operationalised from the cross-cutting failure modes in the wiki-provider-landscape audit (research draft §4). The ratification table reflects the workspace action matrix per `CLAUDE.md` §11 — Master scope, Root scope, Task scope are differentiated.
