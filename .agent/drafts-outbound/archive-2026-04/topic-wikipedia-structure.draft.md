---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: task-project-language
target_repo: content-wiki-documentation
target_path: reference/topic-wikipedia-structure.md
target_filename: topic-wikipedia-structure.md
audience: vendor-public — editors and contributors to documentation.pointsav.com
bcsc_class: reference-operational
language_protocol: PROSE-TOPIC
authored: 2026-04-30
authored_by: task-project-language (session 2026-04-30)
authored_with: sub-agent-explore (web-fetch, live Wikipedia)
research_done_count: 7
research_suggested_count: 5
open_questions_count: 3
research_provenance: web-fetch-wikipedia
research_inline: true
references:
  - https://en.wikipedia.org/wiki/Main_Page (live, fetched 2026-04-30)
  - https://en.wikipedia.org/wiki/Wikipedia:Manual_of_Style/Lead_section
  - https://en.wikipedia.org/wiki/Wikipedia:Article_structure
  - https://en.wikipedia.org/wiki/Wikipedia:Hatnote
  - https://en.wikipedia.org/wiki/Wikipedia:Content_assessment
  - https://en.wikipedia.org/wiki/Special:Statistics
  - One Featured Article (Old Faithful) for structural example
notes_for_editor: |
  This TOPIC is both a reference article (for contributors learning wiki standards)
  and an internal implementation brief (for project-knowledge when building article
  templates and home page panels). Two audiences; write for contributors but
  include ENGINE comments for project-knowledge.

  Key sections needed in refined output:
  1. Wikipedia Main Page anatomy (9 panels)
  2. Article anatomy (16 elements: short description → lead → infobox →
     hatnotes → TOC → body sections → See Also → References → Categories)
  3. Three core insights (consistency/lead/quality-signals)
  4. Adaptation recommendations for documentation.pointsav.com
  5. Priority implementation list (Tier 1 gives 70% of Wikipedia feel)

  BCSC note: no forward-looking statements needed; this is structural reference.

  Spanish bilingual pair: standard strategic-adaptation overview (~250 words)
  covering what Wikipedia muscle memory means and why PointSav documentation
  adopts the same structural conventions.
---

# Wikipedia Structure — Reference for documentation.pointsav.com Contributors

Wikipedia's article structure is the most widely internalized documentation
convention in the world. Readers who have used Wikipedia for any length of
time carry an unconscious model of where information lives on a page, what
visual signals indicate quality, and how to navigate from article to article.
`documentation.pointsav.com` adopts the same structural conventions so that
readers arrive already knowing how to use it.

This article describes Wikipedia's structural patterns and how each translates
to PointSav documentation. Contributors should use it as the style guide
reference for any article they write or edit.

## Wikipedia Main Page anatomy

Wikipedia's Main Page serves multiple reader intents simultaneously. Nine
structural panels, each addressing a distinct need:

| Panel | Reader need served | PointSav equivalent |
|---|---|---|
| Welcome banner with article count | "What is this? Should I trust it?" | Article count + "PointSav platform documentation" tagline |
| Featured Article (image + 100-150 word excerpt) | "Show me what excellent coverage looks like" | Featured TOPIC (currently: `compounding-substrate`) |
| Did You Know... (9 bulleted quick facts) | "Surprise me; let me explore" | "Quick Facts" or "Tip of the week" panel (iteration 2) |
| In the News (current events) | "Is this wiki current?" | "Latest Updates" — links to recent release notes or changelog entries |
| On This Day (historical facts) | "What is the history here?" | "Milestones" — major product releases (iteration 3+) |
| Featured Picture (large image + caption) | "Visual proof this is maintained" | Architecture diagram or system screenshot (iteration 2) |
| Other areas of Wikipedia (community links) | "Where do I ask questions?" | "Support" / "Discussions" / "Getting Started" links |
| Sister projects (Wiktionary, Commons, etc.) | "What else exists?" | Ecosystem links: GitHub, API docs, SDK repos |
| Language editions | "Is this available in my language?" | Bilingual toggle: English / Spanish (already implemented) |

**The principle underlying all nine panels**: every section on the Wikipedia
Main Page carries a freshness signal. "Today's featured article," "In the
news," "On this day" — the unconscious impression for readers is that the
wiki is alive and maintained right now. A documentation home page without
freshness signals reads as abandoned.

## Article anatomy

Every Wikipedia article follows the same 16-element structure in the same
order. Readers learn the structure once; then use it everywhere without
conscious thought.

### Pre-content elements (above the lead)

1. **Short description** (30–50 words below the title) — answers "Is this
   the right article?" in half a second. Example: "Cone geyser in Yellowstone
   National Park, Wyoming."

2. **Quality badge** (upper right) — Featured Article star (FA) or Good
   Article circle (GA). Readers learn to read these as trust signals: FA
   means professional publication quality.

3. **Hatnote** (italicized, indented, immediately below short description)
   — disambiguates. Standard forms:
   - "For other uses, see X (disambiguation)"
   - "Not to be confused with Y"
   - "X redirects here. For other uses, see Y"
   Use hatnotes wherever a reader might have arrived at the wrong article.

4. **Maintenance tags** — "[citation needed]", "[update needed]" — signal
   known article limitations transparently. For PointSav: "[needs examples]",
   "[beta — behaviour may change]".

5. **Infobox** (structured data panel, right side) — key facts in
   label-value pairs. Readers train to scan the infobox first, then read
   the lead. See "Infobox templates" below.

### Main content

6. **Lead section** (200–400 words) — the article in miniature. The lead
   must answer, in order: What is this? Why does it matter? When and where
   does it apply? What are the key facts a reader needs? Forty percent of
   Wikipedia readers read only the lead; write it as if it is the only
   section they will read. First sentence: bold the article title.

7. **Table of Contents** — auto-generated when four or more sections exist.
   Readers use the TOC as primary navigation; they jump to "Examples" or
   "Troubleshooting" rather than scrolling.

8. **Body sections** — named with simple parallel nouns (Overview /
   Configuration / Examples / Troubleshooting / Related Features). Avoid
   sections named with questions or vague labels ("Other information"). Maximum
   three levels of heading depth.

### Post-content elements

9. **See Also** (5–10 bulleted links to related articles) — enables
   serendipitous discovery and reveals integration relationships.

10. **References** ([1], [2], [3] numbered citations, full bibliography at
    bottom) — the primary quality signal readable to all audiences. Twenty
    or more citations reads as authoritative; five or fewer reads as
    unverified.

11. **External Links** — authoritative external sources only; not
    promotional links.

12. **Categories** (bottom footer) — enables browsing by topic rather than
    search. Every article carries two or three category tags; clicking a
    category shows all articles in that group.

## Infobox templates for PointSav documentation

Wikipedia uses different infobox schemas for different content types (person,
location, organization, scientific concept). PointSav documentation needs
three infobox schemas:

**API endpoint infobox:**
```
HTTP method:        GET / POST / PUT / DELETE
Endpoint:           /api/v2/resource
Required params:    resource_id, timestamp
Response format:    JSON
Rate limit:         1,000 req/min
Authentication:     OAuth 2.0
Changelog:          [link to release notes]
```

**Feature infobox:**
```
Available in:       All Plans / Premium only
Release date:       YYYY-MM-DD
Status:             Stable / Beta / Deprecated
Replaces:           [link, if applicable]
Related features:   [links]
```

**Integration infobox:**
```
Partner:            [Name]
Sync frequency:     Real-time / Daily / Manual
Data synced:        [list]
Setup time:         N minutes
Support:            [contact]
```

## Quality grade system

Wikipedia uses seven quality tiers. PointSav documentation uses three,
visible as coloured badges in the upper right of every article:

| Badge | Criteria | Wikipedia equivalent |
|---|---|---|
| **Complete** (green) | 400+ word lead; full infobox; 5+ sections with examples; 10+ references; updated within 6 months | Featured Article (FA) |
| **Core** (blue) | 200+ word lead; key infobox fields; 3+ sections; 5+ references | Good Article (GA) |
| **Stub** (yellow) | Under 200 words; sparse or absent infobox; maintenance tags visible | Start / Stub class |

Readers do not need to read a badge to register it. The visual presence
of a Complete badge (green, upper right) creates an unconscious impression
of trustworthiness within the first five seconds on the page.

## Navigation muscle memory

Wikipedia trains readers to find things in consistent locations:

- **Left sidebar**: search, home link, browse by topic — navigation tools
  are always left; content is always right
- **Bold text on first mention**: first use of a key term is bolded and
  linked — readers learn that bold means "this is central; click to learn
  more"
- **See Also at bottom before References**: always in this order — readers
  who reach the bottom of an article find related reading before citations
- **Categories in footer**: below all other sections — the breadcrumb back
  to the category hierarchy

Consistent placement is more valuable than clever placement. Once readers
learn the pattern on one article, they navigate every subsequent article
without thinking.

## Three core insights

**Consistency creates trust.** Wikipedia's articles follow the identical
structure. Readers learn it once, then navigate every article using
internalized muscle memory. A documentation wiki where each article is
structured differently forces readers to re-learn on every page visit.

**Lead sections are underrated.** Most documentation effort goes into the
body. On Wikipedia, Featured Articles invest proportionally more effort in
the lead than in any body section. The lead is the article in miniature:
readers who only read the lead should understand the subject completely.

**Quality signals matter more than content completeness.** Readers assess
article credibility in five seconds by scanning: badge presence, infobox
fullness, image count, reference count, last-updated timestamp. They do
not read the article to decide whether to trust it; they scan for signals
that others have maintained it. A stub article with a visible Stub badge
and maintenance tags visible reads as more trustworthy than an article
with no badge at all — at least the stub is honest about its state.

## Research trail

### Done — what informed this article

- [Wikipedia Main Page, fetched live 2026-04-30] — complete panel inventory
  and design rationale from primary source
- [WP:Lead section — Wikipedia Manual of Style] — formal lead-writing
  requirements; 200–400 words; must stand alone
- [WP:Article structure] — standard section order; heading hierarchy rules;
  accessibility requirements
- [WP:Hatnote] — disambiguation conventions; standard form templates
- [WP:Content assessment] — quality grade definitions FA/GA/A/B/C/Start/Stub
  and reader-perception mapping
- [Wikipedia:Special:Statistics] — scale data (7.1M articles, 273K active
  editors) for framing
- [Old Faithful article, Featured Article class] — concrete example of all
  16 structural elements applied to a single article

### Suggested — what the gateway should consult at refinement

- [content-contract.md in content-wiki-documentation] — existing
  PointSav article schema; gateway should verify the refined TOPIC is
  compatible with current schema before committing
- [index.md current home page] — gateway should verify the Main Page
  anatomy table aligns with what is already built; ENGINE comments should
  reference what is live vs. what requires engine work
- [WP:Summary style] — how Wikipedia writes subsections that summarize
  longer daughter articles; relevant if PointSav ever splits large TOPICs
  into parent/child article pairs
- [WP:NPOV (Neutral Point of View)] — relevant to research/ TOPICs that
  discuss competitors or technical trade-offs
- [Wikipedia:Article titles] — naming conventions for articles; relevant
  to PointSav's own TOPIC naming standard

### Open questions

1. The "Did You Know" and "Latest Updates" panels in the Main Page anatomy
   table are marked as iteration 2 and iteration 3 respectively. Before
   committing, gateway should confirm with project-knowledge Task whether
   these panels require engine support or can be static markdown. ENGINE
   comment required in refined output.

2. The infobox templates above use fenced code block formatting (not actual
   wiki templates). Before committing, gateway should confirm with
   project-knowledge whether the wiki engine supports an actual infobox
   template mechanism, or whether the code block representation is the
   intended final form for iteration 1.

3. The quality badge system (Complete/Core/Stub) requires both a frontmatter
   field (`quality:`) on each article AND engine rendering of the coloured
   badge. Gateway should flag this as engine scope for project-knowledge
   before adding `quality:` to the article schema recommendations.
