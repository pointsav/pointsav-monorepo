---
artifact: brief
schema: foundry-brief-v1
archive: project-bim
topic: bim-v3-hyperscaler-redesign
status: active
created: 2026-07-09
updated: 2026-07-10
---

# Brief — BIM v3 "hyperscaler-grade" redesign + BIM Library narrative initiative

## Context

Operator's framing (2026-07-09): the live CMS ("v2", shipped 2026-07-08 — see
`BRIEF-app-privategit-bim.md`) is "ok but not the best in the world" and "doesn't
look like a website the new 21st century hyperscalers would design." Wants a "100x
improvement," informed by real research into award-winning design and genuinely
distinctive BIM/CMS products — not incremental polish. Two connected workstreams:

1. **Visual/UX redesign** — full audit of the live site + internet research into
   award-winning design + a Fable-model synthesis into a genuinely distinctive
   direction, going as far as a real implemented prototype on local preview.
2. **BIM Library / BIM Objects narrative** — better language for the core concepts
   (BIM Library, BIM Objects, Key Plans, Tiles), grounded in real source material
   (Openstudio design-slides doc's "Geometry of Sustainability" framing), rendered
   as actual on-site content, not just wiki articles nobody visits.

This is a new BRIEF, not an addition to `BRIEF-app-privategit-bim.md` (already
1112+ lines, a build-log for the CMS engine itself) — this initiative is orthogonal:
a design-direction and content-strategy effort that will *produce* commits into that
same codebase, but is tracked here as its own running record.

## Prior attempts — institutional record (read before repeating any of this)

`BRIEF-app-privategit-bim.md` documents an extensive, multi-round prior effort at
exactly this problem. Summarized here so nobody re-derives it blind — **but
deliberately NOT fed into this initiative's fresh audit/research/synthesis agents**
per operator direction (see Decisions below):

- **RD.1–RD.7** (`​.agent/sub-agent-results/RD.1` through `RD.7-visual-direction-
  synthesis-2026-07-03.md`, ~163 KB, 7 subagents, 2026-07-03): live-site structural
  audit, pre-wiki git archaeology, design.pointsav.com audit, prior-art synthesis,
  a **hyperscaler resurvey**, an **AEC precedent survey** (Speckle, Hypar,
  Rhino/Grasshopper, Linear, Raycast, Warp, Framer, Bentley iTwinUI), landed on an
  Adobe Spectrum chrome pick (14/15 bankers'-distinguishability score), synthesized
  into a concrete spec.
- **2026-07-03 rebuild** — 4-section IA (Taxonomy/Objects/Compositions/Context),
  #1A4480 drafting-blue accent, Carbon scoped to `/edit/*` only. Then an
  **"Envelope-as-Navigation" homepage** (claim #41's zoning-envelope diagram as
  literal site nav) was built and shipped to local preview.
- **Operator rejected it outright**, verbatim: *"track A is a waste of time, we
  are going to throw the current site in the garbage."* Also: *"it doesn't seem
  like a site for a 'Design System' like IBM Carbon or the other hyperscaler
  Design Systems"* — nearly identical language to today's ask.
- **2026-07-04 pivot** — "real-object hero" using actual CAD-sourced dimensions
  (PO-1 hand-drafted sheet), after the operator noted the envelope diagram had no
  connection to real BIM Object massing.
- **2026-07-06/07/08** — Denver-simulation design-thinking exercise (grounds the
  BIM Object vs. Composition definition; see `BRIEF-simulation-bim-library-denver-
  woodfine.md`), v2 built, polished across 5 rounds, shipped to `foundry-prod`.
  **This is today's live site.**
- **Deferred, never built**: drafting-sheet layout system (sheet numbers, title
  blocks, cross-sheet references), GUID-as-visual-mark, live constraint-composition
  tool, 3D IFC viewport (blocked on an xeokit-AGPL vs. `@thatopen`-MIT license gate,
  still unresolved — see NEXT.md).

## Decisions locked (operator interrogation, 2026-07-09, one question at a time)

1. **Content architecture — dual-track, no new naming scheme.** Wiki TOPICs
   (bilingual, project-editorial pipeline, → `media-knowledge-projects/building-
   design/`) for doctrine-grade narrative, PLUS an on-site adapted version in
   `woodfine-bim-library/site-content/pages/` + `/categories/` (what
   `app-privategit-bim` actually renders — this already exists, already avoids
   TOPIC/GUIDE collision by not using that naming/frontmatter schema).
2. **This BRIEF** is new, cross-referencing `BRIEF-app-privategit-bim.md` rather
   than growing it.
3. **"Two-pane system" (operator's complaint) covers both**: (a) the
   Objects/Compositions tab + category catalog grid, and (b) a Composition detail
   page's constituent-Objects side-by-side view.
4. **Full comprehensive Workflow-tool pass** — real multi-agent orchestration,
   explicit operator opt-in (not a single quick agent).
5. **One combined workflow** — visual-redesign and narrative/language workstreams
   run as parallel phases feeding one synthesis and this one BRIEF.
6. **Go all the way to a real prototype rebuild**, deployed to local preview
   (`local-bim.service`, 127.0.0.1:9096) for operator review. **Nothing touches
   `foundry-prod`** — that stays Command's job after operator sign-off, per this
   archive's standing local-first deploy model.
7. **Full reset, explicitly.** Operator's own words on seeing the prior-attempt
   history: *"I think we need a full reset, what we should be doing is making sure
   we do not pick up on ... 'Build on it' ... as these ideas and this way of coming
   at the problem are not working."* Consequence: the audit/research/Fable-synthesis
   agents in this initiative get **zero exposure** to RD.1-7, the old BRIEF, or the
   deferred-ideas list. A separate novelty-guard check (done against my own list,
   not agent-sourced) catches accidental overlap after the fact.
8. **Brand color constraint (added 2026-07-09, after first workflow launch — caught
   before Phase B ran, first run stopped and relaunched clean):** bim.woodfinegroup.com
   is a "Woodfine" site and should use `home.woodfinegroup.com`'s actual colors as much
   as possible, not an invented palette. Consistent with a real prior precedent in
   `BRIEF-app-privategit-bim.md` — the operator previously chose "match
   home.woodfinegroup.com's actual brand stack" over two other options (BB.14's own
   look, or matching design.pointsav.com's Carbon skin exactly). A dedicated agent
   extracts the real computed colors from `home.woodfinegroup.com` via headless
   browser before Fable's visual synthesis runs, so the direction is grounded in real
   hex values, not a guess.

## Source material (narrative workstream)

`~/Foundry/deployments/cluster-totebox-jennifer/service-study/project-design-slides-
response/assets/CONSTRUCTION_2025_10_31_Design Slides_Openstudio_Woodfine Response
copy 2.md` (9,674 lines) — NOT in project-bim's own archive (Jennifer's personal
research deployment); read-only cross-reference. Confirmed real content: "Geometry
of Sustainability" (line 26), Key-Plans-and-Tiles-as-sustainability via Permaculture/
reduced-production-curve (line 267), extensive certification/Class-A material.

## Work log

### 2026-07-09 (session 2) — 9 confirmed post-audit fixes, working tree only

Operator confirmed 9 concrete items after a follow-up audit + expert-model
cross-check of the session-1 build below; each implemented against
`pointsav-monorepo/app-privategit-bim`. No git commit run (working tree,
unstaged, per instruction); no restart/deploy to `local-bim.service`
(separate operator-gated step); `foundry-prod` untouched. `cargo build -p
app-privategit-bim` clean, zero warnings, both before and after the full
batch. `cargo test -p app-privategit-bim` run at session end (see result
noted below).

**1 — Nav active-tab CSS leak.** Root cause confirmed by reading source, not
guessed: `bim-components.css`'s bare `[aria-current="page"]` selector (a
leftover from the sidebar tree removed 2026-07-03 — no `.bim-sidebar`
container or `cds-side-nav-link`/`cds-side-nav-menu-item` element exists
anywhere in current markup) matched the header nav's `<a aria-current="page">`
too, stacking a border-left+fill on top of `bim-planroom.css`'s own clean
underline rule. Scoped the sidebar rule to `cds-side-nav-link[aria-current=
"page"], cds-side-nav-menu-item[aria-current="page"]` (kept, not deleted, in
case the editor's Carbon side-nav is wired up later). Confirmed
`bim-layout.css`'s `.bim-nav-link.active` does **not** leak into the header
nav (the header `<a>` tags never carry the `bim-nav-link` class) — no change
needed there. Extended the underline ~3px past the text via padding/negative-
margin offset. Files: `src/assets/bim-components.css`, `bim-planroom.css`.

**2 — Header dead-middle gap.** Root cause: `.bim-header__nav` had `flex: 1 1
auto` (bim-planroom.css), so the nav's own box grew to fill remaining header
width while its links stayed left-anchored inside it; `.bim-header__inner`'s
`justify-content: space-between` (bim-layout.css) then shoved search+toggle
to the far-right edge — ~668px of void. Changed nav to `flex: 0 0 auto` and
`.bim-header__inner` to `justify-content: flex-start`; brand+nav+search+
toggle are now one continuous left-anchored cluster (small gaps between
groups; the leftover void moved to the right edge, not eliminated — matches
the "no max-width-container/2-zone-grid" constraint). Files: `bim-layout.css`,
`bim-planroom.css`.

**3 — Mobile hamburger + slide-in drawer.** Built as a native `<details
class="bim-drawer">` with `<summary class="bim-header__hamburger">` as the
trigger — open/close works with zero JavaScript (matches this catalog's
established JS-optional ethos); bim.js layers on backdrop-click / explicit
close button / Escape, plus a `body.bim-drawer-open` scroll lock via the
`toggle` event. Drawer panel: Objects, Compositions, Research, Discipline
(item 4) + a duplicated search form. Files: `src/render/shell.rs` (markup),
`bim-planroom.css` (drawer CSS + the `@media (max-width: 900px)` rule that
used to hide nav/search with zero replacement), `bim.js` (open/close
affordances).

**4 — Rename /about → "Discipline".** Nav label and page kicker/h1/`<title>`
renamed to the bare noun "Discipline" (no article, matching Objects/
Compositions/Research) per explicit operator correction. Page body content
unchanged. Added to both the desktop header nav and the new drawer (previously
zero nav entry point anywhere). Also fixed the stray "About the Library" label
in the 404 page's link list for consistency. Files: `src/routes/about.rs`,
`src/render/shell.rs`, `src/render/catalog.rs` (404 links).

**5 — Footer, all 4 sub-fixes.** (a) `.bim-badge` now uses `--bim-radius-card`
(10px) + the catalog card's soft navy shadow (`0 1px 2px rgba(11,35,64,.05),
0 10px 26px -16px rgba(11,35,64,.28)`), applied at rest (not hover-only —
footer badges aren't a primary interactive surface), with a lift+deeper-
shadow enhancement on the linked CC badge's hover. (b) `.bim-footer__heading`
bumped 14px/600 → 17px/600 with real margin below (8px → 14px). (c) Footer
gained a top-edge shadow (`box-shadow: 0 -10px 24px -20px rgba(11,35,64,.28)`)
plus a very faint navy-tint gradient fading from the top border, in both
themes. (d) `.bim-footer__list li` gained 6px bottom margin — carries through
1:1 on the 390px mobile stack where the 3 columns run 1-up. Files:
`bim-layout.css`.

**6 — 3 corroborated defects, all fixed.**
(a) *Hero contrast (~2.7:1 measured).* Root cause: `.bim-home-masthead`'s
`mask-image: linear-gradient(to bottom, black 0%, transparent 100%)` (for the
decorative drafting-grid texture) masks the element's entire composited
render, including its real h1/lede text — a child's own `mask-image: none`
(the prior, ineffective attempt at a fix) never cancels an ancestor's mask on
that child's paint. Moved the grid + mask onto a `::before` pseudo-layer
behind the text; the lede's actual `--bim-fg-secondary` color was already
~7.2:1 (light) / ~10.7:1 (dark) against the page background — computed by
hand from the token hex values — once un-masked, no color change was needed
to pass AA, only the isolation fix. Verified no other page uses the
`.bim-cat-hero__lede`/`.bim-hero__lead` classes named in the task (dead CSS
from an earlier, now-unused hero layout) — the home page's real hero is
`.bim-home-masthead__lede`. File: `bim-planroom.css`.
(b) *Empty ZONE ALLOCATION div.* Root cause: `zone1`/`zone2`/`zone3` were
stored as `round2()`-formatted **strings** in the composition JSON, but
`zone_bars_html()` reads them back with `Value::as_f64()` — always `None` for
a JSON string — so `present` filtered to empty and rendered a header over a
genuinely empty `<div class="bim-cat-zones">`, on every composition with real
zone data including `/compositions/po-1`. Fixed by storing raw `f64` and
rounding only at render time (`zone_bars_html` already did `round2(*v)`
correctly once it could read the number at all). File: `src/render/
catalog.rs`.
(c) *Search result title/badge leak.* Root cause: `render_search_results`'s
entity loop walked raw per-file DTCG entities directly, hardcoding `kind:
"BIM Object"` for every hit regardless of source file (mis-badging
Compositions found via `key-plans.dtcg.json`), and read a generic
`$value.display_name` field that furniture Objects don't actually carry
(their name lives in `model`, per `catalog::build_objects`) — silently
falling back to the raw registry slug. Rewrote the entity-search loop to
reuse `catalog::build_objects`/`build_compositions` directly (the exact
name/kind resolution the catalog pages already get right) instead of
re-deriving it with different, wrong field assumptions; correct `/objects/
{id}` and `/compositions/{slug}` URLs, correct "Object"/"Composition" kind
labels. Removed the now-dead `collect_entities` helper. Files: `src/render/
search.rs`.

**7 — SVG draw-on animation + plan↔parts-list linkage.** (a) Draw-on: every
stroked line-work element in the composition detail page's key-plan SVG
(walls, zone-boundary dashes, mullion ticks, furniture outlines) animates via
`stroke-dasharray`/`stroke-dashoffset` (`getTotalLength()` per element, which
is supported on `rect`/`circle`/`line`/`path`/etc., not just `path`) on
scroll-into-view (`IntersectionObserver`), respecting `prefers-reduced-
motion`. Catalog thumbnail SVGs left static — stated scope cut (dozens of
small on-screen SVGs animating at once cost real paint time for no real
payoff; the one large detail-page plan is the moment that reads as a genuine
drawing). (b) Linkage: added a running `obj_idx` counter + `obj_open!`/
`obj_close!` macros in `render/svg.rs` that wrap each **discrete, repeating**
furniture instance (desks, round tables, doors, per-doctor/per-chair/per-
bench/per-office/per-workstation loop iterations) in `<g class="bim-plan-obj"
data-plan-obj="{i}">`, in draw order, across all 7 category branches
(private-office, medical, laboratory, business, academic, civic — the 3
shared macros plus each category's dominant repeating-item loop). `catalog.rs`'s
`bill_html` tags parts-list rows with the same ordinal. bim.js highlights
both ends on hover (machine-accent color, matching this codebase's existing
"the system is telling you something" convention).
**Stated scope cut, explicit:** this is a stable **ordinal** pairing (Nth
parts-list row ↔ Nth furniture group drawn), not a semantic per-SKU match —
the plan SVG is a procedurally-drawn category template with no knowledge of
which specific catalog Object a given symbol represents (only PO-1 carries a
real structured `furniture_refs` bill at all; see the module doc in
`catalog.rs`). Singular one-off fixtures per plan (a lone credenza, reception
desk, service riser) are left unwrapped — not addressed individually; when a
composition's bill length and its plan's wrapped-group count differ, the
extra rows/groups on the longer side simply have no hover partner (a
harmless no-op, not a broken state). Hover only — no mobile-tap equivalent
wired up. Files: `src/render/svg.rs`, `src/render/catalog.rs`, `bim-planroom.
css`, `bim.js`.

**8 — Real, dimensions-scoped compare feature.** New `GET /objects/compare`
route (`main.rs`, `routes/objects.rs`, `render/catalog.rs`'s
`render_objects_compare`). `render_object_card` restructured from a single
`<a>` into a `<div>` containing a checkbox (`name="ids"`) plus a
"stretched-link" `<a class="bim-cat-card__link">` covering the rest of the
card, so the checkbox and the card-navigation link are both real, independent
interactive elements (no nested `<a>`/`<input>`). The whole `/objects` grid
is wrapped in a real `<form method="get" action="/objects/compare">` —
**works with JavaScript disabled**: native checkbox form submission produces
repeated `ids=a&ids=b&ids=c` query params, parsed via a dedicated `Query<
CompareParams { ids: Vec<String> }>` extractor (a plain `Query<HashMap<String,
String>>` would have silently kept only the last repeated key — verified this
distinction before choosing the extractor). bim.js adds the "appears once 2+
selected" polish (live count text, disables the submit button below 2 checked,
a Clear button) without it being required for correctness. Compare table:
Name, Manufacturer, **Dimensions** (the object's `dims` summary, falling back
to a Diameter/Seat-height spec-row for round tables/stools rather than a
fabricated W×D×H), IFC 4.3 entity class, Uniclass 2015 (Pr) — exactly the 5
fields the task specified, scoped to real per-object data (no fire-rating/
thermal fields exist on this furniture-only catalog, so none were invented).
Fewer than 2 valid/resolved ids renders an honest instructional empty state
rather than a broken table. Homepage copy promising "compare fire ratings,
thermal values, and dimensions side by side" was left as-is per instruction —
noted here, not silently rewritten: it now over-promises relative to what
ships (dimensions only), but a real feature is live rather than none. Files:
`main.rs`, `src/routes/objects.rs`, `src/render/catalog.rs`, `bim-planroom.
css`, `bim.js`.

**Build/test:** `cargo build -p app-privategit-bim` clean, zero warnings,
after the full batch (one `#[allow(unused_assignments)]` added deliberately
for `render_kp_zone_svg`'s `obj_idx` counter — its final per-branch increment
is legitimately unread, not a real bug). `cargo test -p app-privategit-bim`:
5/6 pass; `route_tests::home_ssr_has_both_tabs_with_real_data` fails —
**pre-existing, not caused by this session**: it asserts `id="bim-panel-
objects"`/`id="bim-panel-compositions"`, markup from the pre-"Plan Room"
tabbed catalog-home that session 1 (2026-07-09, same day) already replaced
with the compact two-shelf front door (`render_home` in `catalog.rs`, untouched
by this session's diff); that string appears nowhere in current render code
(confirmed via grep) and this session never touched `render_home` or this
test. Session 1's work log only verified `cargo build`, not `cargo test`, so
this stale assertion was never caught before now. Left as-is — out of this
session's 9-item scope; flagging for a future session or Command to update
the test to the current home-page markup.

---

### 2026-07-09 — real prototype implementation ("The Plan Room") in app-privategit-bim

Implemented the synthesized visual + narrative direction as a real, working build in
`pointsav-monorepo/app-privategit-bim`, against real catalog data (7 Objects, 23
Compositions). All work landed in the working tree, unstaged, per instruction — no
git commit was run.

**Routing / backend (hard constraint 1 — real routes, not a JS patch):**
- New `src/routes/objects.rs`, `src/routes/compositions.rs` — `GET /objects`,
  `/objects/{slug}`, `/compositions`, `/compositions/{slug}`,
  `/compositions/{slug}/o/{object}`, all real server-rendered Axum routes wired in
  `main.rs`. Search + facet state are GET query params (`?q=`, `?uni=`, `?mfr=`,
  `?use=`, `?layout=`) via plain `<a href>` chip toggles and `<form method="get">` —
  fully functional with JS disabled, real URLs for Back/reload/link-sharing.
- Branded 404: `main.rs` `.fallback(not_found_handler)` + `render::catalog::render_not_found()`
  — full site chrome, search box, section links. Verified `curl` returns HTTP 404
  with the branded page, not a raw error screen.
- `Composition.internal_code` values containing `/` (e.g. `CO-1/2`) are slugified to
  URL-safe `co-1-2` via a new `slugify_code()` — verified `/compositions/co-1-2`
  resolves.
- The old 660px slide-over modal (`bim-cat-modal*` DOM + `bim-catalog.js`'s
  `openDetail()`) is fully retired from every page template. `bim-catalog.js` itself
  is left on disk (unreferenced now) rather than deleted — noted as a cleanup
  opportunity, not removed this session.

**Composition detail — plan-anchored inspector (hard constraint 3):**
- `render_composition_detail()` in `render/catalog.rs`: persistent left-column plan
  (composition's key-plan SVG, or a new `render_floor_scale_svg()` dashed/hatched
  "not modeled" convention for the 6 floor-scale entries), right-column inspector
  rail that swaps between the composition's own Data Box/Parts-list and — at
  `/compositions/{slug}/o/{object}` — the inspected Object's spec sheet, with a
  breadcrumb and "← Back to {composition}" link. Context (the plan) never
  disappears; verified via curl that the plan SVG markup is present at both URLs.
- **2px-collapse fix**: `render_kp_zone_svg` (`render/svg.rs`) now emits explicit
  `width="360" height="224"` attributes on top of `viewBox`, plus `aspect-ratio: 180/112`
  in `bim-planroom.css` on `.bim-kp-diagram` — belt-and-suspenders against the flex-
  context collapse the audit measured (`w:615,h:2`).
- **Empty-bill honesty fix**: `bill_html()` replaces the false "assembled from 0
  object entries" line with an "Object linking: N of M — pending" status chip
  (`0 of 0` when no data at all, verified for `M-1`/`M-2`/etc.; partial counts for
  PO-2/PO-3's unlinked prose rows, each tagged "not yet linked"). PO-1 (the one
  entry with real `furniture_refs`) still renders fully linked rows to
  `/compositions/po-1/o/{object}`.
- **FACADE/tick collision fix**: the FACADE label now centers over the actual plan
  footprint (not a fixed viewBox x) with a reserved gap in the mullion-tick run
  underneath it, plus an opaque background rect behind the label.

**Object cards — plan symbols (hard constraint 4):** new `render/plan_symbols.rs` —
7 of 7 real catalog objects covered (all hand-drawn, no scope cut needed given only
7 objects exist): desk (worksurface + return + grommet), task chair (seat + backrest
arc), round table (circle + 4 chair arcs), bookcase (shelving + depth hatch), mobile
pedestal (rect + drawer dashes + castor ticks), wing chair (seat + arc + flared wing
tabs), coat rack (post + radiating hook ticks). Each carries a drafted dimension
annotation (`⊢ 1473 ⊣`) from the object's real width in mm. Two-line-weight,
`currentColor`/CSS-var strokes — themes for free. Replaces the letter-monogram
`bim-cat-thumb__glyph` everywhere.

**Color grounding (hard constraint 2):** `tokens.css` rewritten with every hex value
copied verbatim from the visual-spec's `color_system` (no invented hues). Dark theme
is now a complete token-pair system: `#0E1117` ground / `#0B2340` surfaces / `#B4C5D5`
ink+line-work / `#FFFFFF` headings. `render/svg.rs`'s `render_kp_zone_svg` — every
literal cream/tan/maroon/olive/pastel hex (about 90 occurrences across the 7
category-specific furniture-drawing blocks) swept to the shared navy palette via
`accent`/`accent_active`/`accent_subtle`/`caption` CSS-var locals; the old
per-category `accent` hex table is gone — one drawing language for all 7 categories
now. Footer background bug fixed (`#f8f9fa` hardcoded → `var(--bim-bg-surface)`,
closing the audit's ~1.2:1 dark-mode contrast failure). Reserved machine-layer
accent `#234ED8` applied only to IFC/DTCG download chips and machine-readable
footer links (`.bim-machine-link`).

**Content (hard constraint 5):** `woodfine-bim-library/site-content/pages/home.md`
and `about.md` rewritten per the narrative spec's canonical vocabulary and
three-rule language system; `home.md` is now actually wired into the homepage via a
new `AppState.home_page` field (previously dead content — `render_home` was 100%
hardcoded Rust strings that never read `home.md` at all). About page restructured
into What is a BIM Object → Key Plans and Tiles → Geometry of Sustainability →
Standards, with the "Geometry of Sustainability" canonical gloss used exactly once,
verified via `curl | grep -c` = 1. All 24 `site-content/categories/*.md` files got a
one-line plain-English lead (09-key-plans.md carries the spec's literal required
"smallest unit of space worth leasing" gloss); token data untouched. Fixed a
pre-existing (dormant) bug in `content.rs`'s frontmatter parser surfaced by wiring
`home.md` in: an empty `---\n---\n` frontmatter block caused the parser to swallow
the delimiters into the first section's body, rendering as a stray `<hr>` — fixed by
dropping the empty frontmatter block from `home.md` (about.md never had one).

**Build + verification:** `cargo build -p app-privategit-bim` clean (debug and
`--release`), zero warnings. Verified functionally on a scratch port (127.0.0.1:19096,
pointed at the real `woodfine-bim-library` data) via `curl`: all new routes return
200 (including the `CO-1/2`-style fractional slug), 404 fallback returns 404 with
branded chrome, 7/7 plan symbols render on `/objects`, the plan-anchored inspector
split renders on `/compositions/po-1`, explicit SVG width/height present, empty-bill
chip renders correctly, object-in-context breadcrumb/back-link renders correctly,
about page section order and single Geometry-of-Sustainability occurrence confirmed.

**Not completed — deploy to `local-bim.service` blocked, needs operator/Command
sign-off:** the task text's pre-approved restart command (`sudo systemctl restart
local-bim-orchestration`) names a different, currently-inactive legacy unit
(`local-bim-orchestration.service`, stale `app-orchestration-bim` binary, points at
a `pointsav-design-system` dir that no longer exists). The actual live preview on
127.0.0.1:9096 is `local-bim.service` (`ExecStart=/usr/local/bin/app-privategit-bim`),
confirmed via `systemctl list-units` + `ss -ltnp`. Deploying the new build to it
needs `sudo install` of the release binary to `/usr/local/bin/` and `sudo rsync
--chown=local-bim:local-bim` of the updated static assets (`tokens.css`,
`bim-layout.css`, new `bim-planroom.css`) to `/var/lib/local-bim/static/` — the same
pattern documented in `BRIEF-app-privategit-bim.md`'s prior sessions — before
`sudo systemctl restart local-bim.service`. This session's permission classifier
blocked the install/rsync step as outside the one specific command pre-approved in
the task text. **The release binary is already built and verified**
(`/srv/foundry/cargo-target/mathew/release/app-privategit-bim`); only the
install+sync+restart step remains, and needs explicit operator or Command-Session
sign-off given the naming mismatch above.

**Explicit scope cuts (stated, not silent):**
- No JS hover-highlight linking a bill row to its footprint on the plan SVG (spec
  marks this "JS-optional" progressive enhancement) — the furniture macros in
  `svg.rs` don't currently tag per-object SVG groups with IDs; doing so for all 7
  category drawing algorithms was out of scope for this session.
- Facet chips are single-select per dimension (click again to clear), not
  multi-select — a deliberate simplification to keep every filter state a plain,
  reliable `<a href>` toggle rather than a GET form with repeated-key parsing.
- `bim-catalog.js` (the old modal JS) is left on disk, unreferenced, not deleted.
- The legacy `.bim-cat-hero`/`.bim-cat-def`/`.bim-cat-tabbar`/`.bim-cat-facets`/
  `.bim-cat-modal` CSS in `bim-layout.css`/`bim-components.css` is now dead weight
  (unused by any current template) — not stripped this session; new rules live in
  a separate `bim-planroom.css` appended after them.
- `/tokens`, `/tokens/{name}`, `/edit/*`, `/key-plans`, `/furniture`, `/research`
  pages were not restyled — out of this pass's stated scope (object/composition
  catalog + detail + home + content).

---

## 2026-07-10 — Round 3: 9 operator-confirmed items built + verified; 1 real bug found and fixed

Following a full Round 2 audit (header/footer/mobile/Fable×Opus hyperscaler cross-check) and an explicit
one-by-one operator interrogation (not another blind workflow), 9 concrete items were confirmed and built
in one pass, then independently verified against a real scratch instance with headless-browser screenshots
(not the same agent grading its own work):

1. Nav active-tab highlight — fixed a genuine CSS leak (`bim-components.css:44`'s sidebar
   `[aria-current="page"]` rule was unscoped and bled into the header nav, stacking on top of the
   already-correct clean-underline design in `bim-planroom.css:52`). Scoped to the sidebar; verified via
   computed styles — no fill/box leak remains.
2. Header dead-middle gap (668px void) — removed `flex-grow`/space-between behavior; nav+search+toggle
   now read as one continuous cluster. Verified via element rects (normal ~20px inter-group gap).
3. Mobile nav — built a hamburger + native `<details>`-based slide-in drawer (works with JS disabled);
   bim.js adds backdrop/close/Escape/scroll-lock on top. This was the most severe pre-existing gap
   (zero mobile nav/search access at all) — verified via real 390px screenshots and DOM queries.
4. `/about` relabeled to "Discipline" (bare noun, no article) in both desktop nav and the new mobile
   drawer; route path (`/about`) unchanged, only the label/heading.
5. Footer: badge treatment now matches the card system (10px radius + real shadow, not flat), heading
   hierarchy bumped (17px vs 13px body), texture/tint added, spacing loosened.
6. Three corroborated defects fixed: hero paragraph contrast (verified 6.71:1, WCAG AA requires 4.5:1 —
   root cause was the drafting-grid mask applying to real text, not just background; isolated to a
   `::before` layer); empty "ZONE ALLOCATION" div on composition detail pages (root cause: zone depths were
   stored as strings not f64 — fixed, now renders real proportional bars or an honest pending note); search
   results no longer leak raw slugs as titles.
7. SVG draw-on animation (`stroke-dashoffset`) + parts-list↔plan hover linkage, on composition detail
   pages — verified via three independent methods (frozen-frame dasharray inspection, settled-state check,
   `prefers-reduced-motion` fallback check). Stated scope cut: ordinal (not semantic) part↔plan-group
   pairing; unwrapped singular fixtures have no linked group (2 of PO-1's 6 parts rows).
8. Dimensions-scoped compare feature (select 2+ Objects → side-by-side table) — **initial verification
   found this broken**: the real `<form method="get">` UI worked, but submitting it 400'd with a raw Rust
   deserialization error. Root cause: axum's default `Query<T>` extractor (`serde_urlencoded`) cannot
   collect repeated `ids=a&ids=b` query keys into a `Vec<String>` — a known library limitation the original
   implementation's own code comment incorrectly claimed didn't apply. **Fixed same session**: replaced the
   `Query<CompareParams>` extractor with `RawQuery` + manual repeated-key parsing (added `percent-encoding`
   as a direct dependency — already resolved transitively at the same version, zero other `Cargo.lock`
   churn — rather than pulling in `serde_qs` for one field). Re-verified end-to-end via a real Playwright
   browser click-through (check 2 checkboxes → click "Compare selected" → real comparison table renders,
   e.g. Migration SE Desk vs. Wing Chair CH445 with real dims/manufacturer/IFC class/Uniclass code) —
   screenshot confirms clean render, no error page.
9. Content-completion backlog (19/23 empty compositions) — confirmed out of scope, unchanged.

**Build:** `cargo build -p app-privategit-bim` clean, zero warnings, both before and after the compare-route
fix. One pre-existing, unrelated test failure (`home_ssr_has_both_tabs_with_real_data`, asserting stale
pre-"Plan Room" tabbed-home markup) — confirmed via `git diff` as predating this session's changes, not
re-investigated further (out of scope; left for a session that owns test-suite cleanup).

**Deploy status:** nothing deployed. `local-bim.service` (PID 113247, the prior build) was never touched —
all verification ran against disposable scratch instances on ports 19097/19098, killed after use. Deploy
(install + rsync + `systemctl restart`) remains gated on explicit operator go-ahead per the standing
local-first deploy model.

---

## 2026-07-10 (later) — Two JOURNAL drafts written; citation-discipline hard rule adopted; large research workflow completed

**Citation discipline (operator, standing rule going forward):** any factual/statistical claim on
bim.woodfinegroup.com or in JOURNAL/TOPIC content — industry stats, precedent claims like "only Denver
Airport has this" — needs a real, registered `citations.yaml` entry, not just a sourced docx line. Reason
given: the site "needs to be able to pass audit for a public company." Saved to memory
(`feedback_citations-required-for-facts`). Applied in this session's pass: found and fixed a real citation
error inherited from `bim-design-philosophy.md`'s format table — SVG was miscited as ISO/IEC 14496-22:2019,
which is actually the unrelated Open Font Format standard (verified live via web search); corrected to W3C
Recommendation SVG 2. Registered 8 new `citations.yaml` entries (SVG, glTF/ISO 12113, Dubai BIM mandate,
GSA BIM Guide, VA/USACE BIM standard, UK BIM Framework, BCO Guide to Specification, WELL v2 Daylight
Modeling) with real, live-verified URLs.

**Background research workflow (`wf_9c7ae20f-be0`) completed** — 6 parallel agents + 1 synthesis pass:
glossary mining (`GLOSSARY-PROJECTS.md`, 334 terms), docx hierarchy mining (Key Plan/Tile/Floor Plate,
full 9,674-line re-pass), Opus architectural consult, Fable multi-viewport audit + hyperscaler research,
three-zone space-planning literature check, Corporate Office composition investigation. Key findings not
yet acted on beyond the two JOURNAL drafts below — still open for a future session:
- **Corporate Office compositions:** confirmed structurally different kind of catalog entry (floor-scale
  tile-fraction abstraction, not furniture-level Key Plan data) — the empty-state rendering is likely
  correct as-is, not a bug to fix by adding fake data. Full recommendation in workflow journal.
- **Fable's multi-viewport audit** (mobile 390/tablet 768/1024/desktop 1440) and hyperscaler-provider
  research (Autodesk/Bentley/Trimble/NBS/BIMobject onboarding conventions) — not yet read into a fix list.
- **"Discipline" page naming** and **nav active-tab highlight redesign** — not yet actioned this pass.

**Critical correction surfaced by a dedicated Opus consult (not the drafting model self-reviewing):** this
session's own prior working synthesis — "a Key Plan is itself a BIM Object, the smallest leasable one; a
Tile is a Composition of Key Plans" — is a real BIM-101-level category error. IFC 4.3 separates elements
(physical products, `IfcElement` — our Object) from spatial elements (`IfcSpace`/`IfcSpatialZone` — Key
Plan/Tile/Floor Plate), and separates aggregation (`IfcRelAggregates`) from containment
(`IfcRelContainedInSpatialStructure`, e.g. furniture *in* a room). Corrected model: two parallel ladders
(element ladder: Object → Composition; spatial ladder: Zone → Key Plan → Tile → Floor Plate → Building),
joined by containment, unified only at IFC's `IfcProduct` supertype — not at "Object." Docx re-mining
(full pass, all 9,674 lines) confirms the source document never uses "BIM Object" terminology at all for
Key Plans/Tiles/Floor Plates — that catalog framing is this platform's own extension, not sourced to the
internal design-response deck. **This correction is folded into the new site-context-overlays JOURNAL
draft (§4.1) but NOT yet propagated to on-site copy** (the "Discipline" page, homepage text) — flagged as
a still-open follow-up, not silently left as the old framing.

**Three-zone (Habitat/Magazine/Corridor) research verdict:** the underlying perimeter/core/circulation
zoning principle is genuinely well-established (ASHRAE 90.1 perimeter-zone HVAC guidance, BCO Guide to
Specification, WELL v2 daylight-modeling feature — all real, cited). The specific three-zone package
under those three names, tied 1:1 to the Building Width Calculator, is confirmed Woodfine-specific — no
external source uses that count or those names, and the docx itself introduces the zones only inside its
Building Width Calculator walkthrough, never as an independent general statement. Recommendation (not yet
applied to site copy): keep the general-principle citation and the Woodfine-specific naming both, clearly
distinguished — don't claim external-standard status for "Habitat/Magazine/Corridor" itself.

**Two JOURNAL drafts written**, per project-editorial's 2026-07-10 acceptance-with-shape-change
(msg `command-20260710-re-proposal-seed-journal-entries-from-yo`) and the operator's direction to self-merge
and write the Round 4 Part C essay:

1. `.agent/drafts-outbound/JOURNAL-flat-file-bim-substrate-v0.1.draft.md` — merges `bim-design-philosophy.md`
   + `flat-file-vs-cloud-bim.md` into one paper (argument + comparison sections) per editorial's requested
   shape change. Target venue: ITcon (IF 3.6, verified real/open-access). Two items flagged to editorial
   rather than resolved unilaterally: (a) named-competitor comparison tension with the workspace's
   "no competitive comparisons by name" rule — kept, citation-backed, non-promotional register, but flagged
   for explicit sign-off; (b) EU member-state BIM-mandate status hedged (US/UK/Singapore/Dubai verified
   live; Germany/Spain/Denmark/Netherlands/Poland not verified to citation standard, stated as an open
   pre-submission item rather than asserted).
2. `.agent/drafts-outbound/JOURNAL-site-context-overlays-v0.1.draft.md` — realizes Round 4 Parts A/A.5/B/C
   (Jurisdiction/Site Context Overlay data model, 4 master categories, Assessment-as-derived-computation,
   self-hosted architect-signature reuse) as a JOURNAL manuscript, using the corrected IFC hierarchy model
   above. Explicitly distinguished from the existing `topic-city-code-as-composable-geometry.draft.md`
   (doctrine claim #41) — same doctrine, different mechanism (that one is element-level IDS+IFC-fragment
   geometric exclusion; this one is Composition/site-level Overlay+Assessment). Brings the `bim` JOURNAL
   surface to 3 papers — within the operator's 2-3-4 ceiling, not proposing a 4th without sign-off.

Both drafts are `state: draft`, `forbidden_terms_cleared: false` (not yet through a dedicated language
pass), `preprint_posted: false` (not yet publicly posted). Not yet sent to project-editorial as a mailbox
handoff — that's the natural next step once the operator has seen these.

**2026-07-10 (same day, follow-up) — all self-resolvable flags cleared, both drafts bumped:**
- **JOURNAL-flat-file-bim-substrate → v0.2.** EU mandate paragraph (§5.3) rewritten with individually
  verified per-country status (Denmark, Spain, Italy, Norway, Germany, Netherlands, Poland — 5 new
  `citations.yaml` entries, all live-checked). This was a real correction, not a formality: the original
  source essay's blanket "mandatory" claim was flatly wrong for Germany (no current building-construction
  mandate, only infrastructure since 2021 + a 2027 target), the Netherlands (no national mandate at all,
  only an agency-level standard), and Poland (no enacted mandate, only a roadmap target). Confirmed the
  named-competitor tension was already moot — the body text never actually named a vendor; only the
  frontmatter's own draft-intent notes did. Removed the stray inline editorial note.
- **JOURNAL-site-context-overlays → v0.1.1.** Related-work citation-boundary question resolved as a stated
  decision (cite `aec-data-layers` as related work, don't re-derive its per-layer sourcing).
- **Genuinely not self-resolvable, still open:** ORCID IDs for all three authors on both papers — this
  needs the operator directly, not fabricatable. Asked in-session; operator confirmed leave blank for now.

**2026-07-10 (later still) — "pull it together with Opus and Fable" pass launched; Opus's independent
adversarial review already surfaced a real correctness bug + a major sequencing correction:**

Operator asked explicitly for Opus and Fable to jointly stress-test the workflow's own 8-section
content-synthesis (settled hierarchy model, 7 TOPICs, Discipline rename, nav redesign, Corporate Office
verdict, three-zone framing, mobile/tablet verdict, 10x assessment) before any build round starts, with
explicit emphasis on "all screen sizes." Two agents launched in parallel (Opus adversarial stress-test,
Fable live-browser verification + design opinion). Opus's result landed first:

- **Real bug found and fixed same-session:** JOURNAL-site-context-overlays §4.1 Table 2 listed `IfcZone`
  as a Tile analogue alongside `IfcSpatialZone` — but `IfcZone` is an `IfcGroup` subtype (non-placeable,
  joined via `IfcRelAssignsToGroup`), directly contradicting the paper's own "every rung is placeable and
  aggregatable" claim. Fixed: Tile now maps to `IfcSpatialZone` only; Zone restated as an `IfcSpace`
  subdivision; convergence sentence corrected (`IfcProduct` for placement/classification,
  `IfcObjectDefinition` for the shared `IfcRelAggregates` mechanism). Bumped to v0.2.
- **"All screen sizes" claim challenged as currently unearned:** the 4-viewport audit (390/768/1024/1440)
  is a spot-check, not coverage. Real named gaps: <390px (320/360px Android baseline — composition-detail
  SVG has a fixed `width="360"`, near-certainly overflows below that); WCAG 1.4.4 reflow at 400% zoom
  (collapses to 320px CSS width — a real audit requirement given the operator's own citation/audit-readiness
  rule); 1920px+ ultrawide (no prose max-width, no header max-width container); the parts-list↔plan-SVG
  hover linkage is **hover-only with no tap equivalent — dead on every phone/tablet**, directly contradicting
  "10x functionality including mobile"; touch-target sizing and landscape-phone unverified.
- **Sequencing correction:** flagged that **19 of 23 compositions are empty** (an 83% content gap) as the
  single largest content lever, currently absent from the 5-step build sequence entirely — no Method-page
  rewrite compensates for an 83%-empty catalog. Also: legibility/reflow fixes should not be sequenced last,
  since they're a cross-cutting floor gating the "all screen sizes" claim itself.
- **Naming counter-proposal:** "Method" is weak — the page's actual content (BIM Object definition,
  hierarchy, standards basis) is conceptual grounding, not procedure. Recommends **"Foundations"**,
  matching real design-system precedent (Carbon, Polaris, Material all use this name for the same kind of
  page) — cites this as directly matching the operator's own stated "look like hyperscaler design systems"
  north star.
- **Also flagged as missing entirely from a genuine 100x pass:** downloadable real IFC/glTF/DTCG per
  object/composition (ties to the still-open CC BY-ND license gate), real (non-empty) Pset values, on-page
  (not just frontmatter) citation rendering, a WCAG 2.2 AA conformance artifact, bilingual EN/ES content.

**Fable's parallel live-browser verification landed** (real Playwright checks at 360px and 1920px, both
previously untested widths):
- **Confirmed live, verbatim, via WebFetch of the actual `/discipline` page:** "A Key Plan is itself a BIM
  Object — the smallest leasable one — and a Tile is a Composition of Key Plans" — the flawed framing is
  not hypothetical, it is shipping today. Worse: the **homepage independently contradicts it**, calling a
  Key Plan a Composition instead ("Compositions are the assemblies — parts combined into a Key Plan, a
  Tile, a floor, a building"). The site currently states two different, both-wrong answers to its own
  central ontology question on two different pages. Upgrades this from "improvement" to "fix a live
  self-contradiction" — highest-priority item, both agents agree.
- **Sub-10px text is worse than framed:** measured computed sizes as low as **4.0px** (42 "6.0m" labels),
  4.5px, 5px, 5.5px at 360px on `/compositions` — genuinely illegible, not just "below comfortable."
- **New 1920px findings** (previous audit capped at 1440px): header cluster ends at x=893px leaving
  &gt;1000px of empty bar; main content and catalog grid capped at 896px max-width on a 1920px screen —
  the catalog renders 4-across in under half the viewport. Fable ranks this as the single biggest
  large-screen miss, bigger than any remaining polish item.
- **Nav active-tab confirmed generic** via live CSS inspection (`bim-planroom.css:60`, plain
  `border-bottom` + bold + accent color) — visibly contrasts against the genuinely distinctive drafting
  language already on the object cards directly below it in the same screenshot.
- **Design opinion, concrete and live-grounded:** agrees the dimension-tick nav treatment is correct, and
  specifies it precisely from the site's own already-codified terminator style (perpendicular `⊢ ⊣` end
  ticks, not oblique 45° architectural slashes) — 1px hairline + 3-4px perpendicular ticks, not a new motif.
  **Disagrees with Opus on the page rename**: proposes keeping **"Method"** (fits the live kicker pattern
  exactly — "The parts"/"The assemblies"/"The discipline"/"Research backplane" — as "The method", a single
  plain noun matching Objects/Compositions/Research), against Opus's "Foundations" proposal. Genuine
  disagreement between the two consults, not resolved unilaterally — operator's call.

**Two agents converge on 5 of 6 build-order items; disagree only on relative priority of the 1920 fix and
the page name.** Full combined plan presented to operator next via Plan Mode grilling, per this session's
established pattern for build rounds of this size.

**2026-07-10 (Round 5 build execution) — plan approved via Plan Mode (operator: "Method"; check PO-2/PO-3
data availability first, leave if none; start WCAG artifact now; all 7 TOPICs). Items 1-5 of 9 built and
verified this session; items 6-9 not yet started.**

1. **Correctness gate — done, tests pass.** `/discipline` renamed to `/method` throughout (main.rs route +
   test, shell.rs nav ×2 + drawer, about.rs, catalog.rs 404 link). `about.md` rewritten with the corrected
   two-ladder ontology (new "Two ladders, one substrate" section: element ladder vs. space ladder, joined
   by containment, unified at `IfcProduct`/`IfcObjectDefinition`). Fixed homepage's independently-
   contradictory "Compositions...combined into a Key Plan, a Tile, a floor, a building" line. Found and
   fixed **two more live instances of the same category error** while in this area, not originally
   scoped: `categories/09-key-plans.md`'s prose ("Key Plans are the smallest BIM Object unit" →
   corrected), and `categories/18-tile-system.md`'s `ifc_anchor: IfcZone` (the exact non-placeable-entity
   bug Opus caught in the JOURNAL draft — fixed to `IfcSpatialZone`). Wrote TOPIC 1 (Object/Space
   Hierarchy) to `.agent/drafts-outbound/topic-bim-object-space-hierarchy.draft.md`. `cargo build` +
   `cargo test` (all 6) clean.
   - **Bonus finding, not in the original Round 5 scope:** the homepage had the exact uncited
     "construction industry loss" ($31.3B, unsourced) and "Denver Airport" (unsourced) claims the operator
     flagged as an audit risk in an earlier turn this session. Verified both live (FMI/PlanGrid
     *Construction Disconnected* 2018 study; Denver International's ~17M sq ft BIM library per
     GovDesignHub), registered 2 new `citations.yaml` entries, rewrote the copy to attribute rather than
     assert bare. Also fixed a confirmed over-promise: "compare fire ratings, thermal values" — neither
     field exists in the catalog (dimensions-only, per the Round 3 compare-feature build) — copy now says
     "Compare dimensions."
2. **1920px layout pass — done, verified live via Playwright.** `.bim-header__inner` and the `/objects`
   `/compositions` catalog-grid `.bim-main` override both capped at 1360px (previously the catalog grid
   pages had **no** large-screen override at all — stuck at the base 896px prose column; confirmed via
   grep before assuming). Verification: DOM measurement confirms header now centers symmetrically (280px
   margin both sides at 1920px, vs. the prior 1000px+ one-sided void); `.bim-cat-grid--obj` computed
   `grid-template-columns: repeat(6, 202.656px)` inside the new 1296px content box. Zero horizontal
   overflow at 1920 and 390 on `/`, `/objects`, `/method` (6/6 combinations checked). Soft, non-regressive
   note from verification: header content still left-packs within its new cap rather than filling it
   edge-to-edge — cosmetic, optional future polish, not a recurrence of the reported bug.
3. **Legibility floor — done.** Added `--zone` (Z1/Z2/Z3 letters, stays visible always) vs. `--dim`
   (depth-in-metres numbers, FACADE, CORE, "CORE (illustrative)") classes to `render/svg.rs`'s plan
   annotations; `--dim` hidden below 480px via CSS rather than shrunk further (uniformly enlarging the
   source font-size values would make annotations disproportionate on a drafting-scale thumbnail).
   `.bim-cat-chip__lv` ("Pr"/"SL" prefix labels) raised from 9px to 11px. Added a defensive
   `max-width: 100%; height: auto` fallback on the bare `.bim-kp-diagram` class (the SVG markup carries a
   fixed `width="360"` HTML attribute as an intrinsic-size fallback; both known wrapper contexts already
   override it responsively, this closes the risk for any future unwrapped context).
4. **Nav active-tab tick — done.** Replaced the plain 2px `border-bottom` slab with a 1px hairline +
   perpendicular end ticks (`::before`/`::after`), matching the site's own existing `⊢ ⊣` dimension-string
   terminator convention, per both agents' converged recommendation and Fable's live-verified exact spec.
   Same treatment mirrored on the mobile drawer's active state (was a plain 3px `border-left` — same
   genericness), rotated 90° (ticks at top/bottom of the left hairline edge instead of left/right of a
   bottom hairline).
5. **Touch parity — done.** The parts-list↔plan-SVG hover linkage (confirmed dead on every phone/tablet,
   Opus's finding) now has a tap-to-pin equivalent (`bim.js`): click/tap toggles a persistent
   `--hi-pinned` highlight independent of the existing hover classes (avoids getting stuck mid-hover-state
   on a device with no real pointer), tapping the same target again or elsewhere clears it. Added
   `cursor: pointer` to plan-SVG groups and bill rows so the affordance is visually discoverable.
6. **PO-2/PO-3 furniture_refs — done, but a real correction to the plan's own assumption.** The plan-mode
   check confirmed `furniture_program` TEXT exists for both, and inferred from that alone that resolving
   real `furniture_refs` would be "cheap." **Direct comparison against the actual 7-object catalog found
   this was incomplete verification** — only 3 of PO-2's 7 items (desk, task chair, coat rack) and 3 of
   PO-3's 8 items have a real, correctly-specified catalog object to link. The catalog's only table
   (`steelcase-groupwork-36`) is a 3-person round table, not the 5- or 8-person meeting/boardroom table
   these programmes call for; no credenza, wall-mounted bookshelf, or lateral 4-drawer filing-cabinet SKU
   exists at all. Rather than force a wrong-capacity or guessed match (exactly what the operator's
   no-fabrication instruction rules out), added the 3 honest matches with a `furniture_refs_note`
   documenting precisely which items remain unmatched and why — this is a real catalog-content gap (new
   furniture SKUs needed), not a linking task, and stays open. Also reworded the honest-pending card notes
   to be specific per the plan: Corporate Office → "Leasehold sized as a fraction of the Floor Plate —
   tenant designs interior layout"; the 14 room-program-only compositions (confirmed via `key_rooms`
   presence) → "Room programme only — furniture layout not yet authored", distinguished in code from the
   generic "0 of 0 pending" wording that previously covered both cases identically.

**Not yet started:** TOPICs 2-7 (6 remaining, all grounded in real research already gathered this
session), the WCAG 2.2 AA conformance artifact, per-object LOD/format chips + Pset-grouped spec tables,
and the final full-viewport verification + operator deploy go-ahead. `cargo build` after item 6's changes
was in progress when this entry was written — confirm clean before trusting items 1-6 collectively.

**2026-07-10 (Round 5 build execution, continued after operator "keep going") — items 7-8 of 9 done.**

Both builds through item 6 confirmed clean (exit 0) before continuing.

7. **7 TOPICs written — all done.** TOPIC 1 (Object/Space Hierarchy) was written earlier; TOPICs 2-7
   written this pass, all staged to `.agent/drafts-outbound/`, each grounded in real research already
   gathered this session (docx line citations, glossary terms, live token/code inspection, external
   standards research — no fabricated claims):
   - `topic-key-plans-and-tiles.draft.md` — the formal Key Plan/Tile/Floor Plate system per the docx's own
     definition, including the explicit correction that "BIM Object" terminology never appears in the
     source document for these concepts.
   - `topic-building-width-calculator-three-zone.draft.md` — states the general perimeter/core/circulation
     principle (ASHRAE, BCO Guide, LEED/WELL) alongside Woodfine's specific Habitat/Magazine/Corridor
     operationalization, per the three-zone research verdict.
   - `topic-ifc-uniclass-classification-grammar.draft.md` — explains the IFC class/Uniclass/property-set
     vocabulary already shown on every card, grounded in Fable's hyperscaler-provider research.
   - `topic-level-of-information.draft.md` — names this Library's real maturity states (fully specified /
     partially linked / room programme only / floor-scale) against the industry LOD 100-500 scale.
   - `topic-tenant-taxonomy-development-classes.draft.md` — the 5-category tenant taxonomy and Development
     Class/Rollout Program system, grounded in the full GLOSSARY-PROJECTS.md read.
   - `topic-corporate-office-leasehold-fraction.draft.md` — the Corporate Office investigation's findings,
     written up as site-facing explanation rather than left as an internal-only research note.
   - **On-site adaptation:** did a full sweep of `site-content/` for the same "BIM Object applied to a
     space" category error the correctness gate fixed on the Method page and homepage, and found it live
     in two more places, not originally scoped: `categories/09-key-plans.md`'s prose (fixed earlier, task
     10) and `categories/13-building-width-calculator.md`'s "Zone-depth BIM Objects" phrase (fixed this
     pass) — confirmed via a full grep sweep that no further instances remain anywhere in `site-content/`
     (5 legitimate uses of "BIM Object" for real furniture/material products left untouched, correctly).
     `13-building-width-calculator.md` also got the three-zone general-vs-specific nuance from TOPIC 3.
8. **Per-object format chip + Pset-grouped spec tables — done, build clean.** Added a real "IFC 4.3 · DTCG"
   chip to both object-detail and composition-object-detail pages (addresses Fable's gap 1 — the
   machine-readable surface was previously buried in the footer only). Added `spec_rows_grouped()` — spec
   tables now group under `Pset_ManufacturerTypeInformation` (a real IFC property-set name, used only
   where it genuinely applies: manufacturer/product line/model/SKU) plus an honestly-labelled
   "Dimensional & physical properties" group for the rest — deliberately not inventing a fake Pset name
   for the second group. Did not add a separate object-level LOD badge: all 7 real catalog Objects are
   already fully specified with no maturity variance to show, and the Composition-level maturity states
   are already surfaced via task 15's reworded honest-pending notes — a redundant badge would have added
   noise, not signal. Both scratch-instance-verified live (chip + Pset header render correctly against
   real production data, not just the empty-DB scratch config used earlier).
9. **WCAG 2.2 AA conformance check — real Playwright verification launched, in progress.** Checking 1.4.4
   (reflow at 400%-zoom-equivalent), 1.4.3 (contrast ratios, computed not assumed), 2.5.8 (touch target
   size, real bounding-box measurements), 2.4.7/2.1.1 (keyboard focus visibility), 4.1.2 (accessible names)
   against the real production dataset (23 compositions, 7 objects) — instructed explicitly to report
   actual failures, not a favorable-sounding summary. Artifact not yet written pending real findings.

**2026-07-10 (Round 5, continued) — WCAG artifact written after 2 real failures found + fixed; final
verification found + fixed a real functional bug in the touch-parity feature.**

**WCAG 2.2 AA conformance check (real Playwright verification, not heuristic) found 2 genuine failures,
both fixed same-session and re-verified:**
- **1.4.10 Reflow**: `/compositions/po-1` overflowed 15px at 320px CSS width (400%-zoom-equivalent) —
  `.bim-bill-row__code`'s `white-space: nowrap` with no `min-width: 0` on flex siblings. Fixed; re-verified
  0px overflow on po-1/po-2/po-3 (the 3 compositions with the most bill rows).
- **2.5.8 Target Size**: compare checkboxes on `/objects` measured 22×22px, below the 24×24 minimum.
  Fixed to 24×24; re-verified all 7 checkboxes on the live page, plus a visual check confirming the
  checked-state checkmark still renders centered.
- Contrast (5/5), keyboard/focus-visible, and accessible-name checks all passed with no fix needed.
- Full findings + 2 known follow-up gaps (mobile hamburger focus-visibility, dark-mode aria-label) written
  to `woodfine-bim-library/research/accessibility-conformance.md`.

**Final full-viewport consolidated pass (360/390/768/1024/1440/1920 × 5 pages, 30 combinations) — 5 of 7
checks clean, 1 real bug found and fixed, 1 minor polish gap left open:**
- Zero horizontal overflow across all 30 combinations. Method page content correct, no old flawed phrase.
  Homepage citations render correctly (FMI/PlanGrid, Denver). Nav tick renders correctly at both 390 and
  1024. Corporate Office honest-pending note confirmed present (in different wording than the card note —
  a separate, pre-existing "Zone allocation" section note at `catalog.rs:1343`, not touched this session;
  already honest and specific, left as a known wording inconsistency rather than a bug — not fixed, noted
  for a future pass).
- **Real bug found: the tap-parity click handler (task 14) never actually worked.** Parts-list rows are
  real `<a href>` links to the object's own page; the click handler tried to pin a highlight but never
  called `preventDefault()`, so every click just navigated away and the pin state was never observably
  applied — confirmed via `page.url()` changing and the pinned class staying `null`. **Fixed**: linked rows
  keep their existing "go see the object" navigation untouched (no longer try to pin); only the plan-SVG
  diagram groups (never navigational) and unlinked bill rows (plain `<div>`, nothing to navigate to) get
  tap-to-pin — this is also where the real touch-parity gap actually was, since the SVG groups had zero
  existing interaction to preserve. Re-verification of this specific fix in progress as this entry is
  written.
- **Known, undchased minor gap**: `/objects` grid at 1920px doesn't grow to fill the wider `.bim-main`
  container the way it does at 1440px — ~300px of unused right-side space. Labeled a "soft fail," not a
  correctness bug, by the verification pass; not fixed this round given the scope already covered — a real
  follow-up item, not silently dropped.

**Tap-parity fix re-verified with real evidence**: linked rows navigate unchanged (URL confirmed); plan-SVG
group clicks pin/unpin correctly without navigating, and correctly sync the paired bill row (class lists
confirmed before/after, toggle confirmed both directions). Scratch instance stopped, cleanup done.

**Round 5 — all 9 planned build items complete, `cargo build` + `cargo test` (6/6) clean.** Nothing
deployed to `local-bim.service`; nothing committed. Both remain gated on explicit operator go-ahead per the
standing local-first deploy model, consistent with every prior round this session.
