---
schema: foundry-draft-v1
state: draft-refined
originating_cluster: project-language
target_repo: content-wiki-documentation
target_path: index.md
target_filename: index.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
category: root
authored: 2026-04-29T01:30:00Z
authored_by: sonnet-4-6 sub-agent (refinement pass; parent task-project-language session 12376c0e4bc33ea7; operator: ps-administrator)
authored_with: claude-sonnet-4-6
references:
  - naming-convention.md §4 (nine-category set, operator-ratified 2026-04-28T22:00Z)
  - naming-convention.md §7 (MOC landing pattern)
  - naming-convention.md §9 (investor-audience design)
  - conventions/cluster-wiki-draft-pipeline.md
  - conventions/bcsc-disclosure-posture.md
  - content-contract.md §1 (index.md as wiki home)
  - content-contract.md §2 (directory layout)
  - content-contract.md §4 (category: root for index.md)
  - content-contract.md §7 (URL table — / served by index.md)
  - content-contract.md §9 (migration state — no index.md yet at root)
  - clones/project-knowledge/.claude/outbox.md (Q1+Q2 answers relayed via Master)
  - external:en.wikipedia.org/wiki/Main_Page
notes_for_editor: |
  Iteration 2 (refinement pass) — Q1 and Q2 are CLOSED.

  Q1 closure: index.md wins as the canonical home filename per
  content-contract.md §1/§2/§4/§7/§9. category: root is required.
  target_path and target_filename updated accordingly. The cluster-
  outbound filenames (TOPIC-HOME.draft.md / .es.draft.md) are retained
  in cluster-outbound; rename to index.md / index.es.md happens at
  gateway-commit time.

  Q2 closure: featured-topic.yaml at repo root; schema: slug: (required),
  since: (optional YYYY-MM-DD), note: (optional one-liner engine ignores).
  ENGINE comment updated to remove the hedge "(or equivalent pin mechanism
  agreed with project-knowledge)" — the pin format is now confirmed canonical.

  Refinement disciplines applied:
  - Bloomberg-grade tightening throughout; lead paragraph rewritten
  - Banned-vocab sweep: no violations found in iteration-1; confirmed clean
  - BCSC posture: customer-wiki placeholder and stub-category note carry
    planned/intended framing; verified
  - CC BY 4.0 inline URL retained (no cc-by-4-0 registry entry; creative-commons
    entry in citations.yaml points to creativecommons.org root, not the specific
    license URL — inline link is operationally correct for a license declaration)
  - LOOSE markers removed; loose content rewritten or removed
  - ENGINE comments preserved verbatim except Q2 hedge removed
  - Research trail converted to Provenance footer per claim #39 §2.3
  - Open questions Q1+Q2 moved to CLOSED in Provenance footer

  State moved to draft-refined. Ready for gateway-commit handoff to
  content-wiki-documentation Root Claude, who commits as index.md at
  repo root.
research_done_count: 9
research_suggested_count: 1
open_questions_count: 0
research_provenance: mixed
research_inline: true
---

# documentation.pointsav.com — Home

<!-- ENGINE: render this file at the URL `/` — it is the wiki home,
     served as index.md per content-contract.md §1/§2/§7. -->

PointSav's platform documentation covers the architecture, services,
operating systems, and governance conventions of the PointSav substrate.
Articles are written for engineers, writers, designers, and readers with
a financial interest in the platform. All content is published under
[CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).

<!-- ENGINE: insert live TOPIC count: "N articles across 9 categories,
     last updated YYYY-MM-DD." Derive N from count of *.md files in
     category directories (excluding _index.md). Derive date from
     max(last_edited:) across all articles. -->

---

## Platform areas

<!-- ENGINE: render this section as a card grid — 3 columns × 3 rows.
     Each card: category name as heading, description paragraph, link
     to category landing (_index.md), and TOPIC count for that category.
     Category TOPIC counts derive from file count in each directory.
     If a category directory has 0 articles (infrastructure/, company/,
     help/ at launch), still render the card with count "0 articles —
     in preparation." -->

**Architecture**
Design principles, substrate patterns, and cross-cutting invariants that
govern how the platform is built. Covers the three-ring stack, compounding
substrate, AI routing, cryptographic audit, and the editorial pipeline.
→ [Browse architecture](/architecture/)

**Services**
Autonomous services implementing Ring 1 and Ring 2 of the three-ring
architecture: ingest, processing, search, egress, and the linguistic
air-lock.
→ [Browse services](/services/)

**Systems**
The operating systems at the foundation layer: ToteboxOS and its
seL4-based capability model, orchestration, and tenant isolation.
→ [Browse systems](/systems/)

**Applications**
User-facing and internal applications built on the platform substrate,
including the wiki engine that serves this site.
→ [Browse applications](/applications/)

**Governance**
Architecture decision records, licensing posture, contributor model,
and compliance conventions.
→ [Browse governance](/governance/)

**Infrastructure**
Fleet deployment, cloud topology, and operational runtime.
In preparation — articles are planned for this category.
→ [Browse infrastructure](/infrastructure/)

**Company** <!-- BCSC: no forward-looking statements in the category
description; company/ articles carry their own FLI labels per Rule 1 -->
Corporate entities, organisational structure, and public disclosures.
→ [Browse company](/company/)

**Reference**
Glossary, nomenclature matrix, and style guides for contributors across
all audiences.
→ [Browse reference](/reference/)

**Help**
Onboarding guides for engineers, writers, and designers contributing to
the platform and its documentation.
→ [Browse help](/help/)

---

## Featured article

<!-- ENGINE: read the file `featured-topic.yaml` from the repo root.
     Parse the pinned slug. Fetch that article's title and first
     paragraph. Render as a framed panel with the title, lead
     paragraph, and a "Read more →" link. If the pin file is absent,
     suppress this section entirely — do not render an empty frame.
     featured-topic.yaml schema: slug: (required), since: (optional
     YYYY-MM-DD), note: (optional one-liner; engine ignores). -->

[[compounding-substrate]] — The Compounding Substrate describes how
PointSav's platform is designed so that each capability a customer installs
makes the next capability easier to operate. The substrate pattern is the
design philosophy behind the three-ring architecture and the leapfrog-2030
trajectory.

[Read more →](/architecture/compounding-substrate)

---

## Recent additions

<!-- ENGINE: sort all article files by last_edited: frontmatter date
     descending (fall back to git commit date if last_edited: is
     absent). Render the top 5 as an unordered list:
     "- [Title](/category/slug) — YYYY-MM-DD"
     If fewer than 5 articles exist, render however many there are.
     Do not render this section if the article count is 0.
     Per Q5.A operator ratification: dated-announcement TOPICs
     (filename pattern `topic-*-YYYY-MM-DD.md`) route here rather than
     to permanent category articles. -->

*Most recently added or updated articles:*

<!-- ENGINE: dynamic list — 5 items, last_edited desc -->

---

## Other areas

Related resources outside this wiki:

- **[PointSav on GitHub](https://github.com/pointsav)** — canonical engineering source; `pointsav/*` organisations host the vendor-tier repositories.
- **[Woodfine Management Corp. on GitHub](https://github.com/woodfine)** — customer-tier mirror; `woodfine/*` organisations host the downstream consumer repositories.
- **[design-system](https://github.com/pointsav/pointsav-design-system)** — visual design tokens, component recipes, and brand conventions.
- **[factory-release-engineering](https://github.com/pointsav/factory-release-engineering)** — licensing matrix, contributor agreements, and governance policies.

<!-- EDITORIAL NOTE: a Woodfine customer-facing wiki is planned.
     Add the link here once the deployment is live. -->

---

## Contributing

This wiki is published under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).
Content originates from the PointSav and Woodfine contributor flow.
See [[contributing-as-engineer]], [[contributing-as-writer]], and
[[contributing-as-designer]] for onboarding guides. These articles are
planned for the help/ category; they will render as red links until written.

Corrections and additions follow the staging-tier commit flow described
in [[style-guide-topic]].

---

## Provenance

This artifact derives from research captured at draft time. Q1 and Q2
are now closed; all open questions resolved before this refinement pass.

- Citations consulted: [external:en.wikipedia.org/wiki/Main_Page] (home-page structural patterns); [conventions/cluster-wiki-draft-pipeline.md §3] (velocity tiers, bilingual requirement); [conventions/bcsc-disclosure-posture.md Rule 1] (forward-looking labelling, company/ card and customer-wiki placeholder)
- Workspace references: [clones/project-language/content-wiki-documentation/.claude/rules/naming-convention.md §4, §7, §9] (nine-category set, MOC landing pattern, investor-audience design); [clones/project-language/content-wiki-documentation/.claude/rules/content-contract.md §1, §2, §4, §7, §9] (index.md as home, category: root, URL routing)
- Q1 resolved at refinement: `index.md` confirmed as canonical home filename; `category: root` required; rename from cluster-outbound TOPIC-HOME.draft.md happens at gateway-commit time.
- Q2 resolved at refinement: `featured-topic.yaml` at repo root with schema `slug:` (required) + `since:` (optional) + `note:` (optional). ENGINE comment updated accordingly.
- Open questions deferred: verify the featured-TOPIC lead paragraph paraphrase against [[compounding-substrate]] §1–2 before the editorial pin is set for publication (low priority; editorial pass, not a structural question).
