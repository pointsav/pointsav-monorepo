---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-knowledge
target_repo: pointsav-fleet-deployment
target_path: media-knowledge-documentation/
target_filename: guide-keep-the-home-page-the-gold-standard.md
audience: vendor-internal
bcsc_class: no-disclosure-implication
language_protocol: PROSE-GUIDE
authored: 2026-04-30T01:40:00Z
authored_by: task-project-knowledge (Opus parent synthesis)
authored_with: claude-opus-4-7
references:
  - clones/project-knowledge/.claude/drafts-outbound/research-wikipedia-leapfrog-2030.draft.md
  - clones/project-knowledge/.claude/drafts-outbound/topic-knowledge-wiki-home-page-design.draft.md
  - vendor/pointsav-monorepo/app-mediakit-knowledge/src/server.rs (home_chrome handler)
  - content-wiki-documentation/index.md
  - content-wiki-documentation/featured-topic.yaml
  - content-wiki-documentation/.claude/rules/naming-convention.md
  - external:en.wikipedia.org/wiki/Main_Page
notes_for_editor: |
  This GUIDE goes inside the deployment subfolder per CLAUDE.md §14 (guides
  live with the deployment they operate). English-only per §14 — operational,
  not for public-facing distribution.

  Audience: an operator (PointSav engineering, Master Claude, future Root
  Claude in content-wiki-documentation) who needs to keep the home page the
  gold standard as the corpus grows.

  This GUIDE is about EDITORIAL discipline at the home-page surface, not
  about engine operations (those live in guide-operate-knowledge-wiki and
  guide-deployment).

  Load-bearing sections:

  §2 — The format invariants. Each is a hard rule, not a guideline. The
  format-invariant table is what distinguishes a Wikipedia-class home page
  from a documentation-aesthetic home page.

  §3 — The featured-topic.yaml rotation procedure. Concrete, executable
  steps. Operator should be able to rotate the pin in 5 minutes following
  this section.

  §4 — Anti-patterns. Each anti-pattern is something a well-meaning operator
  might do that would degrade the home page. Each comes with the structural
  reason not to do it.

  §5 — When to ask for ratification. The operator's discretion ends at
  certain decision points (changing the 9-category set, adding a new home
  page slot, ratifying a new featured-pin slug not yet in the corpus).
  Each is named explicitly.
research_done_count: 6
research_suggested_count: 2
open_questions_count: 1
research_provenance: synthesized-from-research-draft + workspace-direct-consultation
research_inline: true
---

# GUIDE: Keep the documentation.pointsav.com home page the gold standard

This GUIDE is for an operator — Master Claude, a future Root Claude in `content-wiki-documentation`, or a manually-acting administrator — who needs to maintain the home page of documentation.pointsav.com as a Wikipedia-class encyclopedic surface as the corpus grows. It covers the format invariants the home page must preserve, the featured-topic rotation procedure, and the anti-patterns that degrade the encyclopedic register.

It is the operational complement to [[topic-knowledge-wiki-home-page-design]] (the public-facing narrative) and `clones/project-knowledge/.claude/drafts-outbound/research-wikipedia-leapfrog-2030.draft.md` (the substrate-internal design research).

## 1. Where the home page lives in the substrate

The home page is rendered by the `app-mediakit-knowledge` crate's `index()` handler at `pointsav-monorepo/app-mediakit-knowledge/src/server.rs:641` (calling `home_chrome()` at line 466). The handler reads three sources at request time:

1. `<content-dir>/index.md` — the home lede (rendered Markdown body)
2. `<content-dir>/featured-topic.yaml` — the featured-pin slug + optional `since:` and `note:` fields
3. The category subdirectories — populated by `bucket_topics_by_category()` from `last_edited:` frontmatter on each article

The handler composes: site header → lede (rendered from `index.md`) → featured-pin panel (when `featured-topic.yaml` resolves a valid slug) → "Browse by category" 3×3 grid (all 9 ratified categories) → "Recent additions" feed (top 5 by `last_edited` desc) → site footer.

The deployment instance at `~/Foundry/deployments/media-knowledge-documentation-1/` runs this binary as a systemd service (`local-knowledge.service`) bound to `127.0.0.1:9090`, fronted by an nginx vhost serving documentation.pointsav.com over HTTPS (Let's Encrypt cert).

## 2. Format invariants — hard rules

| Element | Rule | Source |
|---|---|---|
| Welcome banner | One sentence + structural statistics. No adjectives of self-description. | Wikipedia Main Page; preserved verbatim |
| Featured-pin lead | Body-register paraphrase, not advertising copy; closes with "→ Read" or "(Full article...)" — never "Click here", "Learn more", or button-styled CTAs | Wikipedia TFA blurb format |
| Featured-pin length | Target 200–280 character paraphrase of the article lead | Wikipedia TFA enforces 909–1009; documentation register tightens |
| Category cards | Always render all 9 categories, even when empty. Empty cards show "0 articles — in preparation" — not hidden, not collapsed | Operator-ratified per naming-convention.md §10 Q5-A |
| Category order | architecture / services / systems / applications / governance / infrastructure / company / reference / help. Never alphabetised, never reordered | Operator-ratified |
| Recent additions | Top 5 by `last_edited` desc. Never more, never fewer. Always rendered when corpus has ≥1 article | Wikipedia DYK 9-hook density adapted to documentation scale |
| Footer | License + GitHub source link + ARCHITECTURE.md link. No advertising, no newsletter signup, no social buttons | Wikipedia footer convention |

Anything that violates one of these is a regression. The reader who has internalised the muscle memory will notice; the reader who has not will receive a degraded encyclopedic register.

## 3. Featured-pin rotation procedure

The featured-pin is rotated by editing `featured-topic.yaml` at `content-wiki-documentation/` repo root. This is content-wiki-documentation Root scope (not Task, not Master). A future Root Claude in that repo, or the manually-acting administrator, executes:

1. **Identify the candidate article.** A slug from any of the 9 ratified categories. The candidate must have a substantive `## Lead section` paragraph that paraphrases cleanly (the engine's lead extraction reads the first 200–300 characters of the body after the H1; if the lead is thin or the article opens with a TOC, the rendered featured panel will be visually bad).

2. **Verify the slug resolves.** From `content-wiki-documentation/`, run:
   ```
   find . -name "<candidate-slug>.md" -not -path "*/.*"
   ```
   The file must exist in one of the 9 category subdirectories or at repo root. If the result is empty, the engine's render will defensive-suppress the featured panel (acceptable degradation but the rotation didn't take effect).

3. **Edit `featured-topic.yaml`.** Replace the `slug:` field. Optional: update `since:` to today's date. The `note:` field is engine-ignored but useful for future Root Claudes reading the file.

4. **Commit via the staging-tier helper.** From `content-wiki-documentation/`:
   ```
   git add featured-topic.yaml
   ~/Foundry/bin/commit-as-next.sh "rotate featured pin: <slug> — <one-line rationale>"
   ```
   Author identity alternates between Jennifer and Peter per the workspace commit-toggle pattern.

5. **Master executes Stage-6 promotion** to push the commit to canonical `pointsav/content-wiki-documentation`. The deployed instance polls or is reloaded to pick up the new content; the next visit to documentation.pointsav.com renders the new pin.

Cadence: weekly is the current target. Daily is aspirational but requires sustained editorial labour at a level the cluster does not yet have. Skipping a week is acceptable; rotating mid-week to react to a substantive new article is acceptable.

## 4. Anti-patterns

### 4.1 Adding a "Get started" CTA to the home page

A well-meaning operator may notice that documentation.pointsav.com lacks an obvious onramp for a first-time reader and propose adding a "Get started" panel above the lede. **Do not.** This is the documentation-aesthetic of SaaS product marketing; ssee [[topic-wiki-provider-landscape]] §3(iii) for the structural reason. The reader-to-contributor onramp is the "Other areas" footer block linking to GitHub repositories and the contribution-onboarding articles. That is the encyclopedia-register equivalent and it is sufficient.

### 4.2 Reducing the category grid to "active" categories only

A well-meaning operator may notice that infrastructure/, company/, and help/ have few articles and propose hiding empty categories. **Do not.** The visible "0 articles — in preparation" placeholder is editorially load-bearing — it signals the platform's intended scope at launch and gives contributors a visible target. Hiding empty categories collapses the platform into "what we have" rather than "what we plan to have", which is the wrong scope-signal for both engineering and financial-community audiences.

### 4.3 Adding promotional copy to the welcome banner

A well-meaning operator may propose extending the welcome banner from "PointSav's platform documentation covers..." to "The leading platform documentation for the substrate sovereignty era." **Do not.** Adjectives of self-description destroy encyclopedic register on contact. See research draft §2 welcome-banner format invariant — Wikipedia's banner is one sentence + two statistics, no adjectives, and that absence is load-bearing.

### 4.4 Reordering the category grid by article count

A well-meaning operator may propose ordering the category grid descending by article count, so that the largest categories surface first. **Do not.** The 9-category order is operator-ratified (architecture / services / systems / applications / governance / infrastructure / company / reference / help) and the order encodes a structural argument — architecture is foundational, services build on architecture, systems are the OS layer, applications are the user-facing surface, governance is cross-cutting, infrastructure is operational substrate, company is the BCSC continuous-disclosure surface, reference is dictionary-grade, help is participatory onramp. Reordering by count loses that argument.

### 4.5 Featuring an article from the same category for consecutive weeks

A well-meaning operator may rotate the featured-pin from `compounding-substrate` (architecture) to `apprenticeship-substrate` (architecture) to `language-protocol-substrate` (architecture) over three weeks. The result is a home page that signals "we have one thing to talk about and it's architecture". **Do not.** Diversity across the 9 categories is editorial signal that the corpus has breadth. Acceptable rotation: architecture → governance → services → reference → architecture. Unacceptable: three consecutive picks from any single category.

### 4.6 Removing the "Recent additions" section when the corpus is empty

If the corpus has fewer than 5 articles, the engine's `recent_topics_by_last_edited()` returns however many it has. The handler renders whatever is returned, even if 1 or 2 articles. **Do not** add a special case to suppress the section — the engine's behaviour is correct. The section signals "this is what was added recently"; if there are 2 articles, the section says "2 articles were added recently" and that is accurate.

## 5. When the operator's discretion ends — ratification asks

These changes are not the operator's call. Each requires explicit ratification.

| Change | Whose ratification |
|---|---|
| Modify the 9-category set (add a 10th, remove one, rename one) | Master Claude + operator presence |
| Add a new home-page slot (e.g., "Today's word", "Random article", "Open editorial questions") | Master Claude + operator presence |
| Change the featured-pin format (e.g., add an image, change the closer text) | Master Claude (substrate scope) |
| Modify the welcome banner to include new statistics or framing | Master Claude (BCSC scope; statistics that are forward-looking trigger ni-51-102 considerations) |
| Switch the home page to dark-mode-default | project-design ratifies the substrate convention; Master ratifies the deployment-instance default |
| Change the recent-additions ordering (recency vs ratification gate) | Master Claude (architectural decision; affects the engine handler) |

## 6. What does not require ratification

The operator can:

- Rotate the featured-pin to any existing article slug at any cadence (per §3 above)
- Rewrite the lede text in `index.md` for editorial improvement (within the format invariants of §2)
- Update the `since:` and `note:` fields in `featured-topic.yaml`
- Add or improve articles in any category — the home page picks up the count and the recent-additions feed automatically
- Edit the footer text for accuracy (license year, GitHub repo paths if reorganised) within the format invariants

These are routine operational changes. The operator commits, Master Stage-6 promotes, the deployment picks up.

## 7. Open editorial item

The recent-additions section ranks by `last_edited`. A future iteration may switch to `content_reviewed_on` (a separate frontmatter field denoting last editorial review). The substrate-design tradeoff is named in the research draft §6.3 and the engine integration is tracked in the project-knowledge cluster's NEXT.md. Until that switch, recent-additions ranks by recency. Operator should be aware that a cosmetic-only edit will surface to recent-additions; this is acceptable degradation pending the engine update.

## Provenance

Authored 2026-04-30 alongside the home-page-design TOPIC and the design-research draft. The format invariants in §2 are derived from the Wikipedia Main Page primitive audit (research draft §2). The anti-patterns in §4 are operationalised from the cross-cutting failure modes in the wiki-provider-landscape audit (research draft §4). The ratification table in §5 reflects the workspace action matrix per CLAUDE.md §11 — Master scope, Root scope, Task scope are differentiated.

This GUIDE is operational and English-only per workspace CLAUDE.md §14 (operational guides do not bilingualise; they live inside the deployment subfolder).
