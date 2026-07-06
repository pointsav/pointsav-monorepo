---
artifact: brief
schema: foundry-brief-v1
archive: project-bim
topic: simulation-bim-library-denver-woodfine
status: reference
created: 2026-07-06
updated: 2026-07-06
---

# Brief — Denver Airport BIM Library simulation → Woodfine v2 design candidate

## What this is (read this first)

This is an internal design-thinking record, not a product claim. The Denver
International Airport (DEN) scenario below is a **pass-through structural
exercise only** — it exists to derive UX/IA decisions, nothing more. **No
Denver/airport content ships anywhere in the Woodfine artifact, in
`bim.woodfinegroup.com`, or in any other project-bim product surface.** The
Woodfine artifact (`.agent/briefs/assets/woodfine-bim-library.html`) was
built clean-sheet — verified zero references to Denver/airport/jet
bridge/concourse/check-in/gate (grep-confirmed by the agent that built it).

## Why this was run

Operator asked for a design simulation (PointSav sells its `os-privategit`
BIM CMS to Denver Airport) specifically to stress-test what a genuinely
cutting-edge BIM Object catalog looks like, then use the *pattern* learned
to inform a real v2 candidate for Woodfine's own BIM Object Library —
because the operator judged the current `bim.woodfinegroup.com` build is
not a polished baseline worth lightly annotating; this should be approached
like designing a new website from a clean sheet.

Separately, the operator flagged that project-bim's product framing may
have drifted: is a "Key Plan"/"Tile" the same thing as a "BIM Object," or
is it what architects build *from* BIM Objects? That question turned out to
be load-bearing for the whole exercise, so it's answered here first.

## The corrected definition — BIM Object vs. Composition

Three opus-model research agents (NBS/buildingSMART/bSDD, Uniclass 2015,
Revit Family-vs-Group/Assembly precedent) confirmed the operator's instinct:

> **A BIM Object is a single, atomic building-component specification** —
> one product/entity (a door, a chair, a wall assembly) identified by its
> IFC 4.3 entity class and Uniclass 2015 **Pr (Products)** or **Ss
> (Systems)** classification, carrying manufacturer/generic data and
> applicable property sets/constraints. **A Key Plan or Tile is a
> Composition** — a named template assembled *from* several BIM Objects,
> classified at Uniclass **EF (Elements-functions)** or **SL
> (Spaces-locations)** level. Industry-equivalent to a Revit Family (atomic)
> vs. a Revit Group/Assembly (composed), or a furniture dealer's "typical" /
> test-fit layout. A Composition is never a BIM Object itself.

Sources: [NBS — What are BIM objects?](https://www.thenbs.com/knowledge/what-are-bim-objects),
[NBS BIM Object Standard](https://source.thenbs.com/bimlibrary/nbs-bim-object-standard),
[buildingSMART Data Dictionary](https://www.buildingsmart.org/users/services/buildingsmart-data-dictionary/),
[Uniclass 2015 overview](https://rebim.io/classification-systems-uniclass-2015/),
[Groups vs Assemblies in Revit](https://atgusa.com/how-to-use-groups-and-assemblies-in-revit/).

**Where this workspace already had it right:** `BRIEF-app-privategit-bim.md`'s
2026-07-03 CMS repositioning filed Key Plans under a "Compositions" IA
section, separate from "Objects" — that direction was correct all along.

**Where this workspace had it wrong (now fixed):**
`.agent/plans/plan-bim-objects.md` stated "A Key Plan is the smallest BIM
Object unit" and "A Tile is a BIM Object composed of..." — corrected
2026-07-06 to define both as Compositions. (`topic-bim-token-what-it-is.draft.md`
was also checked — on close reading it does not actually conflate the two
terms; no edit was needed there.)

## How Compositions surface in a catalog (the design answer)

Using the Uniclass Pr→Ss→EF→SL ladder: BIM Objects get a classification chip
at Pr/Ss level; Compositions get a **visually distinct** chip at EF/SL level.
Compositions get their own top-level catalog tab, and a Composition's detail
view differs structurally from an Object's — instead of a manufacturer spec
sheet, it shows a **"Composed from" bill-of-objects** (linked entries to each
constituent BIM Object, reusing DTCG `{token.reference}` aliasing already in
the data model), the methodology/constraints it satisfies, and the computed
bounding box/net area.

## Part 1 — Denver Airport pass-through exercise

**Scenario:** DEN licenses `os-privategit` (`app-privategit-source` registry
backend + `app-privategit-bim` CMS frontend) to host the BIM Objects behind
its real, cited BIM program — Digital Facilities Infrastructure (DFI) IDSM,
BIM Project Execution Plan (BEP), 600+ Revit models across 93 buildings
(~17M sq ft), a standardized BIM-code location hierarchy, Autodesk
Revit/Civil3D/ACC toolchain, COBie handoff to IBM Maximo. (These DEN facts
are real and public — leaned into rather than invented, per
[flydenver.com's published DSMs](https://cdn.flydenver.com/app/uploads/2023/09/14083414/DENDigitalFacilitiesInfrastructureIDSM-1.pdf).)

**Design pattern derived:** a Carbon/Spectrum-grade token-driven catalog
shell, two tabs (Objects / Compositions), faceted search + classification
chips + spec tables + a bill-of-objects detail pattern for Compositions.
Worked Composition examples used: an airline **"Check-In Counter — Standard
(T-C1)"** and a leasehold-tenant **"Concourse Retail Concession — 800 SF
Typical"** — chosen because they're the two composition archetypes DEN
would actually need (airline-operated vs. tenant-leased space), and because
they map directly onto Woodfine's own architecture of Compositions
(furniture-based space-plans satisfying a methodology).

**Artifact:** `.agent/briefs/assets/denver-airport-bim-library.html`
(68.8 KB) — **internal only, not rendered/published, not to be referenced
outside this brief.** Accent color: signal amber (`#e8a417`/`#f0b429`)
against cool blue-slate neutrals, distinct from Woodfine's navy.

## Part 2 — Woodfine v2 design candidate

**This is a real proposal for operator review, not committed work.** Built
clean-sheet (per operator direction — not adapted from current
bim.woodfinegroup.com copy, only the *pattern* from Part 1 was reused, plus
real Woodfine facts below):

- **Brand:** Woodfine's real tokens — navy `#164679` (hover `#1D5795`,
  pressed `#10365C`) as the sole interactive accent, Inter/Source Serif
  4/Source Code Pro, sticky dark-navy masthead, white cards with the
  signature 3px navy top-rule, light footer with inline `<details>`
  disclosure (NI 51-102 / OSC SN 51-721 register, issuer-of-record
  attribution to Woodfine Capital Projects Inc.) — sourced from
  `app-mediakit-marketing-2/static/{tokens.css,fonts.css}`, already ported
  into `app-privategit-bim`'s own asset files.
- **Vocabulary:** Woodfine's real development site-types — Professional
  Centre, Suburban Office, Retail Select, Tech Industrial — not invented
  building names.
- **Objects tab:** 8 real Steelcase furniture BIM Objects (Leap V2,
  Migration SE, Ology, Groupwork Round, TS Pedestal, Universal File,
  Series 1, generic acoustic screen), Uniclass Pr/Ss classification,
  faceted sidebar, full spec/classification/download detail view.
- **Compositions tab:** the real Key Plan data model — Private Office,
  Medical, Business, Laboratory, Academic, Civic (3 sizes each) + Corporate
  Office (5 sizes) — **23 cards, confirmed correct** (resolved 2026-07-06).
  The historical "24 Key Plan cards" claim was a simple arithmetic slip,
  never checked against its own table (6×3 + 5 = 23, not 24). Cross-checked
  directly against the source-of-record
  `woodfine-bim-library/tokens/bim/key-plans.dtcg.json` (23 leaf entries) and
  `key-plans-registry.md`'s per-category tables (same 23) — both independent
  sources agree. No 24th Key Plan exists in this data set; the wider registry
  does list additional categories (Circulation/Utility, Professional
  Centre/Suburban Office Infrastructure, Retail Select, Tech Industrial) but
  those were never part of the "24 cards" claim's scope — they were never
  built into the live catalog page at all, and remain a separate, larger
  backlog of unimplemented Key Plans.
  Each Composition shows a "Composed from" bill-of-objects linking to
  constituent furniture Objects, plus the real circulation/lighting/
  accessibility constraints (DIN 18040, EN 12464-1, ASR A1.2, ArbStättV §12).

**Artifact:** `.agent/briefs/assets/woodfine-bim-library.html` (78.3 KB) —
rendered as a viewable Artifact for operator review.

## Findings applied to project-bim (concrete changes made 2026-07-06)

1. `.agent/plans/plan-bim-objects.md` — fixed the BIM Object/Composition
   conflation (see above).
2. `BRIEF-key-plans-site.md` renamed to `BRIEF-app-orchestration-bim.md`
   (`git mv`), status flipped back `superseded` → `active`. New forward-scope
   section: `app-orchestration-bim` is the **BIM Editor/Viewer** (in-browser
   IFC/BIM model viewing and editing) — a different product from
   `app-privategit-bim` (the CMS/catalog for BIM Objects + Compositions) and
   from `tool-keyplan` (the Composition compiler). No implementation started;
   this locks the definition before building it, per operator direction.
3. `BRIEF-app-privategit-bim.md` — added a 2026-07-06 section confirming the
   corrected CMS role and framing the Woodfine artifact above as a proposed
   v2 design direction, not committed work.
4. `briefs/README.md` — updated to 3 active briefs (`app-privategit-bim`,
   `app-orchestration-bim`, `tool-keyplan`) + this brief as reference.

## Open items for the operator

- ~~Decide whether to adopt the Woodfine v2 candidate~~ — **decided
  2026-07-06: adopt, build as real Rust integration** (not a static push).
  See `BRIEF-app-privategit-bim.md`'s implementation section for the plan.
- ~~Resolve the 23-vs-24 Key Plan count discrepancy~~ — **resolved
  2026-07-06: 23 is correct**, see above.
- `app-orchestration-bim`'s BIM Editor/Viewer scope is defined but
  unbuilt — genuinely future work, not scheduled here.
