---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-knowledge
target_repo: content-wiki-documentation
target_path: applications/
target_filename: topic-wikipedia-leapfrog-design.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-04-28T05:00:00Z
authored_by: task-project-knowledge (brief 04)
authored_with: sonnet-4-6
references:
  - vendor/pointsav-monorepo/app-mediakit-knowledge/docs/UX-DESIGN.md
  - vendor/pointsav-monorepo/app-mediakit-knowledge/ARCHITECTURE.md
  - clones/project-knowledge/.claude/drafts-outbound/topic-app-mediakit-knowledge.draft.md
  - conventions/cluster-wiki-draft-pipeline.md
  - https://en.wikipedia.org/wiki/Wikipedia:Vector_2022
  - https://meta.wikimedia.org/wiki/Research:Which_parts_of_an_article_do_readers_read
  - https://www.mdpi.com/2227-9709/12/3/97
  - https://blog.chromium.org/2023/05/an-update-on-lock-icon.html
  - https://en.wikipedia.org/wiki/BBC_Verify
  - https://www.grammarly.com/blog/product/better-writing-with-grammarly/
  - ni-51-102
  - osc-sn-51-721
notes_for_editor: |
  This TOPIC goes deeper than the chrome inventory in topic-app-mediakit-knowledge.draft.md §4.
  That sibling draft lists WHAT was kept and added; this TOPIC explains WHY and what the
  leapfrog-2030 structural headroom means.

  Load-bearing sections for refinement pass:

  §1 — The two-audience contract is set up here. The ~2 billion readers framing anchors
  the market-entry rationale; do not pare it to a throwaway sentence. The Phase 1 vs Phase 1.1
  split (which items shipped in which phase) is load-bearing for engineering readers; preserve
  the inventory items with their numbers so readers can cross-reference UX-DESIGN.md §1.

  §2 — Five additions, each with its own additive-discipline rationale. The BBC Verify
  reference (§2.4) is a structural-positioning note: "the page-level masthead follows the
  same structural logic as BBC Verify's branded byline approach." It is NOT a competitive
  comparison and must not be edited into one. Citation badges and FLI banner are the most
  technically distinctive items; do not compress these to one-liners.

  §3 — The "muscle memory, not literal mimicry" line is the headline for this section. The
  specific list of what IS tracked (line-height, body size, line length, serif/sans stack)
  vs what is NOT (colour palette, wordmark) must remain explicit — vague treatment of
  this section loses the key message for both audiences.

  §4 — Both audience paragraphs must stay. Financial community reader first, engineering
  reader second. Neither should mention the other. They are structurally parallel but
  describe different affordances. Do not merge into a single "readers" paragraph.

  §5 — Forward reference section. All headroom features (real-time collab, mobile editor,
  semantic browse, citation-graph navigation) must carry planned/intended language per BCSC
  posture. No timelines. Cross-references to sibling TOPICs are load-bearing citations.
---

# Wikipedia leapfrog design — muscle memory, 5% headroom

The design narrative behind `app-mediakit-knowledge`'s chrome: what
was kept from Wikipedia, what was added beyond Wikipedia, the
deliberate visual-identity divergence, and what the 5% leapfrog
headroom means for both readers and engineers.

This TOPIC goes deeper than the chrome inventory at
`topic-app-mediakit-knowledge.md §4`. That TOPIC lists what the
chrome contains; this TOPIC explains why each choice was made and
what the structural contract with two different audiences looks like.

---

## §1 The muscle-memory contract

### 1.1 Why Wikipedia patterns are the substrate's market-entry mechanism

Wikipedia has approximately two billion monthly readers. Those readers
have developed navigational reflexes over two decades of exposure to
a specific chrome — the location of tabs, the structure of article
sections, the visual language of footnotes and hatnotes, the left-rail
table of contents. These reflexes are not brand loyalty; they are
motor programs. A reader of any Wikipedia article navigates without
thinking about navigation.

When a new knowledge-publication substrate enters this environment, it
faces a structural choice. It can differentiate its chrome from
Wikipedia — building a visual identity distinct enough that readers
know immediately they are looking at something new. The cost of that
choice is that readers must learn new navigational motor programs
before they can work efficiently. Or it can adopt the same chrome,
accepting that some readers will briefly experience it as a Wikipedia
knockoff, in exchange for zero onboarding friction.

The substrate's choice is the second path, with one important
qualification: the adoption must be principled, not superficial.
A superficial copy fails the first time a reader looks for an affordance
that the original had but the copy omitted. A principled adoption
identifies the exact set of patterns that carry the muscle memory and
holds them inviolable.

### 1.2 The 95%/5% contract stated explicitly

`UX-DESIGN.md §3` names the structural allocation: 95% of the
chrome is the Wikipedia muscle-memory inventory, held inviolable across
all phases; 5% is the leapfrog headroom — additions that no Wikipedia
reader has encountered, shipped as additive rather than replacements,
so the baseline experience is undisturbed for readers who do not
attend to the additions.

The Vector 2022 skin — Wikipedia's current production skin — provides
the template. The Vector 2022 redesign process was deliberate about
additive discipline: the community RfC showed 165 oppose, 153 support,
yet the deployment proceeded because the close found rough consensus
that nothing had been removed from the existing experience. A subsequent
MDPI 2025 study found the redesign drove pageviews up 1.25% and
internal link clicks up 1.06 million monthly, with no significant
external referral disruption. The data supports additive. The substrate
follows the same discipline.

### 1.3 What was kept — the Phase 1 / Phase 1.1 split

The eighteen sacred patterns catalogued in `UX-DESIGN.md §1` are the
inventory. The engineering delivery splits across two phases:

**Phase 1 (shipped)** covers items where the muscle memory is expressed
in render output rather than chrome structure: footnote convention
(item 4, bracketed superscript plus back-arrow reference list), infobox
right-rail capability (item 7), link colours (item 10, blue unvisited /
purple visited / red missing target), body typography (item 11, serif
stack, body at minimum 17px, line-height 1.5 or greater, 45-75 character
line length), centre-top search placeholder (item 13), and mobile chrome
(item 16, hamburger left, sections collapsed by default, infobox stacked
under lead).

**Phase 1.1 (additive over Phase 1)** covers the chrome elements that
require structural additions to the template layer — the items that a
reader interacts with by clicking rather than by reading:

- Article / Talk tab pair (item 1), positioned at the top-left of the
  title row
- Read / Edit / View history tabs (item 2), positioned at the top-right
  of the title row, in that order
- Per-section `[edit]` pencils (item 3), right-floated on every heading
- End-of-article ordering (item 5): See also, Notes, References, Further
  reading, External links, Categories — the Wikipedia Manual of Style
  Layout sequence, because readers have learned to find citations at the
  bottom and that expectation should be met
- Hatnote (item 6), italic and indented, at the top of the article body
  above the infobox in source order, for disambiguation and cross-references
- Lead first-sentence convention (item 8), bolded subject plus copula
  plus one-line definition — the pattern that lets a reader identify
  the subject of any article within two seconds of arriving
- "From PointSav Knowledge" tagline (item 9), rendered under the article
  title in the Vector 2022 tagline element
- Collapsible left-rail table of contents (item 12), following the reader
  on scroll — the biggest visible structural change from Vector 2022,
  retained because it increased internal link clicks
- Language switcher (item 14), as a button next to the title, not buried
  in the sidebar — the Vector 2022 placement that reduced the friction
  of switching to the Spanish sibling
- Footer convention (item 15): categories, license notice, About /
  Contact links

Phase 1.1 also ships two IVC chrome placeholders (described in §2
below) — visual surfaces that carry no machinery in Phase 1.1 but
establish the structural location that Phase 7 fills.

---

## §2 What was added beyond Wikipedia

Five additions ship beyond the Wikipedia inventory. Each is additive —
no existing Wikipedia muscle-memory pattern is removed or altered. The
additions appear at locations and in visual registers that do not
interfere with the base reading experience. A reader who never attends
to any of them receives the Wikipedia experience they expected.

### 2.1 Citation badges (IVC marks)

Next to every inline `[citation-id]` reference, the chrome renders a
small badge in the C2PA "CR" pin glyph convention. The C2PA Content
Credentials mark is shipping in Adobe's Content Authenticity app and
on the Google Pixel 10 camera; it carries growing visual literacy
without requiring the substrate to invent a new symbol.

The default colour is neutral grey. This is the critical design
decision, taken directly from the TLS padlock lesson: Chrome removed
the green padlock in May 2023 because 89% of users misread the positive
state as "this site is trustworthy" rather than "the connection is
encrypted." When a positive-state signal is ubiquitous, it becomes
noise; only the exception deserves colour. Citation badges follow the
same calibration: neutral grey for all-verified, amber for source drift,
red for a missing or hash-mismatched citation, blue for forward-looking
information. The positive state (verified green) appears only when the
reader has explicitly toggled the "show all verified marks" density
setting.

This addition does not interfere with footnote conventions (item 4 in
the inventory). The badge appears at clause end, next to — not replacing
— the existing `[n]` footnote superscript. A reader who ignores badges
reads footnotes exactly as before.

### 2.2 FLI banner pattern

Articles whose frontmatter sets `forward_looking: true` render a
cautionary banner in reading view. In authoring view, the SAA editor
renders a blue squiggle on forward-looking statements that lack the
frontmatter flag, prompting the author to add it.

The banner is sourced from BCSC continuous-disclosure posture: NI
51-102 and OSC Staff Notice 51-721 require that forward-looking
information be labelled, carry a stated reasonable basis, and include
material assumptions and cautionary language. The banner is the reading-
surface expression of that requirement. An article about planned or
intended future capability that sets `forward_looking: true` carries the
banner as structural transparency, not as an editorial judgment.

This addition appears as a visual band immediately below the article
title in reading view, distinct in colour from the body, with a
formulaic cautionary sentence. It does not interfere with the hatnote
(item 6) or the lead first-sentence (item 8), both of which appear below
the banner in source order.

### 2.3 BCSC disclosure_class field

Articles carry a `disclosure_class` field in frontmatter with three
enumeration values: `narrative`, `financial`, `governance`. In the
current phase this field is invisible to readers — it is expressed in
the JSON-LD structured data in every rendered article's `<head>` block,
where LLM crawlers and structured data parsers can consume it, but it
does not appear as visible chrome.

Starting in Phase 8, a frontmatter linter checks that articles
classified `disclosure_class: financial` carry an iXBRL block,
that articles with `forward_looking: true` carry the cautionary language
patterns, and that any third-party governance claims appear with
documented `cites:` resolution. The field is invisible today and
consequential at Phase 8. It does not interfere with any Wikipedia
muscle-memory pattern because it has no visible reading-surface
expression until the Phase 8 linter activates.

### 2.4 IVC masthead band placeholder

A single horizontal strip at the top of every article, below the title
row. In Phase 1.1 this band renders placeholder text indicating that
verification is not yet available. Phase 7 fills the band with a live
verification summary line: the count of claims in the article, the
count verified, and the time since the last drift check.

The structural logic follows the same pattern as BBC Verify's branded
byline approach — a single line in the page chrome that gives the reader
an at-a-glance trust signal without distributing per-claim marks
throughout the prose. The masthead band is the location where the
positive verification state lives, reserving the inline badge colours
for exceptions only (§2.1). That structural split — positive signal in
the chrome band, exception signals inline — is the design that makes
the IVC system legible without becoming visual noise.

The Phase 1.1 placeholder band occupies the location now so that the
template is not restructured at Phase 7. Phase 7 replaces the
placeholder text with live data; the band's position, size, and colour
register do not change.

### 2.5 Reader density toggle

A preference UI with three states: Off, Exceptions only (default), All.
The setting persists across sessions. In Phase 1.1 the preference is
stored but has no effect on the chrome because the IVC machinery
(Phase 7) does not yet exist. Phase 7 honours the setting.

- **Off** — no IVC marks in the chrome; the pure Wikipedia reading
  experience for readers who do not want the verification layer
- **Exceptions only (default)** — neutral grey marks rendered at low
  visual weight; coloured exception marks (amber, red, blue) prominent
- **All** — verified-green marks rendered alongside exception marks;
  for auditors, regulators, and power readers who want the full
  verification picture

The default is Exceptions only. This is the operationalisation of the
TLS padlock lesson at the substrate layer: the baseline reading
experience suppresses the positive signal, showing only deviations from
the expected verified state.

This addition appears in the user preferences interface. It does not
interfere with any reading-surface Wikipedia pattern.

---

## §3 The deliberate visual-identity divergence

### 3.1 Muscle memory, not literal mimicry

The substrate's adoption of Wikipedia chrome is principled, not
imitative. The principle is: inherit the ergonomic substrate of the
muscle memory; diverge visually at every point where divergence does
not cost ergonomic familiarity.

What is the ergonomic substrate? It is the spatial and typographic
structure that trained two decades of navigational reflexes: where the
tabs are, where the table of contents is, where headings fall, how
footnotes are numbered, how section boundaries are delimited. A reader
navigates by these structural landmarks. Changing their position costs
the muscle memory; leaving their position unchanged preserves it.

What is NOT the ergonomic substrate? Colour. Typeface. Logo. Wordmark.
These are brand assets, not navigational affordances. A reader does not
navigate to a footnote by its colour. A reader does not find the table
of contents by recognising a wordmark. These elements are the points of
deliberate divergence.

### 3.2 What is tracked from Vector 2022

The substrate tracks Vector 2022's typography and spacing discipline
because these are ergonomic, not brand:

- Body text at minimum 17px (Wikipedia moved from 14px in 2023; the
  MDPI study found this contributed to the pageview gain)
- Line-height 1.5 or greater for body paragraphs
- 45 to 75 character line length, which is the decades-old typographic
  consensus for comfortable sustained reading
- Serif / sans-serif choice appropriate to the content register
  (Wikipedia uses Linux Libertine for body, sans-serif for chrome;
  the substrate adopts a real typographic choice rather than browser
  default)

These numbers are not Wikipedia's inventions. They derive from decades
of typographic research into reading comfort — Butterick's Practical
Typography, the Nielsen Norman Group research, the Bringhurst tradition.
Wikipedia adopted them because they are correct. The substrate adopts
them for the same reason. Adopting correct typography does not require
crediting Wikipedia; Wikipedia itself does not credit Bringhurst.

Vector 2022's content-column width (approximately 960px maximum) is
tracked because it reflects the same character-count reasoning at
typical screen sizes. The substrate does not reproduce Vector 2022's
exact pixel values as a deference to Wikipedia; it uses values that
satisfy the same typographic criteria.

### 3.3 What is deliberately divergent

The PointSav house colour palette applies to all chrome elements: tab
bar background, tab active-state indicator, link underline colour, section
heading colour, sidebar background, footer band. These are the visual
surfaces a reader sees as colour, not as structure. The colour is
distinctly not Wikipedia grey and Wikipedia blue.

Link colours honour the blue / purple / red visited-state convention
(item 10 in the muscle-memory inventory), because this convention is
navigational, not brand — readers have learned to interpret link colour
state across every browser and every website that uses default link
colours. The specific blue, the specific purple, and the specific red
are in the PointSav house palette, not Wikipedia's exact hex values.

No Wikimedia logo. No Wikipedia wordmark. No Vector 2022 stylesheet is
linked or loaded. The chrome is implemented in the substrate's own CSS
bundle, which happens to satisfy the same typographic and spatial
criteria as Vector 2022 because those criteria are correct.

A reader who arrives at `documentation.pointsav.com` and has spent
years reading Wikipedia will immediately know how to navigate. They will
not mistake the site for Wikipedia. The visual identity is distinct.
The navigational identity is familiar.

---

## §4 Why both audiences feel at home

### 4.1 The financial-community reader

A reader arriving from the financial community — an analyst, an investor,
a regulatory observer — arrives at `documentation.pointsav.com` via a
link to a specific article. They see a title, a tagline, article and
talk tabs at the top left, read and edit and view history tabs at the
top right. A bolded subject line opens the article body. A hatnote
above the lead paragraph cross-references related articles. Footnotes
appear inline and resolve at the bottom of the article. Categories
appear in a footer band.

They have never visited this site before. They have not been given a
tutorial. They navigate without friction because they have navigated
this structure thousands of times on Wikipedia.

The IVC masthead band is present — in Phase 1.1 it says verification
is not yet available; in Phase 7 it says how many claims in the article
have been verified. This reader knows what a news organisation's branded
verification label looks like. They understand the convention
immediately. They do not need to understand what IVC means technically
to form an intuition that the site takes sourcing seriously.

The `disclosure_class` field is invisible to this reader in Phase 1.1.
When the Phase 8 linter activates, articles classified as financial or
governance will carry structural markers that this reader's professional
context has trained them to recognise: iXBRL for financial-statement
content, forward-looking-information banners for content that describes
plans rather than facts.

### 4.2 The engineering reader

The same reader with an engineering background arrives at the same
article and has the same navigational experience. They also notice
things the financial-community reader may not attend to.

In the article's page source, `<script type="application/ld+json">`
carries a Schema.org `TechArticle` profile. The article URL slug is
stable and corresponds to a Markdown filename in a Git repository. The
`/feed.atom` endpoint is advertised in the page header. An `/llms.txt`
file at the site root maps the corpus structure for LLM crawler
ingestion.

In Phase 4, when the MCP server is live, the engineering reader can
point any MCP-compatible agent at the wiki's MCP manifest and interact
with the corpus programmatically — searching topics, fetching revisions,
proposing edits, navigating backlinks — all through the same structured
interface that the substrate exposes natively rather than through a
MediaWiki Action API shim.

In Phase 4's edit surface, the engineering reader sees `[[wikilink]]`
syntax, the same as Wikipedia's convention. They see a CodeMirror 6
editor they already recognise from Replit and Chrome DevTools. They see
the substrate squiggle lint markers that cite the rule they are enforcing.

Both readers are at home. Neither had to learn a new paradigm. The
financial-community reader recognised the Wikipedia reading surface; the
engineering reader recognised the Wikipedia reading surface AND the
substrate-native API surface that sits behind it.

---

## §5 Forward reference — the 5% leapfrog headroom

The 5% leapfrog headroom named in `UX-DESIGN.md §3` enumerates the
capability additions that no surveyed knowledge-publication system
ships as of 2026. Each item below is intended or planned for later
phases; no specific timelines are stated.

**Per-claim cryptographic verification badges** (IVC, Phase 7 planned)
— the inline badge system described in §2 above, wired to the Phase 7
content-addressed federation seam. The masthead band placeholder and
reader density toggle ship as structural locations in Phase 1.1; the
machinery intended to fill them lands at Phase 7.

**Real-time collaborative editing** (Phase 2 Step 7, opt-in) — the
`y-codemirror.next` and Yjs CRDT implementation that ships behind the
`--enable-collab` flag. Intended for trusted multi-user deployments
where multiple authors edit the same article simultaneously with shared
cursor awareness. Git remains canonical; the CRDT is intended as
session-ephemeral state that serialises to a commit on explicit save.

**Mobile editor** — a mobile-first edit surface is intended for a later
phase. The current editor is desktop-first; a mobile edit surface that
works on touch represents the single largest unmet-need gap relative to
Wikipedia, where approximately 95% of mobile sessions involve zero edits.

**Citation-graph navigation** — sibling to Wikipedia's "What links
here": intended affordances for navigating "what this article cites"
and "what cites this article", powered by the substrate's content-
addressed citation registry. Intended to land after the Phase 4 redb
wikilink graph ships.

**Semantic browse** — "articles like this" and "concepts adjacent to
this" surfaced from the citation graph, not from an LLM in the read
path. Intended as a complement to category browse. The brand promise
for the read path is deterministic; AI is structurally optional in
Ring 3 per the compounding-substrate architecture.

All items above are intended or planned. They carry no specific
delivery dates. Material changes to the delivery plan would be recorded
in the engineering phase plan documents and the workspace changelog.

Cross-references:

- `topic-app-mediakit-knowledge.md` — the engine architecture; chrome
  inventory at §4; compatibility surface rationale at §8
- `topic-source-of-truth-inversion.md` — the canonical / view /
  ephemeral pattern that makes Git the disclosure record and the
  CRDT a non-source-of-truth surface
- `topic-substrate-native-compatibility.md` — the Action API drop
  rationale and the substrate-native API surface set that replaces it
