# Brief: topic-wikipedia-leapfrog-design.md — bulk draft

**target**: Author a `foundry-draft-v1` bulk draft on the muscle-memory chrome design narrative — what was kept from Wikipedia, what was added beyond Wikipedia, and the deliberate visual-identity divergence — for vendor-public TOPIC publication via the Reverse-Funnel Editorial Pattern.
**target_files**:
- Write: `/srv/foundry/clones/project-knowledge/.claude/drafts-outbound/topic-wikipedia-leapfrog-design.draft.md`
- Append JSONL event: `/srv/foundry/data/training-corpus/apprenticeship/prose-edit/pointsav/draft-2026-04-28-topic-wikipedia-leapfrog-design.jsonl`
**expected_output**: One `.draft.md` file with valid `foundry-draft-v1` frontmatter (`state: draft-pending-language-pass`, `audience: vendor-public`, `bcsc_class: no-disclosure-implication`, `language_protocol: PROSE-TOPIC`) + ~300–400 lines of body covering the five sections in the Specification. No `.es.md` — bilingual pair is project-language's responsibility. One JSONL line with `event: draft-created`.
**max_response_lines**: 400
**model_tier**: sonnet
**parallelisable**: no (writes)
**confidence_gate_passes**: yes — well-specified structural exposition; sub-agent fills substance not framing
**layer_scope**: task
**anti_slop_check**: this draft satisfies the wiki-leg of the project-knowledge Tetrad (manifest line `topic-wikipedia-leapfrog-design.md` listed as future planned TOPIC) and grounds the substrate's UX choices in publishable narrative form
**dependencies**:
- Read: `/srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/UX-DESIGN.md` §1 (muscle-memory chrome inventory)
- Read: `/srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/UX-DESIGN.md` §3 (5% leapfrog headroom)
- Read: `/srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/UX-DESIGN.md` §4 (FLI banner, IVC masthead band, citation badges, density toggle)
- Read: `/srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/UX-DESIGN.md` §5.3 (SAA squiggle pattern)
- Read: `/srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/ARCHITECTURE.md` §3 Phase 1.1 + §4 + §6 (frontmatter schema)
- Read: `/srv/foundry/clones/project-knowledge/.claude/drafts-outbound/topic-app-mediakit-knowledge.draft.md` §4 (existing chrome treatment in the engine TOPIC — go deeper here, do not duplicate)
- Read: `/srv/foundry/conventions/cluster-wiki-draft-pipeline.md` §2 + §2.2 (frontmatter schema + bulk discipline)

## Specification

The TOPIC explains the design narrative behind the muscle-memory chrome: not just what the chrome is, but why each choice was made and what the leapfrog-2030 headroom means structurally. It goes deeper than `topic-app-mediakit-knowledge.draft.md §4` (which lists the chrome inventory); this TOPIC explains the rationale and the two-audience contract.

Section structure to author in order:

**§1 — The muscle-memory contract (50–70 lines).** Open with the ~2 billion-monthly-readers framing for why Wikipedia patterns are the substrate's market-entry mechanism. State the 95% Wikipedia muscle-memory + 5% leapfrog headroom contract explicitly. Enumerate the Wikipedia patterns carried unchanged (UX-DESIGN.md §1 items 1-3, 5, 6, 8, 9, 12, 14, 15: article/talk tabs, read/edit/history tabs, per-section [edit] pencils, end-of-article ordering, hatnote, lead first-sentence convention, tagline, collapsible left-rail TOC, language switcher, footer convention). Note Phase 1 vs Phase 1.1 split per ARCHITECTURE.md §3 (Phase 1 = footnotes/infobox/link-colours/typography/search-placeholder/mobile chrome; Phase 1.1 = the rest).

**§2 — What was added beyond Wikipedia (60–80 lines).** Five additions, each with a one-paragraph explanation of why it does NOT violate the muscle-memory contract (additive-not-removal discipline): citation badges (IVC marks; neutral grey by default; C2PA "CR" pin glyph at clause end; colour only for exceptions); FLI banner pattern (blue squiggle in authoring; cautionary banner in reading view; `forward_looking: true` frontmatter + BCSC `disclosure_class` field per ARCH §6); BCSC disclosure_class field (`narrative | financial | governance`; invisible to readers until Phase 8 linter flags); IVC masthead band placeholder (single horizontal strip below title row; placeholder in Phase 1.1; pattern reference: BBC Verify branded-byline approach per UX-DESIGN.md §4.5); reader density toggle (three states; persists across sessions; operationalises TLS padlock lesson).

**§3 — The deliberate visual-identity divergence (40–60 lines).** Principle: "muscle memory, not literal mimicry." Wikipedia is a reference substrate; this engine is a knowledge substrate. The distinction shows in colour palette and typographic register, not in information architecture. Track Vector 2022 skin spacing and typography (line-height ≥1.5, body ≥17px, 45–75 char line length, serif/sans choice) — these are the ergonomic substrate of the muscle memory, not brand assets. Adopting them is correct; not crediting them is fine because they are derived from decades of typography research, not invented by Wikipedia. Distinct PointSav colour palette for chrome, tab bars, link underlines, section headings. Link colours honour blue/purple/red visited-state convention (UX-DESIGN.md §1 item 10) but in the house palette. No Wikimedia logo, no Wikipedia wordmark, no Vector 2022 stylesheet.

**§4 — Why both audiences feel at home (30–40 lines).** Financial-community reader paragraph: describes arrival at `documentation.pointsav.com`, finding article/talk tabs, bolded subject line, hatnote, footnote convention. The IVC masthead band is present but quiet. Reader has never seen this site and does not need a tutorial. Engineering reader paragraph: same arrival, but reader also notices JSON-LD in page source, `/feed.atom` endpoint, `[[wikilink]]` syntax in edit mode, MCP server path appearing in Phase 4. Both readers are at home for different reasons; neither had to learn a new paradigm.

**§5 — Forward reference (10–15 lines).** The 5% leapfrog headroom in UX-DESIGN.md §3 (real-time collab, mobile editor, semantic browse, citation-graph navigation) is the planned surface for later phases. Mark as planned/intended language per BCSC posture. No specific timelines. Cross-reference sibling TOPICs: `topic-app-mediakit-knowledge.md` (engine architecture), `topic-source-of-truth-inversion.md` (canonical/view/ephemeral pattern), `topic-substrate-native-compatibility.md` (Action API drop rationale).

The sub-agent must NOT: attempt register-discipline (no Bloomberg pruning — project-language's role), resolve citation IDs (inline URLs OK), generate a Spanish sibling, write forward-looking content as current fact, or duplicate the engine TOPIC's §4 chrome inventory verbatim.

## Acceptance criteria

- Valid `foundry-draft-v1` frontmatter with all required fields populated
- `state: draft-pending-language-pass`, `audience: vendor-public`, `bcsc_class: no-disclosure-implication`, `language_protocol: PROSE-TOPIC`, `authored_with: sonnet-4-6`
- All five sections present with line counts within the specified ranges
- §2 enumerates exactly the five additions (citation badges, FLI banner, disclosure_class, IVC masthead band, density toggle)
- §3 makes the "muscle memory not literal mimicry" framing explicit
- §4 contains both audience paragraphs
- §5 carries planned/intended language for the leapfrog headroom features
- `notes_for_editor:` block present, naming load-bearing sections for refinement
- JSONL `draft-created` event appended to corpus path

## Risks / unknowns

- The two-audience contract is load-bearing; sub-agent must not let one audience crowd out the other in any section
- BBC Verify reference (UX-DESIGN.md §4.5) is a structural-positioning note, not a competitive comparison — sub-agent must frame as "pattern reference" not "alternative to"
- The Vector 2022 typography tracking + colour-palette divergence may feel like a fine line; sub-agent should be specific about WHAT is tracked (spacing, line-height, font-stack) vs WHAT is divergent (colour palette, brand wordmark) rather than glossing in summary
